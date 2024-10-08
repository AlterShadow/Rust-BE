use crate::AppState;
use api::cmc::CoinMarketCap;
use axum::http::StatusCode;
use bytes::Bytes;
use chrono::Utc;
use eth_sdk::dex_tracker::{
    get_strategy_id_from_watching_wallet, parse_dex_trade,
    update_expert_listened_wallet_asset_balance_cache,
    update_user_strategy_pool_asset_balances_on_copy_trade,
};
use eth_sdk::erc20::Erc20Token;
use eth_sdk::escrow::{parse_escrow_withdraw_event, EscrowContract};
use eth_sdk::escrow_tracker::escrow::parse_escrow_transfer;
use eth_sdk::evm::{parse_quickalert_payload, DexTrade};
use eth_sdk::execute_transaction_and_ensure_success;
use eth_sdk::pancake_swap::execute::PancakeSmartRouterContract;
use eth_sdk::strategy_pool::StrategyPoolContract;
use eth_sdk::strategy_pool_herald::parse_strategy_pool_herald_redeem_event;
use eth_sdk::utils::{
    decimal_to_u256, u256_to_decimal, wait_for_confirmations, wait_for_confirmations_simple,
};
use eth_sdk::{
    evm, EthereumRpcConnection, ScaledMath, TransactionFetcher, TransactionReady, CONFIRMATIONS,
    MAX_RETRIES, POLL_INTERVAL,
};
use eyre::*;
use gen::database::*;
use gen::model::*;
use lib::log::DynLogger;
use mc2fi_user::shared_method::{
    update_asset_balances_and_ledger_exit_strategy,
    update_strategy_token_balances_and_ledger_exit_strategy,
};
use rust_decimal::Decimal;
use std::sync::Arc;
use std::time::Duration;
use tracing::*;
use web3::ethabi::Address;
use web3::signing::Key;
use web3::types::U256;
pub async fn handle_swaps(
    state: Arc<AppState>,
    body: Bytes,
    blockchain: EnumBlockChain,
) -> Result<(), StatusCode> {
    let hashes = parse_quickalert_payload(body).map_err(|e| {
        error!("failed to parse QuickAlerts payload: {:?}", e);
        StatusCode::BAD_REQUEST
    })?;

    for hash in hashes {
        let conn = state.eth_pool.get(blockchain).await.map_err(|err| {
            error!("error fetching connection guard: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        let state = state.clone();
        tokio::spawn(async move {
            /* the transactions from the quickalerts payload might not be yet mined */
            match wait_for_confirmations_simple(&conn.eth(), hash, Duration::from_secs(10), 10)
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    error!("swap tx was not mined: {:?}", e);
                    return;
                }
            }
            // TODO: wait for confirmations blocks before processing to properly handle ommer blocks & reorgs
            let tx = match TransactionFetcher::new_and_assume_ready(hash, &conn).await {
                Ok(tx) => tx,
                Err(err) => {
                    error!("error processing tx: {:?}", err);
                    return;
                }
            };
            if let Err(e) = evm::cache_ethereum_transaction(&tx, &state.db, blockchain).await {
                error!("error caching transaction: {:?}", e);
            };
            match handle_swap_transaction(state.clone(), blockchain, tx).await {
                Ok(_) => {}
                Err(e) => {
                    error!("error handling swap: {:?}", e);
                }
            }
        });
    }

    Ok(())
}

pub async fn handle_swap_transaction(
    state: Arc<AppState>,
    blockchain: EnumBlockChain,
    tx: TransactionReady,
) -> Result<()> {
    /* parse trade */
    let expert_trade = parse_dex_trade(
        blockchain,
        &tx,
        &state.dex_addresses,
        &state.pancake_swap_parser,
    )
    .await?;
    let token_in_row = state
        .db
        .execute(FunUserListEscrowTokenContractAddressReq {
            limit: 1,
            offset: 0,
            token_id: None,
            blockchain: Some(blockchain),
            address: Some(expert_trade.token_in.into()),
            symbol: None,
            is_stablecoin: None,
        })
        .await?
        .into_result()
        .with_context(|| {
            format!(
                "could not find token_in {:?} in escrow token contracts",
                expert_trade.token_in
            )
        })?;
    let token_in_decimals = token_in_row.decimals;
    let token_out_decimals = state
        .db
        .execute(FunUserListEscrowTokenContractAddressReq {
            limit: 1,
            offset: 0,
            token_id: None,
            blockchain: Some(blockchain),
            address: Some(expert_trade.token_out.into()),
            symbol: None,
            is_stablecoin: None,
        })
        .await?
        .into_result()
        .with_context(|| {
            format!(
                "could not find token_out {:?} in escrow token contracts",
                expert_trade.token_out
            )
        })?
        .decimals;
    /* update last dex trade cache table */
    state
        .db
        .execute(FunWatcherUpsertLastDexTradeForPairReq {
            transaction_hash: tx.get_hash().into(),
            blockchain,
            dex: EnumDex::PancakeSwap,
            token_in_address: expert_trade.token_in.into(),
            token_out_address: expert_trade.token_out.into(),
            amount_in: u256_to_decimal(expert_trade.amount_in, token_in_decimals as _),
            amount_out: u256_to_decimal(expert_trade.amount_out, token_out_decimals as _),
        })
        .await?;

    /* check if caller is a strategy watching wallet & get strategy id */
    let caller = tx.get_from().context("no from address found")?;

    /* update wallet activity ledger & make sure this transaction is not a duplicate */
    let saved = state
        .db
        .execute(FunWatcherSaveStrategyWatchingWalletTradeLedgerReq {
            address: expert_trade.caller.clone().into(),
            transaction_hash: expert_trade.hash.into(),
            blockchain,
            contract_address: expert_trade.contract.into(),
            dex: Some(EnumDex::PancakeSwap.to_string()),
            token_in_address: Some(expert_trade.token_in.into()),
            token_out_address: Some(expert_trade.token_out.into()),
            amount_in: Some(u256_to_decimal(
                expert_trade.amount_in,
                token_in_decimals as _,
            )),
            amount_out: Some(u256_to_decimal(
                expert_trade.amount_out,
                token_out_decimals as _,
            )),
            happened_at: Some(Utc::now().timestamp()),
        })
        .await
        .context("swap transaction is a duplicate")?
        .into_result();
    if let Some(saved) = saved {
        /* update expert wallet's asset balances in the database */

        update_expert_listened_wallet_asset_balance_cache(
            &state.db,
            &expert_trade,
            saved.fkey_token_out,
            saved.fkey_token_in,
            blockchain,
        )
        .await?;

        for strategy_id in
            get_strategy_id_from_watching_wallet(&state.db, blockchain, caller).await?
        {
            if let Err(err) = copy_trade_for_strategy(
                state.clone(),
                strategy_id,
                blockchain,
                expert_trade.clone(),
            )
            .await
            {
                error!("error copy trading for strategy {}: {:?}", strategy_id, err);
            }
        }
    }

    Ok(())
}
pub async fn copy_trade_for_strategy(
    state: Arc<AppState>,
    strategy_id: i64,
    blockchain: EnumBlockChain,
    expert_trade: DexTrade,
) -> Result<()> {
    info!("start copy trading for strategy {}", strategy_id);

    let conn = state.eth_pool.get(blockchain).await?;
    let token_in_row = state
        .db
        .execute(FunUserListEscrowTokenContractAddressReq {
            limit: 1,
            offset: 0,
            token_id: None,
            blockchain: Some(blockchain),
            address: Some(expert_trade.token_in.into()),
            symbol: None,
            is_stablecoin: None,
        })
        .await?
        .into_result()
        .with_context(|| {
            format!(
                "could not find token_in {} in escrow token contracts",
                expert_trade.token_in
            )
        })?;
    let token_in_decimals = token_in_row.decimals;
    let token_out_decimals = state
        .db
        .execute(FunUserListEscrowTokenContractAddressReq {
            limit: 1,
            offset: 0,
            token_id: None,
            blockchain: Some(blockchain),
            address: Some(expert_trade.token_out.into()),
            symbol: None,
            is_stablecoin: None,
        })
        .await?
        .into_result()
        .with_context(|| {
            format!(
                "could not find token_out {} in escrow token contracts",
                expert_trade.token_out
            )
        })?
        .decimals;
    /* get expert wallet token_in asset balance prior to the trade */
    let expert_wallet_asset_token_in_previous_amount = state
        .db
        .execute(FunWatcherListExpertListenedWalletAssetBalanceReq {
            limit: None,
            offset: None,
            strategy_id: Some(strategy_id),
            token_id: Some(token_in_row.token_id),
            blockchain: Some(blockchain),
            address: None,
        })
        .await?
        .into_rows()
        .into_iter()
        .fold(Decimal::new(0, 0), |acc, row| acc + row.balance);

    if expert_wallet_asset_token_in_previous_amount == Decimal::new(0, 0) {
        bail!("sold asset was not previously an expert wallet asset, can't calculate trade token_in allocation for strategy pool");
    }

    // TODO: for loop to copy-trade for strategy pools on all deployed chains when we support multi-chain
    /* if token_in was already a expert wallet asset trade it from SPs in ratio traded_amount / old_amount */
    let strategy_pool_contract_row = state
        .db
        .execute(FunWatcherListStrategyPoolContractReq {
            limit: 1,
            offset: 0,
            strategy_id: Some(strategy_id),
            blockchain: Some(blockchain),
            address: None,
        })
        .await?
        .into_result()
        .context("could not fetch strategy pool contract row on this chain")?;

    let address: Address = strategy_pool_contract_row.address.into();
    /* check if SP contract holds token_in */
    let strategy_pool_contract = StrategyPoolContract::new(conn.clone(), address)?;
    let strategy_pool_asset_token_in_row = state
        .db
        .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
            strategy_pool_contract_id: Some(strategy_pool_contract_row.pkey_id),
            token_address: Some(expert_trade.token_in.into()),
            blockchain: Some(blockchain),
            strategy_id: None,
        })
        .await?
        .into_result()
        .context("strategy pool contract does not hold asset to sell")?;

    let sp_asset_token_in_previous_amount = strategy_pool_asset_token_in_row.balance;
    if sp_asset_token_in_previous_amount.is_zero() {
        bail!("strategy pool has no asset to sell");
    }
    /* calculate how much to spend */
    let corrected_amount_in = if u256_to_decimal(expert_trade.amount_in, token_in_decimals as _)
        > expert_wallet_asset_token_in_previous_amount
    {
        /* if the traded amount_in is larger than the total amount of token_in we know of the strategy,
             it means that the trader has acquired tokens from sources we have not read
             if we used an amount_in that is larger in the calculation, it would make amount_to_spend
             larger than the amount of token_in in the strategy pool contract, which would revert the transaction
             so we use the total amount of token_in we know of the strategy,
             which will result in a trade of all the strategy pool balance of this asset
        */
        expert_wallet_asset_token_in_previous_amount
    } else {
        u256_to_decimal(expert_trade.amount_in, token_in_decimals as _)
    };
    let amount_to_spend = corrected_amount_in * sp_asset_token_in_previous_amount
        / expert_wallet_asset_token_in_previous_amount;
    if amount_to_spend.is_zero() {
        bail!("spent ratio is too small to be represented in amount of token_in owned by strategy pool contract");
    }
    info!(
        "amount_to_spend: {} {:?}",
        amount_to_spend, expert_trade.token_in
    );
    /* instantiate pancake swap contract */
    let pancake_contract = PancakeSmartRouterContract::new(
        conn.clone(),
        state
            .dex_addresses
            .get(blockchain, EnumDex::PancakeSwap)
            .ok_or_else(|| eyre!("pancake swap not available on this chain"))?,
    )?;

    /* acquire asset before trade */
    let acquire_asset_before_trade_transaction = || {
        let amount_to_spend = decimal_to_u256(amount_to_spend, 18);
        strategy_pool_contract.acquire_asset_before_trade(
            &conn,
            state.master_key.clone(),
            expert_trade.token_in,
            amount_to_spend,
        )
    };

    execute_transaction_and_ensure_success(
        acquire_asset_before_trade_transaction,
        &conn,
        CONFIRMATIONS,
        MAX_RETRIES,
        POLL_INTERVAL,
        &DynLogger::empty(),
    )
    .await?;
    /* instantiate token_in and token_out contracts from expert's trade */
    let token_in_contract = Erc20Token::new(conn.clone(), expert_trade.token_in)?;
    let token_out_contract = Erc20Token::new(conn.clone(), expert_trade.token_out)?;

    /* approve pancakeswap to trade token_in */
    let approve_transaction = || {
        let amount_to_spend = decimal_to_u256(amount_to_spend, token_in_decimals as _);
        token_in_contract.approve(
            &conn,
            state.master_key.clone(),
            pancake_contract.address(),
            amount_to_spend,
            DynLogger::empty(),
        )
    };

    execute_transaction_and_ensure_success(
        approve_transaction,
        &conn,
        CONFIRMATIONS,
        MAX_RETRIES,
        POLL_INTERVAL,
        &DynLogger::empty(),
    )
    .await?;

    /* trade token_in for token_out */
    info!(
        "copy_trade_and_ensure_success: amount_in: {}, amount_out_minimum: {}",
        amount_to_spend, 1
    );

    let copy_trade_pair_paths = expert_trade.get_pancake_pair_paths()?;
    let copy_trade_transaction = || {
        let amount_to_spend = decimal_to_u256(amount_to_spend, token_in_decimals as _);
        pancake_contract.copy_trade(
            &conn,
            state.master_key.clone(),
            copy_trade_pair_paths.clone(),
            amount_to_spend,
            U256::one(),
        )
    };

    let copy_trade_hash = execute_transaction_and_ensure_success(
        copy_trade_transaction,
        &conn,
        CONFIRMATIONS,
        MAX_RETRIES,
        POLL_INTERVAL,
        &DynLogger::empty(),
    )
    .await?;

    info!(
        "copy_trade_and_ensure_success: tx_hash: {:?}",
        copy_trade_hash
    );

    let pending_wallet_trade_receipt = conn
        .eth()
        .transaction_receipt(copy_trade_hash)
        .await?
        .context("could not find transaction receipt for copy trade")?;

    /* parse trade to find amount_out */
    let strategy_pool_pending_wallet_trade = parse_dex_trade(
        blockchain,
        &TransactionFetcher::new_and_assume_ready(
            pending_wallet_trade_receipt.transaction_hash,
            &conn,
        )
        .await?,
        &state.dex_addresses,
        &state.pancake_swap_parser,
    )
    .await?;

    /* approve strategy pool for amount_out */
    info!(
        "approve strategy pool for amount_out: {:?} {:?} {:?}",
        state.master_key.address(),
        strategy_pool_contract.address(),
        strategy_pool_pending_wallet_trade.amount_out,
    );

    let approve_transaction = || {
        token_out_contract.approve(
            &conn,
            state.master_key.clone(),
            strategy_pool_contract.address(),
            strategy_pool_pending_wallet_trade.amount_out,
            DynLogger::empty(),
        )
    };

    execute_transaction_and_ensure_success(
        approve_transaction,
        &conn,
        CONFIRMATIONS,
        MAX_RETRIES,
        POLL_INTERVAL,
        &DynLogger::empty(),
    )
    .await?;

    /* give back traded assets */
    info!(
        "give_back_assets_after_trade: {:?} {:?} {:?}",
        state.master_key.address(),
        strategy_pool_contract.address(),
        strategy_pool_pending_wallet_trade.amount_out,
    );
    let give_back_assets_after_trade_transaction = || {
        strategy_pool_contract.give_back_assets_after_trade(
            &conn,
            state.master_key.clone(),
            vec![expert_trade.token_out],
            vec![strategy_pool_pending_wallet_trade.amount_out],
        )
    };

    execute_transaction_and_ensure_success(
        give_back_assets_after_trade_transaction,
        &conn,
        CONFIRMATIONS,
        MAX_RETRIES,
        POLL_INTERVAL,
        &DynLogger::empty(),
    )
    .await?;

    /* update strategy pool contract asset balances & ledger */

    state
        .db
        .execute(FunWatcherUpsertStrategyPoolContractAssetBalanceReq {
            strategy_pool_contract_id: strategy_pool_contract_row.pkey_id,
            token_address: expert_trade.token_in.into(),
            blockchain,
            new_balance: sp_asset_token_in_previous_amount - amount_to_spend,
        })
        .await?;

    let maybe_strategy_pool_asset_token_out_row = state
        .db
        .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
            strategy_pool_contract_id: Some(strategy_pool_contract_row.pkey_id),
            token_address: Some(expert_trade.token_out.into()),
            blockchain: Some(blockchain),
            strategy_id: None,
        })
        .await?
        .into_result();
    let token_in_amount = u256_to_decimal(
        strategy_pool_pending_wallet_trade.amount_in,
        token_in_decimals as _,
    );
    let token_out_amount = u256_to_decimal(
        strategy_pool_pending_wallet_trade.amount_out,
        token_out_decimals as _,
    );
    let strategy_pool_asset_token_out_new_balance = match maybe_strategy_pool_asset_token_out_row {
        Some(token_out) => token_out.balance + token_out_amount,
        None => token_out_amount,
    };

    state
        .db
        .execute(FunWatcherUpsertStrategyPoolContractAssetBalanceReq {
            strategy_pool_contract_id: strategy_pool_contract_row.pkey_id,
            token_address: expert_trade.token_out.into(),
            blockchain,
            new_balance: strategy_pool_asset_token_out_new_balance,
        })
        .await?;

    state
        .db
        .execute(FunUserAddStrategyPoolContractAssetLedgerEntryReq {
            strategy_pool_contract_id: strategy_pool_contract_row.pkey_id,
            token_address: strategy_pool_pending_wallet_trade.token_in.into(),
            blockchain,
            amount: token_in_amount,
            transaction_hash: pending_wallet_trade_receipt.transaction_hash.into(),
            is_add: false,
        })
        .await?;

    state
        .db
        .execute(FunUserAddStrategyPoolContractAssetLedgerEntryReq {
            strategy_pool_contract_id: strategy_pool_contract_row.pkey_id,
            token_address: strategy_pool_pending_wallet_trade.token_out.into(),
            blockchain,
            amount: token_out_amount,
            transaction_hash: pending_wallet_trade_receipt.transaction_hash.into(),
            is_add: true,
        })
        .await?;

    /* update per-user strategy pool contract asset balances & ledger */
    info!(
        "update_user_strategy_pool_asset_balances_on_copy_trade: strategy_pool_contract_id: {}, token_in: {}, token_out: {}",
        strategy_pool_contract_row.pkey_id, strategy_pool_pending_wallet_trade.token_in, strategy_pool_pending_wallet_trade.token_out
    );
    update_user_strategy_pool_asset_balances_on_copy_trade(
        &state.db,
        blockchain,
        strategy_pool_contract_row.pkey_id,
        strategy_pool_pending_wallet_trade.token_in,
        token_in_amount,
        sp_asset_token_in_previous_amount,
        strategy_pool_pending_wallet_trade.token_out,
        token_out_amount,
    )
    .await?;

    // TODO: multi-chain for loop ends here

    Ok(())
}

