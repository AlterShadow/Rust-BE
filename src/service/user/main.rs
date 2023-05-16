mod method;

use eyre::*;

use crate::endpoints::*;
use crate::method::*;
use gen::model::EnumService;
use lib::config::{load_config, WsServerConfig};
use lib::database::{connect_to_database, DatabaseConfig};
use lib::log::{setup_logs, LogLevel};
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
    let config: Config = load_config("user".to_owned())?;
    setup_logs(config.log_level)?;

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
    server.add_handler(endpoint_user_follow_strategy(), MethodUserFollowStrategy);
    server.add_handler(
        endpoint_user_list_followed_strategies(),
        MethodUserListFollowedStrategies,
    );
    server.add_handler(
        endpoint_user_unfollow_strategy(),
        MethodUserUnfollowStrategy,
    );
    server.add_handler(endpoint_user_list_strategies(), MethodUserListStrategies);

    server.add_handler(endpoint_user_register_wallet(), MethodUserRegisterWallet);
    server.add_handler(
        endpoint_user_deregister_wallet(),
        MethodUserDeregisterWallet,
    );
    server.add_handler(endpoint_user_create_strategy(), MethodUserCreateStrategy);
    server.add_handler(endpoint_user_update_strategy(), MethodUserUpdateStrategy);
    server.add_handler(
        endpoint_user_add_strategy_watching_wallet(),
        MethodUserAddStrategyWatchingWallet,
    );
    server.add_handler(
        endpoint_user_remove_strategy_watching_wallet(),
        MethodUserRemoveStrategyWatchingWallet,
    );
    server.listen().await?;
    Ok(())
}
