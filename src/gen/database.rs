use crate::model::*;
use lib::database::*;
use lib::types::*;
use postgres_from_row::FromRow;
use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAdminAddAuditRuleRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAdminAddEscrowContractAddressRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAdminAddEscrowTokenContractAddressRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAdminApproveStrategyRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAdminApproveUserBecomeExpertRespRow {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAdminGetSystemConfigRespRow {
    #[serde(default)]
    pub platform_fee: Option<f64>,
    #[serde(default)]
    pub config_placeholder_2: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAdminListBackersRespRow {
    pub total: i64,
    pub user_id: i64,
    pub user_public_id: i64,
    pub username: String,
    pub login_wallet_address: BlockchainAddress,
    pub joined_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAdminListEscrowContractAddressRespRow {
    pub pkey_id: i64,
    pub blockchain: EnumBlockChain,
    pub address: BlockchainAddress,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAdminListEscrowTokenContractAddressRespRow {
    pub pkey_id: i64,
    pub symbol: String,
    pub short_name: String,
    pub description: String,
    pub address: BlockchainAddress,
    pub blockchain: EnumBlockChain,
    pub is_stablecoin: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunAdminListPendingUserExpertApplicationsRespRow {
    pub total: i64,
    pub user_public_id: i64,
    pub name: String,
    pub linked_wallet: BlockchainAddress,
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
    pub address: BlockchainAddress,
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
    pub total: i64,
    pub public_id: i64,
    pub username: String,
    #[serde(default)]
    pub family_name: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
    pub linked_wallet: BlockchainAddress,
    pub backed_at: i64,
    pub joined_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunExpertListFollowersRespRow {
    pub total: i64,
    pub public_id: i64,
    pub username: String,
    #[serde(default)]
    pub family_name: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
    pub linked_wallet: BlockchainAddress,
    pub followed_at: i64,
    pub joined_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserAddStrategyAuditRuleRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserAddStrategyInitialTokenRatioRespRow {
    pub strategy_initial_token_ratio_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserAddStrategyPoolContractRespRow {
    pub strategy_pool_contract_id: i64,
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
pub struct FunUserAddUserStrategyPoolContractAssetLedgerEntryRespRow {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserAddWhitelistedWalletRespRow {
    pub whitelisted_wallet_id: i64,
}

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
pub struct FunUserCalculateUserEscrowBalanceFromLedgerRespRow {
    pub wallet_address: BlockchainAddress,
    pub balance: BlockchainDecimal,
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
    pub strategy_count: i64,
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
    pub linked_wallet: BlockchainAddress,
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
pub struct FunUserGetStrategyIdFromWatchingWalletRespRow {
    pub strategy_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserGetStrategyStatisticsBackLedgerRespRow {
    pub time: i64,
    pub backer_count: f64,
    pub backer_quantity_usd: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserGetStrategyStatisticsFollowLedgerRespRow {
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
pub struct FunUserListBackStrategyLedgerRespRow {
    pub total: i64,
    pub back_ledger_id: i64,
    pub user_id: i64,
    pub strategy_id: i64,
    pub quantity: BlockchainDecimal,
    pub blockchain: EnumBlockChain,
    pub transaction_hash: BlockchainTransactionHash,
    pub happened_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListDepositWithdrawLedgerRespRow {
    pub total: i64,
    pub transaction_id: i64,
    pub blockchain: EnumBlockChain,
    pub user_address: BlockchainAddress,
    pub contract_address: BlockchainAddress,
    pub receiver_address: BlockchainAddress,
    pub quantity: BlockchainDecimal,
    pub transaction_hash: BlockchainTransactionHash,
    pub is_deposit: bool,
    pub happened_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListEscrowTokenContractAddressRespRow {
    pub total: i64,
    pub token_id: i64,
    pub blockchain: EnumBlockChain,
    pub address: BlockchainAddress,
    pub symbol: String,
    pub short_name: String,
    pub description: String,
    pub is_stablecoin: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListExitStrategyLedgerRespRow {
    pub total: i64,
    pub back_ledger_id: i64,
    pub user_id: i64,
    pub strategy_id: i64,
    pub quantity: BlockchainDecimal,
    pub blockchain: EnumBlockChain,
    pub transaction_hash: BlockchainTransactionHash,
    pub happened_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListRequestRefundLedgerRespRow {
    pub request_refund_id: i64,
    pub user_id: i64,
    pub blockchain: EnumBlockChain,
    pub quantity: BlockchainDecimal,
    pub wallet_address: BlockchainAddress,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListStrategyAuditRulesRespRow {
    pub rule_id: i64,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListStrategyBackersRespRow {
    pub user_id: i64,
    pub total: i64,
    pub user_public_id: i64,
    pub username: String,
    pub wallet_address: BlockchainAddress,
    pub backed_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListStrategyFollowersRespRow {
    pub total: i64,
    pub user_id: i64,
    pub user_public_id: i64,
    pub username: String,
    pub wallet_address: BlockchainAddress,
    pub followed_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListStrategyInitialTokenRatiosRespRow {
    pub total: i64,
    pub blockchain: EnumBlockChain,
    pub token_id: i64,
    pub token_name: String,
    pub token_address: BlockchainAddress,
    pub quantity: BlockchainDecimal,
    pub strategy_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListStrategyPoolContractAssetLedgerRespRow {
    pub entry_id: i64,
    pub strategy_id: i64,
    pub token_id: i64,
    pub token_symbol: String,
    pub blockchain: EnumBlockChain,
    pub transaction_hash: BlockchainTransactionHash,
    #[serde(default)]
    pub dex: Option<String>,
    pub amount: BlockchainDecimal,
    pub is_add: bool,
    pub happened_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListStrategyWalletsRespRow {
    pub total: i64,
    pub wallet_id: i64,
    pub blockchain: EnumBlockChain,
    pub address: BlockchainAddress,
    pub is_platform_managed: bool,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListStrategyWatchWalletsRespRow {
    pub watch_wallet_id: i64,
    pub wallet_address: BlockchainAddress,
    pub blockchain: EnumBlockChain,
    pub ratio: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListStrategyWhitelistedTokensRespRow {
    pub token_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListUserBackStrategyAttemptRespRow {
    pub total: i64,
    pub user_back_strategy_attempt_id: i64,
    pub strategy_id: i64,
    pub strategy_name: String,
    pub token_id: i64,
    pub token_symbol: String,
    pub back_quantity: BlockchainDecimal,
    pub strategy_wallet_address: BlockchainAddress,
    pub log_id: i64,
    pub happened_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListUserBackStrategyLogRespRow {
    pub total: i64,
    pub log_entry_id: i64,
    pub message: String,
    pub happened_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListUserDepositWithdrawBalanceRespRow {
    pub deposit_withdraw_balance_id: i64,
    pub user_id: i64,
    pub blockchain: EnumBlockChain,
    pub token_id: i64,
    pub token_symbol: String,
    pub token_name: String,
    pub balance: BlockchainDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListUserStrategyBalanceRespRow {
    pub total: i64,
    pub strategy_id: i64,
    pub strategy_name: String,
    pub balance: BlockchainDecimal,
    pub user_strategy_wallet_address: BlockchainAddress,
    pub blockchain: EnumBlockChain,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListUserStrategyPoolContractAssetBalancesRespRow {
    pub user_id: i64,
    pub strategy_wallet_id: i64,
    pub strategy_wallet_address: BlockchainAddress,
    pub is_strategy_wallet_managed: bool,
    pub token_id: i64,
    pub token_name: String,
    pub token_symbol: String,
    pub token_address: BlockchainAddress,
    pub blockchain: EnumBlockChain,
    pub balance: BlockchainDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListUserStrategyPoolContractAssetLedgerEntriesRespRow {
    pub user_strategy_pool_contract_asset_ledger_id: i64,
    pub strategy_pool_contract_id: i64,
    pub strategy_id: i64,
    pub strategy_wallet_id: i64,
    pub strategy_wallet_address: BlockchainAddress,
    pub is_strategy_wallet_managed: bool,
    pub token_id: i64,
    pub token_symbol: String,
    pub token_name: String,
    pub token_address: BlockchainAddress,
    pub blockchain: EnumBlockChain,
    pub amount: BlockchainDecimal,
    pub happened_at: i64,
    pub is_add: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserListWhitelistedWalletsRespRow {
    pub registered_wallet_id: i64,
    pub blockchain: EnumBlockChain,
    pub address: BlockchainAddress,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserReduceQuantityFromUserDepositWithdrawLedgerRespRow {
    pub request_refund_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserRemoveStrategyInitialTokenRatioRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserRemoveStrategyWatchWalletRespRow {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserRemoveWhitelistedWalletRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserRequestRefundRespRow {
    pub request_refund_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserSaveUserBackStrategyAttemptRespRow {
    pub user_back_strategy_attempt_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserSaveUserBackStrategyLogRespRow {}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserStrategyRowType {
    pub total: i64,
    pub strategy_id: i64,
    pub strategy_name: String,
    pub strategy_description: String,
    pub current_usdc: BlockchainDecimal,
    pub total_backed_usdc: BlockchainDecimal,
    pub total_exited_usdc: BlockchainDecimal,
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
    pub blockchain: EnumBlockChain,
    #[serde(default)]
    pub strategy_pool_address: Option<BlockchainAddress>,
    #[serde(default)]
    pub number_of_tokens: Option<i64>,
    #[serde(default)]
    pub swap_fee: Option<f64>,
    #[serde(default)]
    pub platform_fee: Option<f64>,
    #[serde(default)]
    pub expert_fee: Option<f64>,
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
pub struct FunUserUpdateUserDepositWithdrawBalanceRespRow {
    pub updated: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunUserUpsertUserStrategyPoolContractAssetBalanceRespRow {
    pub user_strategy_pool_contract_asset_balance_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunWatcherGetExpertWalletAssetsFromLedgerRespRow {
    pub token_id: i64,
    pub token_name: String,
    pub token_symbol: String,
    pub token_address: BlockchainAddress,
    pub blockchain: EnumBlockChain,
    pub amount: BlockchainDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunWatcherGetRawTransactionRespRow {
    pub transaction_cache_id: i64,
    pub transaction_hash: BlockchainTransactionHash,
    pub chain: String,
    #[serde(default)]
    pub dex: Option<String>,
    pub raw_transaction: String,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunWatcherListExpertListenedWalletAssetBalanceRespRow {
    pub pkey_id: i64,
    pub address: BlockchainAddress,
    pub blockchain: EnumBlockChain,
    pub token_id: i64,
    pub balance: BlockchainDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunWatcherListLastDexTradesForPairRespRow {
    pub transaction_hash: BlockchainTransactionHash,
    pub blockchain: EnumBlockChain,
    pub dex: EnumDex,
    pub token_in_id: i64,
    pub token_out_id: i64,
    pub amount_in: BlockchainDecimal,
    pub amount_out: BlockchainDecimal,
    pub happened_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunWatcherListStrategyEscrowPendingWalletBalanceRespRow {
    pub strategy_id: i64,
    pub blockchain: EnumBlockChain,
    pub address: BlockchainAddress,
    pub token_id: i64,
    pub token_address: BlockchainAddress,
    pub token_name: String,
    pub token_symbol: String,
    pub balance: BlockchainDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunWatcherListStrategyPoolContractAssetBalancesRespRow {
    pub token_id: i64,
    pub token_name: String,
    pub token_symbol: String,
    pub token_address: BlockchainAddress,
    pub blockchain: EnumBlockChain,
    pub balance: BlockchainDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunWatcherListStrategyPoolContractRespRow {
    pub pkey_id: i64,
    pub strategy_id: i64,
    pub blockchain: EnumBlockChain,
    pub address: BlockchainAddress,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunWatcherListUserStrategyBalanceRespRow {
    pub strategy_id: i64,
    pub user_id: i64,
    pub blockchain: EnumBlockChain,
    pub strategy_pool_contract_address: BlockchainAddress,
    pub user_strategy_wallet_address: BlockchainAddress,
    pub balance: BlockchainDecimal,
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
pub struct FunWatcherSaveStrategyWatchingWalletTradeLedgerRespRow {
    pub strategy_watching_wallet_trade_ledger_id: i64,
    pub expert_watched_wallet_id: i64,
    pub fkey_token_in: i64,
    pub fkey_token_in_name: String,
    pub fkey_token_out: i64,
    pub fkey_token_out_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunWatcherSaveUserDepositWithdrawLedgerRespRow {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunWatcherUpsertExpertListenedWalletAssetBalanceRespRow {
    pub expert_listened_wallet_asset_balance_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunWatcherUpsertLastDexTradeForPairRespRow {
    pub last_dex_trade_for_pair_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunWatcherUpsertStrategyPoolContractAssetBalanceRespRow {
    pub strategy_contract_asset_balance_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunWatcherUpsertUserDepositWithdrawBalanceRespRow {
    pub ret_pkey_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct FunWatcherUpsertUserStrategyBalanceRespRow {
    pub ret_pkey_id: i64,
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
    pub expert_id: Option<i64>,
    #[serde(default)]
    pub expert_public_id: Option<i64>,
    #[serde(default)]
    pub expert_name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub blockchain: Option<EnumBlockChain>,
    #[serde(default)]
    pub strategy_pool_address: Option<BlockchainAddress>,
    #[serde(default)]
    pub approved: Option<bool>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListStrategiesReq {
    type ResponseRow = FunUserStrategyRowType;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_strategies(a_user_id => $1::bigint, a_limit => $2::bigint, a_offset => $3::bigint, a_strategy_id => $4::bigint, a_strategy_name => $5::varchar, a_expert_id => $6::bigint, a_expert_public_id => $7::bigint, a_expert_name => $8::varchar, a_description => $9::varchar, a_blockchain => $10::enum_block_chain, a_strategy_pool_address => $11::varchar, a_approved => $12::boolean);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.strategy_name as &(dyn ToSql + Sync),
            &self.expert_id as &(dyn ToSql + Sync),
            &self.expert_public_id as &(dyn ToSql + Sync),
            &self.expert_name as &(dyn ToSql + Sync),
            &self.description as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.strategy_pool_address as &(dyn ToSql + Sync),
            &self.approved as &(dyn ToSql + Sync),
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
pub struct FunUserGetStrategyStatisticsFollowLedgerReq {
    pub strategy_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserGetStrategyStatisticsFollowLedgerReq {
    type ResponseRow = FunUserGetStrategyStatisticsFollowLedgerRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_get_strategy_statistics_follow_ledger(a_strategy_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.strategy_id as &(dyn ToSql + Sync)]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetStrategyStatisticsBackLedgerReq {
    pub strategy_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserGetStrategyStatisticsBackLedgerReq {
    type ResponseRow = FunUserGetStrategyStatisticsBackLedgerRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_get_strategy_statistics_back_ledger(a_strategy_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.strategy_id as &(dyn ToSql + Sync)]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserBackStrategyReq {
    pub user_id: i64,
    pub strategy_id: i64,
    pub quantity: BlockchainDecimal,
    pub new_total_backed_quantity: BlockchainDecimal,
    pub old_total_backed_quantity: BlockchainDecimal,
    pub new_current_quantity: BlockchainDecimal,
    pub old_current_quantity: BlockchainDecimal,
    pub blockchain: EnumBlockChain,
    pub transaction_hash: BlockchainTransactionHash,
    pub earn_sp_tokens: BlockchainDecimal,
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
pub struct FunUserListBackStrategyLedgerReq {
    pub limit: i64,
    pub offset: i64,
    #[serde(default)]
    pub user_id: Option<i64>,
    #[serde(default)]
    pub strategy_id: Option<i64>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListBackStrategyLedgerReq {
    type ResponseRow = FunUserListBackStrategyLedgerRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_back_strategy_ledger(a_limit => $1::bigint, a_offset => $2::bigint, a_user_id => $3::bigint, a_strategy_id => $4::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListExitStrategyLedgerReq {
    pub limit: i64,
    pub offset: i64,
    #[serde(default)]
    pub user_id: Option<i64>,
    #[serde(default)]
    pub strategy_id: Option<i64>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListExitStrategyLedgerReq {
    type ResponseRow = FunUserListExitStrategyLedgerRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_exit_strategy_ledger(a_limit => $1::bigint, a_offset => $2::bigint, a_user_id => $3::bigint, a_strategy_id => $4::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserExitStrategyReq {
    pub user_id: i64,
    pub strategy_id: i64,
    pub quantity: BlockchainDecimal,
    pub redeem_sp_tokens: BlockchainDecimal,
    pub blockchain: EnumBlockChain,
    pub transaction_hash: BlockchainTransactionHash,
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
pub struct FunUserListUserStrategyPoolContractAssetLedgerEntriesReq {
    pub limit: i64,
    pub offset: i64,
    pub user_id: i64,
    pub strategy_pool_contract_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListUserStrategyPoolContractAssetLedgerEntriesReq {
    type ResponseRow = FunUserListUserStrategyPoolContractAssetLedgerEntriesRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_user_strategy_pool_contract_asset_ledger_entries(a_limit => $1::bigint, a_offset => $2::bigint, a_user_id => $3::bigint, a_strategy_pool_contract_id => $4::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
            &self.strategy_pool_contract_id as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserAddUserStrategyPoolContractAssetLedgerEntryReq {
    pub strategy_wallet_id: i64,
    pub strategy_pool_contract_id: i64,
    pub token_address: BlockchainAddress,
    pub blockchain: EnumBlockChain,
    pub amount: BlockchainDecimal,
    pub is_add: bool,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserAddUserStrategyPoolContractAssetLedgerEntryReq {
    type ResponseRow = FunUserAddUserStrategyPoolContractAssetLedgerEntryRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_add_user_strategy_pool_contract_asset_ledger_entry(a_strategy_wallet_id => $1::bigint, a_strategy_pool_contract_id => $2::bigint, a_token_address => $3::varchar, a_blockchain => $4::enum_block_chain, a_amount => $5::varchar, a_is_add => $6::boolean);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_wallet_id as &(dyn ToSql + Sync),
            &self.strategy_pool_contract_id as &(dyn ToSql + Sync),
            &self.token_address as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.amount as &(dyn ToSql + Sync),
            &self.is_add as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListUserStrategyPoolContractAssetBalancesReq {
    #[serde(default)]
    pub strategy_pool_contract_id: Option<i64>,
    #[serde(default)]
    pub user_id: Option<i64>,
    #[serde(default)]
    pub strategy_wallet_id: Option<i64>,
    #[serde(default)]
    pub token_address: Option<BlockchainAddress>,
    #[serde(default)]
    pub blockchain: Option<EnumBlockChain>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListUserStrategyPoolContractAssetBalancesReq {
    type ResponseRow = FunUserListUserStrategyPoolContractAssetBalancesRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_user_strategy_pool_contract_asset_balances(a_strategy_pool_contract_id => $1::bigint, a_user_id => $2::bigint, a_strategy_wallet_id => $3::bigint, a_token_address => $4::varchar, a_blockchain => $5::enum_block_chain);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_pool_contract_id as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
            &self.strategy_wallet_id as &(dyn ToSql + Sync),
            &self.token_address as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserUpsertUserStrategyPoolContractAssetBalanceReq {
    pub strategy_wallet_id: i64,
    pub strategy_pool_contract_id: i64,
    pub token_address: BlockchainAddress,
    pub blockchain: EnumBlockChain,
    pub old_balance: BlockchainDecimal,
    pub new_balance: BlockchainDecimal,
    pub amount: BlockchainDecimal,
    pub is_add: bool,
    pub transaction_hash: BlockchainTransactionHash,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserUpsertUserStrategyPoolContractAssetBalanceReq {
    type ResponseRow = FunUserUpsertUserStrategyPoolContractAssetBalanceRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_upsert_user_strategy_pool_contract_asset_balance(a_strategy_wallet_id => $1::bigint, a_strategy_pool_contract_id => $2::bigint, a_token_address => $3::varchar, a_blockchain => $4::enum_block_chain, a_old_balance => $5::varchar, a_new_balance => $6::varchar, a_amount => $7::varchar, a_is_add => $8::boolean, a_transaction_hash => $9::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_wallet_id as &(dyn ToSql + Sync),
            &self.strategy_pool_contract_id as &(dyn ToSql + Sync),
            &self.token_address as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.old_balance as &(dyn ToSql + Sync),
            &self.new_balance as &(dyn ToSql + Sync),
            &self.amount as &(dyn ToSql + Sync),
            &self.is_add as &(dyn ToSql + Sync),
            &self.transaction_hash as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListStrategyPoolContractAssetLedgerReq {
    pub limit: i64,
    pub offset: i64,
    #[serde(default)]
    pub strategy_id: Option<i64>,
    #[serde(default)]
    pub token_id: Option<i64>,
    #[serde(default)]
    pub blockchain: Option<EnumBlockChain>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListStrategyPoolContractAssetLedgerReq {
    type ResponseRow = FunUserListStrategyPoolContractAssetLedgerRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_strategy_pool_contract_asset_ledger(a_limit => $1::bigint, a_offset => $2::bigint, a_strategy_id => $3::bigint, a_token_id => $4::bigint, a_blockchain => $5::enum_block_chain);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.token_id as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
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
    pub swap_fee: f64,
    pub expert_fee: f64,
    pub agreed_tos: bool,
    pub wallet_address: BlockchainAddress,
    pub blockchain: EnumBlockChain,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserCreateStrategyReq {
    type ResponseRow = FunUserCreateStrategyRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_create_strategy(a_user_id => $1::bigint, a_name => $2::varchar, a_description => $3::varchar, a_strategy_thesis_url => $4::varchar, a_minimum_backing_amount_usd => $5::double precision, a_swap_fee => $6::double precision, a_expert_fee => $7::double precision, a_agreed_tos => $8::boolean, a_wallet_address => $9::varchar, a_blockchain => $10::enum_block_chain);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.name as &(dyn ToSql + Sync),
            &self.description as &(dyn ToSql + Sync),
            &self.strategy_thesis_url as &(dyn ToSql + Sync),
            &self.minimum_backing_amount_usd as &(dyn ToSql + Sync),
            &self.swap_fee as &(dyn ToSql + Sync),
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
    pub wallet_address: BlockchainAddress,
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
pub struct FunUserAddWhitelistedWalletReq {
    pub user_id: i64,
    pub blockchain: EnumBlockChain,
    pub address: BlockchainAddress,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserAddWhitelistedWalletReq {
    type ResponseRow = FunUserAddWhitelistedWalletRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_add_whitelisted_wallet(a_user_id => $1::bigint, a_blockchain => $2::enum_block_chain, a_address => $3::varchar);"
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
pub struct FunUserRemoveWhitelistedWalletReq {
    pub whitelisted_wallet_id: i64,
    pub user_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserRemoveWhitelistedWalletReq {
    type ResponseRow = FunUserRemoveWhitelistedWalletRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_remove_whitelisted_wallet(a_whitelisted_wallet_id => $1::bigint, a_user_id => $2::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.whitelisted_wallet_id as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListWhitelistedWalletsReq {
    pub limit: i64,
    pub offset: i64,
    #[serde(default)]
    pub user_id: Option<i64>,
    #[serde(default)]
    pub blockchain: Option<EnumBlockChain>,
    #[serde(default)]
    pub address: Option<BlockchainAddress>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListWhitelistedWalletsReq {
    type ResponseRow = FunUserListWhitelistedWalletsRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_whitelisted_wallets(a_limit => $1::bigint, a_offset => $2::bigint, a_user_id => $3::bigint, a_blockchain => $4::enum_block_chain, a_address => $5::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.address as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserReduceQuantityFromUserDepositWithdrawLedgerReq {
    pub user_id: i64,
    pub token_id: i64,
    pub blockchain: EnumBlockChain,
    pub user_address: BlockchainAddress,
    pub contract_address: BlockchainAddress,
    pub contract_address_id: i64,
    pub receiver_address: BlockchainAddress,
    pub quantity: BlockchainDecimal,
    pub transaction_hash: BlockchainTransactionHash,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserReduceQuantityFromUserDepositWithdrawLedgerReq {
    type ResponseRow = FunUserReduceQuantityFromUserDepositWithdrawLedgerRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_reduce_quantity_from_user_deposit_withdraw_ledger(a_user_id => $1::bigint, a_token_id => $2::bigint, a_blockchain => $3::enum_block_chain, a_user_address => $4::varchar, a_contract_address => $5::varchar, a_contract_address_id => $6::bigint, a_receiver_address => $7::varchar, a_quantity => $8::varchar, a_transaction_hash => $9::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.token_id as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.user_address as &(dyn ToSql + Sync),
            &self.contract_address as &(dyn ToSql + Sync),
            &self.contract_address_id as &(dyn ToSql + Sync),
            &self.receiver_address as &(dyn ToSql + Sync),
            &self.quantity as &(dyn ToSql + Sync),
            &self.transaction_hash as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserRequestRefundReq {
    pub user_id: i64,
    pub token_id: i64,
    pub blockchain: EnumBlockChain,
    pub user_address: BlockchainAddress,
    pub contract_address: BlockchainAddress,
    pub contract_address_id: i64,
    pub receiver_address: BlockchainAddress,
    pub quantity: BlockchainDecimal,
    pub transaction_hash: BlockchainTransactionHash,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserRequestRefundReq {
    type ResponseRow = FunUserRequestRefundRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_request_refund(a_user_id => $1::bigint, a_token_id => $2::bigint, a_blockchain => $3::enum_block_chain, a_user_address => $4::varchar, a_contract_address => $5::varchar, a_contract_address_id => $6::bigint, a_receiver_address => $7::varchar, a_quantity => $8::varchar, a_transaction_hash => $9::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.token_id as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.user_address as &(dyn ToSql + Sync),
            &self.contract_address as &(dyn ToSql + Sync),
            &self.contract_address_id as &(dyn ToSql + Sync),
            &self.receiver_address as &(dyn ToSql + Sync),
            &self.quantity as &(dyn ToSql + Sync),
            &self.transaction_hash as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListRequestRefundLedgerReq {
    pub user_id: i64,
    pub limit: i64,
    pub offset: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListRequestRefundLedgerReq {
    type ResponseRow = FunUserListRequestRefundLedgerRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_request_refund_ledger(a_user_id => $1::bigint, a_limit => $2::bigint, a_offset => $3::bigint);"
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
    pub token_id: i64,
    pub quantity: BlockchainDecimal,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserAddStrategyInitialTokenRatioReq {
    type ResponseRow = FunUserAddStrategyInitialTokenRatioRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_add_strategy_initial_token_ratio(a_strategy_id => $1::bigint, a_token_id => $2::bigint, a_quantity => $3::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.token_id as &(dyn ToSql + Sync),
            &self.quantity as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserUpdateStrategyInitialTokenRatioReq {
    pub strategy_id: i64,
    pub token_id: i64,
    pub new_quantity: BlockchainDecimal,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserUpdateStrategyInitialTokenRatioReq {
    type ResponseRow = FunUserUpdateStrategyInitialTokenRatioRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_update_strategy_initial_token_ratio(a_strategy_id => $1::bigint, a_token_id => $2::bigint, a_new_quantity => $3::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.token_id as &(dyn ToSql + Sync),
            &self.new_quantity as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserRemoveStrategyInitialTokenRatioReq {
    pub strategy_id: i64,
    pub token_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserRemoveStrategyInitialTokenRatioReq {
    type ResponseRow = FunUserRemoveStrategyInitialTokenRatioRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_remove_strategy_initial_token_ratio(a_strategy_id => $1::bigint, a_token_id => $2::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.token_id as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListStrategyInitialTokenRatiosReq {
    pub strategy_id: i64,
    #[serde(default)]
    pub token_id: Option<i64>,
    #[serde(default)]
    pub token_address: Option<BlockchainAddress>,
    #[serde(default)]
    pub blockchain: Option<EnumBlockChain>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListStrategyInitialTokenRatiosReq {
    type ResponseRow = FunUserListStrategyInitialTokenRatiosRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_strategy_initial_token_ratios(a_strategy_id => $1::bigint, a_token_id => $2::bigint, a_token_address => $3::varchar, a_blockchain => $4::enum_block_chain);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.token_id as &(dyn ToSql + Sync),
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
pub struct FunUserListDepositWithdrawLedgerReq {
    pub limit: i64,
    pub offset: i64,
    #[serde(default)]
    pub user_id: Option<i64>,
    #[serde(default)]
    pub is_deposit: Option<bool>,
    #[serde(default)]
    pub is_back: Option<bool>,
    #[serde(default)]
    pub is_withdraw: Option<bool>,
    #[serde(default)]
    pub blockchain: Option<EnumBlockChain>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListDepositWithdrawLedgerReq {
    type ResponseRow = FunUserListDepositWithdrawLedgerRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_deposit_withdraw_ledger(a_limit => $1::bigint, a_offset => $2::bigint, a_user_id => $3::bigint, a_is_deposit => $4::boolean, a_is_back => $5::boolean, a_is_withdraw => $6::boolean, a_blockchain => $7::enum_block_chain);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
            &self.is_deposit as &(dyn ToSql + Sync),
            &self.is_back as &(dyn ToSql + Sync),
            &self.is_withdraw as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetUserByAddressReq {
    pub address: BlockchainAddress,
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
    pub address: BlockchainAddress,
    pub is_platform_managed: bool,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserAddStrategyWalletReq {
    type ResponseRow = FunUserAddStrategyWalletRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_add_strategy_wallet(a_user_id => $1::bigint, a_blockchain => $2::enum_block_chain, a_address => $3::varchar, a_is_platform_managed => $4::boolean);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.address as &(dyn ToSql + Sync),
            &self.is_platform_managed as &(dyn ToSql + Sync),
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
pub struct FunUserGetStrategyIdFromWatchingWalletReq {
    pub blockchain: EnumBlockChain,
    pub address: BlockchainAddress,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserGetStrategyIdFromWatchingWalletReq {
    type ResponseRow = FunUserGetStrategyIdFromWatchingWalletRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_get_strategy_id_from_watching_wallet(a_blockchain => $1::enum_block_chain, a_address => $2::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.blockchain as &(dyn ToSql + Sync),
            &self.address as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListUserDepositWithdrawBalanceReq {
    pub limit: i64,
    pub offset: i64,
    pub user_id: i64,
    #[serde(default)]
    pub blockchain: Option<EnumBlockChain>,
    #[serde(default)]
    pub token_address: Option<BlockchainAddress>,
    #[serde(default)]
    pub token_id: Option<i64>,
    #[serde(default)]
    pub escrow_contract_address: Option<BlockchainAddress>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListUserDepositWithdrawBalanceReq {
    type ResponseRow = FunUserListUserDepositWithdrawBalanceRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_user_deposit_withdraw_balance(a_limit => $1::bigint, a_offset => $2::bigint, a_user_id => $3::bigint, a_blockchain => $4::enum_block_chain, a_token_address => $5::varchar, a_token_id => $6::bigint, a_escrow_contract_address => $7::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.token_address as &(dyn ToSql + Sync),
            &self.token_id as &(dyn ToSql + Sync),
            &self.escrow_contract_address as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserUpdateUserDepositWithdrawBalanceReq {
    pub deposit_withdraw_balance_id: i64,
    pub old_balance: BlockchainDecimal,
    pub new_balance: BlockchainDecimal,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserUpdateUserDepositWithdrawBalanceReq {
    type ResponseRow = FunUserUpdateUserDepositWithdrawBalanceRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_update_user_deposit_withdraw_balance(a_deposit_withdraw_balance_id => $1::bigint, a_old_balance => $2::varchar, a_new_balance => $3::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.deposit_withdraw_balance_id as &(dyn ToSql + Sync),
            &self.old_balance as &(dyn ToSql + Sync),
            &self.new_balance as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserAddStrategyPoolContractReq {
    pub strategy_id: i64,
    pub blockchain: EnumBlockChain,
    pub address: BlockchainAddress,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserAddStrategyPoolContractReq {
    type ResponseRow = FunUserAddStrategyPoolContractRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_add_strategy_pool_contract(a_strategy_id => $1::bigint, a_blockchain => $2::enum_block_chain, a_address => $3::varchar);"
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
pub struct FunUserListEscrowTokenContractAddressReq {
    pub limit: i64,
    pub offset: i64,
    #[serde(default)]
    pub token_id: Option<i64>,
    #[serde(default)]
    pub blockchain: Option<EnumBlockChain>,
    #[serde(default)]
    pub address: Option<BlockchainAddress>,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub is_stablecoin: Option<bool>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListEscrowTokenContractAddressReq {
    type ResponseRow = FunUserListEscrowTokenContractAddressRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_escrow_token_contract_address(a_limit => $1::bigint, a_offset => $2::bigint, a_token_id => $3::bigint, a_blockchain => $4::enum_block_chain, a_address => $5::varchar, a_symbol => $6::varchar, a_is_stablecoin => $7::boolean);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.token_id as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.address as &(dyn ToSql + Sync),
            &self.symbol as &(dyn ToSql + Sync),
            &self.is_stablecoin as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListUserStrategyBalanceReq {
    pub limit: i64,
    pub offset: i64,
    pub user_id: i64,
    #[serde(default)]
    pub strategy_id: Option<i64>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListUserStrategyBalanceReq {
    type ResponseRow = FunUserListUserStrategyBalanceRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_user_strategy_balance(a_limit => $1::bigint, a_offset => $2::bigint, a_user_id => $3::bigint, a_strategy_id => $4::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserSaveUserBackStrategyAttemptReq {
    pub strategy_id: i64,
    pub user_id: i64,
    pub token_id: i64,
    pub back_quantity: BlockchainDecimal,
    pub strategy_wallet_address: BlockchainAddress,
    pub log_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserSaveUserBackStrategyAttemptReq {
    type ResponseRow = FunUserSaveUserBackStrategyAttemptRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_save_user_back_strategy_attempt(a_strategy_id => $1::bigint, a_user_id => $2::bigint, a_token_id => $3::bigint, a_back_quantity => $4::varchar, a_strategy_wallet_address => $5::varchar, a_log_id => $6::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
            &self.token_id as &(dyn ToSql + Sync),
            &self.back_quantity as &(dyn ToSql + Sync),
            &self.strategy_wallet_address as &(dyn ToSql + Sync),
            &self.log_id as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListUserBackStrategyAttemptReq {
    pub limit: i64,
    pub offset: i64,
    #[serde(default)]
    pub user_id: Option<i64>,
    #[serde(default)]
    pub strategy_id: Option<i64>,
    #[serde(default)]
    pub token_id: Option<i64>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListUserBackStrategyAttemptReq {
    type ResponseRow = FunUserListUserBackStrategyAttemptRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_user_back_strategy_attempt(a_limit => $1::bigint, a_offset => $2::bigint, a_user_id => $3::bigint, a_strategy_id => $4::bigint, a_token_id => $5::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.token_id as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserSaveUserBackStrategyLogReq {
    pub user_back_strategy_attempt_id: i64,
    pub message: String,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserSaveUserBackStrategyLogReq {
    type ResponseRow = FunUserSaveUserBackStrategyLogRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_save_user_back_strategy_log(a_user_back_strategy_attempt_id => $1::bigint, a_message => $2::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_back_strategy_attempt_id as &(dyn ToSql + Sync),
            &self.message as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListUserBackStrategyLogReq {
    pub limit: i64,
    pub offset: i64,
    pub user_back_strategy_attempt_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListUserBackStrategyLogReq {
    type ResponseRow = FunUserListUserBackStrategyLogRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_user_back_strategy_log(a_limit => $1::bigint, a_offset => $2::bigint, a_user_back_strategy_attempt_id => $3::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.user_back_strategy_attempt_id as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserCalculateUserEscrowBalanceFromLedgerReq {
    pub user_id: i64,
    pub token_id: i64,
    pub blockchain: EnumBlockChain,
    #[serde(default)]
    pub wallet_address: Option<BlockchainAddress>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserCalculateUserEscrowBalanceFromLedgerReq {
    type ResponseRow = FunUserCalculateUserEscrowBalanceFromLedgerRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_calculate_user_escrow_balance_from_ledger(a_user_id => $1::bigint, a_token_id => $2::bigint, a_blockchain => $3::enum_block_chain, a_wallet_address => $4::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.token_id as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.wallet_address as &(dyn ToSql + Sync),
        ]
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
    pub platform_fee: Option<f64>,
    #[serde(default)]
    pub config_placeholder_2: Option<i64>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminUpdateSystemConfigReq {
    type ResponseRow = FunAdminUpdateSystemConfigRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_update_system_config(a_config_id => $1::bigint, a_platform_fee => $2::double precision, a_config_placeholder_2 => $3::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.config_id as &(dyn ToSql + Sync),
            &self.platform_fee as &(dyn ToSql + Sync),
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
pub struct FunAdminAddEscrowTokenContractAddressReq {
    pub pkey_id: i64,
    pub symbol: String,
    pub short_name: String,
    pub description: String,
    pub address: BlockchainAddress,
    pub blockchain: EnumBlockChain,
    pub is_stablecoin: bool,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminAddEscrowTokenContractAddressReq {
    type ResponseRow = FunAdminAddEscrowTokenContractAddressRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_add_escrow_token_contract_address(a_pkey_id => $1::bigint, a_symbol => $2::varchar, a_short_name => $3::varchar, a_description => $4::varchar, a_address => $5::varchar, a_blockchain => $6::enum_block_chain, a_is_stablecoin => $7::boolean);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.pkey_id as &(dyn ToSql + Sync),
            &self.symbol as &(dyn ToSql + Sync),
            &self.short_name as &(dyn ToSql + Sync),
            &self.description as &(dyn ToSql + Sync),
            &self.address as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.is_stablecoin as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminListEscrowTokenContractAddressReq {
    pub limit: i64,
    pub offset: i64,
    #[serde(default)]
    pub blockchain: Option<EnumBlockChain>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminListEscrowTokenContractAddressReq {
    type ResponseRow = FunAdminListEscrowTokenContractAddressRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_list_escrow_token_contract_address(a_limit => $1::bigint, a_offset => $2::bigint, a_blockchain => $3::enum_block_chain);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminAddEscrowContractAddressReq {
    pub pkey_id: i64,
    pub blockchain: EnumBlockChain,
    pub address: BlockchainAddress,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminAddEscrowContractAddressReq {
    type ResponseRow = FunAdminAddEscrowContractAddressRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_add_escrow_contract_address(a_pkey_id => $1::bigint, a_blockchain => $2::enum_block_chain, a_address => $3::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.pkey_id as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.address as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminListEscrowContractAddressReq {
    pub limit: i64,
    pub offset: i64,
    #[serde(default)]
    pub blockchain: Option<EnumBlockChain>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminListEscrowContractAddressReq {
    type ResponseRow = FunAdminListEscrowContractAddressRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_list_escrow_contract_address(a_limit => $1::bigint, a_offset => $2::bigint, a_blockchain => $3::enum_block_chain);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherSaveUserDepositWithdrawLedgerReq {
    pub user_id: i64,
    pub blockchain: EnumBlockChain,
    pub user_address: BlockchainAddress,
    pub contract_address: BlockchainAddress,
    pub receiver_address: BlockchainAddress,
    pub quantity: BlockchainDecimal,
    pub transaction_hash: BlockchainTransactionHash,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherSaveUserDepositWithdrawLedgerReq {
    type ResponseRow = FunWatcherSaveUserDepositWithdrawLedgerRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_save_user_deposit_withdraw_ledger(a_user_id => $1::bigint, a_blockchain => $2::enum_block_chain, a_user_address => $3::varchar, a_contract_address => $4::varchar, a_receiver_address => $5::varchar, a_quantity => $6::varchar, a_transaction_hash => $7::varchar);"
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
pub struct FunWatcherSaveRawTransactionReq {
    pub transaction_hash: BlockchainTransactionHash,
    pub blockchain: EnumBlockChain,
    pub raw_transaction: String,
    #[serde(default)]
    pub dex: Option<String>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherSaveRawTransactionReq {
    type ResponseRow = FunWatcherSaveRawTransactionRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_save_raw_transaction(a_transaction_hash => $1::varchar, a_blockchain => $2::enum_block_chain, a_raw_transaction => $3::varchar, a_dex => $4::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.transaction_hash as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.raw_transaction as &(dyn ToSql + Sync),
            &self.dex as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherGetRawTransactionReq {
    pub transaction_hash: BlockchainTransactionHash,
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
pub struct FunWatcherSaveStrategyWatchingWalletTradeLedgerReq {
    pub address: BlockchainAddress,
    pub transaction_hash: BlockchainTransactionHash,
    pub blockchain: EnumBlockChain,
    pub contract_address: BlockchainAddress,
    #[serde(default)]
    pub dex: Option<String>,
    #[serde(default)]
    pub token_in_address: Option<BlockchainAddress>,
    #[serde(default)]
    pub token_out_address: Option<BlockchainAddress>,
    #[serde(default)]
    pub amount_in: Option<BlockchainDecimal>,
    #[serde(default)]
    pub amount_out: Option<BlockchainDecimal>,
    #[serde(default)]
    pub happened_at: Option<i64>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherSaveStrategyWatchingWalletTradeLedgerReq {
    type ResponseRow = FunWatcherSaveStrategyWatchingWalletTradeLedgerRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_save_strategy_watching_wallet_trade_ledger(a_address => $1::varchar, a_transaction_hash => $2::varchar, a_blockchain => $3::enum_block_chain, a_contract_address => $4::varchar, a_dex => $5::varchar, a_token_in_address => $6::varchar, a_token_out_address => $7::varchar, a_amount_in => $8::varchar, a_amount_out => $9::varchar, a_happened_at => $10::bigint);"
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
pub struct FunWatcherGetExpertWalletAssetsFromLedgerReq {
    pub strategy_id: i64,
    #[serde(default)]
    pub blockchain: Option<EnumBlockChain>,
    #[serde(default)]
    pub symbol: Option<String>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherGetExpertWalletAssetsFromLedgerReq {
    type ResponseRow = FunWatcherGetExpertWalletAssetsFromLedgerRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_get_expert_wallet_assets_from_ledger(a_strategy_id => $1::bigint, a_blockchain => $2::enum_block_chain, a_symbol => $3::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.symbol as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherListLastDexTradesForPairReq {
    pub token_in_address: BlockchainAddress,
    pub token_out_address: BlockchainAddress,
    pub blockchain: EnumBlockChain,
    #[serde(default)]
    pub dex: Option<EnumDex>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherListLastDexTradesForPairReq {
    type ResponseRow = FunWatcherListLastDexTradesForPairRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_list_last_dex_trades_for_pair(a_token_in_address => $1::varchar, a_token_out_address => $2::varchar, a_blockchain => $3::enum_block_chain, a_dex => $4::enum_dex);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.token_in_address as &(dyn ToSql + Sync),
            &self.token_out_address as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.dex as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherUpsertLastDexTradeForPairReq {
    pub transaction_hash: BlockchainTransactionHash,
    pub blockchain: EnumBlockChain,
    pub dex: EnumDex,
    pub token_in_address: BlockchainAddress,
    pub token_out_address: BlockchainAddress,
    pub amount_in: BlockchainDecimal,
    pub amount_out: BlockchainDecimal,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherUpsertLastDexTradeForPairReq {
    type ResponseRow = FunWatcherUpsertLastDexTradeForPairRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_upsert_last_dex_trade_for_pair(a_transaction_hash => $1::varchar, a_blockchain => $2::enum_block_chain, a_dex => $3::enum_dex, a_token_in_address => $4::varchar, a_token_out_address => $5::varchar, a_amount_in => $6::varchar, a_amount_out => $7::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.transaction_hash as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.dex as &(dyn ToSql + Sync),
            &self.token_in_address as &(dyn ToSql + Sync),
            &self.token_out_address as &(dyn ToSql + Sync),
            &self.amount_in as &(dyn ToSql + Sync),
            &self.amount_out as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherUpsertStrategyPoolContractAssetBalanceReq {
    pub strategy_pool_contract_id: i64,
    pub token_address: BlockchainAddress,
    pub blockchain: EnumBlockChain,
    pub new_balance: BlockchainDecimal,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherUpsertStrategyPoolContractAssetBalanceReq {
    type ResponseRow = FunWatcherUpsertStrategyPoolContractAssetBalanceRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_upsert_strategy_pool_contract_asset_balance(a_strategy_pool_contract_id => $1::bigint, a_token_address => $2::varchar, a_blockchain => $3::enum_block_chain, a_new_balance => $4::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_pool_contract_id as &(dyn ToSql + Sync),
            &self.token_address as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.new_balance as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherListStrategyPoolContractAssetBalancesReq {
    #[serde(default)]
    pub strategy_pool_contract_id: Option<i64>,
    #[serde(default)]
    pub strategy_id: Option<i64>,
    #[serde(default)]
    pub blockchain: Option<EnumBlockChain>,
    #[serde(default)]
    pub token_address: Option<BlockchainAddress>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherListStrategyPoolContractAssetBalancesReq {
    type ResponseRow = FunWatcherListStrategyPoolContractAssetBalancesRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_list_strategy_pool_contract_asset_balances(a_strategy_pool_contract_id => $1::bigint, a_strategy_id => $2::bigint, a_blockchain => $3::enum_block_chain, a_token_address => $4::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_pool_contract_id as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.token_address as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherListStrategyEscrowPendingWalletBalanceReq {
    #[serde(default)]
    pub strategy_id: Option<i64>,
    #[serde(default)]
    pub token_address: Option<BlockchainAddress>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherListStrategyEscrowPendingWalletBalanceReq {
    type ResponseRow = FunWatcherListStrategyEscrowPendingWalletBalanceRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_list_strategy_escrow_pending_wallet_balance(a_strategy_id => $1::bigint, a_token_address => $2::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.token_address as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherListUserStrategyBalanceReq {
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
impl DatabaseRequest for FunWatcherListUserStrategyBalanceReq {
    type ResponseRow = FunWatcherListUserStrategyBalanceRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_list_user_strategy_balance(a_limit => $1::bigint, a_offset => $2::bigint, a_strategy_id => $3::bigint, a_user_id => $4::bigint, a_blockchain => $5::enum_block_chain);"
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
pub struct FunWatcherUpsertExpertListenedWalletAssetBalanceReq {
    pub address: BlockchainAddress,
    pub blockchain: EnumBlockChain,
    pub token_id: i64,
    pub old_balance: BlockchainDecimal,
    pub new_balance: BlockchainDecimal,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherUpsertExpertListenedWalletAssetBalanceReq {
    type ResponseRow = FunWatcherUpsertExpertListenedWalletAssetBalanceRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_upsert_expert_listened_wallet_asset_balance(a_address => $1::varchar, a_blockchain => $2::enum_block_chain, a_token_id => $3::bigint, a_old_balance => $4::varchar, a_new_balance => $5::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.address as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.token_id as &(dyn ToSql + Sync),
            &self.old_balance as &(dyn ToSql + Sync),
            &self.new_balance as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherListExpertListenedWalletAssetBalanceReq {
    pub limit: i64,
    pub offset: i64,
    #[serde(default)]
    pub address: Option<BlockchainAddress>,
    #[serde(default)]
    pub blockchain: Option<EnumBlockChain>,
    #[serde(default)]
    pub token_id: Option<i64>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherListExpertListenedWalletAssetBalanceReq {
    type ResponseRow = FunWatcherListExpertListenedWalletAssetBalanceRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_list_expert_listened_wallet_asset_balance(a_limit => $1::bigint, a_offset => $2::bigint, a_address => $3::varchar, a_blockchain => $4::enum_block_chain, a_token_id => $5::bigint);"
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
    pub address: BlockchainAddress,
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
pub struct FunWatcherUpsertUserStrategyBalanceReq {
    pub user_id: i64,
    pub strategy_id: i64,
    pub blockchain: EnumBlockChain,
    pub old_balance: BlockchainDecimal,
    pub new_balance: BlockchainDecimal,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherUpsertUserStrategyBalanceReq {
    type ResponseRow = FunWatcherUpsertUserStrategyBalanceRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_upsert_user_strategy_balance(a_user_id => $1::bigint, a_strategy_id => $2::bigint, a_blockchain => $3::enum_block_chain, a_old_balance => $4::varchar, a_new_balance => $5::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.old_balance as &(dyn ToSql + Sync),
            &self.new_balance as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherUpsertUserDepositWithdrawBalanceReq {
    pub user_id: i64,
    pub token_address: BlockchainAddress,
    pub escrow_contract_address: BlockchainAddress,
    pub blockchain: EnumBlockChain,
    pub old_balance: BlockchainDecimal,
    pub new_balance: BlockchainDecimal,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherUpsertUserDepositWithdrawBalanceReq {
    type ResponseRow = FunWatcherUpsertUserDepositWithdrawBalanceRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_upsert_user_deposit_withdraw_balance(a_user_id => $1::bigint, a_token_address => $2::varchar, a_escrow_contract_address => $3::varchar, a_blockchain => $4::enum_block_chain, a_old_balance => $5::varchar, a_new_balance => $6::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.token_address as &(dyn ToSql + Sync),
            &self.escrow_contract_address as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.old_balance as &(dyn ToSql + Sync),
            &self.new_balance as &(dyn ToSql + Sync),
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
    pub address: Option<BlockchainAddress>,
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
