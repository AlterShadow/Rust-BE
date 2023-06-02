use eth_sdk::escrow::EscrowContract;
use eth_sdk::signer::Secp256k1SecretKey;
use eth_sdk::strategy_pool::StrategyPoolContract;
use eth_sdk::utils::verify_message_address;
use eth_sdk::*;
use eyre::*;
use gen::database::*;
use gen::model::*;
use lib::database::DbClient;
use lib::handler::RequestHandler;
use lib::toolbox::*;
use lib::utils::hex_decode;
use lib::ws::*;
use std::str::FromStr;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tracing::info;
use web3::signing::Key;
use web3::types::{Address, H256, U256};

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
                net_value: 0.0,
                followers: ret.followers as _,
                backers: ret.backers as _,
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
                risk_score: ret.risk_score.unwrap_or(0.0),
                aum: ret.aum.unwrap_or(0.0),
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
pub async fn calculate_sp_tokens() -> U256 {
    // TODO: calculate SP tokens based current price
    U256::from(123)
}

pub async fn deploy_strategy_contract(
    conn: &EthereumRpcConnection,
    key: impl Key,
    strategy_token_name: String,
    strategy_token_symbol: String,
) -> Result<StrategyPoolContract<EitherTransport>> {
    info!("Deploying strategy contract");

    let strategy = StrategyPoolContract::deploy(
        conn.clone().into_raw(),
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
    stablecoin_addresses: &StableCoinAddresses,
    escrow_contract: &EscrowContract<EitherTransport>,
) -> Result<TransactionFetcher> {
    let token_address = stablecoin_addresses
        .get_by_chain_and_token(chain, escrow.token)
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
    amount: U256,
    stablecoin_addresses: &StableCoinAddresses,
    strategy_id: i64,
    strategy_pool_signer: impl Key,
    escrow_signer: impl Key,
    stablecoin: StableCoin,
    escrow_contract: EscrowContract<EitherTransport>,
    // TODO: externally_owned_account: impl Key,
) -> Result<()> {
    let mut user_registered_strategy = db
        .execute(FunUserGetStrategyReq { strategy_id })
        .await?
        .into_result()
        .context("user_registered_strategy")?;
    if user_registered_strategy.evm_contract_address.is_none() {
        let contract = deploy_strategy_contract(
            &conn,
            strategy_pool_signer,
            user_registered_strategy.strategy_name.clone(),
            user_registered_strategy.strategy_name, // strategy symbol
        )
        .await?;
        user_registered_strategy.evm_contract_address = Some(format!("{:?}", contract.address()));
    }
    let sp_tokens = calculate_sp_tokens().await;
    let strategy_address: Address = user_registered_strategy
        .evm_contract_address
        .unwrap()
        .parse()?;

    let escrow_signer_address = escrow_signer.address();
    // we need to trade, not transfer, and then we need to call deposit on the strategy contract
    let transaction = transfer_token_to_strategy_contract(
        conn,
        escrow_signer,
        EscrowTransfer {
            token: stablecoin,
            amount: sp_tokens,
            recipient: strategy_address,
            // TODO: recipient: externally_owned_account.address(),
            owner: escrow_signer_address,
        },
        chain,
        stablecoin_addresses,
        &escrow_contract,
    )
    .await?;
    // TODO: transfer from externally_owned_account to strategy contract, ERC20
    // TODO: need to trade deposit token for strategy's tokens and call "deposit" on the strategy contract wrapper
    db.execute(FunUserBackStrategyReq {
        user_id: ctx.user_id,
        strategy_id: user_registered_strategy.strategy_id,
        quantity: format!("{:?}", amount),
        blockchain: chain.to_string(),
        transaction_hash: format!("{:?}", transaction.get_hash()),
        earn_sp_tokens: format!("{:?}", sp_tokens),
    })
    .await?;
    info!(
        "Transfer token to strategy contract {:?}",
        transaction.get_hash()
    );

    let _tx = TransactionFetcher::new_and_assume_ready(transaction.get_hash(), conn).await?;
    Ok(())
}

pub struct MethodUserBackStrategy {
    pub conn: EthereumRpcConnection,
    pub stablecoin_addresses: Arc<StableCoinAddresses>,
    pub strategy_pool_signer: Arc<Secp256k1SecretKey>,
    pub escrow_contract: EscrowContract<EitherTransport>,
    pub escrow_signer: Arc<Secp256k1SecretKey>,
}
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
        let eth_conn = self.conn.clone();
        let stablecoin_addresses = self.stablecoin_addresses.clone();
        let strategy_pool_signer = self.strategy_pool_signer.clone();
        let escrow_signer = self.escrow_signer.clone();
        let escrow_contract = self.escrow_contract.clone();
        toolbox.spawn_response(ctx.clone(), async move {
            ensure_user_role(&conn, EnumRole::User)?;

            user_back_strategy(
                &eth_conn,
                &ctx,
                &db,
                EnumBlockChain::from_str(&req.blockchain)?,
                req.quantity.parse()?,
                &stablecoin_addresses,
                req.strategy_id,
                &**strategy_pool_signer,
                &**escrow_signer,
                StableCoin::Usdc,
                escrow_contract,
            )
            .await?;
            Ok(())
        })
    }
}
pub struct MethodUserRequestRefund {
    pub conn: EthereumRpcConnection,
    pub stablecoin_addresses: Arc<StableCoinAddresses>,
    pub escrow_contract: EscrowContract<EitherTransport>,
    pub escrow_signer: Arc<Secp256k1SecretKey>,
}

