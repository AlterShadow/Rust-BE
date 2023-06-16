use crate::model::*;
use eyre::*;
use lib::database::*;
use serde::*;

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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthSignupRespRow {
    pub user_id: i64,
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
    fn parse_row(&self, row: Row) -> Result<FunAuthSignupRespRow> {
        let r = FunAuthSignupRespRow {
            user_id: row.try_get(0).context("failed to get field user_id")?,
        };
        Ok(r)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthAuthenticateRespRow {
    pub user_id: i64,
    pub public_user_id: i64,
    pub role: EnumRole,
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
    fn parse_row(&self, row: Row) -> Result<FunAuthAuthenticateRespRow> {
        let r = FunAuthAuthenticateRespRow {
            user_id: row.try_get(0).context("failed to get field user_id")?,
            public_user_id: row
                .try_get(1)
                .context("failed to get field public_user_id")?,
            role: row.try_get(2).context("failed to get field role")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthSetTokenReq {
    pub user_id: i64,
    pub user_token: uuid::Uuid,
    pub admin_token: uuid::Uuid,
    pub service_code: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthSetTokenRespRow {}

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
    fn parse_row(&self, row: Row) -> Result<FunAuthSetTokenRespRow> {
        let r = FunAuthSetTokenRespRow {};
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthRemoveTokenReq {
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthRemoveTokenRespRow {}

#[allow(unused_variables)]
impl DatabaseRequest for FunAuthRemoveTokenReq {
    type ResponseRow = FunAuthRemoveTokenRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_auth_remove_token(a_user_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.user_id as &(dyn ToSql + Sync)]
    }
    fn parse_row(&self, row: Row) -> Result<FunAuthRemoveTokenRespRow> {
        let r = FunAuthRemoveTokenRespRow {};
        Ok(r)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthAuthorizeRespRow {
    pub user_id: i64,
    pub role: EnumRole,
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
    fn parse_row(&self, row: Row) -> Result<FunAuthAuthorizeRespRow> {
        let r = FunAuthAuthorizeRespRow {
            user_id: row.try_get(0).context("failed to get field user_id")?,
            role: row.try_get(1).context("failed to get field role")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthSetRoleReq {
    pub public_user_id: i64,
    pub role: EnumRole,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthSetRoleRespRow {}

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
    fn parse_row(&self, row: Row) -> Result<FunAuthSetRoleRespRow> {
        let r = FunAuthSetRoleRespRow {};
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthChangeLoginWalletAddressReq {
    pub old_wallet_address: String,
    pub new_wallet_address: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthChangeLoginWalletAddressRespRow {}

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
    fn parse_row(&self, row: Row) -> Result<FunAuthChangeLoginWalletAddressRespRow> {
        let r = FunAuthChangeLoginWalletAddressRespRow {};
        Ok(r)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthUpdateUserTableRespRow {}

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
    fn parse_row(&self, row: Row) -> Result<FunAuthUpdateUserTableRespRow> {
        let r = FunAuthUpdateUserTableRespRow {};
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserFollowStrategyReq {
    pub user_id: i64,
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserFollowStrategyRespRow {
    pub success: bool,
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
    fn parse_row(&self, row: Row) -> Result<FunUserFollowStrategyRespRow> {
        let r = FunUserFollowStrategyRespRow {
            success: row.try_get(0).context("failed to get field success")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserUnfollowStrategyReq {
    pub user_id: i64,
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserUnfollowStrategyRespRow {
    pub success: bool,
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
    fn parse_row(&self, row: Row) -> Result<FunUserUnfollowStrategyRespRow> {
        let r = FunUserUnfollowStrategyRespRow {
            success: row.try_get(0).context("failed to get field success")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListFollowedStrategiesReq {
    pub user_id: i64,
    pub limit: i64,
    pub offset: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListFollowedStrategiesRespRow {
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
    pub followed: bool,
    pub approved: bool,
    #[serde(default)]
    pub approved_at: Option<i64>,
    pub pending_approval: bool,
    #[serde(default)]
    pub linked_wallet: Option<String>,
    #[serde(default)]
    pub linked_wallet_blockchain: Option<EnumBlockChain>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListFollowedStrategiesReq {
    type ResponseRow = FunUserListFollowedStrategiesRespRow;
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
    fn parse_row(&self, row: Row) -> Result<FunUserListFollowedStrategiesRespRow> {
        let r = FunUserListFollowedStrategiesRespRow {
            strategy_id: row.try_get(0).context("failed to get field strategy_id")?,
            strategy_name: row
                .try_get(1)
                .context("failed to get field strategy_name")?,
            strategy_description: row
                .try_get(2)
                .context("failed to get field strategy_description")?,
            net_value: row.try_get(3).context("failed to get field net_value")?,
            followers: row.try_get(4).context("failed to get field followers")?,
            backers: row.try_get(5).context("failed to get field backers")?,
            risk_score: row.try_get(6).context("failed to get field risk_score")?,
            aum: row.try_get(7).context("failed to get field aum")?,
            followed: row.try_get(8).context("failed to get field followed")?,
            approved: row.try_get(9).context("failed to get field approved")?,
            approved_at: row.try_get(10).context("failed to get field approved_at")?,
            pending_approval: row
                .try_get(11)
                .context("failed to get field pending_approval")?,
            linked_wallet: row
                .try_get(12)
                .context("failed to get field linked_wallet")?,
            linked_wallet_blockchain: row
                .try_get(13)
                .context("failed to get field linked_wallet_blockchain")?,
        };
        Ok(r)
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
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListStrategiesRespRow {
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
    pub followed: bool,
    #[serde(default)]
    pub linked_wallet: Option<String>,
    #[serde(default)]
    pub linked_wallet_blockchain: Option<EnumBlockChain>,
    pub approved: bool,
    #[serde(default)]
    pub approved_at: Option<i64>,
    pub pending_approval: bool,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListStrategiesReq {
    type ResponseRow = FunUserListStrategiesRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_strategies(a_user_id => $1::bigint, a_limit => $2::bigint, a_offset => $3::bigint, a_strategy_id => $4::bigint, a_strategy_name => $5::varchar, a_expert_public_id => $6::bigint, a_expert_name => $7::varchar, a_description => $8::varchar);"
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
        ]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserListStrategiesRespRow> {
        let r = FunUserListStrategiesRespRow {
            strategy_id: row.try_get(0).context("failed to get field strategy_id")?,
            strategy_name: row
                .try_get(1)
                .context("failed to get field strategy_name")?,
            strategy_description: row
                .try_get(2)
                .context("failed to get field strategy_description")?,
            net_value: row.try_get(3).context("failed to get field net_value")?,
            followers: row.try_get(4).context("failed to get field followers")?,
            backers: row.try_get(5).context("failed to get field backers")?,
            risk_score: row.try_get(6).context("failed to get field risk_score")?,
            aum: row.try_get(7).context("failed to get field aum")?,
            followed: row.try_get(8).context("failed to get field followed")?,
            linked_wallet: row
                .try_get(9)
                .context("failed to get field linked_wallet")?,
            linked_wallet_blockchain: row
                .try_get(10)
                .context("failed to get field linked_wallet_blockchain")?,
            approved: row.try_get(11).context("failed to get field approved")?,
            approved_at: row.try_get(12).context("failed to get field approved_at")?,
            pending_approval: row
                .try_get(13)
                .context("failed to get field pending_approval")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListTopPerformingStrategiesReq {
    pub limit: i64,
    pub offset: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
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
    fn parse_row(&self, row: Row) -> Result<FunUserListTopPerformingStrategiesRespRow> {
        let r = FunUserListTopPerformingStrategiesRespRow {
            strategy_id: row.try_get(0).context("failed to get field strategy_id")?,
            strategy_name: row
                .try_get(1)
                .context("failed to get field strategy_name")?,
            strategy_description: row
                .try_get(2)
                .context("failed to get field strategy_description")?,
            net_value: row.try_get(3).context("failed to get field net_value")?,
            followers: row.try_get(4).context("failed to get field followers")?,
            backers: row.try_get(5).context("failed to get field backers")?,
            risk_score: row.try_get(6).context("failed to get field risk_score")?,
            aum: row.try_get(7).context("failed to get field aum")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetStrategyReq {
    pub strategy_id: i64,
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetStrategyRespRow {
    pub strategy_id: i64,
    pub strategy_name: String,
    pub strategy_description: String,
    pub current_usdc: String,
    pub total_backed_usdc: String,
    pub total_exited_usdc: String,
    pub followers: i64,
    pub backers: i64,
    #[serde(default)]
    pub risk_score: Option<f64>,
    #[serde(default)]
    pub aum: Option<f64>,
    #[serde(default)]
    pub evm_contract_address: Option<String>,
    pub followed: bool,
    pub creator_user_public_id: i64,
    #[serde(default)]
    pub linked_wallet: Option<String>,
    #[serde(default)]
    pub linked_wallet_blockchain: Option<EnumBlockChain>,
    pub created_at: i64,
    pub approved: bool,
    #[serde(default)]
    pub approved_at: Option<i64>,
    pub pending_approval: bool,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserGetStrategyReq {
    type ResponseRow = FunUserGetStrategyRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_get_strategy(a_strategy_id => $1::bigint, a_user_id => $2::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
        ]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserGetStrategyRespRow> {
        let r = FunUserGetStrategyRespRow {
            strategy_id: row.try_get(0).context("failed to get field strategy_id")?,
            strategy_name: row
                .try_get(1)
                .context("failed to get field strategy_name")?,
            strategy_description: row
                .try_get(2)
                .context("failed to get field strategy_description")?,
            current_usdc: row.try_get(3).context("failed to get field current_usdc")?,
            total_backed_usdc: row
                .try_get(4)
                .context("failed to get field total_backed_usdc")?,
            total_exited_usdc: row
                .try_get(5)
                .context("failed to get field total_exited_usdc")?,
            followers: row.try_get(6).context("failed to get field followers")?,
            backers: row.try_get(7).context("failed to get field backers")?,
            risk_score: row.try_get(8).context("failed to get field risk_score")?,
            aum: row.try_get(9).context("failed to get field aum")?,
            evm_contract_address: row
                .try_get(10)
                .context("failed to get field evm_contract_address")?,
            followed: row.try_get(11).context("failed to get field followed")?,
            creator_user_public_id: row
                .try_get(12)
                .context("failed to get field creator_user_public_id")?,
            linked_wallet: row
                .try_get(13)
                .context("failed to get field linked_wallet")?,
            linked_wallet_blockchain: row
                .try_get(14)
                .context("failed to get field linked_wallet_blockchain")?,
            created_at: row.try_get(15).context("failed to get field created_at")?,
            approved: row.try_get(16).context("failed to get field approved")?,
            approved_at: row.try_get(17).context("failed to get field approved_at")?,
            pending_approval: row
                .try_get(18)
                .context("failed to get field pending_approval")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetStrategyStatisticsNetValueReq {
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetStrategyStatisticsNetValueRespRow {
    pub time: i64,
    pub net_value: f64,
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
    fn parse_row(&self, row: Row) -> Result<FunUserGetStrategyStatisticsNetValueRespRow> {
        let r = FunUserGetStrategyStatisticsNetValueRespRow {
            time: row.try_get(0).context("failed to get field time")?,
            net_value: row.try_get(1).context("failed to get field net_value")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetStrategyStatisticsFollowHistoryReq {
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetStrategyStatisticsFollowHistoryRespRow {
    pub time: i64,
    pub follower_count: f64,
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
    fn parse_row(&self, row: Row) -> Result<FunUserGetStrategyStatisticsFollowHistoryRespRow> {
        let r = FunUserGetStrategyStatisticsFollowHistoryRespRow {
            time: row.try_get(0).context("failed to get field time")?,
            follower_count: row
                .try_get(1)
                .context("failed to get field follower_count")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetStrategyStatisticsBackHistoryReq {
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetStrategyStatisticsBackHistoryRespRow {
    pub time: i64,
    pub backer_count: f64,
    pub backer_quantity_usd: f64,
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
    fn parse_row(&self, row: Row) -> Result<FunUserGetStrategyStatisticsBackHistoryRespRow> {
        let r = FunUserGetStrategyStatisticsBackHistoryRespRow {
            time: row.try_get(0).context("failed to get field time")?,
            backer_count: row.try_get(1).context("failed to get field backer_count")?,
            backer_quantity_usd: row
                .try_get(2)
                .context("failed to get field backer_quantity_usd")?,
        };
        Ok(r)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserDepositToEscrowRespRow {
    pub success: bool,
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
    fn parse_row(&self, row: Row) -> Result<FunUserDepositToEscrowRespRow> {
        let r = FunUserDepositToEscrowRespRow {
            success: row.try_get(0).context("failed to get field success")?,
        };
        Ok(r)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserBackStrategyRespRow {
    pub success: bool,
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
    fn parse_row(&self, row: Row) -> Result<FunUserBackStrategyRespRow> {
        let r = FunUserBackStrategyRespRow {
            success: row.try_get(0).context("failed to get field success")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListBackedStrategiesReq {
    pub user_id: i64,
    pub offset: i64,
    pub limit: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListBackedStrategiesRespRow {
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
    pub followed: bool,
    pub approved: bool,
    #[serde(default)]
    pub approved_at: Option<i64>,
    #[serde(default)]
    pub linked_wallet: Option<String>,
    #[serde(default)]
    pub linked_wallet_blockchain: Option<EnumBlockChain>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListBackedStrategiesReq {
    type ResponseRow = FunUserListBackedStrategiesRespRow;
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
    fn parse_row(&self, row: Row) -> Result<FunUserListBackedStrategiesRespRow> {
        let r = FunUserListBackedStrategiesRespRow {
            strategy_id: row.try_get(0).context("failed to get field strategy_id")?,
            strategy_name: row
                .try_get(1)
                .context("failed to get field strategy_name")?,
            strategy_description: row
                .try_get(2)
                .context("failed to get field strategy_description")?,
            net_value: row.try_get(3).context("failed to get field net_value")?,
            followers: row.try_get(4).context("failed to get field followers")?,
            backers: row.try_get(5).context("failed to get field backers")?,
            risk_score: row.try_get(6).context("failed to get field risk_score")?,
            aum: row.try_get(7).context("failed to get field aum")?,
            followed: row.try_get(8).context("failed to get field followed")?,
            approved: row.try_get(9).context("failed to get field approved")?,
            approved_at: row.try_get(10).context("failed to get field approved_at")?,
            linked_wallet: row
                .try_get(11)
                .context("failed to get field linked_wallet")?,
            linked_wallet_blockchain: row
                .try_get(12)
                .context("failed to get field linked_wallet_blockchain")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListBackStrategyHistoryReq {
    pub user_id: i64,
    #[serde(default)]
    pub strategy_id: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListBackStrategyHistoryRespRow {
    pub back_history_id: i64,
    pub strategy_id: i64,
    pub quantity: String,
    pub wallet_address: String,
    pub blockchain: EnumBlockChain,
    pub transaction_hash: String,
    pub time: i64,
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
    fn parse_row(&self, row: Row) -> Result<FunUserListBackStrategyHistoryRespRow> {
        let r = FunUserListBackStrategyHistoryRespRow {
            back_history_id: row
                .try_get(0)
                .context("failed to get field back_history_id")?,
            strategy_id: row.try_get(1).context("failed to get field strategy_id")?,
            quantity: row.try_get(2).context("failed to get field quantity")?,
            wallet_address: row
                .try_get(3)
                .context("failed to get field wallet_address")?,
            blockchain: row.try_get(4).context("failed to get field blockchain")?,
            transaction_hash: row
                .try_get(5)
                .context("failed to get field transaction_hash")?,
            time: row.try_get(6).context("failed to get field time")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserExitStrategyReq {
    pub user_id: i64,
    pub strategy_id: i64,
    pub quantity: String,
    pub blockchain: EnumBlockChain,
    pub dex: String,
    pub back_time: i64,
    pub transaction_hash: String,
    pub purchase_wallet: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserExitStrategyRespRow {
    pub success: bool,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserExitStrategyReq {
    type ResponseRow = FunUserExitStrategyRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_exit_strategy(a_user_id => $1::bigint, a_strategy_id => $2::bigint, a_quantity => $3::varchar, a_blockchain => $4::enum_block_chain, a_dex => $5::varchar, a_back_time => $6::bigint, a_transaction_hash => $7::varchar, a_purchase_wallet => $8::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.quantity as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.dex as &(dyn ToSql + Sync),
            &self.back_time as &(dyn ToSql + Sync),
            &self.transaction_hash as &(dyn ToSql + Sync),
            &self.purchase_wallet as &(dyn ToSql + Sync),
        ]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserExitStrategyRespRow> {
        let r = FunUserExitStrategyRespRow {
            success: row.try_get(0).context("failed to get field success")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListExitStrategyHistoryReq {
    pub user_id: i64,
    #[serde(default)]
    pub strategy_id: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListExitStrategyHistoryRespRow {
    pub exit_history_id: i64,
    pub strategy_id: i64,
    pub exit_quantity: String,
    pub purchase_wallet_address: String,
    pub blockchain: EnumBlockChain,
    pub dex: String,
    pub back_time: i64,
    pub exit_time: i64,
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
    fn parse_row(&self, row: Row) -> Result<FunUserListExitStrategyHistoryRespRow> {
        let r = FunUserListExitStrategyHistoryRespRow {
            exit_history_id: row
                .try_get(0)
                .context("failed to get field exit_history_id")?,
            strategy_id: row.try_get(1).context("failed to get field strategy_id")?,
            exit_quantity: row
                .try_get(2)
                .context("failed to get field exit_quantity")?,
            purchase_wallet_address: row
                .try_get(3)
                .context("failed to get field purchase_wallet_address")?,
            blockchain: row.try_get(4).context("failed to get field blockchain")?,
            dex: row.try_get(5).context("failed to get field dex")?,
            back_time: row.try_get(6).context("failed to get field back_time")?,
            exit_time: row.try_get(7).context("failed to get field exit_time")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserFollowExpertReq {
    pub user_id: i64,
    pub expert_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserFollowExpertRespRow {
    pub success: bool,
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
    fn parse_row(&self, row: Row) -> Result<FunUserFollowExpertRespRow> {
        let r = FunUserFollowExpertRespRow {
            success: row.try_get(0).context("failed to get field success")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserUnfollowExpertReq {
    pub user_id: i64,
    pub expert_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserUnfollowExpertRespRow {
    pub success: bool,
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
    fn parse_row(&self, row: Row) -> Result<FunUserUnfollowExpertRespRow> {
        let r = FunUserUnfollowExpertRespRow {
            success: row.try_get(0).context("failed to get field success")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListFollowedExpertsReq {
    pub user_id: i64,
    pub offset: i64,
    pub limit: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListFollowedExpertsRespRow {
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
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListFollowedExpertsReq {
    type ResponseRow = FunUserListFollowedExpertsRespRow;
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
    fn parse_row(&self, row: Row) -> Result<FunUserListFollowedExpertsRespRow> {
        let r = FunUserListFollowedExpertsRespRow {
            expert_id: row.try_get(0).context("failed to get field expert_id")?,
            user_id: row.try_get(1).context("failed to get field user_id")?,
            user_public_id: row
                .try_get(2)
                .context("failed to get field user_public_id")?,
            listening_wallet: row
                .try_get(3)
                .context("failed to get field listening_wallet")?,
            username: row.try_get(4).context("failed to get field username")?,
            family_name: row.try_get(5).context("failed to get field family_name")?,
            given_name: row.try_get(6).context("failed to get field given_name")?,
            follower_count: row
                .try_get(7)
                .context("failed to get field follower_count")?,
            description: row.try_get(8).context("failed to get field description")?,
            social_media: row.try_get(9).context("failed to get field social_media")?,
            risk_score: row.try_get(10).context("failed to get field risk_score")?,
            reputation_score: row
                .try_get(11)
                .context("failed to get field reputation_score")?,
            aum: row.try_get(12).context("failed to get field aum")?,
            joined_at: row.try_get(13).context("failed to get field joined_at")?,
            requested_at: row
                .try_get(14)
                .context("failed to get field requested_at")?,
            approved_at: row.try_get(15).context("failed to get field approved_at")?,
            pending_expert: row
                .try_get(16)
                .context("failed to get field pending_expert")?,
            approved_expert: row
                .try_get(17)
                .context("failed to get field approved_expert")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListExpertsReq {
    pub limit: i64,
    pub offset: i64,
    pub user_id: i64,
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListExpertsRespRow {
    pub expert_id: i64,
    pub user_id: i64,
    pub user_public_id: i64,
    pub listening_wallet: String,
    pub username: String,
    #[serde(default)]
    pub family_name: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
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
    pub follower_count: i64,
    pub backer_count: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListExpertsReq {
    type ResponseRow = FunUserListExpertsRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_experts(a_limit => $1::bigint, a_offset => $2::bigint, a_user_id => $3::bigint, a_expert_id => $4::bigint, a_expert_user_id => $5::bigint, a_expert_user_public_id => $6::bigint, a_username => $7::varchar, a_family_name => $8::varchar, a_given_name => $9::varchar, a_description => $10::varchar, a_social_media => $11::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.limit as &(dyn ToSql + Sync),
            &self.offset as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
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
    fn parse_row(&self, row: Row) -> Result<FunUserListExpertsRespRow> {
        let r = FunUserListExpertsRespRow {
            expert_id: row.try_get(0).context("failed to get field expert_id")?,
            user_id: row.try_get(1).context("failed to get field user_id")?,
            user_public_id: row
                .try_get(2)
                .context("failed to get field user_public_id")?,
            listening_wallet: row
                .try_get(3)
                .context("failed to get field listening_wallet")?,
            username: row.try_get(4).context("failed to get field username")?,
            family_name: row.try_get(5).context("failed to get field family_name")?,
            given_name: row.try_get(6).context("failed to get field given_name")?,
            description: row.try_get(7).context("failed to get field description")?,
            social_media: row.try_get(8).context("failed to get field social_media")?,
            risk_score: row.try_get(9).context("failed to get field risk_score")?,
            reputation_score: row
                .try_get(10)
                .context("failed to get field reputation_score")?,
            aum: row.try_get(11).context("failed to get field aum")?,
            joined_at: row.try_get(12).context("failed to get field joined_at")?,
            requested_at: row
                .try_get(13)
                .context("failed to get field requested_at")?,
            approved_at: row.try_get(14).context("failed to get field approved_at")?,
            pending_expert: row
                .try_get(15)
                .context("failed to get field pending_expert")?,
            approved_expert: row
                .try_get(16)
                .context("failed to get field approved_expert")?,
            followed: row.try_get(17).context("failed to get field followed")?,
            follower_count: row
                .try_get(18)
                .context("failed to get field follower_count")?,
            backer_count: row
                .try_get(19)
                .context("failed to get field backer_count")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetExpertProfileReq {
    pub expert_id: i64,
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetExpertProfileRespRow {
    #[serde(default)]
    pub expert_id: Option<i64>,
    pub user_id: i64,
    pub user_public_id: i64,
    pub listening_wallet: String,
    pub username: String,
    #[serde(default)]
    pub family_name: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
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
    pub joined_at: i64,
    #[serde(default)]
    pub requested_at: Option<i64>,
    #[serde(default)]
    pub approved_at: Option<i64>,
    pub pending_expert: bool,
    pub approved_expert: bool,
    pub followed: bool,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserGetExpertProfileReq {
    type ResponseRow = FunUserGetExpertProfileRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_get_expert_profile(a_expert_id => $1::bigint, a_user_id => $2::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.expert_id as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
        ]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserGetExpertProfileRespRow> {
        let r = FunUserGetExpertProfileRespRow {
            expert_id: row.try_get(0).context("failed to get field expert_id")?,
            user_id: row.try_get(1).context("failed to get field user_id")?,
            user_public_id: row
                .try_get(2)
                .context("failed to get field user_public_id")?,
            listening_wallet: row
                .try_get(3)
                .context("failed to get field listening_wallet")?,
            username: row.try_get(4).context("failed to get field username")?,
            family_name: row.try_get(5).context("failed to get field family_name")?,
            given_name: row.try_get(6).context("failed to get field given_name")?,
            follower_count: row
                .try_get(7)
                .context("failed to get field follower_count")?,
            description: row.try_get(8).context("failed to get field description")?,
            social_media: row.try_get(9).context("failed to get field social_media")?,
            risk_score: row.try_get(10).context("failed to get field risk_score")?,
            reputation_score: row
                .try_get(11)
                .context("failed to get field reputation_score")?,
            aum: row.try_get(12).context("failed to get field aum")?,
            joined_at: row.try_get(13).context("failed to get field joined_at")?,
            requested_at: row
                .try_get(14)
                .context("failed to get field requested_at")?,
            approved_at: row.try_get(15).context("failed to get field approved_at")?,
            pending_expert: row
                .try_get(16)
                .context("failed to get field pending_expert")?,
            approved_expert: row
                .try_get(17)
                .context("failed to get field approved_expert")?,
            followed: row.try_get(18).context("failed to get field followed")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetUserProfileReq {
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[allow(unused_variables)]
impl DatabaseRequest for FunUserGetUserProfileReq {
    type ResponseRow = FunUserGetUserProfileRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_get_user_profile(a_user_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.user_id as &(dyn ToSql + Sync)]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserGetUserProfileRespRow> {
        let r = FunUserGetUserProfileRespRow {
            expert_id: row.try_get(0).context("failed to get field expert_id")?,
            user_public_id: row
                .try_get(1)
                .context("failed to get field user_public_id")?,
            name: row.try_get(2).context("failed to get field name")?,
            login_wallet: row.try_get(3).context("failed to get field login_wallet")?,
            joined_at: row.try_get(4).context("failed to get field joined_at")?,
            follower_count: row
                .try_get(5)
                .context("failed to get field follower_count")?,
            description: row.try_get(6).context("failed to get field description")?,
            social_media: row.try_get(7).context("failed to get field social_media")?,
            risk_score: row.try_get(8).context("failed to get field risk_score")?,
            reputation_score: row
                .try_get(9)
                .context("failed to get field reputation_score")?,
            aum: row.try_get(10).context("failed to get field aum")?,
        };
        Ok(r)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserCreateExpertProfileRespRow {
    pub expert_id: i64,
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
    fn parse_row(&self, row: Row) -> Result<FunUserCreateExpertProfileRespRow> {
        let r = FunUserCreateExpertProfileRespRow {
            expert_id: row.try_get(0).context("failed to get field expert_id")?,
        };
        Ok(r)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserUpdateExpertProfileRespRow {}

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
    fn parse_row(&self, row: Row) -> Result<FunUserUpdateExpertProfileRespRow> {
        let r = FunUserUpdateExpertProfileRespRow {};
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserApplyBecomeExpertReq {
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserApplyBecomeExpertRespRow {
    pub success: bool,
    pub expert_id: i64,
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
    fn parse_row(&self, row: Row) -> Result<FunUserApplyBecomeExpertRespRow> {
        let r = FunUserApplyBecomeExpertRespRow {
            success: row.try_get(0).context("failed to get field success")?,
            expert_id: row.try_get(1).context("failed to get field expert_id")?,
        };
        Ok(r)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserCreateStrategyRespRow {
    pub success: bool,
    pub strategy_id: i64,
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
    fn parse_row(&self, row: Row) -> Result<FunUserCreateStrategyRespRow> {
        let r = FunUserCreateStrategyRespRow {
            success: row.try_get(0).context("failed to get field success")?,
            strategy_id: row.try_get(1).context("failed to get field strategy_id")?,
        };
        Ok(r)
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
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserUpdateStrategyRespRow {
    pub success: bool,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserUpdateStrategyReq {
    type ResponseRow = FunUserUpdateStrategyRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_update_strategy(a_user_id => $1::bigint, a_strategy_id => $2::bigint, a_name => $3::varchar, a_description => $4::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.name as &(dyn ToSql + Sync),
            &self.description as &(dyn ToSql + Sync),
        ]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserUpdateStrategyRespRow> {
        let r = FunUserUpdateStrategyRespRow {
            success: row.try_get(0).context("failed to get field success")?,
        };
        Ok(r)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserAddStrategyWatchWalletRespRow {
    pub success: bool,
    pub watch_wallet_id: i64,
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
    fn parse_row(&self, row: Row) -> Result<FunUserAddStrategyWatchWalletRespRow> {
        let r = FunUserAddStrategyWatchWalletRespRow {
            success: row.try_get(0).context("failed to get field success")?,
            watch_wallet_id: row
                .try_get(1)
                .context("failed to get field watch_wallet_id")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserRemoveStrategyWatchWalletReq {
    pub user_id: i64,
    pub watch_wallet_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserRemoveStrategyWatchWalletRespRow {
    pub success: bool,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserRemoveStrategyWatchWalletReq {
    type ResponseRow = FunUserRemoveStrategyWatchWalletRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_remove_strategy_watch_wallet(a_user_id => $1::bigint, a_watch_wallet_id => $2::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.watch_wallet_id as &(dyn ToSql + Sync),
        ]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserRemoveStrategyWatchWalletRespRow> {
        let r = FunUserRemoveStrategyWatchWalletRespRow {
            success: row.try_get(0).context("failed to get field success")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListStrategyWatchWalletsReq {
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListStrategyWatchWalletsRespRow {
    pub watch_wallet_id: i64,
    pub wallet_address: String,
    pub blockchain: EnumBlockChain,
    pub ratio: f64,
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
    fn parse_row(&self, row: Row) -> Result<FunUserListStrategyWatchWalletsRespRow> {
        let r = FunUserListStrategyWatchWalletsRespRow {
            watch_wallet_id: row
                .try_get(0)
                .context("failed to get field watch_wallet_id")?,
            wallet_address: row
                .try_get(1)
                .context("failed to get field wallet_address")?,
            blockchain: row.try_get(2).context("failed to get field blockchain")?,
            ratio: row.try_get(3).context("failed to get field ratio")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListStrategyFollowersReq {
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListStrategyFollowersRespRow {
    pub user_id: i64,
    pub user_public_id: i64,
    pub username: String,
    pub wallet_address: String,
    pub followed_at: i64,
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
    fn parse_row(&self, row: Row) -> Result<FunUserListStrategyFollowersRespRow> {
        let r = FunUserListStrategyFollowersRespRow {
            user_id: row.try_get(0).context("failed to get field user_id")?,
            user_public_id: row
                .try_get(1)
                .context("failed to get field user_public_id")?,
            username: row.try_get(2).context("failed to get field username")?,
            wallet_address: row
                .try_get(3)
                .context("failed to get field wallet_address")?,
            followed_at: row.try_get(4).context("failed to get field followed_at")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListStrategyBackersReq {
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListStrategyBackersRespRow {
    pub user_id: i64,
    pub user_public_id: i64,
    pub username: String,
    pub wallet_address: String,
    pub backed_at: i64,
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
    fn parse_row(&self, row: Row) -> Result<FunUserListStrategyBackersRespRow> {
        let r = FunUserListStrategyBackersRespRow {
            user_id: row.try_get(0).context("failed to get field user_id")?,
            user_public_id: row
                .try_get(1)
                .context("failed to get field user_public_id")?,
            username: row.try_get(2).context("failed to get field username")?,
            wallet_address: row
                .try_get(3)
                .context("failed to get field wallet_address")?,
            backed_at: row.try_get(4).context("failed to get field backed_at")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserAddRegisteredWalletReq {
    pub user_id: i64,
    pub blockchain: EnumBlockChain,
    pub address: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserAddRegisteredWalletRespRow {
    pub registered_wallet_id: i64,
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
    fn parse_row(&self, row: Row) -> Result<FunUserAddRegisteredWalletRespRow> {
        let r = FunUserAddRegisteredWalletRespRow {
            registered_wallet_id: row
                .try_get(0)
                .context("failed to get field registered_wallet_id")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserRemoveRegisteredWalletReq {
    pub registered_wallet_id: i64,
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserRemoveRegisteredWalletRespRow {}

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
    fn parse_row(&self, row: Row) -> Result<FunUserRemoveRegisteredWalletRespRow> {
        let r = FunUserRemoveRegisteredWalletRespRow {};
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListRegisteredWalletsReq {
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListRegisteredWalletsRespRow {
    pub registered_wallet_id: i64,
    pub blockchain: EnumBlockChain,
    pub address: String,
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
    fn parse_row(&self, row: Row) -> Result<FunUserListRegisteredWalletsRespRow> {
        let r = FunUserListRegisteredWalletsRespRow {
            registered_wallet_id: row
                .try_get(0)
                .context("failed to get field registered_wallet_id")?,
            blockchain: row.try_get(1).context("failed to get field blockchain")?,
            address: row.try_get(2).context("failed to get field address")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserRequestRefundReq {
    pub user_id: i64,
    pub blockchain: EnumBlockChain,
    pub quantity: String,
    pub wallet_address: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserRequestRefundRespRow {
    pub request_refund_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserRequestRefundReq {
    type ResponseRow = FunUserRequestRefundRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_request_refund(a_user_id => $1::bigint, a_blockchain => $2::enum_block_chain, a_quantity => $3::varchar, a_wallet_address => $4::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.quantity as &(dyn ToSql + Sync),
            &self.wallet_address as &(dyn ToSql + Sync),
        ]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserRequestRefundRespRow> {
        let r = FunUserRequestRefundRespRow {
            request_refund_id: row
                .try_get(0)
                .context("failed to get field request_refund_id")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListRequestRefundHistoryReq {}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListRequestRefundHistoryRespRow {
    pub request_refund_id: i64,
    pub user_id: i64,
    pub blockchain: EnumBlockChain,
    pub quantity: String,
    pub wallet_address: String,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListRequestRefundHistoryReq {
    type ResponseRow = FunUserListRequestRefundHistoryRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_request_refund_history();"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserListRequestRefundHistoryRespRow> {
        let r = FunUserListRequestRefundHistoryRespRow {
            request_refund_id: row
                .try_get(0)
                .context("failed to get field request_refund_id")?,
            user_id: row.try_get(1).context("failed to get field user_id")?,
            blockchain: row.try_get(2).context("failed to get field blockchain")?,
            quantity: row.try_get(3).context("failed to get field quantity")?,
            wallet_address: row
                .try_get(4)
                .context("failed to get field wallet_address")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserUpdateRequestRefundHistoryReq {
    pub request_refund_id: i64,
    pub transaction_hash: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserUpdateRequestRefundHistoryRespRow {}

#[allow(unused_variables)]
impl DatabaseRequest for FunUserUpdateRequestRefundHistoryReq {
    type ResponseRow = FunUserUpdateRequestRefundHistoryRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_update_request_refund_history(a_request_refund_id => $1::bigint, a_transaction_hash => $2::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.request_refund_id as &(dyn ToSql + Sync),
            &self.transaction_hash as &(dyn ToSql + Sync),
        ]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserUpdateRequestRefundHistoryRespRow> {
        let r = FunUserUpdateRequestRefundHistoryRespRow {};
        Ok(r)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserAddStrategyInitialTokenRatioRespRow {
    pub strategy_initial_token_ratio_id: i64,
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
    fn parse_row(&self, row: Row) -> Result<FunUserAddStrategyInitialTokenRatioRespRow> {
        let r = FunUserAddStrategyInitialTokenRatioRespRow {
            strategy_initial_token_ratio_id: row
                .try_get(0)
                .context("failed to get field strategy_initial_token_ratio_id")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserRemoveStrategyInitialTokenRatioReq {
    pub strategy_initial_token_ratio_id: i64,
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserRemoveStrategyInitialTokenRatioRespRow {}

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
    fn parse_row(&self, row: Row) -> Result<FunUserRemoveStrategyInitialTokenRatioRespRow> {
        let r = FunUserRemoveStrategyInitialTokenRatioRespRow {};
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListStrategyInitialTokenRatiosReq {
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[allow(unused_variables)]
impl DatabaseRequest for FunUserListStrategyInitialTokenRatiosReq {
    type ResponseRow = FunUserListStrategyInitialTokenRatiosRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_strategy_initial_token_ratios(a_strategy_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.strategy_id as &(dyn ToSql + Sync)]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserListStrategyInitialTokenRatiosRespRow> {
        let r = FunUserListStrategyInitialTokenRatiosRespRow {
            strategy_initial_token_ratio_id: row
                .try_get(0)
                .context("failed to get field strategy_initial_token_ratio_id")?,
            blockchain: row.try_get(1).context("failed to get field blockchain")?,
            token_name: row.try_get(2).context("failed to get field token_name")?,
            token_address: row
                .try_get(3)
                .context("failed to get field token_address")?,
            quantity: row.try_get(4).context("failed to get field quantity")?,
            strategy_id: row.try_get(5).context("failed to get field strategy_id")?,
            created_at: row.try_get(6).context("failed to get field created_at")?,
            updated_at: row.try_get(7).context("failed to get field updated_at")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunExpertListFollowersReq {
    pub user_id: i64,
    pub limit: i64,
    pub offset: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
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
    fn parse_row(&self, row: Row) -> Result<FunExpertListFollowersRespRow> {
        let r = FunExpertListFollowersRespRow {
            public_id: row.try_get(0).context("failed to get field public_id")?,
            username: row.try_get(1).context("failed to get field username")?,
            family_name: row.try_get(2).context("failed to get field family_name")?,
            given_name: row.try_get(3).context("failed to get field given_name")?,
            followed_at: row.try_get(4).context("failed to get field followed_at")?,
            joined_at: row.try_get(5).context("failed to get field joined_at")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunExpertListBackersReq {
    pub user_id: i64,
    pub limit: i64,
    pub offset: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
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
    fn parse_row(&self, row: Row) -> Result<FunExpertListBackersRespRow> {
        let r = FunExpertListBackersRespRow {
            public_id: row.try_get(0).context("failed to get field public_id")?,
            username: row.try_get(1).context("failed to get field username")?,
            family_name: row.try_get(2).context("failed to get field family_name")?,
            given_name: row.try_get(3).context("failed to get field given_name")?,
            backed_at: row.try_get(4).context("failed to get field backed_at")?,
            joined_at: row.try_get(5).context("failed to get field joined_at")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListDepositHistoryReq {
    pub user_id: i64,
    pub limit: i64,
    pub offset: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListDepositHistoryRespRow {
    pub blockchain: EnumBlockChain,
    pub user_address: String,
    pub contract_address: String,
    pub receiver_address: String,
    pub quantity: String,
    pub transaction_hash: String,
    pub created_at: i64,
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
    fn parse_row(&self, row: Row) -> Result<FunUserListDepositHistoryRespRow> {
        let r = FunUserListDepositHistoryRespRow {
            blockchain: row.try_get(0).context("failed to get field blockchain")?,
            user_address: row.try_get(1).context("failed to get field user_address")?,
            contract_address: row
                .try_get(2)
                .context("failed to get field contract_address")?,
            receiver_address: row
                .try_get(3)
                .context("failed to get field receiver_address")?,
            quantity: row.try_get(4).context("failed to get field quantity")?,
            transaction_hash: row
                .try_get(5)
                .context("failed to get field transaction_hash")?,
            created_at: row.try_get(6).context("failed to get field created_at")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetUserByAddressReq {
    pub address: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[allow(unused_variables)]
impl DatabaseRequest for FunUserGetUserByAddressReq {
    type ResponseRow = FunUserGetUserByAddressRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_get_user_by_address(a_address => $1::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.address as &(dyn ToSql + Sync)]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserGetUserByAddressRespRow> {
        let r = FunUserGetUserByAddressRespRow {
            user_id: row.try_get(0).context("failed to get field user_id")?,
            user_public_id: row
                .try_get(1)
                .context("failed to get field user_public_id")?,
            username: row.try_get(2).context("failed to get field username")?,
            family_name: row.try_get(3).context("failed to get field family_name")?,
            given_name: row.try_get(4).context("failed to get field given_name")?,
            joined_at: row.try_get(5).context("failed to get field joined_at")?,
        };
        Ok(r)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminListUsersRespRow {
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
    fn parse_row(&self, row: Row) -> Result<FunAdminListUsersRespRow> {
        let r = FunAdminListUsersRespRow {
            user_id: row.try_get(0).context("failed to get field user_id")?,
            public_user_id: row
                .try_get(1)
                .context("failed to get field public_user_id")?,
            username: row.try_get(2).context("failed to get field username")?,
            address: row.try_get(3).context("failed to get field address")?,
            last_ip: row.try_get(4).context("failed to get field last_ip")?,
            last_login_at: row
                .try_get(5)
                .context("failed to get field last_login_at")?,
            login_count: row.try_get(6).context("failed to get field login_count")?,
            role: row.try_get(7).context("failed to get field role")?,
            email: row.try_get(8).context("failed to get field email")?,
            updated_at: row.try_get(9).context("failed to get field updated_at")?,
            created_at: row.try_get(10).context("failed to get field created_at")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminSetUserRoleReq {
    pub user_id: i64,
    pub role: EnumRole,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminSetUserRoleRespRow {}

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
    fn parse_row(&self, row: Row) -> Result<FunAdminSetUserRoleRespRow> {
        let r = FunAdminSetUserRoleRespRow {};
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminSetBlockUserReq {
    pub user_id: i64,
    pub blocked: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminSetBlockUserRespRow {}

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
    fn parse_row(&self, row: Row) -> Result<FunAdminSetBlockUserRespRow> {
        let r = FunAdminSetBlockUserRespRow {};
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminApproveUserBecomeExpertReq {
    pub user_public_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminApproveUserBecomeExpertRespRow {
    pub success: bool,
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
    fn parse_row(&self, row: Row) -> Result<FunAdminApproveUserBecomeExpertRespRow> {
        let r = FunAdminApproveUserBecomeExpertRespRow {
            success: row.try_get(0).context("failed to get field success")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminRejectUserBecomeExpertReq {
    pub user_public_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminRejectUserBecomeExpertRespRow {
    pub success: bool,
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
    fn parse_row(&self, row: Row) -> Result<FunAdminRejectUserBecomeExpertRespRow> {
        let r = FunAdminRejectUserBecomeExpertRespRow {
            success: row.try_get(0).context("failed to get field success")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminListPendingUserExpertApplicationsReq {
    pub limit: i64,
    pub offset: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
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
    fn parse_row(&self, row: Row) -> Result<FunAdminListPendingUserExpertApplicationsRespRow> {
        let r = FunAdminListPendingUserExpertApplicationsRespRow {
            user_public_id: row
                .try_get(0)
                .context("failed to get field user_public_id")?,
            name: row.try_get(1).context("failed to get field name")?,
            linked_wallet: row
                .try_get(2)
                .context("failed to get field linked_wallet")?,
            follower_count: row
                .try_get(3)
                .context("failed to get field follower_count")?,
            description: row.try_get(4).context("failed to get field description")?,
            social_media: row.try_get(5).context("failed to get field social_media")?,
            risk_score: row.try_get(6).context("failed to get field risk_score")?,
            reputation_score: row
                .try_get(7)
                .context("failed to get field reputation_score")?,
            aum: row.try_get(8).context("failed to get field aum")?,
            pending_expert: row
                .try_get(9)
                .context("failed to get field pending_expert")?,
            approved_expert: row
                .try_get(10)
                .context("failed to get field approved_expert")?,
            joined_at: row.try_get(11).context("failed to get field joined_at")?,
            requested_at: row
                .try_get(12)
                .context("failed to get field requested_at")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminGetSystemConfigReq {
    pub config_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminGetSystemConfigRespRow {
    #[serde(default)]
    pub config_placeholder_1: Option<i64>,
    #[serde(default)]
    pub config_placeholder_2: Option<i64>,
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
    fn parse_row(&self, row: Row) -> Result<FunAdminGetSystemConfigRespRow> {
        let r = FunAdminGetSystemConfigRespRow {
            config_placeholder_1: row
                .try_get(0)
                .context("failed to get field config_placeholder_1")?,
            config_placeholder_2: row
                .try_get(1)
                .context("failed to get field config_placeholder_2")?,
        };
        Ok(r)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminUpdateSystemConfigRespRow {}

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
    fn parse_row(&self, row: Row) -> Result<FunAdminUpdateSystemConfigRespRow> {
        let r = FunAdminUpdateSystemConfigRespRow {};
        Ok(r)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminListExpertsRespRow {
    pub expert_id: i64,
    pub user_id: i64,
    pub user_public_id: i64,
    pub linked_wallet: String,
    pub name: String,
    #[serde(default)]
    pub family_name: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
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
    pub joined_at: i64,
    #[serde(default)]
    pub requested_at: Option<i64>,
    #[serde(default)]
    pub approved_at: Option<i64>,
    pub pending_expert: bool,
    pub approved_expert: bool,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminListExpertsReq {
    type ResponseRow = FunAdminListExpertsRespRow;
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
    fn parse_row(&self, row: Row) -> Result<FunAdminListExpertsRespRow> {
        let r = FunAdminListExpertsRespRow {
            expert_id: row.try_get(0).context("failed to get field expert_id")?,
            user_id: row.try_get(1).context("failed to get field user_id")?,
            user_public_id: row
                .try_get(2)
                .context("failed to get field user_public_id")?,
            linked_wallet: row
                .try_get(3)
                .context("failed to get field linked_wallet")?,
            name: row.try_get(4).context("failed to get field name")?,
            family_name: row.try_get(5).context("failed to get field family_name")?,
            given_name: row.try_get(6).context("failed to get field given_name")?,
            follower_count: row
                .try_get(7)
                .context("failed to get field follower_count")?,
            description: row.try_get(8).context("failed to get field description")?,
            social_media: row.try_get(9).context("failed to get field social_media")?,
            risk_score: row.try_get(10).context("failed to get field risk_score")?,
            reputation_score: row
                .try_get(11)
                .context("failed to get field reputation_score")?,
            aum: row.try_get(12).context("failed to get field aum")?,
            joined_at: row.try_get(13).context("failed to get field joined_at")?,
            requested_at: row
                .try_get(14)
                .context("failed to get field requested_at")?,
            approved_at: row.try_get(15).context("failed to get field approved_at")?,
            pending_expert: row
                .try_get(16)
                .context("failed to get field pending_expert")?,
            approved_expert: row
                .try_get(17)
                .context("failed to get field approved_expert")?,
        };
        Ok(r)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminListBackersRespRow {
    pub user_id: i64,
    pub user_public_id: i64,
    pub username: String,
    pub login_wallet_address: String,
    pub joined_at: i64,
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
    fn parse_row(&self, row: Row) -> Result<FunAdminListBackersRespRow> {
        let r = FunAdminListBackersRespRow {
            user_id: row.try_get(0).context("failed to get field user_id")?,
            user_public_id: row
                .try_get(1)
                .context("failed to get field user_public_id")?,
            username: row.try_get(2).context("failed to get field username")?,
            login_wallet_address: row
                .try_get(3)
                .context("failed to get field login_wallet_address")?,
            joined_at: row.try_get(4).context("failed to get field joined_at")?,
        };
        Ok(r)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminListStrategiesRespRow {
    pub strategy_id: i64,
    pub strategy_name: String,
    pub expert_id: i64,
    pub expert_public_id: i64,
    pub expert_name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub created_at: i64,
    pub pending_approval: bool,
    pub approved: bool,
    #[serde(default)]
    pub approved_at: Option<i64>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminListStrategiesReq {
    type ResponseRow = FunAdminListStrategiesRespRow;
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
    fn parse_row(&self, row: Row) -> Result<FunAdminListStrategiesRespRow> {
        let r = FunAdminListStrategiesRespRow {
            strategy_id: row.try_get(0).context("failed to get field strategy_id")?,
            strategy_name: row
                .try_get(1)
                .context("failed to get field strategy_name")?,
            expert_id: row.try_get(2).context("failed to get field expert_id")?,
            expert_public_id: row
                .try_get(3)
                .context("failed to get field expert_public_id")?,
            expert_name: row.try_get(4).context("failed to get field expert_name")?,
            description: row.try_get(5).context("failed to get field description")?,
            created_at: row.try_get(6).context("failed to get field created_at")?,
            pending_approval: row
                .try_get(7)
                .context("failed to get field pending_approval")?,
            approved: row.try_get(8).context("failed to get field approved")?,
            approved_at: row.try_get(9).context("failed to get field approved_at")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminApproveStrategyReq {
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminApproveStrategyRespRow {}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminApproveStrategyReq {
    type ResponseRow = FunAdminApproveStrategyRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_approve_strategy(a_strategy_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.strategy_id as &(dyn ToSql + Sync)]
    }
    fn parse_row(&self, row: Row) -> Result<FunAdminApproveStrategyRespRow> {
        let r = FunAdminApproveStrategyRespRow {};
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminRejectStrategiesReq {
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminRejectStrategiesRespRow {}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminRejectStrategiesReq {
    type ResponseRow = FunAdminRejectStrategiesRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_reject_strategies(a_strategy_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.strategy_id as &(dyn ToSql + Sync)]
    }
    fn parse_row(&self, row: Row) -> Result<FunAdminRejectStrategiesRespRow> {
        let r = FunAdminRejectStrategiesRespRow {};
        Ok(r)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherSaveRawTransactionRespRow {
    pub transaction_cache_id: i64,
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
    fn parse_row(&self, row: Row) -> Result<FunWatcherSaveRawTransactionRespRow> {
        let r = FunWatcherSaveRawTransactionRespRow {
            transaction_cache_id: row
                .try_get(0)
                .context("failed to get field transaction_cache_id")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherGetRawTransactionReq {
    pub transaction_hash: String,
    pub chain: String,
    #[serde(default)]
    pub dex: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherGetRawTransactionRespRow {
    pub transaction_cache_id: i64,
    pub transaction_hash: String,
    pub chain: String,
    #[serde(default)]
    pub dex: Option<String>,
    pub raw_transaction: String,
    pub created_at: i64,
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
    fn parse_row(&self, row: Row) -> Result<FunWatcherGetRawTransactionRespRow> {
        let r = FunWatcherGetRawTransactionRespRow {
            transaction_cache_id: row
                .try_get(0)
                .context("failed to get field transaction_cache_id")?,
            transaction_hash: row
                .try_get(1)
                .context("failed to get field transaction_hash")?,
            chain: row.try_get(2).context("failed to get field chain")?,
            dex: row.try_get(3).context("failed to get field dex")?,
            raw_transaction: row
                .try_get(4)
                .context("failed to get field raw_transaction")?,
            created_at: row.try_get(5).context("failed to get field created_at")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherSaveWalletActivityHistoryReq {
    pub address: String,
    pub transaction_hash: String,
    pub blockchain: EnumBlockChain,
    pub contract_address: String,
    pub caller_address: String,
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
    pub swap_calls: Option<serde_json::Value>,
    #[serde(default)]
    pub paths: Option<serde_json::Value>,
    #[serde(default)]
    pub dex_versions: Option<serde_json::Value>,
    #[serde(default)]
    pub created_at: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherSaveWalletActivityHistoryRespRow {
    pub wallet_activity_history_id: i64,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherSaveWalletActivityHistoryReq {
    type ResponseRow = FunWatcherSaveWalletActivityHistoryRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_save_wallet_activity_history(a_address => $1::varchar, a_transaction_hash => $2::varchar, a_blockchain => $3::enum_block_chain, a_contract_address => $4::varchar, a_caller_address => $5::varchar, a_dex => $6::varchar, a_token_in_address => $7::varchar, a_token_out_address => $8::varchar, a_amount_in => $9::varchar, a_amount_out => $10::varchar, a_swap_calls => $11::jsonb, a_paths => $12::jsonb, a_dex_versions => $13::jsonb, a_created_at => $14::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.address as &(dyn ToSql + Sync),
            &self.transaction_hash as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.contract_address as &(dyn ToSql + Sync),
            &self.caller_address as &(dyn ToSql + Sync),
            &self.dex as &(dyn ToSql + Sync),
            &self.token_in_address as &(dyn ToSql + Sync),
            &self.token_out_address as &(dyn ToSql + Sync),
            &self.amount_in as &(dyn ToSql + Sync),
            &self.amount_out as &(dyn ToSql + Sync),
            &self.swap_calls as &(dyn ToSql + Sync),
            &self.paths as &(dyn ToSql + Sync),
            &self.dex_versions as &(dyn ToSql + Sync),
            &self.created_at as &(dyn ToSql + Sync),
        ]
    }
    fn parse_row(&self, row: Row) -> Result<FunWatcherSaveWalletActivityHistoryRespRow> {
        let r = FunWatcherSaveWalletActivityHistoryRespRow {
            wallet_activity_history_id: row
                .try_get(0)
                .context("failed to get field wallet_activity_history_id")?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherListWalletActivityHistoryReq {
    pub address: String,
    pub blockchain: EnumBlockChain,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherListWalletActivityHistoryRespRow {
    pub wallet_activity_history_id: i64,
    pub address: String,
    pub transaction_hash: String,
    pub blockchain: EnumBlockChain,
    #[serde(default)]
    pub dex: Option<String>,
    pub contract_address: String,
    #[serde(default)]
    pub token_in_address: Option<String>,
    #[serde(default)]
    pub token_out_address: Option<String>,
    pub caller_address: String,
    #[serde(default)]
    pub amount_in: Option<String>,
    #[serde(default)]
    pub amount_out: Option<String>,
    #[serde(default)]
    pub swap_calls: Option<serde_json::Value>,
    #[serde(default)]
    pub paths: Option<serde_json::Value>,
    #[serde(default)]
    pub dex_versions: Option<serde_json::Value>,
    #[serde(default)]
    pub created_at: Option<i64>,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunWatcherListWalletActivityHistoryReq {
    type ResponseRow = FunWatcherListWalletActivityHistoryRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_watcher_list_wallet_activity_history(a_address => $1::varchar, a_blockchain => $2::enum_block_chain);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.address as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
        ]
    }
    fn parse_row(&self, row: Row) -> Result<FunWatcherListWalletActivityHistoryRespRow> {
        let r = FunWatcherListWalletActivityHistoryRespRow {
            wallet_activity_history_id: row
                .try_get(0)
                .context("failed to get field wallet_activity_history_id")?,
            address: row.try_get(1).context("failed to get field address")?,
            transaction_hash: row
                .try_get(2)
                .context("failed to get field transaction_hash")?,
            blockchain: row.try_get(3).context("failed to get field blockchain")?,
            dex: row.try_get(4).context("failed to get field dex")?,
            contract_address: row
                .try_get(5)
                .context("failed to get field contract_address")?,
            token_in_address: row
                .try_get(6)
                .context("failed to get field token_in_address")?,
            token_out_address: row
                .try_get(7)
                .context("failed to get field token_out_address")?,
            caller_address: row
                .try_get(8)
                .context("failed to get field caller_address")?,
            amount_in: row.try_get(9).context("failed to get field amount_in")?,
            amount_out: row.try_get(10).context("failed to get field amount_out")?,
            swap_calls: row.try_get(11).context("failed to get field swap_calls")?,
            paths: row.try_get(12).context("failed to get field paths")?,
            dex_versions: row
                .try_get(13)
                .context("failed to get field dex_versions")?,
            created_at: row.try_get(14).context("failed to get field created_at")?,
        };
        Ok(r)
    }
}
