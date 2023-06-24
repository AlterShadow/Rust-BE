use eth_sdk::signer::Secp256k1SecretKey;
use eth_sdk::utils::get_signed_text;
use eyre::*;
use gen::model::*;
use lib::utils::encode_header;
use lib::ws::WsClient;
use mc2fi_auth::endpoints::{endpoint_auth_authorize, endpoint_auth_login, endpoint_auth_signup};
use tracing::*;
use web3::signing::Key;

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

pub async fn get_ws_user_client(req: &AuthorizeRequest) -> Result<WsClient> {
    let header = &encode_header(req, endpoint_auth_authorize())?;
    let connect_addr = "ws://localhost:8889";
    info!("Connecting to {} with header {}", connect_addr, header);
    let mut ws_stream = WsClient::new(connect_addr, header).await?;
    let x: AuthorizeResponse = ws_stream.recv_resp().await?;
    info!("AuthorizeResponse {:?}", x);
    Ok(ws_stream)
}

pub async fn signup(username: impl Into<String>, signer: impl Key + Clone) -> Result<()> {
    let username = username.into();
    let (txt, sig) = get_signed_text(format!("Signup {}", username), signer.clone())?;

    let mut client = get_ws_auth_client(&encode_header(
        SignupRequest {
            address: format!("{:?}", signer.address()),
            signature_text: txt,
            signature: sig,
            email: "qjk2001@gmail.com".to_string(),
            phone: "+00123456".to_string(),
            agreed_tos: true,
            agreed_privacy: true,
            username,
        },
        endpoint_auth_signup(),
    )?)
    .await?;
    let res: SignupResponse = client.recv_resp().await?;
    info!("{:?}", res);
    Ok(())
}
pub async fn login(username: impl Into<String>, signer: impl Key + Clone) -> Result<LoginResponse> {
    let username = username.into();

    let (txt, sig) = get_signed_text(format!("Login {}", username), signer.clone())?;
    let mut client = get_ws_auth_client(&encode_header(
        LoginRequest {
            address: format!("{:?}", signer.address()),
            signature_text: txt,
            signature: sig,
            service: EnumService::User as _,
            device_id: "24787297130491616".to_string(),
            device_os: "android".to_string(),
        },
        endpoint_auth_login(),
    )?)
    .await?;
    let res: LoginResponse = client.recv_resp().await?;
    info!("{:?}", res);
    Ok(res)
}
pub async fn connect_user(
    username: impl Into<String>,
    signer: impl Key + Clone,
) -> Result<WsClient> {
    let login = login(username, signer.clone()).await?;
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
pub async fn connect_user_ext(
    username: impl Into<String>,
    signer: impl Key + Clone,
) -> Result<(WsClient, LoginResponse)> {
    let login = login(username, signer).await?;
    let client = get_ws_user_client(&AuthorizeRequest {
        address: login.address.clone(),
        token: login.user_token.clone(),
        service: EnumService::User as _,
        device_id: "24787297130491616".to_string(),
        device_os: "android".to_string(),
    })
    .await?;
    Ok((client, login))
}

pub async fn prepare_expert() -> Result<(Secp256k1SecretKey, WsClient, Secp256k1SecretKey, WsClient)>
{
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
    // reconnect user to refresh role cache on server side
    let client = connect_user("user1", &user.key).await?;

    Ok((admin, admin_client, user, client))
}
