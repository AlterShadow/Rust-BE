use crate::admin_method::AdminSubscribeTopic;
use crate::audit::{
    get_audit_rules, validate_audit_rule_immutable_tokens, AuditLogger, AUDIT_TOP25_TOKENS,
};
use crate::back_strategy;
use crate::back_strategy::{
    calculate_user_back_strategy_calculate_amount_to_mint,
    CalculateUserBackStrategyCalculateAmountToMintResult,
};
use api::cmc::CoinMarketCap;
use chrono::Utc;
use eth_sdk::erc20::Erc20Token;
use eth_sdk::escrow::refund_asset_and_ensure_success;
use eth_sdk::escrow::{AbstractEscrowContract, EscrowContract};
use eth_sdk::pancake_swap::pair_paths::WorkingPancakePairPaths;
use eth_sdk::signer::Secp256k1SecretKey;
use eth_sdk::strategy_pool::{withdraw_and_ensure_success, StrategyPoolContract};
use eth_sdk::strategy_pool_herald::parse_herald_redeem_event;
use eth_sdk::strategy_wallet::{
    full_redeem_from_strategy_and_ensure_success, redeem_from_strategy_and_ensure_success,
    StrategyWalletContract,
};

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
use lib::types::amount_to_display;
use lib::ws::SubscribeManager;
use lib::{DEFAULT_LIMIT, DEFAULT_OFFSET};
use lru::LruCache;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tracing::*;
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
pub struct MethodUserListFollowedStrategies {
    pub cmc: Arc<CoinMarketCap>,
}

impl RequestHandler for MethodUserListFollowedStrategies {
    type Request = UserListFollowedStrategiesRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        let cmc = self.cmc.clone();
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
                strategies: ret
                    .map_async(|x| convert_strategy_db_to_api_net_value(x, &cmc, &db))
                    .await?,
            })
        }
        .boxed()
    }
}

pub struct MethodUserListStrategies {
    pub cmc: Arc<CoinMarketCap>,
}

impl RequestHandler for MethodUserListStrategies {
    type Request = UserListStrategiesRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        let cmc = self.cmc.clone();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;

            let ret = db
                .execute(FunUserListStrategiesReq {
                    user_id: ctx.user_id,
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    strategy_id: req.strategy_id,
                    strategy_name: req.strategy_name,
                    expert_id: None,
                    expert_public_id: req.expert_public_id,
                    expert_name: req.expert_name,
                    description: req.description,
                    blockchain: req.blockchain,
                    strategy_pool_address: req.strategy_pool_address.map(|x| x.into()),
                    approved: Some(true),
                })
                .await?;

            Ok(UserListStrategiesResponse {
                strategies_total: ret.first(|x| x.total).unwrap_or_default(),
                strategies: ret
                    .map_async(|v| convert_strategy_db_to_api_net_value(v, &cmc, &db))
                    .await?,
            })
        }
        .boxed()
    }
}

pub struct MethodUserListTopPerformingStrategies {
    pub cmc: Arc<CoinMarketCap>,
}

impl RequestHandler for MethodUserListTopPerformingStrategies {
    type Request = UserListTopPerformingStrategiesRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        let cmc = self.cmc.clone();
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
                    expert_id: None,
                    expert_public_id: None,
                    expert_name: None,
                    description: None,
                    blockchain: None,
                    strategy_pool_address: None,
                    approved: Some(true),
                })
                .await?;
            Ok(UserListTopPerformingStrategiesResponse {
                strategies_total: ret.first(|x| x.total).unwrap_or_default(),
                strategies: ret
                    .map_async(|v| convert_strategy_db_to_api_net_value(v, &cmc, &db))
                    .await?,
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
pub struct MethodUserGetStrategy {
    pub cmc: Arc<CoinMarketCap>,
}
impl RequestHandler for MethodUserGetStrategy {
    type Request = UserGetStrategyRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        let cmc = self.cmc.clone();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let ret = db
                .execute(FunUserListStrategiesReq {
                    user_id: ctx.user_id,
                    limit: 1,
                    offset: 0,
                    strategy_id: Some(req.strategy_id),
                    strategy_name: None,
                    expert_id: None,
                    expert_public_id: None,
                    expert_name: None,
                    description: None,
                    blockchain: None,
                    strategy_pool_address: None,
                    approved: None,
                })
                .await?
                .into_result()
                .context("failed to get strategy")?;
            let balances = db
                .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
                    strategy_pool_contract_id: None,
                    strategy_id: Some(req.strategy_id),
                    blockchain: None,
                    token_address: None,
                })
                .await?;
            let ledger = db
                .execute(FunUserListStrategyPoolContractAssetLedgerReq {
                    limit: 1000,
                    offset: 0,
                    strategy_id: Some(req.strategy_id),
                    token_id: None,
                    blockchain: None,
                })
                .await?
                .into_rows();
            let list_strategy_pool_contract_asset_balances = db
                .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
                    strategy_pool_contract_id: None,
                    strategy_id: Some(req.strategy_id),
                    blockchain: None,
                    token_address: None,
                })
                .await?;
            let token_symbols = list_strategy_pool_contract_asset_balances
                .clone()
                .map(|x| x.token_symbol);
            let prices = cmc
                .get_usd_prices_by_symbol(&token_symbols)
                .await
                .context("failed to get price")?;

            Ok(UserGetStrategyResponse {
                strategy: convert_strategy_db_to_api_net_value(ret, &cmc, &db).await?,
                watching_wallets: db
                    .execute(FunUserListStrategyWatchWalletsReq {
                        strategy_id: req.strategy_id,
                    })
                    .await?
                    .map(|x| WatchingWalletRow {
                        watching_wallet_id: x.strategy_watch_wallet_id,
                        wallet_address: x.wallet_address.into(),
                        blockchain: x.blockchain,
                        ratio_distribution: x.ratio,
                    }),
                strategy_pool_asset_updated_at: ledger
                    .last()
                    .map(|x| x.happened_at)
                    .unwrap_or_else(|| Utc::now().timestamp_nanos()),
                strategy_pool_asset_balances: balances
                    .map_async(|x| {
                        let cmc = &cmc;
                        let token_symbols = &token_symbols;
                        let prices = &prices;
                        async move {
                            let price_usd = token_symbols
                                .iter()
                                .zip(prices.iter())
                                .find(|(k, _v)| k.as_str() == x.token_symbol.as_str())
                                .map(|y| *y.1)
                                .unwrap_or_default();
                            let price_usd_7d = cmc
                                .get_usd_price_days_ago(x.token_symbol.clone(), 7)
                                .await?;
                            let price_usd_30d = cmc
                                .get_usd_price_days_ago(x.token_symbol.clone(), 30)
                                .await?;
                            Ok(StrategyPoolAssetBalancesRow {
                                name: x.token_name,
                                symbol: x.token_symbol,
                                address: x.token_address.into(),
                                blockchain: x.blockchain,
                                balance: x.balance.into(),
                                price_usd,
                                price_usd_7d,
                                price_usd_30d,
                            })
                        }
                    })
                    .await?,
                strategy_pool_asset_ledger: ledger
                    .into_iter()
                    .map(|x| StrategyPoolAssetLedgerRow {
                        aum_ledger_id: x.entry_id,
                        symbol: x.token_symbol,
                        token_id: x.token_id,
                        blockchain: x.blockchain,
                        dex: x.dex.unwrap_or_default(),
                        transaction_hash: x.transaction_hash.into(),
                        quantity: x.amount.into(),
                        is_add: x.is_add,
                        happened_at: x.happened_at,
                    })
                    .collect(),
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
pub struct MethodUserGetStrategiesStatistics {
    pub cmc: Arc<CoinMarketCap>,
}
impl RequestHandler for MethodUserGetStrategiesStatistics {
    type Request = UserGetStrategiesStatisticsRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        let cmc = self.cmc.clone();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let strategies = db
                .execute(FunUserListStrategiesReq {
                    user_id: ctx.user_id,
                    limit: 1000,
                    offset: 0,
                    strategy_id: None,
                    strategy_name: None,
                    expert_id: None,
                    expert_public_id: None,
                    expert_name: None,
                    description: None,
                    blockchain: None,
                    strategy_pool_address: None,
                    approved: None,
                })
                .await?
                .map_async(|x| convert_strategy_db_to_api_net_value(x, &cmc, &db))
                .await?;
            let list_strategy_pool_contract_asset_balances = db
                .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
                    strategy_pool_contract_id: None,
                    strategy_id: None,
                    blockchain: None,
                    token_address: None,
                })
                .await?;
            let token_prices = list_strategy_pool_contract_asset_balances
                .clone()
                .map(|x| x.token_symbol);
            let prices = cmc
                .get_usd_prices_by_symbol(&token_prices)
                .await
                .context("failed to get price")?;
            let backing_amount_usd: f64 = list_strategy_pool_contract_asset_balances
                .into_iter()
                .zip(prices.into_iter())
                .map(|(x, price)| x.balance.0.div_as_f64(U256::exp10(18)).unwrap() * price)
                .sum();
            Ok(UserGetStrategiesStatisticsResponse {
                tracking_amount_usd: 0.0,
                backing_amount_usd,
                difference_amount_usd: 0.0,
                aum_value_usd: strategies.iter().map(|x| x.aum).sum(),
                current_value_usd: backing_amount_usd,
                withdrawable_value_usd: backing_amount_usd,
                strategy_pool_tokens: vec![],
                aum_list_history: vec![], // TODO: get history
            })
        }
        .boxed()
    }
}

