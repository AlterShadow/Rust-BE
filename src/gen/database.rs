use crate::model::*;
use eyre::*;
use lib::database::*;
use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthSignupReq {
    pub address: String,
    pub email: String,
    pub phone: String,
    pub age: i32,
    pub preferred_language: String,
    pub agreed_tos: bool,
    pub agreed_privacy: bool,
    pub ip_address: std::net::IpAddr,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthSignupRespRow {
    pub user_id: i64,
}

impl DatabaseRequest for FunAuthSignupReq {
    type ResponseRow = FunAuthSignupRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_auth_signup(a_address => $1::varchar, a_email => $2::varchar, a_phone => $3::varchar, a_age => $4::int, a_preferred_language => $5::varchar, a_agreed_tos => $6::boolean, a_agreed_privacy => $7::boolean, a_ip_address => $8::inet);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.address as &(dyn ToSql + Sync),
            &self.email as &(dyn ToSql + Sync),
            &self.phone as &(dyn ToSql + Sync),
            &self.age as &(dyn ToSql + Sync),
            &self.preferred_language as &(dyn ToSql + Sync),
            &self.agreed_tos as &(dyn ToSql + Sync),
            &self.agreed_privacy as &(dyn ToSql + Sync),
            &self.ip_address as &(dyn ToSql + Sync),
        ]
    }
    fn parse_row(&self, row: Row) -> Result<FunAuthSignupRespRow> {
        let r = FunAuthSignupRespRow {
            user_id: row.try_get(0)?,
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
}

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
            user_id: row.try_get(0)?,
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
            user_id: row.try_get(0)?,
            role: row.try_get(1)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthBasicAuthenticateReq {
    pub address: String,
    pub device_id: String,
    pub device_os: String,
    pub ip_address: std::net::IpAddr,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthBasicAuthenticateRespRow {
    pub user_id: std::net::IpAddr,
}

impl DatabaseRequest for FunAuthBasicAuthenticateReq {
    type ResponseRow = FunAuthBasicAuthenticateRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_auth_basic_authenticate(a_address => $1::varchar, a_device_id => $2::varchar, a_device_os => $3::varchar, a_ip_address => $4::inet);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.address as &(dyn ToSql + Sync),
            &self.device_id as &(dyn ToSql + Sync),
            &self.device_os as &(dyn ToSql + Sync),
            &self.ip_address as &(dyn ToSql + Sync),
        ]
    }
    fn parse_row(&self, row: Row) -> Result<FunAuthBasicAuthenticateRespRow> {
        let r = FunAuthBasicAuthenticateRespRow {
            user_id: row.try_get(0)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminListUsersReq {
    pub offset: i32,
    pub limit: i32,
    pub user_id: Option<i64>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub role: Option<EnumRole>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminListUsersRespRow {
    pub user_id: i64,
    pub email: String,
    pub address: String,
    pub role: EnumRole,
    pub updated_at: u32,
    pub created_at: u32,
}

impl DatabaseRequest for FunAdminListUsersReq {
    type ResponseRow = FunAdminListUsersRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_list_users(a_offset => $1::int, a_limit => $2::int, a_user_id => $3::bigint, a_email => $4::varchar, a_address => $5::varchar, a_role => $6::enum_role);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.offset as &(dyn ToSql + Sync),
            &self.limit as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
            &self.email as &(dyn ToSql + Sync),
            &self.address as &(dyn ToSql + Sync),
            &self.role as &(dyn ToSql + Sync),
        ]
    }
    fn parse_row(&self, row: Row) -> Result<FunAdminListUsersRespRow> {
        let r = FunAdminListUsersRespRow {
            user_id: row.try_get(0)?,
            email: row.try_get(1)?,
            address: row.try_get(2)?,
            role: row.try_get(3)?,
            updated_at: row.try_get(4)?,
            created_at: row.try_get(5)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminAssignRoleReq {
    pub operator_user_id: i64,
    pub user_id: i64,
    pub new_role: EnumRole,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminAssignRoleRespRow {}

impl DatabaseRequest for FunAdminAssignRoleReq {
    type ResponseRow = FunAdminAssignRoleRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_assign_role(a_operator_user_id => $1::bigint, a_user_id => $2::bigint, a_new_role => $3::enum_role);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.operator_user_id as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
            &self.new_role as &(dyn ToSql + Sync),
        ]
    }
    fn parse_row(&self, row: Row) -> Result<FunAdminAssignRoleRespRow> {
        let r = FunAdminAssignRoleRespRow {};
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
            success: row.try_get(0)?,
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
            success: row.try_get(0)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListFollowedStrategiesReq {
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListFollowedStrategiesRespRow {
    pub strategy_id: i64,
    pub strategy_name: String,
    pub strategy_description: String,
    pub net_value: f32,
    pub followers: i32,
    pub backers: i32,
    pub risk_score: f32,
    pub aum: f32,
}

impl DatabaseRequest for FunUserListFollowedStrategiesReq {
    type ResponseRow = FunUserListFollowedStrategiesRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_followed_strategies(a_user_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.user_id as &(dyn ToSql + Sync)]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserListFollowedStrategiesRespRow> {
        let r = FunUserListFollowedStrategiesRespRow {
            strategy_id: row.try_get(0)?,
            strategy_name: row.try_get(1)?,
            strategy_description: row.try_get(2)?,
            net_value: row.try_get(3)?,
            followers: row.try_get(4)?,
            backers: row.try_get(5)?,
            risk_score: row.try_get(6)?,
            aum: row.try_get(7)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListStrategiesReq {}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListStrategiesRespRow {
    pub strategy_id: i64,
    pub strategy_name: String,
    pub strategy_description: String,
    pub net_value: f32,
    pub followers: i32,
    pub backers: i32,
    pub risk_score: f32,
    pub aum: f32,
}

impl DatabaseRequest for FunUserListStrategiesReq {
    type ResponseRow = FunUserListStrategiesRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_strategies();"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserListStrategiesRespRow> {
        let r = FunUserListStrategiesRespRow {
            strategy_id: row.try_get(0)?,
            strategy_name: row.try_get(1)?,
            strategy_description: row.try_get(2)?,
            net_value: row.try_get(3)?,
            followers: row.try_get(4)?,
            backers: row.try_get(5)?,
            risk_score: row.try_get(6)?,
            aum: row.try_get(7)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetStrategyReq {
    pub strategy_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetStrategyRespRow {
    pub strategy_id: i64,
    pub strategy_name: String,
    pub strategy_description: String,
    pub net_value: f32,
    pub followers: i32,
    pub backers: i32,
    pub risk_score: f32,
    pub aum: f32,
}

impl DatabaseRequest for FunUserGetStrategyReq {
    type ResponseRow = FunUserGetStrategyRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_get_strategy(a_strategy_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.strategy_id as &(dyn ToSql + Sync)]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserGetStrategyRespRow> {
        let r = FunUserGetStrategyRespRow {
            strategy_id: row.try_get(0)?,
            strategy_name: row.try_get(1)?,
            strategy_description: row.try_get(2)?,
            net_value: row.try_get(3)?,
            followers: row.try_get(4)?,
            backers: row.try_get(5)?,
            risk_score: row.try_get(6)?,
            aum: row.try_get(7)?,
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
    pub net_value: f32,
}

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
            time: row.try_get(0)?,
            net_value: row.try_get(1)?,
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
    pub follower_count: f32,
}

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
            time: row.try_get(0)?,
            follower_count: row.try_get(1)?,
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
    pub backer_count: f32,
    pub backer_quantity_usd: f32,
}

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
            time: row.try_get(0)?,
            backer_count: row.try_get(1)?,
            backer_quantity_usd: row.try_get(2)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserBackStrategyReq {
    pub user_id: i64,
    pub strategy_id: i64,
    pub quantity: f32,
    pub purchase_wallet: String,
    pub blockchain: String,
    pub dex: String,
    pub transaction_hash: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserBackStrategyRespRow {
    pub success: bool,
}

impl DatabaseRequest for FunUserBackStrategyReq {
    type ResponseRow = FunUserBackStrategyRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_back_strategy(a_user_id => $1::bigint, a_strategy_id => $2::bigint, a_quantity => $3::real, a_purchase_wallet => $4::varchar, a_blockchain => $5::varchar, a_dex => $6::varchar, a_transaction_hash => $7::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.quantity as &(dyn ToSql + Sync),
            &self.purchase_wallet as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.dex as &(dyn ToSql + Sync),
            &self.transaction_hash as &(dyn ToSql + Sync),
        ]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserBackStrategyRespRow> {
        let r = FunUserBackStrategyRespRow {
            success: row.try_get(0)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListBackedStrategiesReq {
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListBackedStrategiesRespRow {
    pub strategy_id: i64,
    pub strategy_name: String,
    pub strategy_description: String,
    pub net_value: f32,
    pub followers: i32,
    pub backers: i32,
    pub risk_score: f32,
    pub aum: f32,
}

impl DatabaseRequest for FunUserListBackedStrategiesReq {
    type ResponseRow = FunUserListBackedStrategiesRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_backed_strategies(a_user_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.user_id as &(dyn ToSql + Sync)]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserListBackedStrategiesRespRow> {
        let r = FunUserListBackedStrategiesRespRow {
            strategy_id: row.try_get(0)?,
            strategy_name: row.try_get(1)?,
            strategy_description: row.try_get(2)?,
            net_value: row.try_get(3)?,
            followers: row.try_get(4)?,
            backers: row.try_get(5)?,
            risk_score: row.try_get(6)?,
            aum: row.try_get(7)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListBackStrategyHistoryReq {
    pub user_id: i64,
    pub strategy_id: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListBackStrategyHistoryRespRow {
    pub back_history_id: i64,
    pub strategy_id: i64,
    pub quantity: f32,
    pub wallet_address: String,
    pub blockchain: String,
    pub dex: String,
    pub transaction_hash: String,
    pub time: i64,
}

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
            back_history_id: row.try_get(0)?,
            strategy_id: row.try_get(1)?,
            quantity: row.try_get(2)?,
            wallet_address: row.try_get(3)?,
            blockchain: row.try_get(4)?,
            dex: row.try_get(5)?,
            transaction_hash: row.try_get(6)?,
            time: row.try_get(7)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserExitStrategyReq {
    pub user_id: i64,
    pub strategy_id: i64,
    pub quantity: f32,
    pub blockchain: String,
    pub dex: String,
    pub back_time: i64,
    pub transaction_hash: String,
    pub purchase_wallet: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserExitStrategyRespRow {
    pub success: bool,
}

impl DatabaseRequest for FunUserExitStrategyReq {
    type ResponseRow = FunUserExitStrategyRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_exit_strategy(a_user_id => $1::bigint, a_strategy_id => $2::bigint, a_quantity => $3::real, a_blockchain => $4::varchar, a_dex => $5::varchar, a_back_time => $6::bigint, a_transaction_hash => $7::varchar, a_purchase_wallet => $8::varchar);"
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
            success: row.try_get(0)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListExitStrategyHistoryReq {
    pub user_id: i64,
    pub strategy_id: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListExitStrategyHistoryRespRow {
    pub exit_history_id: i64,
    pub strategy_id: i64,
    pub exit_quantity: f32,
    pub purchase_wallet_address: String,
    pub blockchain: String,
    pub dex: String,
    pub back_time: i64,
    pub exit_time: i64,
}

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
            exit_history_id: row.try_get(0)?,
            strategy_id: row.try_get(1)?,
            exit_quantity: row.try_get(2)?,
            purchase_wallet_address: row.try_get(3)?,
            blockchain: row.try_get(4)?,
            dex: row.try_get(5)?,
            back_time: row.try_get(6)?,
            exit_time: row.try_get(7)?,
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
            success: row.try_get(0)?,
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
            success: row.try_get(0)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListFollowedExpertsReq {
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListFollowedExpertsRespRow {
    pub expert_id: i64,
    pub name: String,
    pub follower_count: i32,
    pub description: String,
    pub social_media: String,
    pub risk_score: f32,
    pub reputation_score: f32,
    pub aum: f32,
}

impl DatabaseRequest for FunUserListFollowedExpertsReq {
    type ResponseRow = FunUserListFollowedExpertsRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_followed_experts(a_user_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.user_id as &(dyn ToSql + Sync)]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserListFollowedExpertsRespRow> {
        let r = FunUserListFollowedExpertsRespRow {
            expert_id: row.try_get(0)?,
            name: row.try_get(1)?,
            follower_count: row.try_get(2)?,
            description: row.try_get(3)?,
            social_media: row.try_get(4)?,
            risk_score: row.try_get(5)?,
            reputation_score: row.try_get(6)?,
            aum: row.try_get(7)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListExpertsReq {}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListExpertsRespRow {
    pub expert_id: i64,
    pub name: String,
    pub follower_count: i32,
    pub description: String,
    pub social_media: String,
    pub risk_score: f32,
    pub reputation_score: f32,
    pub aum: f32,
}

impl DatabaseRequest for FunUserListExpertsReq {
    type ResponseRow = FunUserListExpertsRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_experts();"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserListExpertsRespRow> {
        let r = FunUserListExpertsRespRow {
            expert_id: row.try_get(0)?,
            name: row.try_get(1)?,
            follower_count: row.try_get(2)?,
            description: row.try_get(3)?,
            social_media: row.try_get(4)?,
            risk_score: row.try_get(5)?,
            reputation_score: row.try_get(6)?,
            aum: row.try_get(7)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetExpertProfileReq {
    pub expert_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetExpertProfileRespRow {
    pub expert_id: i64,
    pub name: String,
    pub follower_count: i32,
    pub description: String,
    pub social_media: String,
    pub risk_score: f32,
    pub reputation_score: f32,
    pub aum: f32,
}

impl DatabaseRequest for FunUserGetExpertProfileReq {
    type ResponseRow = FunUserGetExpertProfileRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_get_expert_profile(a_expert_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.expert_id as &(dyn ToSql + Sync)]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserGetExpertProfileRespRow> {
        let r = FunUserGetExpertProfileRespRow {
            expert_id: row.try_get(0)?,
            name: row.try_get(1)?,
            follower_count: row.try_get(2)?,
            description: row.try_get(3)?,
            social_media: row.try_get(4)?,
            risk_score: row.try_get(5)?,
            reputation_score: row.try_get(6)?,
            aum: row.try_get(7)?,
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
    pub user_id: i64,
    pub name: String,
    pub follower_count: i32,
    pub description: String,
    pub social_media: String,
    pub risk_score: f32,
    pub reputation_score: f32,
    pub aum: f32,
}

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
            user_id: row.try_get(0)?,
            name: row.try_get(1)?,
            follower_count: row.try_get(2)?,
            description: row.try_get(3)?,
            social_media: row.try_get(4)?,
            risk_score: row.try_get(5)?,
            reputation_score: row.try_get(6)?,
            aum: row.try_get(7)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserRegisterWalletReq {
    pub user_id: i64,
    pub blockchain: String,
    pub wallet_address: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserRegisterWalletRespRow {
    pub success: bool,
    pub wallet_id: i64,
}

impl DatabaseRequest for FunUserRegisterWalletReq {
    type ResponseRow = FunUserRegisterWalletRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_register_wallet(a_user_id => $1::bigint, a_blockchain => $2::varchar, a_wallet_address => $3::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.wallet_address as &(dyn ToSql + Sync),
        ]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserRegisterWalletRespRow> {
        let r = FunUserRegisterWalletRespRow {
            success: row.try_get(0)?,
            wallet_id: row.try_get(1)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserDeregisterWalletReq {
    pub user_id: i64,
    pub wallet_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserDeregisterWalletRespRow {
    pub success: bool,
}

impl DatabaseRequest for FunUserDeregisterWalletReq {
    type ResponseRow = FunUserDeregisterWalletRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_deregister_wallet(a_user_id => $1::bigint, a_wallet_id => $2::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.wallet_id as &(dyn ToSql + Sync),
        ]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserDeregisterWalletRespRow> {
        let r = FunUserDeregisterWalletRespRow {
            success: row.try_get(0)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListWalletsReq {
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListWalletsRespRow {
    pub wallet_id: i64,
    pub blockchain: String,
    pub wallet_address: String,
    pub is_default: bool,
}

impl DatabaseRequest for FunUserListWalletsReq {
    type ResponseRow = FunUserListWalletsRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_list_wallets(a_user_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.user_id as &(dyn ToSql + Sync)]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserListWalletsRespRow> {
        let r = FunUserListWalletsRespRow {
            wallet_id: row.try_get(0)?,
            blockchain: row.try_get(1)?,
            wallet_address: row.try_get(2)?,
            is_default: row.try_get(3)?,
        };
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
}

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
            success: row.try_get(0)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminApproveUserBecomeAdminReq {
    pub admin_user_id: i64,
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminApproveUserBecomeAdminRespRow {
    pub success: bool,
}

impl DatabaseRequest for FunAdminApproveUserBecomeAdminReq {
    type ResponseRow = FunAdminApproveUserBecomeAdminRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_approve_user_become_admin(a_admin_user_id => $1::bigint, a_user_id => $2::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.admin_user_id as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
        ]
    }
    fn parse_row(&self, row: Row) -> Result<FunAdminApproveUserBecomeAdminRespRow> {
        let r = FunAdminApproveUserBecomeAdminRespRow {
            success: row.try_get(0)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminRejectUserBecomeAdminReq {
    pub admin_user_id: i64,
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminRejectUserBecomeAdminRespRow {
    pub success: bool,
}

impl DatabaseRequest for FunAdminRejectUserBecomeAdminReq {
    type ResponseRow = FunAdminRejectUserBecomeAdminRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_reject_user_become_admin(a_admin_user_id => $1::bigint, a_user_id => $2::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.admin_user_id as &(dyn ToSql + Sync),
            &self.user_id as &(dyn ToSql + Sync),
        ]
    }
    fn parse_row(&self, row: Row) -> Result<FunAdminRejectUserBecomeAdminRespRow> {
        let r = FunAdminRejectUserBecomeAdminRespRow {
            success: row.try_get(0)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminListPendingUserExpertApplicationsReq {}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminListPendingUserExpertApplicationsRespRow {
    pub user_id: i64,
    pub name: String,
    pub follower_count: i32,
    pub description: String,
    pub social_media: String,
    pub risk_score: f32,
    pub reputation_score: f32,
    pub aum: f32,
}

impl DatabaseRequest for FunAdminListPendingUserExpertApplicationsReq {
    type ResponseRow = FunAdminListPendingUserExpertApplicationsRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_list_pending_user_expert_applications();"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![]
    }
    fn parse_row(&self, row: Row) -> Result<FunAdminListPendingUserExpertApplicationsRespRow> {
        let r = FunAdminListPendingUserExpertApplicationsRespRow {
            user_id: row.try_get(0)?,
            name: row.try_get(1)?,
            follower_count: row.try_get(2)?,
            description: row.try_get(3)?,
            social_media: row.try_get(4)?,
            risk_score: row.try_get(5)?,
            reputation_score: row.try_get(6)?,
            aum: row.try_get(7)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserCreateStrategyReq {
    pub user_id: i64,
    pub name: String,
    pub description: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserCreateStrategyRespRow {
    pub success: bool,
    pub strategy_id: i64,
}

impl DatabaseRequest for FunUserCreateStrategyReq {
    type ResponseRow = FunUserCreateStrategyRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_create_strategy(a_user_id => $1::bigint, a_name => $2::varchar, a_description => $3::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.name as &(dyn ToSql + Sync),
            &self.description as &(dyn ToSql + Sync),
        ]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserCreateStrategyRespRow> {
        let r = FunUserCreateStrategyRespRow {
            success: row.try_get(0)?,
            strategy_id: row.try_get(1)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserUpdateStrategyReq {
    pub user_id: i64,
    pub strategy_id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserUpdateStrategyRespRow {
    pub success: bool,
}

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
            success: row.try_get(0)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserAddStrategyWatchWalletReq {
    pub user_id: i64,
    pub strategy_id: i64,
    pub wallet_address: String,
    pub blockchain: String,
    pub ratio: f32,
    pub dex: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserAddStrategyWatchWalletRespRow {
    pub success: bool,
    pub watch_wallet_id: i64,
}

impl DatabaseRequest for FunUserAddStrategyWatchWalletReq {
    type ResponseRow = FunUserAddStrategyWatchWalletRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_user_add_strategy_watch_wallet(a_user_id => $1::bigint, a_strategy_id => $2::bigint, a_wallet_address => $3::varchar, a_blockchain => $4::varchar, a_ratio => $5::real, a_dex => $6::varchar);"
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
            success: row.try_get(0)?,
            watch_wallet_id: row.try_get(1)?,
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
            success: row.try_get(0)?,
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
    pub blockchain: String,
    pub ratio: f32,
}

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
            watch_wallet_id: row.try_get(0)?,
            wallet_address: row.try_get(1)?,
            blockchain: row.try_get(2)?,
            ratio: row.try_get(3)?,
        };
        Ok(r)
    }
}
