use eth_sdk::erc20::Erc20Token;
use eth_sdk::escrow::{AbstractEscrowContract, EscrowContract};
use eth_sdk::signer::Secp256k1SecretKey;
use eth_sdk::strategy_pool::StrategyPoolContract;
use eth_sdk::utils::{verify_message_address, wait_for_confirmations_simple};
use eth_sdk::v3::smart_router::PancakeSmartRouterV3Contract;
use eth_sdk::*;
use eyre::*;
use futures::FutureExt;
use futures::StreamExt;
use gen::database::*;
use gen::model::*;
use lib::database::DbClient;
use lib::handler::{FutureResponse, RequestHandler};
use lib::toolbox::*;
use lib::utils::hex_decode;
use num_traits::cast::FromPrimitive;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tracing::info;
use web3::signing::Key;
use web3::types::{Address, H256, U256};

pub fn initial_sp_token_supply() -> U256 {
    U256::from(1000000000u64) * U256::exp10(18)
}

pub fn ensure_user_role(ctx: RequestContext, role: EnumRole) -> Result<()> {
    let ctx_role = EnumRole::from_u32(ctx.role).context("Invalid role")?;
    ensure!(
        ctx_role >= role,
        CustomError::new(
            EnumErrorCode::InvalidRole,
            format!("Requires {} Actual {}", role, ctx_role)
        )
    );
    Ok(())
}
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
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;

            let ret = db
                .execute(FunUserListFollowedStrategiesReq {
                    user_id: ctx.user_id,
                })
                .await?;
            Ok(UserListFollowedStrategiesResponse {
                strategies: ret
                    .into_iter()
                    .map(|x| ListStrategiesRow {
                        strategy_id: x.strategy_id,
                        strategy_name: x.strategy_name,
                        strategy_description: x.strategy_description,
                        net_value: x.net_value,
                        followers: x.followers as _,
                        backers: x.backers as _,
                        risk_score: x.risk_score,
                        aum: x.aum,
                        followed: x.followed,
                        swap_price: 233.0,
                        price_change: 0.97,
                        wallet_address: "0x000000000".to_owned(),
                        blockchain: EnumBlockChain::EthereumMainnet,
                    })
                    .collect(),
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
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;

            let ret = db.execute(FunUserListStrategiesReq {}).await?;
            Ok(UserListStrategiesResponse {
                strategies: ret
                    .into_iter()
                    .map(|x| ListStrategiesRow {
                        strategy_id: x.strategy_id,
                        strategy_name: x.strategy_name,
                        strategy_description: x.strategy_description,
                        net_value: x.net_value,
                        followers: x.followers as _,
                        backers: x.backers as _,
                        risk_score: x.risk_score,
                        aum: x.aum,
                        followed: x.followed,
                        swap_price: 233.0,
                        price_change: 0.97,
                        wallet_address: "0x000000000".to_owned(),
                        blockchain: EnumBlockChain::EthereumMainnet,
                    })
                    .collect(),
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
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;

            let ret = db.execute(FunUserListStrategiesReq {}).await?;
            Ok(UserListTopPerformingStrategiesResponse {
                strategies: ret
                    .into_iter()
                    .map(|x| ListStrategiesRow {
                        strategy_id: x.strategy_id,
                        strategy_name: x.strategy_name,
                        strategy_description: x.strategy_description,
                        net_value: x.net_value,
                        followers: x.followers as _,
                        backers: x.backers as _,
                        risk_score: x.risk_score,
                        aum: x.aum,
                        followed: x.followed,
                        swap_price: 233.0,
                        price_change: 0.97,
                        wallet_address: "0x000000000".to_owned(),
                        blockchain: EnumBlockChain::EthereumMainnet,
                    })
                    .collect(),
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
                followers: ret
                    .into_iter()
                    .map(|x| ListStrategyFollowersRow {
                        user_id: x.user_public_id,
                        name: x.username,
                        linked_wallet: x.wallet_address,
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
                backers: ret
                    .into_iter()
                    .map(|x| ListStrategyBackersRow {
                        user_id: x.user_public_id,
                        name: x.username,
                        linked_wallet: x.wallet_address,
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
                net_value: 0.0,
                followers: ret.followers as _,
                backers: ret.backers as _,
                watching_wallets: watching_wallets
                    .into_iter()
                    .map(|x| WatchingWalletRow {
                        watching_wallet_id: x.watch_wallet_id,
                        wallet_address: x.wallet_address,
                        blockchain: x.blockchain,
                        dex: "DEX TODO".to_string(),
                        ratio_distribution: x.ratio,
                    })
                    .collect(),
                risk_score: ret.risk_score.unwrap_or(0.0),
                aum: ret.aum.unwrap_or(0.0),
                reputation: 0,
                aum_history: vec![],
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
                    .into_iter()
                    .map(|x| NetValuePoint {
                        time: x.time,
                        net_value: x.net_value,
                    })
                    .collect(),
                follow_history: follow_hist
                    .into_iter()
                    .map(|x| FollowHistoryPoint {
                        time: x.time,
                        follower_count: x.follower_count,
                    })
                    .collect(),
                back_history: back_hist
                    .into_iter()
                    .map(|x| BackHistoryPoint {
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
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
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
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let ret = db
                .execute(FunUserListBackedStrategiesReq {
                    user_id: ctx.user_id,
                })
                .await?;
            Ok(UserListBackedStrategiesResponse {
                strategies: ret
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
                        followed: true,
                        swap_price: 233.0,
                        price_change: 0.97,
                        wallet_address: "0x000000000".to_owned(),
                        blockchain: EnumBlockChain::EthereumMainnet,
                    })
                    .collect(),
            })
        }
        .boxed()
    }
}
pub async fn calculate_sp_tokens(back_usdc: U256, total_usdc: U256) -> U256 {
    let sp_tokens = back_usdc * initial_sp_token_supply() / total_usdc;
    sp_tokens
}

pub async fn deploy_strategy_contract(
    conn: &EthereumRpcConnection,
    key: impl Key,
    strategy_token_name: String,
    strategy_token_symbol: String,
) -> Result<StrategyPoolContract<EitherTransport>> {
    info!("Deploying strategy contract");

    let strategy = StrategyPoolContract::deploy(
        conn.clone(),
        key,
        strategy_token_name,
        strategy_token_symbol,
    )
    .await?;

    info!("Deploy strategy contract success");
    Ok(strategy)
}
/*
1. User will decides which strategy S to back with his wallet address A
2. Backend will save his backing decision in database, and transfer his USDC to strategy for copy trading(in this step it may involve auto token conversion)

 */

pub async fn transfer_token_to_strategy_contract(
    conn: &EthereumRpcConnection,
    signer: impl Key,
    escrow: EscrowTransfer,
    chain: EnumBlockChain,
    stablecoin_addresses: &BlockchainCoinAddresses,
    escrow_contract: &EscrowContract<EitherTransport>,
) -> Result<TransactionFetcher> {
    let token_address = stablecoin_addresses
        .get(chain, escrow.token)
        .context("Could not find stablecoin address")?;

    let tx_hash = escrow_contract
        .transfer_token_to(signer, token_address, escrow.recipient, escrow.amount)
        .await?;

    let tx = TransactionFetcher::new(tx_hash);
    Ok(tx)
}
pub async fn user_back_strategy(
    conn: &EthereumRpcConnection,
    ctx: &RequestContext,
    db: &DbClient,
    chain: EnumBlockChain,
    back_usdc_amount: U256,
    stablecoin_addresses: &BlockchainCoinAddresses,
    strategy_id: i64,
    strategy_pool_signer: impl Key,
    escrow_signer: impl Key,
    stablecoin: EnumBlockchainCoin,
    escrow_contract: EscrowContract<EitherTransport>,
    externally_owned_account: impl Key + Clone,
    dex_addresses: &DexAddresses,
) -> Result<()> {
    let mut strategy = db
        .execute(FunUserGetStrategyReq { strategy_id })
        .await?
        .into_result()
        .context("user_registered_strategy")?;
    if strategy.evm_contract_address.is_none() {
        let contract = deploy_strategy_contract(
            &conn,
            strategy_pool_signer,
            strategy.strategy_name.clone(),
            strategy.strategy_name, // strategy symbol
        )
        .await?;
        strategy.evm_contract_address = Some(format!("{:?}", contract.address()));
    }
    let sp_tokens = calculate_sp_tokens(back_usdc_amount, strategy.current_usdc.parse()?).await;
    let strategy_address: Address = strategy.evm_contract_address.unwrap().parse()?;

    let escrow_signer_address = escrow_signer.address();
    // we need to trade, not transfer, and then we need to call deposit on the strategy contract
    let transaction = transfer_token_to_strategy_contract(
        conn,
        escrow_signer,
        EscrowTransfer {
            token: stablecoin,
            // TODO: reduce fees from back_usdc_amount
            amount: back_usdc_amount,
            // recipient: strategy_address,
            recipient: externally_owned_account.address(),
            owner: escrow_signer_address,
        },
        chain,
        stablecoin_addresses,
        &escrow_contract,
    )
    .await?;
    let erc20_address = stablecoin_addresses
        .get(chain, stablecoin)
        .context("Could not find stablecoin address")?;
    let erc20 = Erc20Token::new(conn.clone(), erc20_address)?;
    let hash = erc20
        .transfer(
            externally_owned_account.clone(),
            strategy_address,
            sp_tokens,
        )
        .await?;
    wait_for_confirmations_simple(&conn.eth(), hash, Duration::from_secs(3), 5).await?;
    // TODO: process retry here or have a lock
    let ret = db
        .execute(FunUserBackStrategyReq {
            user_id: ctx.user_id,
            strategy_id: strategy.strategy_id,
            quantity: format!("{:?}", back_usdc_amount),
            new_total_backed_quantity: format!(
                "{:?}",
                strategy.total_backed_usdc.parse::<U256>()? + back_usdc_amount
            ),
            old_total_backed_quantity: strategy.total_backed_usdc,
            new_current_quantity: format!(
                "{:?}",
                strategy.current_usdc.parse::<U256>()? + back_usdc_amount
            ),
            old_current_quantity: strategy.current_usdc,
            blockchain: chain,
            transaction_hash: format!("{:?}", transaction.get_hash()),
            earn_sp_tokens: format!("{:?}", sp_tokens),
        })
        .await?
        .into_result()
        .context("No record")?;
    if !ret.success {
        bail!(
            "User back strategy not sucessful due to other clients updated record at the same time"
        )
    }

    info!(
        "Transfer token to strategy contract {:?}",
        transaction.get_hash()
    );

    let _tx = TransactionFetcher::new_and_assume_ready(transaction.get_hash(), conn).await?;

    let initial_tokens = db
        .execute(FunUserListStrategyInitialTokenRatiosReq { strategy_id })
        .await?
        .into_rows();
    trade_usdc_to_tokens_on_pancakeswap(
        conn,
        chain,
        externally_owned_account,
        &dex_addresses,
        initial_tokens,
        // TODO: check if there's overflow or precision loss
        back_usdc_amount.as_u128() as f64,
    )
    .await?;
    Ok(())
}
pub async fn trade_usdc_to_tokens_on_pancakeswap(
    conn: &EthereumRpcConnection,
    chain: EnumBlockChain,
    signer: impl Key + Clone,
    dex_addresses: &DexAddresses,
    tokens: Vec<FunUserListStrategyInitialTokenRatiosRespRow>,
    backed_usdc: f64,
) -> Result<Vec<TransactionFetcher>> {
    let token_address = dex_addresses
        .get(chain, EnumDex::PancakeSwap)
        .context("Could not find stablecoin address")?;
    let pancake_swap = PancakeSmartRouterV3Contract::new(conn.clone(), token_address)?;
    let receipt = signer.address();
    let mut txs = vec![];
    let mut total_value_usdc = 0.0;
    for token in &tokens {
        let market_price: f64 = todo!();
        total_value_usdc += token.quantity.parse::<f64>()? * market_price;
    }
    for token in tokens {
        let market_price: f64 = todo!();
        let should_buy =
            backed_usdc * token.quantity.parse::<f64>()? * market_price / total_value_usdc;
        let path = todo!();
        let tx_hash = pancake_swap
            .swap_exact_tokens_for_tokens(
                signer.clone(),
                receipt,
                (should_buy as u128).into(),
                ((should_buy * market_price * 0.99) as u128).into(),
                path,
            )
            .await?;

        let tx = TransactionFetcher::new(tx_hash);
        txs.push(tx);
    }
    Ok(txs)
}
pub struct MethodUserBackStrategy {
    pub pool: EthereumRpcConnectionPool,
    pub stablecoin_addresses: Arc<BlockchainCoinAddresses>,
    pub strategy_pool_signer: Secp256k1SecretKey,
    pub escrow_contract: Arc<AbstractEscrowContract>,
    pub escrow_signer: Secp256k1SecretKey,
    pub externally_owned_account: Secp256k1SecretKey,
    pub dex_addresses: Arc<DexAddresses>,
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
        let pool = self.pool.clone();
        let stablecoin_addresses = self.stablecoin_addresses.clone();
        let strategy_pool_signer = self.strategy_pool_signer.clone();
        let escrow_signer = self.escrow_signer.clone();
        let escrow_contract = self.escrow_contract.clone();
        let externally_owned_account = self.externally_owned_account.clone();
        let dex_addresses = self.dex_addresses.clone();
        async move {
            let escrow_contract = escrow_contract.get(&pool, req.blockchain).await?;
            let eth_conn = pool.get(EnumBlockChain::LocalNet).await?;
            ensure_user_role(ctx, EnumRole::User)?;

            user_back_strategy(
                &eth_conn,
                &ctx,
                &db,
                req.blockchain,
                req.quantity.parse()?,
                &stablecoin_addresses,
                req.strategy_id,
                &*strategy_pool_signer,
                &*escrow_signer,
                EnumBlockchainCoin::USDC,
                escrow_contract,
                &*externally_owned_account,
                &dex_addresses,
            )
            .await?;
            Ok(UserBackStrategyResponse { success: true })
        }
        .boxed()
    }
}
pub struct MethodUserRequestRefund {
    pub pool: EthereumRpcConnectionPool,
    pub stablecoin_addresses: Arc<BlockchainCoinAddresses>,
    pub escrow_contract: Arc<AbstractEscrowContract>,
    pub escrow_signer: Secp256k1SecretKey,
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
        let escrow_signer = self.escrow_signer.clone();
        let escrow_contract = self.escrow_contract.clone();
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
                &escrow_contract,
                req.quantity.parse()?,
                req.wallet_address.parse()?,
                &escrow_signer.key,
                EnumBlockchainCoin::USDC,
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

pub struct MethodUserListExitStrategyHistory;
impl RequestHandler for MethodUserListExitStrategyHistory {
    type Request = UserListExitStrategyHistoryRequest;

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
                .execute(FunUserListExitStrategyHistoryReq {
                    user_id: ctx.user_id,
                    strategy_id: None,
                })
                .await?;
            Ok(UserListExitStrategyHistoryResponse {
                exit_history: ret
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
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let ret = db
                .execute(FunUserListFollowedExpertsReq {
                    user_id: ctx.user_id,
                })
                .await?;
            Ok(UserListFollowedExpertsResponse {
                experts: ret
                    .into_iter()
                    .map(|x| ListExpertsRow {
                        expert_id: x.expert_id,
                        user_public_id: x.user_public_id,
                        name: x.username,
                        linked_wallet: x.listening_wallet,
                        family_name: x.family_name.unwrap_or_default(),
                        given_name: x.given_name.unwrap_or_default(),
                        follower_count: x.follower_count as _,
                        description: x.description,
                        social_media: x.social_media,
                        risk_score: x.risk_score,
                        reputation_score: x.reputation_score,
                        aum: x.aum,
                        joined_at: x.joined_at,
                        requested_at: x.requested_at,
                    })
                    .collect(),
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
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let ret = db.execute(FunUserListExpertsReq {}).await?;
            Ok(UserListExpertsResponse {
                experts: ret
                    .into_iter()
                    .map(|x| ListExpertsRow {
                        expert_id: x.expert_id,
                        user_public_id: x.user_public_id,
                        name: x.username,
                        linked_wallet: x.listening_wallet,
                        family_name: x.family_name.unwrap_or_default(),
                        given_name: x.given_name.unwrap_or_default(),
                        follower_count: x.follower_count as _,
                        description: x.description,
                        social_media: x.social_media,
                        risk_score: x.risk_score,
                        aum: x.aum,
                        joined_at: x.joined_at,
                        reputation_score: x.reputation_score,
                        requested_at: x.requested_at,
                    })
                    .collect(),
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
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let ret = db.execute(FunUserListExpertsReq {}).await?;
            Ok(UserListTopPerformingExpertsResponse {
                experts: ret
                    .into_iter()
                    .map(|x| ListExpertsRow {
                        expert_id: x.expert_id,
                        user_public_id: x.user_public_id,
                        name: x.username,
                        linked_wallet: x.listening_wallet,
                        family_name: x.family_name.unwrap_or_default(),
                        given_name: x.given_name.unwrap_or_default(),
                        follower_count: x.follower_count as _,
                        description: x.description,
                        social_media: x.social_media,
                        risk_score: x.risk_score,
                        aum: x.aum,
                        joined_at: x.joined_at,
                        reputation_score: x.reputation_score,
                        requested_at: x.requested_at,
                    })
                    .collect(),
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
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let ret = db.execute(FunUserListExpertsReq {}).await?;
            Ok(UserListFeaturedExpertsResponse {
                experts: ret
                    .into_iter()
                    .map(|x| ListExpertsRow {
                        expert_id: x.expert_id,
                        user_public_id: x.user_public_id,
                        name: x.username,
                        linked_wallet: x.listening_wallet,
                        family_name: x.family_name.unwrap_or_default(),
                        given_name: x.given_name.unwrap_or_default(),
                        follower_count: x.follower_count as _,
                        description: x.description,
                        social_media: x.social_media,
                        risk_score: x.risk_score,
                        aum: x.aum,
                        joined_at: x.joined_at,
                        reputation_score: x.reputation_score,
                        requested_at: x.requested_at,
                    })
                    .collect(),
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
                    expert_id: Some(req.expert_id),
                    user_id: None,
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
        }
        .boxed()
    }
}
pub struct MethodUserUpdateExpertProfile;
impl RequestHandler for MethodUserUpdateExpertProfile {
    type Request = UserUpdateExpertProfileRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let expert = db
                .execute(FunUserGetExpertProfileReq {
                    expert_id: None,
                    user_id: Some(ctx.user_id),
                })
                .await?
                .into_result();
            match expert {
                Some(expert_id) => {
                    let ret = db
                        .execute(FunUserUpdateExpertProfileReq {
                            expert_id: expert_id.expert_id,
                            description: req.description,
                            social_media: req.social_media,
                        })
                        .await?
                        .into_result()
                        .context("failed to get expert profile")?;
                }
                None => {
                    let ret = db
                        .execute(FunUserCreateExpertProfileReq {
                            user_id: ctx.user_id,
                            description: req.description,
                            social_media: req.social_media,
                        })
                        .await?
                        .into_result()
                        .context("failed to get expert profile")?;
                }
            }
            Ok(UserUpdateExpertProfileResponse {})
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
        async move {
            ensure_user_role(ctx, EnumRole::User)?;
            let expert = db
                .execute(FunUserGetExpertProfileReq {
                    expert_id: None,
                    user_id: Some(ctx.user_id),
                })
                .await?
                .into_result();
            match expert {
                Some(expert_id) => {
                    let ret = db
                        .execute(FunUserUpdateExpertProfileReq {
                            expert_id: expert_id.expert_id,
                            description: req.description,
                            social_media: req.social_media,
                        })
                        .await?
                        .into_result()
                        .context("failed to get expert profile")?;
                }
                None => {
                    let ret = db
                        .execute(FunUserCreateExpertProfileReq {
                            user_id: ctx.user_id,
                            description: req.description,
                            social_media: req.social_media,
                        })
                        .await?
                        .into_result()
                        .context("failed to get expert profile")?;
                }
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
                .execute(FunUserGetExpertProfileReq {
                    expert_id: None,
                    user_id: Some(ctx.user_id),
                })
                .await?
                .into_result()
                .context("failed to get user profile")?;
            // TODO: get followed experts, followed strategies, backed strategies
            Ok(UserGetUserProfileResponse {
                user_id: ctx.user_id,
                name: ret.name,
                follower_count: ret.follower_count,
                description: ret.description,
                social_media: ret.social_media,
                followed_experts: vec![],
                followed_strategies: vec![],
                backed_strategies: vec![],
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
            let address = Address::from_str(&req.wallet_address).map_err(|x| {
                CustomError::new(
                    EnumErrorCode::UnknownUser,
                    format!("Invalid address: {}", x),
                )
            })?;

            let signature_text = hex_decode(req.message_to_sign.as_bytes())?;
            let signature = hex_decode(req.message_signature.as_bytes())?;

            let verified = verify_message_address(&signature_text, &signature, address)?;

            ensure!(
                verified,
                CustomError::new(EnumErrorCode::InvalidPassword, "Signature is not valid")
            );
            let ret = db
                .execute(FunUserAddRegisteredWalletReq {
                    user_id: ctx.user_id,
                    blockchain: req.blockchain,
                    address: req.wallet_address,
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
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::User)?;

            let ret = db
                .execute(FunUserListRegisteredWalletsReq {
                    user_id: ctx.user_id,
                })
                .await?;

            Ok(UserListRegisteredWalletsResponse {
                wallets: ret
                    .into_iter()
                    .map(|x| ListWalletsRow {
                        wallet_id: x.registered_wallet_id,
                        blockchain: x.blockchain,
                        wallet_address: x.address,
                        is_default: false,
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
        }
        .boxed()
    }
}
pub struct MethodUserCreateStrategy;

impl RequestHandler for MethodUserCreateStrategy {
    type Request = UserCreateStrategyRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            // TODO: check if user is expert
            ensure_user_role(ctx, EnumRole::User)?;

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
        }
        .boxed()
    }
}
pub struct MethodUserUpdateStrategy;
impl RequestHandler for MethodUserUpdateStrategy {
    type Request = UserUpdateStrategyRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            // TODO: check if user is expert

            ensure_user_role(ctx, EnumRole::User)?;

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
        }
        .boxed()
    }
}
// pub struct MethodUserDeleteStrategy;
pub struct MethodUserAddStrategyWatchingWallet;
impl RequestHandler for MethodUserAddStrategyWatchingWallet {
    type Request = UserAddStrategyWatchingWalletRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            // TODO: check if user is expert

            ensure_user_role(ctx, EnumRole::User)?;

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
        }
        .boxed()
    }
}
pub struct MethodUserRemoveStrategyWatchingWallet;
impl RequestHandler for MethodUserRemoveStrategyWatchingWallet {
    type Request = UserRemoveStrategyWatchingWalletRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();

        async move {
            // TODO: check if user is expert

            ensure_user_role(ctx, EnumRole::User)?;

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
        }
        .boxed()
    }
}

pub struct MethodUserListWalletActivityHistory;

impl RequestHandler for MethodUserListWalletActivityHistory {
    type Request = UserListWalletActivityHistoryRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,

        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();

        async move {
            let ret = db
                .execute(FunWatcherListWalletActivityHistoryReq {
                    address: req.wallet_address,
                    blockchain: req.blockchain,
                })
                .await?;

            Ok(UserListWalletActivityHistoryResponse {
                wallet_activities: ret
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
    escrow_contract: &EscrowContract<EitherTransport>,
    quantity: U256,
    wallet_address: Address,
    escrow_signer: impl Key,
    token: EnumBlockchainCoin,
) -> Result<H256> {
    info!(
        "on_user_request_refund {:?} from {:?} transfer {:?} {:?} to {:?}",
        chain,
        escrow_contract.address(),
        quantity,
        token,
        wallet_address
    );
    let row = db
        .execute(FunUserRequestRefundReq {
            user_id: ctx.user_id,
            quantity: format!("{:?}", quantity),
            blockchain: chain,
            wallet_address: format!("{:?}", wallet_address),
        })
        .await?
        .into_result()
        .context("No result")?;
    let token_address = stablecoin_addresses
        .get(chain, token)
        .context("no stablecoin address")?;

    let hash = escrow_contract
        .transfer_token_to(escrow_signer, token_address, wallet_address, quantity)
        .await?;

    db.execute(FunUserUpdateRequestRefundHistoryReq {
        request_refund_id: row.request_refund_id,
        transaction_hash: format!("{:?}", hash),
    })
    .await?;
    // TODO: do we wait until confirmation here?
    Ok(hash)
}

pub struct MethodUserAddStrategyInitialTokenRatio;
impl RequestHandler for MethodUserAddStrategyInitialTokenRatio {
    type Request = UserAddStrategyInitialTokenRatioRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();

        async move {
            ensure_user_role(ctx, EnumRole::Expert)?;
            // TODO: verify strategy belongs to user
            let ret = db
                .execute(FunUserAddStrategyInitialTokenRatioReq {
                    strategy_id: req.strategy_id,
                    token_name: req.token_name,
                    token_address: req.token_address,
                    quantity: req.quantity,
                })
                .await?
                .into_result()
                .context("failed to add strategy initial token ratio")?;

            Ok(UserAddStrategyInitialTokenRatioResponse {
                success: true,
                token_id: ret.strategy_initial_token_ratio_id,
            })
        }
        .boxed()
    }
}
pub struct MethodUserRemoveStrategyInitialTokenRatio;
impl RequestHandler for MethodUserRemoveStrategyInitialTokenRatio {
    type Request = UserRemoveStrategyInitialTokenRatioRequest;

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
                .execute(FunUserRemoveStrategyInitialTokenRatioReq {
                    strategy_initial_token_ratio_id: req.token_id,
                    strategy_id: req.strategy_id,
                })
                .await?
                .into_result()
                .context("failed to remove strategy initial token ratio")?;

            Ok(UserRemoveStrategyInitialTokenRatioResponse { success: true })
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
                })
                .await?;

            Ok(UserListStrategyInitialTokenRatioResponse {
                token_ratios: ret
                    .into_iter()
                    .map(|x| ListStrategyInitialTokenRatioRow {
                        token_id: x.strategy_initial_token_ratio_id,
                        token_name: x.token_name,
                        token_address: x.token_address,
                        quantity: x.quantity,
                        updated_at: x.updated_at,
                        created_at: x.created_at,
                    })
                    .collect(),
            })
        }
        .boxed()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use eth_sdk::escrow_tracker::escrow::parse_escrow;
    use eth_sdk::mock_erc20::deploy_mock_erc20;
    use eth_sdk::signer::Secp256k1SecretKey;
    use lib::database::{
        connect_to_database, database_test_config, drop_and_recreate_database, DatabaseConfig,
    };
    use lib::log::{setup_logs, LogLevel};
    use std::net::Ipv4Addr;
    use std::{format, vec};

    /*
    1. He will transfer tokens C of USDC to escrow address B
    2. We track his transfer, calculate how much SP token user will have, and save the "deposit" information to database (this is for multi chain support)
    */
    pub async fn on_user_deposit(
        _conn: &EthereumRpcConnection,
        ctx: &RequestContext,
        db: &DbClient,
        chain: EnumBlockChain,
        tx: &TransactionReady,
        stablecoin_addresses: &BlockchainCoinAddresses,
        erc_20: &web3::ethabi::Contract,
        escrow_contract: &EscrowContract<EitherTransport>,
    ) -> Result<()> {
        let esc = parse_escrow(chain, tx, stablecoin_addresses, erc_20)?;

        let our_valid_address = esc.recipient == escrow_contract.address();
        ensure!(
            our_valid_address,
            "is not our valid address {:?}",
            esc.recipient
        );

        // USER just deposits to our service
        db.execute(FunUserDepositToEscrowReq {
            user_id: ctx.user_id,
            quantity: format!("{:?}", esc.amount),
            blockchain: chain.to_string(),
            user_address: format!("{:?}", esc.owner),
            contract_address: format!("{:?}", tx.get_to().context("no to")?),
            transaction_hash: format!("{:?}", tx.get_hash()),
            receiver_address: format!("{:?}", esc.recipient),
        })
        .await?;
        Ok(())
    }
    #[tokio::test]
    async fn test_user_ethereum_deposit_back_strategy() -> Result<()> {
        let _ = setup_logs(LogLevel::Info);
        drop_and_recreate_database()?;
        let user_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;
        let admin_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;
        let escrow_key = Secp256k1SecretKey::new_random();
        let conn_pool = EthereumRpcConnectionPool::localnet();
        let conn = conn_pool.get_conn().await?;
        let erc20_mock = deploy_mock_erc20(conn.get_raw().clone(), admin_key.clone()).await?;
        let eoa = Secp256k1SecretKey::new_random();
        erc20_mock
            .mint(
                &admin_key.key,
                user_key.address,
                U256::from(200000000000i64),
            )
            .await?;
        erc20_mock
            .mint(&admin_key.key, eoa.address, U256::from(200000000000i64))
            .await?;
        let eth = EthereumToken::new(conn.get_raw().clone());
        eth.transfer(
            admin_key.clone(),
            escrow_key.address,
            U256::from(1e18 as i64),
        )
        .await?;
        let escrow_contract =
            EscrowContract::deploy(conn.get_raw().clone(), &escrow_key.key).await?;

        let tx_hash = erc20_mock
            .transfer(
                &user_key.key,
                escrow_contract.address(),
                U256::from(20000000000i64),
            )
            .await?;
        let db = connect_to_database(database_test_config()).await?;
        let ret = db
            .execute(FunAuthSignupReq {
                address: format!("{:?}", user_key.address),
                email: "".to_string(),
                phone: "".to_string(),
                preferred_language: "".to_string(),
                agreed_tos: true,
                agreed_privacy: true,
                ip_address: Ipv4Addr::new(127, 0, 0, 1).into(),
                username: None,
                age: None,
                public_id: 1,
            })
            .await?
            .into_result()
            .context("No user signup resp")?;
        let ctx = RequestContext {
            connection_id: 0,
            user_id: ret.user_id,
            seq: 0,
            method: 0,
            log_id: 0,
        };

        let mut stablecoins = BlockchainCoinAddresses::new();
        stablecoins.insert(
            EnumBlockChain::EthereumGoerli,
            EnumBlockchainCoin::USDC,
            erc20_mock.address,
        );

        // at this step, tx should be passed with quickalert
        let tx = TransactionFetcher::new_and_assume_ready(tx_hash, &conn).await?;
        on_user_deposit(
            &conn,
            &ctx,
            &db,
            EnumBlockChain::EthereumGoerli,
            &tx,
            &stablecoins,
            &erc20_mock.contract.abi(),
            &escrow_contract,
        )
        .await?;

        let strategy = db
            .execute(FunUserCreateStrategyReq {
                user_id: ctx.user_id,
                name: "TEST".to_string(),
                description: "TEST".to_string(),
            })
            .await?
            .into_result()
            .context("create strategy")?;

        user_back_strategy(
            &conn,
            &ctx,
            &db,
            EnumBlockChain::EthereumGoerli,
            U256::from(1000),
            &stablecoins,
            strategy.strategy_id,
            &admin_key.key,
            &escrow_key.key,
            EnumBlockchainCoin::USDC,
            escrow_contract,
            eoa,
            &DexAddresses::new(),
        )
        .await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_user_ethereum_deposit_refund() -> Result<()> {
        let _ = setup_logs(LogLevel::Info);
        drop_and_recreate_database()?;
        let user_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;
        let admin_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;
        let escrow_key = Secp256k1SecretKey::new_random();
        let conn_pool = EthereumRpcConnectionPool::localnet();
        let conn = conn_pool.get_conn().await?;
        let erc20_mock = deploy_mock_erc20(conn.get_raw().clone(), admin_key.clone()).await?;
        erc20_mock
            .mint(
                &admin_key.key,
                user_key.address,
                U256::from(200000000000i64),
            )
            .await?;
        let eth = EthereumToken::new(conn.get_raw().clone());
        eth.transfer(
            admin_key.clone(),
            escrow_key.address,
            U256::from(1e18 as i64),
        )
        .await?;
        let escrow_contract =
            EscrowContract::deploy(conn.get_raw().clone(), &escrow_key.key).await?;

        let tx_hash = erc20_mock
            .transfer(
                &user_key.key,
                escrow_contract.address(),
                U256::from(20000000000i64),
            )
            .await?;
        let db = connect_to_database(database_test_config()).await?;
        let ret = db
            .execute(FunAuthSignupReq {
                address: format!("{:?}", user_key.address),
                email: "".to_string(),
                phone: "".to_string(),
                preferred_language: "".to_string(),
                agreed_tos: true,
                agreed_privacy: true,
                ip_address: Ipv4Addr::new(127, 0, 0, 1).into(),
                username: None,
                age: None,
                public_id: 1,
            })
            .await?
            .into_result()
            .context("No user signup resp")?;
        let ctx = RequestContext {
            connection_id: 0,
            user_id: ret.user_id,
            seq: 0,
            method: 0,
            log_id: 0,
        };

        let mut stablecoins = BlockchainCoinAddresses::new();
        stablecoins.insert(
            EnumBlockChain::EthereumGoerli,
            EnumBlockchainCoin::USDC,
            erc20_mock.address,
        );

        // at this step, tx should be passed with quickalert
        let tx = TransactionFetcher::new_and_assume_ready(tx_hash, &conn).await?;
        on_user_deposit(
            &conn,
            &ctx,
            &db,
            EnumBlockChain::EthereumGoerli,
            &tx,
            &stablecoins,
            &erc20_mock.contract.abi(),
            &escrow_contract,
        )
        .await?;

        let strategy = db
            .execute(FunUserCreateStrategyReq {
                user_id: ctx.user_id,
                name: "TEST".to_string(),
                description: "TEST".to_string(),
            })
            .await?
            .into_result()
            .context("create strategy")?;

        on_user_request_refund(
            &conn,
            &ctx,
            &db,
            EnumBlockChain::EthereumGoerli,
            &stablecoins,
            &escrow_contract,
            U256::from(1000),
            user_key.address,
            &escrow_key.key,
            EnumBlockchainCoin::USDC,
        )
        .await?;
        Ok(())
    }
}
