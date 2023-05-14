mod method;

use eyre::*;

use gen::model::EnumService;
use lib::config::{load_config, Config};
use lib::database::connect_to_database;
use lib::log::setup_logs;

use crate::endpoints::endpoint_user_register_wallet;
use crate::method::{MethodUserListStrategies, MethodUserRegisterWallet};
use lib::ws::{EndpointAuthController, WebsocketServer};
use mc2_fi::endpoints::endpoint_auth_authorize;
use mc2_fi::method::MethodAuthAuthorize;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

pub mod endpoints;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum ActivityType {
    TransferProposed,
    TransferApproved,
    TransferStarted,
    TransferCompleted,
    Mint,
    Approve,
}
impl Display for ActivityType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config: Config<()> = load_config("user".to_owned())?;
    setup_logs(config.app.log_level)?;

    let mut server = WebsocketServer::new(config.app.clone());
    server.add_database(connect_to_database(config.app_db).await?);
    server.add_database(connect_to_database(config.auth_db).await?);

    let mut auth_controller = EndpointAuthController::new();
    auth_controller.add_auth_endpoint(
        endpoint_auth_authorize(),
        MethodAuthAuthorize {
            accept_service: EnumService::User,
        },
    );
    server.add_auth_controller(auth_controller);
    server.add_handler(endpoint_user_register_wallet(), MethodUserRegisterWallet);
    server.listen().await?;
    Ok(())
}
