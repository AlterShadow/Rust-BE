use crate::method::ensure_user_role;
use eyre::ContextCompat;
use gen::database::*;
use gen::model::*;
use lib::database::DbClient;
use lib::handler::{RequestHandler, SpawnedResponse};
use lib::toolbox::{RequestContext, Toolbox};
use lib::ws::WsConnection;
use std::sync::Arc;

pub struct MethodAdminListUsers;
impl RequestHandler for MethodAdminListUsers {
    type Request = AdminListUsersRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> SpawnedResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let ret = db
                .execute(FunAdminListUsersReq {
                    user_id: req.user_id,
                    address: None,
                    username: None,
                    email: None,
                    role: req.role,
                    limit: req.limit,
                    offset: req.offset,
                })
                .await?;

            Ok(AdminListUsersResponse {
                users: ret
                    .into_iter()
                    .map(|x| ListUserRow {
                        user_id: x.user_id,
                        address: x.address,
                        last_ip: x.last_ip,
                        last_login_at: x.last_login_at,
                        username: x.username,
                        email: x.email,
                        role: x.role,
                        created_at: x.created_at,
                        updated_at: x.updated_at,
                        public_user_id: x.public_user_id,
                        login_count: x.login_count,
                    })
                    .collect(),
            })
        })
    }
}
pub struct MethodAdminSetUserRole;
impl RequestHandler for MethodAdminSetUserRole {
    type Request = AdminSetUserRoleRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> SpawnedResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let ret = db
                .execute(FunAdminSetUserRoleReq {
                    user_id: req.user_id,
                    role: req.role,
                })
                .await?;

            Ok(AdminSetUserRoleResponse {})
        })
    }
}
pub struct MethodAdminSetBlockUser;
impl RequestHandler for MethodAdminSetBlockUser {
    type Request = AdminSetBlockUserRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> SpawnedResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let ret = db
                .execute(FunAdminSetBlockUserReq {
                    user_id: req.user_id,
                    blocked: req.blocked,
                })
                .await?;

            Ok(AdminSetBlockUserResponse {})
        })
    }
}
pub struct MethodAdminApproveUserBecomeExpert;
impl RequestHandler for MethodAdminApproveUserBecomeExpert {
    type Request = AdminApproveUserBecomeExpertRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> SpawnedResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let ret = db
                .execute(FunAdminApproveUserBecomeAdminReq {
                    user_id: req.user_id,
                })
                .await?;

            Ok(AdminApproveUserBecomeExpertResponse {
                success: ret
                    .into_result()
                    .context("failed to approve user become expert")?
                    .success,
            })
        })
    }
}
pub struct MethodAdminRejectUserBecomeExpert;
impl RequestHandler for MethodAdminRejectUserBecomeExpert {
    type Request = AdminRejectUserBecomeExpertRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> SpawnedResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let ret = db
                .execute(FunAdminRejectUserBecomeAdminReq {
                    admin_user_id: ctx.user_id,
                    user_id: req.user_id,
                })
                .await?;

            Ok(AdminRejectUserBecomeExpertResponse {
                success: ret
                    .into_result()
                    .context("failed to reject user become expert")?
                    .success,
            })
        })
    }
}
pub struct MethodAdminListPendingExpertApplications;
impl RequestHandler for MethodAdminListPendingExpertApplications {
    type Request = AdminListPendingExpertApplicationsRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        _req: Self::Request,
    ) -> SpawnedResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let ret = db
                .execute(FunAdminListPendingUserExpertApplicationsReq {})
                .await?;

            Ok(AdminListPendingExpertApplicationsResponse {
                users: ret
                    .into_rows()
                    .into_iter()
                    .map(|x| ListPendingExpertApplicationsRow {
                        user_id: x.user_id,
                        name: x.name.unwrap_or_default(),
                        follower_count: x.follower_count as _,
                        description: x.description,
                        social_media: x.social_media,
                        risk_score: x.risk_score,
                        reputation_score: x.reputation_score,
                        aum: x.aum,
                    })
                    .collect(),
            })
        })
    }
}
