use api::cmc::CoinMarketCap;
use eth_sdk::escrow::AbstractEscrowContract;
use eth_sdk::pancake_swap::pair_paths::WorkingPancakePairPaths;
use eth_sdk::signer::Secp256k1SecretKey;
use eth_sdk::{DexAddresses, EthereumConns, EthereumRpcConnectionPool};
use eyre::*;
use gen::model::EnumService;
use lib::config::load_config;
use lib::database::{connect_to_database, DatabaseConfig};
use lib::log::{setup_logs, LogLevel};
use lib::ws::{EndpointAuthController, SubscribeManager, WebsocketServer, WsServerConfig};
use lru::LruCache;
use mc2fi_asset_price::AssetPriceClient;
use mc2fi_auth::endpoints::endpoint_auth_authorize;
use mc2fi_auth::method::MethodAuthAuthorize;
use mc2fi_user::admin_method::*;
use mc2fi_user::audit::AuditLogger;
use mc2fi_user::method::*;
use mc2fi_user::shared_method::{load_allow_domain_urls, load_coin_addresses, load_escrow_address};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use std::fmt::Debug;
use std::num::NonZeroUsize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Deserialize)]
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
    pub god_key: SecretString,
    pub cmc_api_key: SecretString,
}
#[tokio::main]
async fn main() -> Result<()> {
    let mut config: Config = load_config("user".to_owned())?;
    setup_logs(config.log_level)?;
    let audit_logger = AuditLogger::new()?;
    let mut server = WebsocketServer::new(config.app.clone());
    let db = connect_to_database(config.app_db).await?;
    let asset_client = Arc::new(AssetPriceClient::new(db.clone()));
    load_allow_domain_urls(&db, &mut config.app).await?;
    server.add_database(db.clone());
    server.add_database(connect_to_database(config.auth_db).await?);

    let mut auth_controller = EndpointAuthController::new();
    auth_controller.add_auth_endpoint(
        endpoint_auth_authorize(),
        MethodAuthAuthorize {
            accept_service: EnumService::User,
        },
    );
    server.add_auth_controller(auth_controller);

    let coin_addresses = load_coin_addresses(&db).await?;

    let escrow_contract_address = load_escrow_address(&db).await?;
    server.add_handler(MethodUserFollowStrategy);
    server.add_handler(MethodUserListFollowedStrategies {
        asset_client: asset_client.clone(),
    });
    server.add_handler(MethodUserUnfollowStrategy);

    server.add_handler(MethodUserWhitelistWallet);
    server.add_handler(MethodUserListWhitelistedWallets);
    server.add_handler(MethodUserUnwhitelistWallet);
    server.add_handler(MethodUserListStrategies {
        asset_client: asset_client.clone(),
    });
    server.add_handler(MethodUserListTopPerformingStrategies {
        asset_client: asset_client.clone(),
    });
    server.add_handler(MethodUserListStrategyFollowers);
    server.add_handler(MethodUserListStrategyBackers);
    server.add_handler(MethodUserGetStrategy {
        asset_client: asset_client.clone(),
    });
    server.add_handler(MethodUserListStrategyPoolContractAssetLedger);
    server.add_handler(MethodUserListUserStrategyPoolContractAssetLedger);
    server.add_handler(MethodUserGetStrategyStatistics);
    server.add_handler(MethodUserGetStrategiesStatistics {
        asset_client: asset_client.clone(),
    });
    server.add_handler(MethodUserListBackedStrategies {
        asset_client: asset_client.clone(),
    });

    server.add_handler(MethodUserListDepositWithdrawLedger);
    server.add_handler(MethodUserListStrategyWallets);

    server.add_handler(MethodUserFollowExpert);
    server.add_handler(MethodExpertListFollowers);
    server.add_handler(MethodExpertListBackers);
    server.add_handler(MethodUserListFollowedExperts);
    server.add_handler(MethodExpertListPublishedStrategies {
        asset_client: asset_client.clone(),
    });
    server.add_handler(MethodExpertListUnpublishedStrategies {
        asset_client: asset_client.clone(),
    });

    server.add_handler(MethodUserUnfollowExpert);
    server.add_handler(MethodUserListExperts);
    server.add_handler(MethodUserListTopPerformingExperts);
    server.add_handler(MethodUserListFeaturedExperts);
    server.add_handler(MethodUserListExpertListenedWalletTradeLedger);
    server.add_handler(MethodUserGetExpertProfile {
        asset_client: asset_client.clone(),
    });

    server.add_handler(MethodUserGetUserProfile {
        asset_client: asset_client.clone(),
    });
    server.add_handler(MethodUserUpdateUserProfile);
    server.add_handler(MethodUserApplyBecomeExpert);

    server.add_handler(MethodUserListUserBackStrategyAttempt);
    server.add_handler(MethodUserListUserBackStrategyLog);

    server.add_handler(MethodExpertCreateStrategy {
        cmc_client: Arc::new(CoinMarketCap::new(config.cmc_api_key.expose_secret())?),
    });
    server.add_handler(MethodExpertUpdateStrategy {
        logger: audit_logger.clone(),
    });
    // TODO: move them to MethodExpertUpdateStrategy
    // server.add_handler(MethodExpertAddStrategyInitialTokenRatio {
    //     logger: audit_logger.clone(),
    // });
    // server.add_handler(MethodExpertRemoveStrategyInitialTokenRatio {
    //     logger: audit_logger.clone(),
    // });

    server.add_handler(MethodExpertRemoveStrategyWatchingWallet {
        logger: audit_logger.clone(),
    });
    server.add_handler(MethodExpertListBackStrategyLedger);
    server.add_handler(MethodExpertListExitStrategyLedger);
    server.add_handler(MethodUserListStrategyInitialTokenRatio);
    server.add_handler(MethodUserGetDepositTokens {
        coin_addresses: coin_addresses.clone(),
    });
    server.add_handler(MethodUserListStrategyAuditRules);
    server.add_handler(MethodUserListDepositWithdrawBalances);
    server.add_handler(MethodUserGetDepositWithdrawBalance {
        escrow_addresses: escrow_contract_address.clone(),
    });
    server.add_handler(MethodUserListEscrowTokenContractAddresses);
    server.add_handler(MethodUserListBackStrategyLedger);
    server.add_handler(MethodUserListExitStrategyLedger);
    server.add_handler(MethodUserListStrategyTokenBalance);
    // they are basically the same but MethodUserGetEscrowAddressForStrategy is more user friendly
    server.add_handler(MethodUserGetDepositAddresses {
        addresses: escrow_contract_address.clone(),
    });
    server.add_handler(MethodUserGetEscrowAddressForStrategy {
        addresses: escrow_contract_address.clone(),
    });
    server.add_handler(MethodUserGetSystemConfig);
    server.add_handler(MethodUserListUserStrategyBalance);

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
    server.add_handler(MethodAdminListWhitelists);
    server.add_handler(MethodAdminListStrategies {
        asset_client: asset_client.clone(),
    });
    server.add_handler(MethodAdminListBackStrategyLedger);
    server.add_handler(MethodAdminListExitStrategyLedger);
    server.add_handler(MethodAdminRejectStrategy);
    server.add_handler(MethodAdminSetBlockchainLogger);
    let sub_manager = SubscribeManager::new();
    sub_manager.add_topic(AdminSubscribeTopic::AdminNotifyEscrowLedgerChange);
    let sub_manager = Arc::new(sub_manager);
    server.add_handler(MethodAdminNotifyEscrowLedgerChange {
        manager: Arc::clone(&sub_manager),
    });

    server.add_handler(MethodUserSubscribeDepositLedger {
        manger: Arc::clone(&sub_manager),
    });
    server.add_handler(MethodUserUnsubscribeDepositLedger {
        manger: Arc::clone(&sub_manager),
    });

    server.add_handler(MethodAdminSubscribeDepositLedger {
        manger: Arc::clone(&sub_manager),
    });
    server.add_handler(MethodAdminUnsubscribeDepositLedger {
        manger: Arc::clone(&sub_manager),
    });

    server.add_handler(MethodAdminAddAuditRule);
    server.add_handler(MethodAdminAddEscrowContractAddress);

    let eth_pool = EthereumRpcConnectionPool::from_conns(config.ethereum_urls);
    server.add_handler(MethodAdminListEscrowTokenContractAddresses {
        asset_client: Arc::new(CoinMarketCap::new(config.cmc_api_key.expose_secret())?),
    });
    server.add_handler(MethodAdminUpdateEscrowTokenContractAddress);
    let escrow_contract = Arc::new(AbstractEscrowContract::new2(
        escrow_contract_address.clone(),
    ));
    let master_key = Secp256k1SecretKey::from_str(config.god_key.expose_secret())?;

    let pancake_paths = WorkingPancakePairPaths::new(coin_addresses.clone(), eth_pool.clone())?;
    let pancake_paths = Arc::new(pancake_paths);
    server.add_handler(MethodExpertAddStrategyWatchingWallet {
        logger: audit_logger.clone(),
        pool: eth_pool.clone(),
    });
    server.add_handler(MethodAdminApproveStrategy {
        pool: eth_pool.clone(),
    });
    server.add_handler(MethodAdminRefreshExpertWalletBalance {
        pool: eth_pool.clone(),
    });
    server.add_handler(MethodUserCreateStrategyWallet {
        pool: eth_pool.clone(),
        master_key: master_key.clone(),
    });
    server.add_handler(MethodUserGetBackStrategyReviewDetail {
        pool: eth_pool.clone(),
        escrow_contract: escrow_contract.clone(),
        master_key: master_key.clone(),
        dex_addresses: Arc::new(DexAddresses::new()),
        asset_client: asset_client.clone(),
        pancake_paths: pancake_paths.clone(),
    });
    // let lru = Arc::new(Mutex::new(LruCache::new(NonZeroUsize::new(1000).unwrap())));
    let lru = Arc::new(Mutex::new(LruCache::new(NonZeroUsize::new(1000).unwrap().into())));
    server.add_handler(MethodUserBackStrategy {
        pool: eth_pool.clone(),
        escrow_contract: escrow_contract.clone(),
        master_key: master_key.clone(),
        dex_addresses: Arc::new(DexAddresses::new()),
        subscribe_manager: Arc::clone(&sub_manager),
        lru: lru.clone(),
        pancake_paths: pancake_paths.clone(),
        asset_client: asset_client.clone(),
    });
    server.add_handler(MethodUserExitStrategy {
        pool: eth_pool.clone(),
        master_key: master_key.clone(),
        lru: lru.clone(),
    });
    server.add_handler(MethodUserRequestRefund {
        pool: eth_pool,
        stablecoin_addresses: coin_addresses,
        escrow_contract: escrow_contract.clone(),
        master_key: master_key.clone(),
        lru,
    });
    server.dump_schemas()?;
    server.listen().await?;
    Ok(())
}
