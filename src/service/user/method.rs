use crate::admin_method::AdminSubscribeTopic;
use crate::audit::{
    get_audit_rules, validate_audit_rule_immutable_tokens, AuditLogger, AUDIT_TOP25_TOKENS,
};
use crate::back_strategy;
use crate::back_strategy::calculate_sp_tokens_to_mint_easy_approach;
use api::cmc::CoinMarketCap;
use chrono::Utc;
use eth_sdk::escrow::transfer_token_to_and_ensure_success;
use eth_sdk::escrow::{AbstractEscrowContract, EscrowContract};
use eth_sdk::signer::Secp256k1SecretKey;
use eth_sdk::strategy_pool::StrategyPoolContract;
use eth_sdk::strategy_wallet::{
    full_redeem_from_strategy_and_ensure_success, redeem_from_strategy_and_ensure_success,
    StrategyWalletContract,
};
use eth_sdk::utils::verify_message_address;
use eth_sdk::*;
use eyre::*;
use futures::FutureExt;
use gen::database::*;
use gen::model::*;
use itertools::Itertools;
use lib::database::DbClient;
use lib::handler::{FutureResponse, RequestHandler};
use lib::log::DynLogger;
use lib::toolbox::*;
use lib::utils::hex_decode;
use lib::ws::SubscribeManager;
use lib::{DEFAULT_LIMIT, DEFAULT_OFFSET};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info};
use web3::signing::Key;
use web3::types::{Address, H256, U256};
include!("../shared/method.rs");

pub struct MethodUserFollowStrategy;

impl RequestHandler for MethodUserFollowStrategy {
    type Request = UserFollowStrategyRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;

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
        }
        .boxed()
    }
}
pub struct MethodUserListFollowedStrategies;

impl RequestHandler for MethodUserListFollowedStrategies {
    type Request = UserListFollowedStrategiesRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;

            let ret = db
                .execute(FunUserListFollowedStrategiesReq {
                    user_id: ctx.user_id,
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                })
                .await?;
            Ok(UserListFollowedStrategiesResponse {
                strategies_total: ret.first(|x| x.total).unwrap_or_default(),
                strategies: ret.map(convert_strategy_db_to_api),
            })
        }
        .boxed()
    }
}

pub struct MethodUserListStrategies;

impl RequestHandler for MethodUserListStrategies {
    type Request = UserListStrategiesRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;

            let ret = db
                .execute(FunUserListStrategiesReq {
                    user_id: ctx.user_id,
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    strategy_id: req.strategy_id,
                    strategy_name: req.strategy_name,
                    expert_public_id: req.expert_public_id,
                    expert_name: req.expert_name,
                    description: req.description,
                    blockchain: req.blockchain,
                    wallet_address: req.wallet_address.map(|x| x.into()),
                })
                .await?;

            Ok(UserListStrategiesResponse {
                strategies_total: ret.first(|x| x.total).unwrap_or_default(),
                strategies: ret.map(convert_strategy_db_to_api),
            })
        }
        .boxed()
    }
}

pub struct MethodUserListTopPerformingStrategies;

impl RequestHandler for MethodUserListTopPerformingStrategies {
    type Request = UserListTopPerformingStrategiesRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            // TODO: use FunUserListTopPerformingStrategiesReq

            let ret = db
                .execute(FunUserListStrategiesReq {
                    user_id: ctx.user_id,
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    strategy_id: None,
                    strategy_name: None,
                    expert_public_id: None,
                    expert_name: None,
                    description: None,
                    blockchain: None,
                    wallet_address: None,
                })
                .await?;
            Ok(UserListTopPerformingStrategiesResponse {
                strategies_total: ret.first(|x| x.total).unwrap_or_default(),
                strategies: ret.map(convert_strategy_db_to_api),
            })
        }
        .boxed()
    }
}
pub struct MethodUserListStrategyFollowers;
impl RequestHandler for MethodUserListStrategyFollowers {
    type Request = UserListStrategyFollowersRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;

            let ret = db
                .execute(FunUserListStrategyFollowersReq {
                    strategy_id: req.strategy_id,
                })
                .await?;
            Ok(UserListStrategyFollowersResponse {
                followers_total: ret.first(|x| x.total).unwrap_or_default(),
                followers: ret
                    .into_iter()
                    .map(|x| ListStrategyFollowersRow {
                        user_id: x.user_public_id,
                        name: x.username,
                        linked_wallet: x.wallet_address.into(),
                        followed_date: x.followed_at,
                    })
                    .collect(),
            })
        }
        .boxed()
    }
}
pub struct MethodUserListStrategyBackers;
impl RequestHandler for MethodUserListStrategyBackers {
    type Request = UserListStrategyBackersRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;

            let ret = db
                .execute(FunUserListStrategyBackersReq {
                    strategy_id: req.strategy_id,
                })
                .await?;
            Ok(UserListStrategyBackersResponse {
                backers_total: ret.first(|x| x.total).unwrap_or_default(),
                backers: ret
                    .into_iter()
                    .map(|x| ListStrategyBackersRow {
                        user_id: x.user_public_id,
                        name: x.username,
                        linked_wallet: x.wallet_address.into(),
                        backed_date: x.backed_at,
                    })
                    .collect(),
            })
        }
        .boxed()
    }
}
pub struct MethodUserGetStrategy;
impl RequestHandler for MethodUserGetStrategy {
    type Request = UserGetStrategyRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let ret = db
                .execute(FunUserListStrategiesReq {
                    user_id: ctx.user_id,
                    limit: 1,
                    offset: 0,
                    strategy_id: Some(req.strategy_id),
                    strategy_name: None,
                    expert_public_id: None,
                    expert_name: None,
                    description: None,
                    blockchain: None,
                    wallet_address: None,
                })
                .await?
                .into_result()
                .context("failed to get strategy")?;

            Ok(UserGetStrategyResponse {
                strategy: convert_strategy_db_to_api(ret),
                watching_wallets: db
                    .execute(FunUserListStrategyWatchWalletsReq {
                        strategy_id: req.strategy_id,
                    })
                    .await?
                    .map(|x| WatchingWalletRow {
                        watching_wallet_id: x.watch_wallet_id,
                        wallet_address: x.wallet_address.into(),
                        blockchain: x.blockchain,
                        ratio_distribution: x.ratio,
                    }),
                aum_ledger: vec![],
                audit_rules: db
                    .execute(FunUserListStrategyAuditRulesReq {
                        strategy_id: req.strategy_id,
                        audit_rule_id: None,
                    })
                    .await?
                    .into_iter()
                    .map(|x| {
                        let rule = get_audit_rules()
                            .iter()
                            .find(|y| x.rule_id == y.id)
                            .context("Could not find rule")?;
                        Ok::<_, Error>(UserListStrategyAuditRulesRow {
                            rule_id: x.rule_id,
                            rule_name: rule.name.to_string(),
                            rule_description: rule.description.to_string(),
                            created_at: x.created_at,
                            enabled: true,
                        })
                    })
                    .try_collect()?,
                whitelisted_tokens: db
                    .execute(FunUserListStrategyWhitelistedTokensReq {
                        strategy_id: req.strategy_id,
                    })
                    .await?
                    .map(|x| x.token_name),
            })
        }
        .boxed()
    }
}
pub struct MethodUserGetStrategyStatistics;
impl RequestHandler for MethodUserGetStrategyStatistics {
    type Request = UserGetStrategyStatisticsRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let net_value = db
                .execute(FunUserGetStrategyStatisticsNetValueReq {
                    strategy_id: req.strategy_id,
                })
                .await?;
            let follow_hist = db
                .execute(FunUserGetStrategyStatisticsFollowLedgerReq {
                    strategy_id: req.strategy_id,
                })
                .await?;
            let back_hist = db
                .execute(FunUserGetStrategyStatisticsBackLedgerReq {
                    strategy_id: req.strategy_id,
                })
                .await?;

