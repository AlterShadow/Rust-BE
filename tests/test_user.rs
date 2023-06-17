pub mod tools;

use eth_sdk::signer::Secp256k1SecretKey;
use eyre::*;
use gen::model::*;
use lib::database::drop_and_recreate_database;
use lib::log::{setup_logs, LogLevel};
use lib::ws::WsClient;
use tools::*;
use tracing::*;

#[tokio::test]
async fn test_create_update_strategy() -> Result<()> {
    let _ = setup_logs(LogLevel::Info);
    drop_and_recreate_database()?;
    let (_admin, _admin_client, user, mut client) = prepare_expert().await?;
    let resp = client
        .request(ExpertCreateStrategyRequest {
            name: "test_strategy".to_string(),
            description: "this is a test strategy".to_string(),
            strategy_thesis_url: "".to_string(),
            minimum_backing_amount_usd: 0.0,
            strategy_fee: 0.0,
            expert_fee: 0.0,
            agreed_tos: true,
            wallet_address: format!("{:?}", user.address),
        })
        .await?;
    info!("Register wallet {:?}", resp);
    client
        .request(ExpertUpdateStrategyRequest {
            strategy_id: resp.strategy_id,
            name: None,
            description: None,
            social_media: None,
            risk_score: None,
            reputation_score: None,
            aum: None,
        })
        .await?;
    let wallet = client
        .request(ExpertAddStrategyWatchingWalletRequest {
            strategy_id: resp.strategy_id,
            blockchain: EnumBlockChain::LocalNet,
            wallet_address: "0x000000000001".to_string(),
            ratio: 1.0,
        })
        .await?;
    info!("Add wallet {:?}", wallet);
    let remove_wallet = client
        .request(ExpertRemoveStrategyWatchingWalletRequest {
            wallet_id: wallet.wallet_id,
        })
        .await?;
    info!("Remove wallet {:?}", remove_wallet);
    Ok(())
}

#[tokio::test]
async fn test_user_follow_strategy() -> Result<()> {
    let _ = setup_logs(LogLevel::Info);
    drop_and_recreate_database()?;
    let (_admin, _admin_client, user, mut client) = prepare_expert().await?;

    let create_strategy_resp = client
        .request(ExpertCreateStrategyRequest {
            name: "test_strategy".to_string(),
            description: "this is a test strategy".to_string(),
            strategy_thesis_url: "".to_string(),
            minimum_backing_amount_usd: 0.0,
            strategy_fee: 0.0,
            expert_fee: 0.0,
            agreed_tos: true,
            wallet_address: format!("{:?}", user.address),
        })
        .await?;
    info!("User Create Strategy {:?}", create_strategy_resp);

    let resp = client
        .request(UserFollowStrategyRequest {
            strategy_id: create_strategy_resp.strategy_id,
        })
        .await?;
    info!("User Follow Strategy {:?}", resp);
    let resp = client
        .request(UserListFollowedStrategiesRequest {
            limit: None,
            offset: None,
        })
        .await?;
    assert_eq!(resp.strategies.len(), 1);
    info!("User List Followed Strategies {:?}", resp);
    let resp = client
        .request(UserUnfollowStrategyRequest {
            strategy_id: create_strategy_resp.strategy_id,
        })
        .await?;
    info!("User Unfollow Strategy {:?}", resp);
    let resp = client
        .request(UserListFollowedStrategiesRequest {
            limit: None,
            offset: None,
        })
        .await?;
    assert_eq!(resp.strategies.len(), 0);
    let resp = client
        .request(UserListStrategiesRequest {
            limit: None,
            offset: None,
            strategy_id: None,
            strategy_name: None,
            expert_public_id: None,
            expert_name: None,
            description: None,
        })
        .await?;
    assert_eq!(resp.strategies.len(), 1);
    info!("User List Strategies {:?}", resp);
    Ok(())
}

