mod method;

use eyre::*;

use crate::endpoints::*;
use crate::method::*;
use eth_sdk::escrow::EscrowContract;
use eth_sdk::signer::Secp256k1SecretKey;
use eth_sdk::EthereumRpcConnectionPool;
use gen::model::EnumService;
use lib::config::{load_config, WsServerConfig};
use lib::database::{connect_to_database, DatabaseConfig};
use lib::log::{setup_logs, LogLevel};
use lib::ws::{EndpointAuthController, WebsocketServer};
use mc2_fi::endpoints::endpoint_auth_authorize;
use mc2_fi::method::MethodAuthAuthorize;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::Arc;

pub mod endpoints;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub app_db: DatabaseConfig,
    pub auth_db: DatabaseConfig,
    pub eth_provider_url: String,
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

    server.add_handler(endpoint_user_register_wallet(), MethodUserRegisterWallet);
    server.add_handler(endpoint_user_list_wallets(), MethodUserListWallets);
    server.add_handler(
        endpoint_user_deregister_wallet(),
        MethodUserDeregisterWallet,
    );
    server.add_handler(endpoint_user_list_strategies(), MethodUserListStrategies);
    server.add_handler(endpoint_user_get_strategy(), MethodUserGetStrategy);
    server.add_handler(
        endpoint_user_get_strategy_statistics(),
        MethodUserGetStrategyStatistics,
    );
    server.add_handler(
        endpoint_user_list_backed_strategies(),
        MethodUserListBackedStrategies,
    );
    server.add_handler(
        endpoint_user_list_exit_strategy_history(),
        MethodUserListExitStrategyHistory,
    );

    server.add_handler(endpoint_user_follow_expert(), MethodUserFollowExpert);

    server.add_handler(
        endpoint_user_list_followed_experts(),
        MethodUserListFollowedExperts,
    );

    server.add_handler(endpoint_user_unfollow_expert(), MethodUserUnfollowExpert);
    server.add_handler(endpoint_user_list_experts(), MethodUserListExperts);
    server.add_handler(
        endpoint_user_get_expert_profile(),
        MethodUserGetExpertProfile,
    );

    server.add_handler(endpoint_user_get_user_profile(), MethodUserGetUserProfile);
    server.add_handler(
        endpoint_user_apply_become_expert(),
        MethodUserApplyBecomeExpert,
    );

    server.add_handler(
        endpoint_admin_approve_user_become_expert(),
        MethodAdminApproveUserBecomeExpert,
    );
    server.add_handler(
        endpoint_admin_reject_user_become_expert(),
        MethodAdminRejectUserBecomeExpert,
    );
    server.add_handler(
        endpoint_admin_list_pending_expert_applications(),
        MethodAdminListPendingExpertApplications,
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
    server.add_handler(
        endpoint_user_list_wallet_activity_history(),
        MethodUserListWalletActivityHistory,
    );
    server.add_handler(
        endpoint_user_add_strategy_initial_token_ratio(),
        MethodUserAddStrategyInitialTokenRatio,
    );
    server.add_handler(
        endpoint_user_remove_strategy_initial_token_ratio(),
        MethodUserRemoveStrategyInitialTokenRatio,
    );
    server.add_handler(
        endpoint_user_list_strategy_initial_token_ratio(),
        MethodUserListStrategyInitialTokenRatio,
    );
    let eth_pool = EthereumRpcConnectionPool::new(config.eth_provider_url.to_string(), 10)?;
    let eth_conn = eth_pool.get_conn().await?.to_owned();
    let escrow_signer = Arc::new(Secp256k1SecretKey::new_random());
    let escrow_contract =
        EscrowContract::deploy(eth_conn.clone().into_raw(), &escrow_signer.key).await?;
    server.add_handler(
        endpoint_user_back_strategy(),
        MethodUserBackStrategy {
            conn: eth_conn.clone(),
            stablecoin_addresses: Arc::new(Default::default()),
            strategy_pool_signer: Arc::new(Secp256k1SecretKey::new_random()),
            escrow_contract: escrow_contract.clone(),
            escrow_signer: escrow_signer.clone(),
            externally_owned_account: Arc::new(Secp256k1SecretKey::new_random()),
            dex_addresses: Arc::new(Default::default()),
        },
    );
    server.add_handler(
        endpoint_user_request_refund(),
        MethodUserRequestRefund {
            conn: eth_pool.get_conn().await?.to_owned(),
            stablecoin_addresses: Arc::new(Default::default()),
            escrow_contract,
            escrow_signer,
        },
    );
    server.listen().await?;
    Ok(())
}