            Ok(UserGetStrategyStatisticsResponse {
                strategy_id: req.strategy_id,
                net_value: net_value
                    .into_iter()
                    .map(|x| NetValuePoint {
                        time: x.time,
                        net_value: 0.0,
                    })
                    .collect(),
                follow_ledger: follow_hist
                    .into_iter()
                    .map(|x| FollowLedgerPoint {
                        time: x.time,
                        follower_count: x.follower_count,
                    })
                    .collect(),
                back_ledger: back_hist
                    .into_iter()
                    .map(|x| BackLedgerPoint {
                        time: x.time,
                        backer_count: x.backer_count,
                        backer_quantity_usd: x.backer_quantity_usd,
                    })
                    .collect(),
            })
        }
        .boxed()
    }
}
pub struct MethodUserGetStrategiesStatistics;
impl RequestHandler for MethodUserGetStrategiesStatistics {
    type Request = UserGetStrategiesStatisticsRequest;

    fn handle(
        &self,
        _toolbox: &Toolbox,
        ctx: RequestContext,
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            // TODO: query from database
            Ok(UserGetStrategiesStatisticsResponse {
                tracking_amount_usd: 0.0,
                backing_amount_usd: 0.0,
                difference_amount_usd: 0.0,
                aum_value_usd: 0.0,
                current_value_usd: 0.0,
                withdrawable_value_usd: 0.0,
            })
        }
        .boxed()
    }
}

pub struct MethodUserListBackedStrategies;
impl RequestHandler for MethodUserListBackedStrategies {
    type Request = UserListBackedStrategiesRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let ret = db
                .execute(FunUserListBackedStrategiesReq {
                    user_id: ctx.user_id,
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                })
                .await?;
            Ok(UserListBackedStrategiesResponse {
                strategies_total: ret.first(|x| x.total).unwrap_or_default(),
                strategies: ret.map(convert_strategy_db_to_api),
            })
        }
        .boxed()
    }
}
pub struct MethodUserListDepositWithdrawBalances;
impl RequestHandler for MethodUserListDepositWithdrawBalances {
    type Request = UserListDepositWithdrawBalancesRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            let balances = db
                .execute(FunUserListUserDepositWithdrawBalanceReq {
                    limit: 10000,
                    offset: 0,
                    user_id: ctx.user_id,
                    blockchain: None,
                    token_address: None,
                    token_id: None,
                    escrow_contract_address: None,
                })
                .await?;
            Ok(UserListDepositWithdrawBalancesResponse {
                balances: balances.map(|x| UserListDepositWithdrawBalance {
                    blockchain: x.blockchain,
                    token_id: x.token_id,
                    token_symbol: x.token_symbol,
                    token_name: x.token_name,
                    balance: x.balance.into(),
                }),
            })
        }
        .boxed()
    }
}

pub struct MethodUserGetDepositWithdrawBalance;
impl RequestHandler for MethodUserGetDepositWithdrawBalance {
    type Request = UserGetDepositWithdrawBalanceRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            let balance = db
                .execute(FunUserListUserDepositWithdrawBalanceReq {
                    limit: 1000,
                    offset: 0,
                    user_id: ctx.user_id,
                    blockchain: None,
                    token_address: None,
                    token_id: Some(req.token_id),
                    escrow_contract_address: None,
                })
                .await?
                .into_result()
                .map(|x| x.balance)
                .unwrap_or_default();
            Ok(UserGetDepositWithdrawBalanceResponse {
                balance: balance.into(),
            })
        }
        .boxed()
    }
}

pub struct MethodUserBackStrategy {
    pub pool: EthereumRpcConnectionPool,
    pub escrow_contract: Arc<AbstractEscrowContract>,
    pub master_key: Secp256k1SecretKey,
    pub dex_addresses: Arc<DexAddresses>,
    pub subscribe_manager: Arc<SubscribeManager<AdminSubscribeTopic>>,
}
impl RequestHandler for MethodUserBackStrategy {
    type Request = UserBackStrategyRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        let toolbox = toolbox.clone();
        let pool = self.pool.clone();
        let dex_addresses = self.dex_addresses.clone();
        let escrow_contract = self.escrow_contract.clone();
        let master_key = self.master_key.clone();
        let subscribe_manager = self.subscribe_manager.clone();
        async move {
            let token = db
                .execute(FunUserListEscrowTokenContractAddressReq {
                    limit: 1,
                    offset: 0,
                    blockchain: None,
                    token_id: Some(req.token_id),
                    address: None,
                    symbol: None,
                    is_stablecoin: None,
                })
                .await?
                .into_result()
                .with_context(|| CustomError::new(EnumErrorCode::NotFound, "Token not found"))?;
            let escrow_contract = escrow_contract.get(&pool, token.blockchain).await?;
            let eth_conn = pool.get(token.blockchain).await?;
            ensure_user_role(ctx, EnumRole::User)?;
            subscribe_manager.subscribe(AdminSubscribeTopic::UserBackProgress, ctx);
            let seq = ctx.seq;
            let report_progress = move |end: bool, msg: &str, hash: H256| {
                subscribe_manager.publish_with_filter(
                    &toolbox,
                    AdminSubscribeTopic::UserBackProgress,
                    &UserBackStrategyStreamResponse {
                        end,
                        msg: msg.to_string(),
                        hash: hash.into(),
                    },
                    |ctx| ctx.seq == seq,
                )
            };
            let logger = DynLogger::new(Arc::new(move |msg| {
                report_progress(false, msg, H256::zero());
            }));
            tokio::spawn(async move {
                if let Err(err) = back_strategy::user_back_strategy(
                    &eth_conn,
                    &ctx,
                    &db,
                    token.blockchain,
                    ctx.user_id,
                    req.quantity.into(),
                    req.strategy_id,
                    token.token_id,
                    token.address.into(),
                    escrow_contract,
                    &dex_addresses,
                    master_key,
                    logger.clone(),
                )
                .await
                {
                    error!("user back strategy error: {:?}", err);
                    logger.log(format!("user back strategy error {}", err));
                }
            });
            Ok(UserBackStrategyResponse {})
        }
        .boxed()
    }
}

