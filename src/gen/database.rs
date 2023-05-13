use crate::model::*;
use eyre::*;
use lib::database::*;
use serde::*;

#[derive(Clone)]
pub struct DbClient {
    pub client: SimpleDbClient,
}
impl DbClient {
    pub fn new(client: SimpleDbClient) -> Self {
        Self { client }
    }
}
impl From<SimpleDbClient> for DbClient {
    fn from(client: SimpleDbClient) -> Self {
        Self::new(client)
    }
}

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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthSignupResp {
    pub rows: Vec<FunAuthSignupRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_auth_signup(&self, req: FunAuthSignupReq) -> Result<FunAuthSignupResp> {
        let rows = self.client.query("SELECT * FROM api.fun_auth_signup(a_address => $1::varchar, a_email => $2::varchar, a_phone => $3::varchar, a_age => $4::int, a_preferred_language => $5::varchar, a_agreed_tos => $6::boolean, a_agreed_privacy => $7::boolean, a_ip_address => $8::inet);", &[&req.address, &req.email, &req.phone, &req.age, &req.preferred_language, &req.agreed_tos, &req.agreed_privacy, &req.ip_address]).await?;
        let mut resp = FunAuthSignupResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAuthSignupRespRow {
                user_id: row.try_get(0)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthAuthenticateResp {
    pub rows: Vec<FunAuthAuthenticateRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_auth_authenticate(
        &self,
        req: FunAuthAuthenticateReq,
    ) -> Result<FunAuthAuthenticateResp> {
        let rows = self.client.query("SELECT * FROM api.fun_auth_authenticate(a_address => $1::varchar, a_service_code => $2::int, a_device_id => $3::varchar, a_device_os => $4::varchar, a_ip_address => $5::inet);", &[&req.address, &req.service_code, &req.device_id, &req.device_os, &req.ip_address]).await?;
        let mut resp = FunAuthAuthenticateResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAuthAuthenticateRespRow {
                user_id: row.try_get(0)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthSetTokenResp {
    pub rows: Vec<FunAuthSetTokenRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_auth_set_token(&self, req: FunAuthSetTokenReq) -> Result<FunAuthSetTokenResp> {
        let rows = self.client.query("SELECT * FROM api.fun_auth_set_token(a_user_id => $1::bigint, a_user_token => $2::uuid, a_admin_token => $3::uuid, a_service_code => $4::int);", &[&req.user_id, &req.user_token, &req.admin_token, &req.service_code]).await?;
        let mut resp = FunAuthSetTokenResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAuthSetTokenRespRow {};
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthAuthorizeResp {
    pub rows: Vec<FunAuthAuthorizeRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_auth_authorize(
        &self,
        req: FunAuthAuthorizeReq,
    ) -> Result<FunAuthAuthorizeResp> {
        let rows = self.client.query("SELECT * FROM api.fun_auth_authorize(a_address => $1::varchar, a_token => $2::uuid, a_service => $3::enum_service, a_device_id => $4::varchar, a_device_os => $5::varchar, a_ip_address => $6::inet);", &[&req.address, &req.token, &req.service, &req.device_id, &req.device_os, &req.ip_address]).await?;
        let mut resp = FunAuthAuthorizeResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAuthAuthorizeRespRow {
                user_id: row.try_get(0)?,
                role: row.try_get(1)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthBasicAuthenticateResp {
    pub rows: Vec<FunAuthBasicAuthenticateRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_auth_basic_authenticate(
        &self,
        req: FunAuthBasicAuthenticateReq,
    ) -> Result<FunAuthBasicAuthenticateResp> {
        let rows = self.client.query("SELECT * FROM api.fun_auth_basic_authenticate(a_address => $1::varchar, a_device_id => $2::varchar, a_device_os => $3::varchar, a_ip_address => $4::inet);", &[&req.address, &req.device_id, &req.device_os, &req.ip_address]).await?;
        let mut resp = FunAuthBasicAuthenticateResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAuthBasicAuthenticateRespRow {
                user_id: row.try_get(0)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminListUsersResp {
    pub rows: Vec<FunAdminListUsersRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_admin_list_users(
        &self,
        req: FunAdminListUsersReq,
    ) -> Result<FunAdminListUsersResp> {
        let rows = self.client.query("SELECT * FROM api.fun_admin_list_users(a_offset => $1::int, a_limit => $2::int, a_user_id => $3::bigint, a_email => $4::varchar, a_address => $5::varchar, a_role => $6::enum_role);", &[&req.offset, &req.limit, &req.user_id, &req.email, &req.address, &req.role]).await?;
        let mut resp = FunAdminListUsersResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAdminListUsersRespRow {
                user_id: row.try_get(0)?,
                email: row.try_get(1)?,
                address: row.try_get(2)?,
                role: row.try_get(3)?,
                updated_at: row.try_get(4)?,
                created_at: row.try_get(5)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminAssignRoleResp {
    pub rows: Vec<FunAdminAssignRoleRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_admin_assign_role(
        &self,
        req: FunAdminAssignRoleReq,
    ) -> Result<FunAdminAssignRoleResp> {
        let rows = self.client.query("SELECT * FROM api.fun_admin_assign_role(a_operator_user_id => $1::bigint, a_user_id => $2::bigint, a_new_role => $3::enum_role);", &[&req.operator_user_id, &req.user_id, &req.new_role]).await?;
        let mut resp = FunAdminAssignRoleResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAdminAssignRoleRespRow {};
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserFollowStrategyResp {
    pub rows: Vec<FunUserFollowStrategyRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_follow_strategy(
        &self,
        req: FunUserFollowStrategyReq,
    ) -> Result<FunUserFollowStrategyResp> {
        let rows = self.client.query("SELECT * FROM api.fun_user_follow_strategy(a_user_id => $1::bigint, a_strategy_id => $2::bigint);", &[&req.user_id, &req.strategy_id]).await?;
        let mut resp = FunUserFollowStrategyResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunUserFollowStrategyRespRow {
                success: row.try_get(0)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserUnfollowStrategyResp {
    pub rows: Vec<FunUserUnfollowStrategyRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_unfollow_strategy(
        &self,
        req: FunUserUnfollowStrategyReq,
    ) -> Result<FunUserUnfollowStrategyResp> {
        let rows = self.client.query("SELECT * FROM api.fun_user_unfollow_strategy(a_user_id => $1::bigint, a_strategy_id => $2::bigint);", &[&req.user_id, &req.strategy_id]).await?;
        let mut resp = FunUserUnfollowStrategyResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunUserUnfollowStrategyRespRow {
                success: row.try_get(0)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListFollowedStrategiesResp {
    pub rows: Vec<FunUserListFollowedStrategiesRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_list_followed_strategies(
        &self,
        req: FunUserListFollowedStrategiesReq,
    ) -> Result<FunUserListFollowedStrategiesResp> {
        let rows = self
            .client
            .query(
                "SELECT * FROM api.fun_user_list_followed_strategies(a_user_id => $1::bigint);",
                &[&req.user_id],
            )
            .await?;
        let mut resp = FunUserListFollowedStrategiesResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
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
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListStrategiesResp {
    pub rows: Vec<FunUserListStrategiesRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_list_strategies(
        &self,
        req: FunUserListStrategiesReq,
    ) -> Result<FunUserListStrategiesResp> {
        let rows = self
            .client
            .query("SELECT * FROM api.fun_user_list_strategies();", &[])
            .await?;
        let mut resp = FunUserListStrategiesResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
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
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetStrategyResp {
    pub rows: Vec<FunUserGetStrategyRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_get_strategy(
        &self,
        req: FunUserGetStrategyReq,
    ) -> Result<FunUserGetStrategyResp> {
        let rows = self
            .client
            .query(
                "SELECT * FROM api.fun_user_get_strategy(a_strategy_id => $1::bigint);",
                &[&req.strategy_id],
            )
            .await?;
        let mut resp = FunUserGetStrategyResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
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
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetStrategyStatisticsNetValueResp {
    pub rows: Vec<FunUserGetStrategyStatisticsNetValueRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_get_strategy_statistics_net_value(
        &self,
        req: FunUserGetStrategyStatisticsNetValueReq,
    ) -> Result<FunUserGetStrategyStatisticsNetValueResp> {
        let rows = self.client.query("SELECT * FROM api.fun_user_get_strategy_statistics_net_value(a_strategy_id => $1::bigint);", &[&req.strategy_id]).await?;
        let mut resp = FunUserGetStrategyStatisticsNetValueResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunUserGetStrategyStatisticsNetValueRespRow {
                time: row.try_get(0)?,
                net_value: row.try_get(1)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetStrategyStatisticsFollowHistoryResp {
    pub rows: Vec<FunUserGetStrategyStatisticsFollowHistoryRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_get_strategy_statistics_follow_history(
        &self,
        req: FunUserGetStrategyStatisticsFollowHistoryReq,
    ) -> Result<FunUserGetStrategyStatisticsFollowHistoryResp> {
        let rows = self.client.query("SELECT * FROM api.fun_user_get_strategy_statistics_follow_history(a_strategy_id => $1::bigint);", &[&req.strategy_id]).await?;
        let mut resp = FunUserGetStrategyStatisticsFollowHistoryResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunUserGetStrategyStatisticsFollowHistoryRespRow {
                time: row.try_get(0)?,
                follower_count: row.try_get(1)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetStrategyStatisticsBackHistoryResp {
    pub rows: Vec<FunUserGetStrategyStatisticsBackHistoryRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_get_strategy_statistics_back_history(
        &self,
        req: FunUserGetStrategyStatisticsBackHistoryReq,
    ) -> Result<FunUserGetStrategyStatisticsBackHistoryResp> {
        let rows = self.client.query("SELECT * FROM api.fun_user_get_strategy_statistics_back_history(a_strategy_id => $1::bigint);", &[&req.strategy_id]).await?;
        let mut resp = FunUserGetStrategyStatisticsBackHistoryResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunUserGetStrategyStatisticsBackHistoryRespRow {
                time: row.try_get(0)?,
                backer_count: row.try_get(1)?,
                backer_quantity_usd: row.try_get(2)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserBackStrategyReq {
    pub user_id: i64,
    pub strategy_id: i64,
    pub quantity: f32,
    pub blockchain: String,
    pub dex: String,
    pub transaction_hash: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserBackStrategyRespRow {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserBackStrategyResp {
    pub rows: Vec<FunUserBackStrategyRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_back_strategy(
        &self,
        req: FunUserBackStrategyReq,
    ) -> Result<FunUserBackStrategyResp> {
        let rows = self.client.query("SELECT * FROM api.fun_user_back_strategy(a_user_id => $1::bigint, a_strategy_id => $2::bigint, a_quantity => $3::real, a_blockchain => $4::varchar, a_dex => $5::varchar, a_transaction_hash => $6::varchar);", &[&req.user_id, &req.strategy_id, &req.quantity, &req.blockchain, &req.dex, &req.transaction_hash]).await?;
        let mut resp = FunUserBackStrategyResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunUserBackStrategyRespRow {
                success: row.try_get(0)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListBackedStrategiesResp {
    pub rows: Vec<FunUserListBackedStrategiesRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_list_backed_strategies(
        &self,
        req: FunUserListBackedStrategiesReq,
    ) -> Result<FunUserListBackedStrategiesResp> {
        let rows = self
            .client
            .query(
                "SELECT * FROM api.fun_user_list_backed_strategies(a_user_id => $1::bigint);",
                &[&req.user_id],
            )
            .await?;
        let mut resp = FunUserListBackedStrategiesResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
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
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListBackStrategyHistoryReq {
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListBackStrategyHistoryRespRow {
    pub back_history_id: i64,
    pub strategy_id: i64,
    pub quantity: f32,
    pub blockchain: String,
    pub dex: String,
    pub transaction_hash: String,
    pub time: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListBackStrategyHistoryResp {
    pub rows: Vec<FunUserListBackStrategyHistoryRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_list_back_strategy_history(
        &self,
        req: FunUserListBackStrategyHistoryReq,
    ) -> Result<FunUserListBackStrategyHistoryResp> {
        let rows = self
            .client
            .query(
                "SELECT * FROM api.fun_user_list_back_strategy_history(a_user_id => $1::bigint);",
                &[&req.user_id],
            )
            .await?;
        let mut resp = FunUserListBackStrategyHistoryResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunUserListBackStrategyHistoryRespRow {
                back_history_id: row.try_get(0)?,
                strategy_id: row.try_get(1)?,
                quantity: row.try_get(2)?,
                blockchain: row.try_get(3)?,
                dex: row.try_get(4)?,
                transaction_hash: row.try_get(5)?,
                time: row.try_get(6)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserUserExitStrategyReq {
    pub user_id: i64,
    pub strategy_id: i64,
    pub quantity: f32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserUserExitStrategyRespRow {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserUserExitStrategyResp {
    pub rows: Vec<FunUserUserExitStrategyRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_user_exit_strategy(
        &self,
        req: FunUserUserExitStrategyReq,
    ) -> Result<FunUserUserExitStrategyResp> {
        let rows = self.client.query("SELECT * FROM api.fun_user_user_exit_strategy(a_user_id => $1::bigint, a_strategy_id => $2::bigint, a_quantity => $3::real);", &[&req.user_id, &req.strategy_id, &req.quantity]).await?;
        let mut resp = FunUserUserExitStrategyResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunUserUserExitStrategyRespRow {
                success: row.try_get(0)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListExitStrategyHistoryResp {
    pub rows: Vec<FunUserListExitStrategyHistoryRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_list_exit_strategy_history(
        &self,
        req: FunUserListExitStrategyHistoryReq,
    ) -> Result<FunUserListExitStrategyHistoryResp> {
        let rows = self.client.query("SELECT * FROM api.fun_user_list_exit_strategy_history(a_user_id => $1::bigint, a_strategy_id => $2::bigint);", &[&req.user_id, &req.strategy_id]).await?;
        let mut resp = FunUserListExitStrategyHistoryResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
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
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserFollowExpertResp {
    pub rows: Vec<FunUserFollowExpertRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_follow_expert(
        &self,
        req: FunUserFollowExpertReq,
    ) -> Result<FunUserFollowExpertResp> {
        let rows = self.client.query("SELECT * FROM api.fun_user_follow_expert(a_user_id => $1::bigint, a_expert_id => $2::bigint);", &[&req.user_id, &req.expert_id]).await?;
        let mut resp = FunUserFollowExpertResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunUserFollowExpertRespRow {
                success: row.try_get(0)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserUnfollowExpertResp {
    pub rows: Vec<FunUserUnfollowExpertRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_unfollow_expert(
        &self,
        req: FunUserUnfollowExpertReq,
    ) -> Result<FunUserUnfollowExpertResp> {
        let rows = self.client.query("SELECT * FROM api.fun_user_unfollow_expert(a_user_id => $1::bigint, a_expert_id => $2::bigint);", &[&req.user_id, &req.expert_id]).await?;
        let mut resp = FunUserUnfollowExpertResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunUserUnfollowExpertRespRow {
                success: row.try_get(0)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListFollowedExpertsResp {
    pub rows: Vec<FunUserListFollowedExpertsRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_list_followed_experts(
        &self,
        req: FunUserListFollowedExpertsReq,
    ) -> Result<FunUserListFollowedExpertsResp> {
        let rows = self
            .client
            .query(
                "SELECT * FROM api.fun_user_list_followed_experts(a_user_id => $1::bigint);",
                &[&req.user_id],
            )
            .await?;
        let mut resp = FunUserListFollowedExpertsResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
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
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListExpertsResp {
    pub rows: Vec<FunUserListExpertsRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_list_experts(
        &self,
        req: FunUserListExpertsReq,
    ) -> Result<FunUserListExpertsResp> {
        let rows = self
            .client
            .query("SELECT * FROM api.fun_user_list_experts();", &[])
            .await?;
        let mut resp = FunUserListExpertsResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
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
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetExpertProfileResp {
    pub rows: Vec<FunUserGetExpertProfileRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_get_expert_profile(
        &self,
        req: FunUserGetExpertProfileReq,
    ) -> Result<FunUserGetExpertProfileResp> {
        let rows = self
            .client
            .query(
                "SELECT * FROM api.fun_user_get_expert_profile(a_expert_id => $1::bigint);",
                &[&req.expert_id],
            )
            .await?;
        let mut resp = FunUserGetExpertProfileResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
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
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetUserProfileResp {
    pub rows: Vec<FunUserGetUserProfileRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_get_user_profile(
        &self,
        req: FunUserGetUserProfileReq,
    ) -> Result<FunUserGetUserProfileResp> {
        let rows = self
            .client
            .query(
                "SELECT * FROM api.fun_user_get_user_profile(a_user_id => $1::bigint);",
                &[&req.user_id],
            )
            .await?;
        let mut resp = FunUserGetUserProfileResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
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
            resp.rows.push(r);
        }
        Ok(resp)
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
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserRegisterWalletResp {
    pub rows: Vec<FunUserRegisterWalletRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_register_wallet(
        &self,
        req: FunUserRegisterWalletReq,
    ) -> Result<FunUserRegisterWalletResp> {
        let rows = self.client.query("SELECT * FROM api.fun_user_register_wallet(a_user_id => $1::bigint, a_blockchain => $2::varchar, a_wallet_address => $3::varchar);", &[&req.user_id, &req.blockchain, &req.wallet_address]).await?;
        let mut resp = FunUserRegisterWalletResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunUserRegisterWalletRespRow {
                success: row.try_get(0)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserDeregisterWalletReq {
    pub user_id: i64,
    pub blockchain: String,
    pub wallet_address: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserDeregisterWalletRespRow {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserDeregisterWalletResp {
    pub rows: Vec<FunUserDeregisterWalletRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_deregister_wallet(
        &self,
        req: FunUserDeregisterWalletReq,
    ) -> Result<FunUserDeregisterWalletResp> {
        let rows = self.client.query("SELECT * FROM api.fun_user_deregister_wallet(a_user_id => $1::bigint, a_blockchain => $2::varchar, a_wallet_address => $3::varchar);", &[&req.user_id, &req.blockchain, &req.wallet_address]).await?;
        let mut resp = FunUserDeregisterWalletResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunUserDeregisterWalletRespRow {
                success: row.try_get(0)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListWalletsResp {
    pub rows: Vec<FunUserListWalletsRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_list_wallets(
        &self,
        req: FunUserListWalletsReq,
    ) -> Result<FunUserListWalletsResp> {
        let rows = self
            .client
            .query(
                "SELECT * FROM api.fun_user_list_wallets(a_user_id => $1::bigint);",
                &[&req.user_id],
            )
            .await?;
        let mut resp = FunUserListWalletsResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunUserListWalletsRespRow {
                wallet_id: row.try_get(0)?,
                blockchain: row.try_get(1)?,
                wallet_address: row.try_get(2)?,
                is_default: row.try_get(3)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserApplyBecomeExpertResp {
    pub rows: Vec<FunUserApplyBecomeExpertRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_apply_become_expert(
        &self,
        req: FunUserApplyBecomeExpertReq,
    ) -> Result<FunUserApplyBecomeExpertResp> {
        let rows = self
            .client
            .query(
                "SELECT * FROM api.fun_user_apply_become_expert(a_user_id => $1::bigint);",
                &[&req.user_id],
            )
            .await?;
        let mut resp = FunUserApplyBecomeExpertResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunUserApplyBecomeExpertRespRow {
                success: row.try_get(0)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminApplyBecomeExpertReq {
    pub admin_user_id: i64,
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminApplyBecomeExpertRespRow {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminApplyBecomeExpertResp {
    pub rows: Vec<FunAdminApplyBecomeExpertRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_admin_apply_become_expert(
        &self,
        req: FunAdminApplyBecomeExpertReq,
    ) -> Result<FunAdminApplyBecomeExpertResp> {
        let rows = self.client.query("SELECT * FROM api.fun_admin_apply_become_expert(a_admin_user_id => $1::bigint, a_user_id => $2::bigint);", &[&req.admin_user_id, &req.user_id]).await?;
        let mut resp = FunAdminApplyBecomeExpertResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAdminApplyBecomeExpertRespRow {
                success: row.try_get(0)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminListPendingUserExpertApplicationsResp {
    pub rows: Vec<FunAdminListPendingUserExpertApplicationsRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_admin_list_pending_user_expert_applications(
        &self,
        req: FunAdminListPendingUserExpertApplicationsReq,
    ) -> Result<FunAdminListPendingUserExpertApplicationsResp> {
        let rows = self
            .client
            .query(
                "SELECT * FROM api.fun_admin_list_pending_user_expert_applications();",
                &[],
            )
            .await?;
        let mut resp = FunAdminListPendingUserExpertApplicationsResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
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
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
