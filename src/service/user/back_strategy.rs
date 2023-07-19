use eth_sdk::erc20::{approve_and_ensure_success, Erc20Token};
use eth_sdk::escrow::{transfer_asset_from_and_ensure_success, EscrowContract};
use eth_sdk::pair_paths::WorkingPancakePairPaths;
use eth_sdk::strategy_pool::{sp_deposit_to_and_ensure_success, StrategyPoolContract};
use eth_sdk::strategy_wallet::StrategyWalletContract;
use eth_sdk::v3::smart_router::{copy_trade_and_ensure_success, PancakeSmartRouterV3Contract};
use eth_sdk::StrategyPoolHeraldAddresses;
use eth_sdk::{
    build_pancake_swap, DexAddresses, EitherTransport, EthereumRpcConnection, ScaledMath,
    TransactionFetcher, CONFIRMATIONS, MAX_RETRIES, POLL_INTERVAL,
};
use eyre::*;
use gen::database::*;
use gen::model::{EnumBlockChain, EnumDex};
use lib::database::DbClient;
use lib::log::DynLogger;
use lib::toolbox::RequestContext;
use lib::types::{amount_to_display, U256};
use std::collections::HashMap;
use std::future::Future;
use tracing::{debug, info, warn};
use web3::signing::Key;
use web3::types::Address;

pub async fn deploy_wallet_contract(
    conn: &EthereumRpcConnection,
    key: impl Key + Clone,
    backer: Address,
    admin: Address,
    logger: DynLogger,
) -> Result<StrategyWalletContract<EitherTransport>> {
    info!("Deploying wallet contract");
    logger.log("Deploying wallet contract");

    let wallet =
        StrategyWalletContract::deploy(conn.clone(), key, backer, admin, logger.clone()).await?;

    info!("Deploy wallet contract success");
    logger.log(&format!("Deploying wallet contract {:?}", wallet.address()));

    Ok(wallet)
}

async fn deploy_strategy_contract(
    conn: &EthereumRpcConnection,
    key: impl Key + Clone,
    strategy_token_name: String,
    strategy_token_symbol: String,
    herald_contract_address: Address,
    logger: DynLogger,
) -> Result<StrategyPoolContract<EitherTransport>> {
    info!("Deploying strategy contract");
    logger.log("Deploying strategy contract");
    let strategy = StrategyPoolContract::deploy(
        conn.clone(),
        key,
        strategy_token_name,
        strategy_token_symbol,
        herald_contract_address,
        logger.clone(),
    )
    .await?;
    logger.log(&format!(
        "Deploying strategy contract {:?}",
        strategy.address()
    ));
    info!("Deploy strategy contract success");
    Ok(strategy)
}

async fn user_get_or_deploy_strategy_wallet(
    conn: &EthereumRpcConnection,
    ctx: &RequestContext,
    db: &DbClient,
    master_key: impl Key + Clone,
    blockchain: EnumBlockChain,
    user_wallet_address_to_receive_shares_on_this_chain: Address,
    logger: DynLogger,
    read_only: bool,
    user_supplied_wallet: Option<Address>,
) -> Result<StrategyWalletContract<EitherTransport>> {
    if let Some(user_supplied_wallet) = user_supplied_wallet {
        return StrategyWalletContract::new(conn.clone(), user_supplied_wallet.into());
    }
    match db
        .execute(FunUserListStrategyWalletsReq {
            user_id: Some(ctx.user_id),
            blockchain: Some(blockchain),
            strategy_wallet_address: None,
        })
        .await?
        .into_result()
    {
        Some(strategy_wallet_contract) => {
            /* if user has wallet on this chain, use it */
            StrategyWalletContract::new(conn.clone(), strategy_wallet_contract.address.into())
        }
        None if !read_only => {
            /* if user does not have a wallet on this chain, deploy it, and use it */
            // TODO: add admin as Address::zero() if user has opted out of having an admin
            let strategy_wallet_contract = deploy_wallet_contract(
                &conn,
                master_key.clone(),
                user_wallet_address_to_receive_shares_on_this_chain,
                master_key.address(),
                logger.clone(),
            )
            .await?;

            /* save wallet to database */
            db.execute(FunUserAddStrategyWalletReq {
                user_id: ctx.user_id,
                blockchain,
                address: strategy_wallet_contract.address().into(),
                is_platform_managed: true,
            })
            .await?;

            Ok(strategy_wallet_contract)
        }
        _ => bail!("User does not have a wallet on this chain {:?}", blockchain),
    }
}

