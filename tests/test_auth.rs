pub mod tools;
use crate::auth_endpoints::endpoint_auth_signup;
use eyre::*;
use gen::model::*;
use lib::log::{setup_logs, LogLevel};
use lib::utils::encode_header;
use mc2_fi::endpoints::endpoint_auth_login;
use rand::{rngs::OsRng, Rng};
use secp256k1::SecretKey;
use tools::*;
use web3::signing::Signature;

fn generate_temp_private_key() -> SecretKey {
    let mut rng = OsRng;
    loop {
        let bytes: [u8; 32] = rng.gen();
        if let Ok(key) = SecretKey::from_slice(&bytes) {
            break key;
        }
    }
}
#[path = "../src/service/auth/endpoints.rs"]
pub mod auth_endpoints;
#[tokio::test]
#[should_panic]
async fn test_bad_login() {
    let mut client = get_ws_auth_client("").await.unwrap();
    let res: LoginResponse = client.recv_resp().await.unwrap();
    println!("{:?}", res);
}

#[tokio::test]
async fn test_signup() -> Result<()> {
    test_signup_inner().await?;
    Ok(())
}

async fn test_signup_inner() -> Result<()> {
    let _ = setup_logs(LogLevel::Trace);
    drop_and_recreate_database()?;

    let mut client = get_ws_auth_client(&encode_header(
        SignupRequest {
            address: "0x111013b7862ebc1b9726420aa0e8728de310ee63".to_string(),
            signature_text: "5468697320726571756573742077696c6c206e6f74207472696767657220616e79207472616e73616374696f6e206f7220696e63757220616e7920636f7374206f7220666565732e200a204974206973206f6e6c7920696e74656e64656420746f2061757468656e74696361746520796f752061726520746865206f776e6572206f662077616c6c65743a0a3078313131303133623738363265626331623937323634323061613065383732386465333130656536336e6f6e63653a0a383632353033343139".to_string(),
            signature: "72f8e93e5e2ba1b3df2f179bddac22b691ca86b39f6f7619a9eedd90b16bed165c0e03dcac13e5e2a1a1ea79ab9cf40a6ba572165a7f58525466a42a9699f0ea1c".to_string(),
            email: "qjk2001@gmail.com".to_string(),
            phone: "+00123456".to_string(),
            agreed_tos: true,
            agreed_privacy: true,
            username: Some("test_username".to_string()),
        },
        endpoint_auth_signup(),
    )?)
    .await?;
    let res: SignupResponse = client.recv_resp().await?;
    println!("{:?}", res);
    Ok(())
}
#[tokio::test]
async fn test_login() -> Result<()> {
    test_signup_inner().await?;
    let mut client = get_ws_auth_client(&encode_header(
        LoginRequest {
            address: "0x111013b7862ebc1b9726420aa0e8728de310ee63".to_string(),
            signature_text: "5468697320726571756573742077696c6c206e6f74207472696767657220616e79207472616e73616374696f6e206f7220696e63757220616e7920636f7374206f7220666565732e200a204974206973206f6e6c7920696e74656e64656420746f2061757468656e74696361746520796f752061726520746865206f776e6572206f662077616c6c65743a0a3078313131303133623738363265626331623937323634323061613065383732386465333130656536336e6f6e63653a0a383632353033343139".to_string(),
            signature: "72f8e93e5e2ba1b3df2f179bddac22b691ca86b39f6f7619a9eedd90b16bed165c0e03dcac13e5e2a1a1ea79ab9cf40a6ba572165a7f58525466a42a9699f0ea1c".to_string(),
            service: EnumService::User as _,
            device_id: "24787297130491616".to_string(),
            device_os: "android".to_string(),
        },
        endpoint_auth_login(),
    )?)
    .await?;
    let res: LoginResponse = client.recv_resp().await?;
    println!("{:?}", res);

    Ok(())
}
