use crate::admin_method::AdminSubscribeTopic;
use crate::audit::{
    get_audit_rules, validate_audit_rule_immutable_tokens, AuditLogger, AUDIT_TOP25_TOKENS,
};
use api::cmc::CoinMarketCap;
use chrono::Utc;
use eth_sdk::erc20::approve_and_ensure_success;
use eth_sdk::erc20::Erc20Token;
use eth_sdk::escrow::transfer_token_to_and_ensure_success;
use eth_sdk::escrow::{AbstractEscrowContract, EscrowContract};
use eth_sdk::pair_paths::WorkingPancakePairPaths;
use eth_sdk::signer::Secp256k1SecretKey;
use eth_sdk::strategy_pool::{deposit_and_ensure_success, StrategyPoolContract};
use eth_sdk::strategy_wallet::{
    full_redeem_from_strategy_and_ensure_success, redeem_from_strategy_and_ensure_success,
    StrategyWalletContract,
};
use eth_sdk::utils::verify_message_address;
use eth_sdk::v3::smart_router::copy_trade_and_ensure_success;
use eth_sdk::v3::smart_router::PancakeSmartRouterV3Contract;
use eth_sdk::*;
use eyre::*;
use futures::FutureExt;
use gen::database::*;
use gen::model::*;
use itertools::Itertools;
use lib::database::DbClient;
use lib::handler::{FutureResponse, RequestHandler};
use lib::toolbox::*;
use lib::utils::hex_decode;
use lib::ws::SubscribeManager;
use lib::{DEFAULT_LIMIT, DEFAULT_OFFSET};
use num_traits::cast::FromPrimitive;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::info;
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
                    wallet_address: req.wallet_address,
                })
                .await?;

            Ok(UserListStrategiesResponse {
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
                        wallet_address: x.wallet_address,
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
                    balance: x.balance,
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
            Ok(UserGetDepositWithdrawBalanceResponse { balance })
        }
        .boxed()
    }
}
async fn deploy_wallet_contract(
    conn: &EthereumRpcConnection,
    key: impl Key + Clone,
    backer: Address,
    admin: Address,
) -> Result<StrategyWalletContract<EitherTransport>> {
    info!("Deploying wallet contract");

    let wallet = StrategyWalletContract::deploy(conn.clone(), key, backer, admin).await?;

    info!("Deploy wallet contract success");

    Ok(wallet)
}

