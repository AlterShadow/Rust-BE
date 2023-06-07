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
    pub public_user_id: i64,
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
            user_id: row.try_get(0)?,
            public_user_id: row.try_get(1)?,
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
            user_id: row.try_get(0)?,
            role: row.try_get(1)?,
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
    pub net_value: f64,
    pub followers: i32,
    pub backers: i32,
    pub risk_score: f64,
    pub aum: f64,
}

#[allow(unused_variables)]
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
    pub net_value: f64,
    pub followers: i64,
    pub backers: i64,
    pub risk_score: f64,
    pub aum: f64,
}

#[allow(unused_variables)]
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
}

#[allow(unused_variables)]
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
            current_usdc: row.try_get(3)?,
            total_backed_usdc: row.try_get(4)?,
            total_exited_usdc: row.try_get(5)?,
            followers: row.try_get(6)?,
            backers: row.try_get(7)?,
            risk_score: row.try_get(8)?,
            aum: row.try_get(9)?,
            evm_contract_address: row.try_get(10)?,
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
            time: row.try_get(0)?,
            backer_count: row.try_get(1)?,
            backer_quantity_usd: row.try_get(2)?,
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
            success: row.try_get(0)?,
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
    pub net_value: f64,
    pub followers: i32,
    pub backers: i32,
    pub risk_score: f64,
    pub aum: f64,
}

#[allow(unused_variables)]
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
            back_history_id: row.try_get(0)?,
            strategy_id: row.try_get(1)?,
            quantity: row.try_get(2)?,
            wallet_address: row.try_get(3)?,
            blockchain: row.try_get(4)?,
            transaction_hash: row.try_get(5)?,
            time: row.try_get(6)?,
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
            success: row.try_get(0)?,
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
    pub follower_count: i64,
    pub description: String,
    pub social_media: String,
    pub risk_score: f64,
    pub reputation_score: f64,
    pub aum: f64,
}

#[allow(unused_variables)]
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
    pub follower_count: i64,
    pub description: String,
    pub social_media: String,
    pub risk_score: f64,
    pub reputation_score: f64,
    pub aum: f64,
}

#[allow(unused_variables)]
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
    pub risk_score: f64,
    pub reputation_score: f64,
    pub aum: f64,
}

