use eyre::*;
use lib::config::{load_config, Config};
use std::net::ToSocketAddrs;

use axum::{
    body::{Body, Bytes},
    extract::State,
    http::StatusCode,
    routing::post,
    Router,
};
use web3::types::H256;

mod rpc_provider;
mod tracker;

use rpc_provider::pool::ConnectionPool;
use tracker::{pancake_swap::PancakeSwap, tx::Tx};

#[derive(Clone, Debug)]
struct AppState {
    eth_pool: Arc<ConnectionPool>,
    pancake_swap: PancakeSwap,
}

const ETH_PROVIDER_URL: &str = "";

#[tokio::main]
async fn main() -> Result<()> {
    let config: Config<()> = load_config("trade_watcher".to_owned())?;
    // setup_logs(config.app.log_level)?;

    let addr = (config.app.host.as_ref(), config.app.port)
        .to_socket_addrs()?
        .next()
        .context("Failed to resolve address")?;

    let app: Router<(), Body> = Router::new()
        .route("/eth-mainnet-swaps", post(handle_eth_swap))
        .with_state(AppState {
            eth_pool: ConnectionPool::new(ETH_PROVIDER_URL.to_string(), 100, 300, 10).await?,
            pancake_swap: PancakeSwap::new().await,
        });

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn handle_eth_swap(State(state): State<AppState>, body: Bytes) -> Result<(), StatusCode> {
    let hashes = parse_quickalert_payload(body).map_err(|e| {
        println!("Failed to parse QuickAlerts payload: {:?}", e);
        StatusCode::BAD_REQUEST
    })?;

    for hash in hashes {
        let eth = match state.eth_pool.clone().get_conn().await {
            Ok(eth) => eth,
            Err(e) => {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
        let pancake_swap = state.pancake_swap.clone();
        tokio::spawn(async move {
            let tx = Tx::new(hash, eth.clone()).await;
            let trade = pancake_swap.get_trade(tx);
            println!("Trade: {:?}", trade);
        });
    }

    Ok(())
}

fn parse_quickalert_payload(payload: Bytes) -> Result<Vec<H256>> {
    let result: Result<Vec<H256>, _> = serde_json::from_slice(&payload);

    match result {
        Ok(hashes) => Ok(hashes),
        Err(e) => Err(e.into()),
    }
}