async fn deploy_strategy_contract(
    conn: &EthereumRpcConnection,
    key: impl Key + Clone,
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

async fn user_back_strategy(
    conn: &EthereumRpcConnection,
    ctx: &RequestContext,
    db: &DbClient,
    blockchain: EnumBlockChain,
    back_usdc_amount: U256,
    token_addresses: &BlockchainCoinAddresses,
    strategy_id: i64,
    escrow_coin: EnumBlockchainCoin,
    escrow_contract: EscrowContract<EitherTransport>,
    dex_addresses: &DexAddresses,
    master_key: impl Key + Clone,
) -> Result<()> {
    if back_usdc_amount == U256::zero() {
        bail!("back zero amount");
    }

    /* check if user has enough balance */
    // TODO: add user balance to the database
    let user_balance = U256::max_value();
    if user_balance < back_usdc_amount {
        bail!("insuficient balance");
    }

    /* fetch user address to receive shares */
    // TODO: fetch the correct address where user desires to receive shares on this chain
    // since users can have multiple addresses, this information is critical
    // for now, we fetch the "address" field from the user table
    let user_wallet_address_to_receive_shares_on_this_chain = Address::from_str(
        &db.execute(FunAdminListUsersReq {
            limit: 1,
            offset: 0,
            user_id: Some(ctx.user_id),
            address: None,
            username: None,
            email: None,
            role: None,
        })
        .await?
        .into_result()
        .context("No such user")?
        .address,
    )?;

    /* instantiate strategy wallet contract wrapper */
    let strategy_wallet_contract = match db
        .execute(FunUserListStrategyWalletsReq {
            user_id: ctx.user_id,
            blockchain: Some(blockchain),
        })
        .await?
        .into_result()
    {
        Some(strategy_wallet_contract) => {
            /* if user has wallet on this chain, use it */
            StrategyWalletContract::new(
                conn.clone(),
                Address::from_str(&strategy_wallet_contract.address)?,
            )?
        }
        None => {
            /* if user does not have a wallet on this chain, deploy it, and use it */
            // TODO: add admin as Address::zero() if user has opted out of having an admin
            let strategy_wallet_contract = deploy_wallet_contract(
                &conn,
                master_key.clone(),
                user_wallet_address_to_receive_shares_on_this_chain,
                master_key.address(),
            )
            .await?;

            /* save wallet to database */
            db.execute(FunUserAddStrategyWalletReq {
                user_id: ctx.user_id,
                blockchain,
                address: format!("{:?}", strategy_wallet_contract.address()),
            })
            .await?;

            strategy_wallet_contract
        }
    };

    /* fetch strategy */
    let strategy = db
        .execute(FunUserListStrategiesReq {
            strategy_id: Some(strategy_id),
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
        .context("strategy is not registered in the database")?;

    /* fetch strategy's tokens */
    let strategy_token_rows = db
        .execute(FunUserListStrategyInitialTokenRatiosReq { strategy_id })
        .await?
        .into_rows();
    let mut total_strategy_tokens = U256::zero();
    let mut all_strategy_tokens: Vec<(EnumBlockChain, Address, U256)> = Vec::new();
    for row in strategy_token_rows {
        let token_address = Address::from_str(&row.token_address)?;
        let token_amount = U256::from_dec_str(&row.quantity)?;
        total_strategy_tokens = total_strategy_tokens.try_checked_add(token_amount)?;
        all_strategy_tokens.push((row.blockchain, token_address, token_amount));
    }

    /* merge all token amounts to this chain's tokens */
    /* this step will fail if any token address is from an unknown token from any chain */
    // TODO: replace this when we have non-hardcoded known tokens and a "tokens" table
    // TODO: fetch each token amount accross chains and address on this chain directly from database
    let strategy_tokens_and_amounts_on_this_chain: HashMap<Address, U256> =
        merge_multichain_strategy_tokens(blockchain, token_addresses, all_strategy_tokens)?;

    /* deduce fees from back amount */
    // TODO: fetch fees from strategy
    // TODO: deduce fees from back amount
    // TODO: use (back amount - fees) to calculate trade spenditure and SP shares
    // TODO: register appropriate fees for the treasury and the strategy creator
    let back_usdc_amount_minus_fees = back_usdc_amount;

    /* calculate how much of back amount to spend on each strategy token */
    let escrow_allocations_for_tokens = calculate_escrow_allocation_for_strategy_tokens(
        back_usdc_amount_minus_fees,
        total_strategy_tokens,
        strategy_tokens_and_amounts_on_this_chain,
    )?;

    /* instantiate strategy contract wrapper */
    let strategy_pool = db
        .execute(FunWatcherListStrategyPoolContractReq {
            limit: 1,
            offset: 0,
            strategy_id: Some(strategy_id),
            blockchain: Some(blockchain),
            address: None,
        })
        .await?
        .into_result();
    let sp_contract: StrategyPoolContract<EitherTransport>;
    match strategy_pool {
        Some(addr) => {
            let address = Address::from_str(&addr.address)?;
            sp_contract = StrategyPoolContract::new(conn.clone(), address)?;
        }
        None => {
            /* if strategy pool doesn't exist in this chain, create it */
            let contract = deploy_strategy_contract(
                &conn,
                master_key.clone(),
                strategy.strategy_name.clone(),
                strategy.strategy_name, // strategy symbol
            )
            .await?;
            /* insert strategy contract address in the database */
            db.execute(FunWatcherAddStrategyPoolContractReq {
                strategy_id,
                blockchain,
                address: format!("{:?}", contract.address()),
            })
            .await?;

            sp_contract = contract;
        }
    };

    /* calculate shares to mint for backer */
    // TODO: find out if we use back amount with or without fees for share calculation
    // currently calculating with back amount minus fees
    let sp_total_shares = sp_contract.total_supply().await?;
    let mut maybe_sp_assets_and_amounts: Option<(Vec<Address>, Vec<U256>)> = None;
    let mut max_retries = 10;
    while maybe_sp_assets_and_amounts.is_none() && max_retries > 0 {
        match sp_contract.assets_and_balances().await {
            Ok(assets_and_amounts) => {
                maybe_sp_assets_and_amounts = Some(assets_and_amounts);
            }
            Err(_) => {
                /* if we can't query the contract's assets, it's because it is currently trading */
                /* wait a bit and try again */
                sleep(Duration::from_secs(10)).await;
                max_retries -= 1;
            }
        }
    }
    let sp_assets_and_amounts = maybe_sp_assets_and_amounts
        .ok_or_else(|| eyre!("failed to query strategy pool assets and amounts"))?;

    let escrow_token_address = token_addresses
        .get(blockchain, escrow_coin)
        .ok_or_else(|| eyre!("usdc address not available on this chain"))?;
    let escrow_token_contract = Erc20Token::new(conn.clone(), escrow_token_address)?;
    let shares_to_mint = calculate_shares(
        &conn,
        &CoinMarketCap::new_debug_key()?,
        sp_total_shares,
        sp_assets_and_amounts,
        sp_contract.decimals().await?,
        back_usdc_amount_minus_fees,
        escrow_coin,
        escrow_token_contract.decimals().await?,
    )
    .await?;

    /* instantiate pancake contract */
    let pancake_contract = PancakeSmartRouterV3Contract::new(
        conn.clone(),
        dex_addresses
            .get(blockchain, EnumDex::PancakeSwap)
            .ok_or_else(|| eyre!("pancake swap not available on this chain"))?,
    )?;

    //TODO: make some way of replaying the correct transactions in case of failure in the middle of the backing process

    /* transfer escrow to our EOA */
    transfer_token_to_and_ensure_success(
        escrow_contract,
        &conn,
        14,
        10,
        Duration::from_secs(10),
        master_key.clone(),
        escrow_token_address,
        master_key.address(),
        back_usdc_amount,
    )
    .await?;

    /* approve pancakeswap to trade escrow token */
    approve_and_ensure_success(
        escrow_token_contract,
        &conn,
        12,
        10,
        Duration::from_secs(10),
        master_key.clone(),
        pancake_contract.address(),
        back_usdc_amount,
    )
    .await?;

    /* trade escrow token for strategy's tokens */
    let (tokens_to_deposit, amounts_to_deposit) = trade_escrow_for_strategy_tokens(
        &conn,
        master_key.clone(),
        blockchain,
        escrow_token_address,
        &pancake_contract,
        escrow_allocations_for_tokens,
    )
    .await?;

    /* approve tokens and amounts to SP contract */
    for (token, amount) in tokens_to_deposit.iter().zip(amounts_to_deposit.iter()) {
        approve_and_ensure_success(
            Erc20Token::new(conn.clone(), token.clone())?,
            &conn,
            12,
            10,
            Duration::from_secs(10),
            master_key.clone(),
            sp_contract.address(),
            amount.clone(),
        )
        .await?;
    }

    /* deposit to strategy pool contract */
    let deposit_transaction_hash = deposit_and_ensure_success(
        sp_contract,
        &conn,
        12,
        10,
        Duration::from_secs(10),
        master_key.clone(),
        tokens_to_deposit,
        amounts_to_deposit,
        shares_to_mint,
        strategy_wallet_contract.address(),
    )
    .await?;

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
            blockchain,
            transaction_hash: format!("{:?}", deposit_transaction_hash),
            earn_sp_tokens: format!("{:?}", shares_to_mint),
        })
        .await?
        .into_result()
        .context("No record")?;
    if !ret.success {
        bail!(
            "User back strategy not sucessful due to other clients updated record at the same time"
        )
    }

    Ok(())
}

fn merge_multichain_strategy_tokens(
    chain: EnumBlockChain,
    known_addresses: &BlockchainCoinAddresses,
    multichain_tokens: Vec<(EnumBlockChain, Address, U256)>,
) -> Result<HashMap<Address, U256>> {
    /* merge multichain token addresses and amounts respective addresses on one chain with summed amounts */
    /* will fail if any token address is from an unknown token */
    /* TODO: replace this when we can query unified balance from tokens accross chains from db */
    let mut merged_strategy_token_amounts_by_chain: HashMap<Address, U256> = HashMap::new();
    for (token_chain, token_address, token_amount) in multichain_tokens {
        let strategy_token = known_addresses
            .get_by_address(token_chain, token_address)
            .ok_or_else(|| eyre!("strategy token is unknown"))?;
        if token_chain == chain {
            /* strategy token is on this chain, use token contract address directly */
            match merged_strategy_token_amounts_by_chain.entry(token_address) {
                Entry::Vacant(e) => {
                    e.insert(token_amount);
                }
                Entry::Occupied(mut e) => {
                    let balance = e.get_mut();
                    *balance = balance.try_checked_add(token_amount)?;
                }
            }
        } else {
            /* strategy token is not on this chain, use this chain's token contract address */
            let strategy_token_address_on_this_chain =
                known_addresses
                    .get(chain, strategy_token)
                    .ok_or_else(|| eyre!("strategy token not available on this chain"))?;
            match merged_strategy_token_amounts_by_chain.entry(strategy_token_address_on_this_chain)
            {
                Entry::Vacant(e) => {
                    e.insert(token_amount);
                }
                Entry::Occupied(mut e) => {
                    let balance = e.get_mut();
                    *balance = balance.try_checked_add(token_amount)?;
                }
            }
        }
    }
    Ok(merged_strategy_token_amounts_by_chain)
}

fn calculate_escrow_allocation_for_strategy_tokens(
    escrow_amount: U256,
    total_strategy_tokens: U256,
    strategy_tokens_and_amounts: HashMap<Address, U256>,
) -> Result<HashMap<Address, U256>> {
    /* calculates how much of escrow to spend on each strategy token */
    /* allocation = (strategy_token_amount * escrow_amount) / total_strategy_token_amounts */
    let mut escrow_allocations: HashMap<Address, U256> = HashMap::new();
    for (token_address, token_amount) in strategy_tokens_and_amounts {
        let escrow_allocation = token_amount.mul_div(escrow_amount, total_strategy_tokens)?;
        escrow_allocations.insert(token_address, escrow_allocation);
    }
    Ok(escrow_allocations)
}

async fn calculate_shares(
    conn: &EthereumRpcConnection,
    cmc: &CoinMarketCap,
    sp_total_shares: U256,
    sp_tokens_and_amounts: (Vec<Address>, Vec<U256>),
    sp_decimals: U256,
    escrow_amount: U256,
    escrow_coin: EnumBlockchainCoin,
    escrow_decimals: U256,
) -> Result<U256> {
    /* calculate shares to mint based on the value of tokens held by strategy pool and the value of escrow */
    let escrow_symbol = match escrow_coin {
        EnumBlockchainCoin::USDC => "USDC".to_string(),
        EnumBlockchainCoin::USDT => "USDT".to_string(),
        EnumBlockchainCoin::BUSD => "BUSD".to_string(),
        _ => bail!("unsupported escrow coin"),
    };
    /* multiply the escrow amount by the price to get its value with no consideration for decimals */
    /* if escrow decimals > sp decimals, divide unconsidered value by 10^(escrow decimals - sp decimals) to account for decimal differences */
    /* if sp decimals > escrow decimals, multiply the unconsidered value by 10^(sp decimals - escrow decimals) to account for decimal differences */
    /* this is valid for all tokens, not just the escrow */
    let escrow_value: U256;
    if escrow_decimals > sp_decimals {
        escrow_value = escrow_amount
            .mul_f64(cmc.get_usd_prices_by_symbol(&vec![escrow_symbol]).await?[0])?
            .try_checked_div(U256::exp10(
                escrow_decimals.as_usize() - sp_decimals.as_usize(),
            ))?;
    } else {
        escrow_value = escrow_amount
            .mul_f64(cmc.get_usd_prices_by_symbol(&vec![escrow_symbol]).await?[0])?
            .try_checked_mul(U256::exp10(
                sp_decimals.as_usize() - escrow_decimals.as_usize(),
            ))?;
    }
    if sp_total_shares == U256::zero() {
        /* if strategy pool is empty, shares = escrow value */
        Ok(escrow_value)
    } else {
        /* if strategy pool is active, shares = (escrow_value * total_strategy_shares) / total_strategy_value */
        let sp_total_value: U256 = {
            let mut total_value = U256::zero();
            for (asset, amount) in sp_tokens_and_amounts
                .0
                .iter()
                .zip(sp_tokens_and_amounts.1.iter())
            {
                let erc20 = Erc20Token::new(conn.clone(), *asset)?;
                let price = cmc
                    .get_usd_prices_by_symbol(&vec![erc20.symbol().await?])
                    .await?;
                /* add to total value the value of each token accounting for decimal differences */
                let token_decimals = erc20.decimals().await?;
                if token_decimals > sp_decimals {
                    total_value =
                        total_value.try_checked_add(amount.mul_f64(price[0])?.try_checked_div(
                            U256::exp10(token_decimals.as_usize() - sp_decimals.as_usize()),
                        )?)?;
                } else {
                    total_value =
                        total_value.try_checked_add(amount.mul_f64(price[0])?.try_checked_mul(
                            U256::exp10(sp_decimals.as_usize() - token_decimals.as_usize()),
                        )?)?;
                }
            }
            total_value
        };
        Ok(escrow_value.mul_div(
            sp_total_shares,
            if sp_total_value == U256::zero() {
                U256::one()
            } else {
                sp_total_value
            },
        )?)
    }
}

async fn trade_escrow_for_strategy_tokens(
    conn: &EthereumRpcConnection,
    master_key: impl Key + Clone,
    chain: EnumBlockChain,
    escrow_token_address: Address,
    dex_contract: &PancakeSmartRouterV3Contract<EitherTransport>,
    tokens_and_amounts_to_buy: HashMap<Address, U256>,
) -> Result<(Vec<Address>, Vec<U256>)> {
    /* buys tokens and amounts and returns a vector or bought tokens and amounts out */
    // TODO: stop using hardcoded hashmaps and retrieve paths from database
    let pancake_trade_parser = build_pancake_swap()?;
    let pancake_paths = WorkingPancakePairPaths::new()?;
    let mut token_addresses_to_deposit: Vec<Address> = Vec::new();
    let mut token_amounts_to_deposit: Vec<U256> = Vec::new();
    for (token_address, amount_to_spend_on_it) in tokens_and_amounts_to_buy {
        let pancake_path_set =
            pancake_paths.get_pair_by_address(chain, escrow_token_address, token_address)?;
        let trade_hash = copy_trade_and_ensure_success(
            dex_contract.clone(),
            &conn,
            12,
            10,
            Duration::from_secs(10),
            master_key.clone(),
            pancake_path_set,
            amount_to_spend_on_it,
            U256::one(), // TODO: find a way to estimate amount out
        )
        .await?;

        let trade = pancake_trade_parser.parse_trade(
            &TransactionFetcher::new_and_assume_ready(trade_hash, &conn).await?,
            chain,
        )?;

        token_addresses_to_deposit.push(token_address);
        token_amounts_to_deposit.push(trade.amount_out);
    }
    Ok((token_addresses_to_deposit, token_amounts_to_deposit))
}

pub struct MethodUserBackStrategy {
    pub pool: EthereumRpcConnectionPool,
    pub stablecoin_addresses: Arc<BlockchainCoinAddresses>,
    pub escrow_contract: Arc<AbstractEscrowContract>,
    pub master_key: Secp256k1SecretKey,
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
        let token_addresses = self.stablecoin_addresses.clone();
        let dex_addresses = self.dex_addresses.clone();
        let escrow_contract = self.escrow_contract.clone();
        let master_key = self.master_key.clone();
        async move {
            let escrow_contract = escrow_contract.get(&pool, req.blockchain).await?;
            let eth_conn = pool.get(req.blockchain).await?;
            ensure_user_role(ctx, EnumRole::User)?;

            user_back_strategy(
                &eth_conn,
                &ctx,
                &db,
                req.blockchain,
                req.quantity.parse()?,
                &token_addresses,
                req.strategy_id,
                EnumBlockchainCoin::USDC,
                escrow_contract,
                &dex_addresses,
                master_key,
            )
            .await?;
            Ok(UserBackStrategyResponse { success: true })
        }
        .boxed()
    }
}

