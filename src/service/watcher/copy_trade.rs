use api::cmc::CoinMarketCap;
use crypto::Signer;
use eth_sdk::evm::DexTrade;
use eth_sdk::logger::BlockchainLogger;
use eth_sdk::smart_router::{copy_trade_and_ensure_success, PancakeSmartRouterV3Contract};
use eth_sdk::{
    ContractCall, EitherTransport, EscrowTransfer, EthereumRpcConnectionPool, PancakePairPathSet,
    PancakeSwap, ScaledMath, CONFIRMATIONS, MAX_RETRIES, POLL_INTERVAL,
};
use eyre::*;
use gen::database::*;
use gen::model::{EnumBlockChain, EnumDex, EnumDexVersion};
use itertools::Itertools;
use lib::database::DbClient;
use lib::log::DynLogger;
use num::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use web3::signing::Key;
use web3::types::{Address, TransactionReceipt, U256};

pub async fn fetch_listened_wallet_asset_balances_and_decimals(
    db: &DbClient,
    chain: EnumBlockChain,
    strategy_id: i64,
) -> Result<(HashMap<Address, U256>, HashMap<Address, usize>)> {
    // TODO MULTICHAIN: get tokens of watched wallets from all chains
    let wallet_balance_rows = db
        .execute(FunWatcherListExpertListenedWalletAssetBalanceReq {
            limit: None,
            offset: None,
            strategy_id: Some(strategy_id),
            blockchain: Some(chain),
            address: None,
            token_id: None,
        })
        .await?
        .into_rows();

    // TODO MULTICHAIN: merge token amounts of the same token to this chain's contract address
    // TODO MULTICHAIN: normalize token amounts to this chain's contract decimals
    let mut wallet_balances: HashMap<Address, U256> = HashMap::new();
    let mut wallet_decimals: HashMap<Address, usize> = HashMap::new();
    for wallet_asset in wallet_balance_rows {
        match wallet_balances.entry(wallet_asset.token_address.into()) {
            Entry::Occupied(mut entry) => {
                let token_amount = entry.get_mut();
                *token_amount = token_amount.try_checked_add(wallet_asset.balance.into())?;
            }
            Entry::Vacant(entry) => {
                entry.insert(wallet_asset.balance.into());
            }
        }
        match wallet_decimals.entry(wallet_asset.token_address.into()) {
            Entry::Vacant(entry) => {
                entry.insert(wallet_asset.token_decimals.to_usize().unwrap());
            }
            _ => continue,
        }
    }

    Ok((wallet_balances, wallet_decimals))
}

pub async fn fetch_strategy_pool_contract_asset_balances_and_decimals(
    db: &DbClient,
    chain: EnumBlockChain,
    strategy_id: i64,
) -> Result<(HashMap<Address, U256>, HashMap<Address, usize>)> {
    let strategy_pool_asset_rows = db
        .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
            strategy_pool_contract_id: None,
            strategy_id: Some(strategy_id),
            blockchain: Some(chain),
            token_address: None,
        })
        .await?
        .into_rows();

    let mut strategy_pool_balances: HashMap<Address, U256> = HashMap::new();
    let mut strategy_pool_decimals: HashMap<Address, usize> = HashMap::new();
    for strategy_pool_asset in strategy_pool_asset_rows {
        strategy_pool_balances.insert(
            strategy_pool_asset.token_address.into(),
            strategy_pool_asset.balance.into(),
        );
        strategy_pool_decimals.insert(
            strategy_pool_asset.token_address.into(),
            strategy_pool_asset.token_decimals.to_usize().unwrap(),
        );
    }

    Ok((strategy_pool_balances, strategy_pool_decimals))
}

