pub mod tools;

use eth_sdk::signer::Secp256k1SecretKey;
use eth_sdk::utils::encode_signature;
use eyre::*;
use gen::model::*;
use lib::database::drop_and_recreate_database;
use lib::log::{setup_logs, LogLevel};
use tools::*;
use tracing::*;
use web3::signing::{hash_message, Key};

#[tokio::test]
async fn test_admin_list_strategies() -> Result<()> {
    let _ = setup_logs(LogLevel::Info);
    drop_and_recreate_database()?;
    let admin = Secp256k1SecretKey::new_random();
    signup("dev-admin", &admin.key).await?;
    let mut admin_client = connect_user("dev-admin", &admin.key).await?;

    let user = Secp256k1SecretKey::new_random();
    signup("user1", &user.key).await?;

    let mut client = connect_user("user1", &user.key).await?;
    let resp = client.request(UserApplyBecomeExpertRequest {}).await?;
    info!("User Apply Become Expert {:?}", resp);

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

    let create_strategy_resp = client
        .request(ExpertCreateStrategyRequest {
            name: "test_strategy".to_string(),
            description: "this is a test strategy".to_string(),
            strategy_thesis_url: "".to_string(),
            minimum_backing_amount_usd: 0.0,
            strategy_fee: 0.0,
            expert_fee: 0.0,
            agreed_tos: true,
            linked_wallets: vec![],
        })
        .await?;
    info!("User Create Strategy {:?}", create_strategy_resp);

    let resp = admin_client
        .request(AdminListStrategiesRequest {
            limit: None,
            offset: None,
        })
        .await?;
    info!("User List Backed Strategies {:?}", resp);
    assert_eq!(resp.strategies.len(), 1);
    Ok(())
}

#[tokio::test]
async fn test_admin_list_experts() -> Result<()> {
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

    let resp = admin_client
        .request(AdminListExpertsRequest {
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
    Ok(())
}
