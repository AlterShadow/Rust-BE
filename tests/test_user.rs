pub mod tools;
use eyre::*;
use gen::client::UserClient;
use gen::model::*;
use lib::utils::encode_header;
use mc2_fi::endpoints::endpoint_auth_signup;
use tools::*;
use tracing::*;

async fn signup(username: impl Into<String>) -> Result<()> {
    let username = username.into();
    let mut client = get_ws_auth_client(&encode_header(
        SignupRequest {
            username: username.clone(),
            password: "AHJQ6X1H68SK8D9P6WW0".to_string(),
            email: format!("{}@mc2.fi", username),
            phone: "+00123456".to_string(),
            agreed_tos: true,
            agreed_privacy: true,
        },
        endpoint_auth_signup(),
    )?)
    .await?;
    let user1: SignupResponse = client.recv_resp().await?;
    info!("{:?}", user1);
    Ok(())
}
async fn login(username: impl Into<String>) -> Result<LoginResponse> {
    let login = auth_login(&LoginRequest {
        username: username.into(),
        password: "AHJQ6X1H68SK8D9P6WW0".to_string(),
        service_code: EnumService::User as _,
        device_id: "24787297130491616".to_string(),
        device_os: "android".to_string(),
    })
    .await?;

    Ok(login)
}
async fn connect_user(username: impl Into<String>) -> Result<UserClient> {
    let username = username.into();
    let login = login(username).await?;
    let client = get_ws_user_client(&AuthorizeRequest {
        username: login.username,
        token: login.user_token,
        service_code: EnumService::User as _,
        device_id: "24787297130491616".to_string(),
        device_os: "android".to_string(),
    })
    .await?;
    Ok(client)
}
#[tokio::test]
async fn test_invite_user_to_organization() -> Result<()> {
    drop_and_recreate_database()?;
    signup("jackhack").await?;
    signup("jackhack2").await?;

    let mut client = connect_user("jackhack").await?;

    let create_org: CreateOrganizationResponse = client
        .create_organization(&CreateOrganizationRequest {
            name: "org".to_string(),
            country: "xx".to_string(),
            tax_id: "xx".to_string(),
            address: "xx".to_string(),
            note: "xxx".to_string(),
        })
        .await?;
    let invitation: InviteUserToOrganizationResponse = client
        .invite_user_to_organization(&InviteUserToOrganizationRequest {
            organization_id: create_org.organization_id,
            email: "".to_string(),
            username: "jackhack2".to_string(),
        })
        .await?;
    info!("{:?}", invitation);
    let mut client2 = connect_user("jackhack2").await?;
    let resp = client2
        .list_organization_invitations_by_user(&ListOrganizationInvitationsByUserRequest {})
        .await?;
    info!("{:?}", resp);
    let accept_invitation: AcceptOrganizationInvitationResponse = client2
        .accept_organization_invitation(&AcceptOrganizationInvitationRequest {
            organization_id: create_org.organization_id,
        })
        .await?;
    info!("{:?}", accept_invitation);
    let resp: ListOrganizationInvitationsByOrganizationResponse = client
        .list_organization_invitations_by_organization(
            &ListOrganizationInvitationsByOrganizationRequest {
                organization_id: create_org.organization_id,
            },
        )
        .await?;
    info!("{:?}", resp);

    Ok(())
}

#[tokio::test]
async fn test_list_buckets() -> Result<()> {
    drop_and_recreate_database()?;
    signup("pepe_pablo").await?;

    let mut client = connect_user("pepe_pablo").await?;

    let create_org: CreateOrganizationResponse = client
        .create_organization(&CreateOrganizationRequest {
            name: "org".to_string(),
            country: "xx".to_string(),
            tax_id: "xx".to_string(),
            address: "xx".to_string(),
            note: "xxx".to_string(),
        })
        .await?;
    info!("create org {:?}", create_org);
    let create_bucket: CreateBucketResponse = client
        .create_bucket(&CreateBucketRequest {
            bucket_name: "BKT".to_string(),
            organization_id: create_org.organization_id,
            description: "Bucket".to_string(),
            always_online: true,
        })
        .await?;
    info!("create bucket {:?}", create_bucket);
    let list_buckets = client
        .list_buckets_by_user(&ListBucketsByUserRequest {})
        .await?;
    info!("list buckets {:?}", list_buckets);
    Ok(())
}