async fn user_get_or_deploy_strategy_pool(
    conn: &EthereumRpcConnection,
    db: &DbClient,
    master_key: impl Key + Clone,
    strategy_id: i64,
    blockchain: EnumBlockChain,
    strategy_token_name: String,
    strategy_token_symbol: String,
    logger: DynLogger,
    dry_run: bool,
) -> Result<Option<(i64, StrategyPoolContract<EitherTransport>)>> {
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
    let sp_contract = match strategy_pool {
        Some(addr) => {
            let address = addr.address.into();
            (
                addr.pkey_id,
                StrategyPoolContract::new(conn.clone(), address)?,
            )
        }
        None if !dry_run => {
            /* if strategy pool doesn't exist in this chain, create it */

            let contract = deploy_strategy_contract(
                &conn,
                master_key.clone(),
                strategy_token_name,
                strategy_token_symbol,
                StrategyPoolHeraldAddresses::new()
                    .get(blockchain, ())
                    .context("could not find herald contract address in this chain")?,
                logger.clone(),
            )
            .await?;
            /* insert strategy contract address in the database */
            let resp = db
                .execute(FunUserAddStrategyPoolContractReq {
                    strategy_id,
                    blockchain,
                    address: contract.address().into(),
                })
                .await?
                .into_result()
                .context("No strategy pool contract address returned")?;

            (resp.strategy_pool_contract_id, contract)
        }
        _ => {
            warn!("Strategy pool contract not found. dry run mode");
            return Ok(None);
        }
    };
    Ok(Some(sp_contract))
}
pub struct CalculateUserBackStrategyCalculateAmountToMintResult {
    pub fees: U256,
    pub back_usdc_amount_minus_fees: U256,
    pub strategy_token_to_mint: U256,
    pub strategy_pool_active: bool,
    pub sp_assets_and_amounts: HashMap<Address, U256>,
    pub escrow_allocations_for_tokens: HashMap<Address, U256>,
    pub strategy_pool_assets_bought_for_this_backer: HashMap<Address, U256>,
    pub user_deposit_amounts: HashMap<Address, U256>,
}
pub async fn calculate_user_back_strategy_calculate_amount_to_mint<
    Fut: Future<Output = Result<U256>>,