#[allow(unused_variables)]
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
    pub risk_score: f64,
    pub reputation_score: f64,
    pub aum: f64,
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
pub struct FunUserApplyBecomeExpertReq {
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserApplyBecomeExpertRespRow {
    pub success: bool,
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
            success: row.try_get(0)?,
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

#[allow(unused_variables)]
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
            success: row.try_get(0)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminUpdateStrategyReq {
    pub user_id: i64,
    pub strategy_id: i64,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub evm_contract_address: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminUpdateStrategyRespRow {
    pub success: bool,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminUpdateStrategyReq {
    type ResponseRow = FunAdminUpdateStrategyRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_update_strategy(a_user_id => $1::bigint, a_strategy_id => $2::bigint, a_name => $3::varchar, a_description => $4::varchar, a_evm_contract_address => $5::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id as &(dyn ToSql + Sync),
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.name as &(dyn ToSql + Sync),
            &self.description as &(dyn ToSql + Sync),
            &self.evm_contract_address as &(dyn ToSql + Sync),
        ]
    }
    fn parse_row(&self, row: Row) -> Result<FunAdminUpdateStrategyRespRow> {
        let r = FunAdminUpdateStrategyRespRow {
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
            watch_wallet_id: row.try_get(0)?,
            wallet_address: row.try_get(1)?,
            blockchain: row.try_get(2)?,
            ratio: row.try_get(3)?,
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
            registered_wallet_id: row.try_get(0)?,
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
            registered_wallet_id: row.try_get(0)?,
            blockchain: row.try_get(1)?,
            address: row.try_get(2)?,
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
            request_refund_id: row.try_get(0)?,
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
            request_refund_id: row.try_get(0)?,
            user_id: row.try_get(1)?,
            blockchain: row.try_get(2)?,
            quantity: row.try_get(3)?,
            wallet_address: row.try_get(4)?,
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
        "SELECT * FROM api.fun_user_add_strategy_initial_token_ratio(a_strategy_id => $1::bigint, a_token_name => $2::varchar, a_token_address => $3::varchar, a_quantity => $4::varchar);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.strategy_id as &(dyn ToSql + Sync),
            &self.token_name as &(dyn ToSql + Sync),
            &self.token_address as &(dyn ToSql + Sync),
            &self.quantity as &(dyn ToSql + Sync),
        ]
    }
    fn parse_row(&self, row: Row) -> Result<FunUserAddStrategyInitialTokenRatioRespRow> {
        let r = FunUserAddStrategyInitialTokenRatioRespRow {
            strategy_initial_token_ratio_id: row.try_get(0)?,
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
            strategy_initial_token_ratio_id: row.try_get(0)?,
            token_name: row.try_get(1)?,
            token_address: row.try_get(2)?,
            quantity: row.try_get(3)?,
            strategy_id: row.try_get(4)?,
            created_at: row.try_get(5)?,
            updated_at: row.try_get(6)?,
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
            user_id: row.try_get(0)?,
            public_user_id: row.try_get(1)?,
            username: row.try_get(2)?,
            address: row.try_get(3)?,
            last_ip: row.try_get(4)?,
            last_login_at: row.try_get(5)?,
            login_count: row.try_get(6)?,
            role: row.try_get(7)?,
            email: row.try_get(8)?,
            updated_at: row.try_get(9)?,
            created_at: row.try_get(10)?,
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
pub struct FunAdminApproveUserBecomeAdminReq {
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminApproveUserBecomeAdminRespRow {
    pub success: bool,
}

#[allow(unused_variables)]
impl DatabaseRequest for FunAdminApproveUserBecomeAdminReq {
    type ResponseRow = FunAdminApproveUserBecomeAdminRespRow;
    fn statement(&self) -> &str {
        "SELECT * FROM api.fun_admin_approve_user_become_admin(a_user_id => $1::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.user_id as &(dyn ToSql + Sync)]
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

#[allow(unused_variables)]
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
    #[serde(default)]
    pub name: Option<String>,
    pub follower_count: i64,
    pub description: String,
    pub social_media: String,
    pub risk_score: f64,
    pub reputation_score: f64,
    pub aum: f64,
}

#[allow(unused_variables)]
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
            transaction_cache_id: row.try_get(0)?,
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
            transaction_cache_id: row.try_get(0)?,
            transaction_hash: row.try_get(1)?,
            chain: row.try_get(2)?,
            dex: row.try_get(3)?,
            raw_transaction: row.try_get(4)?,
            created_at: row.try_get(5)?,
        };
        Ok(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunWatcherSaveWalletActivityHistoryReq {
    pub address: String,
    pub transaction_hash: String,
    pub blockchain: EnumBlockChain,
    pub dex: String,
    pub contract_address: String,
    pub token_in_address: String,
    pub token_out_address: String,
    pub caller_address: String,
    pub amount_in: String,
    pub amount_out: String,
    pub swap_calls: serde_json::Value,
    pub paths: serde_json::Value,
    pub dex_versions: serde_json::Value,
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
        "SELECT * FROM api.fun_watcher_save_wallet_activity_history(a_address => $1::varchar, a_transaction_hash => $2::varchar, a_blockchain => $3::enum_block_chain, a_dex => $4::varchar, a_contract_address => $5::varchar, a_token_in_address => $6::varchar, a_token_out_address => $7::varchar, a_caller_address => $8::varchar, a_amount_in => $9::varchar, a_amount_out => $10::varchar, a_swap_calls => $11::jsonb, a_paths => $12::jsonb, a_dex_versions => $13::jsonb, a_created_at => $14::bigint);"
    }
    fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.address as &(dyn ToSql + Sync),
            &self.transaction_hash as &(dyn ToSql + Sync),
            &self.blockchain as &(dyn ToSql + Sync),
            &self.dex as &(dyn ToSql + Sync),
            &self.contract_address as &(dyn ToSql + Sync),
            &self.token_in_address as &(dyn ToSql + Sync),
            &self.token_out_address as &(dyn ToSql + Sync),
            &self.caller_address as &(dyn ToSql + Sync),
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
            wallet_activity_history_id: row.try_get(0)?,
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
    pub dex: String,
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
            wallet_activity_history_id: row.try_get(0)?,
            address: row.try_get(1)?,
            transaction_hash: row.try_get(2)?,
            blockchain: row.try_get(3)?,
            dex: row.try_get(4)?,
            contract_address: row.try_get(5)?,
            token_in_address: row.try_get(6)?,
            token_out_address: row.try_get(7)?,
            caller_address: row.try_get(8)?,
            amount_in: row.try_get(9)?,
            amount_out: row.try_get(10)?,
            swap_calls: row.try_get(11)?,
            paths: row.try_get(12)?,
            dex_versions: row.try_get(13)?,
            created_at: row.try_get(14)?,
        };
        Ok(r)
    }
}