pub struct MethodUserListBackedStrategies {
    pub cmc: Arc<CoinMarketCap>,
}
impl RequestHandler for MethodUserListBackedStrategies {
    type Request = UserListBackedStrategiesRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        let cmc = self.cmc.clone();
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
                strategies: ret
                    .map_async(|v| convert_strategy_db_to_api_net_value(v, &cmc, &db))
                    .await?,
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

pub struct MethodUserGetDepositWithdrawBalance {
    pub escrow_addresses: Arc<EscrowAddresses>,
}
impl RequestHandler for MethodUserGetDepositWithdrawBalance {
    type Request = UserGetDepositWithdrawBalanceRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        let escrow_addresses = self.escrow_addresses.clone();
        async move {
            let token = db
                .execute(FunUserListEscrowTokenContractAddressReq {
                    limit: 1,
                    blockchain: None,
                    address: None,
                    symbol: None,
                    token_id: Some(req.token_id),
                    offset: 0,
                    is_stablecoin: None,
                })
                .await?
                .into_result()
                .with_context(|| CustomError::new(EnumErrorCode::NotFound, "no such token"))?;

            let balance = db
                .execute(FunUserListUserDepositWithdrawBalanceReq {
                    limit: 1,
                    offset: 0,
                    user_id: ctx.user_id,
                    blockchain: None,
                    token_address: None,
                    token_id: Some(req.token_id),
                    escrow_contract_address: Some(
                        escrow_addresses
                            .get(token.blockchain, ())
                            .context("no such blockchain")?
                            .into(),
                    ),
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
    pub pancake_paths: Arc<WorkingPancakePairPaths>,
    pub lru: Arc<Mutex<LruCache<i64, ()>>>,
    pub cmc: Arc<CoinMarketCap>,
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
        let lru = self.lru.clone();
        let pancake_swap = self.pancake_paths.clone();
        let cmc = self.cmc.clone();
        async move {
            {
                let mut lru = lru.lock().await;
                if lru.put(req.nonce, ()).is_some() {
                    bail!(CustomError::new(
                        EnumErrorCode::DuplicateRequest,
                        "Duplicate request",
                    ));
                }
            }
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
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
            let attempt = db
                .execute(FunUserSaveUserBackStrategyAttemptReq {
                    strategy_id: req.strategy_id,
                    user_id: ctx.user_id,
                    token_id: token.token_id,
                    back_quantity: req.quantity.into(),
                    // TODO: this wallet maybe newly created
                    strategy_wallet_address: req.strategy_wallet.unwrap_or_default().into(),
                    log_id: ctx.log_id as _,
                })
                .await?
                .into_result()
                .context("Failed to save user back strategy attempt")?;
            {
                let db = db.clone();

                tokio::spawn(async move {
                    while let Some(msg) = rx.recv().await {
                        if let Err(err) = db
                            .execute(FunUserSaveUserBackStrategyLogReq {
                                user_back_strategy_attempt_id: attempt
                                    .user_back_strategy_attempt_id,
                                message: msg,
                            })
                            .await
                        {
                            error!("Failed to save user back strategy log: {:?}", err);
                        }
                    }
                });
            }
            let report_progress = move |end: bool, msg: &str, hash: H256| {
                // TODO: have dedicated thread to write logs, instead of writing
                let _ = tx.send(msg.to_string());
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
                    req.strategy_wallet,
                    logger.clone(),
                    &pancake_swap,
                    &cmc,
                )
                .await
                {
                    error!("user back strategy error: {:?}", err);
                    logger.log(format!("user back strategy error {}", err));
                }
            });
            Ok::<_, Error>(UserBackStrategyResponse {})
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
    /* instantiate strategy wallet */
    let strategy_wallet_contract_row = db
        .execute(FunUserListStrategyWalletsReq {
            user_id: Some(ctx.user_id),
            blockchain: Some(blockchain),
            strategy_wallet_address: None,
        })
        .await?
        .into_result()
        .context("user has no strategy wallet on this chain")?;
    if !strategy_wallet_contract_row.is_platform_managed {
        bail!("user strategy wallet is not platform managed");
    }
    let strategy_wallet_contract =
        StrategyWalletContract::new(conn.clone(), strategy_wallet_contract_row.address.into())?;

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

    /* check share balance first */
    let user_strategy_balance = db
        .execute(FunWatcherListUserStrategyBalanceReq {
            limit: 1,
            offset: 0,
            strategy_id: Some(strategy_id),
            user_id: Some(ctx.user_id),
            blockchain: Some(blockchain),
        })
        .await?
        .first(|x| x.balance)
        .unwrap_or_default();

    /* get user strategy pool contract assets owned by this strategy wallet */
    let asset_balances_owned_by_strategy_wallet = db
        .execute(FunUserListUserStrategyPoolContractAssetBalancesReq {
            strategy_pool_contract_id: Some(strategy_pool_contract_row.pkey_id),
            user_id: Some(ctx.user_id),
            strategy_wallet_id: Some(strategy_wallet_contract_row.wallet_id),
            token_address: None,
            blockchain: Some(blockchain),
        })
        .await?
        .into_rows();

    /* redeem */
    let mut assets_to_transfer: Vec<Address> = Vec::new();
    let mut amounts_to_transfer: Vec<U256> = Vec::new();
    let redeem_tx_hash: H256;
    match maybe_strategy_tokens_to_redeem {
        Some(strategy_tokens_to_redeem) => {
            /* check balance first */
            if user_strategy_balance < strategy_tokens_to_redeem.into() {
                bail!(
                    "not enough strategy tokens {} < {}",
                    amount_to_display(*user_strategy_balance),
                    amount_to_display(strategy_tokens_to_redeem)
                );
            }

            /* get assets and amounts to withdraw */
            for asset_balance_owned_by_strategy_wallet in asset_balances_owned_by_strategy_wallet {
                if asset_balance_owned_by_strategy_wallet.balance == U256::zero().into() {
                    continue;
                }

                assets_to_transfer
                    .push(asset_balance_owned_by_strategy_wallet.token_address.into());
                let amount_owned: U256 = asset_balance_owned_by_strategy_wallet.balance.into();
                amounts_to_transfer.push(
                    amount_owned
                        .mul_div(strategy_tokens_to_redeem, user_strategy_balance.into())?,
                );
            }

            /* if strategy is currently trading, redeem is not possible */
            redeem_tx_hash = redeem_from_strategy_and_ensure_success(
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
            .context("redeem is not possible currently")?;
        }
        None => {
            /* get assets and amounts to withdraw */
            for asset_balance_owned_by_strategy_wallet in asset_balances_owned_by_strategy_wallet {
                if asset_balance_owned_by_strategy_wallet.balance == U256::zero().into() {
                    continue;
                }

                assets_to_transfer
                    .push(asset_balance_owned_by_strategy_wallet.token_address.into());
                amounts_to_transfer.push(asset_balance_owned_by_strategy_wallet.balance.into());
            }

            /* if strategy is currently trading, redeem is not possible */
            redeem_tx_hash = full_redeem_from_strategy_and_ensure_success(
                strategy_wallet_contract.clone(),
                &conn,
                CONFIRMATIONS,
                MAX_RETRIES,
                POLL_INTERVAL,
                master_key.clone(),
                strategy_pool_contract.address(),
            )
            .await
            .context("redeem is not possible currently")?;
        }
    };

    /* parse redeem event */
    let redeem_info = parse_herald_redeem_event(
        StrategyPoolHeraldAddresses::new()
            .get(blockchain, ())
            .context(
                "could not retrieve strategy pool herald address for this chain on exit strategy",
            )?,
        conn.eth()
            .transaction_receipt(redeem_tx_hash)
            .await?
            .context("redeem transaction receipt not found even though it has confirmations")?,
    )?;

    /* transfer assets to user wallet address registered in strategy wallet */
    // TODO: get logger from main
    let logger = DynLogger::new(Arc::new(move |msg| {
        println!("{}", msg);
    }));

    // TODO: cache this and withdraw later if strategy pool started trading between redeem and withdraw
    let transaction_hash = withdraw_and_ensure_success(
        strategy_pool_contract,
        &conn,
        CONFIRMATIONS,
        MAX_RETRIES,
        POLL_INTERVAL,
        master_key.clone(),
        assets_to_transfer.clone(),
        amounts_to_transfer.clone(),
        redeem_info.backer,
        logger,
    )
    .await?;

    /* update strategy pool asset balances & ledgers */
    for idx in 0..assets_to_transfer.len() {
        /* update per-user strategy pool asset balance & ledger */
        let asset = assets_to_transfer[idx];
        let amount = amounts_to_transfer[idx];
        let asset_old_balance: U256 = db
            .execute(FunUserListUserStrategyPoolContractAssetBalancesReq {
                strategy_pool_contract_id: Some(strategy_pool_contract_row.pkey_id),
                user_id: Some(ctx.user_id),
                strategy_wallet_id: Some(strategy_wallet_contract_row.wallet_id),
                token_address: Some(asset.into()),
                blockchain: Some(blockchain),
            })
            .await?
            .into_result()
            .context("user strategy pool asset balance not found")?
            .balance
            .into();

        db.execute(FunUserUpsertUserStrategyPoolContractAssetBalanceReq {
            strategy_pool_contract_id: strategy_pool_contract_row.pkey_id,
            strategy_wallet_id: strategy_wallet_contract_row.wallet_id,
            token_address: asset.into(),
            blockchain,
            old_balance: asset_old_balance.into(),
            new_balance: match asset_old_balance.try_checked_sub(amount) {
                Ok(new_balance) => new_balance.into(),
                Err(_) => U256::zero().into(),
            },
        })
        .await?;

        db.execute(FunUserAddUserStrategyPoolContractAssetLedgerEntryReq {
            strategy_pool_contract_id: strategy_pool_contract_row.pkey_id,
            strategy_wallet_id: strategy_wallet_contract_row.wallet_id,
            token_address: asset.into(),
            blockchain,
            amount: amount.into(),
            is_add: false,
        })
        .await?;

        /* update strategy pool asset balances & ledger */
        let old_asset_balance_row = db
            .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
                strategy_pool_contract_id: Some(strategy_pool_contract_row.pkey_id),
                strategy_id: None,
                blockchain: Some(blockchain),
                token_address: Some(asset.into()),
            })
            .await?
            .into_result()
            .context("strategy pool balance of redeemed asset not found")?;

        db.execute(FunWatcherUpsertStrategyPoolContractAssetBalanceReq {
            strategy_pool_contract_id: strategy_pool_contract_row.pkey_id,
            token_address: asset.into(),
            blockchain,
            new_balance: old_asset_balance_row
                .balance
                .try_checked_sub(amount)
                .context("redeemed amount is greater than known balance of strategy pool asset")?
                .into(),
        })
        .await?;

        db.execute(FunUserAddStrategyPoolContractAssetLedgerEntryReq {
            strategy_pool_contract_id: strategy_pool_contract_row.pkey_id,
            token_address: asset.into(),
            blockchain,
            amount: amount.into(),
            is_add: false,
            transaction_hash: transaction_hash.into(),
        })
        .await?;
    }

    /* update exit strategy ledger */
    db.execute(FunUserExitStrategyReq {
        user_id: ctx.user_id,
        strategy_id,
        // TODO: calculate value of sp tokens exit in usdc
        quantity: U256::zero().into(),
        blockchain,
        transaction_hash: redeem_tx_hash.into(),
        redeem_sp_tokens: redeem_info.amount.into(),
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
        new_balance: (user_strategy_balance.try_checked_sub(redeem_info.amount)?).into(),
    })
    .await?;

    Ok(redeem_tx_hash)
}

pub struct MethodUserExitStrategy {
    pub pool: EthereumRpcConnectionPool,
    pub master_key: Secp256k1SecretKey,
    pub lru: Arc<Mutex<LruCache<i64, ()>>>,
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
        let lru = self.lru.clone();
        async move {
            {
                let mut lru = lru.lock().await;
                if lru.put(req.nonce, ()).is_some() {
                    bail!(CustomError::new(
                        EnumErrorCode::DuplicateRequest,
                        "duplicate request"
                    ))
                }
            }
            let eth_conn = pool.get(req.blockchain).await?;
            // TODO: decide if we should ensure user role
            ensure_user_role(ctx, EnumRole::User)?;
            let tx_hash = match user_exit_strategy(
                &eth_conn,
                &ctx,
                &db,
                req.blockchain,
                req.strategy_id,
                Some(req.quantity),
                master_key,
            )
            .await
            {
                Ok(tx_hash) => tx_hash,
                Err(e) => {
                    error!("error on user exit strategy: {:?}", e);
                    let err = format!("{}", e);
                    return Err(CustomError::new(EnumErrorCode::InvalidArgument, err).into());
                }
            };

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
    pub lru: Arc<Mutex<LruCache<i64, ()>>>,
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
        let lru = self.lru.clone();
        async move {
            {
                let mut lru = lru.lock().await;
                if lru.put(req.nonce, ()).is_some() {
                    bail!(CustomError::new(
                        EnumErrorCode::DuplicateRequest,
                        "duplicate request"
                    ))
                }
            }
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

    let refunded_token_row = db
        .execute(FunUserListEscrowTokenContractAddressReq {
            limit: 1,
            offset: 0,
            token_id: None,
            blockchain: Some(chain),
            address: Some(token_address.into()),
            symbol: None,
            is_stablecoin: None,
        })
        .await?
        .into_result()
        .context("could not get refunded token contract from database")?;

    let deposit_withdraw_balance_row = db
        .execute(FunUserListUserDepositWithdrawBalanceReq {
            limit: 1,
            offset: 0,
            user_id: ctx.user_id,
            blockchain: Some(chain),
            token_id: Some(refunded_token_row.token_id),
            token_address: Some(token_address.into()),
            escrow_contract_address: Some(escrow_contract.address().into()),
        })
        .await?
        .into_result()
        .context("could not get deposit withdraw balance from database")?;

    let old_balance: U256 = deposit_withdraw_balance_row.balance.into();
    if old_balance < quantity {
        bail!("unsufficient balance for refund");
    }

    /* get amount deposited by whitelisted wallets necessary for refund */
    let positive_deposit_balance_of_wallet_row = db
        .execute(FunUserCalculateUserEscrowBalanceFromLedgerReq {
            user_id: ctx.user_id,
            blockchain: chain,
            token_id: refunded_token_row.token_id,
            wallet_address: Some(wallet_address.into()),
            escrow_contract_address: escrow_contract.address().into(),
        })
        .await?
        .into_result()
        .context("couldn't find deposits in ledger made by this wallet")?;

    /* check this wallet has enough registered deposits for the refund */
    let positive_deposit_balance_of_wallet: U256 =
        positive_deposit_balance_of_wallet_row.balance.into();
    if positive_deposit_balance_of_wallet < quantity {
        bail!("not enough balance for back amount deposited by this wallet")
    }

    let hash = refund_asset_and_ensure_success(
        escrow_contract.clone(),
        &_conn,
        14,
        10,
        Duration::from_secs(10),
        escrow_signer,
        wallet_address,
        token_address,
        quantity,
        logger.clone(),
    )
    .await?;

    let escrow_contract_row = db
        .execute(FunAdminListEscrowContractAddressReq {
            limit: 1,
            offset: 0,
            blockchain: Some(chain),
        })
        .await?
        .into_result()
        .context("could not get escrow contract from database")?;

    /* update ledger for this wallet address and amount */
    db.execute(FunUserRequestRefundReq {
        user_id: ctx.user_id,
        quantity: quantity.into(),
        blockchain: chain,
        user_address: wallet_address.into(),
        receiver_address: wallet_address.into(),
        contract_address: escrow_contract.address().into(),
        transaction_hash: hash.into(),
        token_id: refunded_token_row.token_id,
        contract_address_id: escrow_contract_row.pkey_id,
    })
    .await?
    .into_result()
    .context("could not add entry to deposit withdraw ledger on request refund")?;

    /* update user balance cache */
    let deposit_withdraw_balance_row = db
        .execute(FunUserListUserDepositWithdrawBalanceReq {
            limit: 1,
            offset: 0,
            user_id: ctx.user_id,
            blockchain: Some(chain),
            token_id: Some(refunded_token_row.token_id),
            token_address: Some(token_address.into()),
            escrow_contract_address: Some(escrow_contract.address().into()),
        })
        .await?
        .into_result()
        .context("could not get deposit withdraw balance from database")?;

    let old_balance: U256 = deposit_withdraw_balance_row.balance.into();
    db.execute(FunUserUpdateUserDepositWithdrawBalanceReq {
        deposit_withdraw_balance_id: deposit_withdraw_balance_row.deposit_withdraw_balance_id,
        old_balance: deposit_withdraw_balance_row.balance,
        new_balance: old_balance
            .try_checked_sub(quantity)
            .unwrap_or(U256::zero())
            .into(),
    })
    .await?;

    Ok(hash)
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
pub struct MethodUserGetExpertProfile {
    pub cmc: Arc<CoinMarketCap>,
}
impl RequestHandler for MethodUserGetExpertProfile {
    type Request = UserGetExpertProfileRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        let cmc = self.cmc.clone();
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
                    expert_id: None,
                    expert_public_id: Some(ret.user_public_id),
                    expert_name: None,
                    description: None,
                    blockchain: None,
                    strategy_pool_address: None,
                    approved: Some(true),
                })
                .await?;
            Ok(UserGetExpertProfileResponse {
                expert_id: ret.expert_id,
                name: ret.username,
                family_name: ret.family_name.unwrap_or_default(),
                given_name: ret.given_name.unwrap_or_default(),
                follower_count: ret.follower_count as _,
                backers_count: ret.backer_count as _,
                description: ret.description.unwrap_or_default(),
                social_media: ret.social_media.unwrap_or_default(),
                risk_score: ret.risk_score.unwrap_or_default(),
                aum: ret.aum.unwrap_or_default(),
                reputation_score: ret.reputation_score.unwrap_or_default(),
                strategies_total: strategies.first(|x| x.total).unwrap_or_default(),
                strategies: strategies
                    .map_async(|x| convert_strategy_db_to_api_net_value(x, &cmc, &db))
                    .await?,
                followed: ret.followed,
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
pub struct MethodUserGetUserProfile {
    pub cmc: Arc<CoinMarketCap>,
}
impl RequestHandler for MethodUserGetUserProfile {
    type Request = UserGetUserProfileRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        let cmc = self.cmc.clone();
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
                    .map_async(|x| convert_strategy_db_to_api_net_value(x, &cmc, &db))
                    .await?,
                backed_strategies: backed_strategies
                    .map_async(|x| convert_strategy_db_to_api_net_value(x, &cmc, &db))
                    .await?,
            })
        }
        .boxed()
    }
}
pub struct MethodUserWhitelistWallet;
impl RequestHandler for MethodUserWhitelistWallet {
    type Request = UserWhitelistWalletRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;

