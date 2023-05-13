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
    pub async fn user_follow_strategy(
        &mut self,
        req: &UserFollowStrategyRequest,
    ) -> Result<UserFollowStrategyResponse> {
        self.client.request(20040, req).await
    }
}
impl UserClient {
    pub async fn user_list_followed_strategies(
        &mut self,
        req: &UserListFollowedStrategiesRequest,
    ) -> Result<UserListFollowedStrategiesResponse> {
        self.client.request(20050, req).await
    }
}
impl UserClient {
    pub async fn user_unfollow_strategy(
        &mut self,
        req: &UserUnfollowStrategyRequest,
    ) -> Result<UserUnfollowStrategyResponse> {
        self.client.request(20060, req).await
    }
}
impl UserClient {
    pub async fn user_list_strategies(
        &mut self,
        req: &UserListStrategiesRequest,
    ) -> Result<UserListStrategiesResponse> {
        self.client.request(20051, req).await
    }
}
impl UserClient {
    pub async fn user_get_strategy(
        &mut self,
        req: &UserGetStrategyRequest,
    ) -> Result<UserGetStrategyResponse> {
        self.client.request(20061, req).await
    }
}
impl UserClient {
    pub async fn user_get_strategy_statistics(
        &mut self,
        req: &UserGetStrategyStatisticsRequest,
    ) -> Result<UserGetStrategyStatisticsResponse> {
        self.client.request(20070, req).await
    }
}
impl UserClient {
    pub async fn user_back_strategy(
        &mut self,
        req: &UserBackStrategyRequest,
    ) -> Result<UserBackStrategyResponse> {
        self.client.request(20080, req).await
    }
}
impl UserClient {
    pub async fn user_list_backed_strategy(
        &mut self,
        req: &UserListBackedStrategyRequest,
    ) -> Result<UserListBackedStrategyResponse> {
        self.client.request(20090, req).await
    }
}
impl UserClient {
    pub async fn user_list_back_strategy_history(
        &mut self,
        req: &UserListBackStrategyHistoryRequest,
    ) -> Result<UserListBackStrategyHistoryResponse> {
        self.client.request(20100, req).await
    }
}
impl UserClient {
    pub async fn user_exit_strategy(
        &mut self,
        req: &UserExitStrategyRequest,
    ) -> Result<UserExitStrategyResponse> {
        self.client.request(20110, req).await
    }
}
impl UserClient {
    pub async fn user_list_exit_strategy_history(
        &mut self,
        req: &UserListExitStrategyHistoryRequest,
    ) -> Result<UserListExitStrategyHistoryResponse> {
        self.client.request(20120, req).await
    }
}
impl UserClient {
    pub async fn user_follow_expert(
        &mut self,
        req: &UserFollowExpertRequest,
    ) -> Result<UserFollowExpertResponse> {
        self.client.request(20130, req).await
    }
}
impl UserClient {
    pub async fn user_list_followed_expert(
        &mut self,
        req: &UserListFollowedExpertRequest,
    ) -> Result<UserListFollowedExpertResponse> {
        self.client.request(20140, req).await
    }
}
impl UserClient {
    pub async fn user_unfollow_expert(
        &mut self,
        req: &UserUnfollowExpertRequest,
    ) -> Result<UserUnfollowExpertResponse> {
        self.client.request(20150, req).await
    }
}
impl UserClient {
    pub async fn user_list_experts(
        &mut self,
        req: &UserListExpertsRequest,
    ) -> Result<UserListExpertsResponse> {
        self.client.request(20160, req).await
    }
}
impl UserClient {
    pub async fn user_get_expert_profile(
        &mut self,
        req: &UserGetExpertProfileRequest,
    ) -> Result<UserGetExpertProfileResponse> {
        self.client.request(20170, req).await
    }
}
impl UserClient {
    pub async fn user_get_user_profile(
        &mut self,
        req: &UserGetUserProfileRequest,
    ) -> Result<UserGetUserProfileResponse> {
        self.client.request(20180, req).await
    }
}
impl UserClient {
    pub async fn user_register_wallet(
        &mut self,
        req: &UserRegisterWalletRequest,
    ) -> Result<UserRegisterWalletResponse> {
        self.client.request(20190, req).await
    }
}
impl UserClient {
    pub async fn user_list_wallets(
        &mut self,
        req: &UserListWalletsRequest,
    ) -> Result<UserListWalletsResponse> {
        self.client.request(20200, req).await
    }
}
impl UserClient {
    pub async fn user_deregister_wallet(
        &mut self,
        req: &UserDeregisterWalletRequest,
    ) -> Result<UserDeregisterWalletResponse> {
        self.client.request(20210, req).await
    }
}
impl UserClient {
    pub async fn user_apply_become_expert(
        &mut self,
        req: &UserApplyBecomeExpertRequest,
    ) -> Result<UserApplyBecomeExpertResponse> {
        self.client.request(20220, req).await
    }
}
impl UserClient {
    pub async fn admin_approve_user_become_expert(
        &mut self,
        req: &AdminApproveUserBecomeExpertRequest,
    ) -> Result<AdminApproveUserBecomeExpertResponse> {
        self.client.request(20230, req).await
    }
}
impl UserClient {
    pub async fn admin_list_pending_user_become_expert(
        &mut self,
        req: &AdminListPendingUserBecomeExpertRequest,
    ) -> Result<AdminListPendingUserBecomeExpertResponse> {
        self.client.request(20240, req).await
    }
}
impl UserClient {
    pub async fn user_make_strategy(
        &mut self,
        req: &UserMakeStrategyRequest,
    ) -> Result<UserMakeStrategyResponse> {
        self.client.request(20250, req).await
    }
}
impl UserClient {
    pub async fn user_update_strategy(
        &mut self,
        req: &UserUpdateStrategyRequest,
    ) -> Result<UserUpdateStrategyResponse> {
        self.client.request(20260, req).await
    }
}
impl UserClient {
    pub async fn user_add_strategy_watching_wallet(
        &mut self,
        req: &UserAddStrategyWatchingWalletRequest,
    ) -> Result<UserAddStrategyWatchingWalletResponse> {
        self.client.request(20270, req).await
    }
}
impl UserClient {
    pub async fn user_remove_strategy_watching_wallet(
        &mut self,
        req: &UserRemoveStrategyWatchingWalletRequest,
    ) -> Result<UserRemoveStrategyWatchingWalletResponse> {
        self.client.request(20280, req).await
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
