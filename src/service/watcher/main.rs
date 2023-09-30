use api::cmc::CoinMarketCap;
use axum::{
    extract::State,
    routing::post,
    http::StatusCode,
    Router,
    body::Body,
};
use axum_server::tls_rustls::RustlsConfig;
use bytes::Bytes;
use eth_sdk::signer::Secp256k1SecretKey;
use eth_sdk::{EthereumConns, EthereumRpcConnectionPool};
use eyre::*;
use gen::model::*;
use lib::config::load_config;
use lib::database::{connect_to_database, DatabaseConfig};
use lib::log::{setup_logs, LogLevel};
use mc2fi_auth::{connect_user, signup};
use mc2fi_user::shared_method::load_coin_addresses;
use mc2fi_watcher::{method, AppState};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use std::sync::Arc;
use tracing::*;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub app_db: DatabaseConfig,
    #[serde(default)]
    pub log_level: LogLevel,

    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: u16,
    #[serde(default)]
    pub pub_cert: Option<String>,
    #[serde(default)]
    pub priv_cert: Option<String>,
    pub ethereum_urls: EthereumConns,
    pub god_key: SecretString,
    pub user_url: String,
    pub auth_url: String,
    pub cmc_api_key: SecretString,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config: Config = load_config("watcher".to_owned())?;
    setup_logs(config.log_level)?;
    let cmc_client = CoinMarketCap::new(config.cmc_api_key.expose_secret())?;
    let master_key = Secp256k1SecretKey::from_str(config.god_key.expose_secret())?;
    if let Err(err) = signup(&config.auth_url, "dev-watcher", &master_key.key).await {
        error!("failed to signup: {}", err);
    }

    let client = connect_user(
        &config.auth_url,
        &config.user_url,
        "dev-watcher",
        &master_key.key,
    )
    .await?;
    let db = connect_to_database(config.app_db).await?;

    let eth_pool = EthereumRpcConnectionPool::from_conns(config.ethereum_urls);
    let coin_addresses = load_coin_addresses(&db).await?;
    let app: Router<(), Body> = Router::new()
        .route("/eth-mainnet-swaps", post(handle_swaps_eth_mainnet))
        .route("/eth-mainnet-escrows", post(handle_escrows_eth_mainnet))
        .route("/eth-mainnet-withdraws", post(handle_withdraws_eth_mainnet))
        .route("/eth-mainnet-redeems", post(handle_redeems_eth_mainnet))
        .route(
            "/eth-mainnet-revoke-adminships",
            post(handle_revoke_adminships_eth_mainnet),
        )
        .route("/eth-goerli-swaps", post(handle_swaps_eth_goerli))
        .route("/eth-goerli-escrows", post(handle_escrows_eth_goerli))
        .route("/eth-goerli-withdraws", post(handle_withdraws_eth_goerli))
        .route("/eth-goerli-redeems", post(handle_redeems_eth_goerli))
        .route(
            "/eth-goerli-revoke-adminships",
            post(handle_revoke_adminships_eth_goerli),
        )
        .route("/bsc-mainnet-swaps", post(handle_swaps_bsc_mainnet))
        .route("/bsc-mainnet-escrows", post(handle_escrows_bsc_mainnet))
        .route("/bsc-mainnet-withdraws", post(handle_withdraws_bsc_mainnet))
        .route("/bsc-mainnet-redeems", post(handle_redeems_bsc_mainnet))
        .route(
            "/bsc-mainnet-revoke-adminships",
            post(handle_revoke_adminships_bsc_mainnet),
        )
        .route("/bsc-testnet-swaps", post(handle_swaps_bsc_testnet))
        .route("/bsc-testnet-escrows", post(handle_escrows_bsc_testnet))
        .route("/bsc-testnet-withdraws", post(handle_withdraws_bsc_testnet))
        .route("/bsc-testnet-redeems", post(handle_redeems_bsc_testnet))
        .route(
            "/bsc-testnet-revoke-adminships",
            post(handle_revoke_adminships_bsc_testnet),
        )
        .with_state(Arc::new(
            AppState::new(db, eth_pool, master_key, client, cmc_client, coin_addresses).await?,
        ));

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

pub async fn handle_swaps_eth_mainnet(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    method::handle_swaps(state.0, body, EnumBlockChain::EthereumMainnet).await
}

pub async fn handle_escrows_eth_mainnet(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    method::handle_escrows(state.0, body, EnumBlockChain::EthereumMainnet).await
}

pub async fn handle_withdraws_eth_mainnet(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    method::handle_withdraws(state.0, body, EnumBlockChain::EthereumMainnet).await
}

pub async fn handle_redeems_eth_mainnet(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    method::handle_redeems(state.0, body, EnumBlockChain::EthereumMainnet).await
}

pub async fn handle_revoke_adminships_eth_mainnet(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    method::handle_revoke_adminships(state.0, body, EnumBlockChain::EthereumMainnet).await
}

pub async fn handle_swaps_eth_goerli(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    method::handle_swaps(state.0, body, EnumBlockChain::EthereumGoerli).await
}

pub async fn handle_escrows_eth_goerli(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    method::handle_escrows(state.0, body, EnumBlockChain::EthereumGoerli).await
}

pub async fn handle_withdraws_eth_goerli(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    method::handle_withdraws(state.0, body, EnumBlockChain::EthereumGoerli).await
}

pub async fn handle_redeems_eth_goerli(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    method::handle_redeems(state.0, body, EnumBlockChain::EthereumGoerli).await
}

pub async fn handle_revoke_adminships_eth_goerli(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    method::handle_revoke_adminships(state.0, body, EnumBlockChain::EthereumGoerli).await
}

pub async fn handle_swaps_bsc_mainnet(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    method::handle_swaps(state.0, body, EnumBlockChain::BscMainnet).await
}

pub async fn handle_escrows_bsc_mainnet(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    method::handle_escrows(state.0, body, EnumBlockChain::BscMainnet).await
}

pub async fn handle_withdraws_bsc_mainnet(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    method::handle_withdraws(state.0, body, EnumBlockChain::BscMainnet).await
}

pub async fn handle_redeems_bsc_mainnet(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    method::handle_redeems(state.0, body, EnumBlockChain::BscMainnet).await
}

pub async fn handle_revoke_adminships_bsc_mainnet(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    method::handle_revoke_adminships(state.0, body, EnumBlockChain::BscMainnet).await
}

pub async fn handle_swaps_bsc_testnet(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    method::handle_swaps(state.0, body, EnumBlockChain::BscTestnet).await
}

pub async fn handle_escrows_bsc_testnet(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    method::handle_escrows(state.0, body, EnumBlockChain::BscTestnet).await
}

pub async fn handle_withdraws_bsc_testnet(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    method::handle_withdraws(state.0, body, EnumBlockChain::BscTestnet).await
}

pub async fn handle_redeems_bsc_testnet(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    method::handle_redeems(state.0, body, EnumBlockChain::BscTestnet).await
}

pub async fn handle_revoke_adminships_bsc_testnet(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    method::handle_revoke_adminships(state.0, body, EnumBlockChain::BscTestnet).await
}
