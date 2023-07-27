use api::cmc::CoinMarketCap;
use eth_sdk::erc20::Erc20Token;
use eth_sdk::escrow::EscrowContract;
use eth_sdk::execute_transaction_and_ensure_success;
use eth_sdk::pancake_swap::execute::PancakeSmartRouterContract;
use eth_sdk::pancake_swap::pair_paths::WorkingPancakePairPaths;
use eth_sdk::pancake_swap::parse::get_pancake_swap_parser;
use eth_sdk::strategy_pool::StrategyPoolContract;
use eth_sdk::strategy_wallet::StrategyWalletContract;
use eth_sdk::utils::{decimal_to_u256, u256_to_decimal};
use eth_sdk::StrategyPoolHeraldAddresses;
use eth_sdk::{
    DexAddresses, EitherTransport, EthereumRpcConnection, TransactionFetcher, CONFIRMATIONS,
    MAX_RETRIES, POLL_INTERVAL,
};
use execution_engine::copy_trade::{
    calculate_copy_trade_plan, fetch_listened_wallet_asset_balances_and_decimals,
    fetch_strategy_pool_contract_asset_balances_and_decimals, get_token_prices,
};
use eyre::*;
use gen::database::*;
use gen::model::{EnumBlockChain, EnumDex};
use itertools::Itertools;
use lib::database::DbClient;
use lib::log::DynLogger;
use lib::toolbox::RequestContext;
use lib::types::U256;
use num_traits::{FromPrimitive, One, Zero};
use rust_decimal::Decimal;
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
    pub fees: Decimal,
    pub back_amount_minus_fees: Decimal,
    pub strategy_token_to_mint: Decimal,
    pub strategy_pool_active: bool,
    pub sp_assets_and_amounts: HashMap<Address, Decimal>,
    pub escrow_allocations_for_tokens: HashMap<Address, Decimal>,
    pub strategy_pool_assets_bought_for_this_backer: HashMap<Address, Decimal>,
    pub token_decimals: HashMap<Address, u32>,
}
pub async fn calculate_user_back_strategy_calculate_amount_to_mint<
    Fut: Future<Output = Result<Decimal>>,
