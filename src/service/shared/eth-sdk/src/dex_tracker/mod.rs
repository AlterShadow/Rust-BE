use bytes::Bytes;
use eyre::*;
use http::StatusCode;
use lib::database::{DbClient, ToSql};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tracing::error;
use web3::types::{Address, U256};

mod parse;
use crate::calc::ScaledMath;
use crate::erc20::Erc20Token;
use crate::evm::DexTrade;
use crate::evm::{parse_quickalert_payload, AppState};
use crate::utils::wait_for_confirmations_simple;
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
    let strategy_id = get_strategy_id_from_watching_wallet(&state.db, &blockchain, &caller).await?;

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
    if let Some(amount) = merged_strategy_tokens.get(&token_in) {
        /* if token_in was already a strategy token trade it from SPs in ratio traded_amount / old_amount */
        // TODO: implement ratio calculation using U256 on calc
        // TODO: make SPs on multiple chains possible
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
        .context("caller is not a strategy watching wallet")?
        .try_get("fkey_strategy_id")
        .context("error parsing fkey_strategy_id from tbl.strategy_watching_wallet")?;

    Ok(strategy_id)
}

pub async fn add_or_update_initial_token_ratio(
    conn: &EthereumRpcConnection,
    db: &DbClient,
    strategy_id: i64,
    trade: &DexTrade,
    blockchain: EnumBlockChain,
) -> Result<()> {
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
            let token_in = Erc20Token::new(conn.clone(), trade.token_in)?;
            db.execute(FunUserAddStrategyInitialTokenRatioReq {
                strategy_id: strategy_id,
                token_address: format!("{:?}", trade.token_in),
                token_name: token_in.symbol().await?,
                quantity: format!("{:?}", token_in.balance_of(trade.caller).await?),
                blockchain: blockchain,
            })
            .await?;
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
                quantity: format!("{:?}", token_out.balance_of(trade.caller).await?),
                blockchain: blockchain,
            })
            .await?;
        }
    };

    Ok(())
}
