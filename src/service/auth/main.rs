mod method;

use crate::endpoints::*;
use crate::method::*;
use eyre::*;
use lib::config::{load_config, Config};
use lib::database::connect_to_database;
use lib::log::setup_logs;
use lib::ws::{EndpointAuthController, WebsocketServer};
use serde_json::Value;

pub mod endpoints;

#[tokio::main]
async fn main() -> Result<()> {
    let mut config: Config<Value> = load_config("auth".to_owned())?;
    setup_logs(config.app.log_level)?;
    config.app.header_only = true;

    let mut server = WebsocketServer::new(config.app);
    server.add_database(connect_to_database(config.app_db).await?);
    server.add_database(connect_to_database(config.auth_db).await?);
    let mut auth_controller = EndpointAuthController::new();
    auth_controller.add_auth_endpoint(endpoint_auth_login(), LoginHandler);
    auth_controller.add_auth_endpoint(endpoint_auth_signup(), SignupHandler);
    server.add_auth_controller(auth_controller);
    server.listen().await?;
    Ok(())
}