>(
    conn: &EthereumRpcConnection,
    db: &DbClient,
    blockchain: EnumBlockChain,
    back_total_amount: Decimal,
    strategy_id: i64,
    token_id: i64,
    token_address: Address,
    master_key: impl Key + Clone,
    logger: DynLogger,
    dry_run: bool,
    user_id: i64,
    _escrow_contract_address: Address,
    get_token_out: impl Fn(Address, Decimal, u32) -> Fut,
    cmc: &CoinMarketCap,
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
    let back_token = db
        .execute(FunUserListEscrowTokenContractAddressReq {
            limit: 1,
            offset: 0,
            token_id: Some(token_id),
            blockchain: None,
            address: None,
            symbol: None,
            is_stablecoin: None,
        })
        .await?
        .into_result()
        .with_context(|| {
            format!(
                "could not find token contract address for token id {}",
                token_id
            )
        })?;
    let back_token_decimals = back_token.decimals as u32;

    /* deduce fees from back amount */
    // TODO: use (back amount - fees) to calculate trade spenditure and SP shares
    // TODO: distribute fees for the treasury and the strategy creator
    let fees = back_total_amount
        * Decimal::from_f64(
            strategy.swap_fee.unwrap_or_default()
                + strategy.platform_fee.unwrap_or_default()
                + strategy.expert_fee.unwrap_or_default(),
        )
        .unwrap();
    ensure!(
        fees < back_total_amount,
        "fees are too high, back amount is too low"
    );
    let back_token_amount_minus_fees = back_total_amount - fees;
    logger.log(format!(
        "fees: {}, back token minus fees: {}",
        fees, back_token_amount_minus_fees
    ));

    let (sp_assets_and_amounts, sp_assets_and_decimals) =
        fetch_strategy_pool_contract_asset_balances_and_decimals(&db, blockchain, strategy_id)
            .await?;
    let strategy_pool_active = sp_assets_and_amounts
        .values()
        .find(|x| **x > Decimal::zero())
        .is_some();
    let (mut expert_asset_amounts, mut expert_asset_amounts_decimals) =
        fetch_listened_wallet_asset_balances_and_decimals(db, blockchain, strategy_id).await?;
    if expert_asset_amounts.is_empty() {
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
        for x in strategy_initial_ratios.iter() {
            expert_asset_amounts_decimals.insert(x.token_address.into(), x.token_decimals as u32);
            expert_asset_amounts.insert(x.token_address.into(), x.quantity);
        }
    }
    logger.log(format!(
        "trading result should follow ratios as {:?}",
        expert_asset_amounts
    ));

    let tokens = sp_assets_and_amounts
        .keys()
        .chain(expert_asset_amounts.keys())
        .chain(&[token_address])
        .unique()
        .cloned()
        .collect::<Vec<_>>();
    let token_prices = get_token_prices(db, cmc, tokens.clone()).await?;

    let token_decimals = sp_assets_and_decimals
        .into_iter()
        .chain(expert_asset_amounts_decimals.into_iter())
        .chain([(token_address, back_token_decimals)])
        .unique()
        .collect::<HashMap<_, _>>();

    /* calculate how much of back amount to spend on each strategy pool asset */
    let mut escrow_allocations_for_tokens = calculate_copy_trade_plan(
        blockchain,
        expert_asset_amounts,
        {
            // TODO: use sp_assets_and_amounts but it may not work
            // let mut assets = sp_assets_and_amounts.clone();
            let mut assets = HashMap::new();
            *assets.entry(token_address).or_default() += back_token_amount_minus_fees;
            assets
        },
        token_prices.clone(),
        token_decimals.clone(),
    )?;
    escrow_allocations_for_tokens.trades = escrow_allocations_for_tokens
        .trades
        .into_iter()
        .filter(|x| x.token_in == token_address)
        .collect();
    info!(
        "escrow_allocations_for_tokens={:?}",
        escrow_allocations_for_tokens
    );
    let tokens_and_escrow_token_to_spend: HashMap<_, _> = escrow_allocations_for_tokens
        .trades
        .iter()
        .map(|x| (x.token_out, x.amount_in))
        .collect();
    let strategy_pool_assets_bought_for_this_backer = trade_escrow_for_strategy_tokens(
        token_address,
        tokens_and_escrow_token_to_spend.clone(),
        token_decimals.clone(),
        logger.clone(),
        get_token_out,
    )
    .await?;

    /* calculate mintage */
    let strategy_token_to_mint = match strategy_pool_active {
        false => {
            logger.log("calculating strategy tokens");
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
            let strategy_pool_token_to_mint =
                calculate_sp_tokens_to_mint_easy_approach(token, back_token_amount_minus_fees)
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

            let mut updated_token_prices: HashMap<Address, Decimal> = token_prices
                .iter()
                .map(|x| (*x.0, Decimal::from_f64(*x.1).unwrap()))
                .collect();
            for strategy_pool_asset in sp_assets_and_amounts.keys() {
                if let Some(amount_bought) =
                    strategy_pool_assets_bought_for_this_backer.get(strategy_pool_asset)
                {
                    /* if strategy pool asset is in initial_token_ratios, it was bought */
                    /* so use these trade values for valuation */

                    if amount_bought.is_zero() {
                        warn!(
                            "amount bought is zero, setting last price to zero {:?}",
                            strategy_pool_asset
                        );
                        updated_token_prices.insert(strategy_pool_asset.clone(), Decimal::zero());
                    } else {
                        let amount_spent_on_asset = tokens_and_escrow_token_to_spend.get(strategy_pool_asset).context("could not get amount spent for backer in strategy pool asset, even though amount bought exists")?.clone();
                        updated_token_prices.insert(
                            strategy_pool_asset.clone(),
                            amount_spent_on_asset / amount_bought.clone(),
                        );
                    }
                } else {
                    /* if strategy pool asset is not in initial_token_ratios, it was not bought */
                    /* fetch the most recent trade values from the database to use for valuation */
                    if let Some(last_dex_trade_row) = db
                        .execute(FunWatcherListLastDexTradesForPairReq {
                            token_in_address: token_address.into(),
                            token_out_address: strategy_pool_asset.clone().into(),
                            blockchain,
                            dex: None,
                        })
                        .await?
                        .into_result()
                    {
                        updated_token_prices.insert(
                            strategy_pool_asset.clone(),
                            last_dex_trade_row.amount_in / last_dex_trade_row.amount_out,
                        );
                    }
                }
            }

            // TODO: find out if we use back amount with or without fees for share calculation
            // currently calculating with back amount minus fees
            // TODO: get these values from database
            let total_strategy_tokens = sp_contract.as_ref().unwrap().1.total_supply().await?;
            let decimals = sp_contract.as_ref().unwrap().1.decimals().await?;

            calculate_sp_tokens_to_mint_nth_backer(
                u256_to_decimal(total_strategy_tokens, decimals.as_u32()),
                sp_assets_and_amounts.clone(),
                updated_token_prices,
                back_token_amount_minus_fees,
                escrow_token_contract.address,
            )?
        }
    };

    Ok(CalculateUserBackStrategyCalculateAmountToMintResult {
        fees,
        back_amount_minus_fees: back_token_amount_minus_fees,
        strategy_token_to_mint,
        strategy_pool_active,
        sp_assets_and_amounts,
        token_decimals,
        escrow_allocations_for_tokens: tokens_and_escrow_token_to_spend,
        strategy_pool_assets_bought_for_this_backer,
    })
}

