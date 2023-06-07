use lib::error_code::ErrorCode;
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
    #[postgres(name = "admin")]
    Admin = 2,
    ///
    #[postgres(name = "expert")]
    Expert = 3,
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
    #[postgres(name = "UserGetStrategy")]
    UserGetStrategy = 20062,
    ///
    #[postgres(name = "UserGetStrategyStatistics")]
    UserGetStrategyStatistics = 20070,
    ///
    #[postgres(name = "UserGetStrategiesStatistics")]
    UserGetStrategiesStatistics = 20071,
    ///
    #[postgres(name = "UserBackStrategy")]
    UserBackStrategy = 20080,
    ///
    #[postgres(name = "UserRequestRefund")]
    UserRequestRefund = 20081,
    ///
    #[postgres(name = "UserListBackedStrategies")]
    UserListBackedStrategies = 20090,
    ///
    #[postgres(name = "UserListBackStrategyHistory")]
    UserListBackStrategyHistory = 20100,
    ///
    #[postgres(name = "UserListExitStrategyHistory")]
    UserListExitStrategyHistory = 20120,
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
    #[postgres(name = "UserRegisterWallet")]
    UserRegisterWallet = 20190,
    ///
    #[postgres(name = "UserListRegisteredWallets")]
    UserListRegisteredWallets = 20200,
    ///
    #[postgres(name = "UserDeregisterWallet")]
    UserDeregisterWallet = 20210,
    ///
    #[postgres(name = "UserApplyBecomeExpert")]
    UserApplyBecomeExpert = 20220,
    ///
    #[postgres(name = "UserCreateStrategy")]
    UserCreateStrategy = 20250,
    ///
    #[postgres(name = "UserUpdateStrategy")]
    UserUpdateStrategy = 20260,
    ///
    #[postgres(name = "UserAddStrategyWatchingWallet")]
    UserAddStrategyWatchingWallet = 20270,
    ///
    #[postgres(name = "UserRemoveStrategyWatchingWallet")]
    UserRemoveStrategyWatchingWallet = 20280,
    ///
    #[postgres(name = "UserListStrategyWatchingWallets")]
    UserListStrategyWatchingWallets = 20290,
    ///
    #[postgres(name = "UserListWalletActivityHistory")]
    UserListWalletActivityHistory = 20300,
    ///
    #[postgres(name = "UserAddStrategyInitialTokenRatio")]
    UserAddStrategyInitialTokenRatio = 20310,
    ///
    #[postgres(name = "UserRemoveStrategyInitialTokenRatio")]
    UserRemoveStrategyInitialTokenRatio = 20320,
    ///
    #[postgres(name = "UserListStrategyInitialTokenRatio")]
    UserListStrategyInitialTokenRatio = 20330,
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
    AdminListPendingExpertApplications = 20240,
    ///
    #[postgres(name = "AdminApproveUserBecomeExpert")]
    AdminApproveUserBecomeExpert = 20230,
    ///
    #[postgres(name = "AdminRejectUserBecomeExpert")]
    AdminRejectUserBecomeExpert = 20231,
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
pub struct ErrorUserNoValidSalt {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserNoAuthToken {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserInvalidAuthToken {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorOrganizationForbidden {
    pub user: String,
    pub organization: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorOrganizationNotFound {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorOrganizationAssignRoleForbiddenAdmin {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorOrganizationAssignRoleForbiddenSelf {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorOrganizationMembershipNotFound {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidEnumLevel {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorAssetNotFound {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorAssetPlanNotFound {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorVaultWalletNotFound {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorVaultNotFound {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorBucketNotFound {}
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
    /// Custom No valid salt
    #[postgres(name = "UserNoValidSalt")]
    UserNoValidSalt = 101603,
    /// Custom No auth token
    #[postgres(name = "UserNoAuthToken")]
    UserNoAuthToken = 101604,
    /// Custom token invalid
    #[postgres(name = "UserInvalidAuthToken")]
    UserInvalidAuthToken = 101605,
    /// Custom Insufficient role for {user} in organization {organization}
    #[postgres(name = "OrganizationForbidden")]
    OrganizationForbidden = 102403,
    /// Custom Organization Not Found
    #[postgres(name = "OrganizationNotFound")]
    OrganizationNotFound = 102404,
    /// Custom Cannot set role higher than admin
    #[postgres(name = "OrganizationAssignRoleForbiddenAdmin")]
    OrganizationAssignRoleForbiddenAdmin = 102601,
    /// Custom Cannot set role higher than your role
    #[postgres(name = "OrganizationAssignRoleForbiddenSelf")]
    OrganizationAssignRoleForbiddenSelf = 102602,
    /// Custom User is not a member in organization
    #[postgres(name = "OrganizationMembershipNotFound")]
    OrganizationMembershipNotFound = 102603,
    /// SQL 22P02 InvalidEnumLevel
    #[postgres(name = "InvalidEnumLevel")]
    InvalidEnumLevel = 3484946,
    /// Custom Asset Not Found
    #[postgres(name = "AssetNotFound")]
    AssetNotFound = 103404,
    /// Custom AssetPlan Not Found
    #[postgres(name = "AssetPlanNotFound")]
    AssetPlanNotFound = 104404,
    /// Custom Vault wallet Not Found
    #[postgres(name = "VaultWalletNotFound")]
    VaultWalletNotFound = 105404,
    /// Custom Vault Not Found
    #[postgres(name = "VaultNotFound")]
    VaultNotFound = 106404,
    /// Custom Bucket Not Found
    #[postgres(name = "BucketNotFound")]
    BucketNotFound = 107404,
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
pub struct AdminListPendingExpertApplicationsRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminListPendingExpertApplicationsResponse {
    pub users: Vec<ListPendingExpertApplicationsRow>,
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
    pub users: Vec<ListUserRow>,
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
pub struct AdminSetUserRoleRequest {
    pub user_id: i64,
    pub role: EnumRole,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminSetUserRoleResponse {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AumHistoryRow {
    pub aum_history_id: i64,
    pub base_token: String,
    pub quote_token: String,
    pub blockchain: EnumBlockChain,
    pub dex: String,
    pub action: String,
    pub wallet_address: String,
    pub price: f64,
    pub current_price: f64,
    pub quantity: f64,
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
pub struct BackHistoryPoint {
    pub time: i64,
    pub backer_count: f64,
    pub backer_quantity_usd: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BackStrategyHistoryRow {
    pub back_history_id: i64,
    pub strategy_id: i64,
    pub quantity: String,
    pub blockchain: EnumBlockChain,
    pub dex: String,
    pub transaction_hash: String,
    pub time: i64,
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
pub struct ExitStrategyHistoryRow {
    pub exit_history_id: i64,
    pub strategy_id: i64,
    pub exit_quantity: String,
    pub purchase_wallet_address: String,
    pub blockchain: EnumBlockChain,
    pub dex: String,
    pub back_time: i64,
    pub exit_time: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FollowHistoryPoint {
    pub time: i64,
    pub follower_count: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListExpertsRow {
    pub expert_id: i64,
    pub name: String,
    pub follower_count: i32,
    pub description: String,
    pub social_media: String,
    pub risk_score: f64,
    pub reputation_score: f64,
    pub aum: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListPendingExpertApplicationsRow {
    pub user_id: i64,
    pub name: String,
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
    pub net_value: f64,
    pub followers: i32,
    pub backers: i32,
    pub risk_score: f64,
    pub aum: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListStrategyInitialTokenRatioRow {
    pub token_id: i64,
    pub token_name: String,
    pub token_address: String,
    pub quantity: String,
    pub updated_at: i64,
    pub created_at: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListStrategyWatchingWalletsRow {
    pub wallet_id: i64,
    pub blockchain: EnumBlockChain,
    pub wallet_address: String,
    pub ratio: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListUserRow {
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListWalletActivityHistoryRow {
    pub record_id: i64,
    pub wallet_address: String,
    pub transaction_hash: String,
    pub dex: String,
    pub blockchain: EnumBlockChain,
    pub contract_address: String,
    pub token_in_address: String,
    pub token_out_address: String,
    pub caller_address: String,
    pub amount_in: String,
    pub amount_out: String,
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
    pub wallet_address: String,
    pub is_default: bool,
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
pub struct UserAddStrategyInitialTokenRatioRequest {
    pub strategy_id: i64,
    pub token_name: String,
    pub token_address: String,
    pub quantity: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserAddStrategyInitialTokenRatioResponse {
    pub success: bool,
    pub token_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserAddStrategyWatchingWalletRequest {
    pub strategy_id: i64,
    pub blockchain: EnumBlockChain,
    pub wallet_address: String,
    pub ratio: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserAddStrategyWatchingWalletResponse {
    pub success: bool,
    pub wallet_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserApplyBecomeExpertRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserApplyBecomeExpertResponse {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserBackStrategyRequest {
    pub strategy_id: i64,
    pub quantity: String,
    pub blockchain: EnumBlockChain,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserBackStrategyResponse {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserCreateStrategyRequest {
    pub name: String,
    pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserCreateStrategyResponse {
    pub success: bool,
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDeregisterWalletRequest {
    pub wallet_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDeregisterWalletResponse {
    pub success: bool,
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
pub struct UserGetExpertProfileRequest {
    pub expert_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetExpertProfileResponse {
    pub expert_id: i64,
    pub name: String,
    pub follower_count: i32,
    pub description: String,
    pub social_media: String,
    pub risk_score: f64,
    pub reputation_score: f64,
    pub aum: f64,
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
    pub strategy_id: i64,
    pub strategy_name: String,
    pub strategy_description: String,
    pub creator_user_id: i64,
    pub social_media: String,
    pub historical_return: f64,
    pub inception_time: i64,
    pub total_amount: f64,
    pub token_allocation: i64,
    pub reputation: i32,
    pub risk_score: f64,
    pub aum: f64,
    pub net_value: f64,
    pub followers: i32,
    pub backers: i32,
    pub watching_wallets: Vec<WatchingWalletRow>,
    pub aum_history: Vec<AumHistoryRow>,
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
    pub follow_history: Vec<FollowHistoryPoint>,
    pub back_history: Vec<BackHistoryPoint>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetUserProfileRequest {
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserGetUserProfileResponse {
    pub user_id: i64,
    pub name: String,
    pub follower_count: i32,
    pub description: String,
    pub social_media: String,
    pub followed_experts: Vec<ListExpertsRow>,
    pub followed_strategies: Vec<ListStrategiesRow>,
    pub backed_strategies: Vec<ListStrategiesRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListBackStrategyHistoryRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListBackStrategyHistoryResponse {
    pub back_history: Vec<BackStrategyHistoryRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListBackedStrategiesRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListBackedStrategiesResponse {
    pub strategies: Vec<ListStrategiesRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListExitStrategyHistoryRequest {
    #[serde(default)]
    pub strategy_id: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListExitStrategyHistoryResponse {
    pub exit_history: Vec<ExitStrategyHistoryRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListExpertsRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListExpertsResponse {
    pub experts: Vec<ListExpertsRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListFeaturedExpertsRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListFeaturedExpertsResponse {
    pub experts: Vec<ListExpertsRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListFollowedExpertsRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListFollowedExpertsResponse {
    pub experts: Vec<ListExpertsRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListFollowedStrategiesRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListFollowedStrategiesResponse {
    pub strategies: Vec<ListStrategiesRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListRegisteredWalletsRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListRegisteredWalletsResponse {
    pub wallets: Vec<ListWalletsRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategiesRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategiesResponse {
    pub strategies: Vec<ListStrategiesRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategyInitialTokenRatioRequest {
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategyInitialTokenRatioResponse {
    pub token_ratios: Vec<ListStrategyInitialTokenRatioRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategyWatchingWalletsRequest {
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListStrategyWatchingWalletsResponse {
    pub wallets: Vec<ListStrategyWatchingWalletsRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListTopPerformingExpertsRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListTopPerformingExpertsResponse {
    pub experts: Vec<ListExpertsRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListTopPerformingStrategiesRequest {}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListTopPerformingStrategiesResponse {
    pub strategies: Vec<ListStrategiesRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListWalletActivityHistoryRequest {
    pub wallet_address: String,
    pub blockchain: EnumBlockChain,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserListWalletActivityHistoryResponse {
    pub wallet_activities: Vec<ListWalletActivityHistoryRow>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserRegisterWalletRequest {
    pub blockchain: EnumBlockChain,
    pub wallet_address: String,
    pub message_to_sign: String,
    pub message_signature: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserRegisterWalletResponse {
    pub success: bool,
    pub wallet_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserRemoveStrategyInitialTokenRatioRequest {
    pub strategy_id: i64,
    pub token_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserRemoveStrategyInitialTokenRatioResponse {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserRemoveStrategyWatchingWalletRequest {
    pub wallet_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserRemoveStrategyWatchingWalletResponse {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserRequestRefundRequest {
    pub quantity: String,
    pub wallet_address: String,
    pub blockchain: EnumBlockChain,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserRequestRefundResponse {
    pub success: bool,
}
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
pub struct UserUpdateStrategyRequest {
    pub strategy_id: i64,
    #[serde(default)]
    pub name: Option<String>,
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateStrategyResponse {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WatchingWalletRow {
    pub watching_wallet_id: i64,
    pub wallet_address: String,
    pub blockchain: EnumBlockChain,
    pub dex: String,
    pub ratio_distribution: f64,
}
impl WsRequest for LoginRequest {
    type Response = LoginResponse;
    const METHOD_ID: u32 = 10020;
}
impl WsResponse for LoginResponse {
    type Request = LoginRequest;
}

impl WsRequest for SignupRequest {
    type Response = SignupResponse;
    const METHOD_ID: u32 = 10010;
}
impl WsResponse for SignupResponse {
    type Request = SignupRequest;
}

impl WsRequest for AuthorizeRequest {
    type Response = AuthorizeResponse;
    const METHOD_ID: u32 = 10030;
}
impl WsResponse for AuthorizeResponse {
    type Request = AuthorizeRequest;
}

impl WsRequest for LogoutRequest {
    type Response = LogoutResponse;
    const METHOD_ID: u32 = 10040;
}
impl WsResponse for LogoutResponse {
    type Request = LogoutRequest;
}

impl WsRequest for ChangeLoginWalletRequest {
    type Response = ChangeLoginWalletResponse;
    const METHOD_ID: u32 = 10050;
}
impl WsResponse for ChangeLoginWalletResponse {
    type Request = ChangeLoginWalletRequest;
}

impl WsRequest for UserFollowStrategyRequest {
    type Response = UserFollowStrategyResponse;
    const METHOD_ID: u32 = 20040;
}
impl WsResponse for UserFollowStrategyResponse {
    type Request = UserFollowStrategyRequest;
}

impl WsRequest for UserListFollowedStrategiesRequest {
    type Response = UserListFollowedStrategiesResponse;
    const METHOD_ID: u32 = 20050;
}
impl WsResponse for UserListFollowedStrategiesResponse {
    type Request = UserListFollowedStrategiesRequest;
}

impl WsRequest for UserUnfollowStrategyRequest {
    type Response = UserUnfollowStrategyResponse;
    const METHOD_ID: u32 = 20060;
}
impl WsResponse for UserUnfollowStrategyResponse {
    type Request = UserUnfollowStrategyRequest;
}

impl WsRequest for UserListStrategiesRequest {
    type Response = UserListStrategiesResponse;
    const METHOD_ID: u32 = 20061;
}
impl WsResponse for UserListStrategiesResponse {
    type Request = UserListStrategiesRequest;
}

impl WsRequest for UserListTopPerformingStrategiesRequest {
    type Response = UserListTopPerformingStrategiesResponse;
    const METHOD_ID: u32 = 20063;
}
impl WsResponse for UserListTopPerformingStrategiesResponse {
    type Request = UserListTopPerformingStrategiesRequest;
}

impl WsRequest for UserGetStrategyRequest {
    type Response = UserGetStrategyResponse;
    const METHOD_ID: u32 = 20062;
}
impl WsResponse for UserGetStrategyResponse {
    type Request = UserGetStrategyRequest;
}

impl WsRequest for UserGetStrategyStatisticsRequest {
    type Response = UserGetStrategyStatisticsResponse;
    const METHOD_ID: u32 = 20070;
}
impl WsResponse for UserGetStrategyStatisticsResponse {
    type Request = UserGetStrategyStatisticsRequest;
}

impl WsRequest for UserGetStrategiesStatisticsRequest {
    type Response = UserGetStrategiesStatisticsResponse;
    const METHOD_ID: u32 = 20071;
}
impl WsResponse for UserGetStrategiesStatisticsResponse {
    type Request = UserGetStrategiesStatisticsRequest;
}

impl WsRequest for UserBackStrategyRequest {
    type Response = UserBackStrategyResponse;
    const METHOD_ID: u32 = 20080;
}
impl WsResponse for UserBackStrategyResponse {
    type Request = UserBackStrategyRequest;
}

impl WsRequest for UserRequestRefundRequest {
    type Response = UserRequestRefundResponse;
    const METHOD_ID: u32 = 20081;
}
impl WsResponse for UserRequestRefundResponse {
    type Request = UserRequestRefundRequest;
}

impl WsRequest for UserListBackedStrategiesRequest {
    type Response = UserListBackedStrategiesResponse;
    const METHOD_ID: u32 = 20090;
}
impl WsResponse for UserListBackedStrategiesResponse {
    type Request = UserListBackedStrategiesRequest;
}

impl WsRequest for UserListBackStrategyHistoryRequest {
    type Response = UserListBackStrategyHistoryResponse;
    const METHOD_ID: u32 = 20100;
}
impl WsResponse for UserListBackStrategyHistoryResponse {
    type Request = UserListBackStrategyHistoryRequest;
}

impl WsRequest for UserListExitStrategyHistoryRequest {
    type Response = UserListExitStrategyHistoryResponse;
    const METHOD_ID: u32 = 20120;
}
impl WsResponse for UserListExitStrategyHistoryResponse {
    type Request = UserListExitStrategyHistoryRequest;
}

impl WsRequest for UserFollowExpertRequest {
    type Response = UserFollowExpertResponse;
    const METHOD_ID: u32 = 20130;
}
impl WsResponse for UserFollowExpertResponse {
    type Request = UserFollowExpertRequest;
}

impl WsRequest for UserListFollowedExpertsRequest {
    type Response = UserListFollowedExpertsResponse;
    const METHOD_ID: u32 = 20140;
}
impl WsResponse for UserListFollowedExpertsResponse {
    type Request = UserListFollowedExpertsRequest;
}

impl WsRequest for UserUnfollowExpertRequest {
    type Response = UserUnfollowExpertResponse;
    const METHOD_ID: u32 = 20150;
}
impl WsResponse for UserUnfollowExpertResponse {
    type Request = UserUnfollowExpertRequest;
}

impl WsRequest for UserListExpertsRequest {
    type Response = UserListExpertsResponse;
    const METHOD_ID: u32 = 20160;
}
impl WsResponse for UserListExpertsResponse {
    type Request = UserListExpertsRequest;
}

impl WsRequest for UserListTopPerformingExpertsRequest {
    type Response = UserListTopPerformingExpertsResponse;
    const METHOD_ID: u32 = 20161;
}
impl WsResponse for UserListTopPerformingExpertsResponse {
    type Request = UserListTopPerformingExpertsRequest;
}

impl WsRequest for UserListFeaturedExpertsRequest {
    type Response = UserListFeaturedExpertsResponse;
    const METHOD_ID: u32 = 20162;
}
impl WsResponse for UserListFeaturedExpertsResponse {
    type Request = UserListFeaturedExpertsRequest;
}

impl WsRequest for UserGetExpertProfileRequest {
    type Response = UserGetExpertProfileResponse;
    const METHOD_ID: u32 = 20170;
}
impl WsResponse for UserGetExpertProfileResponse {
    type Request = UserGetExpertProfileRequest;
}

impl WsRequest for UserGetUserProfileRequest {
    type Response = UserGetUserProfileResponse;
    const METHOD_ID: u32 = 20180;
}
impl WsResponse for UserGetUserProfileResponse {
    type Request = UserGetUserProfileRequest;
}

impl WsRequest for UserRegisterWalletRequest {
    type Response = UserRegisterWalletResponse;
    const METHOD_ID: u32 = 20190;
}
impl WsResponse for UserRegisterWalletResponse {
    type Request = UserRegisterWalletRequest;
}

impl WsRequest for UserListRegisteredWalletsRequest {
    type Response = UserListRegisteredWalletsResponse;
    const METHOD_ID: u32 = 20200;
}
impl WsResponse for UserListRegisteredWalletsResponse {
    type Request = UserListRegisteredWalletsRequest;
}

impl WsRequest for UserDeregisterWalletRequest {
    type Response = UserDeregisterWalletResponse;
    const METHOD_ID: u32 = 20210;
}
impl WsResponse for UserDeregisterWalletResponse {
    type Request = UserDeregisterWalletRequest;
}

impl WsRequest for UserApplyBecomeExpertRequest {
    type Response = UserApplyBecomeExpertResponse;
    const METHOD_ID: u32 = 20220;
}
impl WsResponse for UserApplyBecomeExpertResponse {
    type Request = UserApplyBecomeExpertRequest;
}

impl WsRequest for UserCreateStrategyRequest {
    type Response = UserCreateStrategyResponse;
    const METHOD_ID: u32 = 20250;
}
impl WsResponse for UserCreateStrategyResponse {
    type Request = UserCreateStrategyRequest;
}

impl WsRequest for UserUpdateStrategyRequest {
    type Response = UserUpdateStrategyResponse;
    const METHOD_ID: u32 = 20260;
}
impl WsResponse for UserUpdateStrategyResponse {
    type Request = UserUpdateStrategyRequest;
}

impl WsRequest for UserAddStrategyWatchingWalletRequest {
    type Response = UserAddStrategyWatchingWalletResponse;
    const METHOD_ID: u32 = 20270;
}
impl WsResponse for UserAddStrategyWatchingWalletResponse {
    type Request = UserAddStrategyWatchingWalletRequest;
}

impl WsRequest for UserRemoveStrategyWatchingWalletRequest {
    type Response = UserRemoveStrategyWatchingWalletResponse;
    const METHOD_ID: u32 = 20280;
}
impl WsResponse for UserRemoveStrategyWatchingWalletResponse {
    type Request = UserRemoveStrategyWatchingWalletRequest;
}

impl WsRequest for UserListStrategyWatchingWalletsRequest {
    type Response = UserListStrategyWatchingWalletsResponse;
    const METHOD_ID: u32 = 20290;
}
impl WsResponse for UserListStrategyWatchingWalletsResponse {
    type Request = UserListStrategyWatchingWalletsRequest;
}

impl WsRequest for UserListWalletActivityHistoryRequest {
    type Response = UserListWalletActivityHistoryResponse;
    const METHOD_ID: u32 = 20300;
}
impl WsResponse for UserListWalletActivityHistoryResponse {
    type Request = UserListWalletActivityHistoryRequest;
}

impl WsRequest for UserAddStrategyInitialTokenRatioRequest {
    type Response = UserAddStrategyInitialTokenRatioResponse;
    const METHOD_ID: u32 = 20310;
}
impl WsResponse for UserAddStrategyInitialTokenRatioResponse {
    type Request = UserAddStrategyInitialTokenRatioRequest;
}

impl WsRequest for UserRemoveStrategyInitialTokenRatioRequest {
    type Response = UserRemoveStrategyInitialTokenRatioResponse;
    const METHOD_ID: u32 = 20320;
}
impl WsResponse for UserRemoveStrategyInitialTokenRatioResponse {
    type Request = UserRemoveStrategyInitialTokenRatioRequest;
}

impl WsRequest for UserListStrategyInitialTokenRatioRequest {
    type Response = UserListStrategyInitialTokenRatioResponse;
    const METHOD_ID: u32 = 20330;
}
impl WsResponse for UserListStrategyInitialTokenRatioResponse {
    type Request = UserListStrategyInitialTokenRatioRequest;
}

impl WsRequest for AdminListUsersRequest {
    type Response = AdminListUsersResponse;
    const METHOD_ID: u32 = 30010;
}
impl WsResponse for AdminListUsersResponse {
    type Request = AdminListUsersRequest;
}

impl WsRequest for AdminSetUserRoleRequest {
    type Response = AdminSetUserRoleResponse;
    const METHOD_ID: u32 = 30020;
}
impl WsResponse for AdminSetUserRoleResponse {
    type Request = AdminSetUserRoleRequest;
}

impl WsRequest for AdminSetBlockUserRequest {
    type Response = AdminSetBlockUserResponse;
    const METHOD_ID: u32 = 30030;
}
impl WsResponse for AdminSetBlockUserResponse {
    type Request = AdminSetBlockUserRequest;
}

impl WsRequest for AdminListPendingExpertApplicationsRequest {
    type Response = AdminListPendingExpertApplicationsResponse;
    const METHOD_ID: u32 = 20240;
}
impl WsResponse for AdminListPendingExpertApplicationsResponse {
    type Request = AdminListPendingExpertApplicationsRequest;
}

impl WsRequest for AdminApproveUserBecomeExpertRequest {
    type Response = AdminApproveUserBecomeExpertResponse;
    const METHOD_ID: u32 = 20230;
}
impl WsResponse for AdminApproveUserBecomeExpertResponse {
    type Request = AdminApproveUserBecomeExpertRequest;
}

impl WsRequest for AdminRejectUserBecomeExpertRequest {
    type Response = AdminRejectUserBecomeExpertResponse;
    const METHOD_ID: u32 = 20231;
}
impl WsResponse for AdminRejectUserBecomeExpertResponse {
    type Request = AdminRejectUserBecomeExpertRequest;
}
