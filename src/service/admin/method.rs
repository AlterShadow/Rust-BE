use crate::method::ensure_user_role;
use eyre::ContextCompat;
use futures::FutureExt;
use gen::database::*;
use gen::model::*;
use lib::database::DbClient;
use lib::handler::{FutureResponse, RequestHandler};
use lib::toolbox::{RequestContext, Toolbox};
use lib::{DEFAULT_LIMIT, DEFAULT_OFFSET};

pub struct MethodAdminListUsers;
impl RequestHandler for MethodAdminListUsers {
    type Request = AdminListUsersRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
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
        }
        .boxed()
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
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let ret = db
                .execute(FunAdminSetUserRoleReq {
                    user_id: req.user_id,
                    role: req.role,
                })
                .await?;

            Ok(AdminSetUserRoleResponse {})
        }
        .boxed()
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
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let ret = db
                .execute(FunAdminSetBlockUserReq {
                    user_id: req.user_id,
                    blocked: req.blocked,
                })
                .await?;

            Ok(AdminSetBlockUserResponse {})
        }
        .boxed()
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
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let ret = db
                .execute(FunAdminApproveUserBecomeExpertReq {
                    user_public_id: req.user_id,
                })
                .await?;

            Ok(AdminApproveUserBecomeExpertResponse {
                success: ret
                    .into_result()
                    .context("failed to approve user become expert")?
                    .success,
            })
        }
        .boxed()
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
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let ret = db
                .execute(FunAdminRejectUserBecomeExpertReq {
                    user_public_id: req.user_id,
                })
                .await?;

            Ok(AdminRejectUserBecomeExpertResponse {
                success: ret
                    .into_result()
                    .context("failed to reject user become expert")?
                    .success,
            })
        }
        .boxed()
    }
}
pub struct MethodAdminListPendingExpertApplications;
impl RequestHandler for MethodAdminListPendingExpertApplications {
    type Request = AdminListPendingExpertApplicationsRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let ret = db
                .execute(FunAdminListPendingUserExpertApplicationsReq {
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                })
                .await?;

            Ok(AdminListPendingExpertApplicationsResponse {
                users: ret
                    .into_rows()
                    .into_iter()
                    .map(|x| ListPendingExpertApplicationsRow {
                        user_id: x.user_public_id,
                        name: x.name,
                        linked_wallet: x.linked_wallet,
                        joined_at: x.joined_at.unwrap_or_default(),
                        requested_at: x.requested_at.unwrap_or_default(),
                        follower_count: x.follower_count as _,
                        description: x.description.unwrap_or_default(),
                        social_media: x.social_media.unwrap_or_default(),
                        risk_score: x.risk_score.unwrap_or_default(),
                        reputation_score: x.reputation_score.unwrap_or_default(),
                        aum: x.aum.unwrap_or_default(),
                    })
                    .collect(),
            })
        }
        .boxed()
    }
}
pub struct MethodAdminGetSystemConfig;
impl RequestHandler for MethodAdminGetSystemConfig {
    type Request = AdminGetSystemConfigRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            if let Some(ret) = db
                .execute(FunAdminGetSystemConfigReq { config_id: 0 })
                .await?
                .into_result()
            {
                Ok(AdminGetSystemConfigResponse {
                    config_placeholder_1: ret.config_placeholder_1.unwrap_or_default(),
                    config_placeholder_2: ret.config_placeholder_2.unwrap_or_default(),
                })
            } else {
                Ok(AdminGetSystemConfigResponse {
                    config_placeholder_1: 0,
                    config_placeholder_2: 0,
                })
            }
        }
        .boxed()
    }
}
pub struct MethodAdminUpdateSystemConfig;
impl RequestHandler for MethodAdminUpdateSystemConfig {
    type Request = AdminUpdateSystemConfigRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let ret = db
                .execute(FunAdminUpdateSystemConfigReq {
                    config_id: 0,
                    config_placeholder_1: req.config_placeholder_1,
                    config_placeholder_2: req.config_placeholder_2,
                })
                .await?;

