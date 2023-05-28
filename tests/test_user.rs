pub mod tools;

use eth_sdk::signer::{EthereumSigner, Secp256k1SecretKey};
use eth_sdk::utils::encode_signature;
use eyre::*;
use gen::model::*;
use lib::database::drop_and_recreate_database;
use lib::log::{setup_logs, LogLevel};
use std::sync::Arc;
use tools::*;
use tracing::*;
use web3::signing::{hash_message, Key};

#[tokio::test]
async fn test_register_wallet() -> Result<()> {
    let _ = setup_logs(LogLevel::Info);

    drop_and_recreate_database()?;
    let key = Secp256k1SecretKey::new_random();
    let signer = EthereumSigner::new(Arc::new(key))?;

    signup("user1", &signer).await?;

    let mut client = connect_user("user1", &signer).await?;

    let txt = format!("Register {}", "wallet");
    let signature = signer.sign_message(hash_message(txt.as_bytes()).as_bytes())?;

    let resp = client
        .user_register_wallet(UserRegisterWalletRequest {
            blockchain: "ethereum".to_string(),
            wallet_address: format!("{:?}", signer.address),
            message_to_sign: hex::encode(txt),
            message_signature: encode_signature(&signature),
            strategy_id: 0, // TODO: create strategy id before filling this
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

    let key = Secp256k1SecretKey::new_random();
    let signer = EthereumSigner::new(Arc::new(key))?;

    signup("user1", &signer).await?;

    let mut client = connect_user("user1", &signer).await?;

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
            blockchain: "ethereum".to_string(),
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
