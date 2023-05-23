use eth_sdk::signer::EthereumSigner;
use eth_sdk::utils::encode_signature;
use eyre::*;
use gen::client::*;
use gen::model::{
    AuthorizeRequest, AuthorizeResponse, EnumService, LoginRequest, LoginResponse, SignupRequest,
    SignupResponse,
};
use lib::utils::encode_header;
use lib::ws::WsClient;
use mc2_fi::endpoints::{endpoint_auth_authorize, endpoint_auth_login, endpoint_auth_signup};
use std::path::Path;
use std::process::Command;
use tracing::*;
use web3::signing::{hash_message, Key};

pub async fn get_ws_auth_client(header: &str) -> Result<WsClient> {
    let connect_addr = "ws://localhost:8888";
    info!("Connecting to {} with header {}", connect_addr, header);
    let ws_stream = WsClient::new(connect_addr, header).await?;
    Ok(ws_stream)
}
pub async fn auth_login(req: &LoginRequest) -> Result<LoginResponse> {
    let header = encode_header(req, endpoint_auth_login())?;
    let mut client = get_ws_auth_client(&header).await?;
    let resp: LoginResponse = client.recv_resp().await?;
    Ok(resp)
}

pub async fn get_ws_user_client(req: &AuthorizeRequest) -> Result<UserClient> {
    let header = &encode_header(req, endpoint_auth_authorize())?;
    let connect_addr = "ws://localhost:8889";
    info!("Connecting to {} with header {}", connect_addr, header);
    let mut ws_stream = WsClient::new(connect_addr, header).await?;
    let x: AuthorizeResponse = ws_stream.recv_resp().await?;
    info!("AuthorizeResponse {:?}", x);
    Ok(ws_stream.into())
}

pub fn drop_and_recreate_database() -> Result<()> {
    let script = Path::new("scripts/drop_and_recreate_database.sh");
    Command::new("bash")
        .arg(script)
        .arg("etc/config.json")
        .status()?;
    Ok(())
}

pub async fn signup(username: impl Into<String>, signer: &EthereumSigner) -> Result<()> {
    let txt = format!("Signup {}", username.into());
    let signature = signer.sign_message(hash_message(txt.as_bytes()).as_bytes())?;
    let mut client = get_ws_auth_client(&encode_header(
        SignupRequest {
            address: format!("{:?}", signer.address),
            signature_text: hex::encode(&txt),
            signature: encode_signature(&signature),
            email: "qjk2001@gmail.com".to_string(),
            phone: "+00123456".to_string(),
            agreed_tos: true,
            agreed_privacy: true,
            username: None,
        },
        endpoint_auth_signup(),
    )?)
    .await?;
    let res: SignupResponse = client.recv_resp().await?;
    info!("{:?}", res);
    Ok(())
}
pub async fn login(username: impl Into<String>, signer: &EthereumSigner) -> Result<LoginResponse> {
    let txt = format!("Login {}", username.into());
    let signature = signer.sign_message(hash_message(txt.as_bytes()).as_bytes())?;
    let mut client = get_ws_auth_client(&encode_header(
        LoginRequest {
            address: format!("{:?}", signer.address),
            signature_text: hex::encode(txt),
            signature: encode_signature(&signature),
            service: EnumService::User as _,
            device_id: "24787297130491616".to_string(),
            device_os: "android".to_string(),
        },
        endpoint_auth_login(),
    )?)
    .await?;
    let res: LoginResponse = client.recv_resp().await?;
    println!("{:?}", res);
    Ok(res)
}
pub async fn connect_user(
    username: impl Into<String>,
    signer: &EthereumSigner,
) -> Result<UserClient> {
    let login = login(username, signer).await?;
    let client = get_ws_user_client(&AuthorizeRequest {
        address: login.address,
        token: login.user_token,
        service: EnumService::User as _,
        device_id: "24787297130491616".to_string(),
        device_os: "android".to_string(),
    })
    .await?;
    Ok(client)
}
