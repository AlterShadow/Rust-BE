use eyre::*;
use gen::model::*;
use lib::ws::WsClient;
use mc2fi_auth::{
    connect_user as old_connect_user, connect_user_ext as old_connect_user_ext,
    get_ws_auth_client as old_get_ws_auth_client, login as old_login, signup as old_signup,
};
use web3::signing::Key;
pub const WS_AUTH_URL: &str = "ws://localhost:8888";
pub const WS_USER_URL: &str = "ws://localhost:8889";
pub async fn signup(username: impl Into<String>, signer: impl Key + Clone) -> Result<()> {
    old_signup(WS_AUTH_URL, username, signer).await
}
pub async fn login(username: impl Into<String>, signer: impl Key + Clone) -> Result<LoginResponse> {
    old_login(WS_AUTH_URL, username, signer).await
}

pub async fn get_ws_auth_client(header: &str) -> Result<WsClient> {
    old_get_ws_auth_client(WS_AUTH_URL, header).await
}
pub async fn connect_user(
    username: impl Into<String>,
    signer: impl Key + Clone,
) -> Result<WsClient> {
    old_connect_user(WS_AUTH_URL, WS_USER_URL, username, signer).await
}
pub async fn connect_user_ext(
    username: impl Into<String>,
    signer: impl Key + Clone,
) -> Result<(WsClient, LoginResponse)> {
    old_connect_user_ext(WS_AUTH_URL, WS_USER_URL, username, signer).await
}