pub async fn user_back_strategy(
    conn: &EthereumRpcConnection,
    ctx: &RequestContext,
    db: &DbClient,
    blockchain: EnumBlockChain,
    user_id: i64,
    back_token_amount: Decimal,
    strategy_id: i64,
    token_id: i64,
    token_address: Address,
    escrow_contract: EscrowContract<EitherTransport>,
    dex_addresses: &DexAddresses,
    master_key: impl Key + Clone,
    strategy_wallet: Option<Address>,
    logger: DynLogger,
    pancake_paths: &WorkingPancakePairPaths,
    cmc: &CoinMarketCap,
) -> Result<()> {
    logger.log(format!("checking back amount {}", back_token_amount));
    if back_token_amount.is_zero() {
        bail!("back zero amount");
    }

    /* check if user has enough balance */
    // TODO: add user balance to the database
    // TODO: might call balanceOf of these ERC20 contracts if database is not working correctly
    let user_balance = db
        .execute(FunUserListUserDepositWithdrawBalanceReq {
            limit: Some(1),
            offset: None,
            user_id,
            user_address: None,
            blockchain: Some(blockchain),
            token_address: None,
            token_id: Some(token_id),
            escrow_contract_address: Some(escrow_contract.address().into()),
        })
        .await?;
    debug!("Fetched {} rows of user balance", user_balance.len());
    let user_balance = user_balance
        .into_rows()
        .into_iter()
        .fold(Decimal::new(0, 0), |acc, row| acc + row.balance);
    if user_balance < back_token_amount {
        bail!(
            "insufficient balance {} < {}",
            user_balance,
            back_token_amount
        );
    }

    /* get amount deposited by whitelisted wallets necessary for back amount */
    let wallet_deposit_amounts = get_and_check_user_deposit_balances_by_wallet_to_fill_value(
        &db,
        blockchain,
        user_id,
        token_id,
        back_token_amount,
        escrow_contract.address(),
    )
    .await?;

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
    let token_decimals = escrow_token_contract.decimals().await?.as_u32();
    /* instantiate pancake contract */
    let pancake_contract = PancakeSmartRouterContract::new(
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
        let transfer_asset_from_transaction = || {
            let amount = decimal_to_u256(amount, token_decimals);
            escrow_contract.transfer_asset_from(
                &conn,
                master_key.clone(),
                wallet.clone(),
                token_address,
                amount,
                master_key.address(),
                logger.clone(),
            )
        };

        execute_transaction_and_ensure_success(
            transfer_asset_from_transaction,
            &conn,
            CONFIRMATIONS,
            MAX_RETRIES,
            POLL_INTERVAL,
            &logger,
        )
        .await?;

        /* reduce the value used from ledger */
        db.execute(FunUserAddUserDepositWithdrawLedgerEntryReq {
            user_id,
            blockchain,
            user_address: wallet.clone().into(),
            receiver_address: Default::default(),
            quantity: amount.clone(),
            transaction_hash: Default::default(),
            is_deposit: false,
            is_back: true,
            token_address: token_address.into(),
            escrow_contract_address: escrow_contract.address().into(),
            is_withdraw: false,
        })
        .await?;

        /* reduce the value used from balance */
        let old_balance_row = db
            .execute(FunUserListUserDepositWithdrawBalanceReq {
                limit: Some(1),
                offset: None,
                user_id,
                user_address: Some(wallet.clone().into()),
                blockchain: Some(blockchain),
                token_address: Some(token_address.into()),
                token_id: None,
                escrow_contract_address: Some(escrow_contract.address().into()),
            })
            .await?
            .into_result()
            .context("could not fetch user balance from database")?;
        let old_balance = old_balance_row.balance;
        let new_balance = old_balance - amount;
        db.execute(FunUserUpdateUserDepositWithdrawBalanceReq {
            deposit_withdraw_balance_id: old_balance_row.deposit_withdraw_balance_id,
            old_balance,
            new_balance,
        })
        .await?
        .into_result()
        .context("could not update user balance")?;
    }

    /* approve pancakeswap to trade escrow token */
    info!("approve pancakeswap to trade escrow token");
    let approve_transaction = || {
        let back_token_amount = decimal_to_u256(back_token_amount, token_decimals);
        escrow_token_contract.approve(
            &conn,
            master_key.clone(),
            pancake_contract.address(),
            back_token_amount,
            logger.clone(),
        )
    };

    execute_transaction_and_ensure_success(
        approve_transaction,
        &conn,
        CONFIRMATIONS,
        MAX_RETRIES,
        POLL_INTERVAL,
        &logger,
    )
    .await?;

    /* trade escrow token for strategy's tokens */
    info!("trade escrow token for strategy's tokens");
    let pancake_trade_parser = get_pancake_swap_parser();
    let get_out_amount = |out_token, in_amount: Decimal, out_decimals| {
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
                    "approving {} {:?} to strategy pool contract",
                    in_amount, token_address
                ));

                let out_token_contract = Erc20Token::new(conn.clone(), out_token)?;

                let approve_trasanction = || {
                    out_token_contract.approve(
                        &conn,
                        master_key.clone(),
                        sp_contract.address(),
                        decimal_to_u256(in_amount, token_decimals),
                        logger.clone(),
                    )
                };

                execute_transaction_and_ensure_success(
                    approve_trasanction,
                    &conn,
                    CONFIRMATIONS,
                    MAX_RETRIES,
                    POLL_INTERVAL,
                    &logger,
                )
                .await?;
                Ok(in_amount)
            } else {
                let pancake_path_set = pancake_paths
                    .get_pair_by_address(blockchain, token_address, out_token)
                    .await?;

                logger.log(&format!(
                    "copy_trade_and_ensure_success: token_in: {:?} amount_in: {}, token_out: {:?} amount_out_minimum: {}",
                    token_address,
                    in_amount,
                    out_token,
                    Decimal::one()
                ));

                let copy_trade_transaction = || {
                    pancake_contract.copy_trade(
                        &conn,
                        master_key.clone(),
                        pancake_path_set.clone(),
                        decimal_to_u256(in_amount, token_decimals),
                        U256::one(), // TODO: find a way to estimate amount out
                    )
                };

                let trade_hash = execute_transaction_and_ensure_success(
                    copy_trade_transaction,
                    &conn,
                    CONFIRMATIONS,
                    MAX_RETRIES,
                    POLL_INTERVAL,
                    &logger,
                )
                .await?;

                logger.log(&format!(
                    "copy_trade_and_ensure_success: tx_hash: {:?}",
                    trade_hash
                ));

                let trade = pancake_trade_parser.parse_trade(
                    &TransactionFetcher::new_and_assume_ready(trade_hash, &conn).await?,
                    blockchain,
                )?;

                /* update last dex trade cache table */
                db.execute(FunWatcherUpsertLastDexTradeForPairReq {
                    transaction_hash: trade_hash.into(),
                    blockchain: blockchain.into(),
                    dex: EnumDex::PancakeSwap,
                    token_in_address: token_address.into(),
                    token_out_address: out_token.into(),
                    amount_in: u256_to_decimal(trade.amount_in, token_decimals),
                    amount_out: u256_to_decimal(trade.amount_out, out_decimals),
                })
                .await?;
                logger.log(format!(
                    "approving {} token {:?} to strategy pool contract",
                    u256_to_decimal(trade.amount_out, out_decimals),
                    out_token
                ));

                let out_token_contract = Erc20Token::new(conn.clone(), out_token)?;

                let approve_transaction = || {
                    out_token_contract.approve(
                        &conn,
                        master_key.clone(),
                        sp_contract.address(),
                        trade.amount_out,
                        logger.clone(),
                    )
                };

                execute_transaction_and_ensure_success(
                    approve_transaction,
                    &conn,
                    CONFIRMATIONS,
                    MAX_RETRIES,
                    POLL_INTERVAL,
                    &logger,
                )
                .await?;
                Ok(u256_to_decimal(trade.amount_out, out_decimals))
            }
        }
    };

    let CalculateUserBackStrategyCalculateAmountToMintResult {
        // back_amount_minus_fees,
        /* TODO: fees are now in the pending wallet, we should add it to the database */
        /* TODO: add table to register treasury fees and strategy fees */
        // fees,
        // we discard this value because it's not really exactly the value
        strategy_token_to_mint,
        strategy_pool_assets_bought_for_this_backer,
        token_decimals,
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
        cmc,
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
    let assets_to_deposit: HashMap<Address, Decimal> = strategy_pool_assets_bought_for_this_backer
        .into_iter()
        .filter(|x| !x.1.is_zero())
        .collect();
    let deposit_transaction = || {
        let strategy_token_to_mint = {
            let decimal = token_decimals.get(&token_address).unwrap();
            decimal_to_u256(strategy_token_to_mint, *decimal)
        };
        let assets_to_deposit = assets_to_deposit
            .clone()
            .into_iter()
            .map(|(token, amount)| {
                (token, {
                    let decimal = token_decimals.get(&token).unwrap();
                    decimal_to_u256(amount, *decimal)
                })
            })
            .collect::<HashMap<_, _>>();
        sp_contract.deposit(
            &conn,
            master_key.clone(),
            assets_to_deposit.keys().cloned().collect(),
            assets_to_deposit.values().cloned().collect(),
            strategy_token_to_mint,
            strategy_wallet_contract.address(),
            logger.clone(),
        )
    };

    let deposit_transaction_hash = execute_transaction_and_ensure_success(
        deposit_transaction,
        &conn,
        CONFIRMATIONS,
        MAX_RETRIES,
        POLL_INTERVAL,
        &logger,
    )
    .await?;

    db.execute(FunWatcherUpsertUserStrategyBalanceReq {
        user_id: ctx.user_id,
        strategy_id,
        blockchain,
        old_balance: user_strategy_balance,
        new_balance: user_strategy_balance + strategy_token_to_mint,
    })
    .await?;
    for (token, amount) in assets_to_deposit.clone() {
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
            Some(token_out) => token_out.balance + amount,
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
                let old_amount = existing_amount.balance;
                let new_amount = old_amount + amount;
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
                    old_balance: Decimal::zero().into(),
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
            new_total_backed_quantity: strategy.total_backed_usdc + back_token_amount,
            old_total_backed_quantity: strategy.total_backed_usdc,
            new_current_quantity: strategy.current_usdc + back_token_amount,
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
        strategy_id, back_token_amount, strategy_token_to_mint
    ));
    Ok(())
}

