use crate::shared_method::load_allow_domain_urls;
use eth_sdk::{EthereumConns, EthereumRpcConnectionPool};
use eyre::*;
use lib::config::load_config;
use lib::database::{connect_to_database, DatabaseConfig};
use lib::log::{setup_logs, LogLevel};
use lib::ws::{EndpointAuthController, WebsocketServer, WsServerConfig};
use mc2fi_auth::endpoints::{endpoint_auth_login, endpoint_auth_logout, endpoint_auth_signup};
use mc2fi_auth::method::{MethodAuthLogin, MethodAuthLogout, MethodAuthSignup};
use serde::*;

#[path = "../shared/shared_method.rs"]
pub mod shared_method;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub app_db: DatabaseConfig,
    pub auth_db: DatabaseConfig,
    #[serde(default)]
    pub log_level: LogLevel,
    #[serde(flatten)]
    pub app: WsServerConfig,
    pub ethereum_urls: EthereumConns,
}
#[tokio::main]
async fn main() -> Result<()> {
    let mut config: Config = load_config("auth".to_owned())?;
    setup_logs(config.log_level)?;
    config.app.header_only = true;
    let pool = EthereumRpcConnectionPool::from_conns(config.ethereum_urls.clone());

    let mut server = WebsocketServer::new(config.app);
    let db = connect_to_database(config.app_db).await?;
    load_allow_domain_urls(&db, &mut server.config).await?;
    server.add_database(db);
    server.add_database(connect_to_database(config.auth_db).await?);
    let mut auth_controller = EndpointAuthController::new();
    auth_controller.add_auth_endpoint(endpoint_auth_login(), MethodAuthLogin);
    auth_controller.add_auth_endpoint(endpoint_auth_logout(), MethodAuthLogout);
    auth_controller.add_auth_endpoint(endpoint_auth_signup(), MethodAuthSignup { pool });
    server.add_auth_controller(auth_controller);
    server.listen().await?;
    Ok(())
}
