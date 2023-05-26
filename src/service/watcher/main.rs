use crate::dex_tracker::pancake::build_pancake_swap;
use crate::dex_tracker::*;
use crate::escrow_tracker::*;
use crate::evm::AppState;
use axum::{body::Body, routing::post, Router};
use axum_server::tls_rustls::RustlsConfig;
use eth_sdk::erc20::build_erc_20;
use eth_sdk::EthereumRpcConnectionPool;
use eyre::*;
use lib::config::load_config;
use lib::database::{connect_to_database, DatabaseConfig};
use lib::log::{setup_logs, LogLevel};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::*;

pub mod contract_wrappers;
pub mod dex_tracker;
pub mod escrow_tracker;
pub mod evm;

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

    let eth_pool = EthereumRpcConnectionPool::new(config.eth_provider_url.to_string(), 10)?;
    let app: Router<(), Body> = Router::new()
        .route("/eth-mainnet-swaps", post(handle_eth_swap_mainnet))
        .route("/eth-goerli-swaps", post(handle_eth_swap_goerli))
        .route("/eth-mainnet-escrows", post(handle_eth_escrows_mainnet))
        .route("/eth-goerli-escrows", post(handle_eth_escrows_goerli))
        .with_state(Arc::new(AppState {
            dex_addresses: DexAddresses::new(),
            stablecoin_addresses: StableCoinAddresses::default(),
            eth_pool,
            erc_20: build_erc_20()?,
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
