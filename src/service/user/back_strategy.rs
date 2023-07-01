use api::cmc::CoinMarketCap;
use eth_sdk::erc20::{approve_and_ensure_success, Erc20Token};
use eth_sdk::escrow::{transfer_token_to_and_ensure_success, EscrowContract};
use eth_sdk::pair_paths::WorkingPancakePairPaths;
use eth_sdk::strategy_pool::{sp_deposit_to_and_ensure_success, StrategyPoolContract};
use eth_sdk::strategy_wallet::StrategyWalletContract;
use eth_sdk::v3::smart_router::{copy_trade_and_ensure_success, PancakeSmartRouterV3Contract};
use eth_sdk::{
    build_pancake_swap, DexAddresses, EitherTransport, EthereumRpcConnection, ScaledMath,
    TransactionFetcher, CONFIRMATIONS, MAX_RETRIES, POLL_INTERVAL,
};
use eyre::*;
use eyre::{bail, ensure, eyre, ContextCompat, WrapErr};
use gen::database::{
    FunAdminListUsersReq, FunUserAddStrategyPoolContractReq, FunUserAddStrategyWalletReq,
    FunUserBackStrategyReq, FunUserListStrategiesReq, FunUserListStrategyInitialTokenRatiosReq,
    FunUserListStrategyWalletsReq, FunUserListUserDepositWithdrawBalanceReq,
    FunWatcherListLastDexTradesForPairReq, FunWatcherListStrategyPoolContractAssetBalancesReq,
    FunWatcherListStrategyPoolContractReq, FunWatcherListUserStrategyBalanceReq,
    FunWatcherUpsertLastDexTradeForPairReq, FunWatcherUpsertStrategyPoolContractAssetBalanceReq,
    FunWatcherUpsertUserStrategyBalanceReq,
};
use gen::model::{EnumBlockChain, EnumDex};
use lib::database::DbClient;
use lib::toolbox::RequestContext;
use lib::types::U256;
use std::collections::HashMap;
use tracing::{debug, info};
use web3::signing::Key;
use web3::types::Address;

pub async fn deploy_wallet_contract(
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

async fn user_get_or_deploy_strategy_wallet(
    conn: &EthereumRpcConnection,
    ctx: &RequestContext,
    db: &DbClient,
    master_key: impl Key + Clone,
    blockchain: EnumBlockChain,
    user_wallet_address_to_receive_shares_on_this_chain: Address,
) -> Result<StrategyWalletContract<EitherTransport>> {
    match db
        .execute(FunUserListStrategyWalletsReq {
            user_id: ctx.user_id,
            blockchain: Some(blockchain),
        })
        .await?
        .into_result()
    {
        Some(strategy_wallet_contract) => {
            /* if user has wallet on this chain, use it */
            StrategyWalletContract::new(conn.clone(), strategy_wallet_contract.address.into())
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
                address: strategy_wallet_contract.address().into(),
            })
            .await?;

            Ok(strategy_wallet_contract)
        }
    }
}

