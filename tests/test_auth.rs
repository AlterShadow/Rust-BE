pub mod tools;
use crate::auth_endpoints::endpoint_auth_signup;
use eyre::*;
use gen::model::*;
use lib::utils::encode_header;
use mc2_fi::endpoints::endpoint_auth_login;
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

#[tokio::test]
async fn test_login() -> Result<()> {
    let mut client = get_ws_auth_client(&encode_header(
        LoginRequest {
            username: "pepe_pablo".to_string(),
            password: "AHJQ6X1H68SK8D9P6WW0".to_string(),
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

#[tokio::test]
async fn test_signup() -> Result<()> {
    let mut client = get_ws_auth_client(&encode_header(
        SignupRequest {
            username: "pepe_pablo".to_string(),
            password: "AHJQ6X1H68SK8D9P6WW0".to_string(),
            email: "pepe_pablo@defi.digital".to_string(),
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