async fn user_exit_strategy(
    conn: &EthereumRpcConnection,
    ctx: &RequestContext,
    db: &DbClient,
    blockchain: EnumBlockChain,
    strategy_id: i64,
    shares: Option<U256>,
    master_key: impl Key + Clone,
) -> Result<H256> {
    /* instantiate strategy wallet */
    let strategy_wallet_contract = db
        .execute(FunUserListStrategyWalletsReq {
            user_id: ctx.user_id,
            blockchain: Some(blockchain),
        })
        .await?
        .into_result()
        .context("user has no strategy wallet on this chain")?;
    let strategy_wallet_contract = StrategyWalletContract::new(
        conn.clone(),
        Address::from_str(&strategy_wallet_contract.address)?,
    )?;

    /* if master key eoa is not admin, we can't redeem */
    if strategy_wallet_contract.admin().await? != master_key.address() {
        bail!("strategy wallet has another or no admin");
    }

    let strategy_pool = db
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
    let sp_contract =
        StrategyPoolContract::new(conn.clone(), Address::from_str(&strategy_pool.address)?)?;

    /* check if strategy is trading */
    if sp_contract.is_paused().await? {
        bail!("strategy is currently trading, redeem is not possible");
    }

    /* redeem */
    let shares_redeemed: U256;
    let tx_hash = match shares {
        Some(shares) => {
            /* check share balance first */
            if sp_contract
                .balance_of(strategy_wallet_contract.address())
                .await?
                < shares
            {
                bail!("not enough shares");
            }
            shares_redeemed = shares;
            /* if strategy is currently trading, redeem is not possible */
            redeem_from_strategy_and_ensure_success(
                strategy_wallet_contract.clone(),
                &conn,
                12,
                10,
                Duration::from_secs(10),
                master_key.clone(),
                sp_contract.address(),
                shares,
            )
            .await
            .context("redeem is not possible currently")?
        }
        None => {
            /* check share balance first */
            let share_balance = sp_contract
                .balance_of(strategy_wallet_contract.address())
                .await?;
            if share_balance == U256::zero() {
                bail!("no shares to redeem");
            }

            shares_redeemed = share_balance;
            /* if strategy is currently trading, redeem is not possible */
            full_redeem_from_strategy_and_ensure_success(
                strategy_wallet_contract.clone(),
                &conn,
                12,
                10,
                Duration::from_secs(10),
                master_key.clone(),
                sp_contract.address(),
            )
            .await
            .context("redeem is not possible currently")?
        }
    };

    /* update exit strategy ledger */
    db.execute(FunUserExitStrategyReq {
        user_id: ctx.user_id,
        strategy_id,
        // TODO: calculate value of sp tokens exit in usdc
        quantity: format!("{:?}", 0),
        blockchain,
        transaction_hash: format!("{:?}", tx_hash),
        redeem_sp_tokens: format!("{:?}", shares_redeemed),
    })
    .await?;

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
            let eth_conn = pool.get(EnumBlockChain::LocalNet).await?;
            // TODO: decide if we should ensure user role
            ensure_user_role(ctx, EnumRole::User)?;

            let tx_hash = user_exit_strategy(
                &eth_conn,
                &ctx,
                &db,
                req.blockchain,
                req.strategy_id,
                match req.quantity {
                    Some(quantity) => Some(quantity.parse()?),
                    None => None,
                },
                master_key,
            )
            .await?;
            Ok(UserExitStrategyResponse {
                success: true,
                transaction_hash: format!("{:?}", tx_hash),
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
                req.quantity.parse()?,
                req.wallet_address.parse()?,
                master_key,
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
                exit_ledger: ret
                    .into_iter()
                    .map(|x| ExitStrategyLedgerRow {
                        exit_ledger_id: x.exit_ledger_id,
                        strategy_id: x.strategy_id,
                        exit_quantity: x.exit_quantity,
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
                experts: ret
                    .into_iter()
                    .map(|x| UserListFollowedExpertsRow {
                        expert_id: x.expert_id,
                        user_public_id: x.user_public_id,
                        name: x.username,
                        linked_wallet: x.listening_wallet,
                        family_name: x.family_name,
                        given_name: x.given_name,
                        follower_count: x.follower_count as _,
                        description: x.description.unwrap_or_default(),
                        social_media: x.social_media.unwrap_or_default(),
                        risk_score: x.risk_score.unwrap_or_default(),
                        reputation_score: x.reputation_score.unwrap_or_default(),
                        aum: x.aum.unwrap_or_default(),
                        joined_at: x.joined_at,
                        requested_at: x.requested_at.unwrap_or_default(),
                        approved_at: x.approved_at,
                        pending_expert: x.pending_expert,
                        approved_expert: x.approved_expert,
                        followed: true,
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

            let ret = db
                .execute(FunUserCreateStrategyReq {
                    user_id: ctx.user_id,
                    name: req.name,
                    description: req.description,
                    strategy_thesis_url: req.strategy_thesis_url,
                    minimum_backing_amount_usd: req.minimum_backing_amount_usd,
                    strategy_fee: req.strategy_fee,
                    expert_fee: req.expert_fee,
                    agreed_tos: req.agreed_tos,
                    wallet_address: req.wallet_address,
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
                    wallet_address: req.wallet_address,
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
    )
    .await?;

    db.execute(FunUserRequestRefundReq {
        user_id: ctx.user_id,
        quantity: format!("{:?}", quantity),
        blockchain: chain,
        user_address: format!("{:?}", wallet_address),
        receiver_address: format!("{:?}", wallet_address),
        contract_address: format!("{:?}", escrow_contract.address()),
        transaction_hash: format!("{:?}", hash),
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
                    token_name: req.token_name,
                    token_address: req.token_address,
                    blockchain: req.blockchain,
                    quantity: req.quantity,
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
                    strategy_initial_token_ratio_id: req.token_id,
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
                        address: format!("{:?}", address),
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
                    user_id: ctx.user_id,
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                })
                .await?;
            Ok(UserListDepositLedgerResponse {
                ledger: resp
                    .into_iter()
                    .map(|x| UserListDepositLedgerRow {
                        blockchain: x.blockchain,
                        user_address: x.user_address,
                        contract_address: x.contract_address,
                        receiver_address: x.receiver_address,
                        quantity: x.quantity,
                        transaction_hash: x.transaction_hash,
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
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let manager = self.manger.clone();
        let toolbox = toolbox.clone();
        async move {
            manager.subscribe(AdminSubscribeTopic::AdminNotifyEscrowLedgerChange, ctx);
            // TODO: this is to provide mock data to FE. Remove this when we have real data
            tokio::spawn(async move {
                for i in 0..10 {
                    sleep(Duration::from_secs(3)).await;
                    let amount = U256::from(i);
                    let key = Secp256k1SecretKey::new_random();
                    info!("Sending mock data to FE, {}..", i);
                    manager.publish_to_all(
                        &toolbox,
                        AdminSubscribeTopic::AdminNotifyEscrowLedgerChange,
                        &UserListDepositLedgerRow {
                            quantity: format!("{:?}", amount),
                            blockchain: EnumBlockChain::EthereumMainnet,
                            user_address: format!("{:?}", key.address),
                            contract_address: format!("{:?}", key.address),
                            transaction_hash: format!("{:?}", key.address),
                            receiver_address: format!("{:?}", key.address),
                            created_at: Utc::now().timestamp(),
                        },
                    )
                }
            });
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
        toolbox: &Toolbox,
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
                wallets: resp
                    .into_iter()
                    .map(|x| UserListStrategyWalletsRow {
                        blockchain: x.blockchain,
                        address: x.address,
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
            let strategy_wallet = deploy_wallet_contract(
                &conn,
                master_key.clone(),
                Address::from_str(&req.wallet_address)?,
                match req.adminship {
                    true => master_key.address(),
                    false => Address::zero(),
                },
            )
            .await?;

            db.execute(FunUserAddStrategyWalletReq {
                // TODO: add opt in adminship in database for each strategy wallet
                // TODO: add backer wallet address registered in strategy wallet in database
                user_id: ctx.user_id,
                blockchain: req.blockchain,
                address: format!("{:?}", strategy_wallet.address()),
            })
            .await?;
            Ok(UserCreateStrategyWalletResponse {
                blockchain: req.blockchain,
                address: format!("{:?}", strategy_wallet.address()),
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
                                .find(|y| x.rule_id == y.id as i64)
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
    pub token_addresses: Arc<BlockchainCoinAddresses>,
}
impl RequestHandler for MethodUserGetEscrowAddressForStrategy {
    type Request = UserGetEscrowAddressForStrategyRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let addresses = self.addresses.clone();
        let token_addresses = self.token_addresses.clone();
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
            Ok(UserGetEscrowAddressForStrategyResponse {
                tokens: addresses
                    .into_iter()
                    .filter(|x| x.blockchain == strategy.blockchain)
                    // TODO: filter by token id
                    .map(|x| {
                        let usdc = token_addresses
                            .get(x.blockchain, EnumBlockchainCoin::USDC)
                            .unwrap();
                        UserAllowedEscrowTransferInfo {
                            receiver_address: x.address,
                            blockchain: x.blockchain,
                            token_id: 0,
                            token_symbol: "USDC".to_string(),
                            token_name: "USDC".to_string(),
                            token_address: format!("{:?}", usdc),
                        }
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

    pub async fn add_strategy_initial_token_ratio(
        db: &DbClient,
        strategy_id: i64,
        wbnb_address_on_bsc_testnet: Address,
        ts: i64,
    ) -> Result<()> {
        db.query(
            "
			INSERT INTO tbl.strategy_initial_token_ratio
			(fkey_strategy_id, blockchain, token_name, token_address, quantity, updated_at, created_at)
			VALUES
			($1, $2, $3, $4, $5, $6, $7);
			",
            &[
                &strategy_id as &(dyn ToSql + Sync),
                &EnumBlockChain::BscTestnet as &(dyn ToSql + Sync),
                &"WBNB".to_string() as &(dyn ToSql + Sync),
                &format!("{:?}", wbnb_address_on_bsc_testnet) as &(dyn ToSql + Sync),
                &"100000000".to_string() as &(dyn ToSql + Sync),
                &ts as &(dyn ToSql + Sync),
                &ts as &(dyn ToSql + Sync),
            ],
        )
        .await?;
        Ok(())
    }
    use eth_sdk::escrow_tracker::escrow::parse_escrow;
    use eth_sdk::mock_erc20::deploy_mock_erc20;
    use eth_sdk::signer::Secp256k1SecretKey;
    use eth_sdk::utils::wait_for_confirmations_simple;
    use lib::database::{connect_to_database, database_test_config, drop_and_recreate_database};
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
            blockchain: chain,
            user_address: format!("{:?}", esc.owner),
            contract_address: format!("{:?}", tx.get_to().context("no to")?),
            transaction_hash: format!("{:?}", tx.get_hash()),
            receiver_address: format!("{:?}", esc.recipient),
        })
        .await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_user_back_strategy_testnet() -> Result<()> {
        drop_and_recreate_database()?;
        let user_key = Secp256k1SecretKey::new_random();
        let conn_pool = EthereumRpcConnectionPool::new();
        let conn = conn_pool.get(EnumBlockChain::BscTestnet).await?;
        let token_addresses = BlockchainCoinAddresses::new();
        let db = connect_to_database(database_test_config()).await?;
        use eth_sdk::DEV_ACCOUNT_PRIV_KEY;
        let master_key = Secp256k1SecretKey::from_str(DEV_ACCOUNT_PRIV_KEY)
            .context("failed to parse dev account private key")?;
        let wbnb_address_on_bsc_testnet = token_addresses
            .get(EnumBlockChain::BscTestnet, EnumBlockchainCoin::WBNB)
            .ok_or_else(|| eyre!("could not find WBNB address on BSC Testnet"))?;
        let busd_address_on_bsc_testnet = token_addresses
            .get(EnumBlockChain::BscTestnet, EnumBlockchainCoin::BUSD)
            .ok_or_else(|| eyre!("could not find USDC address on BSC Testnet"))?;
        let busd_decimals = 10u64.pow(
            Erc20Token::new(conn.clone(), busd_address_on_bsc_testnet)?
                .decimals()
                .await?
                .as_u32(),
        ) as i64;

        /* create user */
        let ret = db
            .execute(FunAuthSignupReq {
                address: format!("{:?}", user_key.address()),
                email: "".to_string(),
                phone: "".to_string(),
                preferred_language: "".to_string(),
                agreed_tos: true,
                agreed_privacy: true,
                ip_address: Ipv4Addr::new(127, 0, 0, 1).into(),
                username: Some("TEST".to_string()),
                age: None,
                public_id: 1,
            })
            .await?
            .into_result()
            .context("no user signup resp")?;

        /* create strategy */
        let strategy = db
            .execute(FunUserCreateStrategyReq {
                user_id: ret.user_id,
                name: "TEST".to_string(),
                description: "TEST".to_string(),
                strategy_thesis_url: "TEST".to_string(),
                minimum_backing_amount_usd: 1.0,
                strategy_fee: 1.0,
                expert_fee: 1.0,
                agreed_tos: true,
                blockchain: EnumBlockChain::BscTestnet,
                wallet_address: format!("{:?}", Address::zero()),
            })
            .await?
            .into_result()
            .context("failed to create strategy")?;

        /* insert strategy initial token ratio */
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let timestamp_in_seconds = since_the_epoch.as_secs() as i64;
        add_strategy_initial_token_ratio(
            &db,
            strategy.strategy_id,
            wbnb_address_on_bsc_testnet,
            timestamp_in_seconds,
        )
        .await?;

        let ctx = RequestContext {
            connection_id: 0,
            user_id: ret.user_id,
            seq: 0,
            method: 0,
            log_id: 0,
            ip_addr: Ipv4Addr::new(127, 0, 0, 1).into(),
            role: EnumRole::Expert as u32,
        };

        /* deploy escrow contract */
        let escrow_contract = EscrowContract::deploy(conn.clone(), master_key.clone()).await?;

        /* make sure dev account has enough BUSD on BSC Testnet */
        /* transfer 10 BUSD to escrow contract */
        let busd_contract = Erc20Token::new(conn.clone(), busd_address_on_bsc_testnet)?;
        let transfer_tx_hash = busd_contract
            .transfer(
                &conn,
                master_key.clone(),
                escrow_contract.address(),
                U256::from(10).try_checked_mul(U256::from(busd_decimals))?,
            )
            .await?;
        wait_for_confirmations_simple(
            &conn.clone().eth(),
            transfer_tx_hash,
            Duration::from_secs(10),
            10,
        )
        .await?;

        user_back_strategy(
            &conn,
            &ctx,
            &db,
            EnumBlockChain::BscTestnet,
            U256::from(10).try_checked_mul(U256::from(busd_decimals))?,
            &token_addresses,
            strategy.strategy_id,
            EnumBlockchainCoin::BUSD,
            escrow_contract,
            &DexAddresses::new(),
            master_key,
        )
        .await?;

        /* fetch created strategy address */
        let strategy = db
            .execute(FunUserGetStrategyReq {
                strategy_id: strategy.strategy_id,
                user_id: ret.user_id,
            })
            .await?
            .into_result()
            .context("could not retrieve strategy")?;
        let sp_address = Address::from_str(
            strategy
                .evm_contract_address
                .ok_or_else(|| {
                    eyre!(
                        "could not retrieve strategy address after running back strategy on test!"
                    )
                })?
                .as_ref(),
        )?;

        /* instantiate strategy pool contract */
        let sp_contract = StrategyPoolContract::new(conn.clone(), sp_address)?;

        /* fetch user's strategy wallet address on this chain */
        let strategy_wallet_address = Address::from_str(
            &db.execute(FunUserListStrategyWalletsReq {
                user_id: ret.user_id,
                blockchain: Some(EnumBlockChain::BscTestnet),
            })
            .await?
            .into_result()
            .context("could not retrieve strategy wallet address")?
            .address,
        )?;

        /* check that SP has positive WBNB balance */
        let sp_assets = sp_contract.assets().await?;
        assert_eq!(sp_assets.len(), 1);
        assert_eq!(sp_assets[0], wbnb_address_on_bsc_testnet);
        let (sp_assets_from_another_func, sp_balances) = sp_contract.assets_and_balances().await?;
        assert_eq!(sp_assets_from_another_func.len(), 1);
        assert_eq!(sp_assets_from_another_func[0], wbnb_address_on_bsc_testnet);
        assert_eq!(sp_balances.len(), 1);
        assert!(sp_balances[0] > U256::zero());
        assert!(
            sp_contract
                .asset_balance(wbnb_address_on_bsc_testnet)
                .await?
                > U256::zero()
        );

        /* check that user's strategy wallet has shares > 9 * 1e18 */
        assert!(
            sp_contract.balance_of(strategy_wallet_address).await?
                > U256::from(9).try_checked_mul(U256::from(busd_decimals))?
        );
        /* check that SP has shares > 9 * 1e18 */
        assert!(
            sp_contract.total_supply().await?
                > U256::from(9).try_checked_mul(U256::from(busd_decimals))?
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_user_exit_strategy() -> Result<()> {
        drop_and_recreate_database()?;
        let master_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;
        let user_key = Secp256k1SecretKey::new_random();
        let conn_pool = EthereumRpcConnectionPool::new();
        let conn = conn_pool.get(EnumBlockChain::LocalNet).await?;
        let db = connect_to_database(database_test_config()).await?;

        /* create user */
        let ret = db
            .execute(FunAuthSignupReq {
                address: format!("{:?}", user_key.address()),
                email: "".to_string(),
                phone: "".to_string(),
                preferred_language: "".to_string(),
                agreed_tos: true,
                agreed_privacy: true,
                ip_address: Ipv4Addr::new(127, 0, 0, 1).into(),
                username: Some("TEST".to_string()),
                age: None,
                public_id: 1,
            })
            .await?
            .into_result()
            .context("no user signup resp")?;

        /* deploy strategy wallet contract with master key as admin */
        let strategy_wallet_contract = StrategyWalletContract::deploy(
            conn.clone(),
            master_key.clone(),
            user_key.address(),
            master_key.address(),
        )
        .await?;

        /* insert strategy wallet on this chain into database */
        db.execute(FunUserAddStrategyWalletReq {
            user_id: ret.user_id,
            blockchain: EnumBlockChain::LocalNet,
            address: format!("{:?}", strategy_wallet_contract.address()),
        })
        .await?;

        /* create strategy */
        let strategy = db
            .execute(FunUserCreateStrategyReq {
                user_id: ret.user_id,
                name: "TEST".to_string(),
                description: "TEST".to_string(),
                strategy_thesis_url: "TEST".to_string(),
                minimum_backing_amount_usd: 1.0,
                strategy_fee: 1.0,
                expert_fee: 1.0,
                agreed_tos: true,
                blockchain: EnumBlockChain::LocalNet,
                wallet_address: format!("{:?}", Address::zero()),
            })
            .await?
            .into_result()
            .context("failed to create strategy")?;

        /* deploy strategy contract */
        let strategy_contract = StrategyPoolContract::deploy(
            conn.clone(),
            master_key.clone(),
            "TEST".to_string(),
            "TEST".to_string(),
        )
        .await?;
        db.execute(FunWatcherSaveStrategyPoolContractReq {
            strategy_id: strategy.strategy_id,
            blockchain: EnumBlockChain::LocalNet,
            address: format!("{:?}", strategy_contract.address()),
        })
        .await?;

        /* deploy token contract */
        let token_contract = deploy_mock_erc20(conn.clone(), master_key.clone()).await?;

        let tokens_minted = U256::from(1000000);
        /* mint tokens for master key (simulating transferring escrow to our eoa and trading) */
        wait_for_confirmations_simple(
            &conn.eth(),
            token_contract
                .mint(
                    &conn,
                    master_key.clone(),
                    master_key.address(),
                    tokens_minted,
                )
                .await?,
            Duration::from_secs(1),
            10,
        )
        .await?;

        /* approve strategy contract for tokens */
        wait_for_confirmations_simple(
            &conn.eth(),
            token_contract
                .approve(
                    &conn,
                    master_key.clone(),
                    strategy_contract.address(),
                    tokens_minted,
                )
                .await?,
            Duration::from_secs(1),
            10,
        )
        .await?;

        let shares_minted = U256::from(1000000);
        /* deposit tokens in strategy pool to strategy wallet's address */
        let deposit_hash = strategy_contract
            .deposit(
                &conn,
                master_key.clone(),
                vec![token_contract.address],
                vec![tokens_minted],
                shares_minted,
                strategy_wallet_contract.address(),
            )
            .await?;
        wait_for_confirmations_simple(&conn.eth(), deposit_hash, Duration::from_secs(1), 10)
            .await?;

        /* insert into back strategy Ledger */
        /* here ends the back strategy simulation */
        db.execute(FunUserBackStrategyReq {
            user_id: ret.user_id,
            strategy_id: strategy.strategy_id,
            quantity: "1000000".to_string(),
            new_total_backed_quantity: "1000000".to_string(),
            old_total_backed_quantity: "0".to_string(),
            new_current_quantity: "1000000".to_string(),
            old_current_quantity: "0".to_string(),
            blockchain: EnumBlockChain::LocalNet,
            transaction_hash: format!("{:?}", deposit_hash),
            earn_sp_tokens: format!("{:?}", shares_minted),
        })
        .await?;

        /* call exit strategy */
        let ctx = RequestContext {
            connection_id: 0,
            user_id: ret.user_id,
            seq: 0,
            method: 0,
            log_id: 0,
            ip_addr: Ipv4Addr::new(127, 0, 0, 1).into(),
            role: EnumRole::User as u32,
        };
        let _exit_hash = user_exit_strategy(
            &conn,
            &ctx,
            &db,
            EnumBlockChain::LocalNet,
            strategy.strategy_id,
            Some(shares_minted),
            master_key.clone(),
        )
        .await?;

        /* check user key now has the tokens */
        assert_eq!(
            token_contract.balance_of(user_key.address()).await?,
            tokens_minted
        );

        /* check user exit strategy is in database */
        let exit_strategy = db
            .execute(FunUserListExitStrategyLedgerReq {
                user_id: ret.user_id,
                strategy_id: Some(strategy.strategy_id),
            })
            .await?
            .into_result()
            .context("no exit strategy")?;

        assert_eq!(exit_strategy.exit_quantity, shares_minted.to_string());

        Ok(())
    }

    #[tokio::test]
    async fn test_user_ethereum_deposit_refund() -> Result<()> {
        let _ = setup_logs(LogLevel::Info);
        drop_and_recreate_database()?;
        let user_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;
        let admin_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;
        let escrow_key = Secp256k1SecretKey::new_random();
        let conn_pool = EthereumRpcConnectionPool::new();
        let conn = conn_pool.get(EnumBlockChain::LocalNet).await?;
        let erc20_mock = deploy_mock_erc20(conn.clone(), admin_key.clone()).await?;
        erc20_mock
            .mint(
                &conn,
                &admin_key.key,
                user_key.address,
                U256::from(200000000000i64),
            )
            .await?;
        let eth = EthereumToken::new(conn.clone());
        eth.transfer(
            admin_key.clone(),
            escrow_key.address,
            U256::from(1e18 as i64),
        )
        .await?;
        let escrow_contract = EscrowContract::deploy(conn.clone(), &escrow_key.key).await?;

        let tx_hash = erc20_mock
            .transfer(
                &conn,
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
            role: 0,
            ip_addr: "127.0.0.1".parse()?,
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

        let _strategy = db
            .execute(FunUserCreateStrategyReq {
                user_id: ctx.user_id,
                name: "TEST".to_string(),
                description: "TEST".to_string(),
                strategy_thesis_url: "".to_string(),
                minimum_backing_amount_usd: 0.0,
                strategy_fee: 0.0,
                expert_fee: 0.0,
                agreed_tos: false,
                blockchain: EnumBlockChain::BscTestnet,
                wallet_address: format!("{:?}", Address::zero()),
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
            escrow_contract,
            U256::from(1000),
            user_key.address,
            &escrow_key.key,
            EnumBlockchainCoin::USDC,
        )
        .await?;
        Ok(())
    }
}