async fn user_get_or_deploy_strategy_pool(
    conn: &EthereumRpcConnection,
    _ctx: &RequestContext,
    db: &DbClient,
    master_key: impl Key + Clone,
    strategy_id: i64,
    blockchain: EnumBlockChain,
    strategy_token_name: String,
    strategy_token_symbol: String,
) -> Result<(i64, StrategyPoolContract<EitherTransport>)> {
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
        None => {
            /* if strategy pool doesn't exist in this chain, create it */
            let contract = deploy_strategy_contract(
                &conn,
                master_key.clone(),
                strategy_token_name,
                strategy_token_symbol,
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
    };
    Ok(sp_contract)
}

pub async fn user_back_strategy(
    conn: &EthereumRpcConnection,
    ctx: &RequestContext,
    db: &DbClient,
    blockchain: EnumBlockChain,
    user_id: i64,
    back_usdc_amount: U256,
    strategy_id: i64,
    token_id: i64,
    token_address: Address,
    escrow_contract: EscrowContract<EitherTransport>,
    dex_addresses: &DexAddresses,
    master_key: impl Key + Clone,
) -> Result<()> {
    if back_usdc_amount == U256::zero() {
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
    let user_balance = user_balance.into_result().context("insufficient balance")?;
    let user_balance: U256 = user_balance.balance.into();
    if user_balance < back_usdc_amount {
        bail!("insufficient balance");
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

    /* instantiate strategy wallet contract wrapper */
    let strategy_wallet_contract = user_get_or_deploy_strategy_wallet(
        &conn,
        &ctx,
        &db,
        master_key.clone(),
        blockchain,
        user_wallet_address_to_receive_shares_on_this_chain,
    )
    .await?;

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
    let strategy_initial_ratios = db
        .execute(FunUserListStrategyInitialTokenRatiosReq {
            strategy_id,
            token_address: None,
            blockchain: Some(blockchain),
        })
        .await?
        .into_rows();
    ensure!(
        !strategy_initial_ratios.is_empty(),
        "strategy has no initial ratios"
    );
    /* deduce fees from back amount */
    // TODO: use (back amount - fees) to calculate trade spenditure and SP shares
    // TODO: distribute fees for the treasury and the strategy creator
    let platform_fee = 0.01;
    let divide_scale = 10000;
    let fees = back_usdc_amount
        * (((strategy.swap_fee.unwrap_or_default()
            + strategy.strategy_fee.unwrap_or_default()
            + strategy.expert_fee.unwrap_or_default()
            + platform_fee)
            * divide_scale as f64) as u64)
        / divide_scale;
    let back_usdc_amount_minus_fees = back_usdc_amount - fees;

    /* instantiate strategy contract wrapper */
    let (strategy_pool_contract_id, sp_contract) = user_get_or_deploy_strategy_pool(
        &conn,
        &ctx,
        &db,
        master_key.clone(),
        strategy_id,
        blockchain,
        strategy.strategy_name.clone(),
        strategy.strategy_name,
    )
    .await?;

    /* calculate shares to mint for backer */
    // TODO: find out if we use back amount with or without fees for share calculation
    // currently calculating with back amount minus fees
    // TODO: get these values from database
    let total_strategy_pool_tokens = sp_contract.total_supply().await?;

    let sp_assets_and_amounts = sp_contract
        .assets_and_balances()
        .await
        .context("failed to query strategy pool assets and amounts")?;
    let sp_assets_and_amounts: HashMap<Address, U256> = sp_assets_and_amounts
        .0
        .into_iter()
        .zip(sp_assets_and_amounts.1.into_iter())
        .collect();
    let escrow_token_contract = Erc20Token::new(conn.clone(), token_address)?;
    let strategy_pool_token_to_mint = calculate_sp_tokens_to_mint_easy_approach(
        &conn,
        &CoinMarketCap::new_debug_key()?,
        total_strategy_pool_tokens,
        sp_assets_and_amounts,
        sp_contract.decimals().await?,
        escrow_token_contract.symbol().await?,
        back_usdc_amount_minus_fees,
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

    // FIXME: we should do it in escrow pending contract or somewhere
    /* transfer escrow to our EOA */
    transfer_token_to_and_ensure_success(
        escrow_contract,
        &conn,
        CONFIRMATIONS,
        MAX_RETRIES,
        POLL_INTERVAL,
        master_key.clone(),
        token_address,
        master_key.address(),
        back_usdc_amount_minus_fees,
    )
    .await?;
    let mut strategy_initial_token_ratios: HashMap<Address, U256> = HashMap::new();
    for x in strategy_initial_ratios.iter() {
        strategy_initial_token_ratios.insert(x.token_address.into(), x.quantity.into());
    }

    /* calculate how much of back amount to spend on each strategy token */
    let escrow_allocations_for_tokens = calculate_escrow_allocation_for_strategy_tokens(
        back_usdc_amount_minus_fees,
        strategy_initial_token_ratios,
    )?;
    /* approve pancakeswap to trade escrow token */
    approve_and_ensure_success(
        escrow_token_contract,
        &conn,
        CONFIRMATIONS,
        MAX_RETRIES,
        POLL_INTERVAL,
        master_key.clone(),
        pancake_contract.address(),
        back_usdc_amount,
    )
    .await?;

    /* trade escrow token for strategy's tokens */
    let (tokens_to_deposit, amounts_to_deposit) = trade_escrow_for_strategy_tokens(
        &conn,
        &db,
        master_key.clone(),
        blockchain,
        token_address,
        &pancake_contract,
        escrow_allocations_for_tokens,
    )
    .await?;

    /* approve tokens and amounts to SP contract */
    for (token, amount) in tokens_to_deposit.iter().zip(amounts_to_deposit.iter()) {
        approve_and_ensure_success(
            Erc20Token::new(conn.clone(), token.clone())?,
            &conn,
            CONFIRMATIONS,
            MAX_RETRIES,
            POLL_INTERVAL,
            master_key.clone(),
            sp_contract.address(),
            amount.clone(),
        )
        .await?;
    }

    /* mint strategy pool token to strategy wallet contract */
    let deposit_transaction_hash = sp_deposit_to_and_ensure_success(
        sp_contract,
        &conn,
        CONFIRMATIONS,
        MAX_RETRIES,
        POLL_INTERVAL,
        master_key.clone(),
        tokens_to_deposit.clone(),
        amounts_to_deposit.clone(),
        strategy_pool_token_to_mint,
        strategy_wallet_contract.address(),
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

    db.execute(FunWatcherUpsertUserStrategyBalanceReq {
        user_id: ctx.user_id,
        strategy_id,
        blockchain,
        old_balance: user_strategy_balance,
        new_balance: (*user_strategy_balance + strategy_pool_token_to_mint).into(),
    })
    .await?;
    for (token, amount) in tokens_to_deposit
        .iter()
        .cloned()
        .zip(amounts_to_deposit.iter().cloned())
    {
        let sp_asset_token = db
            .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
                strategy_pool_contract_id,
                token_address: Some(token.into()),
                blockchain: Some(blockchain),
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
    }

    let ret = db
        .execute(FunUserBackStrategyReq {
            user_id: ctx.user_id,
            strategy_id: strategy.strategy_id,
            quantity: back_usdc_amount.into(),
            new_total_backed_quantity: (*strategy.total_backed_usdc + back_usdc_amount).into(),
            old_total_backed_quantity: strategy.total_backed_usdc,
            new_current_quantity: (*strategy.current_usdc + back_usdc_amount).into(),
            old_current_quantity: strategy.current_usdc,
            blockchain,
            transaction_hash: deposit_transaction_hash.into(),
            earn_sp_tokens: strategy_pool_token_to_mint.into(),
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

fn calculate_escrow_allocation_for_strategy_tokens(
    escrow_amount: U256,
    strategy_initial_token_ratios: HashMap<Address, U256>,
) -> Result<HashMap<Address, U256>> {
    let total_initial_token_numbers: U256 = strategy_initial_token_ratios
        .values()
        .fold(U256::zero(), |acc, x| acc + x);
    ensure!(
        total_initial_token_numbers > U256::zero(),
        "Total initial token numbers is zero"
    );
    /* calculates how much of escrow to spend on each strategy token */
    /* allocation = (initial_strategy_token_amount * escrow_amount) / total_initial_strategy_token_amounts */
    let mut escrow_allocations: HashMap<Address, U256> = HashMap::new();
    for (token_address, token_amount) in strategy_initial_token_ratios {
        let escrow_allocation = token_amount.mul_div(escrow_amount, total_initial_token_numbers)?;
        escrow_allocations.insert(token_address, escrow_allocation);
    }
    Ok(escrow_allocations)
}

async fn calculate_sp_tokens_to_mint_easy_approach(
    conn: &EthereumRpcConnection,
    cmc: &CoinMarketCap,
    sp_total_shares: U256,
    sp_escrow_token_balances: HashMap<Address, U256>,
    sp_decimals: U256,
    escrow_symbol: String,
    escrow_amount: U256,
    escrow_decimals: U256,
) -> Result<U256> {
    /* multiply the escrow amount by the price to get its value with no consideration for decimals */
    /* if escrow decimals > sp decimals, divide unconsidered value by 10^(escrow decimals - sp decimals) to account for decimal differences */
    /* if sp decimals > escrow decimals, multiply the unconsidered value by 10^(sp decimals - escrow decimals) to account for decimal differences */
    /* this is valid for all tokens, not just the escrow */
    let factor = cmc.get_usd_prices_by_symbol(&vec![escrow_symbol]).await?[0];
    let escrow_value: U256 = if escrow_decimals > sp_decimals {
        escrow_amount.mul_f64(factor)?.try_checked_div(U256::exp10(
            escrow_decimals.as_usize() - sp_decimals.as_usize(),
        ))?
    } else {
        escrow_amount.mul_f64(factor)?.try_checked_mul(U256::exp10(
            sp_decimals.as_usize() - escrow_decimals.as_usize(),
        ))?
    };
    if sp_total_shares == U256::zero() {
        /* if strategy pool is empty, shares = escrow value */
        return Ok(escrow_value);
    }
    /* if strategy pool is active, shares = (escrow_value * total_strategy_shares) / total_strategy_value */
    let mut sp_total_value = U256::zero();
    for (asset, amount) in sp_escrow_token_balances.iter() {
        let erc20 = Erc20Token::new(conn.clone(), *asset)?;
        let price = cmc
            .get_usd_prices_by_symbol(&vec![erc20.symbol().await?])
            .await?;
        /* add to total value the value of each token accounting for decimal differences */
        let token_decimals = erc20.decimals().await?;
        if token_decimals > sp_decimals {
            sp_total_value =
                sp_total_value.try_checked_add(amount.mul_f64(price[0])?.try_checked_div(
                    U256::exp10(token_decimals.as_usize() - sp_decimals.as_usize()),
                )?)?;
        } else {
            sp_total_value =
                sp_total_value.try_checked_add(amount.mul_f64(price[0])?.try_checked_mul(
                    U256::exp10(sp_decimals.as_usize() - token_decimals.as_usize()),
                )?)?;
        }
    }

    Ok(escrow_value.mul_div(
        sp_total_shares,
        if sp_total_value == U256::zero() {
            U256::one()
        } else {
            sp_total_value
        },
    )?)
}

async fn trade_escrow_for_strategy_tokens(
    conn: &EthereumRpcConnection,
    db: &DbClient,
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
        if token_address == escrow_token_address {
            /* if sp holds escrow token, deposit it directly */
            token_addresses_to_deposit.push(token_address);
            token_amounts_to_deposit.push(amount_to_spend_on_it);
            continue;
        }
        let pancake_path_set =
            pancake_paths.get_pair_by_address(chain, escrow_token_address, token_address)?;
        let trade_hash = copy_trade_and_ensure_success(
            dex_contract.clone(),
            &conn,
            CONFIRMATIONS,
            MAX_RETRIES,
            POLL_INTERVAL,
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

        /* update last dex trade cache table */
        db.execute(FunWatcherUpsertLastDexTradeForPairReq {
            transaction_hash: trade_hash.into(),
            blockchain: chain.into(),
            dex: EnumDex::PancakeSwap,
            token_in_address: escrow_token_address.into(),
            token_out_address: token_address.into(),
            amount_in: trade.amount_in.into(),
            amount_out: trade.amount_out.into(),
        })
        .await?;
    }
    Ok((token_addresses_to_deposit, token_amounts_to_deposit))
}

pub async fn user_back_strategy_sergio_tries_to_help(
    conn: &EthereumRpcConnection,
    ctx: &RequestContext,
    db: &DbClient,
    blockchain: EnumBlockChain,
    user_id: i64,
    back_usdc_amount: U256,
    strategy_id: i64,
    token_id: i64,
    token_address: Address,
    escrow_contract: EscrowContract<EitherTransport>,
    dex_addresses: &DexAddresses,
    master_key: impl Key + Clone,
) -> Result<()> {
    if back_usdc_amount == U256::zero() {
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
    let user_balance_row = user_balance.into_result().context("insufficient balance")?;
    let user_balance: U256 = user_balance_row.balance.into();
    if user_balance < back_usdc_amount {
        bail!("insufficient balance");
    }

    /* fetch user address to receive strategy assets */
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

    /* instantiate strategy wallet contract wrapper */
    let strategy_wallet_contract = user_get_or_deploy_strategy_wallet(
        &conn,
        &ctx,
        &db,
        master_key.clone(),
        blockchain,
        user_wallet_address_to_receive_shares_on_this_chain,
    )
    .await?;

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

    /* fetch strategy assets */
    let strategy_initial_ratios = db
        .execute(FunUserListStrategyInitialTokenRatiosReq {
            strategy_id,
            token_address: None,
            blockchain: Some(blockchain),
        })
        .await?
        .into_rows();
    ensure!(
        !strategy_initial_ratios.is_empty(),
        "strategy has no initial ratios"
    );

    /* deduce fees from back amount */
    // TODO: use (back amount - fees) to calculate trade spenditure and SP shares
    // TODO: distribute fees for the treasury and the strategy creator
    let platform_fee = 0.01;
    let divide_scale = 10000;
    let fees = back_usdc_amount
        * (((strategy.swap_fee.unwrap_or_default()
            + strategy.strategy_fee.unwrap_or_default()
            + strategy.expert_fee.unwrap_or_default()
            + platform_fee)
            * divide_scale as f64) as u64)
        / divide_scale;
    let back_usdc_amount_minus_fees = back_usdc_amount - fees;

    /* instantiate strategy contract wrapper */
    let (strategy_pool_contract_id, sp_contract) = user_get_or_deploy_strategy_pool(
        &conn,
        &ctx,
        &db,
        master_key.clone(),
        strategy_id,
        blockchain,
        strategy.strategy_name.clone(),
        strategy.strategy_name,
    )
    .await?;

    /* calculate shares to mint for backer */
    // TODO: find out if we use back amount with or without fees for share calculation
    // currently calculating with back amount minus fees
    // TODO: get these values from database
    let total_strategy_pool_tokens = sp_contract.total_supply().await?;

    let sp_asset_rows = db
        .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
            strategy_pool_contract_id: strategy_pool_contract_id,
            token_address: None,
            blockchain: Some(blockchain),
        })
        .await?
        .into_rows();

    let mut strategy_pool_is_active: bool = false;
    let mut sp_assets_and_amounts: HashMap<Address, U256> = HashMap::new();
    for sp_asset_row in sp_asset_rows {
        if sp_asset_row.balance > U256::zero().into() {
            strategy_pool_is_active = true;
        }
        sp_assets_and_amounts.insert(
            sp_asset_row.token_address.into(),
            sp_asset_row.balance.into(),
        );
    }

    /* instantiate base token contract, and pancake contract */
    let escrow_token_contract = Erc20Token::new(conn.clone(), token_address)?;
    let pancake_contract = PancakeSmartRouterV3Contract::new(
        conn.clone(),
        dex_addresses
            .get(blockchain, EnumDex::PancakeSwap)
            .ok_or_else(|| eyre!("pancake swap not available on this chain"))?,
    )?;

    // TODO: make some way of replaying the correct transactions in case of failure in the middle of the backing process
    // FIXME: we should do it in escrow pending contract or somewhere
    /* transfer escrow to our EOA */
    transfer_token_to_and_ensure_success(
        escrow_contract,
        &conn,
        CONFIRMATIONS,
        MAX_RETRIES,
        POLL_INTERVAL,
        master_key.clone(),
        token_address,
        master_key.address(),
        back_usdc_amount_minus_fees,
    )
    .await?;
    let mut strategy_initial_token_ratios: HashMap<Address, U256> = HashMap::new();
    for x in strategy_initial_ratios.iter() {
        strategy_initial_token_ratios.insert(x.token_address.into(), x.quantity.into());
    }

    /* calculate how much of back amount to spend on each strategy pool asset */
    let escrow_allocations_for_tokens = calculate_escrow_allocation_for_strategy_tokens(
        back_usdc_amount_minus_fees,
        strategy_initial_token_ratios,
    )?;

    /* approve pancakeswap to trade base token */
    approve_and_ensure_success(
        escrow_token_contract.clone(),
        &conn,
        CONFIRMATIONS,
        MAX_RETRIES,
        POLL_INTERVAL,
        master_key.clone(),
        pancake_contract.address(),
        back_usdc_amount,
    )
    .await?;

    /* trade base token for strategy pool assets */
    let (tokens_to_deposit, amounts_to_deposit) = trade_escrow_for_strategy_tokens(
        &conn,
        &db,
        master_key.clone(),
        blockchain,
        token_address,
        &pancake_contract,
        escrow_allocations_for_tokens.clone(),
    )
    .await?;

    // WARNING: without this mintage calculation might break for nth backer
    /* get amount spent and amount bought for every strategy pool asset to calculate sp valuation for mintage */
    /* necessary because there could be a strategy pool asset that is not in initial_token_ratios */
    // TODO: remove this when back strategy buys all strategy pool assets instead of initial_token_ratios
    // TODO: use strategy_pool_assets_bought_for_this_backer and escrow_allocations_for_tokens directly
    let strategy_pool_assets_bought_for_this_backer: HashMap<Address, U256> = tokens_to_deposit
        .iter()
        .cloned()
        .zip(amounts_to_deposit.iter().cloned())
        .collect::<HashMap<_, _>>();
    let mut strategy_pool_assets_bought: HashMap<Address, U256> = HashMap::new();
    let mut base_tokens_spent_on_strategy_pool_assets: HashMap<Address, U256> = HashMap::new();
    for (strategy_pool_asset, _) in sp_assets_and_amounts.iter() {
        if let Some(amount_bought) =
            strategy_pool_assets_bought_for_this_backer.get(strategy_pool_asset)
        {
            /* if strategy pool asset is in initial_token_ratios, it was bought */
            /* so use these trade values for valuation */
            strategy_pool_assets_bought.insert(strategy_pool_asset.clone(), amount_bought.clone());
            let amount_spent_on_asset = escrow_allocations_for_tokens.get(strategy_pool_asset).context("could not get amount spent for backer in strategy pool asset, even though amount bought exists")?.clone();
            base_tokens_spent_on_strategy_pool_assets
                .insert(strategy_pool_asset.clone(), amount_spent_on_asset);
        } else {
            /* if strategy pool asset is not in initial_token_ratios, it was not bought */
            /* fetch the most recent trade values from the database to use for valuation */
            let last_dex_trade_row = db
                .execute(FunWatcherListLastDexTradesForPairReq {
                    token_in_address: token_address.into(),
                    token_out_address: strategy_pool_asset.clone().into(),
                    blockchain: blockchain,
                    dex: None,
                })
                .await?
                .into_result()
                .context("could not fetch last dex trade for strategy pool asset")?;
            strategy_pool_assets_bought.insert(
                strategy_pool_asset.clone(),
                last_dex_trade_row.amount_out.into(),
            );
            base_tokens_spent_on_strategy_pool_assets.insert(
                strategy_pool_asset.clone(),
                last_dex_trade_row.amount_in.into(),
            );
        }
    }

    /* calculate mintage */
    let strategy_pool_token_to_mint = match strategy_pool_is_active {
        false => calculate_sp_tokens_to_mint_1st_backer_sergio_tries_to_help(
            sp_contract.decimals().await?,
            escrow_token_contract.decimals().await?,
            back_usdc_amount_minus_fees,
        )?,
        true => calculate_sp_tokens_to_mint_nth_backer_sergio_tries_to_help(
            &EnumDex::PancakeSwap,
            total_strategy_pool_tokens,
            sp_assets_and_amounts,
            strategy_pool_assets_bought,
            base_tokens_spent_on_strategy_pool_assets,
            back_usdc_amount_minus_fees,
        )?,
    };

    /* approve bought assets to strategy pool contract before deposit */
    for (token, amount) in tokens_to_deposit.iter().zip(amounts_to_deposit.iter()) {
        approve_and_ensure_success(
            Erc20Token::new(conn.clone(), token.clone())?,
            &conn,
            CONFIRMATIONS,
            MAX_RETRIES,
            POLL_INTERVAL,
            master_key.clone(),
            sp_contract.address(),
            amount.clone(),
        )
        .await?;
    }

    /* deposit bought assets to strategy wallet contract and mint strategy tokens for backer's strategy wallet */
    let deposit_transaction_hash = sp_deposit_to_and_ensure_success(
        sp_contract,
        &conn,
        CONFIRMATIONS,
        MAX_RETRIES,
        POLL_INTERVAL,
        master_key.clone(),
        tokens_to_deposit.clone(),
        amounts_to_deposit.clone(),
        strategy_pool_token_to_mint,
        strategy_wallet_contract.address(),
    )
    .await?;

    /* update user strategy tokens balance in database */
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

    db.execute(FunWatcherUpsertUserStrategyBalanceReq {
        user_id: ctx.user_id,
        strategy_id,
        blockchain,
        old_balance: user_strategy_balance,
        new_balance: (*user_strategy_balance + strategy_pool_token_to_mint).into(),
    })
    .await?;

    /* update strategy pool contract assets in database */
    for (token, amount) in tokens_to_deposit
        .iter()
        .cloned()
        .zip(amounts_to_deposit.iter().cloned())
    {
        let sp_asset_token = db
            .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
                strategy_pool_contract_id,
                token_address: Some(token.into()),
                blockchain: Some(blockchain),
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
    }

    /* update user back exit strategy ledger in database */
    let ret = db
        .execute(FunUserBackStrategyReq {
            user_id: ctx.user_id,
            strategy_id: strategy.strategy_id,
            quantity: back_usdc_amount.into(),
            new_total_backed_quantity: (*strategy.total_backed_usdc + back_usdc_amount).into(),
            old_total_backed_quantity: strategy.total_backed_usdc,
            new_current_quantity: (*strategy.current_usdc + back_usdc_amount).into(),
            old_current_quantity: strategy.current_usdc,
            blockchain,
            transaction_hash: deposit_transaction_hash.into(),
            earn_sp_tokens: strategy_pool_token_to_mint.into(),
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

fn calculate_sp_tokens_to_mint_1st_backer_sergio_tries_to_help(
    strategy_token_decimals: U256,
    base_token_decimals: U256,
    base_token_actual_amount: U256,
) -> Result<U256> {
    /* after normalization, the value is equivalent to what it would be in strategy token decimals */
    /* e.g. if actual_amount is 1.0 base tokens (i.e. 1 * 10^base_token_decimals) */
    /* normalized_amount is 1.0 strategy tokens (i.e. 1 * 10^strategy_token_decimals) */
    let decimal_normalized_base_token: U256 = if base_token_decimals > strategy_token_decimals {
        base_token_actual_amount.try_checked_div(U256::exp10(
            base_token_decimals.as_usize() - strategy_token_decimals.as_usize(),
        ))?
    } else {
        base_token_actual_amount.try_checked_mul(U256::exp10(
            strategy_token_decimals.as_usize() - base_token_decimals.as_usize(),
        ))?
    };

    // TODO: discover what the constant should be
    // remember that strategy token decimals is 18, this value is already considerably large
    let constant = U256::one();
    Ok(decimal_normalized_base_token.try_checked_mul(constant)?)
}

fn calculate_sp_tokens_to_mint_nth_backer_sergio_tries_to_help(
    dex: &EnumDex,
    strategy_token_total_supply: U256,
    strategy_pool_asset_balances: HashMap<Address, U256>,
    bought_asset_amounts: HashMap<Address, U256>,
    spent_base_token_allocation: HashMap<Address, U256>,
    base_token_actual_amount: U256,
) -> Result<U256> {
    /* calculate strategy pool assets total value in base tokens based on the price paid on assets */
    let mut strategy_pool_assets_total_value = U256::zero();
    for (strategy_pool_asset, strategy_pool_asset_amount) in strategy_pool_asset_balances.iter() {
        /* get amount of base token spent and amount of asset bought */
        let amount_of_base_token_spent_on_asset = spent_base_token_allocation
            .get(strategy_pool_asset)
            .context("could not find amount spend on asset to calculate mintage")?;
        let amount_of_asset_bought = bought_asset_amounts
            .get(strategy_pool_asset)
            .context("could not find bought asset to calculate mintage")?;

        /* get base token value of one asset */
        /* i.e. asset price in base tokens */
        // TODO: cache asset price in base tokens + DEX in database
        let asset_price_in_base_tokens =
            amount_of_base_token_spent_on_asset.div_as_f64(*amount_of_asset_bought)?;

        /* get value of strategy pool asset amount in base tokens */
        let strategy_pool_asset_value =
            strategy_pool_asset_amount.mul_f64(asset_price_in_base_tokens)?;

        /* add value of strategy pool asset to total value */
        strategy_pool_assets_total_value =
            strategy_pool_assets_total_value.try_checked_add(strategy_pool_asset_value)?;
    }

    /* calculate ratio as total strategy pool value / total supply */
    /* i.e. the share value of one base token */
    /* i.e. the base token price in shares */
    let ratio = strategy_pool_assets_total_value.div_as_f64(strategy_token_total_supply)?;

    /* calculate strategy pool tokens to mint as actual_amount * ratio */
    Ok(base_token_actual_amount.mul_f64(ratio)?)
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
                strategy_fee: 1.0,
                expert_fee: 1.0,
                agreed_tos: true,
                blockchain: EnumBlockChain::BscTestnet,
                wallet_address: Address::zero().into(),
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
            U256::from(10).try_checked_mul(U256::from(busd_decimals))?,
            ret.user_id,
            strategy.strategy_id,
            token.token_id,
            busd_address_on_bsc_testnet,
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
