use lib::error_code::ErrorCode;
use lib::types::*;
use lib::ws::*;
use num_derive::FromPrimitive;
use serde::*;
use strum_macros::{Display, EnumString};
use tokio_postgres::types::*;

#[derive(
    Debug,
    Clone,
    Copy,
    ToSql,
    FromSql,
    Serialize,
    Deserialize,
    FromPrimitive,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    EnumString,
    Display,
    Hash,
)]
#[postgres(name = "enum_role")]
pub enum EnumRole {
    ///
    #[postgres(name = "guest")]
    Guest = 0,
    ///
    #[postgres(name = "user")]
    User = 1,
    ///
    #[postgres(name = "expert")]
    Expert = 2,
    ///
    #[postgres(name = "admin")]
    Admin = 3,
    ///
    #[postgres(name = "developer")]
    Developer = 4,
}
#[derive(
    Debug,
    Clone,
    Copy,
    ToSql,
    FromSql,
    Serialize,
    Deserialize,
    FromPrimitive,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    EnumString,
    Display,
    Hash,
)]
#[postgres(name = "enum_block_chain")]
pub enum EnumBlockChain {
    ///
    #[postgres(name = "EthereumMainnet")]
    EthereumMainnet = 0,
    ///
    #[postgres(name = "EthereumGoerli")]
    EthereumGoerli = 1,
    ///
    #[postgres(name = "BscMainnet")]
    BscMainnet = 2,
    ///
    #[postgres(name = "BscTestnet")]
    BscTestnet = 3,
    ///
    #[postgres(name = "LocalNet")]
    LocalNet = 4,
    ///
    #[postgres(name = "EthereumSepolia")]
    EthereumSepolia = 5,
}
#[derive(
    Debug,
    Clone,
    Copy,
    ToSql,
    FromSql,
    Serialize,
    Deserialize,
    FromPrimitive,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    EnumString,
    Display,
    Hash,
)]
#[postgres(name = "enum_blockchain_coin")]
pub enum EnumBlockchainCoin {
    ///
    #[postgres(name = "USDC")]
    USDC = 0,
    ///
    #[postgres(name = "USDT")]
    USDT = 1,
    ///
    #[postgres(name = "BUSD")]
    BUSD = 2,
    ///
    #[postgres(name = "WETH")]
    WETH = 4,
    ///
    #[postgres(name = "WBNB")]
    WBNB = 5,
}
#[derive(
    Debug,
    Clone,
    Copy,
    ToSql,
    FromSql,
    Serialize,
    Deserialize,
    FromPrimitive,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    EnumString,
    Display,
    Hash,
)]
#[postgres(name = "enum_dex")]
pub enum EnumDex {
    ///
    #[postgres(name = "UniSwap")]
    UniSwap = 0,
    ///
    #[postgres(name = "PancakeSwap")]
    PancakeSwap = 1,
    ///
    #[postgres(name = "SushiSwap")]
    SushiSwap = 2,
}
#[derive(
    Debug,
    Clone,
    Copy,
    ToSql,
    FromSql,
    Serialize,
    Deserialize,
    FromPrimitive,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    EnumString,
    Display,
    Hash,
)]
#[postgres(name = "enum_dex_version")]
pub enum EnumDexVersion {
    ///
    #[postgres(name = "V1")]
    V1 = 0,
    ///
    #[postgres(name = "V2")]
    V2 = 1,
    ///
    #[postgres(name = "V3")]
    V3 = 2,
}
#[derive(
    Debug,
    Clone,
    Copy,
    ToSql,
    FromSql,
    Serialize,
    Deserialize,
    FromPrimitive,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    EnumString,
    Display,
    Hash,
)]
#[postgres(name = "enum_service")]
pub enum EnumService {
    ///
    #[postgres(name = "auth")]
    Auth = 1,
    ///
    #[postgres(name = "user")]
    User = 2,
    ///
    #[postgres(name = "admin")]
    Admin = 3,
    ///
    #[postgres(name = "watcher")]
    Watcher = 4,
}
#[derive(
    Debug,
    Clone,
    Copy,
    ToSql,
    FromSql,
    Serialize,
    Deserialize,
    FromPrimitive,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    EnumString,
    Display,
    Hash,
)]
#[postgres(name = "enum_Endpoint")]
pub enum EnumEndpoint {
    ///
    #[postgres(name = "Login")]
    Login = 10020,
    ///
    #[postgres(name = "Signup")]
    Signup = 10010,
    ///
    #[postgres(name = "Authorize")]
    Authorize = 10030,
    ///
    #[postgres(name = "Logout")]
    Logout = 10040,
    ///
    #[postgres(name = "ChangeLoginWallet")]
    ChangeLoginWallet = 10050,
    ///
    #[postgres(name = "UserFollowStrategy")]
    UserFollowStrategy = 20040,
    ///
    #[postgres(name = "UserListFollowedStrategies")]
    UserListFollowedStrategies = 20050,
    ///
    #[postgres(name = "UserUnfollowStrategy")]
    UserUnfollowStrategy = 20060,
    ///
    #[postgres(name = "UserListStrategies")]
    UserListStrategies = 20061,
    ///
    #[postgres(name = "UserListTopPerformingStrategies")]
    UserListTopPerformingStrategies = 20063,
    ///
    #[postgres(name = "UserListStrategyBackers")]
    UserListStrategyBackers = 20064,
    ///
    #[postgres(name = "UserListStrategyFollowers")]
    UserListStrategyFollowers = 20065,
    ///
    #[postgres(name = "UserGetStrategy")]
    UserGetStrategy = 20062,
    ///
    #[postgres(name = "UserGetStrategyStatistics")]
    UserGetStrategyStatistics = 20070,
    ///
    #[postgres(name = "UserGetStrategiesStatistics")]
    UserGetStrategiesStatistics = 20071,
    ///
    #[postgres(name = "UserUpdateUserProfile")]
    UserUpdateUserProfile = 20172,
    ///
    #[postgres(name = "UserBackStrategy")]
    UserBackStrategy = 20080,
    ///
    #[postgres(name = "UserExitStrategy")]
    UserExitStrategy = 20110,
    ///
    #[postgres(name = "UserRequestRefund")]
    UserRequestRefund = 20081,
    ///
    #[postgres(name = "UserListBackedStrategies")]
    UserListBackedStrategies = 20090,
    ///
    #[postgres(name = "UserListBackStrategyLedger")]
    UserListBackStrategyLedger = 20100,
    ///
    #[postgres(name = "ExpertListBackStrategyLedger")]
    ExpertListBackStrategyLedger = 20101,
    ///
    #[postgres(name = "UserListExitStrategyLedger")]
    UserListExitStrategyLedger = 20120,
    ///
    #[postgres(name = "ExpertListExitStrategyLedger")]
    ExpertListExitStrategyLedger = 20121,
    ///
    #[postgres(name = "UserFollowExpert")]
    UserFollowExpert = 20130,
    ///
    #[postgres(name = "UserListFollowedExperts")]
    UserListFollowedExperts = 20140,
    ///
    #[postgres(name = "UserUnfollowExpert")]
    UserUnfollowExpert = 20150,
    ///
    #[postgres(name = "UserListExperts")]
    UserListExperts = 20160,
    ///
    #[postgres(name = "UserListTopPerformingExperts")]
    UserListTopPerformingExperts = 20161,
    ///
    #[postgres(name = "UserListFeaturedExperts")]
    UserListFeaturedExperts = 20162,
    ///
    #[postgres(name = "UserGetExpertProfile")]
    UserGetExpertProfile = 20170,
    ///
    #[postgres(name = "UserGetUserProfile")]
    UserGetUserProfile = 20180,
    ///
    #[postgres(name = "UserWhitelistWallet")]
    UserWhitelistWallet = 20190,
    ///
    #[postgres(name = "UserListWhitelistedWallets")]
    UserListWhitelistedWallets = 20200,
    ///
    #[postgres(name = "UserUnwhitelistWallet")]
    UserUnwhitelistWallet = 20210,
    ///
    #[postgres(name = "UserApplyBecomeExpert")]
    UserApplyBecomeExpert = 20220,
    ///
    #[postgres(name = "ExpertCreateStrategy")]
    ExpertCreateStrategy = 20250,
    ///
    #[postgres(name = "ExpertUpdateStrategy")]
    ExpertUpdateStrategy = 20260,
    ///
    #[postgres(name = "ExpertFreezeStrategy")]
    ExpertFreezeStrategy = 20265,
    ///
    #[postgres(name = "ExpertAddStrategyWatchingWallet")]
    ExpertAddStrategyWatchingWallet = 20270,
    ///
    #[postgres(name = "ExpertRemoveStrategyWatchingWallet")]
    ExpertRemoveStrategyWatchingWallet = 20280,
    ///
    #[postgres(name = "UserListStrategyWatchingWallets")]
    UserListStrategyWatchingWallets = 20290,
    ///
    #[postgres(name = "UserListWalletActivityLedger")]
    UserListWalletActivityLedger = 20300,
    ///
    #[postgres(name = "ExpertAddStrategyInitialTokenRatio")]
    ExpertAddStrategyInitialTokenRatio = 20310,
    ///
    #[postgres(name = "ExpertRemoveStrategyInitialTokenRatio")]
    ExpertRemoveStrategyInitialTokenRatio = 20320,
    ///
    #[postgres(name = "UserListStrategyInitialTokenRatio")]
    UserListStrategyInitialTokenRatio = 20330,
    ///
    #[postgres(name = "ExpertListFollowers")]
    ExpertListFollowers = 20340,
    ///
    #[postgres(name = "ExpertListBackers")]
    ExpertListBackers = 20350,
    ///
    #[postgres(name = "UserGetDepositTokens")]
    UserGetDepositTokens = 20360,
    ///
    #[postgres(name = "UserGetDepositAddresses")]
    UserGetDepositAddresses = 20370,
    ///
    #[postgres(name = "UserListDepositWithdrawLedger")]
    UserListDepositWithdrawLedger = 20380,
    ///
    #[postgres(name = "UserSubscribeDepositLedger")]
    UserSubscribeDepositLedger = 20381,
    ///
    #[postgres(name = "UserUnsubscribeDepositLedger")]
    UserUnsubscribeDepositLedger = 20382,
    ///
    #[postgres(name = "UserListStrategyWallets")]
    UserListStrategyWallets = 20390,
    ///
    #[postgres(name = "UserCreateStrategyWallet")]
    UserCreateStrategyWallet = 20391,
    ///
    #[postgres(name = "UserListStrategyAuditRules")]
    UserListStrategyAuditRules = 20400,
    ///
    #[postgres(name = "UserAddStrategyAuditRule")]
    UserAddStrategyAuditRule = 20410,
    ///
    #[postgres(name = "UserRemoveStrategyAuditRule")]
    UserRemoveStrategyAuditRule = 20420,
    ///
    #[postgres(name = "UserGetEscrowAddressForStrategy")]
    UserGetEscrowAddressForStrategy = 20500,
    ///
    #[postgres(name = "UserListDepositWithdrawBalances")]
    UserListDepositWithdrawBalances = 20510,
    ///
    #[postgres(name = "UserGetDepositWithdrawBalance")]
    UserGetDepositWithdrawBalance = 20511,
    ///
    #[postgres(name = "UserListEscrowTokenContractAddresses")]
    UserListEscrowTokenContractAddresses = 20520,
    ///
    #[postgres(name = "UserListStrategyTokenBalance")]
    UserListStrategyTokenBalance = 20530,
    ///
    #[postgres(name = "UserGetBackStrategyReviewDetail")]
    UserGetBackStrategyReviewDetail = 20540,
    ///
    #[postgres(name = "UserListUserBackStrategyAttempt")]
    UserListUserBackStrategyAttempt = 20550,
    ///
    #[postgres(name = "UserListUserBackStrategyLog")]
    UserListUserBackStrategyLog = 20560,
    ///
    #[postgres(name = "UserGetSystemConfig")]
    UserGetSystemConfig = 20570,
    ///
    #[postgres(name = "AdminListUsers")]
    AdminListUsers = 30010,
    ///
    #[postgres(name = "AdminSetUserRole")]
    AdminSetUserRole = 30020,
    ///
    #[postgres(name = "AdminSetBlockUser")]
    AdminSetBlockUser = 30030,
    ///
    #[postgres(name = "AdminListPendingExpertApplications")]
    AdminListPendingExpertApplications = 30060,
    ///
    #[postgres(name = "AdminApproveUserBecomeExpert")]
    AdminApproveUserBecomeExpert = 30040,
    ///
    #[postgres(name = "AdminRejectUserBecomeExpert")]
    AdminRejectUserBecomeExpert = 30050,
    ///
    #[postgres(name = "AdminGetSystemConfig")]
    AdminGetSystemConfig = 30070,
    ///
    #[postgres(name = "AdminUpdateSystemConfig")]
    AdminUpdateSystemConfig = 30080,
    ///
    #[postgres(name = "AdminListExperts")]
    AdminListExperts = 30090,
    ///
    #[postgres(name = "AdminListBackers")]
    AdminListBackers = 30100,
    ///
    #[postgres(name = "AdminListStrategies")]
    AdminListStrategies = 30110,
    ///
    #[postgres(name = "AdminApproveStrategy")]
    AdminApproveStrategy = 30120,
    ///
    #[postgres(name = "AdminRejectStrategy")]
    AdminRejectStrategy = 30130,
    ///
    #[postgres(name = "AdminAddAuditRule")]
    AdminAddAuditRule = 31002,
    ///
    #[postgres(name = "AdminNotifyEscrowLedgerChange")]
    AdminNotifyEscrowLedgerChange = 32010,
    ///
    #[postgres(name = "AdminSubscribeDepositLedger")]
    AdminSubscribeDepositLedger = 32011,
    ///
    #[postgres(name = "AdminUnsubscribeDepositLedger")]
    AdminUnsubscribeDepositLedger = 32012,
    ///
    #[postgres(name = "AdminAddEscrowTokenContractAddress")]
    AdminAddEscrowTokenContractAddress = 32020,
    ///
    #[postgres(name = "AdminAddEscrowContractAddress")]
    AdminAddEscrowContractAddress = 32030,
    ///
    #[postgres(name = "AdminListBackStrategyLedger")]
    AdminListBackStrategyLedger = 32040,
    ///
    #[postgres(name = "AdminListExitStrategyLedger")]
    AdminListExitStrategyLedger = 32041,
    ///
    #[postgres(name = "AdminSetBlockchainLogger")]
    AdminSetBlockchainLogger = 32050,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorBadRequest {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInternalServerError {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorNotImplemented {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorNotFound {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDatabaseError {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidService {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserForbidden {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserNotFound {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserMustAgreeTos {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserMustAgreePrivacyPolicy {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserNoAuthToken {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserInvalidAuthToken {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorTokenNotTop25 {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorImmutableStrategy {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserWhitelistedWalletNotSameNetworkAsStrategy {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDuplicateRequest {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidEnumLevel {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorError {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidArgument {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidState {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidSeq {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidMethod {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorProtocolViolation {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMalformedRequest {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUnknownUser {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorBlockedUser {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidPassword {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidToken {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorTemporarilyUnavailable {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUnexpectedException {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorBackPressureIncreased {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidPublicId {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidRange {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorBankAccountAlreadyExists {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInsufficientFunds {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorLogicalError {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorRestrictedUserPrivileges {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorIdenticalReplacement {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidRecoveryQuestions {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidRole {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorWrongRecoveryAnswers {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessageNotDelivered {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorNoReply {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorNullAttribute {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorConsentMissing {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorActiveSubscriptionRequired {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUsernameAlreadyRegistered {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorRecoveryQuestionsNotSet {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMustSubmitAllRecoveryQuestions {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidRecoveryToken {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorRoutingError {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUnauthorizedMessage {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorAuthError {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInternalError {}
#[derive(
    Debug,
    Clone,
    Copy,
    ToSql,
    FromSql,
    Serialize,
    Deserialize,
    FromPrimitive,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    EnumString,
    Display,
    Hash,
)]
#[postgres(name = "enum_ErrorCode")]
pub enum EnumErrorCode {
    /// Custom Bad Request
    #[postgres(name = "BadRequest")]
    BadRequest = 100400,
    /// Custom Internal Server Error
    #[postgres(name = "InternalServerError")]
    InternalServerError = 100500,
    /// Custom Method not implemented
    #[postgres(name = "NotImplemented")]
    NotImplemented = 100501,
    /// Custom NotFoundResource
    #[postgres(name = "NotFound")]
    NotFound = 100404,
    /// Custom Database error
    #[postgres(name = "DatabaseError")]
    DatabaseError = 100601,
    /// Custom Invalid Service
    #[postgres(name = "InvalidService")]
    InvalidService = 100602,
    /// Custom Insufficient role for user
    #[postgres(name = "UserForbidden")]
    UserForbidden = 101403,
    /// Custom User not found
    #[postgres(name = "UserNotFound")]
    UserNotFound = 101404,
    /// Custom Must agree to the terms of service
    #[postgres(name = "UserMustAgreeTos")]
    UserMustAgreeTos = 101601,
    /// Custom Must agree to the privacy policy
    #[postgres(name = "UserMustAgreePrivacyPolicy")]
    UserMustAgreePrivacyPolicy = 101602,
    /// Custom No auth token
    #[postgres(name = "UserNoAuthToken")]
    UserNoAuthToken = 101604,
    /// Custom token invalid
    #[postgres(name = "UserInvalidAuthToken")]
    UserInvalidAuthToken = 101605,
    /// Audit Token is not top 25
    #[postgres(name = "TokenNotTop25")]
    TokenNotTop25 = 102602,
    /// Audit Strategy is immutable
    #[postgres(name = "ImmutableStrategy")]
    ImmutableStrategy = 102603,
    /// Audit User whitelisted wallet not same network as strategy
    #[postgres(name = "UserWhitelistedWalletNotSameNetworkAsStrategy")]
    UserWhitelistedWalletNotSameNetworkAsStrategy = 102604,
    /// Custom Duplicate request
    #[postgres(name = "DuplicateRequest")]
    DuplicateRequest = 103001,
    /// SQL 22P02 InvalidEnumLevel
    #[postgres(name = "InvalidEnumLevel")]
    InvalidEnumLevel = 3484946,
    /// SQL R0000 Error
    #[postgres(name = "Error")]
    Error = 4349632,
    /// SQL R0001 InvalidArgument
    #[postgres(name = "InvalidArgument")]
    InvalidArgument = 45349633,
    /// SQL R0002 InvalidState
    #[postgres(name = "InvalidState")]
    InvalidState = 45349634,
    /// SQL R0003 InvalidSeq
    #[postgres(name = "InvalidSeq")]
    InvalidSeq = 45349635,
    /// SQL R0004 InvalidMethod
    #[postgres(name = "InvalidMethod")]
    InvalidMethod = 45349636,
    /// SQL R0005 ProtocolViolation
    #[postgres(name = "ProtocolViolation")]
    ProtocolViolation = 45349637,
    /// SQL R0006 MalformedRequest
    #[postgres(name = "MalformedRequest")]
    MalformedRequest = 45349638,
    /// SQL R0007 UnknownUser
    #[postgres(name = "UnknownUser")]
    UnknownUser = 45349639,
    /// SQL R0008 BlockedUser
    #[postgres(name = "BlockedUser")]
    BlockedUser = 45349640,
    /// SQL R0009 InvalidPassword
    #[postgres(name = "InvalidPassword")]
    InvalidPassword = 45349641,
    /// SQL R000A InvalidToken
    #[postgres(name = "InvalidToken")]
    InvalidToken = 45349642,
    /// SQL R000B TemporarilyUnavailable
    #[postgres(name = "TemporarilyUnavailable")]
    TemporarilyUnavailable = 45349643,
    /// SQL R000C UnexpectedException
    #[postgres(name = "UnexpectedException")]
    UnexpectedException = 45349644,
    /// SQL R000D BackPressureIncreased
    #[postgres(name = "BackPressureIncreased")]
    BackPressureIncreased = 45349645,
    /// SQL R000E InvalidPublicId
    #[postgres(name = "InvalidPublicId")]
    InvalidPublicId = 45349646,
    /// SQL R000F InvalidRange
    #[postgres(name = "InvalidRange")]
    InvalidRange = 45349647,
    /// SQL R000G BankAccountAlreadyExists
    #[postgres(name = "BankAccountAlreadyExists")]
    BankAccountAlreadyExists = 45349648,
    /// SQL R000H InsufficientFunds
    #[postgres(name = "InsufficientFunds")]
    InsufficientFunds = 45349649,
    /// SQL R000M LogicalError
    #[postgres(name = "LogicalError")]
    LogicalError = 45349654,
    /// SQL R000N RestrictedUserPrivileges
    #[postgres(name = "RestrictedUserPrivileges")]
    RestrictedUserPrivileges = 45349655,
    /// SQL R000O IdenticalReplacement
    #[postgres(name = "IdenticalReplacement")]
    IdenticalReplacement = 45349656,
    /// SQL R000R InvalidRecoveryQuestions
    #[postgres(name = "InvalidRecoveryQuestions")]
    InvalidRecoveryQuestions = 45349659,
    /// SQL R000S InvalidRole
    #[postgres(name = "InvalidRole")]
    InvalidRole = 45349660,
    /// SQL R000T WrongRecoveryAnswers
    #[postgres(name = "WrongRecoveryAnswers")]
    WrongRecoveryAnswers = 45349661,
    /// SQL R000U MessageNotDelivered
    #[postgres(name = "MessageNotDelivered")]
    MessageNotDelivered = 45349662,
    /// SQL R000V NoReply
    #[postgres(name = "NoReply")]
    NoReply = 45349663,
    /// SQL R000W NullAttribute
    #[postgres(name = "NullAttribute")]
    NullAttribute = 45349664,
    /// SQL R000X ConsentMissing
    #[postgres(name = "ConsentMissing")]
    ConsentMissing = 45349665,
    /// SQL R000Y ActiveSubscriptionRequired
    #[postgres(name = "ActiveSubscriptionRequired")]
    ActiveSubscriptionRequired = 45349666,
    /// SQL R000Z UsernameAlreadyRegistered
    #[postgres(name = "UsernameAlreadyRegistered")]
    UsernameAlreadyRegistered = 45349667,
    /// SQL R0010 RecoveryQuestionsNotSet
    #[postgres(name = "RecoveryQuestionsNotSet")]
    RecoveryQuestionsNotSet = 45349668,
    /// SQL R0011 MustSubmitAllRecoveryQuestions
    #[postgres(name = "MustSubmitAllRecoveryQuestions")]
    MustSubmitAllRecoveryQuestions = 45349669,
    /// SQL R0012 InvalidRecoveryToken
    #[postgres(name = "InvalidRecoveryToken")]
    InvalidRecoveryToken = 45349670,
    /// SQL R0018 RoutingError
    #[postgres(name = "RoutingError")]
    RoutingError = 45349676,
    /// SQL R0019 UnauthorizedMessage
    #[postgres(name = "UnauthorizedMessage")]
    UnauthorizedMessage = 45349677,
    /// SQL R001B AuthError
    #[postgres(name = "AuthError")]
    AuthError = 45349679,
    /// SQL R001G InternalError
    #[postgres(name = "InternalError")]
    InternalError = 45349684,
}

impl Into<ErrorCode> for EnumErrorCode {
    fn into(self) -> ErrorCode {
        ErrorCode::new(self as _)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminAddAuditRuleRequest {
    pub rule_id: i64,
    pub name: String,
    pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminAddAuditRuleResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminAddEscrowContractAddressRequest {
    pub pkey_id: i64,
    #[serde(with = "WithBlockchainAddress")]
    pub address: Address,
    pub blockchain: EnumBlockChain,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminAddEscrowContractAddressResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminAddEscrowTokenContractAddressRequest {
    pub pkey_id: i64,
    pub symbol: String,
    pub short_name: String,
    pub description: String,
    #[serde(with = "WithBlockchainAddress")]
    pub address: Address,
    pub blockchain: EnumBlockChain,
    pub is_stablecoin: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminAddEscrowTokenContractAddressResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminApproveStrategyRequest {
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminApproveStrategyResponse {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminApproveUserBecomeExpertRequest {
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminApproveUserBecomeExpertResponse {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminBackStrategyLedgerRow {
    pub back_ledger_id: i64,
    pub user_id: i64,
    pub strategy_id: i64,
    #[serde(with = "WithBlockchainDecimal")]
    pub quantity: U256,
    pub blockchain: EnumBlockChain,
    #[serde(with = "WithBlockchainTransactionHash")]
    pub transaction_hash: H256,
    pub happened_at: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminExitStrategyLedgerRow {
    pub back_ledger_id: i64,
    pub user_id: i64,
    pub strategy_id: i64,
    #[serde(with = "WithBlockchainDecimal")]
    pub quantity: U256,
    pub blockchain: EnumBlockChain,
    #[serde(with = "WithBlockchainTransactionHash")]
    pub transaction_hash: H256,
    pub happened_at: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminGetSystemConfigRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminGetSystemConfigResponse {
    pub platform_fee: f64,
    pub config_placeholder_2: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminListBackStrategyLedgerRequest {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
    #[serde(default)]
    pub strategy_id: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminListBackStrategyLedgerResponse {
    pub back_ledger_total: i64,
    pub back_ledger: Vec<AdminBackStrategyLedgerRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminListBackersRequest {
    #[serde(default)]
    pub offset: Option<i64>,
    #[serde(default)]
    pub limit: Option<i64>,
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminListBackersResponse {
    pub backers_total: i64,
    pub backers: Vec<AdminListBackersRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminListBackersRow {
    pub username: String,
    pub user_id: i64,
    #[serde(with = "WithBlockchainAddress")]
    pub login_wallet_address: Address,
    pub joined_at: i64,
    pub total_platform_fee_paid: f64,
    pub total_strategy_fee_paid: f64,
    pub total_backing_amount: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminListExitStrategyLedgerRequest {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
    #[serde(default)]
    pub strategy_id: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminListExitStrategyLedgerResponse {
    pub exit_ledger_total: i64,
    pub exit_ledger: Vec<AdminExitStrategyLedgerRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminListExpertsRequest {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminListExpertsResponse {
    pub experts_total: i64,
    pub experts: Vec<ListExpertsRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminListPendingExpertApplicationsRequest {
    #[serde(default)]
    pub offset: Option<i64>,
    #[serde(default)]
    pub limit: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminListPendingExpertApplicationsResponse {
    pub users_total: i64,
    pub users: Vec<ListPendingExpertApplicationsRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminListStrategiesRequest {
    #[serde(default)]
    pub offset: Option<i64>,
    #[serde(default)]
    pub limit: Option<i64>,
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
    pub pending_approval: Option<bool>,
    #[serde(default)]
    pub approved: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminListStrategiesResponse {
    pub strategies_total: i64,
    pub strategies: Vec<ListStrategiesRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminListUsersRequest {
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminListUsersResponse {
    pub users_total: i64,
    pub users: Vec<ListUserRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminNotifyEscrowLedgerChangeRequest {
    pub pkey_id: i64,
    pub user_id: i64,
    pub balance: UserListDepositLedgerRow,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminNotifyEscrowLedgerChangeResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminRejectStrategyRequest {
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminRejectStrategyResponse {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminRejectUserBecomeExpertRequest {
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminRejectUserBecomeExpertResponse {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminSetBlockUserRequest {
    pub user_id: i64,
    pub blocked: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminSetBlockUserResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminSetBlockchainLoggerRequest {
    pub enabled: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminSetBlockchainLoggerResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminSetUserRoleRequest {
    pub user_id: i64,
    pub role: EnumRole,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminSetUserRoleResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminSubscribeDepositLedgerRequest {
    #[serde(default)]
    pub initial_data: Option<i64>,
    #[serde(default)]
    pub blockchain: Option<EnumBlockChain>,
    #[serde(default)]
    pub mock_data: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminSubscribeDepositLedgerResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminUnsubscribeDepositLedgerRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminUnsubscribeDepositLedgerResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminUpdateSystemConfigRequest {
    #[serde(default)]
    pub platform_fee: Option<f64>,
    #[serde(default)]
    pub config_placeholder_2: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminUpdateSystemConfigResponse {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AumLedgerRow {
    pub aum_ledger_id: i64,
    pub base_token: String,
    pub quote_token: String,
    pub blockchain: EnumBlockChain,
    pub dex: String,
    pub action: String,
    #[serde(with = "WithBlockchainAddress")]
    pub wallet_address: Address,
    pub price: f64,
    pub current_price: f64,
    #[serde(with = "WithBlockchainDecimal")]
    pub quantity: U256,
    pub yield_7d: f64,
    pub yield_30d: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeRequest {
    pub address: String,
    pub token: uuid::Uuid,
    pub service: EnumService,
    pub device_id: String,
    pub device_os: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeResponse {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BackLedgerPoint {
    pub time: i64,
    pub backer_count: f64,
    pub backer_quantity_usd: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BackStrategyLedgerRow {
    pub back_ledger_id: i64,
    pub strategy_id: i64,
    #[serde(with = "WithBlockchainDecimal")]
    pub quantity: U256,
    pub blockchain: EnumBlockChain,
    #[serde(with = "WithBlockchainTransactionHash")]
    pub transaction_hash: H256,
    pub happened_at: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLoginWalletRequest {
    pub old_address: String,
    pub old_signature_text: String,
    pub old_signature: String,
    pub new_address: String,
    pub new_signature_text: String,
    pub new_signature: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLoginWalletResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EstimatedBackedTokenRatios {
    pub token_id: i64,
    pub token_name: String,
    #[serde(with = "WithBlockchainDecimal")]
    pub back_amount: U256,
    #[serde(with = "WithBlockchainDecimal")]
    pub back_value_in_usd: U256,
    pub back_value_ratio: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExitStrategyLedgerRow {
    pub exit_ledger_id: i64,
    pub strategy_id: i64,
    #[serde(with = "WithBlockchainDecimal")]
    pub quantity: U256,
    pub blockchain: EnumBlockChain,
    #[serde(with = "WithBlockchainTransactionHash")]
    pub transaction_hash: H256,
    pub happened_at: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertAddStrategyInitialTokenRatioRequest {
    pub strategy_id: i64,
    pub token_id: i64,
    #[serde(with = "WithBlockchainDecimal")]
    pub quantity: U256,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertAddStrategyInitialTokenRatioResponse {
    pub success: bool,
    pub token_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertAddStrategyWatchingWalletRequest {
    pub strategy_id: i64,
    pub blockchain: EnumBlockChain,
    #[serde(with = "WithBlockchainAddress")]
    pub wallet_address: Address,
    pub ratio: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertAddStrategyWatchingWalletResponse {
    pub success: bool,
    pub wallet_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertCreateStrategyRequest {
    pub name: String,
    pub description: String,
    pub strategy_thesis_url: String,
    #[serde(default)]
    pub minimum_backing_amount_usd: Option<f64>,
    pub expert_fee: f64,
    pub agreed_tos: bool,
    #[serde(with = "WithBlockchainAddress")]
    pub wallet_address: Address,
    pub wallet_blockchain: EnumBlockChain,
    #[serde(default)]
    pub strategy_token_relative_to_usdc_ratio: Option<U256>,
    pub initial_tokens: Vec<UserCreateStrategyInitialTokenRow>,
    #[serde(default)]
    pub audit_rules: Option<Vec<i64>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertCreateStrategyResponse {
    pub success: bool,
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertFreezeStrategyRequest {
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertFreezeStrategyResponse {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertListBackStrategyLedgerRequest {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
    #[serde(default)]
    pub strategy_id: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertListBackStrategyLedgerResponse {
    pub back_ledger_total: i64,
    pub back_ledger: Vec<BackStrategyLedgerRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertListBackersRequest {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertListBackersResponse {
    pub backers_total: i64,
    pub backers: Vec<ExpertListBackersRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertListBackersRow {
    pub public_id: i64,
    pub username: String,
    #[serde(default)]
    pub family_name: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
    pub backed_at: i64,
    pub joined_at: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertListExitStrategyLedgerRequest {
    #[serde(default)]
    pub strategy_id: Option<i64>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertListExitStrategyLedgerResponse {
    pub exit_ledger_total: i64,
    pub exit_ledger: Vec<ExitStrategyLedgerRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertListFollowersRequest {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertListFollowersResponse {
    pub followers_total: i64,
    pub followers: Vec<ExpertListFollowersRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertListFollowersRow {
    pub public_id: i64,
    pub username: String,
    #[serde(default)]
    pub family_name: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
    pub followed_at: i64,
    pub joined_at: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertRemoveStrategyInitialTokenRatioRequest {
    pub strategy_id: i64,
    pub token_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertRemoveStrategyInitialTokenRatioResponse {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertRemoveStrategyWatchingWalletRequest {
    pub strategy_id: i64,
    pub wallet_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertRemoveStrategyWatchingWalletResponse {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertUpdateStrategyRequest {
    pub strategy_id: i64,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub social_media: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpertUpdateStrategyResponse {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FollowLedgerPoint {
    pub time: i64,
    pub follower_count: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListExpertsRow {
    pub expert_id: i64,
    pub user_public_id: i64,
    #[serde(with = "WithBlockchainAddress")]
    pub linked_wallet: Address,
    pub name: String,
    #[serde(default)]
    pub family_name: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
    pub follower_count: i64,
    pub backer_count: i64,
    pub description: String,
    pub social_media: String,
    pub risk_score: f64,
    pub reputation_score: f64,
    pub consistent_score: f64,
    pub aum: f64,
    pub joined_at: i64,
    pub requested_at: i64,
    #[serde(default)]
    pub approved_at: Option<i64>,
    pub pending_expert: bool,
    pub approved_expert: bool,
    pub followed: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListPendingExpertApplicationsRow {
    pub user_id: i64,
    pub name: String,
    #[serde(with = "WithBlockchainAddress")]
    pub linked_wallet: Address,
    pub joined_at: i64,
    pub requested_at: i64,
    pub follower_count: i32,
    pub description: String,
    pub social_media: String,
    pub risk_score: f64,
    pub reputation_score: f64,
    pub aum: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListStrategiesRow {
    pub strategy_id: i64,
    pub strategy_name: String,
    pub strategy_description: String,
    pub followers: i32,
    pub backers: i32,
    pub aum: f64,
    pub followed: bool,
    pub swap_price: f64,
    pub price_change: f64,
    #[serde(default)]
    pub strategy_pool_address: Option<Address>,
    pub approved: bool,
    #[serde(default)]
    pub approved_at: Option<i64>,
    pub blockchain: EnumBlockChain,
    #[serde(default)]
    pub requested_at: Option<i64>,
    pub created_at: i64,
    pub expert_public_id: i64,
    pub expert_username: String,
    pub expert_family_name: String,
    pub expert_given_name: String,
    pub reputation: i32,
    pub risk_score: f64,
    pub strategy_pool_token: String,
    pub strategy_fee: f64,
    pub platform_fee: f64,
    pub expert_fee: f64,
    pub swap_fee: f64,
    pub total_fee: f64,
    pub number_of_tokens: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListStrategyBackersRow {
    pub user_id: i64,
    pub name: String,
    #[serde(with = "WithBlockchainAddress")]
    pub linked_wallet: Address,
    pub backed_date: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListStrategyFollowersRow {
    pub user_id: i64,
    pub name: String,
    #[serde(with = "WithBlockchainAddress")]
    pub linked_wallet: Address,
    pub followed_date: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListStrategyInitialTokenRatioRow {
    pub token_id: i64,
    pub token_name: String,
    #[serde(with = "WithBlockchainAddress")]
    pub token_address: Address,
    #[serde(with = "WithBlockchainDecimal")]
    pub quantity: U256,
    pub updated_at: i64,
    pub created_at: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListStrategyWatchingWalletsRow {
    pub wallet_id: i64,
    pub blockchain: EnumBlockChain,
    #[serde(with = "WithBlockchainAddress")]
    pub wallet_address: Address,
    pub ratio: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListUserRow {
    pub user_id: i64,
    pub public_user_id: i64,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(with = "WithBlockchainAddress")]
    pub address: Address,
    pub last_ip: std::net::IpAddr,
    pub last_login_at: i64,
    pub login_count: i32,
    pub role: EnumRole,
    #[serde(default)]
    pub email: Option<String>,
    pub updated_at: i64,
    pub created_at: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListWalletActivityLedgerRow {
    pub record_id: i64,
    #[serde(with = "WithBlockchainAddress")]
    pub wallet_address: Address,
    #[serde(with = "WithBlockchainTransactionHash")]
    pub transaction_hash: H256,
    pub dex: String,
    pub blockchain: EnumBlockChain,
    #[serde(with = "WithBlockchainAddress")]
    pub contract_address: Address,
    #[serde(with = "WithBlockchainAddress")]
    pub token_in_address: Address,
    #[serde(with = "WithBlockchainAddress")]
    pub token_out_address: Address,
    #[serde(with = "WithBlockchainAddress")]
    pub caller_address: Address,
    #[serde(with = "WithBlockchainDecimal")]
    pub amount_in: U256,
    #[serde(with = "WithBlockchainDecimal")]
    pub amount_out: U256,
    pub swap_calls: serde_json::Value,
    pub paths: serde_json::Value,
    pub dex_versions: serde_json::Value,
    pub created_at: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListWalletsRow {
    pub wallet_id: i64,
    pub blockchain: EnumBlockChain,
    #[serde(with = "WithBlockchainAddress")]
    pub wallet_address: Address,
    pub is_default: bool,
    pub is_compatible: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub address: String,
    pub signature_text: String,
    pub signature: String,
    pub service: EnumService,
    pub device_id: String,
    pub device_os: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub address: String,
    pub role: EnumRole,
    pub user_id: i64,
    pub user_token: uuid::Uuid,
    pub admin_token: uuid::Uuid,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LogoutRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LogoutResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetValuePoint {
    pub time: i64,
    pub net_value: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignupRequest {
    pub address: String,
    pub signature_text: String,
    pub signature: String,
    pub email: String,
    pub phone: String,
    pub agreed_tos: bool,
    pub agreed_privacy: bool,
    pub username: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignupResponse {
    pub address: String,
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserAddStrategyAuditRuleRequest {
    pub strategy_id: i64,
    pub rule_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserAddStrategyAuditRuleResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserAllowedEscrowTransferInfo {
    #[serde(with = "WithBlockchainAddress")]
    pub receiver_address: Address,
    pub blockchain: EnumBlockChain,
    pub token_id: i64,
    pub token_symbol: String,
    pub token_name: String,
    #[serde(with = "WithBlockchainAddress")]
    pub token_address: Address,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserApplyBecomeExpertRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserApplyBecomeExpertResponse {
    pub success: bool,
    pub expert_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserBackStrategyAttempt {
    pub attempt_id: i64,
    pub strategy_id: i64,
    pub strategy_name: String,
    pub token_id: i64,
    pub token_symbol: String,
    pub token_name: String,
    #[serde(with = "WithBlockchainDecimal")]
    pub quantity: U256,
    pub happened_at: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserBackStrategyLog {
    pub pkey_id: i64,
    pub message: String,
    pub happened_at: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserBackStrategyRequest {
    pub strategy_id: i64,
    #[serde(with = "WithBlockchainDecimal")]
    pub quantity: U256,
    pub token_id: i64,
    #[serde(default)]
    pub strategy_wallet: Option<Address>,
    pub nonce: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserBackStrategyResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserBackStrategyStreamResponse {
    pub end: bool,
    pub msg: String,
    #[serde(with = "WithBlockchainTransactionHash")]
    pub hash: H256,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserCreateStrategyInitialTokenRow {
    pub token_id: i64,
    #[serde(with = "WithBlockchainDecimal")]
    pub quantity: U256,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserCreateStrategyWalletRequest {
    pub blockchain: EnumBlockChain,
    #[serde(default)]
    pub user_managed_wallet_address: Option<Address>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserCreateStrategyWalletResponse {
    pub blockchain: EnumBlockChain,
    #[serde(with = "WithBlockchainAddress")]
    pub address: Address,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserExitStrategyRequest {
    pub strategy_id: i64,
    #[serde(with = "WithBlockchainDecimal")]
    pub quantity: U256,
    pub blockchain: EnumBlockChain,
    pub nonce: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserExitStrategyResponse {
    pub success: bool,
    #[serde(with = "WithBlockchainTransactionHash")]
    pub transaction_hash: H256,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserFollowExpertRequest {
    pub expert_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserFollowExpertResponse {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserFollowStrategyRequest {
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserFollowStrategyResponse {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetBackStrategyReviewDetailRequest {
    pub strategy_id: i64,
    pub token_id: i64,
    #[serde(with = "WithBlockchainDecimal")]
    pub quantity: U256,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetBackStrategyReviewDetailResponse {
    #[serde(with = "WithBlockchainDecimal")]
    pub strategy_fee: U256,
    #[serde(with = "WithBlockchainDecimal")]
    pub total_amount_to_back: U256,
    #[serde(with = "WithBlockchainDecimal")]
    pub total_amount_to_back_after_fee: U256,
    pub user_strategy_wallets: Vec<UserStrategyWallet>,
    #[serde(with = "WithBlockchainDecimal")]
    pub estimated_amount_of_strategy_tokens: U256,
    pub estimated_backed_token_ratios: Vec<EstimatedBackedTokenRatios>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetDepositAddressesRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetDepositAddressesResponse {
    pub addresses: Vec<UserGetDepositAddressesRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetDepositAddressesRow {
    pub blockchain: EnumBlockChain,
    #[serde(with = "WithBlockchainAddress")]
    pub address: Address,
    pub short_name: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetDepositTokensRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetDepositTokensResponse {
    pub tokens: Vec<UserGetDepositTokensRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetDepositTokensRow {
    pub blockchain: EnumBlockChain,
    pub token: String,
    #[serde(with = "WithBlockchainAddress")]
    pub address: Address,
    pub short_name: String,
    pub icon_url: String,
    pub conversion: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetDepositWithdrawBalanceRequest {
    pub token_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetDepositWithdrawBalanceResponse {
    #[serde(with = "WithBlockchainDecimal")]
    pub balance: U256,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetEscrowAddressForStrategyRequest {
    pub strategy_id: i64,
    #[serde(default)]
    pub token_id: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetEscrowAddressForStrategyResponse {
    pub tokens: Vec<UserAllowedEscrowTransferInfo>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetExpertProfileRequest {
    pub expert_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetExpertProfileResponse {
    pub expert_id: i64,
    pub name: String,
    pub follower_count: i32,
    pub backers_count: i32,
    pub description: String,
    pub social_media: String,
    pub risk_score: f64,
    pub reputation_score: f64,
    pub aum: f64,
    pub followed: bool,
    pub strategies_total: i64,
    pub strategies: Vec<ListStrategiesRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetStrategiesStatisticsRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetStrategiesStatisticsResponse {
    pub tracking_amount_usd: f64,
    pub backing_amount_usd: f64,
    pub difference_amount_usd: f64,
    pub aum_value_usd: f64,
    pub current_value_usd: f64,
    pub withdrawable_value_usd: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetStrategyRequest {
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetStrategyResponse {
    pub strategy: ListStrategiesRow,
    pub watching_wallets: Vec<WatchingWalletRow>,
    pub aum_ledger: Vec<AumLedgerRow>,
    pub audit_rules: Vec<UserListStrategyAuditRulesRow>,
    pub whitelisted_tokens: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetStrategyStatisticsRequest {
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetStrategyStatisticsResponse {
    pub strategy_id: i64,
    pub net_value: Vec<NetValuePoint>,
    pub follow_ledger: Vec<FollowLedgerPoint>,
    pub back_ledger: Vec<BackLedgerPoint>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetSystemConfigRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetSystemConfigResponse {
    pub platform_fee: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetUserProfileRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetUserProfileResponse {
    pub name: String,
    pub login_wallet: String,
    pub joined_at: i64,
    pub follower_count: i32,
    pub description: String,
    pub social_media: String,
    pub followed_experts: Vec<ListExpertsRow>,
    pub followed_strategies: Vec<ListStrategiesRow>,
    pub backed_strategies: Vec<ListStrategiesRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListBackStrategyLedgerRequest {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
    #[serde(default)]
    pub strategy_id: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListBackStrategyLedgerResponse {
    pub back_ledger_total: i64,
    pub back_ledger: Vec<BackStrategyLedgerRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListBackedStrategiesRequest {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListBackedStrategiesResponse {
    pub strategies_total: i64,
    pub strategies: Vec<ListStrategiesRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListDepositLedgerRow {
    pub transaction_id: i64,
    pub blockchain: EnumBlockChain,
    #[serde(with = "WithBlockchainAddress")]
    pub user_address: Address,
    #[serde(with = "WithBlockchainAddress")]
    pub contract_address: Address,
    #[serde(with = "WithBlockchainAddress")]
    pub receiver_address: Address,
    #[serde(with = "WithBlockchainDecimal")]
    pub quantity: U256,
    #[serde(with = "WithBlockchainTransactionHash")]
    pub transaction_hash: H256,
    pub is_deposit: bool,
    pub happened_at: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListDepositWithdrawBalance {
    pub blockchain: EnumBlockChain,
    pub token_id: i64,
    pub token_symbol: String,
    pub token_name: String,
    #[serde(with = "WithBlockchainDecimal")]
    pub balance: U256,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListDepositWithdrawBalancesRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListDepositWithdrawBalancesResponse {
    pub balances: Vec<UserListDepositWithdrawBalance>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListDepositWithdrawLedgerRequest {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
    #[serde(default)]
    pub blockchain: Option<EnumBlockChain>,
    #[serde(default)]
    pub id_deposit: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListDepositWithdrawLedgerResponse {
    pub ledger_total: i64,
    pub ledger: Vec<UserListDepositLedgerRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListEscrowTokenContractAddressesRequest {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
    #[serde(default)]
    pub blockchain: Option<EnumBlockChain>,
    #[serde(default)]
    pub is_stablecoin: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListEscrowTokenContractAddressesResponse {
    pub tokens_total: i64,
    pub tokens: Vec<UserListEscrowTokenContractAddressesRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListEscrowTokenContractAddressesRow {
    pub token_id: i64,
    pub token_symbol: String,
    pub token_name: String,
    #[serde(with = "WithBlockchainAddress")]
    pub token_address: Address,
    pub description: String,
    pub blockchain: EnumBlockChain,
    pub is_stablecoin: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListExitStrategyLedgerRequest {
    #[serde(default)]
    pub strategy_id: Option<i64>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListExitStrategyLedgerResponse {
    pub exit_ledger_total: i64,
    pub exit_ledger: Vec<ExitStrategyLedgerRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListExpertsRequest {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
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
    #[serde(default)]
    pub sort_by_followers: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListExpertsResponse {
    pub experts_total: i64,
    pub experts: Vec<ListExpertsRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListFeaturedExpertsRequest {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListFeaturedExpertsResponse {
    pub experts_total: i64,
    pub experts: Vec<ListExpertsRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListFollowedExpertsRequest {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListFollowedExpertsResponse {
    pub experts_total: i64,
    pub experts: Vec<ListExpertsRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListFollowedStrategiesRequest {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListFollowedStrategiesResponse {
    pub strategies_total: i64,
    pub strategies: Vec<ListStrategiesRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategiesRequest {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
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
    pub strategy_pool_address: Option<Address>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategiesResponse {
    pub strategies_total: i64,
    pub strategies: Vec<ListStrategiesRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategyAuditRulesRequest {
    #[serde(default)]
    pub strategy_id: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategyAuditRulesResponse {
    pub audit_rules: Vec<UserListStrategyAuditRulesRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategyAuditRulesRow {
    pub rule_id: i64,
    pub rule_name: String,
    pub rule_description: String,
    pub created_at: i64,
    pub enabled: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategyBackersRequest {
    pub strategy_id: i64,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategyBackersResponse {
    pub backers_total: i64,
    pub backers: Vec<ListStrategyBackersRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategyFollowersRequest {
    pub strategy_id: i64,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategyFollowersResponse {
    pub followers_total: i64,
    pub followers: Vec<ListStrategyFollowersRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategyInitialTokenRatioRequest {
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategyInitialTokenRatioResponse {
    pub token_ratios_total: i64,
    pub token_ratios: Vec<ListStrategyInitialTokenRatioRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategyTokenBalanceRequest {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
    #[serde(default)]
    pub strategy_id: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategyTokenBalanceResponse {
    pub tokens_total: i64,
    pub tokens: Vec<UserListStrategyTokenBalanceRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategyTokenBalanceRow {
    pub strategy_id: i64,
    pub strategy_name: String,
    #[serde(with = "WithBlockchainDecimal")]
    pub balance: U256,
    #[serde(with = "WithBlockchainAddress")]
    pub address: Address,
    pub blockchain: EnumBlockChain,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategyWalletsRequest {
    #[serde(default)]
    pub blockchain: Option<EnumBlockChain>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategyWalletsResponse {
    pub wallets_total: i64,
    pub wallets: Vec<UserListStrategyWalletsRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategyWalletsRow {
    pub blockchain: EnumBlockChain,
    #[serde(with = "WithBlockchainAddress")]
    pub address: Address,
    pub is_platform_managed: bool,
    pub created_at: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategyWatchingWalletsRequest {
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategyWatchingWalletsResponse {
    pub wallets_total: i64,
    pub wallets: Vec<ListStrategyWatchingWalletsRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListTopPerformingExpertsRequest {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListTopPerformingExpertsResponse {
    pub experts_total: i64,
    pub experts: Vec<ListExpertsRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListTopPerformingStrategiesRequest {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListTopPerformingStrategiesResponse {
    pub strategies_total: i64,
    pub strategies: Vec<ListStrategiesRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListUserBackStrategyAttemptRequest {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
    #[serde(default)]
    pub strategy_id: Option<i64>,
    #[serde(default)]
    pub token_id: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListUserBackStrategyAttemptResponse {
    pub total: i64,
    pub back_attempts: Vec<UserBackStrategyAttempt>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListUserBackStrategyLogRequest {
    pub attempt_id: i64,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListUserBackStrategyLogResponse {
    pub back_logs_total: i64,
    pub back_logs: Vec<UserBackStrategyLog>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListWalletActivityLedgerRequest {
    #[serde(with = "WithBlockchainAddress")]
    pub wallet_address: Address,
    pub blockchain: EnumBlockChain,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListWalletActivityLedgerResponse {
    pub wallet_activities_total: i64,
    pub wallet_activities: Vec<ListWalletActivityLedgerRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListWhitelistedWalletsRequest {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
    #[serde(default)]
    pub wallet_id: Option<i64>,
    #[serde(default)]
    pub blockchain: Option<EnumBlockChain>,
    #[serde(default)]
    pub wallet_address: Option<Address>,
    #[serde(default)]
    pub strategy_id: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListWhitelistedWalletsResponse {
    pub wallets: Vec<ListWalletsRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserRemoveStrategyAuditRuleRequest {
    pub strategy_id: i64,
    pub rule_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserRemoveStrategyAuditRuleResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserRequestRefundRequest {
    #[serde(with = "WithBlockchainDecimal")]
    pub quantity: U256,
    #[serde(with = "WithBlockchainAddress")]
    pub wallet_address: Address,
    pub blockchain: EnumBlockChain,
    pub nonce: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserRequestRefundResponse {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserStrategyWallet {
    pub wallet_id: i64,
    #[serde(with = "WithBlockchainAddress")]
    pub address: Address,
    pub blockchain: EnumBlockChain,
    pub is_platform_address: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserSubscribeDepositLedgerRequest {
    #[serde(default)]
    pub initial_data: Option<i64>,
    #[serde(default)]
    pub blockchain: Option<EnumBlockChain>,
    #[serde(default)]
    pub mock_data: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserSubscribeDepositLedgerResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserUnfollowExpertRequest {
    pub expert_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserUnfollowExpertResponse {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserUnfollowStrategyRequest {
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserUnfollowStrategyResponse {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserUnsubscribeDepositLedgerRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserUnsubscribeDepositLedgerResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserUnwhitelistWalletRequest {
    pub wallet_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserUnwhitelistWalletResponse {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateUserProfileRequest {
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateUserProfileResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserWhitelistWalletRequest {
    pub blockchain: EnumBlockChain,
    #[serde(with = "WithBlockchainAddress")]
    pub wallet_address: Address,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserWhitelistWalletResponse {
    pub success: bool,
    pub wallet_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WatchingWalletRow {
    pub watching_wallet_id: i64,
    #[serde(with = "WithBlockchainAddress")]
    pub wallet_address: Address,
    pub blockchain: EnumBlockChain,
    pub ratio_distribution: f64,
}
impl WsRequest for LoginRequest {
    type Response = LoginResponse;
    const METHOD_ID: u32 = 10020;
    const SCHEMA: &'static str = r#"{
  "name": "Login",
  "code": 10020,
  "parameters": [
    {
      "name": "address",
      "ty": "String"
    },
    {
      "name": "signature_text",
      "ty": "String"
    },
    {
      "name": "signature",
      "ty": "String"
    },
    {
      "name": "service",
      "ty": {
        "EnumRef": "service"
      }
    },
    {
      "name": "device_id",
      "ty": "String"
    },
    {
      "name": "device_os",
      "ty": "String"
    }
  ],
  "returns": [
    {
      "name": "address",
      "ty": "String"
    },
    {
      "name": "role",
      "ty": {
        "EnumRef": "role"
      }
    },
    {
      "name": "user_id",
      "ty": "BigInt"
    },
    {
      "name": "user_token",
      "ty": "UUID"
    },
    {
      "name": "admin_token",
      "ty": "UUID"
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for LoginResponse {
    type Request = LoginRequest;
}

impl WsRequest for SignupRequest {
    type Response = SignupResponse;
    const METHOD_ID: u32 = 10010;
    const SCHEMA: &'static str = r#"{
  "name": "Signup",
  "code": 10010,
  "parameters": [
    {
      "name": "address",
      "ty": "String"
    },
    {
      "name": "signature_text",
      "ty": "String"
    },
    {
      "name": "signature",
      "ty": "String"
    },
    {
      "name": "email",
      "ty": "String"
    },
    {
      "name": "phone",
      "ty": "String"
    },
    {
      "name": "agreed_tos",
      "ty": "Boolean"
    },
    {
      "name": "agreed_privacy",
      "ty": "Boolean"
    },
    {
      "name": "username",
      "ty": "String"
    }
  ],
  "returns": [
    {
      "name": "address",
      "ty": "String"
    },
    {
      "name": "user_id",
      "ty": "BigInt"
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for SignupResponse {
    type Request = SignupRequest;
}

impl WsRequest for AuthorizeRequest {
    type Response = AuthorizeResponse;
    const METHOD_ID: u32 = 10030;
    const SCHEMA: &'static str = r#"{
  "name": "Authorize",
  "code": 10030,
  "parameters": [
    {
      "name": "address",
      "ty": "String"
    },
    {
      "name": "token",
      "ty": "UUID"
    },
    {
      "name": "service",
      "ty": {
        "EnumRef": "service"
      }
    },
    {
      "name": "device_id",
      "ty": "String"
    },
    {
      "name": "device_os",
      "ty": "String"
    }
  ],
  "returns": [
    {
      "name": "success",
      "ty": "Boolean"
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for AuthorizeResponse {
    type Request = AuthorizeRequest;
}

impl WsRequest for LogoutRequest {
    type Response = LogoutResponse;
    const METHOD_ID: u32 = 10040;
    const SCHEMA: &'static str = r#"{
  "name": "Logout",
  "code": 10040,
  "parameters": [],
  "returns": [],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for LogoutResponse {
    type Request = LogoutRequest;
}

impl WsRequest for ChangeLoginWalletRequest {
    type Response = ChangeLoginWalletResponse;
    const METHOD_ID: u32 = 10050;
    const SCHEMA: &'static str = r#"{
  "name": "ChangeLoginWallet",
  "code": 10050,
  "parameters": [
    {
      "name": "old_address",
      "ty": "String"
    },
    {
      "name": "old_signature_text",
      "ty": "String"
    },
    {
      "name": "old_signature",
      "ty": "String"
    },
    {
      "name": "new_address",
      "ty": "String"
    },
    {
      "name": "new_signature_text",
      "ty": "String"
    },
    {
      "name": "new_signature",
      "ty": "String"
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for ChangeLoginWalletResponse {
    type Request = ChangeLoginWalletRequest;
}

impl WsRequest for UserFollowStrategyRequest {
    type Response = UserFollowStrategyResponse;
    const METHOD_ID: u32 = 20040;
    const SCHEMA: &'static str = r#"{
  "name": "UserFollowStrategy",
  "code": 20040,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": "BigInt"
    }
  ],
  "returns": [
    {
      "name": "success",
      "ty": "Boolean"
    }
  ],
  "stream_response": null,
  "description": "User follows a strategy",
  "json_schema": null
}"#;
}
impl WsResponse for UserFollowStrategyResponse {
    type Request = UserFollowStrategyRequest;
}

impl WsRequest for UserListFollowedStrategiesRequest {
    type Response = UserListFollowedStrategiesResponse;
    const METHOD_ID: u32 = 20050;
    const SCHEMA: &'static str = r#"{
  "name": "UserListFollowedStrategies",
  "code": 20050,
  "parameters": [
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "returns": [
    {
      "name": "strategies_total",
      "ty": "BigInt"
    },
    {
      "name": "strategies",
      "ty": {
        "Vec": {
          "Struct": {
            "name": "ListStrategiesRow",
            "fields": [
              {
                "name": "strategy_id",
                "ty": "BigInt"
              },
              {
                "name": "strategy_name",
                "ty": "String"
              },
              {
                "name": "strategy_description",
                "ty": "String"
              },
              {
                "name": "followers",
                "ty": "Int"
              },
              {
                "name": "backers",
                "ty": "Int"
              },
              {
                "name": "aum",
                "ty": "Numeric"
              },
              {
                "name": "followed",
                "ty": "Boolean"
              },
              {
                "name": "swap_price",
                "ty": "Numeric"
              },
              {
                "name": "price_change",
                "ty": "Numeric"
              },
              {
                "name": "strategy_pool_address",
                "ty": {
                  "Optional": "BlockchainAddress"
                }
              },
              {
                "name": "approved",
                "ty": "Boolean"
              },
              {
                "name": "approved_at",
                "ty": {
                  "Optional": "BigInt"
                }
              },
              {
                "name": "blockchain",
                "ty": {
                  "EnumRef": "block_chain"
                }
              },
              {
                "name": "requested_at",
                "ty": {
                  "Optional": "BigInt"
                }
              },
              {
                "name": "created_at",
                "ty": "BigInt"
              },
              {
                "name": "expert_public_id",
                "ty": "BigInt"
              },
              {
                "name": "expert_username",
                "ty": "String"
              },
              {
                "name": "expert_family_name",
                "ty": "String"
              },
              {
                "name": "expert_given_name",
                "ty": "String"
              },
              {
                "name": "reputation",
                "ty": "Int"
              },
              {
                "name": "risk_score",
                "ty": "Numeric"
              },
              {
                "name": "strategy_pool_token",
                "ty": "String"
              },
              {
                "name": "strategy_fee",
                "ty": "Numeric"
              },
              {
                "name": "platform_fee",
                "ty": "Numeric"
              },
              {
                "name": "expert_fee",
                "ty": "Numeric"
              },
              {
                "name": "swap_fee",
                "ty": "Numeric"
              },
              {
                "name": "total_fee",
                "ty": "Numeric"
              },
              {
                "name": "number_of_tokens",
                "ty": "BigInt"
              }
            ]
          }
        }
      }
    }
  ],
  "stream_response": null,
  "description": "User lists followed strategies",
  "json_schema": null
}"#;
}
impl WsResponse for UserListFollowedStrategiesResponse {
    type Request = UserListFollowedStrategiesRequest;
}

impl WsRequest for UserUnfollowStrategyRequest {
    type Response = UserUnfollowStrategyResponse;
    const METHOD_ID: u32 = 20060;
    const SCHEMA: &'static str = r#"{
  "name": "UserUnfollowStrategy",
  "code": 20060,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": "BigInt"
    }
  ],
  "returns": [
    {
      "name": "success",
      "ty": "Boolean"
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserUnfollowStrategyResponse {
    type Request = UserUnfollowStrategyRequest;
}

impl WsRequest for UserListStrategiesRequest {
    type Response = UserListStrategiesResponse;
    const METHOD_ID: u32 = 20061;
    const SCHEMA: &'static str = r#"{
  "name": "UserListStrategies",
  "code": 20061,
  "parameters": [
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "strategy_id",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "strategy_name",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "expert_public_id",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "expert_name",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "description",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "blockchain",
      "ty": {
        "Optional": {
          "EnumRef": "block_chain"
        }
      }
    },
    {
      "name": "strategy_pool_address",
      "ty": {
        "Optional": "BlockchainAddress"
      }
    }
  ],
  "returns": [
    {
      "name": "strategies_total",
      "ty": "BigInt"
    },
    {
      "name": "strategies",
      "ty": {
        "Vec": {
          "Struct": {
            "name": "ListStrategiesRow",
            "fields": [
              {
                "name": "strategy_id",
                "ty": "BigInt"
              },
              {
                "name": "strategy_name",
                "ty": "String"
              },
              {
                "name": "strategy_description",
                "ty": "String"
              },
              {
                "name": "followers",
                "ty": "Int"
              },
              {
                "name": "backers",
                "ty": "Int"
              },
              {
                "name": "aum",
                "ty": "Numeric"
              },
              {
                "name": "followed",
                "ty": "Boolean"
              },
              {
                "name": "swap_price",
                "ty": "Numeric"
              },
              {
                "name": "price_change",
                "ty": "Numeric"
              },
              {
                "name": "strategy_pool_address",
                "ty": {
                  "Optional": "BlockchainAddress"
                }
              },
              {
                "name": "approved",
                "ty": "Boolean"
              },
              {
                "name": "approved_at",
                "ty": {
                  "Optional": "BigInt"
                }
              },
              {
                "name": "blockchain",
                "ty": {
                  "EnumRef": "block_chain"
                }
              },
              {
                "name": "requested_at",
                "ty": {
                  "Optional": "BigInt"
                }
              },
              {
                "name": "created_at",
                "ty": "BigInt"
              },
              {
                "name": "expert_public_id",
                "ty": "BigInt"
              },
              {
                "name": "expert_username",
                "ty": "String"
              },
              {
                "name": "expert_family_name",
                "ty": "String"
              },
              {
                "name": "expert_given_name",
                "ty": "String"
              },
              {
                "name": "reputation",
                "ty": "Int"
              },
              {
                "name": "risk_score",
                "ty": "Numeric"
              },
              {
                "name": "strategy_pool_token",
                "ty": "String"
              },
              {
                "name": "strategy_fee",
                "ty": "Numeric"
              },
              {
                "name": "platform_fee",
                "ty": "Numeric"
              },
              {
                "name": "expert_fee",
                "ty": "Numeric"
              },
              {
                "name": "swap_fee",
                "ty": "Numeric"
              },
              {
                "name": "total_fee",
                "ty": "Numeric"
              },
              {
                "name": "number_of_tokens",
                "ty": "BigInt"
              }
            ]
          }
        }
      }
    }
  ],
  "stream_response": null,
  "description": "User lists strategies",
  "json_schema": null
}"#;
}
impl WsResponse for UserListStrategiesResponse {
    type Request = UserListStrategiesRequest;
}

impl WsRequest for UserListTopPerformingStrategiesRequest {
    type Response = UserListTopPerformingStrategiesResponse;
    const METHOD_ID: u32 = 20063;
    const SCHEMA: &'static str = r#"{
  "name": "UserListTopPerformingStrategies",
  "code": 20063,
  "parameters": [
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "returns": [
    {
      "name": "strategies_total",
      "ty": "BigInt"
    },
    {
      "name": "strategies",
      "ty": {
        "Vec": {
          "Struct": {
            "name": "ListStrategiesRow",
            "fields": [
              {
                "name": "strategy_id",
                "ty": "BigInt"
              },
              {
                "name": "strategy_name",
                "ty": "String"
              },
              {
                "name": "strategy_description",
                "ty": "String"
              },
              {
                "name": "followers",
                "ty": "Int"
              },
              {
                "name": "backers",
                "ty": "Int"
              },
              {
                "name": "aum",
                "ty": "Numeric"
              },
              {
                "name": "followed",
                "ty": "Boolean"
              },
              {
                "name": "swap_price",
                "ty": "Numeric"
              },
              {
                "name": "price_change",
                "ty": "Numeric"
              },
              {
                "name": "strategy_pool_address",
                "ty": {
                  "Optional": "BlockchainAddress"
                }
              },
              {
                "name": "approved",
                "ty": "Boolean"
              },
              {
                "name": "approved_at",
                "ty": {
                  "Optional": "BigInt"
                }
              },
              {
                "name": "blockchain",
                "ty": {
                  "EnumRef": "block_chain"
                }
              },
              {
                "name": "requested_at",
                "ty": {
                  "Optional": "BigInt"
                }
              },
              {
                "name": "created_at",
                "ty": "BigInt"
              },
              {
                "name": "expert_public_id",
                "ty": "BigInt"
              },
              {
                "name": "expert_username",
                "ty": "String"
              },
              {
                "name": "expert_family_name",
                "ty": "String"
              },
              {
                "name": "expert_given_name",
                "ty": "String"
              },
              {
                "name": "reputation",
                "ty": "Int"
              },
              {
                "name": "risk_score",
                "ty": "Numeric"
              },
              {
                "name": "strategy_pool_token",
                "ty": "String"
              },
              {
                "name": "strategy_fee",
                "ty": "Numeric"
              },
              {
                "name": "platform_fee",
                "ty": "Numeric"
              },
              {
                "name": "expert_fee",
                "ty": "Numeric"
              },
              {
                "name": "swap_fee",
                "ty": "Numeric"
              },
              {
                "name": "total_fee",
                "ty": "Numeric"
              },
              {
                "name": "number_of_tokens",
                "ty": "BigInt"
              }
            ]
          }
        }
      }
    }
  ],
  "stream_response": null,
  "description": "User lists top performing strategies",
  "json_schema": null
}"#;
}
impl WsResponse for UserListTopPerformingStrategiesResponse {
    type Request = UserListTopPerformingStrategiesRequest;
}

impl WsRequest for UserListStrategyBackersRequest {
    type Response = UserListStrategyBackersResponse;
    const METHOD_ID: u32 = 20064;
    const SCHEMA: &'static str = r#"{
  "name": "UserListStrategyBackers",
  "code": 20064,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": "BigInt"
    },
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "returns": [
    {
      "name": "backers_total",
      "ty": "BigInt"
    },
    {
      "name": "backers",
      "ty": {
        "DataTable": {
          "name": "ListStrategyBackersRow",
          "fields": [
            {
              "name": "user_id",
              "ty": "BigInt"
            },
            {
              "name": "name",
              "ty": "String"
            },
            {
              "name": "linked_wallet",
              "ty": "BlockchainAddress"
            },
            {
              "name": "backed_date",
              "ty": "BigInt"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserListStrategyBackersResponse {
    type Request = UserListStrategyBackersRequest;
}

impl WsRequest for UserListStrategyFollowersRequest {
    type Response = UserListStrategyFollowersResponse;
    const METHOD_ID: u32 = 20065;
    const SCHEMA: &'static str = r#"{
  "name": "UserListStrategyFollowers",
  "code": 20065,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": "BigInt"
    },
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "returns": [
    {
      "name": "followers_total",
      "ty": "BigInt"
    },
    {
      "name": "followers",
      "ty": {
        "DataTable": {
          "name": "ListStrategyFollowersRow",
          "fields": [
            {
              "name": "user_id",
              "ty": "BigInt"
            },
            {
              "name": "name",
              "ty": "String"
            },
            {
              "name": "linked_wallet",
              "ty": "BlockchainAddress"
            },
            {
              "name": "followed_date",
              "ty": "BigInt"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserListStrategyFollowersResponse {
    type Request = UserListStrategyFollowersRequest;
}

impl WsRequest for UserGetStrategyRequest {
    type Response = UserGetStrategyResponse;
    const METHOD_ID: u32 = 20062;
    const SCHEMA: &'static str = r#"{
  "name": "UserGetStrategy",
  "code": 20062,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": "BigInt"
    }
  ],
  "returns": [
    {
      "name": "strategy",
      "ty": {
        "Struct": {
          "name": "ListStrategiesRow",
          "fields": [
            {
              "name": "strategy_id",
              "ty": "BigInt"
            },
            {
              "name": "strategy_name",
              "ty": "String"
            },
            {
              "name": "strategy_description",
              "ty": "String"
            },
            {
              "name": "followers",
              "ty": "Int"
            },
            {
              "name": "backers",
              "ty": "Int"
            },
            {
              "name": "aum",
              "ty": "Numeric"
            },
            {
              "name": "followed",
              "ty": "Boolean"
            },
            {
              "name": "swap_price",
              "ty": "Numeric"
            },
            {
              "name": "price_change",
              "ty": "Numeric"
            },
            {
              "name": "strategy_pool_address",
              "ty": {
                "Optional": "BlockchainAddress"
              }
            },
            {
              "name": "approved",
              "ty": "Boolean"
            },
            {
              "name": "approved_at",
              "ty": {
                "Optional": "BigInt"
              }
            },
            {
              "name": "blockchain",
              "ty": {
                "EnumRef": "block_chain"
              }
            },
            {
              "name": "requested_at",
              "ty": {
                "Optional": "BigInt"
              }
            },
            {
              "name": "created_at",
              "ty": "BigInt"
            },
            {
              "name": "expert_public_id",
              "ty": "BigInt"
            },
            {
              "name": "expert_username",
              "ty": "String"
            },
            {
              "name": "expert_family_name",
              "ty": "String"
            },
            {
              "name": "expert_given_name",
              "ty": "String"
            },
            {
              "name": "reputation",
              "ty": "Int"
            },
            {
              "name": "risk_score",
              "ty": "Numeric"
            },
            {
              "name": "strategy_pool_token",
              "ty": "String"
            },
            {
              "name": "strategy_fee",
              "ty": "Numeric"
            },
            {
              "name": "platform_fee",
              "ty": "Numeric"
            },
            {
              "name": "expert_fee",
              "ty": "Numeric"
            },
            {
              "name": "swap_fee",
              "ty": "Numeric"
            },
            {
              "name": "total_fee",
              "ty": "Numeric"
            },
            {
              "name": "number_of_tokens",
              "ty": "BigInt"
            }
          ]
        }
      }
    },
    {
      "name": "watching_wallets",
      "ty": {
        "DataTable": {
          "name": "WatchingWalletRow",
          "fields": [
            {
              "name": "watching_wallet_id",
              "ty": "BigInt"
            },
            {
              "name": "wallet_address",
              "ty": "BlockchainAddress"
            },
            {
              "name": "blockchain",
              "ty": {
                "EnumRef": "block_chain"
              }
            },
            {
              "name": "ratio_distribution",
              "ty": "Numeric"
            }
          ]
        }
      }
    },
    {
      "name": "aum_ledger",
      "ty": {
        "DataTable": {
          "name": "AumLedgerRow",
          "fields": [
            {
              "name": "aum_ledger_id",
              "ty": "BigInt"
            },
            {
              "name": "base_token",
              "ty": "String"
            },
            {
              "name": "quote_token",
              "ty": "String"
            },
            {
              "name": "blockchain",
              "ty": {
                "EnumRef": "block_chain"
              }
            },
            {
              "name": "dex",
              "ty": "String"
            },
            {
              "name": "action",
              "ty": "String"
            },
            {
              "name": "wallet_address",
              "ty": "BlockchainAddress"
            },
            {
              "name": "price",
              "ty": "Numeric"
            },
            {
              "name": "current_price",
              "ty": "Numeric"
            },
            {
              "name": "quantity",
              "ty": "BlockchainDecimal"
            },
            {
              "name": "yield_7d",
              "ty": "Numeric"
            },
            {
              "name": "yield_30d",
              "ty": "Numeric"
            }
          ]
        }
      }
    },
    {
      "name": "audit_rules",
      "ty": {
        "DataTable": {
          "name": "UserListStrategyAuditRulesRow",
          "fields": [
            {
              "name": "rule_id",
              "ty": "BigInt"
            },
            {
              "name": "rule_name",
              "ty": "String"
            },
            {
              "name": "rule_description",
              "ty": "String"
            },
            {
              "name": "created_at",
              "ty": "BigInt"
            },
            {
              "name": "enabled",
              "ty": "Boolean"
            }
          ]
        }
      }
    },
    {
      "name": "whitelisted_tokens",
      "ty": {
        "Vec": "String"
      }
    }
  ],
  "stream_response": null,
  "description": "User gets a strategy",
  "json_schema": null
}"#;
}
impl WsResponse for UserGetStrategyResponse {
    type Request = UserGetStrategyRequest;
}

impl WsRequest for UserGetStrategyStatisticsRequest {
    type Response = UserGetStrategyStatisticsResponse;
    const METHOD_ID: u32 = 20070;
    const SCHEMA: &'static str = r#"{
  "name": "UserGetStrategyStatistics",
  "code": 20070,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": "BigInt"
    }
  ],
  "returns": [
    {
      "name": "strategy_id",
      "ty": "BigInt"
    },
    {
      "name": "net_value",
      "ty": {
        "DataTable": {
          "name": "NetValuePoint",
          "fields": [
            {
              "name": "time",
              "ty": "BigInt"
            },
            {
              "name": "net_value",
              "ty": "Numeric"
            }
          ]
        }
      }
    },
    {
      "name": "follow_ledger",
      "ty": {
        "DataTable": {
          "name": "FollowLedgerPoint",
          "fields": [
            {
              "name": "time",
              "ty": "BigInt"
            },
            {
              "name": "follower_count",
              "ty": "Numeric"
            }
          ]
        }
      }
    },
    {
      "name": "back_ledger",
      "ty": {
        "DataTable": {
          "name": "BackLedgerPoint",
          "fields": [
            {
              "name": "time",
              "ty": "BigInt"
            },
            {
              "name": "backer_count",
              "ty": "Numeric"
            },
            {
              "name": "backer_quantity_usd",
              "ty": "Numeric"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "User gets a strategy statistics",
  "json_schema": null
}"#;
}
impl WsResponse for UserGetStrategyStatisticsResponse {
    type Request = UserGetStrategyStatisticsRequest;
}

impl WsRequest for UserGetStrategiesStatisticsRequest {
    type Response = UserGetStrategiesStatisticsResponse;
    const METHOD_ID: u32 = 20071;
    const SCHEMA: &'static str = r#"{
  "name": "UserGetStrategiesStatistics",
  "code": 20071,
  "parameters": [],
  "returns": [
    {
      "name": "tracking_amount_usd",
      "ty": "Numeric"
    },
    {
      "name": "backing_amount_usd",
      "ty": "Numeric"
    },
    {
      "name": "difference_amount_usd",
      "ty": "Numeric"
    },
    {
      "name": "aum_value_usd",
      "ty": "Numeric"
    },
    {
      "name": "current_value_usd",
      "ty": "Numeric"
    },
    {
      "name": "withdrawable_value_usd",
      "ty": "Numeric"
    }
  ],
  "stream_response": null,
  "description": "User gets statistics of all strategies related to the user",
  "json_schema": null
}"#;
}
impl WsResponse for UserGetStrategiesStatisticsResponse {
    type Request = UserGetStrategiesStatisticsRequest;
}

impl WsRequest for UserUpdateUserProfileRequest {
    type Response = UserUpdateUserProfileResponse;
    const METHOD_ID: u32 = 20172;
    const SCHEMA: &'static str = r#"{
  "name": "UserUpdateUserProfile",
  "code": 20172,
  "parameters": [
    {
      "name": "username",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "family_name",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "given_name",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "description",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "social_media",
      "ty": {
        "Optional": "String"
      }
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "User update its expert profile",
  "json_schema": null
}"#;
}
impl WsResponse for UserUpdateUserProfileResponse {
    type Request = UserUpdateUserProfileRequest;
}

impl WsRequest for UserBackStrategyRequest {
    type Response = UserBackStrategyResponse;
    const METHOD_ID: u32 = 20080;
    const SCHEMA: &'static str = r#"{
  "name": "UserBackStrategy",
  "code": 20080,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": "BigInt"
    },
    {
      "name": "quantity",
      "ty": "BlockchainDecimal"
    },
    {
      "name": "token_id",
      "ty": "BigInt"
    },
    {
      "name": "strategy_wallet",
      "ty": {
        "Optional": "BlockchainAddress"
      }
    },
    {
      "name": "nonce",
      "ty": "BigInt"
    }
  ],
  "returns": [],
  "stream_response": {
    "Struct": {
      "name": "UserBackStrategyStreamResponse",
      "fields": [
        {
          "name": "end",
          "ty": "Boolean"
        },
        {
          "name": "msg",
          "ty": "String"
        },
        {
          "name": "hash",
          "ty": "BlockchainTransactionHash"
        }
      ]
    }
  },
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserBackStrategyResponse {
    type Request = UserBackStrategyRequest;
}

impl WsRequest for UserExitStrategyRequest {
    type Response = UserExitStrategyResponse;
    const METHOD_ID: u32 = 20110;
    const SCHEMA: &'static str = r#"{
  "name": "UserExitStrategy",
  "code": 20110,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": "BigInt"
    },
    {
      "name": "quantity",
      "ty": "BlockchainDecimal"
    },
    {
      "name": "blockchain",
      "ty": {
        "EnumRef": "block_chain"
      }
    },
    {
      "name": "nonce",
      "ty": "BigInt"
    }
  ],
  "returns": [
    {
      "name": "success",
      "ty": "Boolean"
    },
    {
      "name": "transaction_hash",
      "ty": "BlockchainTransactionHash"
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserExitStrategyResponse {
    type Request = UserExitStrategyRequest;
}

impl WsRequest for UserRequestRefundRequest {
    type Response = UserRequestRefundResponse;
    const METHOD_ID: u32 = 20081;
    const SCHEMA: &'static str = r#"{
  "name": "UserRequestRefund",
  "code": 20081,
  "parameters": [
    {
      "name": "quantity",
      "ty": "BlockchainDecimal"
    },
    {
      "name": "wallet_address",
      "ty": "BlockchainAddress"
    },
    {
      "name": "blockchain",
      "ty": {
        "EnumRef": "block_chain"
      }
    },
    {
      "name": "nonce",
      "ty": "BigInt"
    }
  ],
  "returns": [
    {
      "name": "success",
      "ty": "Boolean"
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserRequestRefundResponse {
    type Request = UserRequestRefundRequest;
}

impl WsRequest for UserListBackedStrategiesRequest {
    type Response = UserListBackedStrategiesResponse;
    const METHOD_ID: u32 = 20090;
    const SCHEMA: &'static str = r#"{
  "name": "UserListBackedStrategies",
  "code": 20090,
  "parameters": [
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "returns": [
    {
      "name": "strategies_total",
      "ty": "BigInt"
    },
    {
      "name": "strategies",
      "ty": {
        "Vec": {
          "Struct": {
            "name": "ListStrategiesRow",
            "fields": [
              {
                "name": "strategy_id",
                "ty": "BigInt"
              },
              {
                "name": "strategy_name",
                "ty": "String"
              },
              {
                "name": "strategy_description",
                "ty": "String"
              },
              {
                "name": "followers",
                "ty": "Int"
              },
              {
                "name": "backers",
                "ty": "Int"
              },
              {
                "name": "aum",
                "ty": "Numeric"
              },
              {
                "name": "followed",
                "ty": "Boolean"
              },
              {
                "name": "swap_price",
                "ty": "Numeric"
              },
              {
                "name": "price_change",
                "ty": "Numeric"
              },
              {
                "name": "strategy_pool_address",
                "ty": {
                  "Optional": "BlockchainAddress"
                }
              },
              {
                "name": "approved",
                "ty": "Boolean"
              },
              {
                "name": "approved_at",
                "ty": {
                  "Optional": "BigInt"
                }
              },
              {
                "name": "blockchain",
                "ty": {
                  "EnumRef": "block_chain"
                }
              },
              {
                "name": "requested_at",
                "ty": {
                  "Optional": "BigInt"
                }
              },
              {
                "name": "created_at",
                "ty": "BigInt"
              },
              {
                "name": "expert_public_id",
                "ty": "BigInt"
              },
              {
                "name": "expert_username",
                "ty": "String"
              },
              {
                "name": "expert_family_name",
                "ty": "String"
              },
              {
                "name": "expert_given_name",
                "ty": "String"
              },
              {
                "name": "reputation",
                "ty": "Int"
              },
              {
                "name": "risk_score",
                "ty": "Numeric"
              },
              {
                "name": "strategy_pool_token",
                "ty": "String"
              },
              {
                "name": "strategy_fee",
                "ty": "Numeric"
              },
              {
                "name": "platform_fee",
                "ty": "Numeric"
              },
              {
                "name": "expert_fee",
                "ty": "Numeric"
              },
              {
                "name": "swap_fee",
                "ty": "Numeric"
              },
              {
                "name": "total_fee",
                "ty": "Numeric"
              },
              {
                "name": "number_of_tokens",
                "ty": "BigInt"
              }
            ]
          }
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserListBackedStrategiesResponse {
    type Request = UserListBackedStrategiesRequest;
}

impl WsRequest for UserListBackStrategyLedgerRequest {
    type Response = UserListBackStrategyLedgerResponse;
    const METHOD_ID: u32 = 20100;
    const SCHEMA: &'static str = r#"{
  "name": "UserListBackStrategyLedger",
  "code": 20100,
  "parameters": [
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "strategy_id",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "returns": [
    {
      "name": "back_ledger_total",
      "ty": "BigInt"
    },
    {
      "name": "back_ledger",
      "ty": {
        "DataTable": {
          "name": "BackStrategyLedgerRow",
          "fields": [
            {
              "name": "back_ledger_id",
              "ty": "BigInt"
            },
            {
              "name": "strategy_id",
              "ty": "BigInt"
            },
            {
              "name": "quantity",
              "ty": "BlockchainDecimal"
            },
            {
              "name": "blockchain",
              "ty": {
                "EnumRef": "block_chain"
              }
            },
            {
              "name": "transaction_hash",
              "ty": "BlockchainTransactionHash"
            },
            {
              "name": "happened_at",
              "ty": "BigInt"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserListBackStrategyLedgerResponse {
    type Request = UserListBackStrategyLedgerRequest;
}

impl WsRequest for ExpertListBackStrategyLedgerRequest {
    type Response = ExpertListBackStrategyLedgerResponse;
    const METHOD_ID: u32 = 20101;
    const SCHEMA: &'static str = r#"{
  "name": "ExpertListBackStrategyLedger",
  "code": 20101,
  "parameters": [
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "strategy_id",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "returns": [
    {
      "name": "back_ledger_total",
      "ty": "BigInt"
    },
    {
      "name": "back_ledger",
      "ty": {
        "DataTable": {
          "name": "BackStrategyLedgerRow",
          "fields": [
            {
              "name": "back_ledger_id",
              "ty": "BigInt"
            },
            {
              "name": "strategy_id",
              "ty": "BigInt"
            },
            {
              "name": "quantity",
              "ty": "BlockchainDecimal"
            },
            {
              "name": "blockchain",
              "ty": {
                "EnumRef": "block_chain"
              }
            },
            {
              "name": "transaction_hash",
              "ty": "BlockchainTransactionHash"
            },
            {
              "name": "happened_at",
              "ty": "BigInt"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for ExpertListBackStrategyLedgerResponse {
    type Request = ExpertListBackStrategyLedgerRequest;
}

impl WsRequest for UserListExitStrategyLedgerRequest {
    type Response = UserListExitStrategyLedgerResponse;
    const METHOD_ID: u32 = 20120;
    const SCHEMA: &'static str = r#"{
  "name": "UserListExitStrategyLedger",
  "code": 20120,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "returns": [
    {
      "name": "exit_ledger_total",
      "ty": "BigInt"
    },
    {
      "name": "exit_ledger",
      "ty": {
        "DataTable": {
          "name": "ExitStrategyLedgerRow",
          "fields": [
            {
              "name": "exit_ledger_id",
              "ty": "BigInt"
            },
            {
              "name": "strategy_id",
              "ty": "BigInt"
            },
            {
              "name": "quantity",
              "ty": "BlockchainDecimal"
            },
            {
              "name": "blockchain",
              "ty": {
                "EnumRef": "block_chain"
              }
            },
            {
              "name": "transaction_hash",
              "ty": "BlockchainTransactionHash"
            },
            {
              "name": "happened_at",
              "ty": "BigInt"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserListExitStrategyLedgerResponse {
    type Request = UserListExitStrategyLedgerRequest;
}

impl WsRequest for ExpertListExitStrategyLedgerRequest {
    type Response = ExpertListExitStrategyLedgerResponse;
    const METHOD_ID: u32 = 20121;
    const SCHEMA: &'static str = r#"{
  "name": "ExpertListExitStrategyLedger",
  "code": 20121,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "returns": [
    {
      "name": "exit_ledger_total",
      "ty": "BigInt"
    },
    {
      "name": "exit_ledger",
      "ty": {
        "DataTable": {
          "name": "ExitStrategyLedgerRow",
          "fields": [
            {
              "name": "exit_ledger_id",
              "ty": "BigInt"
            },
            {
              "name": "strategy_id",
              "ty": "BigInt"
            },
            {
              "name": "quantity",
              "ty": "BlockchainDecimal"
            },
            {
              "name": "blockchain",
              "ty": {
                "EnumRef": "block_chain"
              }
            },
            {
              "name": "transaction_hash",
              "ty": "BlockchainTransactionHash"
            },
            {
              "name": "happened_at",
              "ty": "BigInt"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for ExpertListExitStrategyLedgerResponse {
    type Request = ExpertListExitStrategyLedgerRequest;
}

impl WsRequest for UserFollowExpertRequest {
    type Response = UserFollowExpertResponse;
    const METHOD_ID: u32 = 20130;
    const SCHEMA: &'static str = r#"{
  "name": "UserFollowExpert",
  "code": 20130,
  "parameters": [
    {
      "name": "expert_id",
      "ty": "BigInt"
    }
  ],
  "returns": [
    {
      "name": "success",
      "ty": "Boolean"
    }
  ],
  "stream_response": null,
  "description": "User follows an expert",
  "json_schema": null
}"#;
}
impl WsResponse for UserFollowExpertResponse {
    type Request = UserFollowExpertRequest;
}

impl WsRequest for UserListFollowedExpertsRequest {
    type Response = UserListFollowedExpertsResponse;
    const METHOD_ID: u32 = 20140;
    const SCHEMA: &'static str = r#"{
  "name": "UserListFollowedExperts",
  "code": 20140,
  "parameters": [
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "returns": [
    {
      "name": "experts_total",
      "ty": "BigInt"
    },
    {
      "name": "experts",
      "ty": {
        "Vec": {
          "Struct": {
            "name": "ListExpertsRow",
            "fields": [
              {
                "name": "expert_id",
                "ty": "BigInt"
              },
              {
                "name": "user_public_id",
                "ty": "BigInt"
              },
              {
                "name": "linked_wallet",
                "ty": "BlockchainAddress"
              },
              {
                "name": "name",
                "ty": "String"
              },
              {
                "name": "family_name",
                "ty": {
                  "Optional": "String"
                }
              },
              {
                "name": "given_name",
                "ty": {
                  "Optional": "String"
                }
              },
              {
                "name": "follower_count",
                "ty": "BigInt"
              },
              {
                "name": "backer_count",
                "ty": "BigInt"
              },
              {
                "name": "description",
                "ty": "String"
              },
              {
                "name": "social_media",
                "ty": "String"
              },
              {
                "name": "risk_score",
                "ty": "Numeric"
              },
              {
                "name": "reputation_score",
                "ty": "Numeric"
              },
              {
                "name": "consistent_score",
                "ty": "Numeric"
              },
              {
                "name": "aum",
                "ty": "Numeric"
              },
              {
                "name": "joined_at",
                "ty": "BigInt"
              },
              {
                "name": "requested_at",
                "ty": "BigInt"
              },
              {
                "name": "approved_at",
                "ty": {
                  "Optional": "BigInt"
                }
              },
              {
                "name": "pending_expert",
                "ty": "Boolean"
              },
              {
                "name": "approved_expert",
                "ty": "Boolean"
              },
              {
                "name": "followed",
                "ty": "Boolean"
              }
            ]
          }
        }
      }
    }
  ],
  "stream_response": null,
  "description": "User lists followed experts",
  "json_schema": null
}"#;
}
impl WsResponse for UserListFollowedExpertsResponse {
    type Request = UserListFollowedExpertsRequest;
}

impl WsRequest for UserUnfollowExpertRequest {
    type Response = UserUnfollowExpertResponse;
    const METHOD_ID: u32 = 20150;
    const SCHEMA: &'static str = r#"{
  "name": "UserUnfollowExpert",
  "code": 20150,
  "parameters": [
    {
      "name": "expert_id",
      "ty": "BigInt"
    }
  ],
  "returns": [
    {
      "name": "success",
      "ty": "Boolean"
    }
  ],
  "stream_response": null,
  "description": "User unfollows an expert",
  "json_schema": null
}"#;
}
impl WsResponse for UserUnfollowExpertResponse {
    type Request = UserUnfollowExpertRequest;
}

impl WsRequest for UserListExpertsRequest {
    type Response = UserListExpertsResponse;
    const METHOD_ID: u32 = 20160;
    const SCHEMA: &'static str = r#"{
  "name": "UserListExperts",
  "code": 20160,
  "parameters": [
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "expert_id",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "user_id",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "user_public_id",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "username",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "family_name",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "given_name",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "description",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "social_media",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "sort_by_followers",
      "ty": {
        "Optional": "Boolean"
      }
    }
  ],
  "returns": [
    {
      "name": "experts_total",
      "ty": "BigInt"
    },
    {
      "name": "experts",
      "ty": {
        "Vec": {
          "Struct": {
            "name": "ListExpertsRow",
            "fields": [
              {
                "name": "expert_id",
                "ty": "BigInt"
              },
              {
                "name": "user_public_id",
                "ty": "BigInt"
              },
              {
                "name": "linked_wallet",
                "ty": "BlockchainAddress"
              },
              {
                "name": "name",
                "ty": "String"
              },
              {
                "name": "family_name",
                "ty": {
                  "Optional": "String"
                }
              },
              {
                "name": "given_name",
                "ty": {
                  "Optional": "String"
                }
              },
              {
                "name": "follower_count",
                "ty": "BigInt"
              },
              {
                "name": "backer_count",
                "ty": "BigInt"
              },
              {
                "name": "description",
                "ty": "String"
              },
              {
                "name": "social_media",
                "ty": "String"
              },
              {
                "name": "risk_score",
                "ty": "Numeric"
              },
              {
                "name": "reputation_score",
                "ty": "Numeric"
              },
              {
                "name": "consistent_score",
                "ty": "Numeric"
              },
              {
                "name": "aum",
                "ty": "Numeric"
              },
              {
                "name": "joined_at",
                "ty": "BigInt"
              },
              {
                "name": "requested_at",
                "ty": "BigInt"
              },
              {
                "name": "approved_at",
                "ty": {
                  "Optional": "BigInt"
                }
              },
              {
                "name": "pending_expert",
                "ty": "Boolean"
              },
              {
                "name": "approved_expert",
                "ty": "Boolean"
              },
              {
                "name": "followed",
                "ty": "Boolean"
              }
            ]
          }
        }
      }
    }
  ],
  "stream_response": null,
  "description": "User lists experts",
  "json_schema": null
}"#;
}
impl WsResponse for UserListExpertsResponse {
    type Request = UserListExpertsRequest;
}

impl WsRequest for UserListTopPerformingExpertsRequest {
    type Response = UserListTopPerformingExpertsResponse;
    const METHOD_ID: u32 = 20161;
    const SCHEMA: &'static str = r#"{
  "name": "UserListTopPerformingExperts",
  "code": 20161,
  "parameters": [
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "returns": [
    {
      "name": "experts_total",
      "ty": "BigInt"
    },
    {
      "name": "experts",
      "ty": {
        "Vec": {
          "Struct": {
            "name": "ListExpertsRow",
            "fields": [
              {
                "name": "expert_id",
                "ty": "BigInt"
              },
              {
                "name": "user_public_id",
                "ty": "BigInt"
              },
              {
                "name": "linked_wallet",
                "ty": "BlockchainAddress"
              },
              {
                "name": "name",
                "ty": "String"
              },
              {
                "name": "family_name",
                "ty": {
                  "Optional": "String"
                }
              },
              {
                "name": "given_name",
                "ty": {
                  "Optional": "String"
                }
              },
              {
                "name": "follower_count",
                "ty": "BigInt"
              },
              {
                "name": "backer_count",
                "ty": "BigInt"
              },
              {
                "name": "description",
                "ty": "String"
              },
              {
                "name": "social_media",
                "ty": "String"
              },
              {
                "name": "risk_score",
                "ty": "Numeric"
              },
              {
                "name": "reputation_score",
                "ty": "Numeric"
              },
              {
                "name": "consistent_score",
                "ty": "Numeric"
              },
              {
                "name": "aum",
                "ty": "Numeric"
              },
              {
                "name": "joined_at",
                "ty": "BigInt"
              },
              {
                "name": "requested_at",
                "ty": "BigInt"
              },
              {
                "name": "approved_at",
                "ty": {
                  "Optional": "BigInt"
                }
              },
              {
                "name": "pending_expert",
                "ty": "Boolean"
              },
              {
                "name": "approved_expert",
                "ty": "Boolean"
              },
              {
                "name": "followed",
                "ty": "Boolean"
              }
            ]
          }
        }
      }
    }
  ],
  "stream_response": null,
  "description": "User lists experts",
  "json_schema": null
}"#;
}
impl WsResponse for UserListTopPerformingExpertsResponse {
    type Request = UserListTopPerformingExpertsRequest;
}

impl WsRequest for UserListFeaturedExpertsRequest {
    type Response = UserListFeaturedExpertsResponse;
    const METHOD_ID: u32 = 20162;
    const SCHEMA: &'static str = r#"{
  "name": "UserListFeaturedExperts",
  "code": 20162,
  "parameters": [
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "returns": [
    {
      "name": "experts_total",
      "ty": "BigInt"
    },
    {
      "name": "experts",
      "ty": {
        "Vec": {
          "Struct": {
            "name": "ListExpertsRow",
            "fields": [
              {
                "name": "expert_id",
                "ty": "BigInt"
              },
              {
                "name": "user_public_id",
                "ty": "BigInt"
              },
              {
                "name": "linked_wallet",
                "ty": "BlockchainAddress"
              },
              {
                "name": "name",
                "ty": "String"
              },
              {
                "name": "family_name",
                "ty": {
                  "Optional": "String"
                }
              },
              {
                "name": "given_name",
                "ty": {
                  "Optional": "String"
                }
              },
              {
                "name": "follower_count",
                "ty": "BigInt"
              },
              {
                "name": "backer_count",
                "ty": "BigInt"
              },
              {
                "name": "description",
                "ty": "String"
              },
              {
                "name": "social_media",
                "ty": "String"
              },
              {
                "name": "risk_score",
                "ty": "Numeric"
              },
              {
                "name": "reputation_score",
                "ty": "Numeric"
              },
              {
                "name": "consistent_score",
                "ty": "Numeric"
              },
              {
                "name": "aum",
                "ty": "Numeric"
              },
              {
                "name": "joined_at",
                "ty": "BigInt"
              },
              {
                "name": "requested_at",
                "ty": "BigInt"
              },
              {
                "name": "approved_at",
                "ty": {
                  "Optional": "BigInt"
                }
              },
              {
                "name": "pending_expert",
                "ty": "Boolean"
              },
              {
                "name": "approved_expert",
                "ty": "Boolean"
              },
              {
                "name": "followed",
                "ty": "Boolean"
              }
            ]
          }
        }
      }
    }
  ],
  "stream_response": null,
  "description": "User lists experts",
  "json_schema": null
}"#;
}
impl WsResponse for UserListFeaturedExpertsResponse {
    type Request = UserListFeaturedExpertsRequest;
}

impl WsRequest for UserGetExpertProfileRequest {
    type Response = UserGetExpertProfileResponse;
    const METHOD_ID: u32 = 20170;
    const SCHEMA: &'static str = r#"{
  "name": "UserGetExpertProfile",
  "code": 20170,
  "parameters": [
    {
      "name": "expert_id",
      "ty": "BigInt"
    }
  ],
  "returns": [
    {
      "name": "expert_id",
      "ty": "BigInt"
    },
    {
      "name": "name",
      "ty": "String"
    },
    {
      "name": "follower_count",
      "ty": "Int"
    },
    {
      "name": "backers_count",
      "ty": "Int"
    },
    {
      "name": "description",
      "ty": "String"
    },
    {
      "name": "social_media",
      "ty": "String"
    },
    {
      "name": "risk_score",
      "ty": "Numeric"
    },
    {
      "name": "reputation_score",
      "ty": "Numeric"
    },
    {
      "name": "aum",
      "ty": "Numeric"
    },
    {
      "name": "followed",
      "ty": "Boolean"
    },
    {
      "name": "strategies_total",
      "ty": "BigInt"
    },
    {
      "name": "strategies",
      "ty": {
        "Vec": {
          "Struct": {
            "name": "ListStrategiesRow",
            "fields": [
              {
                "name": "strategy_id",
                "ty": "BigInt"
              },
              {
                "name": "strategy_name",
                "ty": "String"
              },
              {
                "name": "strategy_description",
                "ty": "String"
              },
              {
                "name": "followers",
                "ty": "Int"
              },
              {
                "name": "backers",
                "ty": "Int"
              },
              {
                "name": "aum",
                "ty": "Numeric"
              },
              {
                "name": "followed",
                "ty": "Boolean"
              },
              {
                "name": "swap_price",
                "ty": "Numeric"
              },
              {
                "name": "price_change",
                "ty": "Numeric"
              },
              {
                "name": "strategy_pool_address",
                "ty": {
                  "Optional": "BlockchainAddress"
                }
              },
              {
                "name": "approved",
                "ty": "Boolean"
              },
              {
                "name": "approved_at",
                "ty": {
                  "Optional": "BigInt"
                }
              },
              {
                "name": "blockchain",
                "ty": {
                  "EnumRef": "block_chain"
                }
              },
              {
                "name": "requested_at",
                "ty": {
                  "Optional": "BigInt"
                }
              },
              {
                "name": "created_at",
                "ty": "BigInt"
              },
              {
                "name": "expert_public_id",
                "ty": "BigInt"
              },
              {
                "name": "expert_username",
                "ty": "String"
              },
              {
                "name": "expert_family_name",
                "ty": "String"
              },
              {
                "name": "expert_given_name",
                "ty": "String"
              },
              {
                "name": "reputation",
                "ty": "Int"
              },
              {
                "name": "risk_score",
                "ty": "Numeric"
              },
              {
                "name": "strategy_pool_token",
                "ty": "String"
              },
              {
                "name": "strategy_fee",
                "ty": "Numeric"
              },
              {
                "name": "platform_fee",
                "ty": "Numeric"
              },
              {
                "name": "expert_fee",
                "ty": "Numeric"
              },
              {
                "name": "swap_fee",
                "ty": "Numeric"
              },
              {
                "name": "total_fee",
                "ty": "Numeric"
              },
              {
                "name": "number_of_tokens",
                "ty": "BigInt"
              }
            ]
          }
        }
      }
    }
  ],
  "stream_response": null,
  "description": "User gets an expert profile",
  "json_schema": null
}"#;
}
impl WsResponse for UserGetExpertProfileResponse {
    type Request = UserGetExpertProfileRequest;
}

impl WsRequest for UserGetUserProfileRequest {
    type Response = UserGetUserProfileResponse;
    const METHOD_ID: u32 = 20180;
    const SCHEMA: &'static str = r#"{
  "name": "UserGetUserProfile",
  "code": 20180,
  "parameters": [],
  "returns": [
    {
      "name": "name",
      "ty": "String"
    },
    {
      "name": "login_wallet",
      "ty": "String"
    },
    {
      "name": "joined_at",
      "ty": "BigInt"
    },
    {
      "name": "follower_count",
      "ty": "Int"
    },
    {
      "name": "description",
      "ty": "String"
    },
    {
      "name": "social_media",
      "ty": "String"
    },
    {
      "name": "followed_experts",
      "ty": {
        "Vec": {
          "Struct": {
            "name": "ListExpertsRow",
            "fields": [
              {
                "name": "expert_id",
                "ty": "BigInt"
              },
              {
                "name": "user_public_id",
                "ty": "BigInt"
              },
              {
                "name": "linked_wallet",
                "ty": "BlockchainAddress"
              },
              {
                "name": "name",
                "ty": "String"
              },
              {
                "name": "family_name",
                "ty": {
                  "Optional": "String"
                }
              },
              {
                "name": "given_name",
                "ty": {
                  "Optional": "String"
                }
              },
              {
                "name": "follower_count",
                "ty": "BigInt"
              },
              {
                "name": "backer_count",
                "ty": "BigInt"
              },
              {
                "name": "description",
                "ty": "String"
              },
              {
                "name": "social_media",
                "ty": "String"
              },
              {
                "name": "risk_score",
                "ty": "Numeric"
              },
              {
                "name": "reputation_score",
                "ty": "Numeric"
              },
              {
                "name": "consistent_score",
                "ty": "Numeric"
              },
              {
                "name": "aum",
                "ty": "Numeric"
              },
              {
                "name": "joined_at",
                "ty": "BigInt"
              },
              {
                "name": "requested_at",
                "ty": "BigInt"
              },
              {
                "name": "approved_at",
                "ty": {
                  "Optional": "BigInt"
                }
              },
              {
                "name": "pending_expert",
                "ty": "Boolean"
              },
              {
                "name": "approved_expert",
                "ty": "Boolean"
              },
              {
                "name": "followed",
                "ty": "Boolean"
              }
            ]
          }
        }
      }
    },
    {
      "name": "followed_strategies",
      "ty": {
        "Vec": {
          "Struct": {
            "name": "ListStrategiesRow",
            "fields": [
              {
                "name": "strategy_id",
                "ty": "BigInt"
              },
              {
                "name": "strategy_name",
                "ty": "String"
              },
              {
                "name": "strategy_description",
                "ty": "String"
              },
              {
                "name": "followers",
                "ty": "Int"
              },
              {
                "name": "backers",
                "ty": "Int"
              },
              {
                "name": "aum",
                "ty": "Numeric"
              },
              {
                "name": "followed",
                "ty": "Boolean"
              },
              {
                "name": "swap_price",
                "ty": "Numeric"
              },
              {
                "name": "price_change",
                "ty": "Numeric"
              },
              {
                "name": "strategy_pool_address",
                "ty": {
                  "Optional": "BlockchainAddress"
                }
              },
              {
                "name": "approved",
                "ty": "Boolean"
              },
              {
                "name": "approved_at",
                "ty": {
                  "Optional": "BigInt"
                }
              },
              {
                "name": "blockchain",
                "ty": {
                  "EnumRef": "block_chain"
                }
              },
              {
                "name": "requested_at",
                "ty": {
                  "Optional": "BigInt"
                }
              },
              {
                "name": "created_at",
                "ty": "BigInt"
              },
              {
                "name": "expert_public_id",
                "ty": "BigInt"
              },
              {
                "name": "expert_username",
                "ty": "String"
              },
              {
                "name": "expert_family_name",
                "ty": "String"
              },
              {
                "name": "expert_given_name",
                "ty": "String"
              },
              {
                "name": "reputation",
                "ty": "Int"
              },
              {
                "name": "risk_score",
                "ty": "Numeric"
              },
              {
                "name": "strategy_pool_token",
                "ty": "String"
              },
              {
                "name": "strategy_fee",
                "ty": "Numeric"
              },
              {
                "name": "platform_fee",
                "ty": "Numeric"
              },
              {
                "name": "expert_fee",
                "ty": "Numeric"
              },
              {
                "name": "swap_fee",
                "ty": "Numeric"
              },
              {
                "name": "total_fee",
                "ty": "Numeric"
              },
              {
                "name": "number_of_tokens",
                "ty": "BigInt"
              }
            ]
          }
        }
      }
    },
    {
      "name": "backed_strategies",
      "ty": {
        "Vec": {
          "Struct": {
            "name": "ListStrategiesRow",
            "fields": [
              {
                "name": "strategy_id",
                "ty": "BigInt"
              },
              {
                "name": "strategy_name",
                "ty": "String"
              },
              {
                "name": "strategy_description",
                "ty": "String"
              },
              {
                "name": "followers",
                "ty": "Int"
              },
              {
                "name": "backers",
                "ty": "Int"
              },
              {
                "name": "aum",
                "ty": "Numeric"
              },
              {
                "name": "followed",
                "ty": "Boolean"
              },
              {
                "name": "swap_price",
                "ty": "Numeric"
              },
              {
                "name": "price_change",
                "ty": "Numeric"
              },
              {
                "name": "strategy_pool_address",
                "ty": {
                  "Optional": "BlockchainAddress"
                }
              },
              {
                "name": "approved",
                "ty": "Boolean"
              },
              {
                "name": "approved_at",
                "ty": {
                  "Optional": "BigInt"
                }
              },
              {
                "name": "blockchain",
                "ty": {
                  "EnumRef": "block_chain"
                }
              },
              {
                "name": "requested_at",
                "ty": {
                  "Optional": "BigInt"
                }
              },
              {
                "name": "created_at",
                "ty": "BigInt"
              },
              {
                "name": "expert_public_id",
                "ty": "BigInt"
              },
              {
                "name": "expert_username",
                "ty": "String"
              },
              {
                "name": "expert_family_name",
                "ty": "String"
              },
              {
                "name": "expert_given_name",
                "ty": "String"
              },
              {
                "name": "reputation",
                "ty": "Int"
              },
              {
                "name": "risk_score",
                "ty": "Numeric"
              },
              {
                "name": "strategy_pool_token",
                "ty": "String"
              },
              {
                "name": "strategy_fee",
                "ty": "Numeric"
              },
              {
                "name": "platform_fee",
                "ty": "Numeric"
              },
              {
                "name": "expert_fee",
                "ty": "Numeric"
              },
              {
                "name": "swap_fee",
                "ty": "Numeric"
              },
              {
                "name": "total_fee",
                "ty": "Numeric"
              },
              {
                "name": "number_of_tokens",
                "ty": "BigInt"
              }
            ]
          }
        }
      }
    }
  ],
  "stream_response": null,
  "description": "User gets an user profile",
  "json_schema": null
}"#;
}
impl WsResponse for UserGetUserProfileResponse {
    type Request = UserGetUserProfileRequest;
}

impl WsRequest for UserWhitelistWalletRequest {
    type Response = UserWhitelistWalletResponse;
    const METHOD_ID: u32 = 20190;
    const SCHEMA: &'static str = r#"{
  "name": "UserWhitelistWallet",
  "code": 20190,
  "parameters": [
    {
      "name": "blockchain",
      "ty": {
        "EnumRef": "block_chain"
      }
    },
    {
      "name": "wallet_address",
      "ty": "BlockchainAddress"
    }
  ],
  "returns": [
    {
      "name": "success",
      "ty": "Boolean"
    },
    {
      "name": "wallet_id",
      "ty": "BigInt"
    }
  ],
  "stream_response": null,
  "description": "User registers a wallet",
  "json_schema": null
}"#;
}
impl WsResponse for UserWhitelistWalletResponse {
    type Request = UserWhitelistWalletRequest;
}

impl WsRequest for UserListWhitelistedWalletsRequest {
    type Response = UserListWhitelistedWalletsResponse;
    const METHOD_ID: u32 = 20200;
    const SCHEMA: &'static str = r#"{
  "name": "UserListWhitelistedWallets",
  "code": 20200,
  "parameters": [
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "wallet_id",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "blockchain",
      "ty": {
        "Optional": {
          "EnumRef": "block_chain"
        }
      }
    },
    {
      "name": "wallet_address",
      "ty": {
        "Optional": "BlockchainAddress"
      }
    },
    {
      "name": "strategy_id",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "returns": [
    {
      "name": "wallets",
      "ty": {
        "DataTable": {
          "name": "ListWalletsRow",
          "fields": [
            {
              "name": "wallet_id",
              "ty": "BigInt"
            },
            {
              "name": "blockchain",
              "ty": {
                "EnumRef": "block_chain"
              }
            },
            {
              "name": "wallet_address",
              "ty": "BlockchainAddress"
            },
            {
              "name": "is_default",
              "ty": "Boolean"
            },
            {
              "name": "is_compatible",
              "ty": "Boolean"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "User lists wallets",
  "json_schema": null
}"#;
}
impl WsResponse for UserListWhitelistedWalletsResponse {
    type Request = UserListWhitelistedWalletsRequest;
}

impl WsRequest for UserUnwhitelistWalletRequest {
    type Response = UserUnwhitelistWalletResponse;
    const METHOD_ID: u32 = 20210;
    const SCHEMA: &'static str = r#"{
  "name": "UserUnwhitelistWallet",
  "code": 20210,
  "parameters": [
    {
      "name": "wallet_id",
      "ty": "BigInt"
    }
  ],
  "returns": [
    {
      "name": "success",
      "ty": "Boolean"
    }
  ],
  "stream_response": null,
  "description": "User deregisters a wallet",
  "json_schema": null
}"#;
}
impl WsResponse for UserUnwhitelistWalletResponse {
    type Request = UserUnwhitelistWalletRequest;
}

impl WsRequest for UserApplyBecomeExpertRequest {
    type Response = UserApplyBecomeExpertResponse;
    const METHOD_ID: u32 = 20220;
    const SCHEMA: &'static str = r#"{
  "name": "UserApplyBecomeExpert",
  "code": 20220,
  "parameters": [],
  "returns": [
    {
      "name": "success",
      "ty": "Boolean"
    },
    {
      "name": "expert_id",
      "ty": "BigInt"
    }
  ],
  "stream_response": null,
  "description": "User applies to become an expert",
  "json_schema": null
}"#;
}
impl WsResponse for UserApplyBecomeExpertResponse {
    type Request = UserApplyBecomeExpertRequest;
}

impl WsRequest for ExpertCreateStrategyRequest {
    type Response = ExpertCreateStrategyResponse;
    const METHOD_ID: u32 = 20250;
    const SCHEMA: &'static str = r#"{
  "name": "ExpertCreateStrategy",
  "code": 20250,
  "parameters": [
    {
      "name": "name",
      "ty": "String"
    },
    {
      "name": "description",
      "ty": "String"
    },
    {
      "name": "strategy_thesis_url",
      "ty": "String"
    },
    {
      "name": "minimum_backing_amount_usd",
      "ty": {
        "Optional": "Numeric"
      }
    },
    {
      "name": "expert_fee",
      "ty": "Numeric"
    },
    {
      "name": "agreed_tos",
      "ty": "Boolean"
    },
    {
      "name": "wallet_address",
      "ty": "BlockchainAddress"
    },
    {
      "name": "wallet_blockchain",
      "ty": {
        "EnumRef": "block_chain"
      }
    },
    {
      "name": "strategy_token_relative_to_usdc_ratio",
      "ty": {
        "Optional": "BlockchainDecimal"
      }
    },
    {
      "name": "initial_tokens",
      "ty": {
        "DataTable": {
          "name": "UserCreateStrategyInitialTokenRow",
          "fields": [
            {
              "name": "token_id",
              "ty": "BigInt"
            },
            {
              "name": "quantity",
              "ty": "BlockchainDecimal"
            }
          ]
        }
      }
    },
    {
      "name": "audit_rules",
      "ty": {
        "Optional": {
          "Vec": "BigInt"
        }
      }
    }
  ],
  "returns": [
    {
      "name": "success",
      "ty": "Boolean"
    },
    {
      "name": "strategy_id",
      "ty": "BigInt"
    }
  ],
  "stream_response": null,
  "description": "User makes a strategy",
  "json_schema": null
}"#;
}
impl WsResponse for ExpertCreateStrategyResponse {
    type Request = ExpertCreateStrategyRequest;
}

impl WsRequest for ExpertUpdateStrategyRequest {
    type Response = ExpertUpdateStrategyResponse;
    const METHOD_ID: u32 = 20260;
    const SCHEMA: &'static str = r#"{
  "name": "ExpertUpdateStrategy",
  "code": 20260,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": "BigInt"
    },
    {
      "name": "name",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "description",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "social_media",
      "ty": {
        "Optional": "String"
      }
    }
  ],
  "returns": [
    {
      "name": "success",
      "ty": "Boolean"
    }
  ],
  "stream_response": null,
  "description": "Expert updates a strategy",
  "json_schema": null
}"#;
}
impl WsResponse for ExpertUpdateStrategyResponse {
    type Request = ExpertUpdateStrategyRequest;
}

impl WsRequest for ExpertFreezeStrategyRequest {
    type Response = ExpertFreezeStrategyResponse;
    const METHOD_ID: u32 = 20265;
    const SCHEMA: &'static str = r#"{
  "name": "ExpertFreezeStrategy",
  "code": 20265,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": "BigInt"
    }
  ],
  "returns": [
    {
      "name": "success",
      "ty": "Boolean"
    }
  ],
  "stream_response": null,
  "description": "Expert freezes a strategy, by making it immutable",
  "json_schema": null
}"#;
}
impl WsResponse for ExpertFreezeStrategyResponse {
    type Request = ExpertFreezeStrategyRequest;
}

impl WsRequest for ExpertAddStrategyWatchingWalletRequest {
    type Response = ExpertAddStrategyWatchingWalletResponse;
    const METHOD_ID: u32 = 20270;
    const SCHEMA: &'static str = r#"{
  "name": "ExpertAddStrategyWatchingWallet",
  "code": 20270,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": "BigInt"
    },
    {
      "name": "blockchain",
      "ty": {
        "EnumRef": "block_chain"
      }
    },
    {
      "name": "wallet_address",
      "ty": "BlockchainAddress"
    },
    {
      "name": "ratio",
      "ty": "Numeric"
    }
  ],
  "returns": [
    {
      "name": "success",
      "ty": "Boolean"
    },
    {
      "name": "wallet_id",
      "ty": "BigInt"
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for ExpertAddStrategyWatchingWalletResponse {
    type Request = ExpertAddStrategyWatchingWalletRequest;
}

impl WsRequest for ExpertRemoveStrategyWatchingWalletRequest {
    type Response = ExpertRemoveStrategyWatchingWalletResponse;
    const METHOD_ID: u32 = 20280;
    const SCHEMA: &'static str = r#"{
  "name": "ExpertRemoveStrategyWatchingWallet",
  "code": 20280,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": "BigInt"
    },
    {
      "name": "wallet_id",
      "ty": "BigInt"
    }
  ],
  "returns": [
    {
      "name": "success",
      "ty": "Boolean"
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for ExpertRemoveStrategyWatchingWalletResponse {
    type Request = ExpertRemoveStrategyWatchingWalletRequest;
}

impl WsRequest for UserListStrategyWatchingWalletsRequest {
    type Response = UserListStrategyWatchingWalletsResponse;
    const METHOD_ID: u32 = 20290;
    const SCHEMA: &'static str = r#"{
  "name": "UserListStrategyWatchingWallets",
  "code": 20290,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": "BigInt"
    }
  ],
  "returns": [
    {
      "name": "wallets_total",
      "ty": "BigInt"
    },
    {
      "name": "wallets",
      "ty": {
        "DataTable": {
          "name": "ListStrategyWatchingWalletsRow",
          "fields": [
            {
              "name": "wallet_id",
              "ty": "BigInt"
            },
            {
              "name": "blockchain",
              "ty": {
                "EnumRef": "block_chain"
              }
            },
            {
              "name": "wallet_address",
              "ty": "BlockchainAddress"
            },
            {
              "name": "ratio",
              "ty": "Numeric"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserListStrategyWatchingWalletsResponse {
    type Request = UserListStrategyWatchingWalletsRequest;
}

impl WsRequest for UserListWalletActivityLedgerRequest {
    type Response = UserListWalletActivityLedgerResponse;
    const METHOD_ID: u32 = 20300;
    const SCHEMA: &'static str = r#"{
  "name": "UserListWalletActivityLedger",
  "code": 20300,
  "parameters": [
    {
      "name": "wallet_address",
      "ty": "BlockchainAddress"
    },
    {
      "name": "blockchain",
      "ty": {
        "EnumRef": "block_chain"
      }
    }
  ],
  "returns": [
    {
      "name": "wallet_activities_total",
      "ty": "BigInt"
    },
    {
      "name": "wallet_activities",
      "ty": {
        "DataTable": {
          "name": "ListWalletActivityLedgerRow",
          "fields": [
            {
              "name": "record_id",
              "ty": "BigInt"
            },
            {
              "name": "wallet_address",
              "ty": "BlockchainAddress"
            },
            {
              "name": "transaction_hash",
              "ty": "BlockchainTransactionHash"
            },
            {
              "name": "dex",
              "ty": "String"
            },
            {
              "name": "blockchain",
              "ty": {
                "EnumRef": "block_chain"
              }
            },
            {
              "name": "contract_address",
              "ty": "BlockchainAddress"
            },
            {
              "name": "token_in_address",
              "ty": "BlockchainAddress"
            },
            {
              "name": "token_out_address",
              "ty": "BlockchainAddress"
            },
            {
              "name": "caller_address",
              "ty": "BlockchainAddress"
            },
            {
              "name": "amount_in",
              "ty": "BlockchainDecimal"
            },
            {
              "name": "amount_out",
              "ty": "BlockchainDecimal"
            },
            {
              "name": "swap_calls",
              "ty": "Object"
            },
            {
              "name": "paths",
              "ty": "Object"
            },
            {
              "name": "dex_versions",
              "ty": "Object"
            },
            {
              "name": "created_at",
              "ty": "BigInt"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserListWalletActivityLedgerResponse {
    type Request = UserListWalletActivityLedgerRequest;
}

impl WsRequest for ExpertAddStrategyInitialTokenRatioRequest {
    type Response = ExpertAddStrategyInitialTokenRatioResponse;
    const METHOD_ID: u32 = 20310;
    const SCHEMA: &'static str = r#"{
  "name": "ExpertAddStrategyInitialTokenRatio",
  "code": 20310,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": "BigInt"
    },
    {
      "name": "token_id",
      "ty": "BigInt"
    },
    {
      "name": "quantity",
      "ty": "BlockchainDecimal"
    }
  ],
  "returns": [
    {
      "name": "success",
      "ty": "Boolean"
    },
    {
      "name": "token_id",
      "ty": "BigInt"
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for ExpertAddStrategyInitialTokenRatioResponse {
    type Request = ExpertAddStrategyInitialTokenRatioRequest;
}

impl WsRequest for ExpertRemoveStrategyInitialTokenRatioRequest {
    type Response = ExpertRemoveStrategyInitialTokenRatioResponse;
    const METHOD_ID: u32 = 20320;
    const SCHEMA: &'static str = r#"{
  "name": "ExpertRemoveStrategyInitialTokenRatio",
  "code": 20320,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": "BigInt"
    },
    {
      "name": "token_id",
      "ty": "BigInt"
    }
  ],
  "returns": [
    {
      "name": "success",
      "ty": "Boolean"
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for ExpertRemoveStrategyInitialTokenRatioResponse {
    type Request = ExpertRemoveStrategyInitialTokenRatioRequest;
}

impl WsRequest for UserListStrategyInitialTokenRatioRequest {
    type Response = UserListStrategyInitialTokenRatioResponse;
    const METHOD_ID: u32 = 20330;
    const SCHEMA: &'static str = r#"{
  "name": "UserListStrategyInitialTokenRatio",
  "code": 20330,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": "BigInt"
    }
  ],
  "returns": [
    {
      "name": "token_ratios_total",
      "ty": "BigInt"
    },
    {
      "name": "token_ratios",
      "ty": {
        "DataTable": {
          "name": "ListStrategyInitialTokenRatioRow",
          "fields": [
            {
              "name": "token_id",
              "ty": "BigInt"
            },
            {
              "name": "token_name",
              "ty": "String"
            },
            {
              "name": "token_address",
              "ty": "BlockchainAddress"
            },
            {
              "name": "quantity",
              "ty": "BlockchainDecimal"
            },
            {
              "name": "updated_at",
              "ty": "BigInt"
            },
            {
              "name": "created_at",
              "ty": "BigInt"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserListStrategyInitialTokenRatioResponse {
    type Request = UserListStrategyInitialTokenRatioRequest;
}

impl WsRequest for ExpertListFollowersRequest {
    type Response = ExpertListFollowersResponse;
    const METHOD_ID: u32 = 20340;
    const SCHEMA: &'static str = r#"{
  "name": "ExpertListFollowers",
  "code": 20340,
  "parameters": [
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "returns": [
    {
      "name": "followers_total",
      "ty": "BigInt"
    },
    {
      "name": "followers",
      "ty": {
        "DataTable": {
          "name": "ExpertListFollowersRow",
          "fields": [
            {
              "name": "public_id",
              "ty": "BigInt"
            },
            {
              "name": "username",
              "ty": "String"
            },
            {
              "name": "family_name",
              "ty": {
                "Optional": "String"
              }
            },
            {
              "name": "given_name",
              "ty": {
                "Optional": "String"
              }
            },
            {
              "name": "followed_at",
              "ty": "BigInt"
            },
            {
              "name": "joined_at",
              "ty": "BigInt"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for ExpertListFollowersResponse {
    type Request = ExpertListFollowersRequest;
}

impl WsRequest for ExpertListBackersRequest {
    type Response = ExpertListBackersResponse;
    const METHOD_ID: u32 = 20350;
    const SCHEMA: &'static str = r#"{
  "name": "ExpertListBackers",
  "code": 20350,
  "parameters": [
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "returns": [
    {
      "name": "backers_total",
      "ty": "BigInt"
    },
    {
      "name": "backers",
      "ty": {
        "DataTable": {
          "name": "ExpertListBackersRow",
          "fields": [
            {
              "name": "public_id",
              "ty": "BigInt"
            },
            {
              "name": "username",
              "ty": "String"
            },
            {
              "name": "family_name",
              "ty": {
                "Optional": "String"
              }
            },
            {
              "name": "given_name",
              "ty": {
                "Optional": "String"
              }
            },
            {
              "name": "backed_at",
              "ty": "BigInt"
            },
            {
              "name": "joined_at",
              "ty": "BigInt"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for ExpertListBackersResponse {
    type Request = ExpertListBackersRequest;
}

impl WsRequest for UserGetDepositTokensRequest {
    type Response = UserGetDepositTokensResponse;
    const METHOD_ID: u32 = 20360;
    const SCHEMA: &'static str = r#"{
  "name": "UserGetDepositTokens",
  "code": 20360,
  "parameters": [],
  "returns": [
    {
      "name": "tokens",
      "ty": {
        "DataTable": {
          "name": "UserGetDepositTokensRow",
          "fields": [
            {
              "name": "blockchain",
              "ty": {
                "EnumRef": "block_chain"
              }
            },
            {
              "name": "token",
              "ty": "String"
            },
            {
              "name": "address",
              "ty": "BlockchainAddress"
            },
            {
              "name": "short_name",
              "ty": "String"
            },
            {
              "name": "icon_url",
              "ty": "String"
            },
            {
              "name": "conversion",
              "ty": "Numeric"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserGetDepositTokensResponse {
    type Request = UserGetDepositTokensRequest;
}

impl WsRequest for UserGetDepositAddressesRequest {
    type Response = UserGetDepositAddressesResponse;
    const METHOD_ID: u32 = 20370;
    const SCHEMA: &'static str = r#"{
  "name": "UserGetDepositAddresses",
  "code": 20370,
  "parameters": [],
  "returns": [
    {
      "name": "addresses",
      "ty": {
        "DataTable": {
          "name": "UserGetDepositAddressesRow",
          "fields": [
            {
              "name": "blockchain",
              "ty": {
                "EnumRef": "block_chain"
              }
            },
            {
              "name": "address",
              "ty": "BlockchainAddress"
            },
            {
              "name": "short_name",
              "ty": "String"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserGetDepositAddressesResponse {
    type Request = UserGetDepositAddressesRequest;
}

impl WsRequest for UserListDepositWithdrawLedgerRequest {
    type Response = UserListDepositWithdrawLedgerResponse;
    const METHOD_ID: u32 = 20380;
    const SCHEMA: &'static str = r#"{
  "name": "UserListDepositWithdrawLedger",
  "code": 20380,
  "parameters": [
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "blockchain",
      "ty": {
        "Optional": {
          "EnumRef": "block_chain"
        }
      }
    },
    {
      "name": "id_deposit",
      "ty": {
        "Optional": "Boolean"
      }
    }
  ],
  "returns": [
    {
      "name": "ledger_total",
      "ty": "BigInt"
    },
    {
      "name": "ledger",
      "ty": {
        "Vec": {
          "Struct": {
            "name": "UserListDepositLedgerRow",
            "fields": [
              {
                "name": "transaction_id",
                "ty": "BigInt"
              },
              {
                "name": "blockchain",
                "ty": {
                  "EnumRef": "block_chain"
                }
              },
              {
                "name": "user_address",
                "ty": "BlockchainAddress"
              },
              {
                "name": "contract_address",
                "ty": "BlockchainAddress"
              },
              {
                "name": "receiver_address",
                "ty": "BlockchainAddress"
              },
              {
                "name": "quantity",
                "ty": "BlockchainDecimal"
              },
              {
                "name": "transaction_hash",
                "ty": "BlockchainTransactionHash"
              },
              {
                "name": "is_deposit",
                "ty": "Boolean"
              },
              {
                "name": "happened_at",
                "ty": "BigInt"
              }
            ]
          }
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserListDepositWithdrawLedgerResponse {
    type Request = UserListDepositWithdrawLedgerRequest;
}

impl WsRequest for UserSubscribeDepositLedgerRequest {
    type Response = UserSubscribeDepositLedgerResponse;
    const METHOD_ID: u32 = 20381;
    const SCHEMA: &'static str = r#"{
  "name": "UserSubscribeDepositLedger",
  "code": 20381,
  "parameters": [
    {
      "name": "initial_data",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "blockchain",
      "ty": {
        "Optional": {
          "EnumRef": "block_chain"
        }
      }
    },
    {
      "name": "mock_data",
      "ty": {
        "Optional": "Boolean"
      }
    }
  ],
  "returns": [],
  "stream_response": {
    "Struct": {
      "name": "UserListDepositLedgerRow",
      "fields": [
        {
          "name": "transaction_id",
          "ty": "BigInt"
        },
        {
          "name": "blockchain",
          "ty": {
            "EnumRef": "block_chain"
          }
        },
        {
          "name": "user_address",
          "ty": "BlockchainAddress"
        },
        {
          "name": "contract_address",
          "ty": "BlockchainAddress"
        },
        {
          "name": "receiver_address",
          "ty": "BlockchainAddress"
        },
        {
          "name": "quantity",
          "ty": "BlockchainDecimal"
        },
        {
          "name": "transaction_hash",
          "ty": "BlockchainTransactionHash"
        },
        {
          "name": "is_deposit",
          "ty": "Boolean"
        },
        {
          "name": "happened_at",
          "ty": "BigInt"
        }
      ]
    }
  },
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserSubscribeDepositLedgerResponse {
    type Request = UserSubscribeDepositLedgerRequest;
}

impl WsRequest for UserUnsubscribeDepositLedgerRequest {
    type Response = UserUnsubscribeDepositLedgerResponse;
    const METHOD_ID: u32 = 20382;
    const SCHEMA: &'static str = r#"{
  "name": "UserUnsubscribeDepositLedger",
  "code": 20382,
  "parameters": [],
  "returns": [],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserUnsubscribeDepositLedgerResponse {
    type Request = UserUnsubscribeDepositLedgerRequest;
}

impl WsRequest for UserListStrategyWalletsRequest {
    type Response = UserListStrategyWalletsResponse;
    const METHOD_ID: u32 = 20390;
    const SCHEMA: &'static str = r#"{
  "name": "UserListStrategyWallets",
  "code": 20390,
  "parameters": [
    {
      "name": "blockchain",
      "ty": {
        "Optional": {
          "EnumRef": "block_chain"
        }
      }
    }
  ],
  "returns": [
    {
      "name": "wallets_total",
      "ty": "BigInt"
    },
    {
      "name": "wallets",
      "ty": {
        "DataTable": {
          "name": "UserListStrategyWalletsRow",
          "fields": [
            {
              "name": "blockchain",
              "ty": {
                "EnumRef": "block_chain"
              }
            },
            {
              "name": "address",
              "ty": "BlockchainAddress"
            },
            {
              "name": "is_platform_managed",
              "ty": "Boolean"
            },
            {
              "name": "created_at",
              "ty": "BigInt"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserListStrategyWalletsResponse {
    type Request = UserListStrategyWalletsRequest;
}

impl WsRequest for UserCreateStrategyWalletRequest {
    type Response = UserCreateStrategyWalletResponse;
    const METHOD_ID: u32 = 20391;
    const SCHEMA: &'static str = r#"{
  "name": "UserCreateStrategyWallet",
  "code": 20391,
  "parameters": [
    {
      "name": "blockchain",
      "ty": {
        "EnumRef": "block_chain"
      }
    },
    {
      "name": "user_managed_wallet_address",
      "ty": {
        "Optional": "BlockchainAddress"
      }
    }
  ],
  "returns": [
    {
      "name": "blockchain",
      "ty": {
        "EnumRef": "block_chain"
      }
    },
    {
      "name": "address",
      "ty": "BlockchainAddress"
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserCreateStrategyWalletResponse {
    type Request = UserCreateStrategyWalletRequest;
}

impl WsRequest for UserListStrategyAuditRulesRequest {
    type Response = UserListStrategyAuditRulesResponse;
    const METHOD_ID: u32 = 20400;
    const SCHEMA: &'static str = r#"{
  "name": "UserListStrategyAuditRules",
  "code": 20400,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "returns": [
    {
      "name": "audit_rules",
      "ty": {
        "DataTable": {
          "name": "UserListStrategyAuditRulesRow",
          "fields": [
            {
              "name": "rule_id",
              "ty": "BigInt"
            },
            {
              "name": "rule_name",
              "ty": "String"
            },
            {
              "name": "rule_description",
              "ty": "String"
            },
            {
              "name": "created_at",
              "ty": "BigInt"
            },
            {
              "name": "enabled",
              "ty": "Boolean"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserListStrategyAuditRulesResponse {
    type Request = UserListStrategyAuditRulesRequest;
}

impl WsRequest for UserAddStrategyAuditRuleRequest {
    type Response = UserAddStrategyAuditRuleResponse;
    const METHOD_ID: u32 = 20410;
    const SCHEMA: &'static str = r#"{
  "name": "UserAddStrategyAuditRule",
  "code": 20410,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": "BigInt"
    },
    {
      "name": "rule_id",
      "ty": "BigInt"
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserAddStrategyAuditRuleResponse {
    type Request = UserAddStrategyAuditRuleRequest;
}

impl WsRequest for UserRemoveStrategyAuditRuleRequest {
    type Response = UserRemoveStrategyAuditRuleResponse;
    const METHOD_ID: u32 = 20420;
    const SCHEMA: &'static str = r#"{
  "name": "UserRemoveStrategyAuditRule",
  "code": 20420,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": "BigInt"
    },
    {
      "name": "rule_id",
      "ty": "BigInt"
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserRemoveStrategyAuditRuleResponse {
    type Request = UserRemoveStrategyAuditRuleRequest;
}

impl WsRequest for UserGetEscrowAddressForStrategyRequest {
    type Response = UserGetEscrowAddressForStrategyResponse;
    const METHOD_ID: u32 = 20500;
    const SCHEMA: &'static str = r#"{
  "name": "UserGetEscrowAddressForStrategy",
  "code": 20500,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": "BigInt"
    },
    {
      "name": "token_id",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "returns": [
    {
      "name": "tokens",
      "ty": {
        "DataTable": {
          "name": "UserAllowedEscrowTransferInfo",
          "fields": [
            {
              "name": "receiver_address",
              "ty": "BlockchainAddress"
            },
            {
              "name": "blockchain",
              "ty": {
                "EnumRef": "block_chain"
              }
            },
            {
              "name": "token_id",
              "ty": "BigInt"
            },
            {
              "name": "token_symbol",
              "ty": "String"
            },
            {
              "name": "token_name",
              "ty": "String"
            },
            {
              "name": "token_address",
              "ty": "BlockchainAddress"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserGetEscrowAddressForStrategyResponse {
    type Request = UserGetEscrowAddressForStrategyRequest;
}

impl WsRequest for UserListDepositWithdrawBalancesRequest {
    type Response = UserListDepositWithdrawBalancesResponse;
    const METHOD_ID: u32 = 20510;
    const SCHEMA: &'static str = r#"{
  "name": "UserListDepositWithdrawBalances",
  "code": 20510,
  "parameters": [],
  "returns": [
    {
      "name": "balances",
      "ty": {
        "DataTable": {
          "name": "UserListDepositWithdrawBalance",
          "fields": [
            {
              "name": "blockchain",
              "ty": {
                "EnumRef": "block_chain"
              }
            },
            {
              "name": "token_id",
              "ty": "BigInt"
            },
            {
              "name": "token_symbol",
              "ty": "String"
            },
            {
              "name": "token_name",
              "ty": "String"
            },
            {
              "name": "balance",
              "ty": "BlockchainDecimal"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserListDepositWithdrawBalancesResponse {
    type Request = UserListDepositWithdrawBalancesRequest;
}

impl WsRequest for UserGetDepositWithdrawBalanceRequest {
    type Response = UserGetDepositWithdrawBalanceResponse;
    const METHOD_ID: u32 = 20511;
    const SCHEMA: &'static str = r#"{
  "name": "UserGetDepositWithdrawBalance",
  "code": 20511,
  "parameters": [
    {
      "name": "token_id",
      "ty": "BigInt"
    }
  ],
  "returns": [
    {
      "name": "balance",
      "ty": "BlockchainDecimal"
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserGetDepositWithdrawBalanceResponse {
    type Request = UserGetDepositWithdrawBalanceRequest;
}

impl WsRequest for UserListEscrowTokenContractAddressesRequest {
    type Response = UserListEscrowTokenContractAddressesResponse;
    const METHOD_ID: u32 = 20520;
    const SCHEMA: &'static str = r#"{
  "name": "UserListEscrowTokenContractAddresses",
  "code": 20520,
  "parameters": [
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "blockchain",
      "ty": {
        "Optional": {
          "EnumRef": "block_chain"
        }
      }
    },
    {
      "name": "is_stablecoin",
      "ty": {
        "Optional": "Boolean"
      }
    }
  ],
  "returns": [
    {
      "name": "tokens_total",
      "ty": "BigInt"
    },
    {
      "name": "tokens",
      "ty": {
        "DataTable": {
          "name": "UserListEscrowTokenContractAddressesRow",
          "fields": [
            {
              "name": "token_id",
              "ty": "BigInt"
            },
            {
              "name": "token_symbol",
              "ty": "String"
            },
            {
              "name": "token_name",
              "ty": "String"
            },
            {
              "name": "token_address",
              "ty": "BlockchainAddress"
            },
            {
              "name": "description",
              "ty": "String"
            },
            {
              "name": "blockchain",
              "ty": {
                "EnumRef": "block_chain"
              }
            },
            {
              "name": "is_stablecoin",
              "ty": "Boolean"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserListEscrowTokenContractAddressesResponse {
    type Request = UserListEscrowTokenContractAddressesRequest;
}

impl WsRequest for UserListStrategyTokenBalanceRequest {
    type Response = UserListStrategyTokenBalanceResponse;
    const METHOD_ID: u32 = 20530;
    const SCHEMA: &'static str = r#"{
  "name": "UserListStrategyTokenBalance",
  "code": 20530,
  "parameters": [
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "strategy_id",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "returns": [
    {
      "name": "tokens_total",
      "ty": "BigInt"
    },
    {
      "name": "tokens",
      "ty": {
        "DataTable": {
          "name": "UserListStrategyTokenBalanceRow",
          "fields": [
            {
              "name": "strategy_id",
              "ty": "BigInt"
            },
            {
              "name": "strategy_name",
              "ty": "String"
            },
            {
              "name": "balance",
              "ty": "BlockchainDecimal"
            },
            {
              "name": "address",
              "ty": "BlockchainAddress"
            },
            {
              "name": "blockchain",
              "ty": {
                "EnumRef": "block_chain"
              }
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserListStrategyTokenBalanceResponse {
    type Request = UserListStrategyTokenBalanceRequest;
}

impl WsRequest for UserGetBackStrategyReviewDetailRequest {
    type Response = UserGetBackStrategyReviewDetailResponse;
    const METHOD_ID: u32 = 20540;
    const SCHEMA: &'static str = r#"{
  "name": "UserGetBackStrategyReviewDetail",
  "code": 20540,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": "BigInt"
    },
    {
      "name": "token_id",
      "ty": "BigInt"
    },
    {
      "name": "quantity",
      "ty": "BlockchainDecimal"
    }
  ],
  "returns": [
    {
      "name": "strategy_fee",
      "ty": "BlockchainDecimal"
    },
    {
      "name": "total_amount_to_back",
      "ty": "BlockchainDecimal"
    },
    {
      "name": "total_amount_to_back_after_fee",
      "ty": "BlockchainDecimal"
    },
    {
      "name": "user_strategy_wallets",
      "ty": {
        "DataTable": {
          "name": "UserStrategyWallet",
          "fields": [
            {
              "name": "wallet_id",
              "ty": "BigInt"
            },
            {
              "name": "address",
              "ty": "BlockchainAddress"
            },
            {
              "name": "blockchain",
              "ty": {
                "EnumRef": "block_chain"
              }
            },
            {
              "name": "is_platform_address",
              "ty": "Boolean"
            }
          ]
        }
      }
    },
    {
      "name": "estimated_amount_of_strategy_tokens",
      "ty": "BlockchainDecimal"
    },
    {
      "name": "estimated_backed_token_ratios",
      "ty": {
        "DataTable": {
          "name": "EstimatedBackedTokenRatios",
          "fields": [
            {
              "name": "token_id",
              "ty": "BigInt"
            },
            {
              "name": "token_name",
              "ty": "String"
            },
            {
              "name": "back_amount",
              "ty": "BlockchainDecimal"
            },
            {
              "name": "back_value_in_usd",
              "ty": "BlockchainDecimal"
            },
            {
              "name": "back_value_ratio",
              "ty": "Numeric"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserGetBackStrategyReviewDetailResponse {
    type Request = UserGetBackStrategyReviewDetailRequest;
}

impl WsRequest for UserListUserBackStrategyAttemptRequest {
    type Response = UserListUserBackStrategyAttemptResponse;
    const METHOD_ID: u32 = 20550;
    const SCHEMA: &'static str = r#"{
  "name": "UserListUserBackStrategyAttempt",
  "code": 20550,
  "parameters": [
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "strategy_id",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "token_id",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "returns": [
    {
      "name": "total",
      "ty": "BigInt"
    },
    {
      "name": "back_attempts",
      "ty": {
        "DataTable": {
          "name": "UserBackStrategyAttempt",
          "fields": [
            {
              "name": "attempt_id",
              "ty": "BigInt"
            },
            {
              "name": "strategy_id",
              "ty": "BigInt"
            },
            {
              "name": "strategy_name",
              "ty": "String"
            },
            {
              "name": "token_id",
              "ty": "BigInt"
            },
            {
              "name": "token_symbol",
              "ty": "String"
            },
            {
              "name": "token_name",
              "ty": "String"
            },
            {
              "name": "quantity",
              "ty": "BlockchainDecimal"
            },
            {
              "name": "happened_at",
              "ty": "BigInt"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserListUserBackStrategyAttemptResponse {
    type Request = UserListUserBackStrategyAttemptRequest;
}

impl WsRequest for UserListUserBackStrategyLogRequest {
    type Response = UserListUserBackStrategyLogResponse;
    const METHOD_ID: u32 = 20560;
    const SCHEMA: &'static str = r#"{
  "name": "UserListUserBackStrategyLog",
  "code": 20560,
  "parameters": [
    {
      "name": "attempt_id",
      "ty": "BigInt"
    },
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "returns": [
    {
      "name": "back_logs_total",
      "ty": "BigInt"
    },
    {
      "name": "back_logs",
      "ty": {
        "DataTable": {
          "name": "UserBackStrategyLog",
          "fields": [
            {
              "name": "pkey_id",
              "ty": "BigInt"
            },
            {
              "name": "message",
              "ty": "String"
            },
            {
              "name": "happened_at",
              "ty": "BigInt"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for UserListUserBackStrategyLogResponse {
    type Request = UserListUserBackStrategyLogRequest;
}

impl WsRequest for UserGetSystemConfigRequest {
    type Response = UserGetSystemConfigResponse;
    const METHOD_ID: u32 = 20570;
    const SCHEMA: &'static str = r#"{
  "name": "UserGetSystemConfig",
  "code": 20570,
  "parameters": [],
  "returns": [
    {
      "name": "platform_fee",
      "ty": "Numeric"
    }
  ],
  "stream_response": null,
  "description": "User get system config",
  "json_schema": null
}"#;
}
impl WsResponse for UserGetSystemConfigResponse {
    type Request = UserGetSystemConfigRequest;
}

impl WsRequest for AdminListUsersRequest {
    type Response = AdminListUsersResponse;
    const METHOD_ID: u32 = 30010;
    const SCHEMA: &'static str = r#"{
  "name": "AdminListUsers",
  "code": 30010,
  "parameters": [
    {
      "name": "limit",
      "ty": "BigInt"
    },
    {
      "name": "offset",
      "ty": "BigInt"
    },
    {
      "name": "user_id",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "address",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "username",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "email",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "role",
      "ty": {
        "Optional": {
          "EnumRef": "role"
        }
      }
    }
  ],
  "returns": [
    {
      "name": "users_total",
      "ty": "BigInt"
    },
    {
      "name": "users",
      "ty": {
        "DataTable": {
          "name": "ListUserRow",
          "fields": [
            {
              "name": "user_id",
              "ty": "BigInt"
            },
            {
              "name": "public_user_id",
              "ty": "BigInt"
            },
            {
              "name": "username",
              "ty": {
                "Optional": "String"
              }
            },
            {
              "name": "address",
              "ty": "BlockchainAddress"
            },
            {
              "name": "last_ip",
              "ty": "Inet"
            },
            {
              "name": "last_login_at",
              "ty": "BigInt"
            },
            {
              "name": "login_count",
              "ty": "Int"
            },
            {
              "name": "role",
              "ty": {
                "EnumRef": "role"
              }
            },
            {
              "name": "email",
              "ty": {
                "Optional": "String"
              }
            },
            {
              "name": "updated_at",
              "ty": "BigInt"
            },
            {
              "name": "created_at",
              "ty": "BigInt"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for AdminListUsersResponse {
    type Request = AdminListUsersRequest;
}

impl WsRequest for AdminSetUserRoleRequest {
    type Response = AdminSetUserRoleResponse;
    const METHOD_ID: u32 = 30020;
    const SCHEMA: &'static str = r#"{
  "name": "AdminSetUserRole",
  "code": 30020,
  "parameters": [
    {
      "name": "user_id",
      "ty": "BigInt"
    },
    {
      "name": "role",
      "ty": {
        "EnumRef": "role"
      }
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for AdminSetUserRoleResponse {
    type Request = AdminSetUserRoleRequest;
}

impl WsRequest for AdminSetBlockUserRequest {
    type Response = AdminSetBlockUserResponse;
    const METHOD_ID: u32 = 30030;
    const SCHEMA: &'static str = r#"{
  "name": "AdminSetBlockUser",
  "code": 30030,
  "parameters": [
    {
      "name": "user_id",
      "ty": "BigInt"
    },
    {
      "name": "blocked",
      "ty": "Boolean"
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for AdminSetBlockUserResponse {
    type Request = AdminSetBlockUserRequest;
}

impl WsRequest for AdminListPendingExpertApplicationsRequest {
    type Response = AdminListPendingExpertApplicationsResponse;
    const METHOD_ID: u32 = 30060;
    const SCHEMA: &'static str = r#"{
  "name": "AdminListPendingExpertApplications",
  "code": 30060,
  "parameters": [
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "returns": [
    {
      "name": "users_total",
      "ty": "BigInt"
    },
    {
      "name": "users",
      "ty": {
        "DataTable": {
          "name": "ListPendingExpertApplicationsRow",
          "fields": [
            {
              "name": "user_id",
              "ty": "BigInt"
            },
            {
              "name": "name",
              "ty": "String"
            },
            {
              "name": "linked_wallet",
              "ty": "BlockchainAddress"
            },
            {
              "name": "joined_at",
              "ty": "BigInt"
            },
            {
              "name": "requested_at",
              "ty": "BigInt"
            },
            {
              "name": "follower_count",
              "ty": "Int"
            },
            {
              "name": "description",
              "ty": "String"
            },
            {
              "name": "social_media",
              "ty": "String"
            },
            {
              "name": "risk_score",
              "ty": "Numeric"
            },
            {
              "name": "reputation_score",
              "ty": "Numeric"
            },
            {
              "name": "aum",
              "ty": "Numeric"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "Admin approves a user to become an expert",
  "json_schema": null
}"#;
}
impl WsResponse for AdminListPendingExpertApplicationsResponse {
    type Request = AdminListPendingExpertApplicationsRequest;
}

impl WsRequest for AdminApproveUserBecomeExpertRequest {
    type Response = AdminApproveUserBecomeExpertResponse;
    const METHOD_ID: u32 = 30040;
    const SCHEMA: &'static str = r#"{
  "name": "AdminApproveUserBecomeExpert",
  "code": 30040,
  "parameters": [
    {
      "name": "user_id",
      "ty": "BigInt"
    }
  ],
  "returns": [
    {
      "name": "success",
      "ty": "Boolean"
    }
  ],
  "stream_response": null,
  "description": "Admin approves a user to become an expert",
  "json_schema": null
}"#;
}
impl WsResponse for AdminApproveUserBecomeExpertResponse {
    type Request = AdminApproveUserBecomeExpertRequest;
}

impl WsRequest for AdminRejectUserBecomeExpertRequest {
    type Response = AdminRejectUserBecomeExpertResponse;
    const METHOD_ID: u32 = 30050;
    const SCHEMA: &'static str = r#"{
  "name": "AdminRejectUserBecomeExpert",
  "code": 30050,
  "parameters": [
    {
      "name": "user_id",
      "ty": "BigInt"
    }
  ],
  "returns": [
    {
      "name": "success",
      "ty": "Boolean"
    }
  ],
  "stream_response": null,
  "description": "Admin approves a user to become an expert",
  "json_schema": null
}"#;
}
impl WsResponse for AdminRejectUserBecomeExpertResponse {
    type Request = AdminRejectUserBecomeExpertRequest;
}

impl WsRequest for AdminGetSystemConfigRequest {
    type Response = AdminGetSystemConfigResponse;
    const METHOD_ID: u32 = 30070;
    const SCHEMA: &'static str = r#"{
  "name": "AdminGetSystemConfig",
  "code": 30070,
  "parameters": [],
  "returns": [
    {
      "name": "platform_fee",
      "ty": "Numeric"
    },
    {
      "name": "config_placeholder_2",
      "ty": "BigInt"
    }
  ],
  "stream_response": null,
  "description": "Admin get system config",
  "json_schema": null
}"#;
}
impl WsResponse for AdminGetSystemConfigResponse {
    type Request = AdminGetSystemConfigRequest;
}

impl WsRequest for AdminUpdateSystemConfigRequest {
    type Response = AdminUpdateSystemConfigResponse;
    const METHOD_ID: u32 = 30080;
    const SCHEMA: &'static str = r#"{
  "name": "AdminUpdateSystemConfig",
  "code": 30080,
  "parameters": [
    {
      "name": "platform_fee",
      "ty": {
        "Optional": "Numeric"
      }
    },
    {
      "name": "config_placeholder_2",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "returns": [
    {
      "name": "success",
      "ty": "Boolean"
    }
  ],
  "stream_response": null,
  "description": "Admin updates system config",
  "json_schema": null
}"#;
}
impl WsResponse for AdminUpdateSystemConfigResponse {
    type Request = AdminUpdateSystemConfigRequest;
}

impl WsRequest for AdminListExpertsRequest {
    type Response = AdminListExpertsResponse;
    const METHOD_ID: u32 = 30090;
    const SCHEMA: &'static str = r#"{
  "name": "AdminListExperts",
  "code": 30090,
  "parameters": [
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "expert_id",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "user_id",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "user_public_id",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "username",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "family_name",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "given_name",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "description",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "social_media",
      "ty": {
        "Optional": "String"
      }
    }
  ],
  "returns": [
    {
      "name": "experts_total",
      "ty": "BigInt"
    },
    {
      "name": "experts",
      "ty": {
        "Vec": {
          "Struct": {
            "name": "ListExpertsRow",
            "fields": [
              {
                "name": "expert_id",
                "ty": "BigInt"
              },
              {
                "name": "user_public_id",
                "ty": "BigInt"
              },
              {
                "name": "linked_wallet",
                "ty": "BlockchainAddress"
              },
              {
                "name": "name",
                "ty": "String"
              },
              {
                "name": "family_name",
                "ty": {
                  "Optional": "String"
                }
              },
              {
                "name": "given_name",
                "ty": {
                  "Optional": "String"
                }
              },
              {
                "name": "follower_count",
                "ty": "BigInt"
              },
              {
                "name": "backer_count",
                "ty": "BigInt"
              },
              {
                "name": "description",
                "ty": "String"
              },
              {
                "name": "social_media",
                "ty": "String"
              },
              {
                "name": "risk_score",
                "ty": "Numeric"
              },
              {
                "name": "reputation_score",
                "ty": "Numeric"
              },
              {
                "name": "consistent_score",
                "ty": "Numeric"
              },
              {
                "name": "aum",
                "ty": "Numeric"
              },
              {
                "name": "joined_at",
                "ty": "BigInt"
              },
              {
                "name": "requested_at",
                "ty": "BigInt"
              },
              {
                "name": "approved_at",
                "ty": {
                  "Optional": "BigInt"
                }
              },
              {
                "name": "pending_expert",
                "ty": "Boolean"
              },
              {
                "name": "approved_expert",
                "ty": "Boolean"
              },
              {
                "name": "followed",
                "ty": "Boolean"
              }
            ]
          }
        }
      }
    }
  ],
  "stream_response": null,
  "description": "Admin lists experts",
  "json_schema": null
}"#;
}
impl WsResponse for AdminListExpertsResponse {
    type Request = AdminListExpertsRequest;
}

impl WsRequest for AdminListBackersRequest {
    type Response = AdminListBackersResponse;
    const METHOD_ID: u32 = 30100;
    const SCHEMA: &'static str = r#"{
  "name": "AdminListBackers",
  "code": 30100,
  "parameters": [
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "user_id",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "user_public_id",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "username",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "family_name",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "given_name",
      "ty": {
        "Optional": "String"
      }
    }
  ],
  "returns": [
    {
      "name": "backers_total",
      "ty": "BigInt"
    },
    {
      "name": "backers",
      "ty": {
        "DataTable": {
          "name": "AdminListBackersRow",
          "fields": [
            {
              "name": "username",
              "ty": "String"
            },
            {
              "name": "user_id",
              "ty": "BigInt"
            },
            {
              "name": "login_wallet_address",
              "ty": "BlockchainAddress"
            },
            {
              "name": "joined_at",
              "ty": "BigInt"
            },
            {
              "name": "total_platform_fee_paid",
              "ty": "Numeric"
            },
            {
              "name": "total_strategy_fee_paid",
              "ty": "Numeric"
            },
            {
              "name": "total_backing_amount",
              "ty": "Numeric"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for AdminListBackersResponse {
    type Request = AdminListBackersRequest;
}

impl WsRequest for AdminListStrategiesRequest {
    type Response = AdminListStrategiesResponse;
    const METHOD_ID: u32 = 30110;
    const SCHEMA: &'static str = r#"{
  "name": "AdminListStrategies",
  "code": 30110,
  "parameters": [
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "strategy_id",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "strategy_name",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "expert_public_id",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "expert_name",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "description",
      "ty": {
        "Optional": "String"
      }
    },
    {
      "name": "pending_approval",
      "ty": {
        "Optional": "Boolean"
      }
    },
    {
      "name": "approved",
      "ty": {
        "Optional": "Boolean"
      }
    }
  ],
  "returns": [
    {
      "name": "strategies_total",
      "ty": "BigInt"
    },
    {
      "name": "strategies",
      "ty": {
        "Vec": {
          "Struct": {
            "name": "ListStrategiesRow",
            "fields": [
              {
                "name": "strategy_id",
                "ty": "BigInt"
              },
              {
                "name": "strategy_name",
                "ty": "String"
              },
              {
                "name": "strategy_description",
                "ty": "String"
              },
              {
                "name": "followers",
                "ty": "Int"
              },
              {
                "name": "backers",
                "ty": "Int"
              },
              {
                "name": "aum",
                "ty": "Numeric"
              },
              {
                "name": "followed",
                "ty": "Boolean"
              },
              {
                "name": "swap_price",
                "ty": "Numeric"
              },
              {
                "name": "price_change",
                "ty": "Numeric"
              },
              {
                "name": "strategy_pool_address",
                "ty": {
                  "Optional": "BlockchainAddress"
                }
              },
              {
                "name": "approved",
                "ty": "Boolean"
              },
              {
                "name": "approved_at",
                "ty": {
                  "Optional": "BigInt"
                }
              },
              {
                "name": "blockchain",
                "ty": {
                  "EnumRef": "block_chain"
                }
              },
              {
                "name": "requested_at",
                "ty": {
                  "Optional": "BigInt"
                }
              },
              {
                "name": "created_at",
                "ty": "BigInt"
              },
              {
                "name": "expert_public_id",
                "ty": "BigInt"
              },
              {
                "name": "expert_username",
                "ty": "String"
              },
              {
                "name": "expert_family_name",
                "ty": "String"
              },
              {
                "name": "expert_given_name",
                "ty": "String"
              },
              {
                "name": "reputation",
                "ty": "Int"
              },
              {
                "name": "risk_score",
                "ty": "Numeric"
              },
              {
                "name": "strategy_pool_token",
                "ty": "String"
              },
              {
                "name": "strategy_fee",
                "ty": "Numeric"
              },
              {
                "name": "platform_fee",
                "ty": "Numeric"
              },
              {
                "name": "expert_fee",
                "ty": "Numeric"
              },
              {
                "name": "swap_fee",
                "ty": "Numeric"
              },
              {
                "name": "total_fee",
                "ty": "Numeric"
              },
              {
                "name": "number_of_tokens",
                "ty": "BigInt"
              }
            ]
          }
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for AdminListStrategiesResponse {
    type Request = AdminListStrategiesRequest;
}

impl WsRequest for AdminApproveStrategyRequest {
    type Response = AdminApproveStrategyResponse;
    const METHOD_ID: u32 = 30120;
    const SCHEMA: &'static str = r#"{
  "name": "AdminApproveStrategy",
  "code": 30120,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": "BigInt"
    }
  ],
  "returns": [
    {
      "name": "success",
      "ty": "Boolean"
    }
  ],
  "stream_response": null,
  "description": "Admin approves strategy",
  "json_schema": null
}"#;
}
impl WsResponse for AdminApproveStrategyResponse {
    type Request = AdminApproveStrategyRequest;
}

impl WsRequest for AdminRejectStrategyRequest {
    type Response = AdminRejectStrategyResponse;
    const METHOD_ID: u32 = 30130;
    const SCHEMA: &'static str = r#"{
  "name": "AdminRejectStrategy",
  "code": 30130,
  "parameters": [
    {
      "name": "strategy_id",
      "ty": "BigInt"
    }
  ],
  "returns": [
    {
      "name": "success",
      "ty": "Boolean"
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for AdminRejectStrategyResponse {
    type Request = AdminRejectStrategyRequest;
}

impl WsRequest for AdminAddAuditRuleRequest {
    type Response = AdminAddAuditRuleResponse;
    const METHOD_ID: u32 = 31002;
    const SCHEMA: &'static str = r#"{
  "name": "AdminAddAuditRule",
  "code": 31002,
  "parameters": [
    {
      "name": "rule_id",
      "ty": "BigInt"
    },
    {
      "name": "name",
      "ty": "String"
    },
    {
      "name": "description",
      "ty": "String"
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for AdminAddAuditRuleResponse {
    type Request = AdminAddAuditRuleRequest;
}

impl WsRequest for AdminNotifyEscrowLedgerChangeRequest {
    type Response = AdminNotifyEscrowLedgerChangeResponse;
    const METHOD_ID: u32 = 32010;
    const SCHEMA: &'static str = r#"{
  "name": "AdminNotifyEscrowLedgerChange",
  "code": 32010,
  "parameters": [
    {
      "name": "pkey_id",
      "ty": "BigInt"
    },
    {
      "name": "user_id",
      "ty": "BigInt"
    },
    {
      "name": "balance",
      "ty": {
        "Struct": {
          "name": "UserListDepositLedgerRow",
          "fields": [
            {
              "name": "transaction_id",
              "ty": "BigInt"
            },
            {
              "name": "blockchain",
              "ty": {
                "EnumRef": "block_chain"
              }
            },
            {
              "name": "user_address",
              "ty": "BlockchainAddress"
            },
            {
              "name": "contract_address",
              "ty": "BlockchainAddress"
            },
            {
              "name": "receiver_address",
              "ty": "BlockchainAddress"
            },
            {
              "name": "quantity",
              "ty": "BlockchainDecimal"
            },
            {
              "name": "transaction_hash",
              "ty": "BlockchainTransactionHash"
            },
            {
              "name": "is_deposit",
              "ty": "Boolean"
            },
            {
              "name": "happened_at",
              "ty": "BigInt"
            }
          ]
        }
      }
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for AdminNotifyEscrowLedgerChangeResponse {
    type Request = AdminNotifyEscrowLedgerChangeRequest;
}

impl WsRequest for AdminSubscribeDepositLedgerRequest {
    type Response = AdminSubscribeDepositLedgerResponse;
    const METHOD_ID: u32 = 32011;
    const SCHEMA: &'static str = r#"{
  "name": "AdminSubscribeDepositLedger",
  "code": 32011,
  "parameters": [
    {
      "name": "initial_data",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "blockchain",
      "ty": {
        "Optional": {
          "EnumRef": "block_chain"
        }
      }
    },
    {
      "name": "mock_data",
      "ty": {
        "Optional": "Boolean"
      }
    }
  ],
  "returns": [],
  "stream_response": {
    "Struct": {
      "name": "UserListDepositLedgerRow",
      "fields": [
        {
          "name": "transaction_id",
          "ty": "BigInt"
        },
        {
          "name": "blockchain",
          "ty": {
            "EnumRef": "block_chain"
          }
        },
        {
          "name": "user_address",
          "ty": "BlockchainAddress"
        },
        {
          "name": "contract_address",
          "ty": "BlockchainAddress"
        },
        {
          "name": "receiver_address",
          "ty": "BlockchainAddress"
        },
        {
          "name": "quantity",
          "ty": "BlockchainDecimal"
        },
        {
          "name": "transaction_hash",
          "ty": "BlockchainTransactionHash"
        },
        {
          "name": "is_deposit",
          "ty": "Boolean"
        },
        {
          "name": "happened_at",
          "ty": "BigInt"
        }
      ]
    }
  },
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for AdminSubscribeDepositLedgerResponse {
    type Request = AdminSubscribeDepositLedgerRequest;
}

impl WsRequest for AdminUnsubscribeDepositLedgerRequest {
    type Response = AdminUnsubscribeDepositLedgerResponse;
    const METHOD_ID: u32 = 32012;
    const SCHEMA: &'static str = r#"{
  "name": "AdminUnsubscribeDepositLedger",
  "code": 32012,
  "parameters": [],
  "returns": [],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for AdminUnsubscribeDepositLedgerResponse {
    type Request = AdminUnsubscribeDepositLedgerRequest;
}

impl WsRequest for AdminAddEscrowTokenContractAddressRequest {
    type Response = AdminAddEscrowTokenContractAddressResponse;
    const METHOD_ID: u32 = 32020;
    const SCHEMA: &'static str = r#"{
  "name": "AdminAddEscrowTokenContractAddress",
  "code": 32020,
  "parameters": [
    {
      "name": "pkey_id",
      "ty": "BigInt"
    },
    {
      "name": "symbol",
      "ty": "String"
    },
    {
      "name": "short_name",
      "ty": "String"
    },
    {
      "name": "description",
      "ty": "String"
    },
    {
      "name": "address",
      "ty": "BlockchainAddress"
    },
    {
      "name": "blockchain",
      "ty": {
        "EnumRef": "block_chain"
      }
    },
    {
      "name": "is_stablecoin",
      "ty": "Boolean"
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for AdminAddEscrowTokenContractAddressResponse {
    type Request = AdminAddEscrowTokenContractAddressRequest;
}

impl WsRequest for AdminAddEscrowContractAddressRequest {
    type Response = AdminAddEscrowContractAddressResponse;
    const METHOD_ID: u32 = 32030;
    const SCHEMA: &'static str = r#"{
  "name": "AdminAddEscrowContractAddress",
  "code": 32030,
  "parameters": [
    {
      "name": "pkey_id",
      "ty": "BigInt"
    },
    {
      "name": "address",
      "ty": "BlockchainAddress"
    },
    {
      "name": "blockchain",
      "ty": {
        "EnumRef": "block_chain"
      }
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for AdminAddEscrowContractAddressResponse {
    type Request = AdminAddEscrowContractAddressRequest;
}

impl WsRequest for AdminListBackStrategyLedgerRequest {
    type Response = AdminListBackStrategyLedgerResponse;
    const METHOD_ID: u32 = 32040;
    const SCHEMA: &'static str = r#"{
  "name": "AdminListBackStrategyLedger",
  "code": 32040,
  "parameters": [
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "strategy_id",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "returns": [
    {
      "name": "back_ledger_total",
      "ty": "BigInt"
    },
    {
      "name": "back_ledger",
      "ty": {
        "DataTable": {
          "name": "AdminBackStrategyLedgerRow",
          "fields": [
            {
              "name": "back_ledger_id",
              "ty": "BigInt"
            },
            {
              "name": "user_id",
              "ty": "BigInt"
            },
            {
              "name": "strategy_id",
              "ty": "BigInt"
            },
            {
              "name": "quantity",
              "ty": "BlockchainDecimal"
            },
            {
              "name": "blockchain",
              "ty": {
                "EnumRef": "block_chain"
              }
            },
            {
              "name": "transaction_hash",
              "ty": "BlockchainTransactionHash"
            },
            {
              "name": "happened_at",
              "ty": "BigInt"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for AdminListBackStrategyLedgerResponse {
    type Request = AdminListBackStrategyLedgerRequest;
}

impl WsRequest for AdminListExitStrategyLedgerRequest {
    type Response = AdminListExitStrategyLedgerResponse;
    const METHOD_ID: u32 = 32041;
    const SCHEMA: &'static str = r#"{
  "name": "AdminListExitStrategyLedger",
  "code": 32041,
  "parameters": [
    {
      "name": "limit",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "offset",
      "ty": {
        "Optional": "BigInt"
      }
    },
    {
      "name": "strategy_id",
      "ty": {
        "Optional": "BigInt"
      }
    }
  ],
  "returns": [
    {
      "name": "exit_ledger_total",
      "ty": "BigInt"
    },
    {
      "name": "exit_ledger",
      "ty": {
        "DataTable": {
          "name": "AdminExitStrategyLedgerRow",
          "fields": [
            {
              "name": "back_ledger_id",
              "ty": "BigInt"
            },
            {
              "name": "user_id",
              "ty": "BigInt"
            },
            {
              "name": "strategy_id",
              "ty": "BigInt"
            },
            {
              "name": "quantity",
              "ty": "BlockchainDecimal"
            },
            {
              "name": "blockchain",
              "ty": {
                "EnumRef": "block_chain"
              }
            },
            {
              "name": "transaction_hash",
              "ty": "BlockchainTransactionHash"
            },
            {
              "name": "happened_at",
              "ty": "BigInt"
            }
          ]
        }
      }
    }
  ],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for AdminListExitStrategyLedgerResponse {
    type Request = AdminListExitStrategyLedgerRequest;
}

impl WsRequest for AdminSetBlockchainLoggerRequest {
    type Response = AdminSetBlockchainLoggerResponse;
    const METHOD_ID: u32 = 32050;
    const SCHEMA: &'static str = r#"{
  "name": "AdminSetBlockchainLogger",
  "code": 32050,
  "parameters": [
    {
      "name": "enabled",
      "ty": "Boolean"
    }
  ],
  "returns": [],
  "stream_response": null,
  "description": "",
  "json_schema": null
}"#;
}
impl WsResponse for AdminSetBlockchainLoggerResponse {
    type Request = AdminSetBlockchainLoggerRequest;
}
