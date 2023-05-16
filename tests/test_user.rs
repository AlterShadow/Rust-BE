pub mod tools;
use eyre::*;
use gen::client::UserClient;
use gen::model::*;
use lib::log::{setup_logs, LogLevel};
use lib::utils::encode_header;
use lib::ws::WsClient;
use mc2_fi::endpoints::{endpoint_auth_login, endpoint_auth_signup};
use tools::*;
use tracing::*;

async fn signup(username: impl Into<String>) -> Result<()> {
    assert_eq!(username.into(), "user1");
    let mut client = get_ws_auth_client(&encode_header(
        SignupRequest {
            address: "0x111013b7862ebc1b9726420aa0e8728de310ee63".to_string(),
            signature_text: "5468697320726571756573742077696c6c206e6f74207472696767657220616e79207472616e73616374696f6e206f7220696e63757220616e7920636f7374206f7220666565732e200a204974206973206f6e6c7920696e74656e64656420746f2061757468656e74696361746520796f752061726520746865206f776e6572206f662077616c6c65743a0a3078313131303133623738363265626331623937323634323061613065383732386465333130656536336e6f6e63653a0a383632353033343139".to_string(),
            signature: "72f8e93e5e2ba1b3df2f179bddac22b691ca86b39f6f7619a9eedd90b16bed165c0e03dcac13e5e2a1a1ea79ab9cf40a6ba572165a7f58525466a42a9699f0ea1c".to_string(),
            email: "qjk2001@gmail.com".to_string(),
            phone: "+00123456".to_string(),
            agreed_tos: true,
            agreed_privacy: true,
        },
        endpoint_auth_signup(),
    )?)
    .await?;
    let res: SignupResponse = client.recv_resp().await?;
    info!("{:?}", res);
    Ok(())
}
async fn login(username: impl Into<String>) -> Result<LoginResponse> {
    assert_eq!(username.into(), "user1");

    let mut client = get_ws_auth_client(&encode_header(
        LoginRequest {
            address: "0x111013b7862ebc1b9726420aa0e8728de310ee63".to_string(),
            signature_text: "5468697320726571756573742077696c6c206e6f74207472696767657220616e79207472616e73616374696f6e206f7220696e63757220616e7920636f7374206f7220666565732e200a204974206973206f6e6c7920696e74656e64656420746f2061757468656e74696361746520796f752061726520746865206f776e6572206f662077616c6c65743a0a3078313131303133623738363265626331623937323634323061613065383732386465333130656536336e6f6e63653a0a383632353033343139".to_string(),
            signature: "72f8e93e5e2ba1b3df2f179bddac22b691ca86b39f6f7619a9eedd90b16bed165c0e03dcac13e5e2a1a1ea79ab9cf40a6ba572165a7f58525466a42a9699f0ea1c".to_string(),
            service_code: EnumService::User as _,
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
async fn connect_user(username: impl Into<String>) -> Result<UserClient> {
    let login = login(username).await?;
    let client = get_ws_user_client(&AuthorizeRequest {
        address: login.address,
        token: login.user_token,
        service_code: EnumService::User as _,
        device_id: "24787297130491616".to_string(),
        device_os: "android".to_string(),
    })
    .await?;
    Ok(client)
}

#[tokio::test]
async fn test_register_wallet() -> Result<()> {
    let _ = setup_logs(LogLevel::Info);

    drop_and_recreate_database()?;
    signup("user1").await?;

    let mut client = connect_user("user1").await?;

    let resp = client
        .user_register_wallet(UserRegisterWalletRequest {
            blockchain: "ethereum".to_string(),
            wallet_address: "0x111013b7862ebc1b9726420aa0e8728de310ee63".to_string(),
            message_to_sign: "5468697320726571756573742077696c6c206e6f74207472696767657220616e79207472616e73616374696f6e206f7220696e63757220616e7920636f7374206f7220666565732e200a204974206973206f6e6c7920696e74656e64656420746f2061757468656e74696361746520796f752061726520746865206f776e6572206f662077616c6c65743a0a3078313131303133623738363265626331623937323634323061613065383732386465333130656536336e6f6e63653a0a383632353033343139".to_string(),
            message_signature: "72f8e93e5e2ba1b3df2f179bddac22b691ca86b39f6f7619a9eedd90b16bed165c0e03dcac13e5e2a1a1ea79ab9cf40a6ba572165a7f58525466a42a9699f0ea1c".to_string(),
        })
        .await?;
    info!("Register wallet {:?}", resp);
    client.user_deregister_wallet(UserDeregisterWalletRequest {
        wallet_id: resp.wallet_id,
    });
    Ok(())
}
#[tokio::test]
async fn test_create_update_strategy() -> Result<()> {
    let _ = setup_logs(LogLevel::Info);
    drop_and_recreate_database()?;
    signup("user1").await?;

    let mut client = connect_user("user1").await?;

    let resp = client
        .user_create_strategy(UserCreateStrategyRequest {
            name: "test_strategy".to_string(),
            description: "this is a test strategy".to_string(),
        })
        .await?;
    info!("Register wallet {:?}", resp);
    client
        .user_update_strategy(UserUpdateStrategyRequest {
            strategy_id: resp.strategy_id,
            name: None,
            description: None,
            social_media: None,
            risk_score: None,
            reputation_score: None,
            aum: None,
        })
        .await?;
    let wallet = client
        .user_add_strategy_watching_wallet(UserAddStrategyWatchingWalletRequest {
            strategy_id: resp.strategy_id,
            blockchain: "ethereum".to_string(),
            wallet_address: "0x000000000001".to_string(),
            ratio: 1.0,
        })
        .await?;
    info!("Add wallet {:?}", wallet);
    let remove_wallet = client
        .user_remove_strategy_watching_wallet(UserRemoveStrategyWatchingWalletRequest {
            wallet_id: wallet.wallet_id,
        })
        .await?;
    info!("Remove wallet {:?}", remove_wallet);
    Ok(())
}