pub async fn user_exit_strategy(
    conn: &EthereumRpcConnection,
    ctx: &RequestContext,
    db: &DbClient,
    blockchain: EnumBlockChain,
    strategy_id: i64,
    maybe_strategy_tokens_to_redeem: Option<U256>,
    master_key: impl Key + Clone,
) -> Result<H256> {
    use eth_sdk::strategy_pool::parse_strategy_pool_withdraw_event;
    /* instantiate strategy wallet */
    let strategy_wallet_contract = db
        .execute(FunUserListStrategyWalletsReq {
            user_id: ctx.user_id,
            blockchain: Some(blockchain),
        })
        .await?
        .into_result()
        .context("user has no strategy wallet on this chain")?;
    let strategy_wallet_contract =
        StrategyWalletContract::new(conn.clone(), strategy_wallet_contract.address.into())?;

    /* if master key eoa is not admin, we can't redeem */
    if strategy_wallet_contract.admin().await? != master_key.address() {
        bail!("strategy wallet has another or no admin");
    }

    let strategy_pool_contract_row = db
        .execute(FunWatcherListStrategyPoolContractReq {
            limit: 1,
            offset: 0,
            strategy_id: Some(strategy_id),
            blockchain: Some(blockchain),
            address: None,
        })
        .await?
        .into_result()
        .context("strategy pool is not registered in the database")?;

    /* instantiate strategy pool contract wrapper */
    let strategy_pool_contract =
        StrategyPoolContract::new(conn.clone(), strategy_pool_contract_row.address.into())?;

    /* check if strategy pool is trading */
    if strategy_pool_contract.is_paused().await? {
        bail!("strategy is currently trading, redeem is not possible");
    }

    /* redeem */
    let tx_hash = match maybe_strategy_tokens_to_redeem {
        Some(strategy_tokens_to_redeem) => {
            /* check share balance first */
            // TODO: check balance from the database when we have a balance table or a pg func from ledger
            if strategy_pool_contract
                .balance_of(strategy_wallet_contract.address())
                .await?
                < strategy_tokens_to_redeem
            {
                bail!("not enough strategy tokens");
            }
            /* if strategy is currently trading, redeem is not possible */
            redeem_from_strategy_and_ensure_success(
                strategy_wallet_contract.clone(),
                &conn,
                CONFIRMATIONS,
                MAX_RETRIES,
                POLL_INTERVAL,
                master_key.clone(),
                strategy_pool_contract.address(),
                strategy_tokens_to_redeem,
            )
            .await
            .context("redeem is not possible currently")?
        }
        None => {
            /* check share balance first */
            // TODO: check balance from the database when we have a balance table or a pg func from ledger
            let strategy_token_balance = strategy_pool_contract
                .balance_of(strategy_wallet_contract.address())
                .await?;
            if strategy_token_balance == U256::zero() {
                bail!("no strategy tokens to redeem");
            }

            /* if strategy is currently trading, redeem is not possible */
            full_redeem_from_strategy_and_ensure_success(
                strategy_wallet_contract.clone(),
                &conn,
                12,
                10,
                Duration::from_secs(10),
                master_key.clone(),
                strategy_pool_contract.address(),
            )
            .await
            .context("redeem is not possible currently")?
        }
    };

    let redeem_info = parse_strategy_pool_withdraw_event(
        strategy_pool_contract.address(),
        conn.eth()
            .transaction_receipt(tx_hash)
            .await?
            .context("redeem transaction receipt not found even though it has confirmations")?,
    )?;

    /* update exit strategy ledger */
    db.execute(FunUserExitStrategyReq {
        user_id: ctx.user_id,
        strategy_id,
        // TODO: calculate value of sp tokens exit in usdc
        quantity: U256::zero().into(),
        blockchain,
        transaction_hash: tx_hash.into(),
        redeem_sp_tokens: redeem_info.strategy_tokens.into(),
    })
    .await?;

    /* update user strategy token balance */
    let user_strategy_balance: U256 = db
        .execute(FunWatcherListUserStrategyBalanceReq {
            limit: 1,
            offset: 0,
            strategy_id: Some(strategy_id),
            user_id: Some(ctx.user_id),
            blockchain: Some(blockchain),
        })
        .await?
        .first(|x| x.balance)
        .context("could not get user strategy token balance from database on exit strategy")?
        .into();
    db.execute(FunWatcherUpsertUserStrategyBalanceReq {
        user_id: ctx.user_id,
        strategy_id,
        blockchain,
        old_balance: user_strategy_balance.into(),
        new_balance: (user_strategy_balance.try_checked_sub(redeem_info.strategy_tokens)?).into(),
    })
    .await?;

    /* update strategy pool contract balance table */
    for idx in 0..redeem_info.strategy_pool_assets.len() {
        let redeemed_asset = redeem_info.strategy_pool_assets[idx];
        let redeemed_amount = redeem_info.strategy_pool_asset_amounts[idx];

        let old_asset_balance_row = db
            .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
                strategy_pool_contract_id: strategy_pool_contract_row.pkey_id,
                blockchain: Some(blockchain),
                token_address: Some(redeemed_asset.into()),
            })
            .await?
            .into_result()
            .context("strategy pool balance of redeemed asset not found")?;

        db.execute(FunWatcherUpsertStrategyPoolContractAssetBalanceReq {
            strategy_pool_contract_id: strategy_pool_contract_row.pkey_id,
            token_address: redeemed_asset.into(),
            blockchain: blockchain,
            new_balance: old_asset_balance_row
                .balance
                .try_checked_sub(redeemed_amount)
                .context("redeemed amount is greater than known balance of strategy pool asset")?
                .into(),
        })
        .await?;
    }

    Ok(tx_hash)
}

pub struct MethodUserExitStrategy {
    pub pool: EthereumRpcConnectionPool,
    pub master_key: Secp256k1SecretKey,
}

impl RequestHandler for MethodUserExitStrategy {
    type Request = UserExitStrategyRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        let pool = self.pool.clone();
        let master_key = self.master_key.clone();
        async move {
            let eth_conn = pool.get(req.blockchain).await?;
            // TODO: decide if we should ensure user role
            ensure_user_role(ctx, EnumRole::User)?;
            let tx_hash = user_exit_strategy(
                &eth_conn,
                &ctx,
                &db,
                req.blockchain,
                req.strategy_id,
                req.quantity.map(|x| x.into()),
                master_key,
            )
            .await?;
            Ok(UserExitStrategyResponse {
                success: true,
                transaction_hash: tx_hash.into(),
            })
        }
        .boxed()
    }
}

pub struct MethodUserRequestRefund {
    pub pool: EthereumRpcConnectionPool,
    pub stablecoin_addresses: Arc<BlockchainCoinAddresses>,
    pub escrow_contract: Arc<AbstractEscrowContract>,
    pub master_key: Secp256k1SecretKey,
}

impl RequestHandler for MethodUserRequestRefund {
    type Request = UserRequestRefundRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        let pool = self.pool.clone();
        let stablecoin_addresses = self.stablecoin_addresses.clone();
        let escrow_contract = self.escrow_contract.clone();
        let master_key = self.master_key.clone();
        async move {
            let escrow_contract = escrow_contract.get(&pool, req.blockchain).await?;
            let eth_conn = pool.get(req.blockchain).await?;

            ensure_user_role(ctx, EnumRole::User)?;

            on_user_request_refund(
                &eth_conn,
                &ctx,
                &db,
                req.blockchain,
                &stablecoin_addresses,
                escrow_contract,
                req.quantity.into(),
                req.wallet_address.into(),
                master_key,
                EnumBlockchainCoin::USDC,
                DynLogger::empty(),
            )
            .await?;
            Ok(UserRequestRefundResponse { success: true })
        }
        .boxed()
    }
}
pub struct MethodUserUnfollowStrategy;
impl RequestHandler for MethodUserUnfollowStrategy {
    type Request = UserUnfollowStrategyRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();

        async move {
            ensure_user_role(ctx, EnumRole::User)?;
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
        }
        .boxed()
    }
}
pub struct MethodUserListExitStrategyLedger;
impl RequestHandler for MethodUserListExitStrategyLedger {
    type Request = UserListExitStrategyLedgerRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let ret = db
                .execute(FunUserListExitStrategyLedgerReq {
                    user_id: ctx.user_id,
                    strategy_id: None,
                })
                .await?;
            Ok(UserListExitStrategyLedgerResponse {
                exit_ledger_total: ret.first(|x| x.total).unwrap_or_default(),
                exit_ledger: ret
                    .into_iter()
                    .map(|x| ExitStrategyLedgerRow {
                        exit_ledger_id: x.exit_ledger_id,
                        strategy_id: x.strategy_id,
                        exit_quantity: x.exit_quantity.into(),
                        blockchain: x.blockchain,
                        exit_time: x.exit_time,
                    })
                    .collect(),
            })
        }
        .boxed()
    }
}
pub struct MethodUserFollowExpert;
impl RequestHandler for MethodUserFollowExpert {
    type Request = UserFollowExpertRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();

        async move {
            ensure_user_role(ctx, EnumRole::User)?;
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
        }
        .boxed()
    }
}

pub struct MethodUserListFollowedExperts;
impl RequestHandler for MethodUserListFollowedExperts {
    type Request = UserListFollowedExpertsRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let ret = db
                .execute(FunUserListFollowedExpertsReq {
                    user_id: ctx.user_id,
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                })
                .await?;
            Ok(UserListFollowedExpertsResponse {
                experts_total: ret.first(|x| x.total).unwrap_or_default(),
                experts: ret.into_iter().map(convert_expert_db_to_api).collect(),
            })
        }
        .boxed()
    }
}
pub struct MethodUserUnfollowExpert;
impl RequestHandler for MethodUserUnfollowExpert {
    type Request = UserUnfollowExpertRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();

