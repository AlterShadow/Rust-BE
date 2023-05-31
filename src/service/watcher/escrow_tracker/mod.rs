use crate::escrow_tracker::escrow::parse_escrow;
use crate::evm::parse_quickalert_payload;
use crate::{evm, AppState};
use axum::extract::State;
use axum::http::StatusCode;
use bytes::Bytes;
use eth_sdk::TransactionFetcher;
use eyre::*;
use gen::model::EnumBlockChain;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::Arc;
use tracing::error;
use web3::types::{Address, H160};

pub mod deposit;
pub mod escrow;

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
            if let Err(e) = parse_escrow(
                EnumBlockChain::EthereumMainnet,
                &tx,
                &state.stablecoin_addresses,
                &state.erc_20,
            ) {
                error!("error parsing escrow trade: {:?}", e);
            };
        });
    }

    Ok(())
}
pub async fn handle_eth_escrows_mainnet(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    handle_eth_escrows(state.0, body, EnumBlockChain::EthereumMainnet).await
}

pub async fn handle_eth_escrows_goerli(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    handle_eth_escrows(state.0, body, EnumBlockChain::EthereumGoerli).await
}
