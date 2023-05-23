pub mod tools;

use crypto::openssl::OpensslPrivateKey;
use eth_sdk::signer::EthereumSigner;
use eyre::*;
use gen::model::*;
use lib::log::{setup_logs, LogLevel};
use std::sync::Arc;

use tools::*;

#[path = "../src/service/auth/endpoints.rs"]
pub mod auth_endpoints;
#[tokio::test]
#[should_panic]
async fn test_bad_login() {
    let mut client = get_ws_auth_client("").await.unwrap();
    let res: LoginResponse = client.recv_resp().await.unwrap();
    println!("{:?}", res);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_signup() -> Result<()> {
    let _ = setup_logs(LogLevel::Trace);
    drop_and_recreate_database()?;

    let key = OpensslPrivateKey::new_secp256k1_none("test_eth_key")?;
    let signer = EthereumSigner::new(Arc::new(key))?;
    signup("user1", &signer).await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_login() -> Result<()> {
    let _ = setup_logs(LogLevel::Trace);

    drop_and_recreate_database()?;

    let key = OpensslPrivateKey::new_secp256k1_none("test_eth_key")?;
    let signer = EthereumSigner::new(Arc::new(key))?;
    signup("user1", &signer).await?;

    login("user1", &signer).await?;

    Ok(())
}