        async move {
            ensure_user_role(ctx, EnumRole::User)?;
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
        }
        .boxed()
    }
}
pub struct MethodUserListExperts;
impl RequestHandler for MethodUserListExperts {
    type Request = UserListExpertsRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let ret = db
                .execute(FunUserListExpertsReq {
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    user_id: ctx.user_id,
                    expert_id: req.expert_id,
                    expert_user_id: req.user_id,
                    expert_user_public_id: req.user_public_id,
                    username: req.username,
                    family_name: req.family_name,
                    given_name: req.given_name,
                    description: req.description,
                    social_media: req.social_media,
                    sort_by_followers: req.sort_by_followers.unwrap_or_default(),
                })
                .await?;
            Ok(UserListExpertsResponse {
                experts_total: ret.first(|x| x.total).unwrap_or_default(),
                experts: ret.map(convert_expert_db_to_api),
            })
        }
        .boxed()
    }
}
pub struct MethodUserListTopPerformingExperts;
impl RequestHandler for MethodUserListTopPerformingExperts {
    type Request = UserListTopPerformingExpertsRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let ret = db
                .execute(FunUserListExpertsReq {
                    user_id: ctx.user_id,
                    expert_id: None,
                    expert_user_id: None,
                    expert_user_public_id: None,
                    username: None,
                    family_name: None,
                    given_name: None,
                    description: None,
                    social_media: None,
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    sort_by_followers: false,
                })
                .await?;
            Ok(UserListTopPerformingExpertsResponse {
                experts_total: ret.first(|x| x.total).unwrap_or_default(),
                experts: ret.map(convert_expert_db_to_api),
            })
        }
        .boxed()
    }
}
pub struct MethodUserListFeaturedExperts;
impl RequestHandler for MethodUserListFeaturedExperts {
    type Request = UserListFeaturedExpertsRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let ret = db
                .execute(FunUserListExpertsReq {
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    user_id: ctx.user_id,
                    expert_id: None,
                    expert_user_id: None,
                    expert_user_public_id: None,
                    username: None,
                    family_name: None,
                    given_name: None,
                    description: None,
                    social_media: None,
                    sort_by_followers: false,
                })
                .await?;
            Ok(UserListFeaturedExpertsResponse {
                experts_total: ret.first(|x| x.total).unwrap_or_default(),
                experts: ret.map(convert_expert_db_to_api),
            })
        }
        .boxed()
    }
}
pub struct MethodUserGetExpertProfile;
impl RequestHandler for MethodUserGetExpertProfile {
    type Request = UserGetExpertProfileRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let ret = db
                .execute(FunUserGetExpertProfileReq {
                    expert_id: req.expert_id,
                    user_id: ctx.user_id,
                })
                .await?
                .into_result()
                .context("failed to get expert profile")?;
            let strategies = db
                .execute(FunUserListStrategiesReq {
                    user_id: ctx.user_id,
                    limit: 10,
                    offset: 0,
                    strategy_id: None,
                    strategy_name: None,
                    expert_public_id: Some(ret.user_public_id),
                    expert_name: None,
                    description: None,
                    blockchain: None,
                    wallet_address: None,
                })
                .await?;
            Ok(UserGetExpertProfileResponse {
                expert_id: ret.expert_id,
                name: ret.username,
                follower_count: ret.follower_count as _,
                description: ret.description.unwrap_or_default(),
                social_media: ret.social_media.unwrap_or_default(),
                risk_score: ret.risk_score.unwrap_or_default(),
                aum: ret.aum.unwrap_or_default(),
                reputation_score: ret.reputation_score.unwrap_or_default(),
                strategies_total: strategies.first(|x| x.total).unwrap_or_default(),
                strategies: strategies.map(convert_strategy_db_to_api),
            })
        }
        .boxed()
    }
}

pub struct MethodUserUpdateUserProfile;
impl RequestHandler for MethodUserUpdateUserProfile {
    type Request = UserUpdateUserProfileRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        // TODO: handle 2nd db for auth
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            db.execute(FunAuthUpdateUserTableReq {
                user_id: ctx.user_id,
                username: req.username,
                family_name: req.family_name,
                given_name: req.given_name,
            })
            .await?;

            let expert = db
                .execute(FunUserGetUserProfileReq {
                    user_id: ctx.user_id,
                })
                .await?
                .into_result()
                .context("Failed to get user profile")?;
            if let Some(expert_id) = expert.expert_id {
                let _ret = db
                    .execute(FunUserUpdateExpertProfileReq {
                        expert_id,
                        description: req.description,
                        social_media: req.social_media,
                    })
                    .await?
                    .into_result()
                    .context("failed to update expert profile")?;
            } else {
                let _ret = db
                    .execute(FunUserCreateExpertProfileReq {
                        user_id: ctx.user_id,
                        description: req.description,
                        social_media: req.social_media,
                    })
                    .await?
                    .into_result()
                    .context("failed to update expert profile")?;
            }

            Ok(UserUpdateUserProfileResponse {})
        }
        .boxed()
    }
}
pub struct MethodUserGetUserProfile;
impl RequestHandler for MethodUserGetUserProfile {
    type Request = UserGetUserProfileRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let ret = db
                .execute(FunUserGetUserProfileReq {
                    user_id: ctx.user_id,
                })
                .await?
                .into_result()
                .context("failed to get expert profile")?;
            let experts = db
                .execute(FunUserListFollowedExpertsReq {
                    user_id: ctx.user_id,
                    offset: 0,
                    limit: DEFAULT_LIMIT,
                })
                .await?;
            let followed_strategies = db
                .execute(FunUserListFollowedStrategiesReq {
                    user_id: ctx.user_id,
                    offset: 0,
                    limit: DEFAULT_LIMIT,
                })
                .await?;
            let backed_strategies = db
                .execute(FunUserListBackedStrategiesReq {
                    user_id: ctx.user_id,
                    offset: 0,
                    limit: DEFAULT_LIMIT,
                })
                .await?;
            // TODO: get followed experts, followed strategies, backed strategies
            Ok(UserGetUserProfileResponse {
                name: ret.name,
                login_wallet: ret.login_wallet,
                joined_at: ret.joined_at,
                follower_count: ret.follower_count.unwrap_or_default() as _,
                description: ret.description.unwrap_or_default(),
                social_media: ret.social_media.unwrap_or_default(),
                followed_experts: experts.map(convert_expert_db_to_api),
                followed_strategies: followed_strategies
                    .into_iter()
                    .map(convert_strategy_db_to_api)
                    .collect(),
                backed_strategies: backed_strategies
                    .into_iter()
                    .map(convert_strategy_db_to_api)
                    .collect(),
            })
        }
        .boxed()
    }
}
pub struct MethodUserRegisterWallet;
impl RequestHandler for MethodUserRegisterWallet {
    type Request = UserRegisterWalletRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;

            let signature_text = hex_decode(req.message_to_sign.as_bytes())?;
            let signature = hex_decode(req.message_signature.as_bytes())?;

            let verified =
                verify_message_address(&signature_text, &signature, req.wallet_address.into())?;

            ensure!(
                verified,
                CustomError::new(EnumErrorCode::InvalidPassword, "Signature is not valid")
            );
            let ret = db
                .execute(FunUserAddRegisteredWalletReq {
                    user_id: ctx.user_id,
                    blockchain: req.blockchain,
                    address: req.wallet_address.into(),
                })
                .await?
                .into_result()
                .context("failed to register wallet")?;

            Ok(UserRegisterWalletResponse {
                success: true,
                wallet_id: ret.registered_wallet_id,
            })
        }
        .boxed()
    }
}

pub struct MethodUserListRegisteredWallets;
impl RequestHandler for MethodUserListRegisteredWallets {
    type Request = UserListRegisteredWalletsRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let blockchain_filter = if let Some(strategy_id) = req.strategy_id {
                let strategy = db
                    .execute(FunUserListStrategiesReq {
                        user_id: ctx.user_id,
                        strategy_id: Some(strategy_id),
                        strategy_name: None,
                        expert_public_id: None,
                        expert_name: None,
                        description: None,
                        blockchain: None,
                        limit: 1,
                        offset: 0,
                        wallet_address: None,
                    })
                    .await?
                    .into_result()
                    .with_context(|| {
                        CustomError::new(EnumErrorCode::NotFound, "Strategy not found")
                    })?;
                Some(strategy.blockchain)
            } else {
                None
            };
            let ret = db
                .execute(FunUserListRegisteredWalletsReq {
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    user_id: Some(ctx.user_id),
                    blockchain: req.blockchain,
                    address: None,
                })
                .await?;

            Ok(UserListRegisteredWalletsResponse {
                wallets: ret
                    .into_iter()
                    .map(|x| ListWalletsRow {
                        wallet_id: x.registered_wallet_id,
                        blockchain: x.blockchain,
                        wallet_address: x.address.into(),
                        is_default: false,
                        is_compatible: if let Some(blockchain) = blockchain_filter {
                            blockchain == x.blockchain
                        } else {
                            true
                        },
                    })
                    .collect(),
            })
        }
        .boxed()
    }
}
pub struct MethodUserDeregisterWallet;
impl RequestHandler for MethodUserDeregisterWallet {
    type Request = UserDeregisterWalletRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;

            let _ret = db
                .execute(FunUserRemoveRegisteredWalletReq {
                    registered_wallet_id: req.wallet_id,
                    user_id: ctx.user_id,
                })
                .await?;