pub async fn get_and_check_user_deposit_balances_by_wallet_to_fill_value(
    db: &DbClient,
    chain: EnumBlockChain,
    user_id: i64,
    token_id: i64,
    value_to_fill: Decimal,
    escrow_contract_address: Address,
) -> Result<HashMap<Address, Decimal>> {
    let wallet_asset_balances = db
        .execute(FunUserListUserDepositWithdrawBalanceReq {
            limit: None,
            offset: None,
            user_id,
            user_address: None,
            blockchain: Some(chain),
            token_id: Some(token_id),
            token_address: None,
            escrow_contract_address: Some(escrow_contract_address.into()),
        })
        .await?
        .into_rows();

    let mut wallet_deposit_amounts: HashMap<Address, Decimal> = HashMap::new();
    let mut total_amount_found: Decimal = Decimal::zero();
    for wallet_balance_row in wallet_asset_balances {
        let wallet_balance: Decimal = wallet_balance_row.balance;
        if wallet_balance.is_zero() || wallet_balance.is_sign_negative() {
            continue;
        }
        if total_amount_found == value_to_fill {
            /* if entire amount was filled, the wallet balances fetched so far are enough */
            break;
        }
        /* if entire amount was not filled, keep looking for other positive balances from wallets */

        let necessary_amount_to_fill = value_to_fill - total_amount_found;
        if wallet_balance >= necessary_amount_to_fill {
            /* if the deposited amount of this wallet can fill the rest for the refund amount */
            /* use only the necessary */

            wallet_deposit_amounts.insert(
                wallet_balance_row.user_address.into(),
                necessary_amount_to_fill,
            );
            total_amount_found = total_amount_found + necessary_amount_to_fill;
        } else {
            /* if the deposited amount of this wallet cannot fill the rest for the refund amount */
            /* use entire amount */

            wallet_deposit_amounts.insert(wallet_balance_row.user_address.into(), wallet_balance);
            total_amount_found = total_amount_found + wallet_balance;
        }
    }

    if total_amount_found < value_to_fill {
        bail!(
            "not enough user balance to fill the amount: {} < {}",
            total_amount_found,
            value_to_fill
        );
    }

    Ok(wallet_deposit_amounts)
}