impl RequestHandler for MethodUserRequestRefund {
    type Request = UserRequestRefundRequest;
    type Response = UserRequestRefundResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();
        let eth_conn = self.conn.clone();
        let stablecoin_addresses = self.stablecoin_addresses.clone();
        let escrow_signer = self.escrow_signer.clone();
        let escrow_contract = self.escrow_contract.clone();
        toolbox.spawn_response(ctx.clone(), async move {
            ensure_user_role(&conn, EnumRole::User)?;

            on_user_request_refund(
                &eth_conn,
                &ctx,
                &db,
                EnumBlockChain::from_str(&req.blockchain)?,
                &stablecoin_addresses,
                &escrow_contract,
                req.quantity.parse()?,
                req.wallet_address.parse()?,
                &escrow_signer.key,
                StableCoin::Usdc,
            )
            .await?;
            Ok(())
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
pub struct MethodUserUnfollowExpert;
impl RequestHandler for MethodUserUnfollowExpert {
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
                .execute(FunUserListRegisteredWalletsReq {
                    user_id: ctx.user_id,
                })
                .await?;

            Ok(UserListWalletsResponse {
                wallets: ret
                    .into_rows()
                    .into_iter()
                    .map(|x| ListWalletsRow {
                        wallet_id: x.registered_wallet_id,
                        blockchain: x.blockchain,
                        wallet_address: x.address,
                        is_default: false,
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

            let _ret = db
                .execute(FunUserRemoveRegisteredWalletReq {
                    registered_wallet_id: req.wallet_id,
                    user_id: ctx.user_id,
                })
                .await?;

            Ok(UserDeregisterWalletResponse { success: true })
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

pub struct MethodUserListWalletActivityHistory;

impl RequestHandler for MethodUserListWalletActivityHistory {
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
pub async fn on_user_request_refund(
    _conn: &EthereumRpcConnection,
    ctx: &RequestContext,
    db: &DbClient,
    chain: EnumBlockChain,
    stablecoin_addresses: &StableCoinAddresses,
    escrow_contract: &EscrowContract<EitherTransport>,
    quantity: U256,
    wallet_address: Address,
    escrow_signer: impl Key,
    token: StableCoin,
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
            blockchain: chain.to_string(),
            wallet_address: format!("{:?}", wallet_address),
        })
        .await?
        .into_result()
        .context("No result")?;
    let token_address = stablecoin_addresses
        .get_by_chain_and_token(chain, token)
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

#[cfg(test)]
mod tests {
    use super::*;
    use eth_sdk::escrow_tracker::escrow::parse_escrow;
    use eth_sdk::mock_erc20::deploy_mock_erc20;
    use eth_sdk::signer::Secp256k1SecretKey;
    use eth_sdk::*;
    use lib::database::{connect_to_database, drop_and_recreate_database, DatabaseConfig};
    use lib::log::{setup_logs, LogLevel};
    use std::net::Ipv4Addr;

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
        stablecoin_addresses: &StableCoinAddresses,
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
        erc20_mock
            .mint(
                &admin_key.key,
                user_key.address,
                U256::from(200000000000i64),
            )
            .await?;
        let eth = EthereumToken::new2(conn.get_raw().clone());
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
        let db = connect_to_database(DatabaseConfig {
            user: Some("postgres".to_string()),
            password: Some("123456".to_string()),
            dbname: Some("mc2fi".to_string()),
            host: Some("localhost".to_string()),
            ..Default::default()
        })
        .await?;
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

        let mut stablecoins = StableCoinAddresses::default();
        stablecoins.inner.insert(
            EnumBlockChain::EthereumGoerli,
            vec![(StableCoin::Usdc, erc20_mock.address)],
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
            StableCoin::Usdc,
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
        let eth = EthereumToken::new2(conn.get_raw().clone());
        eth.transfer(
            admin_key.clone(),
            escrow_key.address,
            U256::from(1e18 as i64),
        )
        .await?;
        let escrow_contract =
            EscrowContract::deploy(conn.get_raw().clone(), &escrow_key.key).await?;
        // TODO: we want to replace escrow_key with escrow_contract
        let tx_hash = erc20_mock
            .transfer(
                &user_key.key,
                escrow_contract.address(),
                U256::from(20000000000i64),
            )
            .await?;
        let db = connect_to_database(DatabaseConfig {
            user: Some("postgres".to_string()),
            password: Some("123456".to_string()),
            dbname: Some("mc2fi".to_string()),
            host: Some("localhost".to_string()),
            ..Default::default()
        })
        .await?;
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

        let mut stablecoins = StableCoinAddresses::default();
        stablecoins.inner.insert(
            EnumBlockChain::EthereumGoerli,
            vec![(StableCoin::Usdc, erc20_mock.address)],
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
            StableCoin::Usdc,
        )
        .await?;
        Ok(())
    }
}