            Ok(AdminUpdateSystemConfigResponse { success: true })
        }
        .boxed()
    }
}
pub struct MethodAdminListExperts;
impl RequestHandler for MethodAdminListExperts {
    type Request = AdminListExpertsRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let ret = db
                .execute(FunAdminListExpertsReq {
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    expert_id: req.expert_id,
                    user_id: req.user_id,
                    user_public_id: req.user_public_id,
                    username: req.username,
                    family_name: req.family_name,
                    given_name: req.given_name,
                    description: req.description,
                    social_media: req.social_media,
                })
                .await?;

            Ok(AdminListExpertsResponse {
                experts: ret
                    .into_rows()
                    .into_iter()
                    .map(|x| AdminListExpertsRow {
                        expert_id: x.expert_id,
                        user_public_id: x.user_public_id,
                        name: x.name,
                        family_name: x.family_name,
                        given_name: x.given_name,
                        follower_count: x.follower_count.unwrap_or_default(),
                        description: x.description.unwrap_or_default(),
                        social_media: x.social_media.unwrap_or_default(),
                        risk_score: x.risk_score.unwrap_or_default(),
                        reputation_score: x.reputation_score.unwrap_or_default(),
                        aum: x.aum.unwrap_or_default(),
                        joined_at: x.joined_at,
                        requested_at: x.requested_at,
                        approved_at: x.approved_at,
                        pending_expert: x.pending_expert,
                        linked_wallet: x.linked_wallet,
                        approved_expert: x.approved_expert,
                    })
                    .collect(),
            })
        }
        .boxed()
    }
}
pub struct MethodAdminListBackers;
impl RequestHandler for MethodAdminListBackers {
    type Request = AdminListBackersRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let ret = db
                .execute(FunAdminListBackersReq {
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    user_id: req.user_id,
                    user_public_id: req.user_public_id,
                    username: req.username,
                    family_name: req.family_name,
                    given_name: req.given_name,
                })
                .await?;

            Ok(AdminListBackersResponse {
                backers: ret
                    .into_iter()
                    .map(|x| AdminListBackersRow {
                        username: x.username,
                        user_id: x.user_public_id,
                        joined_at: x.joined_at,
                        login_wallet_address: x.login_wallet_address,
                        // TODO: calculate these fees and total backing amount
                        total_platform_fee_paid: 0.0,
                        total_strategy_fee_paid: 0.0,
                        total_backing_amount: 0.0,
                    })
                    .collect(),
            })
        }
        .boxed()
    }
}
pub struct MethodAdminListStrategies;
impl RequestHandler for MethodAdminListStrategies {
    type Request = AdminListStrategiesRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let ret = db
                .execute(FunAdminListStrategiesReq {
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    strategy_id: req.strategy_id,
                    strategy_name: req.strategy_name,
                    expert_public_id: req.expert_public_id,
                    expert_name: req.expert_name,
                    description: req.description,
                })
                .await?;

            Ok(AdminListStrategiesResponse {
                strategies: ret
                    .into_iter()
                    .map(|x| AdminListStrategiesRow {
                        strategy_id: x.strategy_id,
                        strategy_name: x.strategy_name,
                        // expert_id: x.expert_id,
                        expert_public_id: x.expert_public_id,
                        expert_name: x.expert_name,
                        description: x.description,
                        created_at: x.created_at,
                        approved_at: x.approved_at,
                        pending_strategy: x.pending_strategy,
                        approved_strategy: x.approved_strategy,
                    })
                    .collect(),
            })
        }
        .boxed()
    }
}

pub struct MethodAdminAddWalletActivityHistory;
impl RequestHandler for MethodAdminAddWalletActivityHistory {
    type Request = AdminAddWalletActivityHistoryRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let ret = db
                .execute(FunWatcherSaveWalletActivityHistoryReq {
                    address: req.wallet_address,
                    transaction_hash: req.transaction_hash,
                    blockchain: req.blockchain,
                    dex: req.dex,
                    contract_address: req.contract_address,
                    token_in_address: req.token_in_address,
                    token_out_address: req.token_out_address,
                    caller_address: req.caller_address,
                    amount_in: req.amount_in,
                    amount_out: req.amount_out,
                    swap_calls: req.swap_calls,
                    paths: req.paths,
                    dex_versions: req.dex_versions,
                    created_at: req.created_at,
                })
                .await?;

            Ok(AdminAddWalletActivityHistoryResponse {})
        }
        .boxed()
    }
}
