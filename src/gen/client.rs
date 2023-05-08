use crate::model::*;
use eyre::*;
use lib::ws::WsClient;

pub struct AuthClient {
    pub client: WsClient,
}
impl AuthClient {
    pub fn new(client: WsClient) -> Self {
        Self { client }
    }
}
impl From<WsClient> for AuthClient {
    fn from(client: WsClient) -> Self {
        Self::new(client)
    }
}

impl AuthClient {
    pub async fn login(&mut self, req: &LoginRequest) -> Result<LoginResponse> {
        self.client.request(10020, req).await
    }
}
impl AuthClient {
    pub async fn signup(&mut self, req: &SignupRequest) -> Result<SignupResponse> {
        self.client.request(10010, req).await
    }
}
impl AuthClient {
    pub async fn authorize(&mut self, req: &AuthorizeRequest) -> Result<AuthorizeResponse> {
        self.client.request(10030, req).await
    }
}
pub struct UserClient {
    pub client: WsClient,
}
impl UserClient {
    pub fn new(client: WsClient) -> Self {
        Self { client }
    }
}
impl From<WsClient> for UserClient {
    fn from(client: WsClient) -> Self {
        Self::new(client)
    }
}

impl UserClient {
    pub async fn transfer_organization_owner(
        &mut self,
        req: &TransferOrganizationOwnerRequest,
    ) -> Result<TransferOrganizationOwnerResponse> {
        self.client.request(20040, req).await
    }
}
impl UserClient {
    pub async fn list_organization_membership(
        &mut self,
        req: &ListOrganizationMembershipRequest,
    ) -> Result<ListOrganizationMembershipResponse> {
        self.client.request(20042, req).await
    }
}
pub struct AdminClient {
    pub client: WsClient,
}
impl AdminClient {
    pub fn new(client: WsClient) -> Self {
        Self { client }
    }
}
impl From<WsClient> for AdminClient {
    fn from(client: WsClient) -> Self {
        Self::new(client)
    }
}
