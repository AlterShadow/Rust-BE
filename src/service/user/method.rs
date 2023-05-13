use ::tap::*;
use eyre::*;
use gen::database::*;
use gen::model::*;
use std::sync::Arc;

use lib::toolbox::*;

use lib::handler::RequestHandler;
use lib::ws::*;
use mc2_fi::method::LoginHandler;
use serde_json::Value;
use std::sync::atomic::Ordering;

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
            ensure!(ret.rows.len() == 1, "failed to follow strategy");
            Ok(UserFollowStrategyResponse {
                success: ret.rows[0].success,
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
                    .rows
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
                    .rows
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
            ensure!(ret.rows.len() == 1, "failed to get strategy");
            let ret = ret.rows.into_iter().next().unwrap();
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
                    .rows
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
                    .rows
                    .into_iter()
                    .map(|x| NetValuePoint {
                        time: x.time,
                        net_value: x.net_value,
                    })
                    .collect(),
                follow_history: follow_hist
                    .rows
                    .into_iter()
                    .map(|x| FollowHistoryPoint {
                        time: x.time,
                        follower_count: x.follower_count,
                    })
                    .collect(),
                back_history: back_hist
                    .rows
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
