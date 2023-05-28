use eyre::*;
use gen::database::*;
use gen::model::*;
use lib::database::DbClient;
use lib::handler::RequestHandler;
use lib::toolbox::*;
use lib::ws::*;
use std::sync::atomic::Ordering;
use std::sync::Arc;

pub fn ensure_user_role(conn: &Connection, role: EnumRole) -> Result<()> {
    let user_role = conn.role.load(Ordering::Relaxed);

    ensure!(
        user_role >= (role as u32),
        CustomError::new(EnumErrorCode::InvalidRole, ErrorInvalidRole {})
    );
    Ok(())
}
pub struct MethodUserFollowStrategy;

impl RequestHandler for MethodUserFollowStrategy {
    type Request = UserFollowStrategyRequest;
    type Response = UserFollowStrategyResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            ensure_user_role(&conn, EnumRole::User)?;

            let ret = db
                .execute(FunUserFollowStrategyReq {
                    user_id: ctx.user_id,
                    strategy_id: req.strategy_id,
                })
                .await?;

            Ok(UserFollowStrategyResponse {
                success: ret
                    .into_result()
                    .context("failed to follow strategy")?
                    .success,
            })
        })
    }
}
pub struct MethodUserListFollowedStrategies;

impl RequestHandler for MethodUserListFollowedStrategies {
    type Request = UserListFollowedStrategiesRequest;
    type Response = UserListFollowedStrategiesResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        _req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            ensure_user_role(&conn, EnumRole::User)?;

            let ret = db
                .execute(FunUserListFollowedStrategiesReq {
                    user_id: ctx.user_id,
                })
                .await?;
            Ok(UserListFollowedStrategiesResponse {
                strategies: ret
                    .into_rows()
                    .into_iter()
                    .map(|x| ListStrategiesRow {
                        strategy_id: x.strategy_id,
                        strategy_name: x.strategy_name,
                        strategy_description: x.strategy_description,
                        net_value: x.net_value,
                        followers: x.followers,
                        backers: x.backers,
                        risk_score: x.risk_score,
                        aum: x.aum,
                    })
                    .collect(),
            })
        })
    }
}

pub struct MethodUserListStrategies;

impl RequestHandler for MethodUserListStrategies {
    type Request = UserListStrategiesRequest;
    type Response = UserListStrategiesResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        _req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            ensure_user_role(&conn, EnumRole::User)?;

            let ret = db.execute(FunUserListStrategiesReq {}).await?;
            Ok(UserListStrategiesResponse {
                strategies: ret
                    .into_rows()
                    .into_iter()
                    .map(|x| ListStrategiesRow {
                        strategy_id: x.strategy_id,
                        strategy_name: x.strategy_name,
                        strategy_description: x.strategy_description,
                        net_value: x.net_value,
                        followers: x.followers,
                        backers: x.backers,
                        risk_score: x.risk_score,
                        aum: x.aum,
                    })
                    .collect(),
            })
        })
    }
}
pub struct MethodUserGetStrategy;
impl RequestHandler for MethodUserGetStrategy {
    type Request = UserGetStrategyRequest;
    type Response = UserGetStrategyResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            ensure_user_role(&conn, EnumRole::User)?;

