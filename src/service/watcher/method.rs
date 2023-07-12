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
use eth_sdk::erc20::{approve_and_ensure_success, Erc20Token};
use eth_sdk::escrow::{
    accept_deposit_and_ensure_success, reject_deposit_and_ensure_success, EscrowContract,
};
use eth_sdk::escrow_tracker::escrow::parse_escrow;
use eth_sdk::evm::parse_quickalert_payload;
use eth_sdk::strategy_pool::{
    acquire_asset_before_trade_and_ensure_success, give_back_assets_after_trade_and_ensure_success,
    StrategyPoolContract,
};
use eth_sdk::utils::{wait_for_confirmations, wait_for_confirmations_simple};
use eth_sdk::v3::smart_router::{copy_trade_and_ensure_success, PancakeSmartRouterV3Contract};
use eth_sdk::{
    evm, EthereumRpcConnection, ScaledMath, TransactionFetcher, TransactionReady, CONFIRMATIONS,
    MAX_RETRIES, POLL_INTERVAL,
};
use eyre::*;
use gen::database::*;
use gen::model::*;
use lib::log::DynLogger;
use std::sync::Arc;
use std::time::Duration;
use tracing::*;
use web3::ethabi::Address;
use web3::signing::Key;
use web3::types::U256;

pub async fn handle_ethereum_dex_transactions(
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
            match handle_pancake_swap_transaction(state.clone(), blockchain, tx).await {
                Ok(_) => {}
                Err(e) => {
                    error!("error handling swap: {:?}", e);
                }
            }
        });
    }

    Ok(())
}

