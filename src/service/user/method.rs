use eyre::*;
use gen::database::*;
use gen::model::*;
use itertools::Itertools;
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
                .fun_user_follow_strategy(FunUserFollowStrategyReq {
                    user_id: conn.get_user_id(),
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
        req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            ensure_user_role(&conn, EnumRole::User)?;

            let ret = db
                .fun_user_list_followed_strategies(FunUserListFollowedStrategiesReq {
                    user_id: conn.get_user_id(),
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
        req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            ensure_user_role(&conn, EnumRole::User)?;

            let ret = db
                .fun_user_list_strategies(FunUserListStrategiesReq {})
                .await?;
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
                .fun_user_get_strategy(FunUserGetStrategyReq {
                    strategy_id: req.strategy_id,
                })
                .await?;
            let ret = ret.into_result().context("failed to get strategy")?;
            let watching_wallets = db
                .fun_user_list_strategy_watch_wallets(FunUserListStrategyWatchWalletsReq {
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
                .fun_user_get_strategy_statistics_net_value(
                    FunUserGetStrategyStatisticsNetValueReq {
                        strategy_id: req.strategy_id,
                    },
                )
                .await?;
            let follow_hist = db
                .fun_user_get_strategy_statistics_follow_history(
                    FunUserGetStrategyStatisticsFollowHistoryReq {
                        strategy_id: req.strategy_id,
                    },
                )
                .await?;
            let back_hist = db
                .fun_user_get_strategy_statistics_back_history(
                    FunUserGetStrategyStatisticsBackHistoryReq {
                        strategy_id: req.strategy_id,
                    },
                )
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

pub struct MethodUserBackStrategy;
impl RequestHandler for MethodUserBackStrategy {
    type Request = UserBackStrategyRequest;
    type Response = UserBackStrategyResponse;

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
            let user = db
                .fun_user_list_wallets(FunUserListWalletsReq {
                    user_id: conn.get_user_id(),
                })
                .await?
                .into_rows()
                .into_iter()
                .filter(|x| x.is_default)
                .next()
                .context("no default wallet")?;
            // TODO: get transaction here
            let transaction_hash = "0x000000000000";
            let ret = db
                .fun_user_back_strategy(FunUserBackStrategyReq {
                    user_id: conn.get_user_id(),
                    strategy_id: req.strategy_id,
                    quantity: req.quantity,
                    purchase_wallet: user.wallet_address,
                    blockchain: user.blockchain,
                    dex: "".to_string(),
                    transaction_hash: transaction_hash.to_string(),
                })
                .await?;
            Ok(UserBackStrategyResponse {
                success: ret
                    .into_result()
                    .context("failed to back strategy")?
                    .success,
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
                .fun_user_list_backed_strategies(FunUserListBackedStrategiesReq {
                    user_id: conn.get_user_id(),
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
                .fun_user_unfollow_strategy(FunUserUnfollowStrategyReq {
                    user_id: conn.get_user_id(),
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
pub struct MethodUserExitStrategy;
impl RequestHandler for MethodUserExitStrategy {
    type Request = UserExitStrategyRequest;
    type Response = UserExitStrategyResponse;

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
            // TODO: actually exit tokens to the user
            let strategy = db
                .fun_user_get_strategy(FunUserGetStrategyReq {
                    strategy_id: req.strategy_id,
                })
                .await?
                .into_result()
                .context("no strategy")?;
            let back_hist = db
                .fun_user_list_back_strategy_history(FunUserListBackStrategyHistoryReq {
                    user_id: conn.get_user_id(),
                    strategy_id: Some(req.strategy_id),
                })
                .await?
                .into_rows()
                .into_iter()
                .sorted_by_key(|x| -(x.time as i64))
                .next()
                .context("no back history")?;
            // TODO: sort this out
            let ret = db
                .fun_user_exit_strategy(FunUserExitStrategyReq {
                    user_id: conn.get_user_id(),
                    strategy_id: req.strategy_id,
                    quantity: req.quantity,
                    blockchain: back_hist.blockchain,
                    dex: back_hist.dex,
                    back_time: back_hist.time,
                    transaction_hash: "0x000000000000".to_string(),
                    purchase_wallet: back_hist.wallet_address,
                })
                .await?;
            Ok(UserExitStrategyResponse {
                success: ret
                    .into_result()
                    .context("failed to exit strategy")?
                    .success,
                transaction_hash: "0x000000000000".to_string(),
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
                .fun_user_list_exit_strategy_history(FunUserListExitStrategyHistoryReq {
                    user_id: conn.get_user_id(),
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
                .fun_user_follow_expert(FunUserFollowExpertReq {
                    user_id: conn.get_user_id(),
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
                .fun_user_list_followed_experts(FunUserListFollowedExpertsReq {
                    user_id: conn.get_user_id(),
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
                .fun_user_unfollow_expert(FunUserUnfollowExpertReq {
                    user_id: conn.get_user_id(),
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
            let ret = db.fun_user_list_experts(FunUserListExpertsReq {}).await?;
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
                .fun_user_get_expert_profile(FunUserGetExpertProfileReq {
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
                .fun_user_get_user_profile(FunUserGetUserProfileReq {
                    user_id: conn.get_user_id(),
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
pub struct MethodUserRegisterWallet;
impl RequestHandler for MethodUserRegisterWallet {
    type Request = UserRegisterWalletRequest;
    type Response = UserRegisterWalletResponse;

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
                .fun_user_register_wallet(FunUserRegisterWalletReq {
                    user_id: conn.get_user_id(),
                    blockchain: req.blockchain,
                    wallet_address: req.wallet_address,
                })
                .await?;

            Ok(UserRegisterWalletResponse {
                success: ret
                    .into_result()
                    .context("failed to register wallet")?
                    .success,
            })
        })
    }
}

pub struct MethodUserListWallets;
impl RequestHandler for MethodUserListWallets {
    type Request = UserListWalletsRequest;
    type Response = UserListWalletsResponse;

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
                .fun_user_list_wallets(FunUserListWalletsReq {
                    user_id: conn.get_user_id(),
                })
                .await?;

            Ok(UserListWalletsResponse {
                wallets: ret
                    .into_rows()
                    .into_iter()
                    .map(|x| ListWalletsRow {
                        wallet_id: x.wallet_id,
                        blockchain: x.blockchain,
                        wallet_address: x.wallet_address,
                        is_default: x.is_default,
                    })
                    .collect(),
            })
        })
    }
}
pub struct MethodUserDeregisterWallet;
impl RequestHandler for MethodUserDeregisterWallet {
    type Request = UserDeregisterWalletRequest;
    type Response = UserDeregisterWalletResponse;

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
                .fun_user_deregister_wallet(FunUserDeregisterWalletReq {
                    user_id: conn.get_user_id(),
                    wallet_id: req.wallet_id,
                })
                .await?;

            Ok(UserDeregisterWalletResponse {
                success: ret
                    .into_result()
                    .context("failed to deregister wallet")?
                    .success,
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
        req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            ensure_user_role(&conn, EnumRole::User)?;

            let ret = db
                .fun_user_apply_become_expert(FunUserApplyBecomeExpertReq {
                    user_id: conn.get_user_id(),
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
                .fun_admin_approve_user_become_admin(FunAdminApproveUserBecomeAdminReq {
                    admin_user_id: conn.get_user_id(),
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
                .fun_admin_reject_user_become_admin(FunAdminRejectUserBecomeAdminReq {
                    admin_user_id: conn.get_user_id(),
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
                .fun_admin_list_pending_user_expert_applications(
                    FunAdminListPendingUserExpertApplicationsReq {},
                )
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
pub struct MethodUserUpdateStrategy;
// pub struct MethodUserDeleteStrategy;
pub struct MethodUserAddStrategyWatchingWallet;
pub struct MethodUserRemoveStrategyWatchingWallet;