            let ret = db
                .execute(FunUserGetStrategyReq {
                    strategy_id: req.strategy_id,
                })
                .await?;
            let ret = ret.into_result().context("failed to get strategy")?;
            let watching_wallets = db
                .execute(FunUserListStrategyWatchWalletsReq {
                    strategy_id: req.strategy_id,
                })
                .await?;
            // TODO: complete missing fields
            Ok(UserGetStrategyResponse {
                strategy_id: ret.strategy_id,
                strategy_name: ret.strategy_name,
                strategy_description: ret.strategy_description,
                creator_user_id: 0,
                social_media: "".to_string(),
                historical_return: 0.0,
                inception_time: 0,
                total_amount: 0.0,
                token_allocation: 0,
                net_value: ret.net_value,
                followers: ret.followers,
                backers: ret.backers,
                watching_wallets: watching_wallets
                    .into_rows()
                    .into_iter()
                    .map(|x| WatchingWalletRow {
                        watching_wallet_id: x.watch_wallet_id,
                        wallet_address: x.wallet_address,
                        blockchain: x.blockchain,
                        dex: "DEX TODO".to_string(),
                        ratio_distribution: x.ratio,
                    })
                    .collect(),
                risk_score: ret.risk_score,
                aum: ret.aum,
                reputation: 0,
                aum_history: vec![],
            })
        })
    }
}
pub struct MethodUserGetStrategyStatistics;
impl RequestHandler for MethodUserGetStrategyStatistics {
    type Request = UserGetStrategyStatisticsRequest;
    type Response = UserGetStrategyStatisticsResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            ensure_user_role(&conn, EnumRole::User)?;
            let net_value = db
                .execute(FunUserGetStrategyStatisticsNetValueReq {
                    strategy_id: req.strategy_id,
                })
                .await?;
            let follow_hist = db
                .execute(FunUserGetStrategyStatisticsFollowHistoryReq {
                    strategy_id: req.strategy_id,
                })
                .await?;
            let back_hist = db
                .execute(FunUserGetStrategyStatisticsBackHistoryReq {
                    strategy_id: req.strategy_id,
                })
                .await?;

            Ok(UserGetStrategyStatisticsResponse {
                strategy_id: req.strategy_id,
                net_value: net_value
                    .into_rows()
                    .into_iter()
                    .map(|x| NetValuePoint {
                        time: x.time,
                        net_value: x.net_value,
                    })
                    .collect(),
                follow_history: follow_hist
                    .into_rows()
                    .into_iter()
                    .map(|x| FollowHistoryPoint {
                        time: x.time,
                        follower_count: x.follower_count,
                    })
                    .collect(),
                back_history: back_hist
                    .into_rows()
                    .into_iter()
                    .map(|x| BackHistoryPoint {
                        time: x.time,
                        backer_count: x.backer_count,
                        backer_quantity_usd: x.backer_quantity_usd,
                    })
                    .collect(),
            })
        })
    }
}

pub struct MethodUserListBackedStrategies;
impl RequestHandler for MethodUserListBackedStrategies {
    type Request = UserListBackedStrategiesRequest;
    type Response = UserListBackedStrategiesResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        _req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            ensure_user_role(&conn, EnumRole::User)?;
            let ret = db
                .execute(FunUserListBackedStrategiesReq {
                    user_id: ctx.user_id,
                })
                .await?;
            Ok(UserListBackedStrategiesResponse {
                strategies: ret
                    .into_rows()
                    .into_iter()
                    .map(|x| ListStrategiesRow {
                        strategy_id: x.strategy_id,
                        strategy_name: x.strategy_name,
                        strategy_description: x.strategy_description,
                        net_value: x.net_value,
                        followers: x.followers,
                        backers: x.backers,
                        risk_score: x.risk_score,
                        aum: x.aum,
                    })
                    .collect(),
            })
        })
    }
}

pub struct MethodUserUnfollowStrategy;
impl RequestHandler for MethodUserUnfollowStrategy {
    type Request = UserUnfollowStrategyRequest;
    type Response = UserUnfollowStrategyResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();

        toolbox.spawn_response(ctx, async move {
            ensure_user_role(&conn, EnumRole::User)?;
            let ret = db
                .execute(FunUserUnfollowStrategyReq {
                    user_id: ctx.user_id,
                    strategy_id: req.strategy_id,
                })
                .await?;
            Ok(UserUnfollowStrategyResponse {
                success: ret
                    .into_result()
                    .context("failed to unfollow strategy")?
                    .success,
            })
        })
    }
}