            // let signature_text = hex_decode(req.message_to_sign.as_bytes())?;
            // let signature = hex_decode(req.message_signature.as_bytes())?;
            //
            // let verified =
            //     verify_message_address(&signature_text, &signature, req.wallet_address.into())?;

            // ensure!(
            //     verified,
            //     CustomError::new(EnumErrorCode::InvalidPassword, "Signature is not valid")
            // );
            let ret = db
                .execute(FunUserAddWhitelistedWalletReq {
                    user_id: ctx.user_id,
                    blockchain: req.blockchain,
                    address: req.wallet_address.into(),
                })
                .await?
                .into_result()
                .context("failed to register wallet")?;

            Ok(UserWhitelistWalletResponse {
                success: true,
                wallet_id: ret.whitelisted_wallet_id,
            })
        }
        .boxed()
    }
}

pub struct MethodUserListWhitelistedWallets;
impl RequestHandler for MethodUserListWhitelistedWallets {
    type Request = UserListWhitelistedWalletsRequest;

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
                        expert_id: None,
                        expert_public_id: None,
                        expert_name: None,
                        description: None,
                        blockchain: None,
                        limit: 1,
                        offset: 0,
                        strategy_pool_address: None,
                        approved: None,
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
                .execute(FunUserListWhitelistedWalletsReq {
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    user_id: Some(ctx.user_id),
                    blockchain: req.blockchain,
                    address: None,
                })
                .await?;

