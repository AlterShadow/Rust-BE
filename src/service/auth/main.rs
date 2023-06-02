mod method;

use crate::endpoints::*;
use crate::method::*;
use eyre::*;
use lib::config::{load_config, WsServerConfig};
use lib::database::{connect_to_database, DatabaseConfig};
use lib::log::{setup_logs, LogLevel};
use lib::ws::{EndpointAuthController, WebsocketServer};
use serde::*;

pub mod endpoints;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub app_db: DatabaseConfig,
    pub auth_db: DatabaseConfig,
    #[serde(default)]
    pub log_level: LogLevel,
    #[serde(flatten)]
    pub app: WsServerConfig,
}
#[tokio::main]
async fn main() -> Result<()> {
    let mut config: Config = load_config("auth".to_owned())?;
    setup_logs(config.log_level)?;
    config.app.header_only = true;

    let mut server = WebsocketServer::new(config.app);
    server.add_database(connect_to_database(config.app_db).await?);
    server.add_database(connect_to_database(config.auth_db).await?);
    let mut auth_controller = EndpointAuthController::new();
    auth_controller.add_auth_endpoint(endpoint_auth_login(), MethodAuthLogin);
    auth_controller.add_auth_endpoint(endpoint_auth_logout(), MethodAuthLogout);
    auth_controller.add_auth_endpoint(endpoint_auth_signup(), MethodAuthSignup);
    server.add_auth_controller(auth_controller);
    server.listen().await?;
    Ok(())
}
