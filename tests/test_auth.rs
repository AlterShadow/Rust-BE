pub mod tools;
use crate::auth_endpoints::endpoint_auth_signup;
use eyre::*;
use gen::model::*;
use lib::utils::encode_header;
use mc2_fi::endpoints::endpoint_auth_login;
use rand::{rngs::OsRng, Rng};
use secp256k1::SecretKey;
use tools::*;
use web3::signing::{Key, SecretKeyRef, Signature};

fn generate_temp_private_key() -> SecretKey {
    let mut rng = OsRng;
    let private_key = loop {
        let bytes: [u8; 32] = rng.gen();
        if let Ok(key) = SecretKey::from_slice(&bytes) {
            break key;
        }
    };
    private_key
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
async fn test_login() -> Result<()> {
    let key = generate_temp_private_key();
    let text = "Hii".to_string();
    let sig = SecretKeyRef::new(&key).sign_message(&text.as_bytes())?;

    let mut client = get_ws_auth_client(&encode_header(
        LoginRequest {
            address: SecretKeyRef::new(&key).address().to_string(),
            signature_text: text,
            signature: encode_signature(&sig),
            service_code: EnumService::User as _,
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
fn encode_signature(sig: &Signature) -> String {
    let mut sig_bytes = vec![];
    sig_bytes.extend_from_slice(sig.r.as_bytes());
    sig_bytes.extend_from_slice(sig.s.as_bytes());
    sig_bytes.push(sig.v as u8);
    hex::encode(sig_bytes)
}
#[tokio::test]
async fn test_signup() -> Result<()> {
    drop_and_recreate_database()?;

    let key = generate_temp_private_key();
    let text = "Hii".to_string();
    let sig = SecretKeyRef::new(&key).sign_message(&text.as_bytes())?;
    let mut client = get_ws_auth_client(&encode_header(
        SignupRequest {
            address: SecretKeyRef::new(&key).address().to_string(),
            signature_text: text,
            signature: encode_signature(&sig),
            email: "qjk2001@gmail.com".to_string(),
            phone: "+00123456".to_string(),
            agreed_tos: true,
            agreed_privacy: true,
        },
        endpoint_auth_signup(),
    )?)
    .await?;
    let res: SignupResponse = client.recv_resp().await?;
    println!("{:?}", res);
    Ok(())
}