            Ok(UserListWhitelistedWalletsResponse {
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
pub struct MethodUserUnwhitelistWallet;
impl RequestHandler for MethodUserUnwhitelistWallet {
    type Request = UserUnwhitelistWalletRequest;

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
                .execute(FunUserRemoveWhitelistedWalletReq {
                    whitelisted_wallet_id: req.wallet_id,
                    user_id: ctx.user_id,
                })
                .await?;

            Ok(UserUnwhitelistWalletResponse { success: true })
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
        mut req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        let cmc_client = self.cmc_client.clone();
        async move {
            ensure_user_role(ctx, EnumRole::Expert)?;

            ensure!(
                0.0 <= req.expert_fee && req.expert_fee <= 1.0,
                CustomError::new(
                    EnumErrorCode::InvalidArgument,
                    "Expert fee must be less than 1.0 and greater than 0.0"
                )
            );

            let ret = db
                .execute(FunUserCreateStrategyReq {
                    user_id: ctx.user_id,
                    name: req.name,
                    description: req.description,
                    strategy_thesis_url: req.strategy_thesis_url,
                    minimum_backing_amount_usd: req.minimum_backing_amount_usd.unwrap_or_default(),
                    swap_fee: 0.0,
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
                for token in token_list {
                    db.execute(FunUserAddStrategyWhitelistedTokenReq {
                        strategy_id: ret.strategy_id,
                        token_name: token.symbol,
                    })
                    .await?;
                }
            }
            let usdc = db
                .execute(FunUserListEscrowTokenContractAddressReq {
                    limit: 1,
                    token_id: None,
                    blockchain: Some(req.wallet_blockchain),
                    address: None,
                    symbol: Some(EnumBlockchainCoin::USDC.to_string()),
                    offset: 0,
                    is_stablecoin: None,
                })
                .await?
                .into_result()
                .with_context(|| {
                    CustomError::new(
                        EnumErrorCode::NotFound,
                        format!("token not found: {}", "USDC"),
                    )
                })?;
            // ensure!(
            //     req.initial_tokens.iter().map(|x| x.quantity).sum::<f64>() > 0.into(),
            //     CustomError::new(
            //         EnumErrorCode::InvalidArgument,
            //         "Initial token quantity must be greater than 0"
            //     )
            // );
            // a hack
            if req.initial_tokens.is_empty() {
                let busd = db
                    .execute(FunUserListEscrowTokenContractAddressReq {
                        limit: 1,
                        token_id: None,
                        blockchain: Some(req.wallet_blockchain),
                        address: None,
                        symbol: Some(EnumBlockchainCoin::BUSD.to_string()),
                        offset: 0,
                        is_stablecoin: None,
                    })
                    .await?
                    .into_result()
                    .with_context(|| {
                        CustomError::new(
                            EnumErrorCode::NotFound,
                            format!("token not found: {}", "BUSD"),
                        )
                    })?;
                req.initial_tokens.push(UserCreateStrategyInitialTokenRow {
                    token_id: busd.token_id,
                    quantity: req
                        .strategy_token_relative_to_usdc_ratio
                        .unwrap_or(U256::exp10(18))
                        .into(),
                });
            }
            if req
                .initial_tokens
                .iter()
                .find(|x| x.token_id == usdc.token_id)
                .is_none()
            {
                req.initial_tokens.push(UserCreateStrategyInitialTokenRow {
                    token_id: usdc.token_id,
                    quantity: req
                        .strategy_token_relative_to_usdc_ratio
                        .unwrap_or(U256::exp10(18))
                        .into(),
                });
            }

            for token in req.initial_tokens {
                db.execute(FunUserAddStrategyInitialTokenRatioReq {
                    strategy_id: ret.strategy_id,
                    token_id: token.token_id,
                    quantity: token.quantity.into(),
                })
                .await?;
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
                    expert_id: None,
                    expert_public_id: None,
                    expert_name: None,
                    description: None,
                    blockchain: None,
                    user_id: ctx.user_id,
                    limit: 1,
                    offset: 0,
                    strategy_pool_address: None,
                    approved: None,
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
    pub pool: EthereumRpcConnectionPool,
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
        let pool = self.pool.clone();
        async move {
            ensure_user_role(ctx, EnumRole::Expert)?;

            validate_audit_rule_immutable_tokens(&logger, &db, req.strategy_id).await?;
            let strategy = db
                .execute(FunUserListStrategiesReq {
                    strategy_id: Some(req.strategy_id),
                    strategy_name: None,
                    expert_id: None,
                    expert_public_id: None,
                    expert_name: None,
                    description: None,
                    blockchain: None,
                    user_id: ctx.user_id,
                    limit: 1,
                    offset: 0,
                    strategy_pool_address: None,
                    approved: None,
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

            fetch_and_update_wallet_balances(&db, &pool, req.blockchain, req.wallet_address.into())
                .await?;

            Ok(ExpertAddStrategyWatchingWalletResponse {
                success: ret.success,
                wallet_id: ret.watch_wallet_id,
            })
        }
        .boxed()
    }
}

pub async fn fetch_and_update_wallet_balances(
    db: &DbClient,
    pool: &EthereumRpcConnectionPool,
    chain: EnumBlockChain,
    wallet_address: Address,
) -> Result<()> {
    let conn = pool.get(chain).await?;
    let known_token_contract_rows = db
        .execute(FunAdminListEscrowTokenContractAddressReq {
            limit: None,
            offset: None,
            blockchain: Some(chain),
            token_address: None,
            token_id: None,
        })
        .await?
        .into_rows();

    if known_token_contract_rows.len() == 0 {
        bail!(
            "no known token contracts found in watched wallet chain: {:?}",
            chain
        );
    }

    for known_token_contract in known_token_contract_rows {
        let token_address = known_token_contract.address;
        let token_id = known_token_contract.pkey_id;
        let token_contract = Erc20Token::new(conn.clone(), token_address.into())?;
        let wallet_balance = token_contract.balance_of(wallet_address.into()).await?;

        let wallet_old_balance = db
            .execute(FunWatcherListExpertListenedWalletAssetBalanceReq {
                limit: Some(1),
                offset: None,
                strategy_id: None,
                address: Some(wallet_address.into()),
                blockchain: Some(chain),
                token_id: Some(token_id),
            })
            .await?
            .first(|x| x.balance)
            .unwrap_or_default();

        db.execute(FunWatcherUpsertExpertListenedWalletAssetBalanceReq {
            address: wallet_address.into(),
            blockchain: chain,
            token_id,
            old_balance: wallet_old_balance,
            new_balance: wallet_balance.into(),
        })
        .await?;
    }

    Ok(())
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
                    expert_id: None,
                    expert_public_id: None,
                    expert_name: None,
                    description: None,
                    blockchain: None,
                    user_id: ctx.user_id,
                    limit: 1,
                    offset: 0,
                    strategy_pool_address: None,
                    approved: None,
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
                    token_id: req.token_id,
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
                        linked_wallet: x.linked_wallet.into(),
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
                        linked_wallet: x.linked_wallet.into(),
                        backed_at: x.backed_at,
                        joined_at: x.joined_at,
                    })
                    .collect(),
            })
        }
        .boxed()
    }
}
pub struct MethodUserGetDepositTokens {
    pub coin_addresses: Arc<BlockchainCoinAddresses>,
}
impl RequestHandler for MethodUserGetDepositTokens {
    type Request = UserGetDepositTokensRequest;

