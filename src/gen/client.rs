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
    pub async fn login(&mut self, req: LoginRequest) -> Result<LoginResponse> {
        self.client.request(10020, req).await
    }
}
impl AuthClient {
    pub async fn signup(&mut self, req: SignupRequest) -> Result<SignupResponse> {
        self.client.request(10010, req).await
    }
}
impl AuthClient {
    pub async fn authorize(&mut self, req: AuthorizeRequest) -> Result<AuthorizeResponse> {
        self.client.request(10030, req).await
    }
}
impl AuthClient {
    pub async fn logout(&mut self, req: LogoutRequest) -> Result<LogoutResponse> {
        self.client.request(10040, req).await
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
        req: UserFollowStrategyRequest,
    ) -> Result<UserFollowStrategyResponse> {
        self.client.request(20040, req).await
    }
}
impl UserClient {
    pub async fn user_list_followed_strategies(
        &mut self,
        req: UserListFollowedStrategiesRequest,
    ) -> Result<UserListFollowedStrategiesResponse> {
        self.client.request(20050, req).await
    }
}
impl UserClient {
    pub async fn user_unfollow_strategy(
        &mut self,
        req: UserUnfollowStrategyRequest,
    ) -> Result<UserUnfollowStrategyResponse> {
        self.client.request(20060, req).await
    }
}
impl UserClient {
    pub async fn user_list_strategies(
        &mut self,
        req: UserListStrategiesRequest,
    ) -> Result<UserListStrategiesResponse> {
        self.client.request(20061, req).await
    }
}
impl UserClient {
    pub async fn user_get_strategy(
        &mut self,
        req: UserGetStrategyRequest,
    ) -> Result<UserGetStrategyResponse> {
        self.client.request(20062, req).await
    }
}
impl UserClient {
    pub async fn user_get_strategy_statistics(
        &mut self,
        req: UserGetStrategyStatisticsRequest,
    ) -> Result<UserGetStrategyStatisticsResponse> {
        self.client.request(20070, req).await
    }
}
impl UserClient {
    pub async fn user_get_strategies_statistics(
        &mut self,
        req: UserGetStrategiesStatisticsRequest,
    ) -> Result<UserGetStrategiesStatisticsResponse> {
        self.client.request(20071, req).await
    }
}
impl UserClient {
    pub async fn user_back_strategy(
        &mut self,
        req: UserBackStrategyRequest,
    ) -> Result<UserBackStrategyResponse> {
        self.client.request(20080, req).await
    }
}
impl UserClient {
    pub async fn user_request_refund(
        &mut self,
        req: UserRequestRefundRequest,
    ) -> Result<UserRequestRefundResponse> {
        self.client.request(20081, req).await
    }
}
impl UserClient {
    pub async fn user_list_backed_strategies(
        &mut self,
        req: UserListBackedStrategiesRequest,
    ) -> Result<UserListBackedStrategiesResponse> {
        self.client.request(20090, req).await
    }
}
impl UserClient {
    pub async fn user_list_back_strategy_history(
        &mut self,
        req: UserListBackStrategyHistoryRequest,
    ) -> Result<UserListBackStrategyHistoryResponse> {
        self.client.request(20100, req).await
    }
}
impl UserClient {
    pub async fn user_list_exit_strategy_history(
        &mut self,
        req: UserListExitStrategyHistoryRequest,
    ) -> Result<UserListExitStrategyHistoryResponse> {
        self.client.request(20120, req).await
    }
}
impl UserClient {
    pub async fn user_follow_expert(
        &mut self,
        req: UserFollowExpertRequest,
    ) -> Result<UserFollowExpertResponse> {
        self.client.request(20130, req).await
    }
}
impl UserClient {
    pub async fn user_list_followed_experts(
        &mut self,
        req: UserListFollowedExpertsRequest,
    ) -> Result<UserListFollowedExpertsResponse> {
        self.client.request(20140, req).await
    }
}
impl UserClient {
    pub async fn user_unfollow_expert(
        &mut self,
        req: UserUnfollowExpertRequest,
    ) -> Result<UserUnfollowExpertResponse> {
        self.client.request(20150, req).await
    }
}
impl UserClient {
    pub async fn user_list_experts(
        &mut self,
        req: UserListExpertsRequest,
    ) -> Result<UserListExpertsResponse> {
        self.client.request(20160, req).await
    }
}
impl UserClient {
    pub async fn user_list_top_performing_experts(
        &mut self,
        req: UserListTopPerformingExpertsRequest,
    ) -> Result<UserListTopPerformingExpertsResponse> {
        self.client.request(20161, req).await
    }
}
impl UserClient {
    pub async fn user_list_featured_experts(
        &mut self,
        req: UserListFeaturedExpertsRequest,
    ) -> Result<UserListFeaturedExpertsResponse> {
        self.client.request(20162, req).await
    }
}
impl UserClient {
    pub async fn user_get_expert_profile(
        &mut self,
        req: UserGetExpertProfileRequest,
    ) -> Result<UserGetExpertProfileResponse> {
        self.client.request(20170, req).await
    }
}
impl UserClient {
    pub async fn user_get_user_profile(
        &mut self,
        req: UserGetUserProfileRequest,
    ) -> Result<UserGetUserProfileResponse> {
        self.client.request(20180, req).await
    }
}
impl UserClient {
    pub async fn user_register_wallet(
        &mut self,
        req: UserRegisterWalletRequest,
    ) -> Result<UserRegisterWalletResponse> {
        self.client.request(20190, req).await
    }
}
impl UserClient {
    pub async fn user_list_wallets(
        &mut self,
        req: UserListWalletsRequest,
    ) -> Result<UserListWalletsResponse> {
        self.client.request(20200, req).await
    }
}
impl UserClient {
    pub async fn user_deregister_wallet(
        &mut self,
        req: UserDeregisterWalletRequest,
    ) -> Result<UserDeregisterWalletResponse> {
        self.client.request(20210, req).await
    }
}
impl UserClient {
    pub async fn user_apply_become_expert(
        &mut self,
        req: UserApplyBecomeExpertRequest,
    ) -> Result<UserApplyBecomeExpertResponse> {
        self.client.request(20220, req).await
    }
}
impl UserClient {
    pub async fn user_create_strategy(
        &mut self,
        req: UserCreateStrategyRequest,
    ) -> Result<UserCreateStrategyResponse> {
        self.client.request(20250, req).await
    }
}
impl UserClient {
    pub async fn user_update_strategy(
        &mut self,
        req: UserUpdateStrategyRequest,
    ) -> Result<UserUpdateStrategyResponse> {
        self.client.request(20260, req).await
    }
}
impl UserClient {
    pub async fn user_add_strategy_watching_wallet(
        &mut self,
        req: UserAddStrategyWatchingWalletRequest,
    ) -> Result<UserAddStrategyWatchingWalletResponse> {
        self.client.request(20270, req).await
    }
}
impl UserClient {
    pub async fn user_remove_strategy_watching_wallet(
        &mut self,
        req: UserRemoveStrategyWatchingWalletRequest,
    ) -> Result<UserRemoveStrategyWatchingWalletResponse> {
        self.client.request(20280, req).await
    }
}
impl UserClient {
    pub async fn user_list_strategy_watching_wallets(
        &mut self,
        req: UserListStrategyWatchingWalletsRequest,
    ) -> Result<UserListStrategyWatchingWalletsResponse> {
        self.client.request(20290, req).await
    }
}
impl UserClient {
    pub async fn user_list_wallet_activity_history(
        &mut self,
        req: UserListWalletActivityHistoryRequest,
    ) -> Result<UserListWalletActivityHistoryResponse> {
        self.client.request(20300, req).await
    }
}
impl UserClient {
    pub async fn user_add_strategy_initial_token_ratio(
        &mut self,
        req: UserAddStrategyInitialTokenRatioRequest,
    ) -> Result<UserAddStrategyInitialTokenRatioResponse> {
        self.client.request(20310, req).await
    }
}
impl UserClient {
    pub async fn user_remove_strategy_initial_token_ratio(
        &mut self,
        req: UserRemoveStrategyInitialTokenRatioRequest,
    ) -> Result<UserRemoveStrategyInitialTokenRatioResponse> {
        self.client.request(20320, req).await
    }
}
impl UserClient {
    pub async fn user_list_strategy_initial_token_ratio(
        &mut self,
        req: UserListStrategyInitialTokenRatioRequest,
    ) -> Result<UserListStrategyInitialTokenRatioResponse> {
        self.client.request(20330, req).await
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

impl AdminClient {
    pub async fn admin_list_users(
        &mut self,
        req: AdminListUsersRequest,
    ) -> Result<AdminListUsersResponse> {
        self.client.request(30010, req).await
    }
}
impl AdminClient {
    pub async fn admin_set_user_role(
        &mut self,
        req: AdminSetUserRoleRequest,
    ) -> Result<AdminSetUserRoleResponse> {
        self.client.request(30020, req).await
    }
}
impl AdminClient {
    pub async fn admin_set_block_user(
        &mut self,
        req: AdminSetBlockUserRequest,
    ) -> Result<AdminSetBlockUserResponse> {
        self.client.request(30030, req).await
    }
}
impl AdminClient {
    pub async fn admin_list_pending_expert_applications(
        &mut self,
        req: AdminListPendingExpertApplicationsRequest,
    ) -> Result<AdminListPendingExpertApplicationsResponse> {
        self.client.request(20240, req).await
    }
}
impl AdminClient {
    pub async fn admin_approve_user_become_expert(
        &mut self,
        req: AdminApproveUserBecomeExpertRequest,
    ) -> Result<AdminApproveUserBecomeExpertResponse> {
        self.client.request(20230, req).await
    }
}
impl AdminClient {
    pub async fn admin_reject_user_become_expert(
        &mut self,
        req: AdminRejectUserBecomeExpertRequest,
    ) -> Result<AdminRejectUserBecomeExpertResponse> {
        self.client.request(20231, req).await
    }
}
pub struct WatcherClient {
    pub client: WsClient,
}
impl WatcherClient {
    pub fn new(client: WsClient) -> Self {
        Self { client }
    }
}
impl From<WsClient> for WatcherClient {
    fn from(client: WsClient) -> Self {
        Self::new(client)
    }
}
