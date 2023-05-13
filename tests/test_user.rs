pub mod tools;
use eyre::*;
use gen::client::UserClient;
use gen::model::*;
use lib::utils::encode_header;
use mc2_fi::endpoints::endpoint_auth_signup;
use tools::*;
use tracing::*;

async fn signup(username: impl Into<String>) -> Result<()> {
    assert_eq!(username, "user1");
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
    todo!()
}
async fn connect_user(username: impl Into<String>) -> Result<UserClient> {
    todo!()
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