pub struct MethodUserListExitStrategyHistory;
impl RequestHandler for MethodUserListExitStrategyHistory {
    type Request = UserListExitStrategyHistoryRequest;
    type Response = UserListExitStrategyHistoryResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        _req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            ensure_user_role(&conn, EnumRole::User)?;
            let ret = db
                .execute(FunUserListExitStrategyHistoryReq {
                    user_id: ctx.user_id,
                    strategy_id: None,
                })
                .await?;
            Ok(UserListExitStrategyHistoryResponse {
                exit_history: ret
                    .into_rows()
                    .into_iter()
                    .map(|x| ExitStrategyHistoryRow {
                        exit_history_id: x.exit_history_id,
                        strategy_id: x.strategy_id,
                        exit_quantity: x.exit_quantity,
                        purchase_wallet_address: x.purchase_wallet_address,
                        blockchain: x.blockchain,
                        dex: x.dex,
                        exit_time: x.exit_time,
                        back_time: x.back_time,
                    })
                    .collect(),
            })
        })
    }
}
pub struct MethodUserFollowExpert;
impl RequestHandler for MethodUserFollowExpert {
    type Request = UserFollowExpertRequest;
    type Response = UserFollowExpertResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();

        toolbox.spawn_response(ctx, async move {
            ensure_user_role(&conn, EnumRole::User)?;
            let ret = db
                .execute(FunUserFollowExpertReq {
                    user_id: ctx.user_id,
                    expert_id: req.expert_id,
                })
                .await?;
            Ok(UserFollowExpertResponse {
                success: ret
                    .into_result()
                    .context("failed to follow expert")?
                    .success,
            })
        })
    }
}
pub struct MethodUserListFollowedExperts;
impl RequestHandler for MethodUserListFollowedExperts {
    type Request = UserListFollowedExpertsRequest;
    type Response = UserListFollowedExpertsResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        _req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            ensure_user_role(&conn, EnumRole::User)?;
            let ret = db
                .execute(FunUserListFollowedExpertsReq {
                    user_id: ctx.user_id,
                })
                .await?;
            Ok(UserListFollowedExpertsResponse {
                experts: ret
                    .into_rows()
                    .into_iter()
                    .map(|x| ListExpertsRow {
                        expert_id: x.expert_id,
                        name: x.name,
                        follower_count: x.follower_count,
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
pub struct UserUnfollowExpert;
impl RequestHandler for UserUnfollowExpert {
    type Request = UserUnfollowExpertRequest;
    type Response = UserUnfollowExpertResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();

        toolbox.spawn_response(ctx, async move {
            ensure_user_role(&conn, EnumRole::User)?;
            let ret = db
                .execute(FunUserUnfollowExpertReq {
                    user_id: ctx.user_id,
                    expert_id: req.expert_id,
                })
                .await?;
            Ok(UserUnfollowExpertResponse {
                success: ret
                    .into_result()
                    .context("failed to unfollow expert")?
                    .success,
            })
        })
    }
}
pub struct MethodUserListExperts;
impl RequestHandler for MethodUserListExperts {
    type Request = UserListExpertsRequest;
    type Response = UserListExpertsResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        _req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            ensure_user_role(&conn, EnumRole::User)?;
            let ret = db.execute(FunUserListExpertsReq {}).await?;
            Ok(UserListExpertsResponse {
                experts: ret
                    .into_rows()
                    .into_iter()
                    .map(|x| ListExpertsRow {
                        expert_id: x.expert_id,
                        name: x.name,
                        follower_count: x.follower_count,
                        description: x.description,
                        social_media: x.social_media,
                        risk_score: x.risk_score,
                        aum: x.aum,
                        reputation_score: x.reputation_score,
                    })
                    .collect(),
            })
        })
    }
}
pub struct MethodUserGetExpertProfile;
impl RequestHandler for MethodUserGetExpertProfile {
    type Request = UserGetExpertProfileRequest;
    type Response = UserGetExpertProfileResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            ensure_user_role(&conn, EnumRole::User)?;
            let ret = db
                .execute(FunUserGetExpertProfileReq {
                    expert_id: req.expert_id,
                })
                .await?
                .into_result()
                .context("failed to get expert profile")?;
            Ok(UserGetExpertProfileResponse {
                expert_id: ret.expert_id,
                name: ret.name,
                follower_count: ret.follower_count,
                description: ret.description,
                social_media: ret.social_media,
                risk_score: ret.risk_score,
                aum: ret.aum,
                reputation_score: ret.reputation_score,
                // TODO: get strategies by expert
                strategies: vec![],
            })
        })
    }
}
pub struct MethodUserGetUserProfile;
impl RequestHandler for MethodUserGetUserProfile {
    type Request = UserGetUserProfileRequest;
    type Response = UserGetUserProfileResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        _req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            ensure_user_role(&conn, EnumRole::User)?;
            let ret = db
                .execute(FunUserGetUserProfileReq {
                    user_id: ctx.user_id,
                })
                .await?
                .into_result()
                .context("failed to get user profile")?;
            // TODO: get followed experts, followed strategies, backed strategies
            Ok(UserGetUserProfileResponse {
                user_id: ret.user_id,
                name: ret.name,
                follower_count: ret.follower_count,
                description: ret.description,
                social_media: ret.social_media,
                followed_experts: vec![],
                followed_strategies: vec![],
                backed_strategies: vec![],
            })
        })
    }
}