>(
    conn: &EthereumRpcConnection,
    db: &DbClient,
    blockchain: EnumBlockChain,
    back_total_amount: U256,
    strategy_id: i64,
    token_id: i64,
    token_address: Address,
    master_key: impl Key + Clone,
    logger: DynLogger,
    dry_run: bool,
    user_id: i64,
    escrow_contract_address: Address,
    get_token_out: impl Fn(Address, U256) -> Fut,
) -> Result<CalculateUserBackStrategyCalculateAmountToMintResult> {
    /* fetch strategy */
    let strategy = db
        .execute(FunUserListStrategiesReq {
            strategy_id: Some(strategy_id),
            strategy_name: None,
            expert_id: None,
            expert_public_id: None,
            expert_name: None,
            description: None,
            blockchain: None,
            user_id,
            limit: 1,
            offset: 0,
            strategy_pool_address: None,
            approved: None,
        })
        .await?
        .into_result()
        .context("strategy is not registered in the database")?;
    /* instantiate strategy contract wrapper */
    let sp_contract = user_get_or_deploy_strategy_pool(
        &conn,
        &db,
        master_key.clone(),
        strategy_id,
        blockchain,
        strategy.strategy_name.clone(),
        strategy.strategy_name,
        logger.clone(),
        dry_run,
    )
    .await?;
    /* fetch strategy's tokens */
    let strategy_initial_ratios = db
        .execute(FunUserListStrategyInitialTokenRatiosReq {
            strategy_id,
            token_id: None,
            token_address: None,
            blockchain: Some(blockchain),
        })
        .await?
        .into_rows();
    ensure!(
        !strategy_initial_ratios.is_empty(),
        "strategy has no initial ratios"
    );
    let mut strategy_initial_token_ratios: HashMap<Address, U256> = HashMap::new();
    for x in strategy_initial_ratios.iter() {
        strategy_initial_token_ratios.insert(x.token_address.into(), x.quantity.into());
    }

    /* deduce fees from back amount */
    // TODO: use (back amount - fees) to calculate trade spenditure and SP shares
    // TODO: distribute fees for the treasury and the strategy creator
    let divide_scale = 10000;
    let fees = back_total_amount
        * (((strategy.swap_fee.unwrap_or_default()
            + strategy.platform_fee.unwrap_or_default()
            + strategy.expert_fee.unwrap_or_default())
            * divide_scale as f64) as u64)
        / divide_scale;
    ensure!(
        fees < back_total_amount,
        "fees are too high, back amount is too low"
    );
    let back_token_amount_minus_fees = back_total_amount - fees;
    logger.log(format!(
        "fees: {}, back token minus fees: {}",
        amount_to_display(fees),
        amount_to_display(back_token_amount_minus_fees)
    ));

    let mut strategy_pool_active: bool = false;
    let mut sp_assets_and_amounts: HashMap<Address, U256> = HashMap::new();
    if let Some((strategy_pool_contract_id, _sp_contract)) = sp_contract.as_ref() {
        let strategy_pool_assets = db
            .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
                strategy_pool_contract_id: Some(*strategy_pool_contract_id),
                token_address: None,
                blockchain: Some(blockchain),
                strategy_id: None,
            })
            .await?
            .into_rows();
        for sp_asset_row in &strategy_pool_assets {
            if sp_asset_row.balance > U256::zero().into() {
                strategy_pool_active = true;
            }
            sp_assets_and_amounts.insert(
                sp_asset_row.token_address.into(),
                sp_asset_row.balance.into(),
            );
        }
    }
    let token_tokens_ret = db
        .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
            strategy_pool_contract_id: None,
            strategy_id: Some(strategy_id),
            blockchain: Some(blockchain),
            token_address: Some(token_address.clone().into()),
        })
        .await?;
    let mut token_tokens = HashMap::new();
    for token_token in token_tokens_ret.into_rows() {
        token_tokens.insert(token_token.token_address.into(), token_token.balance.into());
    }
    /* calculate how much of back amount to spend on each strategy pool asset */
    let escrow_allocations_for_tokens = calculate_escrow_allocation_for_strategy_tokens(
        back_token_amount_minus_fees,
        token_address,
        token_tokens,
    )?;

    let strategy_pool_assets_bought_for_this_backer = trade_escrow_for_strategy_tokens(
        token_address,
        escrow_allocations_for_tokens.clone(),
        logger.clone(),
        get_token_out,
    )
    .await?;

    /* calculate mintage */
    let strategy_token_to_mint = match strategy_pool_active {
        false => {
            logger.log("calculating strategy tokens");
            let escrow_token_contract = Erc20Token::new(conn.clone(), token_address)?;
            let token = db
                .execute(FunUserListStrategyInitialTokenRatiosReq {
                    strategy_id,
                    token_id: Some(token_id),
                    token_address: None,
                    blockchain: None,
                })
                .await?
                .into_result()
                .context("initial token not found in strategy")?;
            info!("Calculating strategy token with easy approach");
            logger.log("calculating strategy tokens with easy approach");
            let strategy_pool_token_to_mint = calculate_sp_tokens_to_mint_easy_approach(
                token,
                back_token_amount_minus_fees,
                escrow_token_contract.decimals().await?,
            )
            .await?;
            strategy_pool_token_to_mint
        }
        true => {
            /* instantiate base token contract, and pancake contract */
            let escrow_token_contract = Erc20Token::new(conn.clone(), token_address)?;

            /* calculate strategy tokens to mint for backer */

            // WARNING: without this mintage calculation might break for nth backer
            /* get amount spent and amount bought for every strategy pool asset to calculate sp valuation for mintage */
            /* necessary because there could be a strategy pool asset that is not in initial_token_ratios */

            let mut strategy_pool_asset_last_prices_in_base_token: HashMap<Address, U256> =
                HashMap::new();
            for strategy_pool_asset in sp_assets_and_amounts.keys() {
                if let Some(amount_bought) =
                    strategy_pool_assets_bought_for_this_backer.get(strategy_pool_asset)
                {
                    /* if strategy pool asset is in initial_token_ratios, it was bought */
                    /* so use these trade values for valuation */

                    let amount_spent_on_asset = escrow_allocations_for_tokens.get(strategy_pool_asset).context("could not get amount spent for backer in strategy pool asset, even though amount bought exists")?.clone();
                    if amount_bought.is_zero() {
                        warn!(
                            "amount bought is zero, setting last price to zero {:?}",
                            strategy_pool_asset
                        );
                        strategy_pool_asset_last_prices_in_base_token
                            .insert(strategy_pool_asset.clone(), U256::zero());
                    } else {
                        strategy_pool_asset_last_prices_in_base_token.insert(
                            strategy_pool_asset.clone(),
                            amount_spent_on_asset
                                .mul_div(U256::exp10(18), amount_bought.clone())?,
                        );
                    }
                } else {
                    /* if strategy pool asset is not in initial_token_ratios, it was not bought */
                    /* fetch the most recent trade values from the database to use for valuation */
                    let last_dex_trade_row = db
                        .execute(FunWatcherListLastDexTradesForPairReq {
                            token_in_address: token_address.into(),
                            token_out_address: strategy_pool_asset.clone().into(),
                            blockchain,
                            dex: None,
                        })
                        .await?
                        .into_result()
                        .context("could not fetch last dex trade for strategy pool asset")?;

                    strategy_pool_asset_last_prices_in_base_token.insert(
                        strategy_pool_asset.clone(),
                        last_dex_trade_row
                            .amount_in
                            .mul_div(U256::exp10(18), last_dex_trade_row.amount_out.0.clone())?,
                    );
                }
            }

            // TODO: find out if we use back amount with or without fees for share calculation
            // currently calculating with back amount minus fees
            // TODO: get these values from database
            let total_strategy_tokens = sp_contract.as_ref().unwrap().1.total_supply().await?;

            calculate_sp_tokens_to_mint_nth_backer(
                total_strategy_tokens,
                sp_assets_and_amounts.clone(),
                strategy_pool_asset_last_prices_in_base_token,
                back_token_amount_minus_fees,
                escrow_token_contract.address,
            )?
        }
    };
    let user_deposit_amounts = get_user_deposit_balances_by_wallet_to_fill_value(
        &db,
        blockchain,
        user_id,
        token_id,
        back_total_amount,
        escrow_contract_address,
    )
    .await
    .context("not enough balance found in ledger for back amount")?;
    Ok(CalculateUserBackStrategyCalculateAmountToMintResult {
        fees,
        back_usdc_amount_minus_fees: back_token_amount_minus_fees,
        strategy_token_to_mint,
        strategy_pool_active,
        sp_assets_and_amounts,
        escrow_allocations_for_tokens,
        strategy_pool_assets_bought_for_this_backer,
        user_deposit_amounts,
    })
}

