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
    pub username: Option<String>,
    pub role: Option<EnumRole>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminListUsersRespRow {
    pub user_id: i64,
    pub email: String,
    pub username: String,
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
        let rows = self.client.query("SELECT * FROM api.fun_admin_list_users(a_offset => $1::int, a_limit => $2::int, a_user_id => $3::bigint, a_email => $4::varchar, a_username => $5::varchar, a_role => $6::enum_role);", &[&req.offset, &req.limit, &req.user_id, &req.email, &req.username, &req.role]).await?;
        let mut resp = FunAdminListUsersResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAdminListUsersRespRow {
                user_id: row.try_get(0)?,
                email: row.try_get(1)?,
                username: row.try_get(2)?,
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
pub struct FunUserCreateOrganizationReq {
    pub user_id: i64,
    pub name: String,
    pub country: String,
    pub tax_id: String,
    pub address: String,
    pub note: String,
    pub approved: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserCreateOrganizationRespRow {
    pub organization_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserCreateOrganizationResp {
    pub rows: Vec<FunUserCreateOrganizationRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_create_organization(
        &self,
        req: FunUserCreateOrganizationReq,
    ) -> Result<FunUserCreateOrganizationResp> {
        let rows = self.client.query("SELECT * FROM api.fun_user_create_organization(a_user_id => $1::bigint, a_name => $2::varchar, a_country => $3::varchar, a_tax_id => $4::varchar, a_address => $5::varchar, a_note => $6::varchar, a_approved => $7::boolean);", &[&req.user_id, &req.name, &req.country, &req.tax_id, &req.address, &req.note, &req.approved]).await?;
        let mut resp = FunUserCreateOrganizationResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunUserCreateOrganizationRespRow {
                organization_id: row.try_get(0)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserLookupUserByEmailOrUsernameReq {
    pub email: String,
    pub username: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserLookupUserByEmailOrUsernameRespRow {
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserLookupUserByEmailOrUsernameResp {
    pub rows: Vec<FunUserLookupUserByEmailOrUsernameRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_lookup_user_by_email_or_username(
        &self,
        req: FunUserLookupUserByEmailOrUsernameReq,
    ) -> Result<FunUserLookupUserByEmailOrUsernameResp> {
        let rows = self.client.query("SELECT * FROM api.fun_user_lookup_user_by_email_or_username(a_email => $1::varchar, a_username => $2::varchar);", &[&req.email, &req.username]).await?;
        let mut resp = FunUserLookupUserByEmailOrUsernameResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunUserLookupUserByEmailOrUsernameRespRow {
                user_id: row.try_get(0)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetOrganizationMembershipReq {
    pub user_id: i64,
    pub organization_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetOrganizationMembershipRespRow {
    pub role: EnumRole,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserGetOrganizationMembershipResp {
    pub rows: Vec<FunUserGetOrganizationMembershipRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_get_organization_membership(
        &self,
        req: FunUserGetOrganizationMembershipReq,
    ) -> Result<FunUserGetOrganizationMembershipResp> {
        let rows = self.client.query("SELECT * FROM api.fun_user_get_organization_membership(a_user_id => $1::bigint, a_organization_id => $2::bigint);", &[&req.user_id, &req.organization_id]).await?;
        let mut resp = FunUserGetOrganizationMembershipResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunUserGetOrganizationMembershipRespRow {
                role: row.try_get(0)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListOrganizationsReq {
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListOrganizationsRespRow {
    pub organization_id: i64,
    pub name: String,
    pub role: EnumRole,
    pub accepted: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListOrganizationsResp {
    pub rows: Vec<FunUserListOrganizationsRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_list_organizations(
        &self,
        req: FunUserListOrganizationsReq,
    ) -> Result<FunUserListOrganizationsResp> {
        let rows = self
            .client
            .query(
                "SELECT * FROM api.fun_user_list_organizations(a_user_id => $1::bigint);",
                &[&req.user_id],
            )
            .await?;
        let mut resp = FunUserListOrganizationsResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunUserListOrganizationsRespRow {
                organization_id: row.try_get(0)?,
                name: row.try_get(1)?,
                role: row.try_get(2)?,
                accepted: row.try_get(3)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
