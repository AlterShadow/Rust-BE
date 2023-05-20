use axum::{
    body::{Body, Bytes},
    extract::State,
    http::StatusCode,
    routing::post,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use eyre::*;
use lib::config::load_config;
use lib::database::{connect_to_database, DatabaseConfig, DbClient};
use lib::log::{setup_logs, LogLevel};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, info};
use web3::types::H256;

pub mod tracker;

use crate::tracker::pancake_swap::pancake::build_pancake_swap;
use crate::tracker::pancake_swap::PancakeSwap;
use crate::tracker::parse::parse_dex_trade;
use crate::tracker::DexAddresses;
use gen::database::FunWatcherSaveRawTransactionReq;
use lib::evm_parse::tx::{parse_ethereum_transaction, Tx};
use lib::evm_parse::Chain;
use lib::rpc_provider::pool::ConnectionPool;

struct AppState {
    dex_addresses: DexAddresses,
    eth_pool: ConnectionPool,
    pancake_swap: PancakeSwap,
    db: DbClient,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub app_db: DatabaseConfig,
    #[serde(default)]
    pub log_level: LogLevel,
    pub eth_provider_url: String,

    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: u16,
    #[serde(default)]
    pub pub_cert: Option<String>,
    #[serde(default)]
    pub priv_cert: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config: Config = load_config("trade-watcher".to_owned())?;
    setup_logs(config.log_level)?;
    let db = connect_to_database(config.app_db).await?;

    let eth_pool = ConnectionPool::new(config.eth_provider_url.to_string(), 10).await?;
    let app: Router<(), Body> = Router::new()
        .route("/eth-mainnet-swaps", post(handle_eth_swap))
        .with_state(Arc::new(AppState {
            dex_addresses: DexAddresses::new(),
            eth_pool,
            pancake_swap: build_pancake_swap()?,
            db,
        }));

    let addr = tokio::net::lookup_host((config.host.as_ref(), config.port))
        .await?
        .next()
        .context("failed to resolve address")?;
    info!("Trade watcher listening on {}", addr);
    if let (Some(pub_cert), Some(priv_key)) = (config.pub_cert, config.priv_cert) {
        // configure certificate and private key used by https
        let config = RustlsConfig::from_pem_file(pub_cert, priv_key).await?;
        axum_server::bind_rustls(addr, config)
            .serve(app.into_make_service())
            .await?;
    } else {
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;
    }

    Ok(())
}

async fn handle_eth_swap(state: State<Arc<AppState>>, body: Bytes) -> Result<(), StatusCode> {
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
            match parse_ethereum_transaction(hash, &conn).await {
                Ok((tx, called_contract)) => {
                    match cache_ethereum_transaction(&hash, &tx, &state.db).await {
                        Ok(_) => {}
                        Err(e) => {
                            error!("error caching transaction: {:?}", e);
                        }
                    };
                    if let Err(e) = parse_dex_trade(
                        Chain::EthereumMainnet,
                        &tx,
                        &called_contract,
                        &state.dex_addresses,
                        &state.pancake_swap,
                    )
                    .await
                    {
                        error!("error parsing dex trade: {:?}", e);
                    };
                }
                Err(err) => {
                    error!("error processing tx: {:?}", err);
                }
            };
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

async fn cache_ethereum_transaction(hash: &H256, tx: &Tx, db: &DbClient) -> Result<()> {
    if let Err(err) = {
        if let Some(content) = tx.get_transaction() {
            db.execute(FunWatcherSaveRawTransactionReq {
                transaction_hash: format!("{:?}", hash),
                chain: "ethereum".to_string(),
                dex: None,
                raw_transaction: serde_json::to_string(content).context("transaction")?,
            })
            .await?;
        }
        Ok::<_, Error>(())
    } {
        return Err(eyre!("failed to save raw transaction: {}", err));
    }

    Ok(())
}
