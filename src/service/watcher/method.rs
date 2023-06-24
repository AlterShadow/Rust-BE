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
use eth_sdk::utils::wait_for_confirmations_simple;
use eth_sdk::v3::smart_router::{copy_trade_and_ensure_success, PancakeSmartRouterV3Contract};
use eth_sdk::{evm, ScaledMath, TransactionFetcher, TransactionReady};
use eyre::*;
use gen::database::*;
use gen::model::*;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};
use web3::ethabi::Address;
use web3::types::U256;

pub async fn handle_ethereum_dex_transactions(
    state: Arc<AppState>,
    body: Bytes,
    blockchain: EnumBlockChain,
) -> eyre::Result<(), StatusCode> {
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
) -> eyre::Result<()> {
    /* check if caller is a strategy watching wallet & get strategy id */
    let caller = tx.get_from().context("no from address found")?;
    let strategy_id = get_strategy_id_from_watching_wallet(&state.db, blockchain, caller)
        .await
        .context("caller is not a strategy watching wallet")?;

    /* parse trade */
    let trade = parse_dex_trade(blockchain, &tx, &state.dex_addresses, &state.pancake_swap).await?;

    /* get called contract */
    let called_address = tx.get_to().context("no to address found")?;

    /* update wallet activity ledger & make sure this transaction is not a duplicate */
    let saved = state
        .db
        .execute(FunWatcherSaveStrategyWatchingWalletTradeLedgerReq {
            address: format!("{:?}", caller.clone()),
            transaction_hash: format!("{:?}", tx.get_hash()),
            blockchain,
            contract_address: format!("{:?}", called_address),
            dex: Some(EnumDex::PancakeSwap.to_string()),
            token_in_address: Some(format!("{:?}", trade.token_in)),
            token_out_address: Some(format!("{:?}", trade.token_out)),
            amount_in: Some(format!("{:?}", trade.amount_in)),
            amount_out: Some(format!("{:?}", trade.amount_out)),
            happened_at: None,
        })
        .await
        .context("swap transaction is a duplicate")?
        .into_result();

    if let Some(saved) = saved {
        /* check if tokens are known */
        let token_in = state
            .token_addresses
            .get_by_address(blockchain, trade.token_in)
            .context("token in is unknown")?;
        state
            .token_addresses
            .get_by_address(blockchain, trade.token_out)
            .context("token out is unknown")?;

        /* get all strategy tokens */

        let all_strategy_tokens = state
            .db
            .execute(
                // fun_watcher_list_user_strategy_ledger
                FunWatcherListStrategyEscrowPendingWalletBalanceReq {
                    strategy_id: Some(strategy_id),
                },
            )
            .await?;

        /* build up multichain token map */
        let mut strategy_token_ledger: HashMap<EnumBlockchainCoin, U256> = HashMap::new();
        for row in all_strategy_tokens.into_iter() {
            let (token_chain, token_address, token_amount) = (
                row.blockchain,
                row.token_address.parse::<Address>()?,
                row.balance.parse::<U256>()?,
            );
            let strategy_token = state
                .token_addresses
                .get_by_address(token_chain, token_address)
                .context("strategy token is unknown")?;
            if strategy_token_ledger
                .insert(strategy_token, token_amount)
                .is_some()
            {
                bail!(
                    "Duplicate entry in strategy token list for {:?} {:?}",
                    strategy_token,
                    token_address
                );
            }
        }

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

        /* check if token_in was a strategy token */
        if let Some(total_strategy_token_in_amount) = strategy_token_ledger.get(&token_in) {
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
                let address = address_row.address;
                /* check if SP contract holds token_in */
                let sp_contract =
                    StrategyPoolContract::new(conn.clone(), Address::from_str(&address)?)?;
                let mut maybe_sp_token_in_amount: Option<U256> = None;
                let mut max_retries = 10;
                while maybe_sp_token_in_amount.is_none() && max_retries > 0 {
                    match sp_contract.asset_balance(trade.token_in).await {
                        Ok(token_in_amount) => {
                            maybe_sp_token_in_amount = Some(token_in_amount);
                        }
                        Err(_) => {
                            /* if we can't query the contract's assets, it's because it is currently trading */
                            /* wait a bit and try again */
                            sleep(Duration::from_secs(30)).await;
                            max_retries -= 1;
                        }
                    }
                }
                let sp_token_in_amount = maybe_sp_token_in_amount
                    .ok_or_else(|| eyre!("failed to query strategy pool token_in amount"))?;

                if sp_token_in_amount == U256::zero() {
                    bail!("strategy pool has no token_in");
                }

                /* calculate how much to spend */
                let amount_to_spend = trade
                    .amount_in
                    .mul_div(sp_token_in_amount, *total_strategy_token_in_amount)?;
                if amount_to_spend == U256::zero() {
                    bail!("spent ratio is too small to be represented in amount of token_in owned by strategy pool");
                }

                /* instantiate token_in and token_out contracts */
                let token_in_contract = Erc20Token::new(conn.clone(), trade.token_in)?;
                let token_out_contract = Erc20Token::new(conn.clone(), trade.token_out)?;

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
                    12,
                    10,
                    Duration::from_secs(10),
                    state.master_key.clone(),
                    trade.token_in,
                    amount_to_spend,
                )
                .await?;

                /* approve pancakeswap to trade token_in */
                approve_and_ensure_success(
                    token_in_contract,
                    &conn,
                    12,
                    10,
                    Duration::from_secs(10),
                    state.master_key.clone(),
                    pancake_contract.address(),
                    amount_to_spend,
                )
                .await?;

                /* trade token_in for token_out */
                let trade_hash = copy_trade_and_ensure_success(
                    pancake_contract,
                    &conn,
                    12,
                    10,
                    Duration::from_secs(10),
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
                    12,
                    10,
                    Duration::from_secs(10),
                    state.master_key.clone(),
                    sp_contract.address(),
                    sp_trade.amount_out,
                )
                .await?;

                /* give back traded assets */
                give_back_assets_after_trade_and_ensure_success(
                    sp_contract,
                    &conn,
                    12,
                    10,
                    Duration::from_secs(10),
                    state.master_key.clone(),
                    vec![trade.token_out],
                    vec![sp_trade.amount_out],
                )
                .await?;
            }
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
                wait_for_confirmations_simple(&conn.eth(), hash, Duration::from_secs(10), 10)
                    .await
                    .context("escrow tx was not mined")?;
                // TODO: wait for confirmations blocks before processing to properly handle ommer blocks & reorgs
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
                        address: format!("{:?}", caller),
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
                });

                /* insert escrow in ledger */
                state
                    .db
                    .execute(FunUserSaveUserDepositWithdrawLedgerReq {
                        user_id: user.user_id,
                        quantity: format!("{:?}", escrow.amount),
                        blockchain,
                        user_address: format!("{:?}", escrow.owner),
                        contract_address: format!("{:?}", called_address),
                        transaction_hash: format!("{:?}", tx.get_hash()),
                        receiver_address: format!("{:?}", escrow.recipient),
                    })
                    .await
                    .context("error inserting escrow in ledger")?;

                let old_balance = state
                    .db
                    .execute(FunUserListUserDepositWithdrawBalanceReq {
                        user_id: user.user_id,
                        blockchain: Some(blockchain),
                        token_address: Some(format!("{:?}", called_address)),
                        token_id: None,
                        escrow_contract_address: Some(format!("{:?}", escrow.recipient)),
                    })
                    .await?
                    .into_result()
                    .map(|x| U256::from_dec_str(&x.balance))
                    .unwrap_or_else(|| Ok(0.into()))?;
                let new_balance = old_balance + escrow.amount;
                state
                    .db
                    .execute(FunWatcherUpsertUserDepositWithdrawBalanceReq {
                        user_id: user.user_id,
                        blockchain,
                        old_balance: format!("{:?}", old_balance),
                        new_balance: format!("{:?}", new_balance),
                        token_address: format!("{:?}", called_address),
                        escrow_contract_address: format!("{:?}", escrow.recipient),
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
                                quantity: format!("{:?}", escrow.amount),
                                blockchain,
                                user_address: format!("{:?}", escrow.owner),
                                contract_address: format!("{:?}", called_address),
                                transaction_hash: format!("{:?}", tx.get_hash()),
                                receiver_address: format!("{:?}", escrow.recipient),
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