pub async fn user_back_strategy(
    conn: &EthereumRpcConnection,
    ctx: &RequestContext,
    db: &DbClient,
    blockchain: EnumBlockChain,
    user_id: i64,
    back_token_amount: U256,
    strategy_id: i64,
    token_id: i64,
    token_address: Address,
    escrow_contract: EscrowContract<EitherTransport>,
    dex_addresses: &DexAddresses,
    master_key: impl Key + Clone,
    strategy_wallet: Option<Address>,
    logger: DynLogger,
    pancake_paths: &WorkingPancakePairPaths,
) -> Result<()> {
    logger.log(format!(
        "checking back amount {}",
        amount_to_display(back_token_amount)
    ));
    if back_token_amount == U256::zero() {
        bail!("back zero amount");
    }

    /* check if user has enough balance */
    // TODO: add user balance to the database
    // TODO: might call balanceOf of these ERC20 contracts if database is not working correctly
    let user_balance = db
        .execute(FunUserListUserDepositWithdrawBalanceReq {
            limit: 1,
            offset: 0,
            user_id,
            blockchain: Some(blockchain),
            token_address: None,
            token_id: Some(token_id),
            escrow_contract_address: Some(escrow_contract.address().into()),
        })
        .await?;
    debug!("Fetched {} rows of user balance", user_balance.len());
    let user_balance_ret = user_balance.into_result().context("insufficient balance")?;
    let user_balance: U256 = user_balance_ret.balance.into();
    if user_balance < back_token_amount {
        bail!("insufficient balance");
    }

    /* get amount deposited by whitelisted wallets necessary for back amount */
    let wallet_deposit_amounts = get_user_deposit_balances_by_wallet_to_fill_value(
        &db,
        blockchain,
        user_id,
        token_id,
        back_token_amount,
        escrow_contract.address(),
    )
    .await
    .context("not enough balance found in ledger for back amount")?;

    let new_user_balance = user_balance - back_token_amount;
    let mut success = false;
    for _ in 0..3 {
        let ret = db
            .execute(FunUserUpdateUserDepositWithdrawBalanceReq {
                deposit_withdraw_balance_id: user_balance_ret.deposit_withdraw_balance_id,
                old_balance: user_balance.into(),
                new_balance: new_user_balance.into(),
            })
            .await?
            .into_result()
            .context("could not update user balance")?;
        if ret.updated {
            success = true;
            break;
        } else {
            continue;
        }
    }
    if !success {
        bail!("could not update user balance, error in database");
    }
    /* fetch user address to receive shares */
    // TODO: fetch the correct address where user desires to receive shares on this chain
    // since users can have multiple addresses, this information is critical
    // for now, we fetch the "address" field from the user table
    let user_wallet_address_to_receive_shares_on_this_chain = db
        .execute(FunAdminListUsersReq {
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
        .address
        .into();
    logger.log("user_get_or_deploy_strategy_wallet");
    /* instantiate strategy wallet contract wrapper */
    let strategy_wallet_contract = user_get_or_deploy_strategy_wallet(
        &conn,
        &ctx,
        &db,
        master_key.clone(),
        blockchain,
        user_wallet_address_to_receive_shares_on_this_chain,
        logger.clone(),
        false,
        strategy_wallet,
    )
    .await?;

    /* fetch strategy */
    let strategy = db
        .execute(FunUserListStrategiesReq {
            strategy_id: Some(strategy_id),
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
        .context("strategy is not registered in the database")?;

    logger.log("calculating fees");

    /* instantiate strategy contract wrapper */
    let (strategy_pool_contract_id, sp_contract) = user_get_or_deploy_strategy_pool(
        &conn,
        &db,
        master_key.clone(),
        strategy_id,
        blockchain,
        strategy.strategy_name.clone(),
        strategy.strategy_name,
        logger.clone(),
        false,
    )
    .await?
    .unwrap();
    logger.log("calculating strategy tokens");

    let escrow_token_contract = Erc20Token::new(conn.clone(), token_address)?;

    /* instantiate pancake contract */
    let pancake_contract = PancakeSmartRouterV3Contract::new(
        conn.clone(),
        dex_addresses
            .get(blockchain, EnumDex::PancakeSwap)
            .ok_or_else(|| eyre!("pancake swap not available on this chain"))?,
    )?;

    //TODO: make some way of replaying the correct transactions in case of failure in the middle of the backing process

    // FIXME: we should do it in escrow pending contract or somewhere
    /* transfer escrow to our EOA */
    info!(
        "transfer escrow to our EOA {:?} for trading",
        master_key.address()
    );

    /* for each wallet that deposited to add to back_token_amount */
    for (wallet, amount) in wallet_deposit_amounts {
        /* transfer from escrow contract to pending wallet */
        transfer_asset_from_and_ensure_success(
            escrow_contract.clone(),
            &conn,
            CONFIRMATIONS,
            MAX_RETRIES,
            POLL_INTERVAL,
            master_key.clone(),
            wallet.clone(),
            token_address,
            amount.into(),
            master_key.address(),
            logger.clone(),
        )
        .await?;

        /* reduce the value used from ledger */
        db.execute(FunUserAddUserDepositWithdrawLedgerEntryReq {
            user_id,
            blockchain,
            user_address: wallet.clone().into(),
            receiver_address: Default::default(),
            quantity: amount.clone().into(),
            transaction_hash: Default::default(),
            is_deposit: false,
            is_back: true,
            token_address: token_address.into(),
            escrow_contract_address: escrow_contract.address().into(),
            is_withdraw: false,
        })
        .await?;
    }

    /* approve pancakeswap to trade escrow token */
    info!("approve pancakeswap to trade escrow token");
    approve_and_ensure_success(
        escrow_token_contract,
        &conn,
        CONFIRMATIONS,
        MAX_RETRIES,
        POLL_INTERVAL,
        master_key.clone(),
        pancake_contract.address(),
        back_token_amount,
        logger.clone(),
    )
    .await?;

    /* trade escrow token for strategy's tokens */
    info!("trade escrow token for strategy's tokens");
    let pancake_trade_parser = build_pancake_swap()?;
    let get_out_amount = |out_token, amount| {
        let db = db.clone();
        let conn = conn.clone();
        let master_key = master_key.clone();
        let pancake_contract = pancake_contract.clone();
        let logger = logger.clone();
        let pancake_trade_parser = pancake_trade_parser.clone();
        let sp_contract = sp_contract.clone();
        async move {
            if token_address == out_token {
                logger.log(format!(
                    "approving {} token {:?} to strategy pool contract",
                    amount_to_display(amount),
                    out_token
                ));
                approve_and_ensure_success(
                    Erc20Token::new(conn.clone(), out_token)?,
                    &conn,
                    CONFIRMATIONS,
                    MAX_RETRIES,
                    POLL_INTERVAL,
                    master_key.clone(),
                    sp_contract.address(),
                    amount,
                    logger.clone(),
                )
                .await?;
                Ok(amount)
            } else {
                let pancake_path_set = pancake_paths
                    .get_pair_by_address(blockchain, token_address, out_token)
                    .await?;
                let trade_hash = copy_trade_and_ensure_success(
                    &pancake_contract,
                    &conn,
                    CONFIRMATIONS,
                    MAX_RETRIES,
                    POLL_INTERVAL,
                    master_key.clone(),
                    &pancake_path_set,
                    amount,
                    U256::one(), // TODO: find a way to estimate amount out
                    logger.clone(),
                )
                .await?;

                let trade = pancake_trade_parser.parse_trade(
                    &TransactionFetcher::new_and_assume_ready(trade_hash.transaction_hash, &conn)
                        .await?,
                    blockchain,
                )?;

                /* update last dex trade cache table */
                db.execute(FunWatcherUpsertLastDexTradeForPairReq {
                    transaction_hash: trade_hash.transaction_hash.into(),
                    blockchain: blockchain.into(),
                    dex: EnumDex::PancakeSwap,
                    token_in_address: token_address.into(),
                    token_out_address: out_token.into(),
                    amount_in: trade.amount_in.into(),
                    amount_out: trade.amount_out.into(),
                })
                .await?;
                logger.log(format!(
                    "approving {} token {:?} to strategy pool contract",
                    amount_to_display(trade.amount_out),
                    out_token
                ));
                approve_and_ensure_success(
                    Erc20Token::new(conn.clone(), out_token)?,
                    &conn,
                    CONFIRMATIONS,
                    MAX_RETRIES,
                    POLL_INTERVAL,
                    master_key.clone(),
                    sp_contract.address(),
                    trade.amount_out,
                    logger.clone(),
                )
                .await?;
                Ok(trade.amount_out)
            }
        }
    };

    let CalculateUserBackStrategyCalculateAmountToMintResult {
        back_usdc_amount_minus_fees,
        /* TODO: fees are now in the pending wallet, we should add it to the database */
        /* TODO: add table to register treasury fees and strategy fees */
        fees,
        // we discard this value because it's not really exactly the value
        strategy_token_to_mint,
        strategy_pool_active,
        sp_assets_and_amounts,
        escrow_allocations_for_tokens,
        strategy_pool_assets_bought_for_this_backer,
        ..
    } = calculate_user_back_strategy_calculate_amount_to_mint(
        conn,
        db,
        blockchain,
        back_token_amount,
        strategy_id,
        token_id,
        token_address,
        master_key.clone(),
        logger.clone(),
        false,
        user_id,
        escrow_contract.address(),
        get_out_amount,
    )
    .await?;

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

    /* mint strategy pool token to strategy wallet contract */
    let deposit_transaction_hash = sp_deposit_to_and_ensure_success(
        sp_contract,
        &conn,
        CONFIRMATIONS,
        MAX_RETRIES,
        POLL_INTERVAL,
        master_key.clone(),
        strategy_pool_assets_bought_for_this_backer.clone(),
        strategy_token_to_mint,
        strategy_wallet_contract.address(),
        logger.clone(),
    )
    .await?;

    db.execute(FunWatcherUpsertUserStrategyBalanceReq {
        user_id: ctx.user_id,
        strategy_id,
        blockchain,
        old_balance: user_strategy_balance,
        new_balance: (*user_strategy_balance + strategy_token_to_mint).into(),
    })
    .await?;
    for (token, amount) in strategy_pool_assets_bought_for_this_backer.clone() {
        /* update strategy pool contract asset balances & ledger */
        let sp_asset_token = db
            .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
                strategy_pool_contract_id: Some(strategy_pool_contract_id),
                token_address: Some(token.into()),
                blockchain: Some(blockchain),
                strategy_id: None,
            })
            .await?
            .into_result();
        let sp_asset_token_out_new_balance = match sp_asset_token {
            Some(token_out) => (*token_out.balance).try_checked_add(amount)?,
            None => amount,
        };
        db.execute(FunWatcherUpsertStrategyPoolContractAssetBalanceReq {
            strategy_pool_contract_id,
            token_address: token.into(),
            blockchain,
            new_balance: sp_asset_token_out_new_balance.into(),
        })
        .await?;

        db.execute(FunUserAddStrategyPoolContractAssetLedgerEntryReq {
            strategy_pool_contract_id,
            token_address: token.into(),
            blockchain,
            amount: amount.into(),
            transaction_hash: deposit_transaction_hash.into(),
            is_add: true,
        })
        .await?;

        /* update per-user strategy pool contract asset balances & ledger */
        let strategy_wallet_row = db
            .execute(FunUserListStrategyWalletsReq {
                user_id: Some(ctx.user_id),
                blockchain: Some(blockchain),
                strategy_wallet_address: None,
            })
            .await?
            .into_result()
            .context("could not fetch strategy wallet after backing")?;

        match db
            .execute(FunUserListUserStrategyPoolContractAssetBalancesReq {
                strategy_pool_contract_id: Some(strategy_pool_contract_id),
                user_id: Some(ctx.user_id),
                strategy_wallet_id: Some(strategy_wallet_row.wallet_id),
                token_address: Some(token.into()),
                blockchain: Some(blockchain),
            })
            .await?
            .into_result()
        {
            Some(existing_amount) => {
                let old_amount: U256 = existing_amount.balance.into();
                let new_amount = old_amount.try_checked_add(amount)?;
                db.execute(FunUserUpsertUserStrategyPoolContractAssetBalanceReq {
                    strategy_pool_contract_id,
                    strategy_wallet_id: strategy_wallet_row.wallet_id,
                    token_address: token.into(),
                    blockchain,
                    old_balance: old_amount.into(),
                    new_balance: new_amount.into(),
                })
                .await?;
            }
            None => {
                db.execute(FunUserUpsertUserStrategyPoolContractAssetBalanceReq {
                    strategy_pool_contract_id,
                    strategy_wallet_id: strategy_wallet_row.wallet_id,
                    token_address: token.into(),
                    blockchain,
                    old_balance: U256::zero().into(),
                    new_balance: amount.into(),
                })
                .await?;
            }
        }
        db.execute(FunUserAddUserStrategyPoolContractAssetLedgerEntryReq {
            strategy_wallet_id: strategy_wallet_row.wallet_id,
            strategy_pool_contract_id,
            token_address: token.into(),
            amount: amount.into(),
            is_add: true,
            blockchain,
        })
        .await?;
    }

    let ret = db
        .execute(FunUserBackStrategyReq {
            user_id: ctx.user_id,
            strategy_id: strategy.strategy_id,
            quantity: back_token_amount.into(),
            new_total_backed_quantity: (*strategy.total_backed_usdc + back_token_amount).into(),
            old_total_backed_quantity: strategy.total_backed_usdc,
            new_current_quantity: (*strategy.current_usdc + back_token_amount).into(),
            old_current_quantity: strategy.current_usdc,
            blockchain,
            transaction_hash: deposit_transaction_hash.into(),
            earn_sp_tokens: strategy_token_to_mint.into(),
        })
        .await?
        .into_result()
        .context("No record")?;
    if !ret.success {
        bail!(
            "User back strategy not successful due to other clients updated record at the same time"
        )
    }
    logger.log(format!(
        "Everything deployed successfully for Strategy {:?}. Spent {} tokens. got {} SP tokens",
        strategy_id,
        amount_to_display(back_token_amount),
        amount_to_display(strategy_token_to_mint)
    ));
    Ok(())
}

pub async fn get_user_deposit_balances_by_wallet_to_fill_value(
    db: &DbClient,
    chain: EnumBlockChain,
    user_id: i64,
    token_id: i64,
    value_to_fill: U256,
    escrow_contract_address: Address,
) -> Result<HashMap<Address, U256>> {
    let wallet_positive_asset_balances = db
        .execute(FunUserCalculateUserEscrowBalanceFromLedgerReq {
            user_id: user_id,
            blockchain: chain,
            token_id: token_id,
            wallet_address: None,
            escrow_contract_address: escrow_contract_address.into(),
        })
        .await?
        .into_rows();

    let mut wallet_deposit_amounts: HashMap<Address, U256> = HashMap::new();
    let mut total_amount_found: U256 = U256::zero();
    for wallet_balance_row in wallet_positive_asset_balances {
        let wallet_balance: U256 = wallet_balance_row.balance.into();
        if wallet_balance == U256::zero() {
            continue;
        }
        if total_amount_found == value_to_fill {
            /* if entire amount was filled, the wallet balances fetched so far are enough */
            break;
        }
        /* if entire amount was not filled, keep looking for other positive balances from wallets */

        let necessary_amount_to_fill = value_to_fill.try_checked_sub(total_amount_found)?;
        if wallet_balance >= necessary_amount_to_fill {
            /* if the deposited amount of this wallet can fill the rest for the refund amount */
            /* use only the necessary */

            wallet_deposit_amounts.insert(
                wallet_balance_row.wallet_address.into(),
                necessary_amount_to_fill,
            );
            total_amount_found = total_amount_found.try_checked_add(necessary_amount_to_fill)?;
        } else {
            /* if the deposited amount of this wallet cannot fill the rest for the refund amount */
            /* use entire amount */

            wallet_deposit_amounts.insert(wallet_balance_row.wallet_address.into(), wallet_balance);
            total_amount_found = total_amount_found.try_checked_add(wallet_balance)?;
        }
    }

    if total_amount_found < value_to_fill {
        bail!(
            "not enough user balance to fill the amount: {} < {}",
            amount_to_display(total_amount_found),
            amount_to_display(value_to_fill)
        );
    }

    Ok(wallet_deposit_amounts)
}

fn calculate_escrow_allocation_for_strategy_tokens(
    escrow_amount: U256,
    escrow_token_address: Address,
    strategy_token_ratios: HashMap<Address, U256>,
) -> Result<HashMap<Address, U256>> {
    // TODO: should we scale it by value of tokens?
    let total_token_numbers: U256 = strategy_token_ratios
        .values()
        .fold(U256::zero(), |acc, x| acc + x);
    /* calculates how much of escrow to spend on each strategy token */
    /* allocation = (initial_strategy_token_amount * escrow_amount) / total_initial_strategy_token_amounts */
    let mut escrow_allocations: HashMap<Address, U256> = HashMap::new();
    if total_token_numbers.is_zero() {
        escrow_allocations.insert(escrow_token_address, escrow_amount);
    } else {
        for (token_address, token_amount) in strategy_token_ratios {
            let escrow_allocation = token_amount.mul_div(escrow_amount, total_token_numbers)?;
            escrow_allocations.insert(token_address, escrow_allocation);
        }
    }
    Ok(escrow_allocations)
}

pub async fn calculate_sp_tokens_to_mint_easy_approach(
    token_ratio: FunUserListStrategyInitialTokenRatiosRespRow,
    escrow_amount: U256,
    escrow_decimals: U256,
) -> Result<U256> {
    let relative_quantity = token_ratio.quantity;
    info!(
        "calculate_sp_tokens_to_mint_easy_approach {:?} {:?} {:?}",
        escrow_amount, relative_quantity, escrow_decimals
    );
    let result = escrow_amount
        .mul_div(
            relative_quantity.into(),
            U256::exp10(escrow_decimals.as_u32() as _),
        )
        .context("calculate_sp_tokens_to_mint_easy_approach")?;
    info!("Calculating strategy token with easy approach done");

    Ok(result)
}

async fn trade_escrow_for_strategy_tokens<Fut: Future<Output = Result<U256>>>(
    escrow_token_address: Address,
    tokens_and_amounts_to_buy: HashMap<Address, U256>,
    logger: DynLogger,
    get_token_out: impl Fn(Address, U256) -> Fut,
) -> Result<HashMap<Address, U256>> {
    /* buys tokens and amounts and returns a vector or bought tokens and amounts out */
    let mut deposit_amounts: HashMap<Address, U256> = HashMap::new();

    for (token_address, amount_to_spend_on_it) in tokens_and_amounts_to_buy {
        logger.log(format!(
            "Trading {} for {}",
            token_address, escrow_token_address
        ));
        let output_amount = get_token_out(token_address, amount_to_spend_on_it).await?;
        deposit_amounts.insert(token_address, output_amount);
    }
    Ok(deposit_amounts)
}

fn calculate_sp_tokens_to_mint_nth_backer(
    strategy_token_total_supply: U256,
    strategy_pool_asset_balances: HashMap<Address, U256>,
    token_prices: HashMap<Address, U256>,
    base_token_actual_amount: U256,
    deposit_asset: Address,
) -> Result<U256> {
    ensure!(
        strategy_pool_asset_balances.len() > 0,
        "strategy pool asset balances must not be empty"
    );
    // sp tokens to mint = actual value of backing / (total value / total supply)
    //                   = actual value of backing * total supply / total value
    /* calculate strategy pool assets total value in base tokens based on the price paid on assets */
    let mut strategy_pool_assets_total_value = U256::zero();
    for (strategy_pool_asset, strategy_pool_asset_amount) in strategy_pool_asset_balances.iter() {
        /* get base token value of one asset */
        /* i.e. asset price in base tokens */
        let asset_price_in_base_tokens = token_prices
            .get(strategy_pool_asset)
            .context("could not get token price")?;

        /* get value of strategy pool asset amount in base tokens */
        let strategy_pool_asset_value =
            strategy_pool_asset_amount.mul_div(*asset_price_in_base_tokens, U256::exp10(18))?;

        /* add value of strategy pool asset to total value */
        strategy_pool_assets_total_value =
            strategy_pool_assets_total_value.try_checked_add(strategy_pool_asset_value)?;
    }

    /* calculate ratio as total strategy pool value / total supply */
    /* i.e. the share value of one base token */
    /* i.e. the base token price in shares */
    let ratio = strategy_token_total_supply
        .div_as_f64(strategy_pool_assets_total_value)
        .context("calculate_sp_tokens_to_mint_nth_backer calculating ratio")?;
    let deposit_asset_price_in_base_token = token_prices
        .get(&deposit_asset)
        .context("could not find amount spend on asset to calculate mintage")?;
    let base_token_actual_value =
        base_token_actual_amount.mul_div(*deposit_asset_price_in_base_token, U256::exp10(18))?;
    /* calculate strategy pool tokens to mint as actual_amount * ratio */
    Ok(base_token_actual_value.mul_f64(ratio)?)
}

#[cfg(test)]
mod tests {
    use super::super::method::user_exit_strategy;
    use super::*;
    use crate::method::on_user_request_refund;
    use eth_sdk::erc20::Erc20Token;
    use eth_sdk::escrow_tracker::escrow::parse_escrow;
    use eth_sdk::mock_erc20::deploy_mock_erc20;
    use eth_sdk::signer::Secp256k1SecretKey;
    use eth_sdk::utils::wait_for_confirmations_simple;
    use eth_sdk::{
        BlockchainCoinAddresses, EthereumRpcConnectionPool, EthereumToken, TransactionReady,
        ANVIL_PRIV_KEY_1, ANVIL_PRIV_KEY_2,
    };
    use gen::database::{
        FunAdminAddEscrowContractAddressReq, FunAdminAddEscrowTokenContractAddressReq,
        FunAuthSignupReq, FunUserAddStrategyInitialTokenRatioReq, FunUserCreateStrategyReq,
        FunUserListEscrowTokenContractAddressReq, FunUserListExitStrategyLedgerReq,
        FunWatcherSaveStrategyPoolContractReq,
    };
    use gen::model::{EnumBlockchainCoin, EnumRole};
    use lib::database::{connect_to_database, database_test_config, drop_and_recreate_database};
    use lib::log::{setup_logs, LogLevel};
    use std::net::Ipv4Addr;
    use std::str::FromStr;
    use std::time::Duration;
    use std::{assert_eq, format, vec};

    pub async fn add_strategy_initial_token_ratio(
        db: &DbClient,
        strategy_id: i64,
        wbnb_address_on_bsc_testnet: Address,
        ts: i64,
    ) -> Result<()> {
        db.execute(FunAdminAddEscrowTokenContractAddressReq {
            pkey_id: 666,
            symbol: "WBNB".to_string(),
            short_name: "WBNB".to_string(),
            description: "WBNB".to_string(),
            address: wbnb_address_on_bsc_testnet.into(),
            blockchain: EnumBlockChain::BscTestnet,
            is_stablecoin: false,
        })
        .await?;
        db.execute(FunUserAddStrategyInitialTokenRatioReq {
            strategy_id,
            token_id: 666,
            quantity: U256::from_dec_str("100000000")?.into(),
        })
        .await?;

        Ok(())
    }
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
                expert_fee: 1.0,
                agreed_tos: true,
                blockchain: EnumBlockChain::BscTestnet,
                wallet_address: Address::zero().into(),
                swap_fee: 0.0,
            })
            .await?
            .into_result()
            .context("failed to create strategy")?;

        /* insert strategy initial token ratio */
        use crate::back_strategy::user_back_strategy;
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

        let token = db
            .execute(FunUserListEscrowTokenContractAddressReq {
                limit: 1,
                offset: 0,
                blockchain: Some(EnumBlockChain::BscTestnet),
                token_id: None,
                address: Some(busd_address_on_bsc_testnet.into()),
                symbol: None,
                is_stablecoin: None,
            })
            .await?
            .into_result()
            .context("no token")?;

        user_back_strategy(
            &conn,
            &ctx,
            &db,
            EnumBlockChain::BscTestnet,
            ret.user_id,
            U256::from(10).try_checked_mul(U256::from(busd_decimals))?,
            strategy.strategy_id,
            token.token_id,
            busd_address_on_bsc_testnet,
            escrow_contract,
            &DexAddresses::new(),
            master_key,
            cmc,
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
                user_id: Some(ret.user_id),
                blockchain: Some(EnumBlockChain::BscTestnet),
                strategy_wallet_address: None,
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
            DynLogger::empty(),
        )
        .await?;

        /* insert strategy wallet on this chain into database */
        db.execute(FunUserAddStrategyWalletReq {
            user_id: ret.user_id,
            blockchain: EnumBlockChain::LocalNet,
            address: strategy_wallet_contract.address().into(),
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
                wallet_address: Address::zero().into(),
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
        let strategy_pool_contract_ret = db
            .execute(FunWatcherSaveStrategyPoolContractReq {
                strategy_id: strategy.strategy_id,
                blockchain: EnumBlockChain::LocalNet,
                address: strategy_contract.address().into(),
            })
            .await?
            .into_result()
            .context("could not save strategy pool contract to database")?;

        /* deploy token contract */
        let token_contract = deploy_mock_erc20(conn.clone(), master_key.clone()).await?;

        /* add token to database */
        let add_token_ret = db
            .execute(FunAdminAddEscrowContractAddressReq {
                pkey_id: 1,
                blockchain: EnumBlockChain::LocalNet,
                address: token_contract.address.into(),
            })
            .await?
            .into_result()
            .context("could not add token to database")?;

        /* mint tokens for master key (simulating transferring escrow to our eoa and trading) */
        let tokens_minted = U256::from(1000000);
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

        /* deposit tokens in strategy pool to strategy wallet's address */
        let strategy_tokens_minted = U256::from(1000000);
        let deposit_hash = strategy_contract
            .deposit(
                &conn,
                master_key.clone(),
                vec![token_contract.address],
                vec![tokens_minted],
                strategy_tokens_minted,
                strategy_wallet_contract.address(),
            )
            .await?;
        wait_for_confirmations_simple(&conn.eth(), deposit_hash, Duration::from_secs(1), 10)
            .await?;

        /* insert into strategy pool contract balance table */
        db.execute(FunWatcherUpsertStrategyPoolContractAssetBalanceReq {
            strategy_pool_contract_id: strategy_pool_contract_ret.pkey_id,
            token_address: token_contract.address.into(),
            blockchain: EnumBlockChain::LocalNet,
            new_balance: tokens_minted.into(),
        })
        .await?;

        /* insert into back strategy Ledger */
        /* here ends the back strategy simulation */
        db.execute(FunUserBackStrategyReq {
            user_id: ret.user_id,
            strategy_id: strategy.strategy_id,
            quantity: U256::from(1000000).into(),
            new_total_backed_quantity: U256::from(1000000).into(),
            old_total_backed_quantity: U256::zero().into(),
            new_current_quantity: U256::from(1000000).into(),
            old_current_quantity: U256::zero().into(),
            blockchain: EnumBlockChain::LocalNet,
            transaction_hash: deposit_hash.into(),
            earn_sp_tokens: strategy_tokens_minted.into(),
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
            Some(strategy_tokens_minted),
            master_key.clone(),
        )
        .await?;

        /* check user key now has the tokens */
        assert_eq!(
            token_contract.balance_of(user_key.address()).await?,
            tokens_minted
        );

        /* check user exit strategy is in back exit ledger */
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
