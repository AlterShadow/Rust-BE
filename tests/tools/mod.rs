use eth_sdk::signer::Secp256k1SecretKey;
use eyre::*;
use gen::model::*;
use lib::ws::WsClient;
use mc2fi_auth::{
    connect_user as old_connect_user, connect_user_ext as old_connect_user_ext,
    get_ws_auth_client as old_get_ws_auth_client, get_ws_user_client as old_get_ws_user_client,
    login as old_login, signup as old_signup,
};
use tracing::*;
use web3::signing::Key;

const WS_AUTH_URL: &str = "ws://localhost:8888";
const WS_USER_URL: &str = "ws://localhost:8889";
pub async fn signup(username: impl Into<String>, signer: impl Key + Clone) -> Result<()> {
    old_signup(WS_AUTH_URL, username, signer).await
}
pub async fn login(username: impl Into<String>, signer: impl Key + Clone) -> Result<LoginResponse> {
    old_login(WS_AUTH_URL, username, signer).await
}

pub async fn connect_user(
    username: impl Into<String>,
    signer: impl Key + Clone,
) -> Result<WsClient> {
    old_connect_user(WS_AUTH_URL, WS_USER_URL, username, signer).await
}
pub async fn get_ws_user_client(auth: &AuthorizeRequest) -> Result<WsClient> {
    old_get_ws_user_client(WS_USER_URL, auth).await
}
pub async fn get_ws_auth_client(header: &str) -> Result<WsClient> {
    old_get_ws_auth_client(WS_AUTH_URL, header).await
}
pub async fn connect_user_ext(
    username: impl Into<String>,
    signer: impl Key + Clone,
) -> Result<(WsClient, LoginResponse)> {
    old_connect_user_ext(WS_AUTH_URL, WS_USER_URL, username, signer).await
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