pub struct MethodUserApplyBecomeExpert;
impl RequestHandler for MethodUserApplyBecomeExpert {
    type Request = UserApplyBecomeExpertRequest;
    type Response = UserApplyBecomeExpertResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        _req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            ensure_user_role(&conn, EnumRole::User)?;

            let ret = db
                .execute(FunUserApplyBecomeExpertReq {
                    user_id: ctx.user_id,
                    // TODO: add fields from request
                    // name: req.name,
                    // description: req.description,
                    // social_media: req.social_media,
                })
                .await?;

            Ok(UserApplyBecomeExpertResponse {
                success: ret
                    .into_result()
                    .context("failed to apply become expert")?
                    .success,
            })
        })
    }
}
pub struct MethodAdminApproveUserBecomeExpert;
impl RequestHandler for MethodAdminApproveUserBecomeExpert {
    type Request = AdminApproveUserBecomeExpertRequest;
    type Response = AdminApproveUserBecomeExpertResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            ensure_user_role(&conn, EnumRole::Admin)?;

            let ret = db
                .execute(FunAdminApproveUserBecomeAdminReq {
                    admin_user_id: ctx.user_id,
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
    type Response = AdminRejectUserBecomeExpertResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            ensure_user_role(&conn, EnumRole::Admin)?;

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
    type Response = AdminListPendingExpertApplicationsResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        _req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            ensure_user_role(&conn, EnumRole::Admin)?;

            let ret = db
                .execute(FunAdminListPendingUserExpertApplicationsReq {})
                .await?;

            Ok(AdminListPendingExpertApplicationsResponse {
                users: ret
                    .into_rows()
                    .into_iter()
                    .map(|x| ListPendingExpertApplicationsRow {
                        user_id: x.user_id,
                        name: x.name,
                        follower_count: x.follower_count,
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
pub struct MethodUserCreateStrategy;

impl RequestHandler for MethodUserCreateStrategy {
    type Request = UserCreateStrategyRequest;
    type Response = UserCreateStrategyResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            // TODO: check if user is expert
            ensure_user_role(&conn, EnumRole::User)?;

            let ret = db
                .execute(FunUserCreateStrategyReq {
                    user_id: ctx.user_id,
                    name: req.name,
                    description: req.description,
                })
                .await?
                .into_result()
                .context("failed to create strategy")?;

            Ok(UserCreateStrategyResponse {
                success: ret.success,
                strategy_id: ret.strategy_id,
            })
        })
    }
}
pub struct MethodUserUpdateStrategy;
impl RequestHandler for MethodUserUpdateStrategy {
    type Request = UserUpdateStrategyRequest;
    type Response = UserUpdateStrategyResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            // TODO: check if user is expert

            ensure_user_role(&conn, EnumRole::User)?;

            let ret = db
                .execute(FunUserUpdateStrategyReq {
                    user_id: ctx.user_id,
                    strategy_id: req.strategy_id,
                    name: req.name,
                    description: req.description,
                })
                .await?
                .into_result()
                .context("failed to update strategy")?;

            Ok(UserUpdateStrategyResponse {
                success: ret.success,
            })
        })
    }
}
// pub struct MethodUserDeleteStrategy;
pub struct MethodUserAddStrategyWatchingWallet;
impl RequestHandler for MethodUserAddStrategyWatchingWallet {
    type Request = UserAddStrategyWatchingWalletRequest;
    type Response = UserAddStrategyWatchingWalletResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            // TODO: check if user is expert