pub async fn handle_pancake_swap_transaction(
    state: Arc<AppState>,
    blockchain: EnumBlockChain,
    tx: TransactionReady,
) -> Result<()> {
    /* check if caller is a strategy watching wallet & get strategy id */
    let caller = tx.get_from().context("no from address found")?;
    let strategy_id =
        match get_strategy_id_from_watching_wallet(&state.db, blockchain, caller).await? {
            Some(strategy_id) => strategy_id,
            None => {
                /* caller is not a strategy watching wallet */
                return Ok(());
            }
        };
    let conn = state.eth_pool.get(blockchain).await?;
    /* parse trade */
    let expert_trade =
        parse_dex_trade(blockchain, &tx, &state.dex_addresses, &state.pancake_swap).await?;

    /* update last dex trade cache table */
    state
        .db
        .execute(FunWatcherUpsertLastDexTradeForPairReq {
            transaction_hash: tx.get_hash().into(),
            blockchain: blockchain,
            dex: EnumDex::PancakeSwap,
            token_in_address: expert_trade.token_in.into(),
            token_out_address: expert_trade.token_out.into(),
            amount_in: expert_trade.amount_in.into(),
            amount_out: expert_trade.amount_out.into(),
        })
        .await?;

    /* instantiate token_in and token_out contracts from expert's trade */
    let token_in_contract = Erc20Token::new(conn.clone(), expert_trade.token_in)?;
    let token_out_contract = Erc20Token::new(conn.clone(), expert_trade.token_out)?;

    /* get expert wallet token_in asset balance prior to the trade */
    let expert_wallet_asset_token_in_previous_amount = state
        .db
        .execute(FunWatcherGetExpertWalletAssetsFromLedgerReq {
            strategy_id,
            blockchain: Some(blockchain),
            symbol: Some(token_in_contract.symbol().await?)
        })
        .await?
        .into_result()
        .context("sold asset was not previously an expert wallet asset, can't calculate trade token_in allocation for strategy pool")?
        .amount;

    /* update wallet activity ledger & make sure this transaction is not a duplicate */
    let saved = state
        .db
        .execute(FunWatcherSaveStrategyWatchingWalletTradeLedgerReq {
            address: caller.clone().into(),
            transaction_hash: tx.get_hash().into(),
            blockchain,
            contract_address: expert_trade.contract.into(),
            dex: Some(EnumDex::PancakeSwap.to_string()),
            token_in_address: Some(expert_trade.token_in.into()),
            token_out_address: Some(expert_trade.token_out.into()),
            amount_in: Some(expert_trade.amount_in.into()),
            amount_out: Some(expert_trade.amount_out.into()),
            happened_at: None,
        })
        .await
        .context("swap transaction is a duplicate")?
        .into_result();
    if let Some(saved) = saved {
        /* update expert wallet's asset balances in the database */
        let conn = state.eth_pool.get(blockchain).await?;
        update_expert_listened_wallet_asset_balance_cache(
            &state.db,
            strategy_id,
            &expert_trade,
            saved.fkey_token_out,
            saved.fkey_token_in,
            blockchain,
        )
        .await?;

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

        let sp_asset_token_in_previous_amount = strategy_pool_asset_token_in_row.balance.into();
        if sp_asset_token_in_previous_amount == U256::zero() {
            bail!("strategy pool has no asset to sell");
        }
        /* calculate how much to spend */
        let corrected_amount_in =
            if expert_trade.amount_in > *expert_wallet_asset_token_in_previous_amount {
                /* if the traded amount_in is larger than the total amount of token_in we know of the strategy,
                     it means that the trader has acquired tokens from sources we have not read
                     if we used an amount_in that is larger in the calculation, it would make amount_to_spend
                     larger than the amount of token_in in the strategy pool contract, which would revert the transaction
                     so we use the total amount of token_in we know of the strategy,
                     which will result in a trade of all the strategy pool balance of this asset
                */
                *expert_wallet_asset_token_in_previous_amount
            } else {
                expert_trade.amount_in
            };
        let amount_to_spend = corrected_amount_in.mul_div(
            sp_asset_token_in_previous_amount,
            *expert_wallet_asset_token_in_previous_amount,
        )?;
        if amount_to_spend == U256::zero() {
            bail!("spent ratio is too small to be represented in amount of token_in owned by strategy pool contract");
        }

        /* instantiate pancake swap contract */
        let pancake_contract = PancakeSmartRouterV3Contract::new(
            conn.clone(),
            state
                .dex_addresses
                .get(blockchain, EnumDex::PancakeSwap)
                .ok_or_else(|| eyre!("pancake swap not available on this chain"))?,
        )?;

        acquire_asset_before_trade_and_ensure_success(
            strategy_pool_contract.clone(),
            &conn,
            CONFIRMATIONS,
            MAX_RETRIES,
            POLL_INTERVAL,
            state.master_key.clone(),
            expert_trade.token_in,
            amount_to_spend,
        )
        .await?;

        /* approve pancakeswap to trade token_in */
        approve_and_ensure_success(
            token_in_contract,
            &conn,
            CONFIRMATIONS,
            MAX_RETRIES,
            POLL_INTERVAL,
            state.master_key.clone(),
            pancake_contract.address(),
            amount_to_spend,
            DynLogger::empty(),
        )
        .await?;

        /* trade token_in for token_out */
        let pending_wallet_trade_receipt = copy_trade_and_ensure_success(
            pancake_contract,
            &conn,
            CONFIRMATIONS,
            MAX_RETRIES,
            POLL_INTERVAL,
            state.master_key.clone(),
            &expert_trade.get_pancake_pair_paths()?,
            amount_to_spend,
            U256::from(1),
            DynLogger::empty(),
        )
        .await?;

        /* parse trade to find amount_out */
        let strategy_pool_pending_wallet_trade = parse_dex_trade(
            blockchain,
            &TransactionFetcher::new_and_assume_ready(
                pending_wallet_trade_receipt.transaction_hash,
                &conn,
            )
            .await?,
            &state.dex_addresses,
            &state.pancake_swap,
        )
        .await?;

        /* approve strategy pool for amount_out */
        approve_and_ensure_success(
            token_out_contract,
            &conn,
            CONFIRMATIONS,
            MAX_RETRIES,
            POLL_INTERVAL,
            state.master_key.clone(),
            strategy_pool_contract.address(),
            strategy_pool_pending_wallet_trade.amount_out,
            DynLogger::empty(),
        )
        .await?;

        /* give back traded assets */
        give_back_assets_after_trade_and_ensure_success(
            strategy_pool_contract,
            &conn,
            CONFIRMATIONS,
            MAX_RETRIES,
            POLL_INTERVAL,
            state.master_key.clone(),
            vec![expert_trade.token_out],
            vec![strategy_pool_pending_wallet_trade.amount_out],
        )
        .await?;

        /* update strategy pool contract asset balances & ledger */
        state
            .db
            .execute(FunWatcherUpsertStrategyPoolContractAssetBalanceReq {
                strategy_pool_contract_id: strategy_pool_contract_row.pkey_id,
                token_address: expert_trade.token_in.into(),
                blockchain,
                new_balance: match sp_asset_token_in_previous_amount
                    .try_checked_sub(amount_to_spend)
                {
                    Ok(new_balance) => new_balance,
                    Err(_) => U256::zero(),
                }
                .into(),
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
        let strategy_pool_asset_token_out_new_balance =
            match maybe_strategy_pool_asset_token_out_row {
                Some(token_out) => (*token_out.balance)
                    .try_checked_add(strategy_pool_pending_wallet_trade.amount_out)?,
                None => strategy_pool_pending_wallet_trade.amount_out,
            };

        state
            .db
            .execute(FunWatcherUpsertStrategyPoolContractAssetBalanceReq {
                strategy_pool_contract_id: strategy_pool_contract_row.pkey_id,
                token_address: expert_trade.token_out.into(),
                blockchain,
                new_balance: strategy_pool_asset_token_out_new_balance.into(),
            })
            .await?;

        state
            .db
            .execute(FunUserAddStrategyPoolContractAssetLedgerEntryReq {
                strategy_pool_contract_id: strategy_pool_contract_row.pkey_id,
                token_address: strategy_pool_pending_wallet_trade.token_in.into(),
                blockchain: blockchain,
                amount: strategy_pool_pending_wallet_trade.amount_in.into(),
                transaction_hash: pending_wallet_trade_receipt.transaction_hash.into(),
                is_add: false,
            })
            .await?;

        state
            .db
            .execute(FunUserAddStrategyPoolContractAssetLedgerEntryReq {
                strategy_pool_contract_id: strategy_pool_contract_row.pkey_id,
                token_address: strategy_pool_pending_wallet_trade.token_out.into(),
                blockchain: blockchain,
                amount: strategy_pool_pending_wallet_trade.amount_out.into(),
                transaction_hash: pending_wallet_trade_receipt.transaction_hash.into(),
                is_add: true,
            })
            .await?;

        /* update per-user strategy pool contract asset balances & ledger */
        update_user_strategy_pool_asset_balances_on_copy_trade(
            &state.db,
            blockchain,
            strategy_pool_contract_row.pkey_id,
            strategy_pool_pending_wallet_trade.token_in,
            strategy_pool_pending_wallet_trade.amount_in,
            sp_asset_token_in_previous_amount,
            strategy_pool_pending_wallet_trade.token_out,
            strategy_pool_pending_wallet_trade.amount_out,
            pending_wallet_trade_receipt.transaction_hash,
        )
        .await?;

        // TODO: multi-chain for loop ends here
    }

    Ok(())
}

pub async fn handle_eth_escrows(
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
    let escrow = parse_escrow(blockchain, &tx, &state.token_addresses, &state.erc_20)
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
        reject_deposit_and_ensure_success(
            escrow_contract,
            &conn,
            CONFIRMATIONS,
            MAX_RETRIES,
            POLL_INTERVAL,
            state.master_key.clone(),
            caller,
            called_address,
            refund_amount,
            state.master_key.address(),
            estimated_refund_fee_in_escrow_token,
            DynLogger::empty(),
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
    accept_deposit_and_ensure_success(
        escrow_contract,
        &conn,
        CONFIRMATIONS,
        MAX_RETRIES,
        POLL_INTERVAL,
        state.master_key.clone(),
        caller,
        called_address,
        escrow.amount,
        DynLogger::empty(),
    )
    .await?;

    /* insert escrow in ledger */
    state
        .db
        .execute(FunWatcherSaveUserDepositWithdrawLedgerReq {
            user_id: user.user_id,
            quantity: escrow.amount.into(),
            blockchain,
            user_address: escrow.owner.into(),
            contract_address: called_address.into(),
            transaction_hash: tx.get_hash().into(),
            receiver_address: escrow.recipient.into(),
        })
        .await
        .context("error inserting escrow in ledger")?;

    let old_balance: U256 = state
        .db
        .execute(FunUserListUserDepositWithdrawBalanceReq {
            limit: 1,
            offset: 0,
            user_id: user.user_id,
            blockchain: Some(blockchain),
            token_address: Some(called_address.into()),
            token_id: None,
            escrow_contract_address: Some(escrow.recipient.into()),
        })
        .await?
        .into_result()
        .map(|x| x.balance)
        .unwrap_or_default()
        .into();
    let new_balance = old_balance.try_checked_add(escrow.amount)?;
    let resp = state
        .db
        .execute(FunWatcherUpsertUserDepositWithdrawBalanceReq {
            user_id: user.user_id,
            blockchain,
            old_balance: old_balance.into(),
            new_balance: new_balance.into(),
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
                    quantity: escrow.amount.into(),
                    blockchain,
                    user_address: escrow.owner.into(),
                    contract_address: called_address.into(),
                    transaction_hash: tx.get_hash().into(),
                    is_deposit: false,
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

#[cfg(test)]
mod tests {
    use super::*;
    use eth_sdk::EthereumRpcConnectionPool;
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