            Ok(UserDeregisterWalletResponse { success: true })
        }
        .boxed()
    }
}
pub struct MethodUserApplyBecomeExpert;
impl RequestHandler for MethodUserApplyBecomeExpert {
    type Request = UserApplyBecomeExpertRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;

            let ret = db
                .execute(FunUserApplyBecomeExpertReq {
                    user_id: ctx.user_id,
                })
                .await?
                .into_result()
                .context("failed to apply become expert")?;

            Ok(UserApplyBecomeExpertResponse {
                success: ret.success,
                expert_id: ret.expert_id,
            })
        }
        .boxed()
    }
}
pub struct MethodExpertCreateStrategy {
    pub cmc_client: Arc<CoinMarketCap>,
}

impl RequestHandler for MethodExpertCreateStrategy {
    type Request = ExpertCreateStrategyRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        let cmc_client = self.cmc_client.clone();
        async move {
            ensure_user_role(ctx, EnumRole::Expert)?;
            ensure!(
                0.0 <= req.strategy_fee && req.strategy_fee <= 1.0,
                CustomError::new(
                    EnumErrorCode::InvalidArgument,
                    "Strategy fee must be less than 1.0 and greater than 0.0"
                )
            );
            ensure!(
                0.0 <= req.expert_fee && req.expert_fee <= 1.0,
                CustomError::new(
                    EnumErrorCode::InvalidArgument,
                    "Expert fee must be less than 1.0 and greater than 0.0"
                )
            );

            let fee_sum = req.strategy_fee + req.expert_fee;
            ensure!(
                fee_sum <= 1.0,
                CustomError::new(
                    EnumErrorCode::InvalidArgument,
                    "Sum of strategy fee and expert fee must be less than 1.0"
                )
            );

            let ret = db
                .execute(FunUserCreateStrategyReq {
                    user_id: ctx.user_id,
                    name: req.name,
                    description: req.description,
                    strategy_thesis_url: req.strategy_thesis_url,
                    minimum_backing_amount_usd: req.minimum_backing_amount_usd.unwrap_or_default(),
                    strategy_fee: req.strategy_fee,
                    expert_fee: req.expert_fee,
                    agreed_tos: req.agreed_tos,
                    wallet_address: req.wallet_address.into(),
                    blockchain: req.wallet_blockchain,
                })
                .await?
                .into_result()
                .context("failed to create strategy")?;
            let audit_rules: Vec<_> = req
                .audit_rules
                .unwrap_or_default()
                .into_iter()
                .filter(|x| get_audit_rules().iter().map(|y| y.id).contains(x))
                .collect();

            for &s in &audit_rules {
                db.execute(FunUserAddStrategyAuditRuleReq {
                    strategy_id: ret.strategy_id,
                    audit_rule_id: s,
                })
                .await?;
            }
            if audit_rules.iter().contains(&AUDIT_TOP25_TOKENS.id) {
                let token_list = cmc_client.get_top_25_coins().await?;
                for token in token_list.data {
                    db.execute(FunUserAddStrategyWhitelistedTokenReq {
                        strategy_id: ret.strategy_id,
                        token_name: token.symbol,
                    })
                    .await?;
                }
            }

            for token in req.initial_tokens {
                let tk = db
                    .execute(FunUserListStrategyInitialTokenRatiosReq {
                        strategy_id: ret.strategy_id,
                        token_id: Some(token.token_id),
                        token_address: None,
                        blockchain: None,
                    })
                    .await?
                    .into_result()
                    .with_context(|| {
                        CustomError::new(
                            EnumErrorCode::NotFound,
                            format!("token not found: {}", token.token_id),
                        )
                    })?;
                if tk.token_name == format!("{:?}", EnumBlockchainCoin::USDC) {
                    db.execute(FunUserAddStrategyInitialTokenRatioReq {
                        strategy_id: ret.strategy_id,
                        token_id: token.token_id,
                        quantity: token.quantity.into(),
                        relative_token_id: Some(tk.token_id),
                        relative_quantity: Some(U256::exp10(18).into()),
                    })
                    .await?;
                } else {
                    db.execute(FunUserAddStrategyInitialTokenRatioReq {
                        strategy_id: ret.strategy_id,
                        token_id: token.token_id,
                        quantity: token.quantity.into(),
                        relative_token_id: None,
                        relative_quantity: None,
                    })
                    .await?;
                }
            }

            Ok(ExpertCreateStrategyResponse {
                success: ret.success,
                strategy_id: ret.strategy_id,
            })
        }
        .boxed()
    }
}
pub struct MethodExpertUpdateStrategy {
    pub logger: AuditLogger,
}
impl RequestHandler for MethodExpertUpdateStrategy {
    type Request = ExpertUpdateStrategyRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        let logger = self.logger.clone();
        async move {
            ensure_user_role(ctx, EnumRole::Expert)?;
            validate_audit_rule_immutable_tokens(&logger, &db, req.strategy_id).await?;
            let strategy = db
                .execute(FunUserListStrategiesReq {
                    strategy_id: Some(req.strategy_id),
                    strategy_name: None,
                    expert_public_id: None,
                    expert_name: None,
                    description: None,
                    blockchain: None,
                    user_id: ctx.user_id,
                    limit: 1,
                    offset: 0,
                    wallet_address: None,
                })
                .await?
                .into_result()
                .with_context(|| {
                    CustomError::new(EnumErrorCode::NotFound, "failed to find strategy")
                })?;
            ensure!(
                strategy.creator_id == ctx.user_id,
                CustomError::new(EnumErrorCode::UserForbidden, "Not your strategy")
            );
            let ret = db
                .execute(FunUserUpdateStrategyReq {
                    user_id: ctx.user_id,
                    strategy_id: req.strategy_id,
                    name: req.name,
                    description: req.description,
                    social_media: req.social_media,
                })
                .await?
                .into_result()
                .context("failed to update strategy")?;

            Ok(ExpertUpdateStrategyResponse {
                success: ret.success,
            })
        }
        .boxed()
    }
}

// pub struct MethodUserDeleteStrategy;
pub struct MethodExpertAddStrategyWatchingWallet {
    pub logger: AuditLogger,
}
impl RequestHandler for MethodExpertAddStrategyWatchingWallet {
    type Request = ExpertAddStrategyWatchingWalletRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        let logger = self.logger.clone();
        async move {
            ensure_user_role(ctx, EnumRole::Expert)?;

            validate_audit_rule_immutable_tokens(&logger, &db, req.strategy_id).await?;
            let strategy = db
                .execute(FunUserListStrategiesReq {
                    strategy_id: Some(req.strategy_id),
                    strategy_name: None,
                    expert_public_id: None,
                    expert_name: None,
                    description: None,
                    blockchain: None,
                    user_id: ctx.user_id,
                    limit: 1,
                    offset: 0,
                    wallet_address: None,
                })
                .await?
                .into_result()
                .with_context(|| {
                    CustomError::new(EnumErrorCode::NotFound, "failed to find strategy")
                })?;
            ensure!(
                strategy.creator_id == ctx.user_id,
                CustomError::new(EnumErrorCode::UserForbidden, "Not your strategy")
            );
            let ret = db
                .execute(FunUserAddStrategyWatchWalletReq {
                    user_id: ctx.user_id,
                    strategy_id: req.strategy_id,
                    wallet_address: req.wallet_address.into(),
                    blockchain: req.blockchain,
                    ratio: req.ratio,
                    // TODO: maybe remove dex?
                    dex: "ALL".to_string(),
                })
                .await?
                .into_result()
                .context("failed to add strategy watching wallet")?;

            Ok(ExpertAddStrategyWatchingWalletResponse {
                success: ret.success,
                wallet_id: ret.watch_wallet_id,
            })
        }
        .boxed()
    }
}
pub struct MethodExpertRemoveStrategyWatchingWallet {
    pub logger: AuditLogger,
}
impl RequestHandler for MethodExpertRemoveStrategyWatchingWallet {
    type Request = ExpertRemoveStrategyWatchingWalletRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        let logger = self.logger.clone();
        async move {
            ensure_user_role(ctx, EnumRole::Expert)?;
            validate_audit_rule_immutable_tokens(&logger, &db, req.strategy_id).await?;
            let ret = db
                .execute(FunUserRemoveStrategyWatchWalletReq {
                    user_id: ctx.user_id,
                    strategy_id: req.strategy_id,
                    watch_wallet_id: req.wallet_id,
                })
                .await?
                .into_result()
                .context("failed to remove strategy watching wallet")?;

            Ok(ExpertRemoveStrategyWatchingWalletResponse {
                success: ret.success,
            })
        }
        .boxed()
    }
}

