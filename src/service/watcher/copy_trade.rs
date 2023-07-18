use std::collections::hash_map::Entry;
use std::collections::HashMap;

use eyre::*;
use num::ToPrimitive;
use web3::types::{Address, H256, U256};

use eth_sdk::evm::DexTrade;
use eth_sdk::ScaledMath;
use gen::database::*;
use gen::model::EnumBlockChain;
use lib::database::DbClient;

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
