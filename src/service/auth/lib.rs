use crate::endpoints::{endpoint_auth_authorize, endpoint_auth_login, endpoint_auth_signup};
use eth_sdk::utils::get_signed_text;
use eyre::*;
use gen::model::{
    AuthorizeRequest, AuthorizeResponse, EnumService, LoginRequest, LoginResponse, SignupRequest,
    SignupResponse,
};
use lib::utils::encode_header;
use lib::ws::WsClient;
use siwe::Version;
use time::OffsetDateTime;
use tracing::info;
use web3::signing::Key;
use web3::types::Address;

pub mod endpoints;
pub mod method;

fn get_signature_message(address: Address) -> String {
    let sign_text = siwe::Message {
        domain: "mc2.pathscale.com".parse().unwrap(),
        address: address.into(),
        statement: None,
        uri: "https://mc2.pathscale.com".parse().unwrap(),
        version: Version::V1,
        chain_id: 1,
        nonce: rand::random::<u64>().to_string(),
        issued_at: OffsetDateTime::now_utc().into(),
        expiration_time: None,
        not_before: None,
        request_id: None,
        resources: vec![],
    };
    sign_text.to_string()
}
pub async fn signup(
    url: &str,
    username: impl Into<String>,
    signer: impl Key + Clone,
) -> Result<()> {
    let username = username.into();
    let sign_text = get_signature_message(signer.address());
    let (txt, sig) = get_signed_text(sign_text.to_string(), signer.clone())?;

    let mut client = get_ws_auth_client(
        url,
        &encode_header(
            SignupRequest {
                address: signer.address().into(),
                signature_text: txt,
                signature: sig,
                email: "qjk2001@gmail.com".to_string(),
                phone: "+00123456".to_string(),
                agreed_tos: true,
                agreed_privacy: true,
                username,
            },
            endpoint_auth_signup(),
        )?,
    )
    .await?;
    let res: SignupResponse = client.recv_resp().await?;
    info!("{:?}", res);
    Ok(())
}
pub async fn login(
    url: &str,
    username: impl Into<String>,
    signer: impl Key + Clone,
) -> Result<LoginResponse> {
    let _username = username.into();
    let sign_text = get_signature_message(signer.address());
    let (txt, sig) = get_signed_text(sign_text.to_string(), signer.clone())?;
    let mut client = get_ws_auth_client(
        url,
        &encode_header(
            LoginRequest {
                address: signer.address().into(),
                signature_text: txt,
                signature: sig,
                service: EnumService::User as _,
                device_id: "24787297130491616".to_string(),
                device_os: "android".to_string(),
            },
            endpoint_auth_login(),
        )?,
    )
    .await?;
    let res: LoginResponse = client.recv_resp().await?;
    info!("{:?}", res);
    Ok(res)
}

pub async fn get_ws_auth_client(url: &str, header: &str) -> Result<WsClient> {
    info!("Connecting to {} with header {}", url, header);
    let ws_stream = WsClient::new(url, header).await?;
    Ok(ws_stream)
}
pub async fn auth_login(url: &str, req: &LoginRequest) -> Result<LoginResponse> {
    let header = encode_header(req, endpoint_auth_login())?;
    let mut client = get_ws_auth_client(url, &header).await?;
    let resp: LoginResponse = client.recv_resp().await?;
    Ok(resp)
}

pub async fn get_ws_user_client(url: &str, req: &AuthorizeRequest) -> Result<WsClient> {
    let header = &encode_header(req, endpoint_auth_authorize())?;

    info!("Connecting to {} with header {}", url, header);
    let mut ws_stream = WsClient::new(url, header).await?;
    let x: AuthorizeResponse = ws_stream.recv_resp().await?;
    info!("AuthorizeResponse {:?}", x);
    Ok(ws_stream)
}

pub async fn connect_user(
    auth_url: &str,
    user_url: &str,
    username: impl Into<String>,
    signer: impl Key + Clone,
) -> Result<WsClient> {
    let login = login(auth_url, username, signer.clone()).await?;
    let client = get_ws_user_client(
        user_url,
        &AuthorizeRequest {
            address: login.address,
            token: login.user_token,
            service: EnumService::User as _,
            device_id: "24787297130491616".to_string(),
            device_os: "android".to_string(),
        },
    )
    .await?;
    Ok(client)
}

pub async fn connect_user_ext(
    auth_url: &str,
    user_url: &str,
    username: impl Into<String>,
    signer: impl Key + Clone,
) -> Result<(WsClient, LoginResponse)> {
    let login = login(auth_url, username, signer).await?;
    let client = get_ws_user_client(
        user_url,
        &AuthorizeRequest {
            address: login.address.clone(),
            token: login.user_token.clone(),
            service: EnumService::User as _,
            device_id: "24787297130491616".to_string(),
            device_os: "android".to_string(),
        },
    )
    .await?;
    Ok((client, login))
}
