pub mod tools;

use eyre::*;
use gen::model::*;
use lib::database::drop_and_recreate_database;
use lib::log::{setup_logs, LogLevel};
use tools::*;
use tracing::*;

#[tokio::test]
async fn test_admin_list_strategies() -> Result<()> {
    let _ = setup_logs(LogLevel::Info);
    drop_and_recreate_database()?;
    let (_admin, mut admin_client, user, mut client) = prepare_expert().await?;

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
            wallet_blockchain: EnumBlockChain::EthereumMainnet,
            audit_rules: None,
        })
        .await?;
    info!("User Create Strategy {:?}", create_strategy_resp);

    let resp = admin_client
        .request(AdminListStrategiesRequest {
            limit: None,
            strategy_id: None,
            strategy_name: None,
            expert_public_id: None,
            expert_name: None,
            description: None,
            pending_approval: None,
            offset: None,
            approved: None,
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

    let (_admin, mut admin_client, _user, _client) = prepare_expert().await?;

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