#[tokio::test]
async fn test_user_follow_strategy_get_user_profile() -> Result<()> {
    let _ = setup_logs(LogLevel::Info);
    drop_and_recreate_database()?;
    let (_admin, _admin_client, user, mut client) = prepare_expert().await?;

    let create_strategy_resp = client
        .request(ExpertCreateStrategyRequest {
            name: "test_strategy".to_string(),
            description: "this is a test strategy".to_string(),
            strategy_thesis_url: "".to_string(),
            minimum_backing_amount_usd: 0.0,
            strategy_fee: 0.0,
            expert_fee: 0.0,
            agreed_tos: true,
            wallet_address: format!("{:?}", user.address),
        })
        .await?;
    info!("User Create Strategy {:?}", create_strategy_resp);

    let resp = client
        .request(UserFollowStrategyRequest {
            strategy_id: create_strategy_resp.strategy_id,
        })
        .await?;
    info!("User Follow Strategy {:?}", resp);
    let resp = client
        .request(UserListFollowedStrategiesRequest {
            limit: None,
            offset: None,
        })
        .await?;
    assert_eq!(resp.strategies.len(), 1);

    let resp = client
        .request(UserUnfollowStrategyRequest {
            strategy_id: create_strategy_resp.strategy_id,
        })
        .await?;
    info!("User Unfollow Strategy {:?}", resp);
    let resp = client.request(UserGetUserProfileRequest {}).await?;
    info!("User Profile {:?}", resp);
    Ok(())
}
#[tokio::test]
async fn test_user_list_strategies() -> Result<()> {
    let _ = setup_logs(LogLevel::Info);
    drop_and_recreate_database()?;
    let (_admin, _admin_client, user, mut client) = prepare_expert().await?;

    let create_strategy_resp = client
        .request(ExpertCreateStrategyRequest {
            name: "test_strategy".to_string(),
            description: "this is a test strategy".to_string(),
            strategy_thesis_url: "".to_string(),
            minimum_backing_amount_usd: 0.0,
            strategy_fee: 0.0,
            expert_fee: 0.0,
            agreed_tos: true,
            wallet_address: format!("{:?}", user.address),
        })
        .await?;
    info!("User Create Strategy {:?}", create_strategy_resp);

    let resp = client
        .request(UserListStrategiesRequest {
            limit: None,
            offset: None,
            strategy_id: None,
            strategy_name: None,
            expert_public_id: None,
            expert_name: None,
            description: None,
        })
        .await?;
    info!("User List Strategies {:?}", resp);
    assert_eq!(resp.strategies.len(), 1);
    // TODO: should be non zero to test pg functions
    let resp = client
        .request(UserListFollowedStrategiesRequest {
            limit: None,
            offset: None,
        })
        .await?;
    assert_eq!(resp.strategies.len(), 0);
    let resp = client
        .request(UserListTopPerformingStrategiesRequest {
            limit: None,
            offset: None,
        })
        .await?;
    info!("User List Top Performing Strategies {:?}", resp);
    assert_eq!(resp.strategies.len(), 1);
    let resp = client
        .request(UserListBackedStrategiesRequest {
            limit: None,
            offset: None,
        })
        .await?;
    info!("User List Backed Strategies {:?}", resp);
    assert_eq!(resp.strategies.len(), 0);
    Ok(())
}
#[tokio::test]
async fn test_user_become_expert() -> Result<()> {
    let _ = setup_logs(LogLevel::Info);
    drop_and_recreate_database()?;
    prepare_expert().await?;

    Ok(())
}

#[tokio::test]
async fn test_user_follow_expert() -> Result<()> {
    let _ = setup_logs(LogLevel::Info);
    drop_and_recreate_database()?;

    let admin = Secp256k1SecretKey::new_random();
    signup("dev-admin", &admin.key).await?;
    let mut admin_client = connect_user("dev-admin", &admin.key).await?;

    let user = Secp256k1SecretKey::new_random();
    signup("user1", &user.key).await?;

    let mut client = connect_user("user1", &user.key).await?;
    let apply_become_expert_resp = client.request(UserApplyBecomeExpertRequest {}).await?;
    info!("User Apply Become Expert {:?}", apply_become_expert_resp);

    let resp = admin_client
        .request(AdminListPendingExpertApplicationsRequest {
            offset: None,
            limit: None,
        })
        .await?;
    assert_eq!(resp.users.len(), 1);

    let resp = admin_client
        .request(AdminApproveUserBecomeExpertRequest {
            user_id: resp.users[0].user_id,
        })
        .await?;
    info!("Approve {:?}", resp);
    let user = Secp256k1SecretKey::new_random();
    signup("user2", &user.key).await?;

    let mut client = connect_user("user2", &user.key).await?;
    let resp = client
        .request(UserFollowExpertRequest {
            expert_id: apply_become_expert_resp.expert_id,
        })
        .await?;
    info!("Follow {:?}", resp);
    assert!(resp.success);
    let resp = client
        .request(UserListFollowedExpertsRequest {
            limit: None,
            offset: None,
        })
        .await?;
    info!("List followed experts {:?}", resp);
    assert_eq!(resp.experts.len(), 1);
    assert_eq!(
        resp.experts[0].expert_id,
        apply_become_expert_resp.expert_id
    );
    let resp = client
        .request(UserUnfollowExpertRequest {
            expert_id: apply_become_expert_resp.expert_id,
        })
        .await?;
    info!("Unfollow {:?}", resp);
    assert!(resp.success);
    let resp = client
        .request(UserListFollowedExpertsRequest {
            limit: None,
            offset: None,
        })
        .await?;
    info!("List followed experts {:?}", resp);
    assert_eq!(resp.experts.len(), 0);
    Ok(())
}

#[tokio::test]
async fn test_user_list_experts() -> Result<()> {
    let _ = setup_logs(LogLevel::Info);
    drop_and_recreate_database()?;
    prepare_expert().await?;

    let user = Secp256k1SecretKey::new_random();
    signup("user2", &user.key).await?;

    let mut client = connect_user("user2", &user.key).await?;
    let resp = client
        .request(UserListExpertsRequest {
            limit: None,
            offset: None,
            expert_id: None,
            user_id: None,
            user_public_id: None,
            username: None,
            family_name: None,
            given_name: None,
            description: None,
            social_media: None,
        })
        .await?;
    info!("Experts {:?}", resp);
    let resp = client
        .request(UserListFeaturedExpertsRequest {
            limit: None,
            offset: None,
        })
        .await?;
    info!("Featured {:?}", resp);
    let resp = client
        .request(UserListTopPerformingExpertsRequest {
            limit: None,
            offset: None,
        })
        .await?;
    info!("Top performing {:?}", resp);
    Ok(())
}
