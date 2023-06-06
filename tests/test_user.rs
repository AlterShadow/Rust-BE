pub mod tools;

use eth_sdk::signer::{Secp256k1SecretKey};
use eth_sdk::utils::encode_signature;
use eyre::*;
use gen::database::FunAuthSetRoleReq;
use gen::model::*;
use lib::database::{
    connect_to_database, database_test_config, drop_and_recreate_database,
};
use lib::log::{setup_logs, LogLevel};


use tools::*;
use tracing::*;
use web3::signing::{hash_message, Key};

#[tokio::test]
async fn test_register_wallet() -> Result<()> {
    let _ = setup_logs(LogLevel::Info);

    drop_and_recreate_database()?;
    let signer = Secp256k1SecretKey::new_random();

    signup("user1", &signer.key).await?;

    let mut client = connect_user("user1", &signer.key).await?;

    let txt = format!("Register {}", "wallet");
    let signature = signer.sign_message(hash_message(txt.as_bytes()).as_bytes())?;

    let resp = client
        .user_register_wallet(UserRegisterWalletRequest {
            blockchain: EnumBlockChain::LocalNet,
            wallet_address: format!("{:?}", signer.address),
            message_to_sign: hex::encode(txt),
            message_signature: encode_signature(&signature),
        })
        .await?;
    info!("Register wallet {:?}", resp);
    client
        .user_deregister_wallet(UserDeregisterWalletRequest {
            wallet_id: resp.wallet_id,
        })
        .await?;
    Ok(())
}
#[tokio::test]
async fn test_create_update_strategy() -> Result<()> {
    let _ = setup_logs(LogLevel::Info);
    drop_and_recreate_database()?;

    let signer = Secp256k1SecretKey::new_random();

    signup("user1", &signer.key).await?;

    let mut client = connect_user("user1", &signer.key).await?;

    let resp = client
        .user_create_strategy(UserCreateStrategyRequest {
            name: "test_strategy".to_string(),
            description: "this is a test strategy".to_string(),
        })
        .await?;
    info!("Register wallet {:?}", resp);
    client
        .user_update_strategy(UserUpdateStrategyRequest {
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
        .user_add_strategy_watching_wallet(UserAddStrategyWatchingWalletRequest {
            strategy_id: resp.strategy_id,
            blockchain: EnumBlockChain::LocalNet,
            wallet_address: "0x000000000001".to_string(),
            ratio: 1.0,
        })
        .await?;
    info!("Add wallet {:?}", wallet);
    let remove_wallet = client
        .user_remove_strategy_watching_wallet(UserRemoveStrategyWatchingWalletRequest {
            wallet_id: wallet.wallet_id,
        })
        .await?;
    info!("Remove wallet {:?}", remove_wallet);
    Ok(())
}

#[tokio::test]
async fn test_user_become_expert() -> Result<()> {
    let _ = setup_logs(LogLevel::Info);
    drop_and_recreate_database()?;

    let admin = Secp256k1SecretKey::new_random();
    signup("admin", &admin.key).await?;
    let (_admin_client, admin_login) = connect_user_ext("admin", &admin.key).await?;
    let db = connect_to_database(database_test_config()).await?;
    db.execute(FunAuthSetRoleReq {
        public_user_id: admin_login.user_id,
        role: EnumRole::Admin,
    })
    .await?;
    let mut admin_client = connect_user("admin", &admin.key).await?;

    let user = Secp256k1SecretKey::new_random();
    signup("user1", &user.key).await?;

    let mut client = connect_user("user1", &user.key).await?;
    let resp = client
        .user_apply_become_expert(UserApplyBecomeExpertRequest {})
        .await?;
    info!("Register wallet {:?}", resp);

    let resp = admin_client
        .admin_list_pending_expert_applications(AdminListPendingExpertApplicationsRequest {})
        .await?;
    assert_eq!(resp.users.len(), 1);
    let resp = admin_client
        .admin_approve_user_become_expert(AdminApproveUserBecomeExpertRequest {
            user_id: resp.users[0].user_id,
        })
        .await?;
    info!("Approve {:?}", resp);
    Ok(())
}