pub async fn calculate_sp_tokens_to_mint_easy_approach(
    token_ratio: FunUserListStrategyInitialTokenRatiosRespRow,
    escrow_amount: Decimal,
) -> Result<Decimal> {
    let relative_quantity = token_ratio.quantity;
    let result = escrow_amount * relative_quantity;
    info!(
        "calculate_sp_tokens_to_mint_easy_approach {}={}*{}",
        result, escrow_amount, relative_quantity
    );

    Ok(result)
}

async fn trade_escrow_for_strategy_tokens<Fut: Future<Output = Result<Decimal>>>(
    escrow_token_address: Address,
    tokens_and_amounts_to_buy: HashMap<Address, Decimal>,
    token_decimals: HashMap<Address, u32>,
    logger: DynLogger,
    get_token_out: impl Fn(Address, Decimal, u32) -> Fut,
) -> Result<HashMap<Address, Decimal>> {
    /* buys tokens and amounts and returns a vector or bought tokens and amounts out */
    let mut deposit_amounts: HashMap<Address, Decimal> = HashMap::new();

    for (token_address, amount_to_spend_on_it) in tokens_and_amounts_to_buy {
        logger.log(format!(
            "Trading {} {:?} for {:?}",
            amount_to_spend_on_it, token_address, escrow_token_address
        ));
        let decimals = token_decimals.get(&token_address).unwrap();
        let output_amount = get_token_out(token_address, amount_to_spend_on_it, *decimals).await?;
        deposit_amounts.insert(token_address, output_amount);
    }
    Ok(deposit_amounts)
}