    fn handle(
        &self,
        _toolbox: &Toolbox,
        _ctx: RequestContext,
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let tokens = self.coin_addresses.clone();

        async move {
            Ok(UserGetDepositTokensResponse {
                tokens: tokens
                    .iter()
                    .filter(|x| match x.2.as_str() {
                        // filter for stablecoins
                        "USDC" | "USDT" | "BUSD" => true,
                        _ => false,
                    })
                    .map(|(_i, blockchain, token, address)| UserGetDepositTokensRow {
                        blockchain: *blockchain,
                        token: token.clone(),
                        address: *address,
                        short_name: token.to_string(),
                        icon_url: format!(
                            "https://etherscan.io/token/images/centre-{}_28.png",
                            token.to_ascii_lowercase()
                        ),
                        conversion: 1.0, // TODO: register this conversion rate
                    })
                    .collect(),
            })
        }
        .boxed()
    }
}
pub struct MethodUserGetDepositAddresses {
    pub addresses: Arc<EscrowAddresses>,
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
        async move {
            Ok(UserGetDepositAddressesResponse {
                addresses: addresses
                    .iter()
                    .map(|x| UserGetDepositAddressesRow {
                        blockchain: x.1,
                        address: x.3,
                        short_name: x.1.to_string(),
                    })
                    .collect(),
            })
        }
        .boxed()
    }
}
pub struct MethodUserListDepositWithdrawLedger;
impl RequestHandler for MethodUserListDepositWithdrawLedger {
    type Request = UserListDepositWithdrawLedgerRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            let resp = db
                .execute(FunUserListDepositWithdrawLedgerReq {
                    user_id: Some(ctx.user_id),
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    blockchain: req.blockchain,
                    is_deposit: req.id_deposit,
                    is_back: None,
                    is_withdraw: None,
                })
                .await?;
            Ok(UserListDepositWithdrawLedgerResponse {
                ledger_total: resp.first(|x| x.total).unwrap_or_default(),
                ledger: resp
                    .into_iter()
                    .map(|x| UserListDepositLedgerRow {
                        transaction_id: x.transaction_id,
                        blockchain: x.blockchain,
                        user_address: x.user_address.into(),
                        contract_address: x.contract_address.into(),
                        receiver_address: x.receiver_address.into(),
                        quantity: x.quantity.into(),
                        transaction_hash: x.transaction_hash.into(),
                        is_deposit: x.is_deposit,
                        happened_at: x.happened_at,
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
                    .execute(FunUserListDepositWithdrawLedgerReq {
                        user_id: Some(ctx.user_id),
                        limit,
                        offset: 0,
                        blockchain: req.blockchain,
                        is_deposit: Some(true),
                        is_back: None,
                        is_withdraw: None,
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
                                transaction_id: row.transaction_id,
                                quantity: row.quantity.into(),
                                blockchain: row.blockchain,
                                user_address: row.user_address.into(),
                                contract_address: row.contract_address.into(),
                                transaction_hash: row.transaction_hash.into(),
                                receiver_address: row.receiver_address.into(),
                                happened_at: row.happened_at,
                                is_deposit: row.is_deposit,
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
                                transaction_id: 0,
                                quantity: amount.into(),
                                blockchain: req
                                    .blockchain
                                    .unwrap_or(EnumBlockChain::EthereumMainnet),
                                user_address: key.address.clone().into(),
                                contract_address: key.address.clone().into(),
                                transaction_hash: H256::random().into(),
                                receiver_address: key.address.clone().into(),
                                happened_at: Utc::now().timestamp(),
                                is_deposit: false,
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
                    user_id: Some(ctx.user_id),
                    blockchain: req.blockchain,
                    strategy_wallet_address: None,
                })
                .await?;
            Ok(UserListStrategyWalletsResponse {
                wallets_total: resp.first(|x| x.total).unwrap_or_default(),
                wallets: resp
                    .into_iter()
                    .map(|x| UserListStrategyWalletsRow {
                        blockchain: x.blockchain,
                        address: x.address.into(),
                        is_platform_managed: x.is_platform_managed,
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
            if let Some(user_managed_wallet_address) = req.user_managed_wallet_address {
                db.execute(FunUserAddStrategyWalletReq {
                    user_id: ctx.user_id,
                    blockchain: req.blockchain,
                    address: user_managed_wallet_address.into(),
                    is_platform_managed: false,
                })
                .await?;
                Ok(UserCreateStrategyWalletResponse {
                    blockchain: req.blockchain,
                    address: user_managed_wallet_address,
                })
            } else {
                let conn = pool.get(req.blockchain).await?;
                let wallet = db
                    .execute(FunUserListWhitelistedWalletsReq {
                        limit: 1,
                        offset: 0,
                        user_id: Some(ctx.user_id),
                        blockchain: Some(req.blockchain),
                        address: None,
                    })
                    .await?
                    .into_result()
                    .with_context(|| {
                        CustomError::new(
                            EnumErrorCode::NotFound,
                            format!("User has not registered wallet on {:?}", req.blockchain),
                        )
                    })?;
                let strategy_wallet = back_strategy::deploy_wallet_contract(
                    &conn,
                    master_key.clone(),
                    wallet.address.into(),
                    master_key.address(),
                    DynLogger::empty(),
                )
                .await?;

                db.execute(FunUserAddStrategyWalletReq {
                    user_id: ctx.user_id,
                    blockchain: req.blockchain,
                    address: strategy_wallet.address().into(),
                    is_platform_managed: true,
                })
                .await?;
                Ok(UserCreateStrategyWalletResponse {
                    blockchain: req.blockchain,
                    address: strategy_wallet.address().into(),
                })
            }
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
                    expert_id: None,
                    expert_public_id: None,
                    expert_name: None,
                    description: None,
                    blockchain: None,
                    user_id: ctx.user_id,
                    limit: 1,
                    offset: 0,
                    strategy_pool_address: None,
                    approved: None,
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
                    expert_id: None,
                    expert_public_id: None,
                    expert_name: None,
                    description: None,
                    blockchain: None,
                    user_id: ctx.user_id,
                    limit: 1,
                    offset: 0,
                    strategy_pool_address: None,
                    approved: None,
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
    pub addresses: Arc<EscrowAddresses>,
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
                    expert_id: None,
                    expert_public_id: None,
                    expert_name: None,
                    description: None,
                    blockchain: None,
                    strategy_pool_address: None,
                    approved: None,
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
                if let Some(escrow_address) = escrow_addresses.get(strategy.blockchain, ()) {
                    let tk = UserAllowedEscrowTransferInfo {
                        receiver_address: escrow_address,
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

pub struct MethodExpertListBackStrategyLedger;
impl RequestHandler for MethodExpertListBackStrategyLedger {
    type Request = ExpertListBackStrategyLedgerRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Expert)?;
            let ledger = db
                .execute(FunUserListBackStrategyLedgerReq {
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    strategy_id: req.strategy_id,
                    user_id: None,
                })
                .await?;
            Ok(ExpertListBackStrategyLedgerResponse {
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

pub struct MethodUserListExitStrategyLedger;
impl RequestHandler for MethodUserListExitStrategyLedger {
    type Request = UserListExitStrategyLedgerRequest;

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
                .execute(FunUserListExitStrategyLedgerReq {
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    strategy_id: req.strategy_id,
                    user_id: Some(ctx.user_id),
                })
                .await?;
            Ok(UserListExitStrategyLedgerResponse {
                exit_ledger_total: ledger.first(|x| x.total).unwrap_or_default(),
                exit_ledger: ledger.map(|x| ExitStrategyLedgerRow {
                    exit_ledger_id: x.back_ledger_id,
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

pub struct MethodExpertListExitStrategyLedger;
impl RequestHandler for MethodExpertListExitStrategyLedger {
    type Request = ExpertListExitStrategyLedgerRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Expert)?;
            let ledger = db
                .execute(FunUserListExitStrategyLedgerReq {
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    strategy_id: req.strategy_id,
                    user_id: None,
                })
                .await?;
            Ok(ExpertListExitStrategyLedgerResponse {
                exit_ledger_total: ledger.first(|x| x.total).unwrap_or_default(),
                exit_ledger: ledger.map(|x| ExitStrategyLedgerRow {
                    exit_ledger_id: x.back_ledger_id,
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
    pub cmc: Arc<CoinMarketCap>,
    pub pancake_paths: Arc<WorkingPancakePairPaths>,
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
        let pool = self.pool.clone();
        let master_key = self.master_key.clone();
        let cmc = self.cmc.clone();
        let escrow_contract = self.escrow_contract.clone();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
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

            let eth_conn = pool.get(token.blockchain).await?;
            let escrow_contract_address = escrow_contract.get(&pool, token.blockchain).await?;
            let blockchain = token.blockchain;
            let token_address = token.address.0;
            let get_token_out = |out_token: Address, amount: U256| {
                let db = db.clone();
                let cmc = cmc.clone();
                async move {
                    let tk = db
                        .execute(FunUserListEscrowTokenContractAddressReq {
                            limit: 1,
                            offset: 0,
                            token_id: None,
                            blockchain: Some(blockchain),
                            address: Some(out_token.into()),
                            symbol: None,
                            is_stablecoin: None,
                        })
                        .await?
                        .into_result()
                        .with_context(|| {
                            format!("No escrow token found for {:?} {:?}", out_token, blockchain)
                        })?;
                    let price = *cmc
                        .get_usd_prices_by_symbol(&vec![tk.symbol])
                        .await?
                        .first()
                        .with_context(|| {
                            format!("No price found for {:?} {:?}", token_address, blockchain)
                        })?;
                    amount.mul_f64(price)
                }
            };
            let CalculateUserBackStrategyCalculateAmountToMintResult {
                fees,
                back_amount_minus_fees,
                strategy_token_to_mint,
                sp_assets_and_amounts,
                strategy_pool_assets_bought_for_this_backer,
                ..
            } = calculate_user_back_strategy_calculate_amount_to_mint(
                &eth_conn,
                &db,
                token.blockchain,
                req.quantity,
                req.strategy_id,
                req.token_id,
                token.address.into(),
                master_key,
                DynLogger::empty(),
                true,
                ctx.user_id,
                escrow_contract_address.address(),
                get_token_out,
                &cmc,
            )
            .await?;
            let mut ratios: Vec<EstimatedBackedTokenRatios> = vec![];
            for address in strategy_pool_assets_bought_for_this_backer.keys() {
                let before_amount = sp_assets_and_amounts
                    .get(address)
                    .cloned()
                    .unwrap_or(U256::zero());
                let add_amount = strategy_pool_assets_bought_for_this_backer
                    .get(address)
                    .cloned()
                    .unwrap_or(U256::zero());
                let after_amount = before_amount + add_amount;
                let ratio = if after_amount.is_zero() {
                    0.0
                } else {
                    add_amount.div_as_f64(after_amount)?
                };
                let token = db
                    .execute(FunUserListEscrowTokenContractAddressReq {
                        limit: 1,
                        offset: 0,
                        token_id: None,
                        blockchain: Some(token.blockchain),
                        address: Some((*address).into()),
                        symbol: None,
                        is_stablecoin: None,
                    })
                    .await?
                    .into_result()
                    .with_context(|| {
                        CustomError::new(EnumErrorCode::NotFound, "Token not found")
                    })?;
                let price = cmc
                    .get_usd_prices_by_symbol(&[token.symbol.clone()])
                    .await?[0];
                ratios.push(EstimatedBackedTokenRatios {
                    token_id: token.token_id,
                    token_name: token.short_name,
                    back_amount: add_amount.into(),
                    back_value_in_usd: add_amount.mul_f64(price)?,
                    back_value_ratio: ratio,
                });
            }
            let wallets = db
                .execute(FunUserListStrategyWalletsReq {
                    user_id: Some(ctx.user_id),
                    blockchain: Some(token.blockchain),
                    strategy_wallet_address: None,
                })
                .await?;
            let balances = db
                .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
                    strategy_pool_contract_id: None,
                    strategy_id: Some(req.strategy_id),
                    blockchain: None,
                    token_address: None,
                })
                .await?;
            let token_symbols: Vec<_> = balances.iter().map(|x| x.token_symbol.clone()).collect();
            let prices = cmc
                .get_usd_prices_by_symbol(&token_symbols)
                .await
                .context("failed to get price")?;
            Ok(UserGetBackStrategyReviewDetailResponse {
                strategy_fee: fees,
                total_amount_to_back: req.quantity,
                total_amount_to_back_after_fee: back_amount_minus_fees,
                user_strategy_wallets: wallets.map(|x| UserStrategyWallet {
                    address: x.address.into(),
                    wallet_id: x.wallet_id,
                    blockchain: x.blockchain,
                    is_platform_address: x.is_platform_managed,
                }),
                estimated_amount_of_strategy_tokens: strategy_token_to_mint,
                estimated_backed_token_ratios: ratios,
                strategy_pool_asset_balances: balances
                    .map_async(|x| {
                        let cmc = &cmc;
                        let token_symbols = &token_symbols;
                        let prices = &prices;
                        async move {
                            let price_usd = token_symbols
                                .iter()
                                .zip(prices.iter())
                                .find(|(k, _v)| k.as_str() == x.token_symbol.as_str())
                                .map(|y| *y.1)
                                .unwrap_or_default();
                            let price_usd_7d = cmc
                                .get_usd_price_days_ago(x.token_symbol.clone(), 7)
                                .await?;
                            let price_usd_30d = cmc
                                .get_usd_price_days_ago(x.token_symbol.clone(), 30)
                                .await?;
                            Ok(StrategyPoolAssetBalancesRow {
                                name: x.token_name,
                                symbol: x.token_symbol,
                                address: x.token_address.into(),
                                blockchain: x.blockchain,
                                balance: x.balance.into(),
                                price_usd,
                                price_usd_7d,
                                price_usd_30d,
                            })
                        }
                    })
                    .await?,
            })
        }
        .boxed()
    }
}

pub struct MethodUserListUserBackStrategyAttempt;
impl RequestHandler for MethodUserListUserBackStrategyAttempt {
    type Request = UserListUserBackStrategyAttemptRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db = toolbox.get_db();
        async move {
            let attempts = db
                .execute(FunUserListUserBackStrategyAttemptReq {
                    user_id: Some(ctx.user_id),
                    strategy_id: None,
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    token_id: None,
                })
                .await?;
            Ok(UserListUserBackStrategyAttemptResponse {
                total: attempts.first(|x| x.total).unwrap_or_default(),
                back_attempts: attempts.map(|x| UserBackStrategyAttempt {
                    attempt_id: x.user_back_strategy_attempt_id,
                    strategy_id: x.strategy_id,
                    strategy_name: x.strategy_name,
                    token_id: x.token_id,
                    token_symbol: x.token_symbol.clone(),
                    token_name: x.token_symbol,
                    quantity: x.back_quantity.into(),
                    happened_at: x.happened_at,
                }),
            })
        }
        .boxed()
    }
}
pub struct MethodUserListUserBackStrategyLog;
impl RequestHandler for MethodUserListUserBackStrategyLog {
    type Request = UserListUserBackStrategyLogRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let logs = db
                .execute(FunUserListUserBackStrategyLogReq {
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    user_back_strategy_attempt_id: req.attempt_id,
                })
                .await?;
            Ok(UserListUserBackStrategyLogResponse {
                back_logs_total: logs.first(|x| x.total).unwrap_or_default(),
                back_logs: logs.map(|x| UserBackStrategyLog {
                    pkey_id: x.log_entry_id,
                    happened_at: x.happened_at,
                    message: x.message,
                }),
            })
        }
        .boxed()
    }
}

pub struct MethodUserGetSystemConfig;
impl RequestHandler for MethodUserGetSystemConfig {
    type Request = UserGetSystemConfigRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let config = db
                .execute(FunAdminGetSystemConfigReq { config_id: 0 })
                .await?
                .into_result();
            Ok(UserGetSystemConfigResponse {
                platform_fee: config.map(|x| x.platform_fee).flatten().unwrap_or_default(),
            })
        }
        .boxed()
    }
}

pub struct MethodExpertListPublishedStrategies {
    pub cmc: Arc<CoinMarketCap>,
}
impl RequestHandler for MethodExpertListPublishedStrategies {
    type Request = ExpertListPublishedStrategiesRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db = toolbox.get_db();
        let cmc = self.cmc.clone();
        async move {
            ensure_user_role(ctx, EnumRole::Expert)?;
            let strategies = db
                .execute(FunUserListStrategiesReq {
                    user_id: ctx.user_id,
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    strategy_id: None,
                    strategy_name: None,
                    expert_id: Some(ctx.user_id),
                    expert_public_id: None,
                    expert_name: None,
                    description: None,
                    blockchain: None,
                    strategy_pool_address: None,
                    approved: Some(true),
                })
                .await?;
            Ok(ExpertListPublishedStrategiesResponse {
                strategies_total: strategies.first(|x| x.total).unwrap_or_default(),
                strategies: strategies
                    .map_async(|x| convert_strategy_db_to_api_net_value(x, &cmc, &db))
                    .await?,
            })
        }
        .boxed()
    }
}
pub struct MethodExpertListUnpublishedStrategies {
    pub cmc: Arc<CoinMarketCap>,
}

impl RequestHandler for MethodExpertListUnpublishedStrategies {
    type Request = ExpertListUnpublishedStrategiesRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db = toolbox.get_db();
        let cmc = self.cmc.clone();
        async move {
            ensure_user_role(ctx, EnumRole::Expert)?;
            let strategies = db
                .execute(FunUserListStrategiesReq {
                    user_id: ctx.user_id,
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    strategy_id: None,
                    strategy_name: None,
                    expert_id: Some(ctx.user_id),
                    expert_public_id: None,
                    expert_name: None,
                    description: None,
                    blockchain: None,
                    strategy_pool_address: None,
                    approved: Some(false),
                })
                .await?;
            Ok(ExpertListUnpublishedStrategiesResponse {
                strategies_total: strategies.first(|x| x.total).unwrap_or_default(),
                strategies: strategies
                    .map_async(|x| convert_strategy_db_to_api_net_value(x, &cmc, &db))
                    .await?,
            })
        }
        .boxed()
    }
}
pub struct MethodUserListUserStrategyBalance;
impl RequestHandler for MethodUserListUserStrategyBalance {
    type Request = UserListUserStrategyBalanceRequest;

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
                .execute(FunUserListUserStrategyBalanceReq {
                    user_id: ctx.user_id,
                    strategy_id: req.strategy_id,
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                })
                .await?;
            Ok(UserListUserStrategyBalanceResponse {
                balances_total: balances.first(|x| x.total).unwrap_or_default(),
                balances: balances.map(|x| UserStrategyBalance {
                    strategy_id: x.strategy_id,
                    strategy_name: x.strategy_name,
                    balance: x.balance.into(),
                    address: x.user_strategy_wallet_address.into(),
                    blockchain: x.blockchain,
                }),
            })
        }
        .boxed()
    }
}
