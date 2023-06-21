use bytes::Bytes;
use eyre::*;
use http::StatusCode;
use lib::database::{DbClient, ToSql};
use std::collections::hash_map::Entry;
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
use crate::{evm, EthereumRpcConnection, TransactionFetcher};
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
    // TODO: make transaction hash only be unique on this chain
    state
        .db
        .execute(FunWatcherSaveWalletActivityHistoryReq {
            address: format!("{:?}", caller.clone()),
            transaction_hash: format!("{:?}", tx.get_hash()),
            blockchain: blockchain,
            contract_address: format!("{:?}", called_address),
            caller_address: format!("{:?}", caller.clone()),
            dex: Some(EnumDex::PancakeSwap.to_string()),
            token_in_address: Some(format!("{:?}", trade.token_in)),
            token_out_address: Some(format!("{:?}", trade.token_out)),
            amount_in: Some(format!("{:?}", trade.amount_in)),
            amount_out: Some(format!("{:?}", trade.amount_out)),
            swap_calls: Some(
                serde_json::to_value(trade.swap_calls.clone())
                    .context("error serializing swap calls")?,
            ),
            paths: Some(
                serde_json::to_value(trade.paths.clone()).context("error serializing paths")?,
            ),
            dex_versions: Some(
                serde_json::to_value(trade.dex_versions.clone())
                    .context("error serializing dex versions")?,
            ),
            created_at: None,
        })
        .await
        .context("swap transaction is a duplicate")?;

    /* check if tokens are known */
    let token_in = state
        .token_addresses
        .get_by_address(blockchain, trade.token_in)
        .context("token in is unknown")?;
    state
        .token_addresses
        .get_by_address(blockchain, trade.token_out)
        .context("token out is unknown")?;

    /* fetch strategy's tokens */
    let strategy_token_rows = state
        .db
        .execute(FunUserListStrategyInitialTokenRatiosReq { strategy_id })
        .await?
        .into_rows();

    /* count all strategy tokens */
    let mut total_strategy_tokens = U256::zero();
    let mut all_strategy_tokens: Vec<(EnumBlockChain, Address, U256)> = Vec::new();
    for row in strategy_token_rows.clone() {
        let token_address = Address::from_str(&row.token_address)?;
        let token_amount = U256::from_dec_str(&row.quantity)?;
        total_strategy_tokens = total_strategy_tokens.try_checked_add(token_amount)?;
        all_strategy_tokens.push((row.blockchain, token_address, token_amount));
    }

    /* merge multichain tokens */
    let mut merged_strategy_tokens: HashMap<EnumBlockchainCoin, U256> = HashMap::new();
    for (token_chain, token_address, token_amount) in all_strategy_tokens {
        let strategy_token = state
            .token_addresses
            .get_by_address(token_chain, token_address)
            .context("strategy token is unknown")?;
        match merged_strategy_tokens.entry(strategy_token) {
            Entry::Vacant(e) => {
                e.insert(token_amount);
            }
            Entry::Occupied(mut e) => {
                let balance = e.get_mut();
                *balance = balance.try_checked_add(token_amount)?;
            }
        }
    }

    /* update database with watched wallet's tokens */
    let conn = state.eth_pool.get(blockchain).await?;
    add_or_update_initial_token_ratio(&conn, &state.db, strategy_id, &trade, blockchain).await?;

    /* check if token_in was a strategy token */
    if let Some(total_strategy_token_in_amount) = merged_strategy_tokens.get(&token_in) {
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

pub async fn add_or_update_initial_token_ratio(
    conn: &EthereumRpcConnection,
    db: &DbClient,
    strategy_id: i64,
    trade: &DexTrade,
    blockchain: EnumBlockChain,
) -> Result<()> {
    // TODO: add fkey_strategy_watching_wallet to tbl.strategy_initial_token_ratio so that we can always add current balance of token_in and token_out
    // correctly adding wallet balance to tbl.strategy_initial_token ratio is not possible because expert can have multiple watching wallets in one chain
    match db
        .execute(FunUserGetStrategyInitialTokenRatioByAddressAndChainReq {
            strategy_id: strategy_id,
            token_address: format!("{:?}", trade.token_in),
            blockchain: blockchain,
        })
        .await?
        .into_result()
    {
        Some(database_token) => {
            /* if token_in is already in the database, update it's amount, or remove it new amount is 0 */
            let old_amount = U256::from_dec_str(&database_token.quantity)?;
            let new_amount = old_amount.try_checked_sub(trade.amount_in)?;
            if new_amount == U256::zero() {
                db.execute(FunUserRemoveStrategyInitialTokenRatioReq {
                    strategy_initial_token_ratio_id: database_token.strategy_initial_token_ratio_id,
                    strategy_id: strategy_id,
                })
                .await?;
            } else {
                db.execute(FunUserUpdateStrategyInitialTokenRatioReq {
                    strategy_initial_token_ratio_id: database_token.strategy_initial_token_ratio_id,
                    new_quantity: format!("{:?}", new_amount),
                })
                .await?;
            }
        }
        None => {
            /* if token_in is not in the database, add it with wallet balance */
            // TODO: find out if we should count token_in balance as a strategy token
            // let token_in = Erc20Token::new(conn.clone(), trade.token_in)?;
            // db.execute(FunUserAddStrategyInitialTokenRatioReq {
            //     strategy_id: strategy_id,
            //     token_address: format!("{:?}", trade.token_in),
            //     token_name: token_in.symbol().await?,
            //     quantity: format!("{:?}", token_in.balance_of(trade.caller).await?),
            //     blockchain: blockchain,
            // })
            // .await?;
        }
    };

    match db
        .execute(FunUserGetStrategyInitialTokenRatioByAddressAndChainReq {
            strategy_id: strategy_id,
            token_address: format!("{:?}", trade.token_out),
            blockchain: blockchain,
        })
        .await?
        .into_result()
    {
        Some(database_token) => {
            /* if token_out is already in the database, update it's amount */
            let old_amount = U256::from_dec_str(&database_token.quantity)?;
            let new_amount = old_amount.try_checked_add(trade.amount_out)?;
            db.execute(FunUserUpdateStrategyInitialTokenRatioReq {
                strategy_initial_token_ratio_id: database_token.strategy_initial_token_ratio_id,
                new_quantity: format!("{:?}", new_amount),
            })
            .await?;
        }
        None => {
            /* if token_out is not in the database, add it with wallet balance */
            let token_out = Erc20Token::new(conn.clone(), trade.token_out)?;
            db.execute(FunUserAddStrategyInitialTokenRatioReq {
                strategy_id: strategy_id,
                token_address: format!("{:?}", trade.token_out),
                token_name: token_out.symbol().await?,
                quantity: format!("{:?}", trade.amount_out),
                blockchain: blockchain,
            })
            .await?;
        }
    };

    Ok(())
}
