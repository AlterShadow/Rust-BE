#[path = "../admin/endpoints.rs"]
pub mod admin_endpoints;
#[path = "../admin/method.rs"]
mod admin_method;
pub mod audit;
pub mod endpoints;
mod method;
mod test_helper;

use crate::admin_method::*;
use crate::method::*;
use eth_sdk::escrow::AbstractEscrowContract;
use eth_sdk::signer::Secp256k1SecretKey;
use eth_sdk::{
    BlockchainCoinAddresses, DexAddresses, EscrowAddresses, EthereumConns,
    EthereumRpcConnectionPool,
};
use eyre::*;
use gen::model::{EnumService, UserGetDepositAddressesRow};
use lib::config::{load_config, WsServerConfig};
use lib::database::{connect_to_database, DatabaseConfig};
use lib::log::{setup_logs, LogLevel};
use lib::ws::{EndpointAuthController, WebsocketServer};
use mc2_fi::endpoints::endpoint_auth_authorize;
use mc2_fi::method::MethodAuthAuthorize;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub app_db: DatabaseConfig,
    pub auth_db: DatabaseConfig,
    #[serde(default)]
    pub log_level: LogLevel,
    #[serde(flatten)]
    pub app: WsServerConfig,
    pub ethereum_urls: EthereumConns,
    #[serde(default)]
    pub setup_ethereum_localnet: bool,
    pub escrow_addresses: Vec<UserGetDepositAddressesRow>,
    pub god_key: String,
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
    server.add_handler(MethodUserFollowStrategy);
    server.add_handler(MethodUserListFollowedStrategies);
    server.add_handler(MethodUserUnfollowStrategy);

    server.add_handler(MethodUserRegisterWallet);
    server.add_handler(MethodUserListRegisteredWallets);
    server.add_handler(MethodUserDeregisterWallet);
    server.add_handler(MethodUserListStrategies);
    server.add_handler(MethodUserListTopPerformingStrategies);
    server.add_handler(MethodUserListStrategyFollowers);
    server.add_handler(MethodUserListStrategyBackers);
    server.add_handler(MethodUserGetStrategy);
    server.add_handler(MethodUserGetStrategyStatistics);
    server.add_handler(MethodUserGetStrategiesStatistics);
    server.add_handler(MethodUserListBackedStrategies);
    server.add_handler(MethodUserListExitStrategyHistory);
    server.add_handler(MethodUserListDepositHistory);
    server.add_handler(MethodUserListStrategyWallets);

    server.add_handler(MethodUserFollowExpert);
    server.add_handler(MethodExpertListFollowers);
    server.add_handler(MethodExpertListBackers);
    server.add_handler(MethodUserListFollowedExperts);

    server.add_handler(MethodUserUnfollowExpert);
    server.add_handler(MethodUserListExperts);
    server.add_handler(MethodUserListTopPerformingExperts);
    server.add_handler(MethodUserListFeaturedExperts);
    server.add_handler(MethodUserGetExpertProfile);

    server.add_handler(MethodUserGetUserProfile);
    server.add_handler(MethodUserUpdateUserProfile);
    server.add_handler(MethodUserApplyBecomeExpert);

    server.add_handler(MethodExpertCreateStrategy);
    server.add_handler(MethodExpertUpdateStrategy);
    server.add_handler(MethodExpertAddStrategyInitialTokenRatio);
    server.add_handler(MethodExpertRemoveStrategyInitialTokenRatio);

    server.add_handler(MethodExpertAddStrategyWatchingWallet);
    server.add_handler(MethodExpertRemoveStrategyWatchingWallet);
    server.add_handler(MethodUserListWalletActivityHistory);
    server.add_handler(MethodUserListStrategyInitialTokenRatio);
    server.add_handler(MethodUserGetDepositTokens);
    server.add_handler(MethodUserGetDepositAddresses {
        addresses: config.escrow_addresses,
    });
    server.add_handler(MethodUserListStrategyAuditRules);

    server.add_handler(MethodAdminListUsers);
    server.add_handler(MethodAdminSetUserRole);
    server.add_handler(MethodAdminSetBlockUser);
    server.add_handler(MethodAdminApproveUserBecomeExpert);
    server.add_handler(MethodAdminRejectUserBecomeExpert);
    server.add_handler(MethodAdminListPendingExpertApplications);
    server.add_handler(MethodAdminGetSystemConfig);
    server.add_handler(MethodAdminUpdateSystemConfig);
    server.add_handler(MethodAdminListBackers);
    server.add_handler(MethodAdminListExperts);
    server.add_handler(MethodAdminListStrategies);

    server.add_handler(MethodAdminAddWalletActivityHistory); // only for mocking purpose
    let eth_pool = EthereumRpcConnectionPool::from_conns(config.ethereum_urls);
    let coin_addresses = Arc::new(BlockchainCoinAddresses::new());
    let escrow_contract_addresses = EscrowAddresses::new();
    let escrow_contract = Arc::new(AbstractEscrowContract::new2(escrow_contract_addresses));
    let master_key = Secp256k1SecretKey::from_str(&config.god_key)?;

    server.add_handler(MethodUserBackStrategy {
        pool: eth_pool.clone(),
        stablecoin_addresses: coin_addresses.clone(),
        escrow_contract: escrow_contract.clone(),
        master_key: master_key.clone(),
        dex_addresses: Arc::new(DexAddresses::new()),
    });
    server.add_handler(MethodUserExitStrategy {
        pool: eth_pool.clone(),
        master_key: master_key.clone(),
    });
    server.add_handler(MethodUserRequestRefund {
        pool: eth_pool,
        stablecoin_addresses: coin_addresses,
        escrow_contract: escrow_contract.clone(),
        master_key: master_key.clone(),
    });
    server.dump_schemas()?;
    server.listen().await?;
    Ok(())
}
