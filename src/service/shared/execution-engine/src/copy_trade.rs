use api::cmc::CoinMarketCap;
use eth_sdk::execute_transaction_and_ensure_success;
use eth_sdk::pancake_swap::execute::PancakeSmartRouterContract;
use eth_sdk::pancake_swap::pair_paths::parse_pancake_swap_dex_path;
use eth_sdk::pancake_swap::PancakePairPathSet;
use eth_sdk::utils::decimal_to_u256;
use eth_sdk::{
    EitherTransport, EthereumRpcConnection, EthereumRpcConnectionPool, ScaledMath, CONFIRMATIONS,
    MAX_RETRIES, POLL_INTERVAL,
};
use eyre::*;
use gen::database::*;
use gen::model::{EnumBlockChain, EnumDex};
use itertools::Itertools;
use lib::database::DbClient;
use lib::log::DynLogger;
use num::Zero;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ops::{AddAssign, SubAssign};
use tracing::info;
use web3::signing::Key;
use web3::types::{Address, TransactionReceipt, U256};

pub async fn fetch_listened_wallet_asset_balances_and_decimals(
    db: &DbClient,
    chain: EnumBlockChain,
    strategy_id: i64,
) -> Result<(HashMap<Address, Decimal>, HashMap<Address, u32>)> {
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
    let mut wallet_balances: HashMap<Address, Decimal> = HashMap::new();
    let mut wallet_decimals: HashMap<Address, u32> = HashMap::new();
    for wallet_asset in wallet_balance_rows {
        match wallet_balances.entry(wallet_asset.token_address.into()) {
            Entry::Occupied(mut entry) => {
                let token_amount = entry.get_mut();
                *token_amount += wallet_asset.balance;
            }
            Entry::Vacant(entry) => {
                entry.insert(wallet_asset.balance);
            }
        }
        match wallet_decimals.entry(wallet_asset.token_address.into()) {
            Entry::Vacant(entry) => {
                entry.insert(wallet_asset.token_decimals as _);
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
) -> Result<(HashMap<Address, Decimal>, HashMap<Address, u32>)> {
    let strategy_pool_asset_rows = db
        .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
            strategy_pool_contract_id: None,
            strategy_id: Some(strategy_id),
            blockchain: Some(chain),
            token_address: None,
        })
        .await?
        .into_rows();

    let mut strategy_pool_balances: HashMap<Address, Decimal> = HashMap::new();
    let mut strategy_pool_decimals: HashMap<Address, u32> = HashMap::new();
    for wallet_asset in strategy_pool_asset_rows {
        match strategy_pool_balances.entry(wallet_asset.token_address.into()) {
            Entry::Occupied(mut entry) => {
                let token_amount = entry.get_mut();
                *token_amount += wallet_asset.balance;
            }
            Entry::Vacant(entry) => {
                entry.insert(wallet_asset.balance);
            }
        }
        match strategy_pool_decimals.entry(wallet_asset.token_address.into()) {
            Entry::Vacant(entry) => {
                entry.insert(wallet_asset.token_decimals as _);
            }
            _ => continue,
        }
    }

    Ok((strategy_pool_balances, strategy_pool_decimals))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CopyTradeEntry {
    pub blockchain: EnumBlockChain,
    pub dex: EnumDex,
    pub token_in: Address,
    pub token_out: Address,
    pub amount_in: Decimal,
    pub amount_in_native: U256,
    pub amount_out: Decimal,
    pub amount_out_native: U256,
    pub trade_ratio: Decimal,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyTradePlan {
    pub trades: Vec<CopyTradeEntry>,
}

/// calculate the value of each asset in USD
pub fn calculate_asset_values(
    amounts: HashMap<Address, Decimal>,
    prices: HashMap<Address, f64>,
) -> Result<HashMap<Address, Decimal>> {
    let mut values: HashMap<Address, Decimal> = HashMap::new();
    for (asset, amount) in amounts {
        let price = *prices
            .get(&asset)
            .with_context(|| format!("price of asset {}", asset.to_string()))?;
        let value = amount * Decimal::from_f64(price).unwrap();
        values.insert(asset, value);
    }
    Ok(values)
}
/// convert amounts to ratios among all assets, terms of USD
pub fn convert_amount_to_ratio(
    amounts: HashMap<Address, Decimal>,
    prices: HashMap<Address, f64>,
) -> Result<(Decimal, HashMap<Address, Decimal>)> {
    let values = calculate_asset_values(amounts, prices)?;
    let total_value = values.values().sum::<Decimal>();
    let mut ratios: HashMap<Address, Decimal> = HashMap::new();
    for (asset, value) in values {
        ratios.insert(asset, value / total_value);
    }
    Ok((total_value, ratios))
}

/// core algorithm of copy trading
/// also used in backing to calculate which assets to buy
pub fn calculate_copy_trade_plan(
    blockchain: EnumBlockChain,
    expert_asset_amounts: HashMap<Address, Decimal>,
    strategy_asset_amounts: HashMap<Address, Decimal>,
    asset_prices: HashMap<Address, f64>,
    asset_decimals: HashMap<Address, u32>,
) -> Result<CopyTradePlan> {
    // first convert expert and strategy amounts to ratios in USD
    let (_, expert_asset_ratios) =
        convert_amount_to_ratio(expert_asset_amounts.clone(), asset_prices.clone())?;
    let (strategy_total_value, strategy_asset_ratios) =
        convert_amount_to_ratio(strategy_asset_amounts.clone(), asset_prices.clone())?;
    // println!("{:#?}", strategy_asset_ratios);

    // select two assets so that the ratios of strategy are closer to the ratios of expert
    let mut ratio_deltas: HashMap<Address, Decimal> = HashMap::new();
    for key in expert_asset_amounts
        .keys()
        .cloned()
        .chain(strategy_asset_amounts.keys().cloned())
        .unique()
    {
        let expert_ratio = expert_asset_ratios.get(&key).copied().unwrap_or_default();
        let strategy_ratio = strategy_asset_ratios.get(&key).copied().unwrap_or_default();
        ratio_deltas.insert(key.clone(), strategy_ratio - expert_ratio);
    }
    let mut plan = CopyTradePlan { trades: vec![] };
    // here comes the core logic
    while let (Some((&token_in, &token_in_ratio)), Some((&token_out, &token_out_ratio))) = {
        // find the token we sell
        // it should be most excessively sufficient in strategy asset and insufficient in expert asset
        // it can be used to buy other tokens several times, but not exceeding excessive ratio
        let token_in_pair = ratio_deltas
            .iter()
            .filter(|x| *x.1 > Decimal::zero())
            .filter(|x| {
                strategy_asset_amounts.get(x.0).unwrap_or(&Decimal::zero()) > &Decimal::zero()
            })
            .max_by(|x, y| x.1.partial_cmp(y.1).unwrap());
        // find the token we buy
        // it should be most insufficient in strategy asset and sufficient in expert asset
        // it can be sold several times, but not exceeding insufficient ratio
        let token_out_pair = ratio_deltas
            .iter()
            .filter(|x| *x.1 < Decimal::zero())
            .filter(|x| {
                expert_asset_amounts.get(x.0).unwrap_or(&Decimal::zero()) > &Decimal::zero()
            })
            .min_by(|x, y| x.1.partial_cmp(y.1).unwrap());
        (token_in_pair, token_out_pair)
    } {
        // we take the minimal ratio of the two tokens
        let trade_ratio = std::cmp::min(token_in_ratio.abs(), token_out_ratio.abs());
        // convert trade ratio to amount need to spend and amount to receive
        let amount_in = strategy_total_value * trade_ratio
            / Decimal::from_f64(*asset_prices.get(&token_in).unwrap()).unwrap();
        let amount_out = strategy_total_value * trade_ratio
            / Decimal::from_f64(*asset_prices.get(&token_out).unwrap()).unwrap();
        ratio_deltas
            .get_mut(&token_in)
            .unwrap()
            .sub_assign(trade_ratio);
        ratio_deltas
            .get_mut(&token_out)
            .unwrap()
            .add_assign(trade_ratio);
        // add trade to plan
        plan.trades.push(CopyTradeEntry {
            blockchain,
            dex: EnumDex::PancakeSwap,
            token_in,
            token_out,
            amount_in,
            amount_in_native: decimal_to_u256(amount_in, *asset_decimals.get(&token_in).unwrap()),
            amount_out,
            amount_out_native: decimal_to_u256(
                amount_out,
                *asset_decimals.get(&token_out).unwrap(),
            ),
            trade_ratio,
        });
    }

    Ok(plan)
}

pub async fn load_dex_path(
    db: &DbClient,
    conn: &EthereumRpcConnection,
    token_in: Address,
    token_out: Address,
) -> Result<PancakePairPathSet> {
    let dex_path = db
        .execute(FunWatcherListDexPathForPairReq {
            token_in_address: token_in.into(),
            token_out_address: token_out.into(),
            blockchain: EnumBlockChain::BscMainnet,
            dex: Some(EnumDex::PancakeSwap),
            format: None,
        })
        .await?
        .into_result()
        .context("no dex path")?;
    let dex_path = parse_pancake_swap_dex_path(dex_path, conn).await?;
    Ok(dex_path)
}

pub async fn execute_copy_trade_plan(
    pool: &EthereumRpcConnectionPool,
    db: &DbClient,
    copy_trade_plan: CopyTradePlan,
    pancakeswap_contract: &PancakeSmartRouterContract<EitherTransport>,
    signer: impl Key + Clone,
) -> Result<TransactionReceipt> {
    let trade = copy_trade_plan.trades.first().context("no trade")?;
    let conn = pool.get(trade.blockchain).await?;
    let paths = load_dex_path(db, &conn, trade.token_in, trade.token_out).await?;
    let amount_out_mininum = trade.amount_out * Decimal::from_f64(0.98).unwrap();
    let amount_out_minimum_native = trade.amount_out_native.mul_f64(0.98)?;
    let copy_trade_transaction = || {
        pancakeswap_contract.copy_trade(
            &conn,
            signer.clone(),
            paths.clone(),
            trade.amount_in_native,
            amount_out_minimum_native,
        )
    };

    info!(
        "copy_trade_and_ensure_success: amount_in: {}, amount_out_minimum: {}",
        trade.amount_in, amount_out_mininum
    );

    let trade_hash = execute_transaction_and_ensure_success(
        copy_trade_transaction,
        &conn,
        CONFIRMATIONS,
        MAX_RETRIES,
        POLL_INTERVAL,
        &DynLogger::empty(),
    )
    .await?;

    info!("copy_trade_and_ensure_success: tx_hash: {:?}", trade_hash);

    Ok(conn
        .eth()
        .transaction_receipt(trade_hash)
        .await?
        .context("could not find transaction receipt for copy trade")?)
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
    Ok(symbols
        .iter()
        .flat_map(|t| prices.get(t))
        .copied()
        .collect())
}

pub async fn execute_copy_trade(
    pool: &EthereumRpcConnectionPool,
    db: &DbClient,
    pancakeswap_contract: &PancakeSmartRouterContract<EitherTransport>,
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
    let decimals: HashMap<Address, u32> = strategy_asset_decimals
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_trading_empty() -> Result<()> {
        let expert_amounts = HashMap::new();
        let strategy_amounts = HashMap::new();
        let prices = HashMap::new();
        let decimals = HashMap::new();
        let plan = calculate_copy_trade_plan(
            EnumBlockChain::LocalNet,
            expert_amounts,
            strategy_amounts,
            prices,
            decimals,
        )?;
        assert_eq!(plan.trades.len(), 0);
        Ok(())
    }
    #[test]
    fn test_copy_trading_synced() -> Result<()> {
        let token_a = Address::from_low_u64_be(1);
        let token_b = Address::from_low_u64_be(2);

        let mut expert_amounts = HashMap::new();
        expert_amounts.insert(token_a, 0.into());
        expert_amounts.insert(token_b, 100.into());
        let mut strategy_amounts = HashMap::new();
        strategy_amounts.insert(token_a, 10.into());
        strategy_amounts.insert(token_b, 0.into());
        let mut prices = HashMap::new();
        prices.insert(token_a, 1.0);
        prices.insert(token_b, 1.0);
        let mut decimals = HashMap::new();
        decimals.insert(token_a, 0);
        decimals.insert(token_b, 0);
        let plan = calculate_copy_trade_plan(
            EnumBlockChain::LocalNet,
            expert_amounts,
            strategy_amounts,
            prices,
            decimals,
        )?;
        assert_eq!(plan.trades.len(), 1);
        assert_eq!(plan.trades[0].token_in, token_a);
        assert_eq!(plan.trades[0].token_out, token_b);
        assert_eq!(plan.trades[0].amount_in, U256::from(10));
        assert_eq!(plan.trades[0].amount_out, U256::from(10));
        Ok(())
    }
    #[test]
    fn test_copy_trading_unsynced() -> Result<()> {
        let token_a = Address::from_low_u64_be(1);
        let token_b = Address::from_low_u64_be(2);
        let token_c = Address::from_low_u64_be(3);
        let mut expert_amounts = HashMap::new();
        expert_amounts.insert(token_a, 0.into());
        expert_amounts.insert(token_b, 100.into());
        expert_amounts.insert(token_c, 1000.into());
        let mut strategy_amounts = HashMap::new();
        strategy_amounts.insert(token_a, 10.into());
        strategy_amounts.insert(token_b, 0.into());
        strategy_amounts.insert(token_c, 0.into());
        let mut prices = HashMap::new();
        prices.insert(token_a, 1.0);
        prices.insert(token_b, 1.0);
        prices.insert(token_c, 0.4);
        let mut decimals = HashMap::new();
        decimals.insert(token_a, 0);
        decimals.insert(token_b, 0);
        decimals.insert(token_c, 0);
        let plan = calculate_copy_trade_plan(
            EnumBlockChain::LocalNet,
            expert_amounts,
            strategy_amounts,
            prices,
            decimals,
        )?;
        println!("{:#?}", plan);
        assert_eq!(plan.trades.len(), 2);
        assert_eq!(plan.trades[0].token_in, token_a);
        assert_eq!(plan.trades[0].token_out, token_c);
        assert_eq!(plan.trades[0].amount_in, U256::from(8));
        assert_eq!(plan.trades[0].amount_out, U256::from(20));
        assert_eq!(plan.trades[1].token_in, token_a);
        assert_eq!(plan.trades[1].token_out, token_b);
        assert_eq!(plan.trades[1].amount_in, U256::from(2));
        assert_eq!(plan.trades[1].amount_out, U256::from(2));

        Ok(())
    }
}
