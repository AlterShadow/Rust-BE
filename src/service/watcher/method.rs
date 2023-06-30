use crate::AppState;
use axum::http::StatusCode;
use bytes::Bytes;
use chrono::Utc;
use eth_sdk::dex_tracker::{
    get_strategy_id_from_watching_wallet, parse_dex_trade,
    update_expert_listened_wallet_asset_ledger,
};
use eth_sdk::erc20::{approve_and_ensure_success, Erc20Token};
use eth_sdk::escrow_tracker::escrow::parse_escrow;
use eth_sdk::evm::parse_quickalert_payload;
use eth_sdk::strategy_pool::{
    acquire_asset_before_trade_and_ensure_success, give_back_assets_after_trade_and_ensure_success,
    StrategyPoolContract,
};
use eth_sdk::utils::{wait_for_confirmations, wait_for_confirmations_simple};
use eth_sdk::v3::smart_router::{copy_trade_and_ensure_success, PancakeSmartRouterV3Contract};
use eth_sdk::{
    evm, ScaledMath, TransactionFetcher, TransactionReady, CONFIRMATIONS, MAX_RETRIES,
    POLL_INTERVAL,
};
use eyre::*;
use gen::database::*;
use gen::model::*;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, trace, warn};
use web3::ethabi::Address;
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
    let trade = parse_dex_trade(blockchain, &tx, &state.dex_addresses, &state.pancake_swap).await?;

    /* instantiate token_in and token_out contracts */
    let token_in_contract = Erc20Token::new(conn.clone(), trade.token_in)?;
    let token_out_contract = Erc20Token::new(conn.clone(), trade.token_out)?;

    /* get strategy tokens prior to the trade */
    let strategy_token_in_previous_amount = state
        .db
        .execute(FunWatcherGetStrategyTokensFromLedgerReq {
            strategy_id,
            blockchain: Some(blockchain),
            symbol: Some(token_in_contract.symbol().await?)
        })
        .await?
        .into_result()
        .context("sold token was not previously owned by strategy, there is no way to calculate the trade")?
        .amount;

    /* update wallet activity ledger & make sure this transaction is not a duplicate */
    let saved = state
        .db
        .execute(FunWatcherSaveStrategyWatchingWalletTradeLedgerReq {
            address: caller.clone().into(),
            transaction_hash: tx.get_hash().into(),
            blockchain,
            contract_address: trade.contract.into(),
            dex: Some(EnumDex::PancakeSwap.to_string()),
            token_in_address: Some(trade.token_in.into()),
            token_out_address: Some(trade.token_out.into()),
            amount_in: Some(trade.amount_in.into()),
            amount_out: Some(trade.amount_out.into()),
            happened_at: None,
        })
        .await
        .context("swap transaction is a duplicate")?
        .into_result();
    // TODO: update strategy_watching_wallet_trade_balance
    if let Some(saved) = saved {
        /* update database with watched wallet's tokens */
        let conn = state.eth_pool.get(blockchain).await?;
        update_expert_listened_wallet_asset_ledger(
            &state.db,
            strategy_id,
            &trade,
            saved.fkey_token_out,
            saved.fkey_token_in,
            blockchain,
        )
        .await?;

        /* if token_in was already a strategy token trade it from SPs in ratio traded_amount / old_amount */
        let strategy_pool = state
            .db
            .execute(FunWatcherListStrategyPoolContractReq {
                limit: 1,
                offset: 0,
                strategy_id: Some(strategy_id),
                blockchain: Some(blockchain),
                address: None,
            })
            .await?
            .into_result();

        /* if there is an SP contract for this strategy,  */
        if let Some(address_row) = strategy_pool {
            let address: Address = address_row.address.into();
            /* check if SP contract holds token_in */
            let sp_contract = StrategyPoolContract::new(conn.clone(), address)?;
            // TODO: we want to save the balance to database, not from on-chain
            // TODO: save this sp_asset_token_in_amount to database somewhere
            // FIXME: there is no trades happening on the strategy pool contract, so before == after
            let sp_asset_token_in = state
                .db
                .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
                    strategy_pool_contract_id: address_row.pkey_id,
                    token_address: Some(trade.token_in.into()),
                    blockchain: Some(blockchain),
                })
                .await?
                .into_result()
                .context("strategy pool contract does not hold asset to sell")?;

            let sp_asset_token_in_amount = sp_asset_token_in.balance.into();
            if sp_asset_token_in_amount == U256::zero() {
                bail!("strategy pool has no asset to sell");
            }
            /* calculate how much to spend */
            // TODO: this formula doesn't seem right. need revise
            // suggested formula: amount_to_spend = trade.amount_in * sp_asset_token_in_amount / expert_wallet_token_in_amount
            let corrected_amount_in = if trade.amount_in > *strategy_token_in_previous_amount {
                /* if the traded amount_in is larger than the total amount of token_in we know of the strategy,
                     it means that the trader has acquired tokens from sources we have not read
                     if we used an amount_in that is larger in the calculation, it would make amount_to_spend
                     larger than the amount of token_in in the strategy pool contract, which would revert the transaction
                     so we use the total amount of token_in we know of the strategy,
                     which will result in a trade of all the strategy pool balance of this asset
                */
                *strategy_token_in_previous_amount
            } else {
                trade.amount_in
            };
            let amount_to_spend = corrected_amount_in
                // TODO: use expert_wallet_token_in_amount from FunWatcherListStrategyWatchingWalletTradeLedger
                .mul_div(sp_asset_token_in_amount, *strategy_token_in_previous_amount)?;
            if amount_to_spend == U256::zero() {
                bail!("spent ratio is too small to be represented in amount of token_in owned by strategy pool");
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
                sp_contract.clone(),
                &conn,
                CONFIRMATIONS,
                MAX_RETRIES,
                POLL_INTERVAL,
                state.master_key.clone(),
                trade.token_in,
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
            )
            .await?;

            /* trade token_in for token_out */
            let trade_hash = copy_trade_and_ensure_success(
                pancake_contract,
                &conn,
                CONFIRMATIONS,
                MAX_RETRIES,
                POLL_INTERVAL,
                state.master_key.clone(),
                trade.get_pancake_pair_paths()?,
                amount_to_spend,
                U256::from(1),
            )
            .await?;

            /* parse trade to find amount_out */
            let sp_trade = parse_dex_trade(
                blockchain,
                &TransactionFetcher::new_and_assume_ready(trade_hash, &conn).await?,
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
                sp_contract.address(),
                sp_trade.amount_out,
            )
            .await?;

            /* give back traded assets */
            give_back_assets_after_trade_and_ensure_success(
                sp_contract,
                &conn,
                CONFIRMATIONS,
                MAX_RETRIES,
                POLL_INTERVAL,
                state.master_key.clone(),
                vec![trade.token_out],
                vec![sp_trade.amount_out],
            )
            .await?;

            /* write new strategy pool contract token balances to database */
            state
                .db
                .execute(FunWatcherUpsertStrategyPoolContractAssetBalanceReq {
                    strategy_pool_contract_id: address_row.pkey_id,
                    token_address: trade.token_in.into(),
                    blockchain,
                    new_balance: match sp_asset_token_in_amount.try_checked_sub(amount_to_spend) {
                        Ok(new_balance) => new_balance,
                        Err(_) => U256::zero(),
                    }
                    .into(),
                })
                .await?;

            let sp_asset_token_out = state
                .db
                .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
                    strategy_pool_contract_id: address_row.pkey_id,
                    token_address: Some(trade.token_out.into()),
                    blockchain: Some(blockchain),
                })
                .await?
                .into_result();
            let sp_asset_token_out_new_balance = match sp_asset_token_out {
                Some(token_out) => (*token_out.balance).try_checked_add(sp_trade.amount_out)?,
                None => sp_trade.amount_out,
            };

            state
                .db
                .execute(FunWatcherUpsertStrategyPoolContractAssetBalanceReq {
                    strategy_pool_contract_id: address_row.pkey_id,
                    token_address: trade.token_out.into(),
                    blockchain,
                    new_balance: sp_asset_token_out_new_balance.into(),
                })
                .await?;
        }
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
            let ret: Result<()> = async {
                /* the transactions from the quickalerts payload might not be yet mined */
                wait_for_confirmations(
                    &conn.eth(),
                    hash,
                    POLL_INTERVAL,
                    MAX_RETRIES,
                    CONFIRMATIONS,
                )
                .await
                .context("escrow tx was not mined")?;
                let tx = TransactionFetcher::new_and_assume_ready(hash, &conn)
                    .await
                    .context("error processing tx")?;
                if let Err(e) = evm::cache_ethereum_transaction(&tx, &state.db, blockchain).await {
                    error!("error caching transaction: {:?}", e);
                };

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

                /* check if transaction is from one of our users */
                // TODO: handle an escrow made by an unknown user
                let caller = tx
                    .get_from()
                    .with_context(|| format!("no caller found for tx: {:?}", tx.get_hash()))?;

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

                /* get token address that was transferred */
                let called_address = tx.get_to().with_context(|| {
                    format!("no called address found for tx: {:?}", tx.get_hash())
                })?;

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

                let old_balance = state
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
                    .unwrap_or_default();
                let new_balance = (*old_balance) + escrow.amount;
                state
                    .db
                    .execute(FunWatcherUpsertUserDepositWithdrawBalanceReq {
                        user_id: user.user_id,
                        blockchain,
                        old_balance,
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
                                quantity: escrow.amount.into(),
                                blockchain,
                                user_address: escrow.owner.into(),
                                contract_address: called_address.into(),
                                transaction_hash: tx.get_hash().into(),
                                receiver_address: escrow.recipient.into(),
                                created_at: Utc::now().timestamp(),
                            },
                        })
                        .await
                    {
                        error!("error notifying admin of escrow ledger change: {:?}", err);
                    }
                }
                Ok::<_, Error>(())
            }
            .await;
            if let Err(err) = ret {
                error!("Error handling ethereum escrow {:?}", err)
            }
        });
    }

    Ok(())
}