pub async fn on_user_request_refund(
    _conn: &EthereumRpcConnection,
    ctx: &RequestContext,
    db: &DbClient,
    chain: EnumBlockChain,
    stablecoin_addresses: &BlockchainCoinAddresses,
    escrow_contract: EscrowContract<EitherTransport>,
    quantity: U256,
    wallet_address: Address,
    escrow_signer: impl Key + Clone,
    token: EnumBlockchainCoin,
    logger: DynLogger,
) -> Result<H256> {
    info!(
        "on_user_request_refund {:?} from {:?} transfer {:?} {:?} to {:?}",
        chain,
        escrow_contract.address(),
        quantity,
        token,
        wallet_address
    );

    let token_address = stablecoin_addresses
        .get(chain, token)
        .context("no stablecoin address")?;

    // TODO: check user balance before transfer

    let hash = transfer_token_to_and_ensure_success(
        escrow_contract.clone(),
        &_conn,
        14,
        10,
        Duration::from_secs(10),
        escrow_signer,
        token_address,
        wallet_address,
        quantity,
        logger.clone(),
    )
    .await?;

    db.execute(FunUserRequestRefundReq {
        user_id: ctx.user_id,
        quantity: quantity.into(),
        blockchain: chain,
        user_address: wallet_address.into(),
        receiver_address: wallet_address.into(),
        contract_address: escrow_contract.address().into(),
        transaction_hash: hash.into(),
    })
    .await?
    .into_result()
    .context("No result")?;

    Ok(hash)
}

pub struct MethodExpertAddStrategyInitialTokenRatio {
    pub logger: AuditLogger,
}
impl RequestHandler for MethodExpertAddStrategyInitialTokenRatio {
    type Request = ExpertAddStrategyInitialTokenRatioRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        let logger = self.logger.clone();
        async move {
            ensure_user_role(ctx, EnumRole::Expert)?;

            validate_audit_rule_immutable_tokens(&logger, &db, req.strategy_id).await?;
            let strategy = db
                .execute(FunUserListStrategiesReq {
                    strategy_id: Some(req.strategy_id),
                    strategy_name: None,
                    expert_public_id: None,
                    expert_name: None,
                    description: None,
                    blockchain: None,
                    user_id: ctx.user_id,
                    limit: 1,
                    offset: 0,
                    wallet_address: None,
                })
                .await?
                .into_result()
                .with_context(|| {
                    CustomError::new(EnumErrorCode::NotFound, "failed to find strategy")
                })?;
            ensure!(
                strategy.creator_id == ctx.user_id,
                CustomError::new(EnumErrorCode::UserForbidden, "Not your strategy")
            );
            let ret = db
                .execute(FunUserAddStrategyInitialTokenRatioReq {
                    strategy_id: req.strategy_id,
                    quantity: req.quantity.into(),
                    relative_token_id: None,
                    token_id: req.token_id,
                    relative_quantity: None,
                })
                .await?
                .into_result()
                .context("failed to add strategy initial token ratio")?;

            Ok(ExpertAddStrategyInitialTokenRatioResponse {
                success: true,
                token_id: ret.strategy_initial_token_ratio_id,
            })
        }
        .boxed()
    }
}
pub struct MethodExpertRemoveStrategyInitialTokenRatio {
    pub logger: AuditLogger,
}
impl RequestHandler for MethodExpertRemoveStrategyInitialTokenRatio {
    type Request = ExpertRemoveStrategyInitialTokenRatioRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        let logger = self.logger.clone();

        async move {
            ensure_user_role(ctx, EnumRole::Expert)?;
            validate_audit_rule_immutable_tokens(&logger, &db, req.strategy_id).await?;
            let _ret = db
                .execute(FunUserRemoveStrategyInitialTokenRatioReq {
                    token_id: req.token_id,
                    strategy_id: req.strategy_id,
                })
                .await?
                .into_result()
                .context("failed to remove strategy initial token ratio")?;

            Ok(ExpertRemoveStrategyInitialTokenRatioResponse { success: true })
        }
        .boxed()
    }
}
pub struct MethodUserListStrategyInitialTokenRatio;
impl RequestHandler for MethodUserListStrategyInitialTokenRatio {
    type Request = UserListStrategyInitialTokenRatioRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();

        async move {
            ensure_user_role(ctx, EnumRole::Expert)?;

            let ret = db
                .execute(FunUserListStrategyInitialTokenRatiosReq {
                    strategy_id: req.strategy_id,
                    token_id: None,
                    token_address: None,
                    blockchain: None,
                })
                .await?;

            Ok(UserListStrategyInitialTokenRatioResponse {
                token_ratios_total: ret.first(|x| x.total).unwrap_or_default(),
                token_ratios: ret.map(|x| ListStrategyInitialTokenRatioRow {
                    token_id: x.token_id,
                    token_name: x.token_name,
                    token_address: x.token_address.into(),
                    quantity: x.quantity.into(),
                    updated_at: x.updated_at,
                    created_at: x.created_at,
                }),
            })
        }
        .boxed()
    }
}

pub struct MethodExpertListFollowers;
impl RequestHandler for MethodExpertListFollowers {
    type Request = ExpertListFollowersRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();

        async move {
            ensure_user_role(ctx, EnumRole::Expert)?;

            let ret = db
                .execute(FunExpertListFollowersReq {
                    user_id: ctx.user_id,
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                })
                .await?;

            Ok(ExpertListFollowersResponse {
                followers_total: ret.first(|x| x.total).unwrap_or_default(),
                followers: ret
                    .into_iter()
                    .map(|x| ExpertListFollowersRow {
                        public_id: x.public_id,
                        username: x.username,
                        family_name: x.family_name,
                        given_name: x.given_name,
                        followed_at: x.followed_at,
                        joined_at: x.joined_at,
                    })
                    .collect(),
            })
        }
        .boxed()
    }
}

pub struct MethodExpertListBackers;
impl RequestHandler for MethodExpertListBackers {
    type Request = ExpertListBackersRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();

        async move {
            ensure_user_role(ctx, EnumRole::Expert)?;

            let ret = db
                .execute(FunExpertListBackersReq {
                    user_id: ctx.user_id,
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                })
                .await?;

            Ok(ExpertListBackersResponse {
                backers_total: ret.first(|x| x.total).unwrap_or_default(),
                backers: ret
                    .into_iter()
                    .map(|x| ExpertListBackersRow {
                        public_id: x.public_id,
                        username: x.username,
                        family_name: x.family_name,
                        given_name: x.given_name,
                        backed_at: x.backed_at,
                        joined_at: x.joined_at,
                    })
                    .collect(),
            })
        }
        .boxed()
    }
}
pub struct MethodUserGetDepositTokens;
impl RequestHandler for MethodUserGetDepositTokens {
    type Request = UserGetDepositTokensRequest;

    fn handle(
        &self,
        _toolbox: &Toolbox,
        _ctx: RequestContext,
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        async move {
            let tokens = BlockchainCoinAddresses::new();
            Ok(UserGetDepositTokensResponse {
                tokens: tokens
                    .iter()
                    .map(|(blockchain, token, address)| UserGetDepositTokensRow {
                        blockchain,
                        token,
                        address: address.into(),
                        short_name: format!("{:?}", token),
                        icon_url: "https://etherscan.io/token/images/centre-usdc_28.png"
                            .to_string(),
                        conversion: 1.0, // TODO: register this conversion rate
                    })
                    .collect(),
            })
        }
        .boxed()
    }
}
pub struct MethodUserGetDepositAddresses {
    pub addresses: Vec<UserGetDepositAddressesRow>,
}
impl RequestHandler for MethodUserGetDepositAddresses {
    type Request = UserGetDepositAddressesRequest;