            ensure_user_role(&conn, EnumRole::User)?;

            let ret = db
                .execute(FunUserAddStrategyWatchWalletReq {
                    user_id: ctx.user_id,
                    strategy_id: req.strategy_id,
                    wallet_address: req.wallet_address,
                    blockchain: req.blockchain,
                    ratio: req.ratio,
                    // TODO: maybe remove dex?
                    dex: "ALL".to_string(),
                })
                .await?
                .into_result()
                .context("failed to add strategy watching wallet")?;

            Ok(UserAddStrategyWatchingWalletResponse {
                success: ret.success,
                wallet_id: ret.watch_wallet_id,
            })
        })
    }
}
pub struct MethodUserRemoveStrategyWatchingWallet;
impl RequestHandler for MethodUserRemoveStrategyWatchingWallet {
    type Request = UserRemoveStrategyWatchingWalletRequest;
    type Response = UserRemoveStrategyWatchingWalletResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();

        toolbox.spawn_response(ctx, async move {
            // TODO: check if user is expert

            ensure_user_role(&conn, EnumRole::User)?;

            let ret = db
                .execute(FunUserRemoveStrategyWatchWalletReq {
                    user_id: ctx.user_id,
                    watch_wallet_id: req.wallet_id,
                })
                .await?
                .into_result()
                .context("failed to remove strategy watching wallet")?;

            Ok(UserRemoveStrategyWatchingWalletResponse {
                success: ret.success,
            })
        })
    }
}

pub struct EndpointUserListWalletActivityHistory;

impl RequestHandler for EndpointUserListWalletActivityHistory {
    type Request = UserListWalletActivityHistoryRequest;
    type Response = UserListWalletActivityHistoryResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        _conn: Arc<Connection>,
        req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();

        toolbox.spawn_response(ctx, async move {
            let ret = db
                .execute(FunWatcherListWalletActivityHistoryReq {
                    address: req.wallet_address,
                    blockchain: req.blockchain,
                })
                .await?;

            Ok(UserListWalletActivityHistoryResponse {
                wallet_activities: ret
                    .into_rows()
                    .into_iter()
                    .map(|x| ListWalletActivityHistoryRow {
                        record_id: x.wallet_activity_history_id,
                        wallet_address: x.address,
                        blockchain: x.blockchain,
                        contract_address: x.contract_address,
                        token_in_address: x.token_in_address,
                        token_out_address: x.token_out_address,
                        caller_address: x.caller_address,
                        amount_in: x.amount_in,
                        amount_out: x.amount_out,
                        swap_calls: x.swap_calls,
                        paths: x.paths,
                        dex_versions: x.dex_versions,
                        dex: x.dex,
                        transaction_hash: x.transaction_hash,
                        created_at: x.created_at,
                    })
                    .collect(),
            })
        })
    }
}
