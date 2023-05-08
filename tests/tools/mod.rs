use eyre::*;
use gen::client::*;
use gen::model::{AuthorizeRequest, AuthorizeResponse, LoginRequest, LoginResponse};
use lib::utils::encode_header;
use lib::ws::WsClient;
use mc2_fi::endpoints::{endpoint_auth_authorize, endpoint_auth_login};
use std::path::Path;
use std::process::Command;
use tracing::*;

pub async fn get_ws_auth_client(header: &str) -> Result<WsClient> {
    let connect_addr = "ws://localhost:8888";
    info!("Connecting to {} with header {}", connect_addr, header);
    let ws_stream = WsClient::new(connect_addr, header).await?;
    Ok(ws_stream.into())
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
    let working_dir = Path::new("scripts").canonicalize()?;
    let script = working_dir.join("drop_and_recreate_database.sh");
    Command::new("bash")
        .arg(script)
        .current_dir(working_dir)
        .status()?;
    Ok(())
}
