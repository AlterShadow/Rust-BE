use crate::calc::ScaledMath;
use crate::evm::DexTrade;
use eyre::*;
use gen::database::*;
use gen::model::EnumBlockChain;
use lib::database::DbClient;
use web3::types::{Address, U256};

mod parse;

pub use parse::*;

pub async fn get_strategy_id_from_watching_wallet(
    db: &DbClient,
    blockchain: EnumBlockChain,
    wallet_address: Address,
) -> Result<Option<i64>> {
    let strategy_id = db
        .execute(FunUserGetStrategyIdFromWatchingWalletReq {
            blockchain,
            address: wallet_address.into(),
        })
        .await?
        .into_result()
        .map(|x| x.strategy_id);

    Ok(strategy_id)
}

pub async fn get_user_id_from_strategy(db: &DbClient, strategy_id: i64) -> Result<i64> {
    let strategy_id: i64 = db
        .execute(FunAdminListStrategiesReq {
            limit: 1,
            offset: 0,
            strategy_id: Some(strategy_id),
            strategy_name: None,
            expert_public_id: None,
            expert_name: None,
            description: None,
            approved: None,
            pending_approval: None,
        })
        .await?
        .into_result()
        .context("error fetching strategy_id from tbl.strategy")?
        .creator_id;

    Ok(strategy_id)
}

pub async fn update_expert_listened_wallet_asset_balance_cache(
    db: &DbClient,
    _strategy_id: i64,
    trade: &DexTrade,
    token_out_id: i64,
    token_in_id: i64,
    blockchain: EnumBlockChain,
) -> Result<()> {
    // correctly adding wallet balance to tbl.strategy_initial_token ratio is not possible because expert can have multiple watching wallets in one chain
    let expert_watched_wallet_address = trade.caller;
    let old_amount: U256 = db
        .execute(FunWatcherListExpertListenedWalletAssetBalanceReq {
            limit: 1,
            blockchain: Some(blockchain),
            address: Some(expert_watched_wallet_address.into()),
            token_id: Some(token_in_id),
            offset: 0,
        })
        .await?
        .into_result()
        .map(|tk| tk.balance.into())
        .unwrap_or_else(|| 0.into());
    if old_amount > 0.into() {
        let new_amount = old_amount.try_checked_sub(trade.amount_in)?;
        /* if token_in is already in the database, update it's amount */
        db.execute(FunWatcherUpsertExpertListenedWalletAssetBalanceReq {
            address: expert_watched_wallet_address.into(),
            blockchain,
            token_id: token_in_id,
            old_balance: old_amount.into(),
            new_balance: new_amount.into(),
        })
        .await?;
    };
    let old_amount: U256 = db
        .execute(FunWatcherListExpertListenedWalletAssetBalanceReq {
            limit: 1,
            blockchain: Some(blockchain),
            address: Some(expert_watched_wallet_address.into()),
            token_id: Some(token_out_id),
            offset: 0,
        })
        .await?
        .into_result()
        .map(|tk| tk.balance.into())
        .unwrap_or_else(|| 0.into());
    let new_amount = old_amount.try_checked_add(trade.amount_out)?;
    db.execute(FunWatcherUpsertExpertListenedWalletAssetBalanceReq {
        address: expert_watched_wallet_address.into(),
        blockchain,
        token_id: token_out_id,
        old_balance: old_amount.into(),
        new_balance: new_amount.into(),
    })
    .await?;

    Ok(())
}

