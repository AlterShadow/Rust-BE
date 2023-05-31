use axum::extract::State;
use axum::http::StatusCode;
use bytes::Bytes;
use eth_sdk::TransactionFetcher;
use gen::model::{EnumBlockChain, EnumDex};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use tracing::error;
use web3::types::H160;

mod pancake_swap;
pub use pancake_swap::*;
mod parse;
use crate::evm;
use crate::evm::AppState;
pub use parse::*;

pub async fn handle_eth_swap(
    state: Arc<AppState>,
    body: Bytes,
    blockchain: EnumBlockChain,
) -> Result<(), StatusCode> {
    let hashes = evm::parse_quickalert_payload(body).map_err(|e| {
        error!("failed to parse QuickAlerts payload: {:?}", e);
        StatusCode::BAD_REQUEST
    })?;

    for hash in hashes {
        let conn = state.eth_pool.get_conn().await.map_err(|err| {
            error!("error fetching connection guard: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        let state = state.clone();
        tokio::spawn(async move {
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
            if let Err(e) = parse_dex_trade(
                EnumBlockChain::EthereumMainnet,
                &tx,
                &state.dex_addresses,
                &state.pancake_swap,
            )
            .await
            {
                error!("error parsing dex trade: {:?}", e);
            };
        });
    }

    Ok(())
}

pub async fn handle_eth_swap_mainnet(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    handle_eth_swap(state.0, body, EnumBlockChain::EthereumMainnet).await
}

pub async fn handle_eth_swap_goerli(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    handle_eth_swap(state.0, body, EnumBlockChain::EthereumGoerli).await
}
