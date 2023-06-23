use bytes::Bytes;
use eyre::*;
use http::StatusCode;
use lib::database::{DbClient, ToSql};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::error;
use web3::types::{Address, U256};

mod parse;
use crate::calc::ScaledMath;
use crate::erc20::approve_and_ensure_success;
use crate::erc20::Erc20Token;
use crate::evm::DexTrade;
use crate::evm::{parse_quickalert_payload, AppState};
use crate::strategy_pool::{
    acquire_asset_before_trade_and_ensure_success, give_back_assets_after_trade_and_ensure_success,
    StrategyPoolContract,
};
use crate::utils::wait_for_confirmations_simple;
use crate::v3::smart_router::{copy_trade_and_ensure_success, PancakeSmartRouterV3Contract};
use crate::TransactionReady;
use crate::{evm, TransactionFetcher};
use gen::database::*;
use gen::model::EnumBlockchainCoin;
use gen::model::{EnumBlockChain, EnumDex};
pub use parse::*;

pub async fn handle_eth_swap(
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
            match handle_swap(state.clone(), blockchain, tx).await {
                Ok(_) => {}
                Err(e) => {
                    error!("error handling swap: {:?}", e);
                }
            }
        });
    }

    Ok(())
}

pub async fn handle_swap(
    state: Arc<AppState>,
    blockchain: EnumBlockChain,
    tx: TransactionReady,
) -> Result<()> {
    /* check if caller is a strategy watching wallet & get strategy id */
    let caller = tx.get_from().context("no from address found")?;
    let strategy_id = get_strategy_id_from_watching_wallet(&state.db, &blockchain, &caller)
        .await
        .context("caller is not a strategy watching wallet")?;

    /* parse trade */
    let trade = parse_dex_trade(blockchain, &tx, &state.dex_addresses, &state.pancake_swap).await?;

    /* get called contract */
    let called_address = tx.get_to().context("no to address found")?;

    /* update wallet activity ledger & make sure this transaction is not a duplicate */
    let saved = state
        .db
        .execute(FunWatcherSaveStrategyWatchingWalletTradeHistoryReq {
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
                FunWatcherListStrategyEscrowPendingWalletLedgerReq {
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
                row.entry.parse::<U256>()?,
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
            /* if token_in was not known to the strategy we can't calculate how much to spend */

            /* fetch user_id from strategy */
            let user_id = get_user_id_from_strategy(&state.db, strategy_id).await?;

            /* fetch strategy */
            let strategy = state
                .db
                .execute(FunUserGetStrategyReq {
                    strategy_id,
                    user_id,
                })
                .await?
                .into_result()
                .context("strategy is not registered in the database")?;
            if let Some(address) = strategy.evm_contract_address {
                // TODO: make SPs on multiple chains possible
                /* if there is an SP contract for this strategy,  */

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

                /* acquire token_in from strategy pool */
                // TODO: treat case where strategy pool started trading, and we can't acquire tokens
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

pub async fn get_strategy_id_from_watching_wallet(
    db: &DbClient,
    chain: &EnumBlockChain,
    wallet: &Address,
) -> Result<i64> {
    let strategy_id: i64 = db
        .query(
            "
				SELECT fkey_strategy_id
				FROM tbl.strategy_watching_wallet
				WHERE address = $1 AND blockchain = $2
			",
            &vec![
                &format!("{:?}", wallet) as &(dyn ToSql + Sync),
                chain as &(dyn ToSql + Sync),
            ],
        )
        .await?
        .first()
        .context("error fetching fkey_strategy_id from tbl.strategy_watching_wallet")?
        .try_get("fkey_strategy_id")
        .context("error parsing fkey_strategy_id from tbl.strategy_watching_wallet")?;

    Ok(strategy_id)
}

pub async fn get_user_id_from_strategy(db: &DbClient, strategy_id: i64) -> Result<i64> {
    let strategy_id: i64 = db
        .query(
            "
				SELECT fkey_user_id
				FROM tbl.strategy
				WHERE pkey_id = $1
			",
            &vec![&strategy_id as &(dyn ToSql + Sync)],
        )
        .await?
        .first()
        .context("error fetching fkey_user_id from tbl.strategy")?
        .try_get("fkey_user_id")
        .context("error parsing fkey_user_id from tbl.strategy")?;

    Ok(strategy_id)
}

pub async fn update_expert_listened_wallet_asset_ledger(
    db: &DbClient,
    _strategy_id: i64,
    trade: &DexTrade,
    token_out_id: i64,
    token_in_id: i64,
    blockchain: EnumBlockChain,
) -> Result<()> {
    // correctly adding wallet balance to tbl.strategy_initial_token ratio is not possible because expert can have multiple watching wallets in one chain
    let expert_watched_wallet_address = trade.caller;

    match db
        .execute(FunWatcherListExpertListenedWalletAssetLedgerReq {
            limit: 1,
            blockchain: Some(blockchain),
            address: Some(format!("{:?}", expert_watched_wallet_address)),
            token_id: Some(token_out_id),
            offset: 0,
        })
        .await?
        .into_result()
    {
        Some(tk) => {
            /* if token_in is already in the database, update it's amount */
            let old_amount = U256::from_dec_str(&tk.entry)?;
            let new_amount = old_amount.try_checked_sub(trade.amount_out)?;
            db.execute(FunWatcherUpsertExpertListenedWalletAssetLedgerReq {
                address: format!("{:?}", expert_watched_wallet_address),
                blockchain,
                token_id: token_out_id,
                old_entry: tk.entry,
                new_entry: format!("{:?}", new_amount),
            })
            .await?;
        }
        None => {
            // what should we do when we have nothing to subtract from?
        }
    };

    match db
        .execute(FunWatcherListExpertListenedWalletAssetLedgerReq {
            limit: 1,
            blockchain: Some(blockchain),
            address: Some(format!("{:?}", expert_watched_wallet_address)),
            token_id: Some(token_in_id),
            offset: 0,
        })
        .await?
        .into_result()
    {
        Some(tk) => {
            /* if token_in is already in the database, update it's amount, or remove it new amount is 0 */
            let old_amount = U256::from_dec_str(&tk.entry)?;
            let new_amount = old_amount.try_checked_add(trade.amount_in)?;
            db.execute(FunWatcherUpsertExpertListenedWalletAssetLedgerReq {
                address: format!("{:?}", expert_watched_wallet_address),
                blockchain,
                token_id: token_in_id,
                old_entry: tk.entry,
                new_entry: format!("{:?}", new_amount),
            })
            .await?;
        }
        None => {
            let old_amount = U256::from(0);
            let new_amount = trade.amount_in;
            db.execute(FunWatcherUpsertExpertListenedWalletAssetLedgerReq {
                address: format!("{:?}", expert_watched_wallet_address),
                blockchain,
                token_id: token_in_id,
                old_entry: format!("{:?}", old_amount),
                new_entry: format!("{:?}", new_amount),
            })
            .await?;
        }
    };

    Ok(())
}