pub async fn update_user_strategy_pool_asset_balances_on_copy_trade(
    db: &DbClient,
    blockchain: EnumBlockChain,
    strategy_pool_contract_id: i64,
    sp_sold_asset_address: Address,
    sp_sold_asset_amount: U256,
    sp_sold_asset_previous_amount: U256,
    sp_bought_asset_address: Address,
    sp_bought_asset_amount: U256,
) -> Result<()> {
    /* get strategy wallets that hold sold asset */
    let strategy_wallet_sold_asset_rows = db
        .execute(FunUserListUserStrategyPoolContractAssetBalancesReq {
            strategy_pool_contract_id,
            token_address: Some(sp_sold_asset_address.into()),
            blockchain: Some(blockchain),
            user_id: None,
            strategy_wallet_id: None,
        })
        .await?
        .into_rows();

    /* update user balances and add ledger entries */
    for strategy_wallet_sold_asset_row in strategy_wallet_sold_asset_rows {
        let currently_owned_sold_asset: U256 = strategy_wallet_sold_asset_row.balance.into();
        let subtracted_sold_amount = currently_owned_sold_asset
            .mul_div_rounding_up(sp_sold_asset_amount, sp_sold_asset_previous_amount)?;
        let new_sold_asset_balance = currently_owned_sold_asset
            .try_checked_sub(subtracted_sold_amount)
            .unwrap_or(U256::zero());
        let added_bought_amount = sp_bought_asset_amount
            .mul_div(currently_owned_sold_asset, sp_sold_asset_previous_amount)?;
        /* update user strategy pool contract asset balances */
        db.execute(FunUserUpsertUserStrategyPoolContractAssetBalanceReq {
            strategy_wallet_id: strategy_wallet_sold_asset_row.strategy_wallet_id,
            strategy_pool_contract_id,
            token_address: sp_sold_asset_address.into(),
            blockchain,
            old_balance: currently_owned_sold_asset.into(),
            new_balance: new_sold_asset_balance.into(),
        })
        .await?;
        match db
            .execute(FunUserListUserStrategyPoolContractAssetBalancesReq {
                strategy_pool_contract_id,
                token_address: Some(sp_bought_asset_address.into()),
                blockchain: Some(blockchain),
                user_id: Some(strategy_wallet_sold_asset_row.user_id),
                strategy_wallet_id: Some(strategy_wallet_sold_asset_row.strategy_wallet_id),
            })
            .await?
            .into_result()
        {
            Some(bought_asset_old_balance_row) => {
                let bought_asset_old_balance: U256 = bought_asset_old_balance_row.balance.into();
                /* if user already held bought asset, add to old balance */
                db.execute(FunUserUpsertUserStrategyPoolContractAssetBalanceReq {
                    strategy_wallet_id: strategy_wallet_sold_asset_row.strategy_wallet_id,
                    strategy_pool_contract_id,
                    token_address: sp_bought_asset_address.into(),
                    blockchain,
                    old_balance: bought_asset_old_balance.into(),
                    new_balance: bought_asset_old_balance
                        .try_checked_add(added_bought_amount)?
                        .into(),
                })
                .await?;
            }
            None => {
                /* if user did not hold bought asset, use new amount */
                db.execute(FunUserUpsertUserStrategyPoolContractAssetBalanceReq {
                    strategy_wallet_id: strategy_wallet_sold_asset_row.strategy_wallet_id,
                    strategy_pool_contract_id,
                    token_address: sp_bought_asset_address.into(),
                    blockchain,
                    old_balance: U256::zero().into(),
                    new_balance: added_bought_amount.into(),
                })
                .await?;
            }
        }

        /* add entries to ledger */
        db.execute(FunUserAddUserStrategyPoolContractAssetLedgerEntryReq {
            strategy_wallet_id: strategy_wallet_sold_asset_row.strategy_wallet_id,
            strategy_pool_contract_id,
            token_address: sp_sold_asset_address.into(),
            blockchain,
            amount: subtracted_sold_amount.into(),
            is_add: false,
        })
        .await?;
        db.execute(FunUserAddUserStrategyPoolContractAssetLedgerEntryReq {
            strategy_wallet_id: strategy_wallet_sold_asset_row.strategy_wallet_id,
            strategy_pool_contract_id,
            token_address: sp_bought_asset_address.into(),
            blockchain,
            amount: added_bought_amount.into(),
            is_add: true,
        })
        .await?;
    }

    Ok(())
}
