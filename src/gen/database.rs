use crate::model::*;
use lib::database::*;
use postgres_from_row::FromRow;
use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAdminAddAuditRuleRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAdminApproveStrategyRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAdminApproveUserBecomeExpertRespRow {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAdminGetSystemConfigRespRow {
    #[serde(default)]
    pub config_placeholder_1: Option<i64>,
    #[serde(default)]
    pub config_placeholder_2: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAdminListBackersRespRow {
    pub user_id: i64,
    pub user_public_id: i64,
    pub username: String,
    pub login_wallet_address: String,
    pub joined_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAdminListPendingUserExpertApplicationsRespRow {
    pub user_public_id: i64,
    pub name: String,
    pub linked_wallet: String,
    pub follower_count: i64,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub social_media: Option<String>,
    #[serde(default)]
    pub risk_score: Option<f64>,
    #[serde(default)]
    pub reputation_score: Option<f64>,
    #[serde(default)]
    pub aum: Option<f64>,
    pub pending_expert: bool,
    pub approved_expert: bool,
    #[serde(default)]
    pub joined_at: Option<i64>,
    #[serde(default)]
    pub requested_at: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAdminListUsersRespRow {
    pub total: i64,
    pub user_id: i64,
    pub public_user_id: i64,
    #[serde(default)]
    pub username: Option<String>,
    pub address: String,
    pub last_ip: std::net::IpAddr,
    pub last_login_at: i64,
    pub login_count: i32,
    pub role: EnumRole,
    #[serde(default)]
    pub email: Option<String>,
    pub updated_at: i64,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAdminRejectStrategyRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAdminRejectUserBecomeExpertRespRow {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAdminSetBlockUserRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAdminSetUserRoleRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAdminUpdateSystemConfigRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAuthAuthenticateRespRow {
    pub user_id: i64,
    pub public_user_id: i64,
    pub role: EnumRole,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAuthAuthorizeRespRow {
    pub user_id: i64,
    pub role: EnumRole,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAuthChangeLoginWalletAddressRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAuthRemoveTokenRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAuthSetRoleRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAuthSetTokenRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAuthSignupRespRow {
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAuthUpdateUserTableRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunExpertListBackersRespRow {
    pub public_id: i64,
    pub username: String,
    #[serde(default)]
    pub family_name: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
    pub backed_at: i64,
    pub joined_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunExpertListFollowersRespRow {
    pub public_id: i64,
    pub username: String,
    #[serde(default)]
    pub family_name: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
    pub followed_at: i64,
    pub joined_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserAddRegisteredWalletRespRow {
    pub registered_wallet_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserAddStrategyAuditRuleRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserAddStrategyInitialTokenRatioRespRow {
    pub strategy_initial_token_ratio_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserAddStrategyWalletRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserAddStrategyWatchWalletRespRow {
    pub success: bool,
    pub watch_wallet_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserAddStrategyWhitelistedTokenRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserApplyBecomeExpertRespRow {
    pub success: bool,
    pub expert_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserBackStrategyRespRow {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserCheckIfTokenWhitelistedRespRow {
    pub whitelisted: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserCreateExpertProfileRespRow {
    pub expert_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserCreateStrategyRespRow {
    pub success: bool,
    pub strategy_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserDelStrategyAuditRuleRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserDepositToEscrowRespRow {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserExitStrategyRespRow {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserExpertRowType {
    pub total: i64,
    pub expert_id: i64,
    pub user_id: i64,
    pub user_public_id: i64,
    pub listening_wallet: String,
    pub username: String,
    #[serde(default)]
    pub family_name: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
    pub follower_count: i64,
    pub backer_count: i64,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub social_media: Option<String>,
    #[serde(default)]
    pub risk_score: Option<f64>,
    #[serde(default)]
    pub reputation_score: Option<f64>,
    #[serde(default)]
    pub aum: Option<f64>,
    pub joined_at: i64,
    #[serde(default)]
    pub requested_at: Option<i64>,
    #[serde(default)]
    pub approved_at: Option<i64>,
    pub pending_expert: bool,
    pub approved_expert: bool,
    pub followed: bool,
    pub linked_wallet: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserFollowExpertRespRow {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserFollowStrategyRespRow {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserGetStrategyInitialTokenRatioByAddressAndChainRespRow {
    pub strategy_initial_token_ratio_id: i64,
    pub blockchain: EnumBlockChain,
    pub token_name: String,
    pub token_address: String,
    pub quantity: String,
    pub strategy_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserGetStrategyStatisticsBackHistoryRespRow {
    pub time: i64,
    pub backer_count: f64,
    pub backer_quantity_usd: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserGetStrategyStatisticsFollowHistoryRespRow {
    pub time: i64,
    pub follower_count: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserGetStrategyStatisticsNetValueRespRow {
    pub time: i64,
    pub net_value: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserGetUserByAddressRespRow {
    pub user_id: i64,
    pub user_public_id: i64,
    pub username: String,
    #[serde(default)]
    pub family_name: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
    pub joined_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserGetUserProfileRespRow {
    #[serde(default)]
    pub expert_id: Option<i64>,
    pub user_public_id: i64,
    pub name: String,
    pub login_wallet: String,
    pub joined_at: i64,
    #[serde(default)]
    pub follower_count: Option<i64>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub social_media: Option<String>,
    #[serde(default)]
    pub risk_score: Option<f64>,
    #[serde(default)]
    pub reputation_score: Option<f64>,
    #[serde(default)]
    pub aum: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListAuditRulesRespRow {
    pub rule_id: i64,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListBackStrategyHistoryRespRow {
    pub back_history_id: i64,
    pub strategy_id: i64,
    pub quantity: String,
    pub blockchain: EnumBlockChain,
    pub transaction_hash: String,
    pub time: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListDepositHistoryRespRow {
    pub blockchain: EnumBlockChain,
    pub user_address: String,
    pub contract_address: String,
    pub receiver_address: String,
    pub quantity: String,
    pub transaction_hash: String,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListExitStrategyHistoryRespRow {
    pub exit_history_id: i64,
    pub strategy_id: i64,
    pub exit_quantity: String,
    pub blockchain: EnumBlockChain,
    pub exit_time: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListRegisteredWalletsRespRow {
    pub registered_wallet_id: i64,
    pub blockchain: EnumBlockChain,
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListRequestRefundHistoryRespRow {
    pub request_refund_id: i64,
    pub user_id: i64,
    pub blockchain: EnumBlockChain,
    pub quantity: String,
    pub wallet_address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListStrategyAuditRulesRespRow {
    pub rule_id: i64,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListStrategyBackersRespRow {
    pub user_id: i64,
    pub user_public_id: i64,
    pub username: String,
    pub wallet_address: String,
    pub backed_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListStrategyFollowersRespRow {
    pub user_id: i64,
    pub user_public_id: i64,
    pub username: String,
    pub wallet_address: String,
    pub followed_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListStrategyInitialTokenRatiosRespRow {
    pub strategy_initial_token_ratio_id: i64,
    pub blockchain: EnumBlockChain,
    pub token_name: String,
    pub token_address: String,
    pub quantity: String,
    pub strategy_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListStrategyWalletsRespRow {
    pub blockchain: EnumBlockChain,
    pub address: String,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListStrategyWatchWalletsRespRow {
    pub watch_wallet_id: i64,
    pub wallet_address: String,
    pub blockchain: EnumBlockChain,
    pub ratio: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListStrategyWhitelistedTokensRespRow {
    pub token_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListTopPerformingStrategiesRespRow {
    pub strategy_id: i64,
    pub strategy_name: String,
    pub strategy_description: String,
    pub net_value: f64,
    pub followers: i64,
    pub backers: i64,
    #[serde(default)]
    pub risk_score: Option<f64>,
    #[serde(default)]
    pub aum: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserRemoveRegisteredWalletRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserRemoveStrategyInitialTokenRatioRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserRemoveStrategyWatchWalletRespRow {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserRequestRefundRespRow {
    pub request_refund_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserStrategyRowType {
    pub total: i64,
    pub strategy_id: i64,
    pub strategy_name: String,
    pub strategy_description: String,
    pub current_usdc: String,
    pub total_backed_usdc: String,
    pub total_exited_usdc: String,
    #[serde(default)]
    pub risk_score: Option<f64>,
    #[serde(default)]
    pub aum: Option<f64>,
    pub followers: i64,
    pub backers: i64,
    pub followed: bool,
    #[serde(default)]
    pub requested_at: Option<i64>,
    pub approved: bool,
    #[serde(default)]
    pub approved_at: Option<i64>,
    pub pending_approval: bool,
    #[serde(default)]
    pub linked_wallet: Option<String>,
    #[serde(default)]
    pub linked_wallet_blockchain: Option<EnumBlockChain>,
    pub created_at: i64,
    pub creator_public_id: i64,
    pub creator_id: i64,
    pub creator_username: String,
    #[serde(default)]
    pub creator_family_name: Option<String>,
    #[serde(default)]
    pub creator_given_name: Option<String>,
    #[serde(default)]
    pub social_media: Option<String>,
    pub immutable_audit_rules: bool,
    #[serde(default)]
    pub strategy_pool_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserUnfollowExpertRespRow {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserUnfollowStrategyRespRow {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserUpdateExpertProfileRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserUpdateStrategyInitialTokenRatioRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserUpdateStrategyRespRow {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunWatcherAddStrategyPoolContractRespRow {
    pub pkey_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunWatcherGetRawTransactionRespRow {
    pub transaction_cache_id: i64,
    pub transaction_hash: String,
    pub chain: String,
    #[serde(default)]
    pub dex: Option<String>,
    pub raw_transaction: String,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunWatcherListExpertListenedWalletAssetLedgerRespRow {
    pub pkey_id: i64,
    pub address: String,
    pub blockchain: EnumBlockChain,
    pub token_id: i64,
    pub entry: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunWatcherListStrategyEscrowPendingWalletLedgerRespRow {
    pub strategy_id: i64,
    pub blockchain: EnumBlockChain,
    pub address: String,
    pub token_id: i64,
    pub token_address: String,
    pub token_name: String,
    pub token_symbol: String,
    pub entry: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunWatcherListStrategyPoolContractRespRow {
    pub pkey_id: i64,
    pub strategy_id: i64,
    pub blockchain: EnumBlockChain,
    pub address: String,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunWatcherListUserStrategyLedgerRespRow {
    pub strategy_id: i64,
    pub user_id: i64,
    pub blockchain: EnumBlockChain,
    pub strategy_pool_contract_address: String,
    pub user_strategy_wallet_address: String,
    pub entry: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunWatcherSaveRawTransactionRespRow {
    pub transaction_cache_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunWatcherSaveStrategyPoolContractRespRow {
    pub pkey_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunWatcherSaveStrategyWatchingWalletTradeHistoryRespRow {
    pub strategy_watching_wallet_trade_history_id: i64,
    pub expert_watched_wallet_id: i64,
    pub fkey_token_in: i64,
    pub fkey_token_in_name: String,
    pub fkey_token_out: i64,
    pub fkey_token_out_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunWatcherUpsertExpertListenedWalletAssetLedgerRespRow {
    pub expert_listened_wallet_asset_ledger_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthSignupReq {
    pub address: String,
    pub email: String,
    pub phone: String,
    pub preferred_language: String,
    pub agreed_tos: bool,
    pub agreed_privacy: bool,
    pub ip_address: std::net::IpAddr,
    pub public_id: i64,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub age: Option<i32>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAuthSignupReq {
    type ResponseRow = FunAuthSignupRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_auth_signup(a_address => $1::varchar, a_email => $2::varchar, a_phone => $3::varchar, a_preferred_language => $4::varchar, a_agreed_tos => $5::boolean, a_agreed_privacy => $6::boolean, a_ip_address => $7::inet, a_public_id => $8::bigint, a_username => $9::varchar, a_age => $10::int);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.address as &(dyn ToSql + Sync),
            &self.email as &(dyn ToSql + Sync),
            &self.phone as &(dyn ToSql + Sync),
            &self.preferred_language as &(dyn ToSql + Sync),
            &self.agreed_tos as &(dyn ToSql + Sync),
            &self.agreed_privacy as &(dyn ToSql + Sync),
            &self.ip_address as &(dyn ToSql + Sync),
            &self.public_id as &(dyn ToSql + Sync),
            &self.username as &(dyn ToSql + Sync),
            &self.age as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthAuthenticateReq {
    pub address: String,
    pub service_code: i32,
    pub device_id: String,
    pub device_os: String,
    pub ip_address: std::net::IpAddr,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAuthAuthenticateReq {
    type ResponseRow = FunAuthAuthenticateRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_auth_authenticate(a_address => $1::varchar, a_service_code => $2::int, a_device_id => $3::varchar, a_device_os => $4::varchar, a_ip_address => $5::inet);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.address as &(dyn ToSql + Sync),
            &self.service_code as &(dyn ToSql + Sync),
            &self.device_id as &(dyn ToSql + Sync),
            &self.device_os as &(dyn ToSql + Sync),
            &self.ip_address as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthSetTokenReq {
    pub user_id: i64,
    pub user_token: uuid::Uuid,
    pub admin_token: uuid::Uuid,
    pub service_code: i32,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAuthSetTokenReq {
    type ResponseRow = FunAuthSetTokenRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_auth_set_token(a_user_id => $1::bigint, a_user_token => $2::uuid, a_admin_token => $3::uuid, a_service_code => $4::int);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.user_token as &(dyn ToSql + Sync),
            &self.admin_token as &(dyn ToSql + Sync),
            &self.service_code as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthRemoveTokenReq {
    pub user_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAuthRemoveTokenReq {
    type ResponseRow = FunAuthRemoveTokenRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_auth_remove_token(a_user_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.user_id as &(dyn ToSql + Sync)]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthAuthorizeReq {
    pub address: String,
    pub token: uuid::Uuid,
    pub service: EnumService,
    pub device_id: String,
    pub device_os: String,
    pub ip_address: std::net::IpAddr,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAuthAuthorizeReq {
    type ResponseRow = FunAuthAuthorizeRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_auth_authorize(a_address => $1::varchar, a_token => $2::uuid, a_service => $3::enum_service, a_device_id => $4::varchar, a_device_os => $5::varchar, a_ip_address => $6::inet);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.address as &(dyn ToSql + Sync),
            &self.token as &(dyn ToSql + Sync),
            &self.service as &(dyn ToSql + Sync),
            &self.device_id as &(dyn ToSql + Sync),
            &self.device_os as &(dyn ToSql + Sync),
            &self.ip_address as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthSetRoleReq {
    pub public_user_id: i64,
    pub role: EnumRole,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAuthSetRoleReq {
    type ResponseRow = FunAuthSetRoleRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_auth_set_role(a_public_user_id => $1::bigint, a_role => $2::enum_role);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.public_user_id as &(dyn ToSql + Sync),
            &self.role as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthChangeLoginWalletAddressReq {
    pub old_wallet_address: String,
    pub new_wallet_address: String,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAuthChangeLoginWalletAddressReq {
    type ResponseRow = FunAuthChangeLoginWalletAddressRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_auth_change_login_wallet_address(a_old_wallet_address => $1::varchar, a_new_wallet_address => $2::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.old_wallet_address as &(dyn ToSql + Sync),
            &self.new_wallet_address as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthUpdateUserTableReq {
    pub user_id: i64,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub family_name: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAuthUpdateUserTableReq {
    type ResponseRow = FunAuthUpdateUserTableRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_auth_update_user_table(a_user_id => $1::bigint, a_username => $2::varchar, a_family_name => $3::varchar, a_given_name => $4::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.username as &(dyn ToSql + Sync),
            &self.family_name as &(dyn ToSql + Sync),
            &self.given_name as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserFollowStrategyReq {
    pub user_id: i64,
    pub strategy_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserFollowStrategyReq {
    type ResponseRow = FunUserFollowStrategyRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_follow_strategy(a_user_id => $1::bigint, a_strategy_id => $2::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserUnfollowStrategyReq {
    pub user_id: i64,
    pub strategy_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserUnfollowStrategyReq {
    type ResponseRow = FunUserUnfollowStrategyRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_unfollow_strategy(a_user_id => $1::bigint, a_strategy_id => $2::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListFollowedStrategiesReq {
    pub user_id: i64,
    pub limit: i64,
    pub offset: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListFollowedStrategiesReq {
    type ResponseRow = FunUserStrategyRowType;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_followed_strategies(a_user_id => $1::bigint, a_limit => $2::bigint, a_offset => $3::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListStrategiesReq {
    pub user_id: i64,
    pub limit: i64,
    pub offset: i64,
    #[serde(default)]
    pub strategy_id: Option<i64>,
    #[serde(default)]
    pub strategy_name: Option<String>,
    #[serde(default)]
    pub expert_public_id: Option<i64>,
    #[serde(default)]
    pub expert_name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub blockchain: Option<EnumBlockChain>,
    #[serde(default)]
    pub wallet_address: Option<String>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListStrategiesReq {
    type ResponseRow = FunUserStrategyRowType;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_strategies(a_user_id => $1::bigint, a_limit => $2::bigint, a_offset => $3::bigint, a_strategy_id => $4::bigint, a_strategy_name => $5::varchar, a_expert_public_id => $6::bigint, a_expert_name => $7::varchar, a_description => $8::varchar, a_blockchain => $9::enum_block_chain, a_wallet_address => $10::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.strategy_name as &(dyn ToSql + Sync),
            &self.expert_public_id as &(dyn ToSql + Sync),
            &self.expert_name as &(dyn ToSql + Sync),
            &self.description as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.wallet_address as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListTopPerformingStrategiesReq {
    pub limit: i64,
    pub offset: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListTopPerformingStrategiesReq {
    type ResponseRow = FunUserListTopPerformingStrategiesRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_top_performing_strategies(a_limit => $1::bigint, a_offset => $2::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetStrategyStatisticsNetValueReq {
    pub strategy_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserGetStrategyStatisticsNetValueReq {
    type ResponseRow = FunUserGetStrategyStatisticsNetValueRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_get_strategy_statistics_net_value(a_strategy_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.strategy_id as &(dyn ToSql + Sync)]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetStrategyStatisticsFollowHistoryReq {
    pub strategy_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserGetStrategyStatisticsFollowHistoryReq {
    type ResponseRow = FunUserGetStrategyStatisticsFollowHistoryRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_get_strategy_statistics_follow_history(a_strategy_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.strategy_id as &(dyn ToSql + Sync)]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetStrategyStatisticsBackHistoryReq {
    pub strategy_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserGetStrategyStatisticsBackHistoryReq {
    type ResponseRow = FunUserGetStrategyStatisticsBackHistoryRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_get_strategy_statistics_back_history(a_strategy_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.strategy_id as &(dyn ToSql + Sync)]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserDepositToEscrowReq {
    pub user_id: i64,
    pub blockchain: EnumBlockChain,
    pub user_address: String,
    pub contract_address: String,
    pub receiver_address: String,
    pub quantity: String,
    pub transaction_hash: String,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserDepositToEscrowReq {
    type ResponseRow = FunUserDepositToEscrowRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_deposit_to_escrow(a_user_id => $1::bigint, a_blockchain => $2::enum_block_chain, a_user_address => $3::varchar, a_contract_address => $4::varchar, a_receiver_address => $5::varchar, a_quantity => $6::varchar, a_transaction_hash => $7::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.user_address as &(dyn ToSql + Sync),
            &self.contract_address as &(dyn ToSql + Sync),
            &self.receiver_address as &(dyn ToSql + Sync),
            &self.quantity as &(dyn ToSql + Sync),
            &self.transaction_hash as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserBackStrategyReq {
    pub user_id: i64,
    pub strategy_id: i64,
    pub quantity: String,
    pub new_total_backed_quantity: String,
    pub old_total_backed_quantity: String,
    pub new_current_quantity: String,
    pub old_current_quantity: String,
    pub blockchain: EnumBlockChain,
    pub transaction_hash: String,
    pub earn_sp_tokens: String,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserBackStrategyReq {
    type ResponseRow = FunUserBackStrategyRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_back_strategy(a_user_id => $1::bigint, a_strategy_id => $2::bigint, a_quantity => $3::varchar, a_new_total_backed_quantity => $4::varchar, a_old_total_backed_quantity => $5::varchar, a_new_current_quantity => $6::varchar, a_old_current_quantity => $7::varchar, a_blockchain => $8::enum_block_chain, a_transaction_hash => $9::varchar, a_earn_sp_tokens => $10::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.quantity as &(dyn ToSql + Sync),
            &self.new_total_backed_quantity as &(dyn ToSql + Sync),
            &self.old_total_backed_quantity as &(dyn ToSql + Sync),
            &self.new_current_quantity as &(dyn ToSql + Sync),
            &self.old_current_quantity as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.transaction_hash as &(dyn ToSql + Sync),
            &self.earn_sp_tokens as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListBackedStrategiesReq {
    pub user_id: i64,
    pub offset: i64,
    pub limit: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListBackedStrategiesReq {
    type ResponseRow = FunUserStrategyRowType;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_backed_strategies(a_user_id => $1::bigint, a_offset => $2::bigint, a_limit => $3::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.limit as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListBackStrategyHistoryReq {
    pub user_id: i64,
    #[serde(default)]
    pub strategy_id: Option<i64>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListBackStrategyHistoryReq {
    type ResponseRow = FunUserListBackStrategyHistoryRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_back_strategy_history(a_user_id => $1::bigint, a_strategy_id => $2::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserExitStrategyReq {
    pub user_id: i64,
    pub strategy_id: i64,
    pub quantity: String,
    pub redeem_sp_tokens: String,
    pub blockchain: EnumBlockChain,
    pub transaction_hash: String,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserExitStrategyReq {
    type ResponseRow = FunUserExitStrategyRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_exit_strategy(a_user_id => $1::bigint, a_strategy_id => $2::bigint, a_quantity => $3::varchar, a_redeem_sp_tokens => $4::varchar, a_blockchain => $5::enum_block_chain, a_transaction_hash => $6::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.quantity as &(dyn ToSql + Sync),
            &self.redeem_sp_tokens as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.transaction_hash as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListExitStrategyHistoryReq {
    pub user_id: i64,
    #[serde(default)]
    pub strategy_id: Option<i64>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListExitStrategyHistoryReq {
    type ResponseRow = FunUserListExitStrategyHistoryRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_exit_strategy_history(a_user_id => $1::bigint, a_strategy_id => $2::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserFollowExpertReq {
    pub user_id: i64,
    pub expert_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserFollowExpertReq {
    type ResponseRow = FunUserFollowExpertRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_follow_expert(a_user_id => $1::bigint, a_expert_id => $2::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.expert_id as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserUnfollowExpertReq {
    pub user_id: i64,
    pub expert_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserUnfollowExpertReq {
    type ResponseRow = FunUserUnfollowExpertRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_unfollow_expert(a_user_id => $1::bigint, a_expert_id => $2::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.expert_id as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListFollowedExpertsReq {
    pub user_id: i64,
    pub offset: i64,
    pub limit: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListFollowedExpertsReq {
    type ResponseRow = FunUserExpertRowType;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_followed_experts(a_user_id => $1::bigint, a_offset => $2::bigint, a_limit => $3::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.limit as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListExpertsReq {
    pub limit: i64,
    pub offset: i64,
    pub user_id: i64,
    pub sort_by_followers: bool,
    #[serde(default)]
    pub expert_id: Option<i64>,
    #[serde(default)]
    pub expert_user_id: Option<i64>,
    #[serde(default)]
    pub expert_user_public_id: Option<i64>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub family_name: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub social_media: Option<String>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListExpertsReq {
    type ResponseRow = FunUserExpertRowType;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_experts(a_limit => $1::bigint, a_offset => $2::bigint, a_user_id => $3::bigint, a_sort_by_followers => $4::boolean, a_expert_id => $5::bigint, a_expert_user_id => $6::bigint, a_expert_user_public_id => $7::bigint, a_username => $8::varchar, a_family_name => $9::varchar, a_given_name => $10::varchar, a_description => $11::varchar, a_social_media => $12::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
            &self.sort_by_followers as &(dyn ToSql + Sync),
            &self.expert_id as &(dyn ToSql + Sync),
            &self.expert_user_id as &(dyn ToSql + Sync),
            &self.expert_user_public_id as &(dyn ToSql + Sync),
            &self.username as &(dyn ToSql + Sync),
            &self.family_name as &(dyn ToSql + Sync),
            &self.given_name as &(dyn ToSql + Sync),
            &self.description as &(dyn ToSql + Sync),
            &self.social_media as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetExpertProfileReq {
    pub expert_id: i64,
    pub user_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserGetExpertProfileReq {
    type ResponseRow = FunUserExpertRowType;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_get_expert_profile(a_expert_id => $1::bigint, a_user_id => $2::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.expert_id as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetUserProfileReq {
    pub user_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserGetUserProfileReq {
    type ResponseRow = FunUserGetUserProfileRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_get_user_profile(a_user_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.user_id as &(dyn ToSql + Sync)]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserCreateExpertProfileReq {
    pub user_id: i64,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub social_media: Option<String>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserCreateExpertProfileReq {
    type ResponseRow = FunUserCreateExpertProfileRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_create_expert_profile(a_user_id => $1::bigint, a_description => $2::varchar, a_social_media => $3::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.description as &(dyn ToSql + Sync),
            &self.social_media as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserUpdateExpertProfileReq {
    pub expert_id: i64,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub social_media: Option<String>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserUpdateExpertProfileReq {
    type ResponseRow = FunUserUpdateExpertProfileRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_update_expert_profile(a_expert_id => $1::bigint, a_description => $2::varchar, a_social_media => $3::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.expert_id as &(dyn ToSql + Sync),
            &self.description as &(dyn ToSql + Sync),
            &self.social_media as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserApplyBecomeExpertReq {
    pub user_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserApplyBecomeExpertReq {
    type ResponseRow = FunUserApplyBecomeExpertRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_apply_become_expert(a_user_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.user_id as &(dyn ToSql + Sync)]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserCreateStrategyReq {
    pub user_id: i64,
    pub name: String,
    pub description: String,
    pub strategy_thesis_url: String,
    pub minimum_backing_amount_usd: f64,
    pub strategy_fee: f64,
    pub expert_fee: f64,
    pub agreed_tos: bool,
    pub wallet_address: String,
    pub blockchain: EnumBlockChain,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserCreateStrategyReq {
    type ResponseRow = FunUserCreateStrategyRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_create_strategy(a_user_id => $1::bigint, a_name => $2::varchar, a_description => $3::varchar, a_strategy_thesis_url => $4::varchar, a_minimum_backing_amount_usd => $5::double precision, a_strategy_fee => $6::double precision, a_expert_fee => $7::double precision, a_agreed_tos => $8::boolean, a_wallet_address => $9::varchar, a_blockchain => $10::enum_block_chain);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.name as &(dyn ToSql + Sync),
            &self.description as &(dyn ToSql + Sync),
            &self.strategy_thesis_url as &(dyn ToSql + Sync),
            &self.minimum_backing_amount_usd as &(dyn ToSql + Sync),
            &self.strategy_fee as &(dyn ToSql + Sync),
            &self.expert_fee as &(dyn ToSql + Sync),
            &self.agreed_tos as &(dyn ToSql + Sync),
            &self.wallet_address as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserUpdateStrategyReq {
    pub user_id: i64,
    pub strategy_id: i64,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub social_media: Option<String>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserUpdateStrategyReq {
    type ResponseRow = FunUserUpdateStrategyRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_update_strategy(a_user_id => $1::bigint, a_strategy_id => $2::bigint, a_name => $3::varchar, a_description => $4::varchar, a_social_media => $5::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.name as &(dyn ToSql + Sync),
            &self.description as &(dyn ToSql + Sync),
            &self.social_media as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserAddStrategyWatchWalletReq {
    pub user_id: i64,
    pub strategy_id: i64,
    pub wallet_address: String,
    pub blockchain: EnumBlockChain,
    pub ratio: f64,
    pub dex: String,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserAddStrategyWatchWalletReq {
    type ResponseRow = FunUserAddStrategyWatchWalletRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_add_strategy_watch_wallet(a_user_id => $1::bigint, a_strategy_id => $2::bigint, a_wallet_address => $3::varchar, a_blockchain => $4::enum_block_chain, a_ratio => $5::double precision, a_dex => $6::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.wallet_address as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.ratio as &(dyn ToSql + Sync),
            &self.dex as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserRemoveStrategyWatchWalletReq {
    pub user_id: i64,
    pub strategy_id: i64,
    pub watch_wallet_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserRemoveStrategyWatchWalletReq {
    type ResponseRow = FunUserRemoveStrategyWatchWalletRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_remove_strategy_watch_wallet(a_user_id => $1::bigint, a_strategy_id => $2::bigint, a_watch_wallet_id => $3::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.watch_wallet_id as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListStrategyWatchWalletsReq {
    pub strategy_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListStrategyWatchWalletsReq {
    type ResponseRow = FunUserListStrategyWatchWalletsRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_strategy_watch_wallets(a_strategy_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.strategy_id as &(dyn ToSql + Sync)]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListStrategyFollowersReq {
    pub strategy_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListStrategyFollowersReq {
    type ResponseRow = FunUserListStrategyFollowersRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_strategy_followers(a_strategy_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.strategy_id as &(dyn ToSql + Sync)]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListStrategyBackersReq {
    pub strategy_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListStrategyBackersReq {
    type ResponseRow = FunUserListStrategyBackersRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_strategy_backers(a_strategy_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.strategy_id as &(dyn ToSql + Sync)]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserAddRegisteredWalletReq {
    pub user_id: i64,
    pub blockchain: EnumBlockChain,
    pub address: String,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserAddRegisteredWalletReq {
    type ResponseRow = FunUserAddRegisteredWalletRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_add_registered_wallet(a_user_id => $1::bigint, a_blockchain => $2::enum_block_chain, a_address => $3::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.address as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserRemoveRegisteredWalletReq {
    pub registered_wallet_id: i64,
    pub user_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserRemoveRegisteredWalletReq {
    type ResponseRow = FunUserRemoveRegisteredWalletRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_remove_registered_wallet(a_registered_wallet_id => $1::bigint, a_user_id => $2::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.registered_wallet_id as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListRegisteredWalletsReq {
    pub user_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListRegisteredWalletsReq {
    type ResponseRow = FunUserListRegisteredWalletsRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_registered_wallets(a_user_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.user_id as &(dyn ToSql + Sync)]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserRequestRefundReq {
    pub user_id: i64,
    pub blockchain: EnumBlockChain,
    pub user_address: String,
    pub contract_address: String,
    pub receiver_address: String,
    pub quantity: String,
    pub transaction_hash: String,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserRequestRefundReq {
    type ResponseRow = FunUserRequestRefundRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_request_refund(a_user_id => $1::bigint, a_blockchain => $2::enum_block_chain, a_user_address => $3::varchar, a_contract_address => $4::varchar, a_receiver_address => $5::varchar, a_quantity => $6::varchar, a_transaction_hash => $7::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.user_address as &(dyn ToSql + Sync),
            &self.contract_address as &(dyn ToSql + Sync),
            &self.receiver_address as &(dyn ToSql + Sync),
            &self.quantity as &(dyn ToSql + Sync),
            &self.transaction_hash as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListRequestRefundHistoryReq {
    pub user_id: i64,
    pub limit: i64,
    pub offset: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListRequestRefundHistoryReq {
    type ResponseRow = FunUserListRequestRefundHistoryRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_request_refund_history(a_user_id => $1::bigint, a_limit => $2::bigint, a_offset => $3::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserAddStrategyInitialTokenRatioReq {
    pub strategy_id: i64,
    pub token_name: String,
    pub token_address: String,
    pub blockchain: EnumBlockChain,
    pub quantity: String,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserAddStrategyInitialTokenRatioReq {
    type ResponseRow = FunUserAddStrategyInitialTokenRatioRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_add_strategy_initial_token_ratio(a_strategy_id => $1::bigint, a_token_name => $2::varchar, a_token_address => $3::varchar, a_blockchain => $4::enum_block_chain, a_quantity => $5::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.token_name as &(dyn ToSql + Sync),
            &self.token_address as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.quantity as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserUpdateStrategyInitialTokenRatioReq {
    pub strategy_initial_token_ratio_id: i64,
    pub new_quantity: String,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserUpdateStrategyInitialTokenRatioReq {
    type ResponseRow = FunUserUpdateStrategyInitialTokenRatioRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_update_strategy_initial_token_ratio(a_strategy_initial_token_ratio_id => $1::bigint, a_new_quantity => $2::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_initial_token_ratio_id as &(dyn ToSql + Sync),
            &self.new_quantity as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserRemoveStrategyInitialTokenRatioReq {
    pub strategy_initial_token_ratio_id: i64,
    pub strategy_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserRemoveStrategyInitialTokenRatioReq {
    type ResponseRow = FunUserRemoveStrategyInitialTokenRatioRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_remove_strategy_initial_token_ratio(a_strategy_initial_token_ratio_id => $1::bigint, a_strategy_id => $2::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_initial_token_ratio_id as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListStrategyInitialTokenRatiosReq {
    pub strategy_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListStrategyInitialTokenRatiosReq {
    type ResponseRow = FunUserListStrategyInitialTokenRatiosRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_strategy_initial_token_ratios(a_strategy_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.strategy_id as &(dyn ToSql + Sync)]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetStrategyInitialTokenRatioByAddressAndChainReq {
    pub strategy_id: i64,
    pub token_address: String,
    pub blockchain: EnumBlockChain,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserGetStrategyInitialTokenRatioByAddressAndChainReq {
    type ResponseRow = FunUserGetStrategyInitialTokenRatioByAddressAndChainRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_get_strategy_initial_token_ratio_by_address_and_chain(a_strategy_id => $1::bigint, a_token_address => $2::varchar, a_blockchain => $3::enum_block_chain);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.token_address as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunExpertListFollowersReq {
    pub user_id: i64,
    pub limit: i64,
    pub offset: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunExpertListFollowersReq {
    type ResponseRow = FunExpertListFollowersRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_expert_list_followers(a_user_id => $1::bigint, a_limit => $2::bigint, a_offset => $3::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunExpertListBackersReq {
    pub user_id: i64,
    pub limit: i64,
    pub offset: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunExpertListBackersReq {
    type ResponseRow = FunExpertListBackersRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_expert_list_backers(a_user_id => $1::bigint, a_limit => $2::bigint, a_offset => $3::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListDepositHistoryReq {
    pub user_id: i64,
    pub limit: i64,
    pub offset: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListDepositHistoryReq {
    type ResponseRow = FunUserListDepositHistoryRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_deposit_history(a_user_id => $1::bigint, a_limit => $2::bigint, a_offset => $3::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetUserByAddressReq {
    pub address: String,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserGetUserByAddressReq {
    type ResponseRow = FunUserGetUserByAddressRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_get_user_by_address(a_address => $1::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.address as &(dyn ToSql + Sync)]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserAddStrategyWalletReq {
    pub user_id: i64,
    pub blockchain: EnumBlockChain,
    pub address: String,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserAddStrategyWalletReq {
    type ResponseRow = FunUserAddStrategyWalletRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_add_strategy_wallet(a_user_id => $1::bigint, a_blockchain => $2::enum_block_chain, a_address => $3::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.address as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListStrategyWalletsReq {
    pub user_id: i64,
    #[serde(default)]
    pub blockchain: Option<EnumBlockChain>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListStrategyWalletsReq {
    type ResponseRow = FunUserListStrategyWalletsRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_strategy_wallets(a_user_id => $1::bigint, a_blockchain => $2::enum_block_chain);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListStrategyAuditRulesReq {
    pub strategy_id: i64,
    #[serde(default)]
    pub audit_rule_id: Option<i64>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListStrategyAuditRulesReq {
    type ResponseRow = FunUserListStrategyAuditRulesRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_strategy_audit_rules(a_strategy_id => $1::bigint, a_audit_rule_id => $2::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.audit_rule_id as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserAddStrategyAuditRuleReq {
    pub strategy_id: i64,
    pub audit_rule_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserAddStrategyAuditRuleReq {
    type ResponseRow = FunUserAddStrategyAuditRuleRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_add_strategy_audit_rule(a_strategy_id => $1::bigint, a_audit_rule_id => $2::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.audit_rule_id as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserDelStrategyAuditRuleReq {
    pub strategy_id: i64,
    pub audit_rule_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserDelStrategyAuditRuleReq {
    type ResponseRow = FunUserDelStrategyAuditRuleRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_del_strategy_audit_rule(a_strategy_id => $1::bigint, a_audit_rule_id => $2::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.audit_rule_id as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserAddStrategyWhitelistedTokenReq {
    pub strategy_id: i64,
    pub token_name: String,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserAddStrategyWhitelistedTokenReq {
    type ResponseRow = FunUserAddStrategyWhitelistedTokenRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_add_strategy_whitelisted_token(a_strategy_id => $1::bigint, a_token_name => $2::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.token_name as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListStrategyWhitelistedTokensReq {
    pub strategy_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListStrategyWhitelistedTokensReq {
    type ResponseRow = FunUserListStrategyWhitelistedTokensRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_strategy_whitelisted_tokens(a_strategy_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.strategy_id as &(dyn ToSql + Sync)]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserCheckIfTokenWhitelistedReq {
    pub strategy_id: i64,
    pub token_name: String,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserCheckIfTokenWhitelistedReq {
    type ResponseRow = FunUserCheckIfTokenWhitelistedRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_check_if_token_whitelisted(a_strategy_id => $1::bigint, a_token_name => $2::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.token_name as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListAuditRulesReq {
    #[serde(default)]
    pub audit_rule_id: Option<i64>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListAuditRulesReq {
    type ResponseRow = FunUserListAuditRulesRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_audit_rules(a_audit_rule_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.audit_rule_id as &(dyn ToSql + Sync)]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminListUsersReq {
    pub limit: i64,
    pub offset: i64,
    #[serde(default)]
    pub user_id: Option<i64>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub role: Option<EnumRole>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminListUsersReq {
    type ResponseRow = FunAdminListUsersRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_list_users(a_limit => $1::bigint, a_offset => $2::bigint, a_user_id => $3::bigint, a_address => $4::varchar, a_username => $5::varchar, a_email => $6::varchar, a_role => $7::enum_role);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
            &self.address as &(dyn ToSql + Sync),
            &self.username as &(dyn ToSql + Sync),
            &self.email as &(dyn ToSql + Sync),
            &self.role as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminSetUserRoleReq {
    pub user_id: i64,
    pub role: EnumRole,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminSetUserRoleReq {
    type ResponseRow = FunAdminSetUserRoleRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_set_user_role(a_user_id => $1::bigint, a_role => $2::enum_role);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.role as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminSetBlockUserReq {
    pub user_id: i64,
    pub blocked: bool,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminSetBlockUserReq {
    type ResponseRow = FunAdminSetBlockUserRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_set_block_user(a_user_id => $1::bigint, a_blocked => $2::boolean);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.blocked as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminApproveUserBecomeExpertReq {
    pub user_public_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminApproveUserBecomeExpertReq {
    type ResponseRow = FunAdminApproveUserBecomeExpertRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_approve_user_become_expert(a_user_public_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.user_public_id as &(dyn ToSql + Sync)]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminRejectUserBecomeExpertReq {
    pub user_public_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminRejectUserBecomeExpertReq {
    type ResponseRow = FunAdminRejectUserBecomeExpertRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_reject_user_become_expert(a_user_public_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.user_public_id as &(dyn ToSql + Sync)]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminListPendingUserExpertApplicationsReq {
    pub limit: i64,
    pub offset: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminListPendingUserExpertApplicationsReq {
    type ResponseRow = FunAdminListPendingUserExpertApplicationsRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_list_pending_user_expert_applications(a_limit => $1::bigint, a_offset => $2::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminGetSystemConfigReq {
    pub config_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminGetSystemConfigReq {
    type ResponseRow = FunAdminGetSystemConfigRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_get_system_config(a_config_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.config_id as &(dyn ToSql + Sync)]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminUpdateSystemConfigReq {
    pub config_id: i64,
    #[serde(default)]
    pub config_placeholder_1: Option<i64>,
    #[serde(default)]
    pub config_placeholder_2: Option<i64>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminUpdateSystemConfigReq {
    type ResponseRow = FunAdminUpdateSystemConfigRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_update_system_config(a_config_id => $1::bigint, a_config_placeholder_1 => $2::bigint, a_config_placeholder_2 => $3::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.config_id as &(dyn ToSql + Sync),
            &self.config_placeholder_1 as &(dyn ToSql + Sync),
            &self.config_placeholder_2 as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminListExpertsReq {
    pub limit: i64,
    pub offset: i64,
    #[serde(default)]
    pub expert_id: Option<i64>,
    #[serde(default)]
    pub user_id: Option<i64>,
    #[serde(default)]
    pub user_public_id: Option<i64>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub family_name: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub social_media: Option<String>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminListExpertsReq {
    type ResponseRow = FunUserExpertRowType;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_list_experts(a_limit => $1::bigint, a_offset => $2::bigint, a_expert_id => $3::bigint, a_user_id => $4::bigint, a_user_public_id => $5::bigint, a_username => $6::varchar, a_family_name => $7::varchar, a_given_name => $8::varchar, a_description => $9::varchar, a_social_media => $10::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.expert_id as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
            &self.user_public_id as &(dyn ToSql + Sync),
            &self.username as &(dyn ToSql + Sync),
            &self.family_name as &(dyn ToSql + Sync),
            &self.given_name as &(dyn ToSql + Sync),
            &self.description as &(dyn ToSql + Sync),
            &self.social_media as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminListBackersReq {
    pub offset: i64,
    pub limit: i64,
    #[serde(default)]
    pub user_id: Option<i64>,
    #[serde(default)]
    pub user_public_id: Option<i64>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub family_name: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminListBackersReq {
    type ResponseRow = FunAdminListBackersRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_list_backers(a_offset => $1::bigint, a_limit => $2::bigint, a_user_id => $3::bigint, a_user_public_id => $4::bigint, a_username => $5::varchar, a_family_name => $6::varchar, a_given_name => $7::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.offset as &(dyn ToSql + Sync),
            &self.limit as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
            &self.user_public_id as &(dyn ToSql + Sync),
            &self.username as &(dyn ToSql + Sync),
            &self.family_name as &(dyn ToSql + Sync),
            &self.given_name as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminListStrategiesReq {
    pub limit: i64,
    pub offset: i64,
    #[serde(default)]
    pub strategy_id: Option<i64>,
    #[serde(default)]
    pub strategy_name: Option<String>,
    #[serde(default)]
    pub expert_public_id: Option<i64>,
    #[serde(default)]
    pub expert_name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub approved: Option<bool>,
    #[serde(default)]
    pub pending_approval: Option<bool>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminListStrategiesReq {
    type ResponseRow = FunUserStrategyRowType;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_list_strategies(a_limit => $1::bigint, a_offset => $2::bigint, a_strategy_id => $3::bigint, a_strategy_name => $4::varchar, a_expert_public_id => $5::bigint, a_expert_name => $6::varchar, a_description => $7::varchar, a_approved => $8::boolean, a_pending_approval => $9::boolean);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.strategy_name as &(dyn ToSql + Sync),
            &self.expert_public_id as &(dyn ToSql + Sync),
            &self.expert_name as &(dyn ToSql + Sync),
            &self.description as &(dyn ToSql + Sync),
            &self.approved as &(dyn ToSql + Sync),
            &self.pending_approval as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminApproveStrategyReq {
    pub strategy_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminApproveStrategyReq {
    type ResponseRow = FunAdminApproveStrategyRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_approve_strategy(a_strategy_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.strategy_id as &(dyn ToSql + Sync)]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminRejectStrategyReq {
    pub strategy_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminRejectStrategyReq {
    type ResponseRow = FunAdminRejectStrategyRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_reject_strategy(a_strategy_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.strategy_id as &(dyn ToSql + Sync)]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminAddAuditRuleReq {
    pub rule_id: i64,
    pub name: String,
    pub description: String,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminAddAuditRuleReq {
    type ResponseRow = FunAdminAddAuditRuleRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_add_audit_rule(a_rule_id => $1::bigint, a_name => $2::varchar, a_description => $3::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.rule_id as &(dyn ToSql + Sync),
            &self.name as &(dyn ToSql + Sync),
            &self.description as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherSaveRawTransactionReq {
    pub transaction_hash: String,
    pub chain: String,
    pub raw_transaction: String,
    #[serde(default)]
    pub dex: Option<String>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherSaveRawTransactionReq {
    type ResponseRow = FunWatcherSaveRawTransactionRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_save_raw_transaction(a_transaction_hash => $1::varchar, a_chain => $2::varchar, a_raw_transaction => $3::varchar, a_dex => $4::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.transaction_hash as &(dyn ToSql + Sync),
            &self.chain as &(dyn ToSql + Sync),
            &self.raw_transaction as &(dyn ToSql + Sync),
            &self.dex as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherGetRawTransactionReq {
    pub transaction_hash: String,
    pub chain: String,
    #[serde(default)]
    pub dex: Option<String>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherGetRawTransactionReq {
    type ResponseRow = FunWatcherGetRawTransactionRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_get_raw_transaction(a_transaction_hash => $1::varchar, a_chain => $2::varchar, a_dex => $3::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.transaction_hash as &(dyn ToSql + Sync),
            &self.chain as &(dyn ToSql + Sync),
            &self.dex as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherSaveStrategyWatchingWalletTradeHistoryReq {
    pub address: String,
    pub transaction_hash: String,
    pub blockchain: EnumBlockChain,
    pub contract_address: String,
    #[serde(default)]
    pub dex: Option<String>,
    #[serde(default)]
    pub token_in_address: Option<String>,
    #[serde(default)]
    pub token_out_address: Option<String>,
    #[serde(default)]
    pub amount_in: Option<String>,
    #[serde(default)]
    pub amount_out: Option<String>,
    #[serde(default)]
    pub happened_at: Option<i64>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherSaveStrategyWatchingWalletTradeHistoryReq {
    type ResponseRow = FunWatcherSaveStrategyWatchingWalletTradeHistoryRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_save_strategy_watching_wallet_trade_history(a_address => $1::varchar, a_transaction_hash => $2::varchar, a_blockchain => $3::enum_block_chain, a_contract_address => $4::varchar, a_dex => $5::varchar, a_token_in_address => $6::varchar, a_token_out_address => $7::varchar, a_amount_in => $8::varchar, a_amount_out => $9::varchar, a_happened_at => $10::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.address as &(dyn ToSql + Sync),
            &self.transaction_hash as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.contract_address as &(dyn ToSql + Sync),
            &self.dex as &(dyn ToSql + Sync),
            &self.token_in_address as &(dyn ToSql + Sync),
            &self.token_out_address as &(dyn ToSql + Sync),
            &self.amount_in as &(dyn ToSql + Sync),
            &self.amount_out as &(dyn ToSql + Sync),
            &self.happened_at as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherListStrategyEscrowPendingWalletLedgerReq {
    #[serde(default)]
    pub strategy_id: Option<i64>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherListStrategyEscrowPendingWalletLedgerReq {
    type ResponseRow = FunWatcherListStrategyEscrowPendingWalletLedgerRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_list_strategy_escrow_pending_wallet_ledger(a_strategy_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.strategy_id as &(dyn ToSql + Sync)]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherListUserStrategyLedgerReq {
    pub limit: i64,
    pub offset: i64,
    #[serde(default)]
    pub strategy_id: Option<i64>,
    #[serde(default)]
    pub user_id: Option<i64>,
    #[serde(default)]
    pub blockchain: Option<EnumBlockChain>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherListUserStrategyLedgerReq {
    type ResponseRow = FunWatcherListUserStrategyLedgerRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_list_user_strategy_ledger(a_limit => $1::bigint, a_offset => $2::bigint, a_strategy_id => $3::bigint, a_user_id => $4::bigint, a_blockchain => $5::enum_block_chain);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherUpsertExpertListenedWalletAssetLedgerReq {
    pub address: String,
    pub blockchain: EnumBlockChain,
    pub token_id: i64,
    pub old_entry: String,
    pub new_entry: String,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherUpsertExpertListenedWalletAssetLedgerReq {
    type ResponseRow = FunWatcherUpsertExpertListenedWalletAssetLedgerRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_upsert_expert_listened_wallet_asset_ledger(a_address => $1::varchar, a_blockchain => $2::enum_block_chain, a_token_id => $3::bigint, a_old_entry => $4::varchar, a_new_entry => $5::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.address as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.token_id as &(dyn ToSql + Sync),
            &self.old_entry as &(dyn ToSql + Sync),
            &self.new_entry as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherListExpertListenedWalletAssetLedgerReq {
    pub limit: i64,
    pub offset: i64,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub blockchain: Option<EnumBlockChain>,
    #[serde(default)]
    pub token_id: Option<i64>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherListExpertListenedWalletAssetLedgerReq {
    type ResponseRow = FunWatcherListExpertListenedWalletAssetLedgerRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_list_expert_listened_wallet_asset_ledger(a_limit => $1::bigint, a_offset => $2::bigint, a_address => $3::varchar, a_blockchain => $4::enum_block_chain, a_token_id => $5::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.address as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.token_id as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherSaveStrategyPoolContractReq {
    pub strategy_id: i64,
    pub blockchain: EnumBlockChain,
    pub address: String,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherSaveStrategyPoolContractReq {
    type ResponseRow = FunWatcherSaveStrategyPoolContractRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_save_strategy_pool_contract(a_strategy_id => $1::bigint, a_blockchain => $2::enum_block_chain, a_address => $3::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.address as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherAddStrategyPoolContractReq {
    pub strategy_id: i64,
    pub blockchain: EnumBlockChain,
    pub address: String,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherAddStrategyPoolContractReq {
    type ResponseRow = FunWatcherAddStrategyPoolContractRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_add_strategy_pool_contract(a_strategy_id => $1::bigint, a_blockchain => $2::enum_block_chain, a_address => $3::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.address as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherListStrategyPoolContractReq {
    pub limit: i64,
    pub offset: i64,
    #[serde(default)]
    pub strategy_id: Option<i64>,
    #[serde(default)]
    pub blockchain: Option<EnumBlockChain>,
    #[serde(default)]
    pub address: Option<String>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherListStrategyPoolContractReq {
    type ResponseRow = FunWatcherListStrategyPoolContractRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_list_strategy_pool_contract(a_limit => $1::bigint, a_offset => $2::bigint, a_strategy_id => $3::bigint, a_blockchain => $4::enum_block_chain, a_address => $5::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.address as &(dyn ToSql + Sync),
        ]
    }
}