pub async fn handle_escrows(
    state: Arc<AppState>,
    body: Bytes,
    blockchain: EnumBlockChain,
) -> Result<(), StatusCode> {
    let hashes = parse_quickalert_payload(body).map_err(|e| {
        error!("failed to parse QuickAlerts payload: {:?}", e);
        StatusCode::BAD_REQUEST
    })?;

    for hash in hashes {
        let conn = state.eth_pool.get(blockchain).await.map_err(|err| {
            error!("error fetching connection guard: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        let state = state.clone();
        tokio::spawn(async move {
            /* the transactions from the quickalerts payload might not be yet mined */
            match wait_for_confirmations(
                &conn.eth(),
                hash,
                POLL_INTERVAL,
                MAX_RETRIES,
                CONFIRMATIONS,
            )
            .await
            {
                Ok(_) => {}
                Err(e) => {
                    error!("escrow tx was not mined: {:?}", e);
                    return;
                }
            }
            let tx = match TransactionFetcher::new_and_assume_ready(hash, &conn).await {
                Ok(tx) => tx,
                Err(e) => {
                    error!("error processing tx: {}", e);
                    return;
                }
            };
            if let Err(e) = evm::cache_ethereum_transaction(&tx, &state.db, blockchain).await {
                error!("error caching transaction: {:?}", e);
                return;
            };

            match handle_escrow_transaction(state.clone(), blockchain, tx).await {
                Ok(_) => {}
                Err(e) => {
                    error!("error handling escrow: {:?}", e);
                }
            }
        });
    }

    Ok(())
}

pub async fn handle_escrow_transaction(
    state: Arc<AppState>,
    blockchain: EnumBlockChain,
    tx: TransactionReady,
) -> Result<()> {
    /* check if it is an escrow to one of our escrow contracts */
    let escrow = parse_escrow_transfer(blockchain, &tx, &state.token_addresses)
        .with_context(|| format!("tx {:?} is not an escrow", tx.get_hash()))?;
    if state
        .escrow_addresses
        .get_by_address(blockchain, escrow.recipient)
        .is_none()
    {
        warn!(
            "no transfer to an escrow contract for tx: {:?}",
            tx.get_hash()
        );
        return Ok(());
    }

    /* check escrow has positive non-zero value */
    // TODO: minimum escrow value?
    if escrow.amount == U256::zero() {
        warn!("escrow amount is zero");
        return Ok(());
    }
    let escrow_transfer_token = state
        .db
        .execute(FunUserListEscrowTokenContractAddressReq {
            limit: 1,
            offset: 0,
            token_id: None,
            blockchain: Some(blockchain),
            address: Some(escrow.token_address.into()),
            symbol: None,
            is_stablecoin: None,
        })
        .await?
        .into_result()
        .with_context(|| {
            format!(
                "could not find token {} in escrow token contracts",
                escrow.token
            )
        })?;

    /* get caller */
    // TODO: handle an escrow made by an unknown user
    let caller = tx
        .get_from()
        .with_context(|| format!("no caller found for tx: {:?}", tx.get_hash()))?;

    /* get token address that was transferred */
    let called_address = tx
        .get_to()
        .with_context(|| format!("no called address found for tx: {:?}", tx.get_hash()))?;

    /* instantiate escrow contract */
    let conn = state.eth_pool.get(blockchain).await?;
    let escrow_contract = EscrowContract::new(
        conn.eth(),
        state
            .escrow_addresses
            .get(blockchain, ())
            .context("could not find escrow contract address on this chain")?,
    )?;
    /* check if transaction was made by a whitelisted wallet */
    let whitelisted_wallet = state
        .db
        .execute(FunUserListWhitelistedWalletsReq {
            limit: 1,
            offset: 0,
            user_id: None,
            blockchain: Some(blockchain),
            address: Some(caller.into()),
        })
        .await?
        .into_result();

    if whitelisted_wallet.is_none() {
        /* escrow was not done by a whitelisted wallet, return it minus fees */
        /* estimate gas of deposit rejection using dummy values */
        let estimated_refund_gas = escrow_contract
            .estimate_gas_reject_deposit(
                state.master_key.clone(),
                caller,
                called_address,
                escrow.amount.try_checked_sub(U256::one())?,
                state.master_key.address(),
                U256::one(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;
        let estimated_refund_fee = estimated_refund_gas.try_checked_mul(estimated_gas_price)?;
        let estimated_refund_fee_in_escrow_token = calculate_gas_fee_in_tokens(
            &state.cmc_client,
            &conn,
            &blockchain,
            called_address,
            estimated_refund_fee,
        )
        .await?;

        if estimated_refund_fee_in_escrow_token >= escrow.amount {
            // TODO: insert into a table non-registered tokens owned by the escrow contract
            info!(
                "estimated refund fee {:?} is greater than escrow amount {:?}, not refunding",
                estimated_refund_fee_in_escrow_token, escrow.amount
            );
            return Ok(());
        }

        let refund_amount = escrow
            .amount
            .try_checked_sub(estimated_refund_fee_in_escrow_token)?;

        // TODO: insert into a table tokens received by pending wallet
        /* run actual reject transaction and transfer estimated fee to pending wallet */
        let reject_deposit_transaction = || {
            escrow_contract.reject_deposit(
                &conn,
                state.master_key.clone(),
                caller,
                called_address,
                refund_amount,
                state.master_key.address(),
                estimated_refund_fee_in_escrow_token,
                DynLogger::empty(),
            )
        };

        execute_transaction_and_ensure_success(
            reject_deposit_transaction,
            &conn,
            CONFIRMATIONS,
            MAX_RETRIES,
            POLL_INTERVAL,
            &DynLogger::empty(),
        )
        .await?;

        info!(
            "escrow was not done by a whitelisted wallet, refunded {:?} tokens to {:?}",
            refund_amount, caller
        );

        return Ok(());
    }

    /* deposit was done by a whitelisted wallet, write it to contract and database */
    let user = match state
        .db
        .execute(FunUserGetUserByAddressReq {
            address: caller.into(),
            blockchain,
        })
        .await?
        .into_result()
    {
        Some(user) => user,
        None => {
            info!("no user has address: {:?}", caller);
            return Ok(());
        }
    };

    /* write deposit to escrow contract */
    let accept_deposit_transaction = || {
        escrow_contract.accept_deposit(
            &conn,
            state.master_key.clone(),
            caller,
            called_address,
            escrow.amount,
            DynLogger::empty(),
        )
    };

    execute_transaction_and_ensure_success(
        accept_deposit_transaction,
        &conn,
        CONFIRMATIONS,
        MAX_RETRIES,
        POLL_INTERVAL,
        &DynLogger::empty(),
    )
    .await?;

    /* insert escrow in ledger */

    state
        .db
        .execute(FunUserAddUserDepositWithdrawLedgerEntryReq {
            user_id: user.user_id,
            token_address: called_address.into(),
            blockchain,
            user_address: escrow.owner.into(),
            escrow_contract_address: escrow.recipient.into(),
            receiver_address: escrow.recipient.into(),
            quantity: u256_to_decimal(escrow.amount, escrow_transfer_token.decimals as _),
            transaction_hash: tx.get_hash().into(),
            is_deposit: true,
            is_back: false,
            is_withdraw: false,
        })
        .await?;
    let old_balance = state
        .db
        .execute(FunUserListUserDepositWithdrawBalanceReq {
            limit: Some(1),
            offset: None,
            user_id: user.user_id,
            user_address: Some(escrow.owner.into()),
            blockchain: Some(blockchain),
            token_address: Some(called_address.into()),
            token_id: None,
            escrow_contract_address: Some(escrow.recipient.into()),
        })
        .await?
        .into_result()
        .map(|x| x.balance)
        .unwrap_or_default();
    let new_balance =
        old_balance + u256_to_decimal(escrow.amount, escrow_transfer_token.decimals as _);
    let resp = state
        .db
        .execute(FunWatcherUpsertUserDepositWithdrawBalanceReq {
            user_id: user.user_id,
            user_address: escrow.owner.into(),
            blockchain,
            old_balance,
            new_balance,
            token_address: called_address.into(),
            escrow_contract_address: escrow.recipient.into(),
        })
        .await?;

    if let Some(admin_client) = state.admin_client.as_ref() {
        if let Err(err) = admin_client
            .lock()
            .await
            .request(AdminNotifyEscrowLedgerChangeRequest {
                pkey_id: 0,
                user_id: user.user_id,
                balance: UserListDepositLedgerRow {
                    transaction_id: resp
                        .first(|x| x.ret_pkey_id)
                        .unwrap_or_else(|| Utc::now().timestamp()),
                    quantity: u256_to_decimal(escrow.amount, escrow_transfer_token.decimals as _),
                    blockchain,
                    user_address: escrow.owner.into(),
                    contract_address: called_address.into(),
                    transaction_hash: tx.get_hash().into(),
                    is_deposit: true,
                    receiver_address: escrow.recipient.into(),

                    happened_at: Utc::now().timestamp(),
                },
            })
            .await
        {
            error!("error notifying admin of escrow ledger change: {:?}", err);
        }
    }
    Ok(())
}

pub async fn calculate_gas_fee_in_tokens(
    cmc: &CoinMarketCap,
    conn: &EthereumRpcConnection,
    blockchain: &EnumBlockChain,
    token_address: Address,
    total_gas_fee_in_wei: U256,
) -> Result<U256> {
    let token_contract = Erc20Token::new(conn.clone(), token_address)?;
    let token_decimals = token_contract.decimals().await?;
    let token_symbol = token_contract.symbol().await?;

    let native_symbol = match blockchain {
        EnumBlockChain::EthereumMainnet => "ETH",
        EnumBlockChain::EthereumGoerli => "ETH",
        EnumBlockChain::EthereumSepolia => "ETH",
        EnumBlockChain::BscMainnet => "BNB",
        EnumBlockChain::BscTestnet => "BNB",
        _ => bail!("unsupported blockchain"),
    };

    /* get token value of 1 native token */
    let native_price = cmc
        .get_quote_price_by_symbol(native_symbol.to_string(), token_symbol)
        .await?;

    let gas_fee_in_tokens = total_gas_fee_in_wei
        /* native price doesn't consider decimals */
        .mul_f64(native_price)?
        /* so multiply native price without decimals by proportion the token decimals take in native decimals */
        .mul_div(U256::exp10(token_decimals.as_usize()), U256::exp10(18))?;

    Ok(gas_fee_in_tokens)
}

pub async fn handle_withdraws(
    state: Arc<AppState>,
    body: Bytes,
    blockchain: EnumBlockChain,
) -> Result<(), StatusCode> {
    let hashes = parse_quickalert_payload(body).map_err(|e| {
        error!("failed to parse QuickAlerts payload: {:?}", e);
        StatusCode::BAD_REQUEST
    })?;

    for hash in hashes {
        let conn = state.eth_pool.get(blockchain).await.map_err(|err| {
            error!("error fetching connection guard: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        let state = state.clone();
        tokio::spawn(async move {
            /* the transactions from the quickalerts payload might not be yet mined */
            match wait_for_confirmations_simple(&conn.eth(), hash, Duration::from_secs(10), 10)
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    error!("withdraw tx was not mined: {:?}", e);
                    return;
                }
            }
            // TODO: wait for confirmations blocks before processing to properly handle ommer blocks & reorgs
            let tx = match TransactionFetcher::new_and_assume_ready(hash, &conn).await {
                Ok(tx) => tx,
                Err(err) => {
                    error!("error processing tx: {:?}", err);
                    return;
                }
            };
            if let Err(e) = evm::cache_ethereum_transaction(&tx, &state.db, blockchain).await {
                error!("error caching transaction: {:?}", e);
            };
            match handle_withdraw_transaction(state.clone(), blockchain, tx).await {
                Ok(_) => {}
                Err(e) => {
                    error!("error handling withdraw: {:?}", e);
                }
            }
        });
    }

    Ok(())
}

pub async fn handle_withdraw_transaction(
    state: Arc<AppState>,
    blockchain: EnumBlockChain,
    tx: TransactionReady,
) -> Result<()> {
    /* parse withdraw event, check it was emitted by the escrow contract */
    let escrow_contract_address = state
        .escrow_addresses
        .get(blockchain, ())
        .context("could not find escrow contract address on withdraw handler")?;
    let withdraw_event =
        parse_escrow_withdraw_event(escrow_contract_address, tx.get_receipt().clone())?;

    /* get user */
    let user = match state
        .db
        .execute(FunUserGetUserByAddressReq {
            address: withdraw_event.proprietor.into(),
            blockchain,
        })
        .await?
        .into_result()
    {
        Some(user) => user,
        None => {
            info!("no user has address: {:?}", withdraw_event.proprietor);
            return Ok(());
        }
    };
    let withdraw_token = state
        .db
        .execute(FunUserListEscrowTokenContractAddressReq {
            limit: 1,
            offset: 0,
            token_id: None,
            blockchain: Some(blockchain),
            address: Some(withdraw_event.asset.into()),
            symbol: None,
            is_stablecoin: None,
        })
        .await?
        .into_result()
        .with_context(|| {
            format!(
                "could not find token {} in escrow token contracts",
                withdraw_event.asset
            )
        })?;
    let withdraw_amount = u256_to_decimal(withdraw_event.amount, withdraw_token.decimals as _);
    /* update user deposit withdraw balance & ledger */
    state
        .db
        .execute(FunUserAddUserDepositWithdrawLedgerEntryReq {
            user_id: user.user_id,
            quantity: withdraw_amount,
            blockchain,
            user_address: withdraw_event.proprietor.into(),
            token_address: withdraw_event.asset.into(),
            escrow_contract_address: escrow_contract_address.into(),
            transaction_hash: tx.get_hash().into(),
            receiver_address: withdraw_event.proprietor.into(),
            is_deposit: false,
            is_back: false,
            is_withdraw: true,
        })
        .await
        .context("error inserting withdraw in ledger")?;

    let old_balance = state
        .db
        .execute(FunUserListUserDepositWithdrawBalanceReq {
            limit: Some(1),
            offset: None,
            user_id: user.user_id,
            user_address: Some(withdraw_event.proprietor.into()),
            blockchain: Some(blockchain),
            token_address: Some(withdraw_event.asset.into()),
            token_id: None,
            escrow_contract_address: Some(escrow_contract_address.into()),
        })
        .await?
        .into_result()
        .map(|x| x.balance)
        .unwrap_or_default();
    let new_balance = old_balance - withdraw_amount;
    state
        .db
        .execute(FunWatcherUpsertUserDepositWithdrawBalanceReq {
            user_id: user.user_id,
            user_address: withdraw_event.proprietor.into(),
            blockchain,
            old_balance,
            new_balance,
            token_address: withdraw_event.asset.into(),
            escrow_contract_address: escrow_contract_address.into(),
        })
        .await?;

    Ok(())
}

pub async fn handle_redeems(
    state: Arc<AppState>,
    body: Bytes,
    blockchain: EnumBlockChain,
) -> Result<(), StatusCode> {
    let hashes = parse_quickalert_payload(body).map_err(|e| {
        error!("failed to parse QuickAlerts payload: {:?}", e);
        StatusCode::BAD_REQUEST
    })?;

    for hash in hashes {
        let conn = state.eth_pool.get(blockchain).await.map_err(|err| {
            error!("error fetching connection guard: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        let state = state.clone();
        tokio::spawn(async move {
            /* the transactions from the quickalerts payload might not be yet mined */
            match wait_for_confirmations_simple(&conn.eth(), hash, Duration::from_secs(10), 10)
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    error!("redeem tx was not mined: {:?}", e);
                    return;
                }
            }
            // TODO: wait for confirmations blocks before processing to properly handle ommer blocks & reorgs
            let tx = match TransactionFetcher::new_and_assume_ready(hash, &conn).await {
                Ok(tx) => tx,
                Err(err) => {
                    error!("error processing tx: {:?}", err);
                    return;
                }
            };
            if let Err(e) = evm::cache_ethereum_transaction(&tx, &state.db, blockchain).await {
                error!("error caching transaction: {:?}", e);
            };
            match handle_redeem_transaction(state.clone(), blockchain, tx).await {
                Ok(_) => {}
                Err(e) => {
                    error!("error handling redeem: {:?}", e);
                }
            }
        });
    }

    Ok(())
}

pub async fn handle_redeem_transaction(
    state: Arc<AppState>,
    blockchain: EnumBlockChain,
    tx: TransactionReady,
) -> Result<()> {
    /* parse redeem event */
    let herald_contract_address = state
        .pool_herald_addresses
        .get(blockchain, ())
        .context("could not find herald contract address on redeem handler")?;
    let redeem_event =
        parse_strategy_pool_herald_redeem_event(herald_contract_address, tx.get_receipt().clone())?;

    /* check if event was triggered by a strategy pool contract */
    let maybe_strategy_pool_contract_row = state
        .db
        .execute(FunWatcherListStrategyPoolContractReq {
            limit: 1,
            offset: 0,
            strategy_id: None,
            blockchain: Some(blockchain),
            address: Some(redeem_event.strategy_pool.into()),
        })
        .await?
        .into_result();

    if maybe_strategy_pool_contract_row.is_none() {
        info!(
            "redeem event read, but no strategy pool contract has address {:?} on chain {:?}",
            redeem_event.strategy_pool, blockchain
        );
        return Ok(());
    }

    let strategy_pool_contract_row = maybe_strategy_pool_contract_row.unwrap();

    /* instantiate strategy wallet */
    let strategy_wallet_contract_row = state
        .db
        .execute(FunUserListStrategyWalletsReq {
            user_id: None,
            blockchain: Some(blockchain),
            strategy_wallet_address: Some(redeem_event.strategy_wallet.into()),
        })
        .await?
        .into_result()
        .context("user has no strategy wallet on this chain")?;

    /* get user strategy pool contract assets owned by this strategy wallet */
    let asset_balances_owned_by_strategy_wallet = state
        .db
        .execute(FunUserListUserStrategyPoolContractAssetBalancesReq {
            strategy_pool_contract_id: Some(strategy_pool_contract_row.pkey_id),
            user_id: Some(strategy_wallet_contract_row.user_id),
            strategy_wallet_id: Some(strategy_wallet_contract_row.wallet_id),
            token_address: None,
            blockchain: Some(blockchain),
        })
        .await?
        .into_rows();

    /* get user strategy token balance */
    let user_strategy_balance = state
        .db
        .execute(FunWatcherListUserStrategyBalanceReq {
            limit: 1,
            offset: 0,
            strategy_id: Some(strategy_pool_contract_row.strategy_id),
            user_id: Some(strategy_wallet_contract_row.user_id),
            blockchain: Some(blockchain),
        })
        .await?
        .first(|x| x.balance)
        .unwrap_or_default();

    /* calculate how much of owned assets to withdraw based on how much of strategy tokens were redeemed */
    let mut assets_to_transfer: Vec<Address> = Vec::new();
    let mut amounts_to_transfer: Vec<Decimal> = Vec::new();
    let mut amounts_to_transfer_raw: Vec<U256> = Vec::new();
    for asset_balance_owned_by_strategy_wallet in asset_balances_owned_by_strategy_wallet {
        if asset_balance_owned_by_strategy_wallet.balance.is_zero() {
            continue;
        }

        assets_to_transfer.push(asset_balance_owned_by_strategy_wallet.token_address.into());
        let amount_owned = asset_balance_owned_by_strategy_wallet.balance;
        let amount = amount_owned
            * u256_to_decimal(
                redeem_event.amount,
                asset_balance_owned_by_strategy_wallet.token_decimals as _,
            )
            / user_strategy_balance;
        amounts_to_transfer.push(amount);
        amounts_to_transfer_raw.push(decimal_to_u256(
            amount,
            asset_balance_owned_by_strategy_wallet.token_decimals as _,
        ));
    }

    /* instantiate strategy pool contract wrapper */
    let conn = state.eth_pool.get(blockchain).await?;
    let strategy_pool_contract =
        StrategyPoolContract::new(conn.clone(), strategy_pool_contract_row.address.into())?;

    /* check if strategy pool is trading */
    // TODO: cache this, and withdraw later if strategy pool started trading
    if strategy_pool_contract.is_paused().await? {
        bail!("strategy pool started trading between redeem and withdraw");
    }

    /* withdraw assets to user wallet address registered in strategy wallet */
    // TODO: get logger from main
    let logger = DynLogger::new(Arc::new(move |msg| {
        println!("{}", msg);
    }));

    let withdraw_transaction = || {
        strategy_pool_contract.withdraw(
            &conn,
            state.master_key.clone(),
            redeem_event.backer,
            assets_to_transfer.clone(),
            amounts_to_transfer_raw.clone(),
            logger.clone(),
        )
    };

    let withdraw_transaction_hash = execute_transaction_and_ensure_success(
        withdraw_transaction,
        &conn,
        CONFIRMATIONS,
        MAX_RETRIES,
        POLL_INTERVAL,
        &logger,
    )
    .await?;

    /* update user strategy token balance & ledger */
    let redeem_amount = u256_to_decimal(redeem_event.amount, 18);
    update_strategy_token_balances_and_ledger_exit_strategy(
        &state.db,
        blockchain,
        strategy_pool_contract_row.strategy_id,
        strategy_wallet_contract_row.user_id,
        tx.get_hash(),
        redeem_amount,
    )
    .await?;

    /* update strategy pool assets balance & ledger */
    update_asset_balances_and_ledger_exit_strategy(
        &state.db,
        blockchain,
        strategy_pool_contract_row.strategy_id,
        strategy_pool_contract_row.pkey_id,
        strategy_wallet_contract_row.user_id,
        strategy_wallet_contract_row.wallet_id,
        withdraw_transaction_hash,
        assets_to_transfer,
        amounts_to_transfer,
    )
    .await?;

    Ok(())
}

pub async fn handle_revoke_adminships(
    state: Arc<AppState>,
    body: Bytes,
    blockchain: EnumBlockChain,
) -> Result<(), StatusCode> {
    let hashes = parse_quickalert_payload(body).map_err(|e| {
        error!("failed to parse QuickAlerts payload: {:?}", e);
        StatusCode::BAD_REQUEST
    })?;

    for hash in hashes {
        let conn = state.eth_pool.get(blockchain).await.map_err(|err| {
            error!("error fetching connection guard: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        let state = state.clone();
        tokio::spawn(async move {
            /* the transactions from the quickalerts payload might not be yet mined */
            match wait_for_confirmations(
                &conn.eth(),
                hash,
                POLL_INTERVAL,
                MAX_RETRIES,
                CONFIRMATIONS,
            )
            .await
            {
                Ok(_) => {}
                Err(e) => {
                    error!("revoke adminship tx was not mined: {:?}", e);
                    return;
                }
            }
            let tx = match TransactionFetcher::new_and_assume_ready(hash, &conn).await {
                Ok(tx) => tx,
                Err(e) => {
                    error!("error processing tx: {}", e);
                    return;
                }
            };
            if let Err(e) = evm::cache_ethereum_transaction(&tx, &state.db, blockchain).await {
                error!("error caching transaction: {:?}", e);
                return;
            };

            match handle_revoke_adminship_transaction(state.clone(), blockchain, tx).await {
                Ok(_) => {}
                Err(e) => {
                    error!("error handling revoke adminship: {:?}", e);
                }
            }
        });
    }

    Ok(())
}

pub async fn handle_revoke_adminship_transaction(
    state: Arc<AppState>,
    blockchain: EnumBlockChain,
    tx: TransactionReady,
) -> Result<()> {
    /* parse revoke adminship event */
    let herald_contract_address = state
        .wallet_herald_addresses
        .get(blockchain, ())
        .context("could not find herald contract address on revoke adminship handler")?;
    let revoke_adminship_event =
        parse_strategy_pool_herald_redeem_event(herald_contract_address, tx.get_receipt().clone())?;

    /* check if event was triggered by a strategy wallet contract */
    let maybe_strategy_wallet_row = state
        .db
        .execute(FunUserListStrategyWalletsReq {
            user_id: None,
            strategy_wallet_address: Some(revoke_adminship_event.strategy_wallet.into()),
            blockchain: Some(blockchain),
        })
        .await?
        .into_result();

    if maybe_strategy_wallet_row.is_none() {
        info!(
            "revoke adminship event read, but no strategy wallet has address {:?} on chain {:?}",
            revoke_adminship_event.strategy_wallet, blockchain
        );
        return Ok(());
    }

    let strategy_wallet_row = maybe_strategy_wallet_row.unwrap();

    /* update strategy wallet platform management */
    state
        .db
        .execute(FunWatcherUpdateStrategyWalletPlatformManagementReq {
            strategy_wallet_id: strategy_wallet_row.wallet_id,
            is_platform_managed: false,
        })
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use eth_sdk::EthereumRpcConnectionPool;
    use std::println;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_calculate_gas_amount_in_token() -> Result<()> {
        let cmc = CoinMarketCap::new_debug_key().unwrap();
        let eth_pool = EthereumRpcConnectionPool::new();
        let conn = eth_pool.get(EnumBlockChain::EthereumMainnet).await?;
        let usdc_address_in_ethereum =
            Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")?;
        // 0.001 ETH or 1000000000000000 WEI
        let total_gas_fee = U256::from_dec_str("1000000000000000")?;
        let gas_fee_in_tokens = calculate_gas_fee_in_tokens(
            &cmc,
            &conn,
            &EnumBlockChain::EthereumMainnet,
            usdc_address_in_ethereum,
            total_gas_fee,
        )
        .await?;
        // divide the ETH current price in USDC by 1000, and this should be the result
        // with 6 extra digits of decimals (USDC decimals on Ethereum)
        // e.g. ETH price is 1k USDC, gas fee in USDC is 1, with decimals that is 1000000
        println!("gas_fee_in_tokens: {:?}", gas_fee_in_tokens);
        Ok(())
    }
}