fn calculate_sp_tokens_to_mint_nth_backer(
    strategy_token_total_supply: Decimal,
    strategy_pool_asset_balances: HashMap<Address, Decimal>,
    token_prices: HashMap<Address, Decimal>,
    base_token_actual_amount: Decimal,
    deposit_asset: Address,
) -> Result<Decimal> {
    ensure!(
        strategy_pool_asset_balances.len() > 0,
        "strategy pool asset balances must not be empty"
    );
    // sp tokens to mint = actual value of backing / (total value / total supply)
    //                   = actual value of backing * total supply / total value
    /* calculate strategy pool assets total value in base tokens based on the price paid on assets */
    let mut strategy_pool_assets_total_value = Decimal::zero();
    for (strategy_pool_asset, strategy_pool_asset_amount) in strategy_pool_asset_balances.iter() {
        /* get base token value of one asset */
        /* i.e. asset price in base tokens */
        let asset_price_in_base_tokens = token_prices
            .get(strategy_pool_asset)
            .with_context(|| format!("could not find price {:?}", strategy_pool_asset))?;

        /* get value of strategy pool asset amount in base tokens */
        let strategy_pool_asset_value = strategy_pool_asset_amount * *asset_price_in_base_tokens;

        /* add value of strategy pool asset to total value */
        strategy_pool_assets_total_value =
            strategy_pool_assets_total_value + strategy_pool_asset_value;
    }

    /* calculate ratio as total strategy pool value / total supply */
    /* i.e. the share value of one base token */
    /* i.e. the base token price in shares */
    let ratio = strategy_token_total_supply / strategy_pool_assets_total_value;
    let deposit_asset_price_in_base_token = token_prices
        .get(&deposit_asset)
        .with_context(|| format!("could not find price of token {:?}", deposit_asset))?;
    let base_token_actual_value = base_token_actual_amount * deposit_asset_price_in_base_token;
    /* calculate strategy pool tokens to mint as actual_amount * ratio */
    Ok(base_token_actual_value / ratio)
}

#[cfg(test)]
mod tests {
    use super::super::method::user_exit_strategy;
    use super::*;
    use crate::method::on_user_request_refund;
    use eth_sdk::erc20::Erc20Token;
    use eth_sdk::escrow_tracker::escrow::parse_escrow_transfer;
    use eth_sdk::mock_erc20::deploy_mock_erc20;
    use eth_sdk::signer::Secp256k1SecretKey;
    use eth_sdk::utils::wait_for_confirmations_simple;
    use eth_sdk::{
        BlockchainCoinAddresses, EthereumRpcConnectionPool, EthereumToken, ScaledMath,
        TransactionReady, ANVIL_PRIV_KEY_1, ANVIL_PRIV_KEY_2,
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
            decimals: 18,
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
        let esc = parse_escrow_transfer(chain, tx, stablecoin_addresses, erc_20)?;

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