    fn handle(
        &self,
        _toolbox: &Toolbox,
        _ctx: RequestContext,
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let addresses = self.addresses.clone();
        async move { Ok(UserGetDepositAddressesResponse { addresses }) }.boxed()
    }
}
pub struct MethodUserListDepositLedger;
impl RequestHandler for MethodUserListDepositLedger {
    type Request = UserListDepositLedgerRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            let resp = db
                .execute(FunUserListDepositLedgerReq {
                    user_id: Some(ctx.user_id),
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    blockchain: req.blockchain,
                })
                .await?;
            Ok(UserListDepositLedgerResponse {
                ledger_total: resp.first(|x| x.total).unwrap_or_default(),
                ledger: resp
                    .into_iter()
                    .map(|x| UserListDepositLedgerRow {
                        blockchain: x.blockchain,
                        user_address: x.user_address.into(),
                        contract_address: x.contract_address.into(),
                        receiver_address: x.receiver_address.into(),
                        quantity: x.quantity.into(),
                        transaction_hash: x.transaction_hash.into(),
                        created_at: x.created_at,
                    })
                    .collect(),
            })
        }
        .boxed()
    }
}
pub struct MethodUserSubscribeDepositLedger {
    pub manger: Arc<SubscribeManager<AdminSubscribeTopic>>,
}
impl RequestHandler for MethodUserSubscribeDepositLedger {
    type Request = UserSubscribeDepositLedgerRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let manager = self.manger.clone();
        let toolbox = toolbox.clone();
        let db: DbClient = toolbox.get_db();
        async move {
            manager.subscribe(AdminSubscribeTopic::AdminNotifyEscrowLedgerChange, ctx);
            if let Some(limit) = req.initial_data {
                let resp = db
                    .execute(FunUserListDepositLedgerReq {
                        user_id: Some(ctx.user_id),
                        limit,
                        offset: 0,
                        blockchain: req.blockchain,
                    })
                    .await?;
                let manager = manager.clone();
                let toolbox = toolbox.clone();
                tokio::spawn(async move {
                    sleep(Duration::from_secs_f32(0.05)).await;
                    for row in resp.into_iter() {
                        manager.publish_with_filter(
                            &toolbox,
                            AdminSubscribeTopic::AdminNotifyEscrowLedgerChange,
                            &UserListDepositLedgerRow {
                                quantity: row.quantity.into(),
                                blockchain: row.blockchain,
                                user_address: row.user_address.into(),
                                contract_address: row.contract_address.into(),
                                transaction_hash: row.transaction_hash.into(),
                                receiver_address: row.receiver_address.into(),
                                created_at: row.created_at,
                            },
                            |x| x.connection_id == ctx.connection_id,
                        )
                    }
                });
            }
            if req.mock_data.unwrap_or_default() {
                tokio::spawn(async move {
                    for i in 0..10 {
                        sleep(Duration::from_secs(3)).await;
                        let amount = U256::from(i);
                        let key = Secp256k1SecretKey::new_random();
                        info!("Sending mock data to FE, {}..", i);
                        manager.publish_with_filter(
                            &toolbox,
                            AdminSubscribeTopic::AdminNotifyEscrowLedgerChange,
                            &UserListDepositLedgerRow {
                                quantity: amount.into(),
                                blockchain: req
                                    .blockchain
                                    .unwrap_or(EnumBlockChain::EthereumMainnet),
                                user_address: key.address.clone().into(),
                                contract_address: key.address.clone().into(),
                                transaction_hash: H256::random().into(),
                                receiver_address: key.address.clone().into(),
                                created_at: Utc::now().timestamp(),
                            },
                            |x| x.connection_id == ctx.connection_id,
                        )
                    }
                });
            }
            Ok(UserSubscribeDepositLedgerResponse {})
        }
        .boxed()
    }
}

pub struct MethodUserUnsubscribeDepositLedger {
    pub manger: Arc<SubscribeManager<AdminSubscribeTopic>>,
}
impl RequestHandler for MethodUserUnsubscribeDepositLedger {
    type Request = UserUnsubscribeDepositLedgerRequest;

    fn handle(
        &self,
        _toolbox: &Toolbox,
        ctx: RequestContext,
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let manager = self.manger.clone();
        async move {
            manager.unsubscribe(
                AdminSubscribeTopic::AdminNotifyEscrowLedgerChange,
                ctx.connection_id,
            );

            Ok(UserUnsubscribeDepositLedgerResponse {})
        }
        .boxed()
    }
}
pub struct MethodUserListStrategyWallets;
impl RequestHandler for MethodUserListStrategyWallets {
    type Request = UserListStrategyWalletsRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            let resp = db
                .execute(FunUserListStrategyWalletsReq {
                    user_id: ctx.user_id,
                    blockchain: req.blockchain,
                })
                .await?;
            Ok(UserListStrategyWalletsResponse {
                wallets_total: resp.first(|x| x.total).unwrap_or_default(),
                wallets: resp
                    .into_iter()
                    .map(|x| UserListStrategyWalletsRow {
                        blockchain: x.blockchain,
                        address: x.address.into(),
                        created_at: x.created_at,
                    })
                    .collect(),
            })
        }
        .boxed()
    }
}

pub struct MethodUserCreateStrategyWallet {
    pub pool: EthereumRpcConnectionPool,
    pub master_key: Secp256k1SecretKey,
}
impl RequestHandler for MethodUserCreateStrategyWallet {
    type Request = UserCreateStrategyWalletRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        let pool = self.pool.clone();
        let master_key = self.master_key.clone();
        async move {
            let conn = pool.get(req.blockchain).await?;
            let strategy_wallet = back_strategy::deploy_wallet_contract(
                &conn,
                master_key.clone(),
                req.wallet_address.into(),
                match req.adminship {
                    true => master_key.address(),
                    false => Address::zero(),
                },
                DynLogger::empty(),
            )
            .await?;

            db.execute(FunUserAddStrategyWalletReq {
                // TODO: add opt in adminship in database for each strategy wallet
                // TODO: add backer wallet address registered in strategy wallet in database
                user_id: ctx.user_id,
                blockchain: req.blockchain,
                address: strategy_wallet.address().into(),
            })
            .await?;
            Ok(UserCreateStrategyWalletResponse {
                blockchain: req.blockchain,
                address: strategy_wallet.address().into(),
            })
        }
        .boxed()
    }
}

pub struct MethodUserListStrategyAuditRules;