pub fn normalize_decimals_to(
    normalize_to: usize,
    token_amount: U256,
    token_decimals: usize,
) -> Result<U256> {
    if normalize_to > token_decimals {
        Ok(token_amount.try_checked_mul(U256::exp10(normalize_to - token_decimals))?)
    } else {
        Ok(token_amount.try_checked_div(U256::exp10(token_decimals - normalize_to))?)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CopyTradeEntry {
    pub blockchain: EnumBlockChain,
    pub dex: EnumDex,
    pub token_in: Address,
    pub token_out: Address,
    // approximation
    pub amount_in: U256,
    pub amount_out: U256,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyTradePlan {
    pub trades: Vec<CopyTradeEntry>,
}

pub fn calculate_asset_values(
    amounts: HashMap<Address, U256>,
    decimals: HashMap<Address, usize>,
    prices: HashMap<Address, f64>,
) -> Result<HashMap<Address, f64>> {
    let mut values: HashMap<Address, f64> = HashMap::new();
    for (asset, amount) in amounts {
        let decimal = *decimals
            .get(&asset)
            .with_context(|| format!("decimals of asset {}", asset.to_string()))?;
        let price = *prices
            .get(&asset)
            .with_context(|| format!("price of asset {}", asset.to_string()))?;
        let value = amount.div_as_f64(U256::exp10(decimal))? * price;
        values.insert(asset, value);
    }
    Ok(values)
}

pub fn convert_amount_to_ratio(
    amounts: HashMap<Address, U256>,
    decimals: HashMap<Address, usize>,
    prices: HashMap<Address, f64>,
) -> Result<(f64, HashMap<Address, f64>)> {
    let values = calculate_asset_values(amounts, decimals, prices)?;
    let total_value = values.values().sum::<f64>();
    let mut ratios: HashMap<Address, f64> = HashMap::new();
    for (asset, value) in values {
        ratios.insert(asset, value / total_value);
    }
    Ok((total_value, ratios))
}
pub fn calculate_copy_trade_plan(
    blockchain: EnumBlockChain,
    expert_asset_amounts: HashMap<Address, U256>,
    strategy_asset_amounts: HashMap<Address, U256>,
    asset_prices: HashMap<Address, f64>,
    asset_decimals: HashMap<Address, usize>,
) -> Result<CopyTradePlan> {
    let (_, expert_asset_ratios) = convert_amount_to_ratio(
        expert_asset_amounts.clone(),
        asset_decimals.clone(),
        asset_prices.clone(),
    )?;
    let (strategy_total_value, strategy_asset_ratios) = convert_amount_to_ratio(
        strategy_asset_amounts.clone(),
        asset_decimals.clone(),
        asset_prices.clone(),
    )?;
    // select two assets so that the ratios of strategy are closer to the ratios of expert
    let mut ratio_deltas: HashMap<Address, f64> = HashMap::new();
    for key in expert_asset_amounts
        .keys()
        .cloned()
        .chain(strategy_asset_amounts.keys().cloned())
        .unique()
    {
        let expert_ratio = expert_asset_ratios.get(&key).copied().unwrap_or_default();
        let strategy_ratio = strategy_asset_ratios.get(&key).copied().unwrap_or_default();
        ratio_deltas.insert(key.clone(), expert_ratio - strategy_ratio);
    }
    let ratio_deltas_ranked_by_delta: Vec<(Address, f64)> = ratio_deltas
        .clone()
        .into_iter()
        .sorted_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap())
        .collect();
    let (token_in, token_in_ratio) = ratio_deltas_ranked_by_delta
        .last()
        .with_context(|| "no token in")?
        .to_owned();
    ensure!(
        token_in_ratio > 0.0,
        "token in ratio should be greater than 0"
    );
    let (token_out, token_out_ratio) = ratio_deltas_ranked_by_delta
        .first()
        .with_context(|| "no token out")?
        .to_owned();
    ensure!(
        token_out_ratio < 0.0,
        "token out ratio should be less than 0"
    );

    let token_delta = token_in_ratio.abs().min(token_out_ratio.abs());
    let token_in_trade_amount = U256::exp10(*asset_decimals.get(&token_in).unwrap())
        .mul_f64(strategy_total_value * token_delta / asset_prices.get(&token_in).unwrap())?;
    let token_out_trade_amount = U256::exp10(*asset_decimals.get(&token_out).unwrap())
        .mul_f64(strategy_total_value * token_delta / asset_prices.get(&token_out).unwrap())?;
    let mut trades: Vec<CopyTradeEntry> = vec![];
    trades.push(CopyTradeEntry {
        blockchain,
        dex: EnumDex::PancakeSwap,
        token_in: token_in.clone(),
        token_out: token_out.clone(),
        amount_in: token_in_trade_amount,
        amount_out: token_out_trade_amount,
    });
    Ok(CopyTradePlan { trades })
}

pub async fn load_dex_path(
    db: &DbClient,
    token_in: Address,
    token_out: Address,
) -> Result<PancakePairPathSet> {
    todo!()
}

pub async fn execute_copy_trade_plan(
    pool: &EthereumRpcConnectionPool,
    db: &DbClient,
    copy_trade_plan: CopyTradePlan,
    pancakeswap_contract: &PancakeSmartRouterV3Contract<EitherTransport>,
    signer: impl Key + Clone,
) -> Result<TransactionReceipt> {
    let trade = copy_trade_plan.trades.first().context("no trade")?;
    let conn = pool.get(trade.blockchain).await?;
    let paths = load_dex_path(db, trade.token_in, trade.token_out).await?;
    copy_trade_and_ensure_success(
        pancakeswap_contract,
        &conn,
        CONFIRMATIONS,
        MAX_RETRIES,
        POLL_INTERVAL,
        signer,
        &paths,
        trade.amount_in,
        trade.amount_out.mul_f64(0.98)?,
        DynLogger::empty(),
    )
    .await
}
pub async fn get_token_prices(
    db: &DbClient,
    cmc: &CoinMarketCap,
    tokens: Vec<Address>,
) -> Result<Vec<f64>> {
    let mut symbols = Vec::new();
    for token in tokens {
        let tk = db
            .execute(FunUserListEscrowTokenContractAddressReq {
                limit: 1,
                offset: 0,
                token_id: None,
                blockchain: None,
                address: Some(token.into()),
                symbol: None,
                is_stablecoin: None,
            })
            .await?
            .into_result()
            .context("no token")?;
        symbols.push(tk.symbol);
    }
    let prices = cmc.get_usd_prices_by_symbol(&symbols).await?;
    Ok(prices)
}

pub async fn execute_copy_trade(
    pool: &EthereumRpcConnectionPool,
    db: &DbClient,
    pancakeswap_contract: &PancakeSmartRouterV3Contract<EitherTransport>,
    signer: impl Key + Clone,
    blochchain: EnumBlockChain,
    cmc: &CoinMarketCap,
    strategy_id: i64,
) -> Result<()> {
    let (strategy_asset_balances, strategy_asset_decimals) =
        fetch_strategy_pool_contract_asset_balances_and_decimals(db, blochchain, strategy_id)
            .await?;
    let (expert_asset_balances, expert_asset_decimals) =
        fetch_listened_wallet_asset_balances_and_decimals(db, blochchain, strategy_id).await?;
    let symbols: Vec<Address> = strategy_asset_balances
        .keys()
        .chain(expert_asset_balances.keys())
        .unique()
        .cloned()
        .collect();
    let prices = get_token_prices(db, cmc, symbols.clone()).await?;
    let prices = symbols.into_iter().zip(prices.into_iter()).collect();
    let decimals: HashMap<Address, usize> = strategy_asset_decimals
        .into_iter()
        .merge(expert_asset_decimals.into_iter())
        .collect();
    let plan = calculate_copy_trade_plan(
        blochchain,
        expert_asset_balances,
        strategy_asset_balances,
        prices,
        decimals,
    )?;
    execute_copy_trade_plan(pool, db, plan, pancakeswap_contract, signer).await?;
    Ok(())
}
