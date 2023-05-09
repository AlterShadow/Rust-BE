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
    pub password_hash: Vec<u8>,
    pub password_salt: Vec<u8>,
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
        let rows = self.client.query("SELECT * FROM api.fun_auth_signup(a_address => $1::varchar, a_email => $2::varchar, a_phone => $3::varchar, a_password_hash => $4::bytea, a_password_salt => $5::bytea, a_age => $6::int, a_preferred_language => $7::varchar, a_agreed_tos => $8::boolean, a_agreed_privacy => $9::boolean, a_ip_address => $10::inet);", &[&req.address, &req.email, &req.phone, &req.password_hash, &req.password_salt, &req.age, &req.preferred_language, &req.agreed_tos, &req.agreed_privacy, &req.ip_address]).await?;
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
    pub password_hash: Vec<u8>,
    pub service_code: i32,
    pub device_id: String,
    pub device_os: String,
    pub ip_address: std::net::IpAddr,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthAuthenticateRespRow {
    pub user_id: i64,
    pub user_public_id: i64,
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
        let rows = self.client.query("SELECT * FROM api.fun_auth_authenticate(a_address => $1::varchar, a_password_hash => $2::bytea, a_service_code => $3::int, a_device_id => $4::varchar, a_device_os => $5::varchar, a_ip_address => $6::inet);", &[&req.address, &req.password_hash, &req.service_code, &req.device_id, &req.device_os, &req.ip_address]).await?;
        let mut resp = FunAuthAuthenticateResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAuthAuthenticateRespRow {
                user_id: row.try_get(0)?,
                user_public_id: row.try_get(1)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthGetPasswordSaltReq {
    pub address: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthGetPasswordSaltRespRow {
    pub salt: Vec<u8>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthGetPasswordSaltResp {
    pub rows: Vec<FunAuthGetPasswordSaltRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_auth_get_password_salt(
        &self,
        req: FunAuthGetPasswordSaltReq,
    ) -> Result<FunAuthGetPasswordSaltResp> {
        let rows = self
            .client
            .query(
                "SELECT * FROM api.fun_auth_get_password_salt(a_address => $1::varchar);",
                &[&req.address],
            )
            .await?;
        let mut resp = FunAuthGetPasswordSaltResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAuthGetPasswordSaltRespRow {
                salt: row.try_get(0)?,
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
pub struct FunAuthChangePasswordReq {
    pub address: String,
    pub old_password_hash: Vec<u8>,
    pub new_password_hash: Vec<u8>,
    pub device_id: String,
    pub device_os: String,
    pub ip_address: std::net::IpAddr,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthChangePasswordRespRow {}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthChangePasswordResp {
    pub rows: Vec<FunAuthChangePasswordRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_auth_change_password(
        &self,
        req: FunAuthChangePasswordReq,
    ) -> Result<FunAuthChangePasswordResp> {
        let rows = self.client.query("SELECT * FROM api.fun_auth_change_password(a_address => $1::varchar, a_old_password_hash => $2::bytea, a_new_password_hash => $3::bytea, a_device_id => $4::varchar, a_device_os => $5::varchar, a_ip_address => $6::inet);", &[&req.address, &req.old_password_hash, &req.new_password_hash, &req.device_id, &req.device_os, &req.ip_address]).await?;
        let mut resp = FunAuthChangePasswordResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAuthChangePasswordRespRow {};
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunGetRecoveryQuestionDataReq {}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunGetRecoveryQuestionDataRespRow {
    pub question_id: i32,
    pub content: String,
    pub category: EnumRecoveryQuestionCategory,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunGetRecoveryQuestionDataResp {
    pub rows: Vec<FunGetRecoveryQuestionDataRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_get_recovery_question_data(
        &self,
        req: FunGetRecoveryQuestionDataReq,
    ) -> Result<FunGetRecoveryQuestionDataResp> {
        let rows = self
            .client
            .query("SELECT * FROM api.fun_get_recovery_question_data();", &[])
            .await?;
        let mut resp = FunGetRecoveryQuestionDataResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunGetRecoveryQuestionDataRespRow {
                question_id: row.try_get(0)?,
                content: row.try_get(1)?,
                category: row.try_get(2)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthSetRecoveryQuestionsReq {
    pub user_id: i64,
    pub question_ids: Vec<i32>,
    pub answers: Vec<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthSetRecoveryQuestionsRespRow {}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthSetRecoveryQuestionsResp {
    pub rows: Vec<FunAuthSetRecoveryQuestionsRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_auth_set_recovery_questions(
        &self,
        req: FunAuthSetRecoveryQuestionsReq,
    ) -> Result<FunAuthSetRecoveryQuestionsResp> {
        let rows = self.client.query("SELECT * FROM api.fun_auth_set_recovery_questions(a_user_id => $1::bigint, a_question_ids => $2::int[], a_answers => $3::int[]);", &[&req.user_id, &req.question_ids, &req.answers]).await?;
        let mut resp = FunAuthSetRecoveryQuestionsResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAuthSetRecoveryQuestionsRespRow {};
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
pub struct FunAuthGetRecoveryQuestionsReq {
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthGetRecoveryQuestionsRespRow {
    pub question_id: i32,
    pub question: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthGetRecoveryQuestionsResp {
    pub rows: Vec<FunAuthGetRecoveryQuestionsRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_auth_get_recovery_questions(
        &self,
        req: FunAuthGetRecoveryQuestionsReq,
    ) -> Result<FunAuthGetRecoveryQuestionsResp> {
        let rows = self
            .client
            .query(
                "SELECT * FROM api.fun_auth_get_recovery_questions(a_user_id => $1::bigint);",
                &[&req.user_id],
            )
            .await?;
        let mut resp = FunAuthGetRecoveryQuestionsResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAuthGetRecoveryQuestionsRespRow {
                question_id: row.try_get(0)?,
                question: row.try_get(1)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunSubmitRecoveryAnswersReq {
    pub user_id: i64,
    pub question_ids: Vec<i32>,
    pub answers: Vec<String>,
    pub password_reset_token: uuid::Uuid,
    pub token_valid: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunSubmitRecoveryAnswersRespRow {}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunSubmitRecoveryAnswersResp {
    pub rows: Vec<FunSubmitRecoveryAnswersRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_submit_recovery_answers(
        &self,
        req: FunSubmitRecoveryAnswersReq,
    ) -> Result<FunSubmitRecoveryAnswersResp> {
        let rows = self.client.query("SELECT * FROM api.fun_submit_recovery_answers(a_user_id => $1::bigint, a_question_ids => $2::int[], a_answers => $3::varchar[], a_password_reset_token => $4::uuid, a_token_valid => $5::int);", &[&req.user_id, &req.question_ids, &req.answers, &req.password_reset_token, &req.token_valid]).await?;
        let mut resp = FunSubmitRecoveryAnswersResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunSubmitRecoveryAnswersRespRow {};
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthResetPasswordReq {
    pub user_id: i64,
    pub new_password_hash: Vec<u8>,
    pub new_password_salt: Vec<u8>,
    pub reset_token: uuid::Uuid,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthResetPasswordRespRow {}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAuthResetPasswordResp {
    pub rows: Vec<FunAuthResetPasswordRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_auth_reset_password(
        &self,
        req: FunAuthResetPasswordReq,
    ) -> Result<FunAuthResetPasswordResp> {
        let rows = self.client.query("SELECT * FROM api.fun_auth_reset_password(a_user_id => $1::bigint, a_new_password_hash => $2::bytea, a_new_password_salt => $3::bytea, a_reset_token => $4::uuid);", &[&req.user_id, &req.new_password_hash, &req.new_password_salt, &req.reset_token]).await?;
        let mut resp = FunAuthResetPasswordResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAuthResetPasswordRespRow {};
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
pub struct FunAdminListOrganizationsReq {
    pub offset: i64,
    pub limit: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminListOrganizationsRespRow {
    pub organization_id: i64,
    pub name: String,
    pub description: String,
    pub approved: bool,
    pub member_count: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminListOrganizationsResp {
    pub rows: Vec<FunAdminListOrganizationsRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_admin_list_organizations(
        &self,
        req: FunAdminListOrganizationsReq,
    ) -> Result<FunAdminListOrganizationsResp> {
        let rows = self.client.query("SELECT * FROM api.fun_admin_list_organizations(a_offset => $1::bigint, a_limit => $2::bigint);", &[&req.offset, &req.limit]).await?;
        let mut resp = FunAdminListOrganizationsResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAdminListOrganizationsRespRow {
                organization_id: row.try_get(0)?,
                name: row.try_get(1)?,
                description: row.try_get(2)?,
                approved: row.try_get(3)?,
                member_count: row.try_get(4)?,
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserInviteUserToOrganizationReq {
    pub organization_id: i64,
    pub target_user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserInviteUserToOrganizationRespRow {}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserInviteUserToOrganizationResp {
    pub rows: Vec<FunUserInviteUserToOrganizationRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_invite_user_to_organization(
        &self,
        req: FunUserInviteUserToOrganizationReq,
    ) -> Result<FunUserInviteUserToOrganizationResp> {
        let rows = self.client.query("SELECT * FROM api.fun_user_invite_user_to_organization(a_organization_id => $1::bigint, a_target_user_id => $2::bigint);", &[&req.organization_id, &req.target_user_id]).await?;
        let mut resp = FunUserInviteUserToOrganizationResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunUserInviteUserToOrganizationRespRow {};
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserAcceptOrganizationInvitationReq {
    pub user_id: i64,
    pub organization_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserAcceptOrganizationInvitationRespRow {
    pub membership_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserAcceptOrganizationInvitationResp {
    pub rows: Vec<FunUserAcceptOrganizationInvitationRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_accept_organization_invitation(
        &self,
        req: FunUserAcceptOrganizationInvitationReq,
    ) -> Result<FunUserAcceptOrganizationInvitationResp> {
        let rows = self.client.query("SELECT * FROM api.fun_user_accept_organization_invitation(a_user_id => $1::bigint, a_organization_id => $2::bigint);", &[&req.user_id, &req.organization_id]).await?;
        let mut resp = FunUserAcceptOrganizationInvitationResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunUserAcceptOrganizationInvitationRespRow {
                membership_id: row.try_get(0)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListOrganizationInvitationsByOrganizationReq {
    pub organization_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListOrganizationInvitationsByOrganizationRespRow {
    pub membership_id: i64,
    pub organization_id: i64,
    pub organization_name: String,
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub created_at: u32,
    pub accepted: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListOrganizationInvitationsByOrganizationResp {
    pub rows: Vec<FunUserListOrganizationInvitationsByOrganizationRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_list_organization_invitations_by_organization(
        &self,
        req: FunUserListOrganizationInvitationsByOrganizationReq,
    ) -> Result<FunUserListOrganizationInvitationsByOrganizationResp> {
        let rows = self.client.query("SELECT * FROM api.fun_user_list_organization_invitations_by_organization(a_organization_id => $1::bigint);", &[&req.organization_id]).await?;
        let mut resp = FunUserListOrganizationInvitationsByOrganizationResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunUserListOrganizationInvitationsByOrganizationRespRow {
                membership_id: row.try_get(0)?,
                organization_id: row.try_get(1)?,
                organization_name: row.try_get(2)?,
                user_id: row.try_get(3)?,
                username: row.try_get(4)?,
                email: row.try_get(5)?,
                created_at: row.try_get(6)?,
                accepted: row.try_get(7)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListOrganizationInvitationsByUserReq {
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListOrganizationInvitationsByUserRespRow {
    pub membership_id: i64,
    pub organization_id: i64,
    pub organization_name: String,
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub created_at: u32,
    pub accepted: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserListOrganizationInvitationsByUserResp {
    pub rows: Vec<FunUserListOrganizationInvitationsByUserRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_list_organization_invitations_by_user(
        &self,
        req: FunUserListOrganizationInvitationsByUserReq,
    ) -> Result<FunUserListOrganizationInvitationsByUserResp> {
        let rows = self.client.query("SELECT * FROM api.fun_user_list_organization_invitations_by_user(a_user_id => $1::bigint);", &[&req.user_id]).await?;
        let mut resp = FunUserListOrganizationInvitationsByUserResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunUserListOrganizationInvitationsByUserRespRow {
                membership_id: row.try_get(0)?,
                organization_id: row.try_get(1)?,
                organization_name: row.try_get(2)?,
                user_id: row.try_get(3)?,
                username: row.try_get(4)?,
                email: row.try_get(5)?,
                created_at: row.try_get(6)?,
                accepted: row.try_get(7)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserSetUserRoleInOrganizationReq {
    pub user_id: i64,
    pub organization_id: i64,
    pub new_role: EnumRole,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserSetUserRoleInOrganizationRespRow {}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserSetUserRoleInOrganizationResp {
    pub rows: Vec<FunUserSetUserRoleInOrganizationRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_set_user_role_in_organization(
        &self,
        req: FunUserSetUserRoleInOrganizationReq,
    ) -> Result<FunUserSetUserRoleInOrganizationResp> {
        let rows = self.client.query("SELECT * FROM api.fun_user_set_user_role_in_organization(a_user_id => $1::bigint, a_organization_id => $2::bigint, a_new_role => $3::enum_role);", &[&req.user_id, &req.organization_id, &req.new_role]).await?;
        let mut resp = FunUserSetUserRoleInOrganizationResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunUserSetUserRoleInOrganizationRespRow {};
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserDeleteUserFromOrganizationReq {
    pub user_id: i64,
    pub organization_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserDeleteUserFromOrganizationRespRow {}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunUserDeleteUserFromOrganizationResp {
    pub rows: Vec<FunUserDeleteUserFromOrganizationRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_user_delete_user_from_organization(
        &self,
        req: FunUserDeleteUserFromOrganizationReq,
    ) -> Result<FunUserDeleteUserFromOrganizationResp> {
        let rows = self.client.query("SELECT * FROM api.fun_user_delete_user_from_organization(a_user_id => $1::bigint, a_organization_id => $2::bigint);", &[&req.user_id, &req.organization_id]).await?;
        let mut resp = FunUserDeleteUserFromOrganizationResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunUserDeleteUserFromOrganizationRespRow {};
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminGetOrganizationReq {
    pub organization_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminGetOrganizationRespRow {
    pub organization_id: i64,
    pub name: String,
    pub description: String,
    pub approved: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminGetOrganizationResp {
    pub rows: Vec<FunAdminGetOrganizationRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_admin_get_organization(
        &self,
        req: FunAdminGetOrganizationReq,
    ) -> Result<FunAdminGetOrganizationResp> {
        let rows = self
            .client
            .query(
                "SELECT * FROM api.fun_admin_get_organization(a_organization_id => $1::bigint);",
                &[&req.organization_id],
            )
            .await?;
        let mut resp = FunAdminGetOrganizationResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAdminGetOrganizationRespRow {
                organization_id: row.try_get(0)?,
                name: row.try_get(1)?,
                description: row.try_get(2)?,
                approved: row.try_get(3)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminApproveOrganizationReq {
    pub organization_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminApproveOrganizationRespRow {}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminApproveOrganizationResp {
    pub rows: Vec<FunAdminApproveOrganizationRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_admin_approve_organization(
        &self,
        req: FunAdminApproveOrganizationReq,
    ) -> Result<FunAdminApproveOrganizationResp> {
        let rows = self.client.query("SELECT * FROM api.fun_admin_approve_organization(a_organization_id => $1::bigint);", &[&req.organization_id]).await?;
        let mut resp = FunAdminApproveOrganizationResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAdminApproveOrganizationRespRow {};
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminDisapproveOrganizationReq {
    pub organization_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminDisapproveOrganizationRespRow {}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunAdminDisapproveOrganizationResp {
    pub rows: Vec<FunAdminDisapproveOrganizationRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_admin_disapprove_organization(
        &self,
        req: FunAdminDisapproveOrganizationReq,
    ) -> Result<FunAdminDisapproveOrganizationResp> {
        let rows = self.client.query("SELECT * FROM api.fun_admin_disapprove_organization(a_organization_id => $1::bigint);", &[&req.organization_id]).await?;
        let mut resp = FunAdminDisapproveOrganizationResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAdminDisapproveOrganizationRespRow {};
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