impl RequestHandler for MethodUserListStrategyAuditRules {
    type Request = UserListStrategyAuditRulesRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        _ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            let rules = get_audit_rules();
            if let Some(strategy_id) = req.strategy_id {
                let resp = db
                    .execute(FunUserListStrategyAuditRulesReq {
                        strategy_id,
                        audit_rule_id: None,
                    })
                    .await?;
                Ok(UserListStrategyAuditRulesResponse {
                    audit_rules: resp
                        .into_iter()
                        .map(|x| {
                            let rule = rules
                                .iter()
                                .find(|y| x.rule_id == y.id)
                                .context("Could not find rule")?;
                            Ok::<_, Error>(UserListStrategyAuditRulesRow {
                                rule_id: x.rule_id,
                                rule_name: rule.name.to_string(),
                                rule_description: rule.description.to_string(),
                                created_at: x.created_at,
                                enabled: true,
                            })
                        })
                        .try_collect()?,
                })
            } else {
                Ok(UserListStrategyAuditRulesResponse {
                    audit_rules: rules
                        .into_iter()
                        .map(|rule| UserListStrategyAuditRulesRow {
                            rule_id: rule.id as _,
                            rule_name: rule.name.to_string(),
                            rule_description: rule.description.to_string(),
                            created_at: 0,
                            enabled: true,
                        })
                        .collect(),
                })
            }
        }
        .boxed()
    }
}
pub struct MethodUserAddStrategyAuditRule;
impl RequestHandler for MethodUserAddStrategyAuditRule {
    type Request = UserAddStrategyAuditRuleRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            let strategy = db
                .execute(FunUserListStrategiesReq {
                    strategy_id: Some(req.strategy_id),
                    strategy_name: None,
                    expert_public_id: None,
                    expert_name: None,
                    description: None,
                    blockchain: None,
                    user_id: ctx.user_id,
                    limit: 1,
                    offset: 0,
                    wallet_address: None,
                })
                .await?
                .into_result()
                .with_context(|| {
                    CustomError::new(EnumErrorCode::NotFound, "failed to find strategy")
                })?;
            ensure!(
                strategy.creator_id == ctx.user_id,
                CustomError::new(EnumErrorCode::UserForbidden, "Not your strategy")
            );
            ensure!(
                strategy.immutable_audit_rules,
                CustomError::new(EnumErrorCode::UserForbidden, "Strategy rules immutable")
            );
            db.execute(FunUserAddStrategyAuditRuleReq {
                strategy_id: req.strategy_id,
                audit_rule_id: req.rule_id,
            })
            .await?;
            Ok(UserAddStrategyAuditRuleResponse {})
        }
        .boxed()
    }
}
pub struct MethodUserRemoveStrategyAuditRule;
impl RequestHandler for MethodUserRemoveStrategyAuditRule {
    type Request = UserRemoveStrategyAuditRuleRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            let strategy = db
                .execute(FunUserListStrategiesReq {
                    strategy_id: Some(req.strategy_id),
                    strategy_name: None,
                    expert_public_id: None,
                    expert_name: None,
                    description: None,
                    blockchain: None,
                    user_id: ctx.user_id,
                    limit: 1,
                    offset: 0,
                    wallet_address: None,
                })
                .await?
                .into_result()
                .with_context(|| {
                    CustomError::new(EnumErrorCode::NotFound, "failed to find strategy")
                })?;
            ensure!(
                strategy.creator_id == ctx.user_id,
                CustomError::new(EnumErrorCode::UserForbidden, "Not your strategy")
            );
            ensure!(
                strategy.immutable_audit_rules,
                CustomError::new(EnumErrorCode::UserForbidden, "Strategy rules immutable")
            );
            db.execute(FunUserDelStrategyAuditRuleReq {
                strategy_id: req.strategy_id,
                audit_rule_id: req.rule_id,
            })
            .await?;
            Ok(UserRemoveStrategyAuditRuleResponse {})
        }
        .boxed()
    }
}
pub struct MethodUserGetEscrowAddressForStrategy {
    pub addresses: Vec<UserGetDepositAddressesRow>,
}
impl RequestHandler for MethodUserGetEscrowAddressForStrategy {
    type Request = UserGetEscrowAddressForStrategyRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let escrow_addresses = self.addresses.clone();
        let db = toolbox.get_db();
        async move {
            let strategy = db
                .execute(FunUserListStrategiesReq {
                    user_id: ctx.user_id,
                    limit: 1,
                    offset: 0,
                    strategy_id: Some(req.strategy_id),
                    strategy_name: None,
                    expert_public_id: None,
                    expert_name: None,
                    description: None,
                    blockchain: None,
                    wallet_address: None,
                })
                .await?
                .into_result()
                .with_context(|| {
                    CustomError::new(EnumErrorCode::NotFound, "Could not find strategy")
                })?;
            let tokens_contracts = db
                .execute(FunUserListEscrowTokenContractAddressReq {
                    limit: 100,
                    offset: 0,
                    token_id: None,
                    blockchain: Some(strategy.blockchain),
                    address: None,
                    // TODO: support other symbols
                    symbol: Some("USDC".to_string()),
                    is_stablecoin: None,
                })
                .await?
                .into_rows();
            let mut tokens = vec![];
            for token in tokens_contracts {
                for address in &escrow_addresses {
                    if address.blockchain != strategy.blockchain {
                        continue;
                    }

                    let tk = UserAllowedEscrowTransferInfo {
                        receiver_address: address.address.clone(),
                        blockchain: token.blockchain,
                        token_id: token.token_id,
                        token_symbol: token.symbol.clone(),
                        token_name: token.short_name.clone(),
                        token_address: token.address.clone().into(),
                    };
                    tokens.push(tk);
                }
            }
            Ok(UserGetEscrowAddressForStrategyResponse { tokens })
        }
        .boxed()
    }
}
pub struct MethodUserListEscrowTokenContractAddresses;
impl RequestHandler for MethodUserListEscrowTokenContractAddresses {
    type Request = UserListEscrowTokenContractAddressesRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let balances = db
                .execute(FunUserListEscrowTokenContractAddressReq {
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    token_id: None,
                    blockchain: req.blockchain,
                    address: None,
                    symbol: None,
                    is_stablecoin: req.is_stablecoin,
                })
                .await?;
            Ok(UserListEscrowTokenContractAddressesResponse {
                tokens_total: balances.first(|x| x.total).unwrap_or_default(),
                tokens: balances.map(|x| UserListEscrowTokenContractAddressesRow {
                    blockchain: x.blockchain,
                    token_id: x.token_id,
                    token_symbol: x.symbol,
                    token_name: x.short_name,
                    token_address: x.address.into(),
                    description: x.description,
                    is_stablecoin: x.is_stablecoin,
                }),
            })
        }
        .boxed()
    }
}
pub struct MethodUserListBackStrategyLedger;
impl RequestHandler for MethodUserListBackStrategyLedger {
    type Request = UserListBackStrategyLedgerRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let ledger = db
                .execute(FunUserListBackStrategyLedgerReq {
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    strategy_id: req.strategy_id,
                    user_id: Some(ctx.user_id),
                })
                .await?;
            Ok(UserListBackStrategyLedgerResponse {
                back_ledger_total: ledger.first(|x| x.total).unwrap_or_default(),
                back_ledger: ledger.map(|x| BackStrategyLedgerRow {
                    back_ledger_id: x.back_ledger_id,
                    strategy_id: x.strategy_id,
                    quantity: x.quantity.into(),
                    blockchain: x.blockchain,
                    transaction_hash: x.transaction_hash.into(),
                    happened_at: x.happened_at,
                }),
            })
        }
        .boxed()
    }
}

pub struct MethodUserListStrategyTokenBalance;
impl RequestHandler for MethodUserListStrategyTokenBalance {
    type Request = UserListStrategyTokenBalanceRequest;
    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db = toolbox.get_db();
        async move {
            let balance = db
                .execute(FunUserListUserStrategyBalanceReq {
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    user_id: ctx.user_id,
                    strategy_id: req.strategy_id,
                })
                .await?;
            Ok(UserListStrategyTokenBalanceResponse {
                tokens_total: balance.first(|x| x.total).unwrap_or_default(),
                tokens: balance.map(|x| UserListStrategyTokenBalanceRow {
                    strategy_id: x.strategy_id,
                    strategy_name: x.strategy_name,
                    blockchain: x.blockchain,
                    address: x.user_strategy_wallet_address.into(),
                    balance: x.balance.into(),
                }),
            })
        }
        .boxed()
    }
}
pub struct MethodUserGetBackStrategyReviewDetail {
    pub pool: EthereumRpcConnectionPool,
    pub escrow_contract: Arc<AbstractEscrowContract>,
    pub master_key: Secp256k1SecretKey,
    pub dex_addresses: Arc<DexAddresses>,
}

impl RequestHandler for MethodUserGetBackStrategyReviewDetail {
    type Request = UserGetBackStrategyReviewDetailRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let strategy = db
                .execute(FunUserListStrategiesReq {
                    user_id: ctx.user_id,
                    limit: 1,
                    offset: 0,
                    strategy_id: Some(req.strategy_id),
                    strategy_name: None,
                    expert_public_id: None,
                    expert_name: None,
                    description: None,
                    blockchain: None,
                    wallet_address: None,
                })
                .await?
                .into_result()
                .with_context(|| {
                    CustomError::new(EnumErrorCode::NotFound, "Could not find strategy")
                })?;
            let user_back_fee_ratio = strategy.swap_fee.unwrap_or_default()
                + strategy.expert_fee.unwrap_or_default()
                + strategy.strategy_fee.unwrap_or_default();
            let token = db
                .execute(FunUserListStrategyInitialTokenRatiosReq {
                    strategy_id: req.strategy_id,
                    token_id: Some(req.token_id),
                    token_address: None,
                    blockchain: None,
                })
                .await?
                .into_result()
                .context("initial token not found in strategy")?;
            let strategy_fee = req.quantity * (100000.0 * user_back_fee_ratio) as u64 / 100000;
            let sp_tokens = calculate_sp_tokens_to_mint_easy_approach(
                token,
                req.quantity - strategy_fee,
                18.into(),
            )
            .await?;
            Ok(UserGetBackStrategyReviewDetailResponse {
                strategy_fee,
                total_amount_to_back: req.quantity,
                estimated_amount_of_strategy_tokens: sp_tokens,
            })
        }
        .boxed()
    }
}
