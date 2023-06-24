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
) -> Result<i64> {
    let strategy_id: i64 = db
        .execute(FunUserGetStrategyIdFromWatchingWalletReq {
            blockchain,
            address: format!("{:?}", wallet_address),
        })
        .await?
        .into_result()
        .context("error fetching strategy_id from tbl.strategy")?
        .strategy_id;

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

pub async fn update_expert_listened_wallet_asset_ledger(
    db: &DbClient,
    _strategy_id: i64,
    trade: &DexTrade,
    token_out_id: i64,
    token_in_id: i64,
    blockchain: EnumBlockChain,
) -> Result<()> {
    // correctly adding wallet balance to tbl.strategy_initial_token ratio is not possible because expert can have multiple watching wallets in one chain
    let expert_watched_wallet_address = trade.caller;

    match db
        .execute(FunWatcherListExpertListenedWalletAssetBalanceReq {
            limit: 1,
            blockchain: Some(blockchain),
            address: Some(format!("{:?}", expert_watched_wallet_address)),
            token_id: Some(token_out_id),
            offset: 0,
        })
        .await?
        .into_result()
    {
        Some(tk) => {
            /* if token_in is already in the database, update it's amount */
            let old_amount = U256::from_dec_str(&tk.balance)?;
            let new_amount = old_amount.try_checked_sub(trade.amount_out)?;
            db.execute(FunWatcherUpsertExpertListenedWalletAssetBalanceReq {
                address: format!("{:?}", expert_watched_wallet_address),
                blockchain,
                token_id: token_out_id,
                old_balance: tk.balance,
                new_balance: format!("{:?}", new_amount),
            })
            .await?;
        }
        None => {
            // what should we do when we have nothing to subtract from?
        }
    };

    match db
        .execute(FunWatcherListExpertListenedWalletAssetBalanceReq {
            limit: 1,
            blockchain: Some(blockchain),
            address: Some(format!("{:?}", expert_watched_wallet_address)),
            token_id: Some(token_in_id),
            offset: 0,
        })
        .await?
        .into_result()
    {
        Some(tk) => {
            /* if token_in is already in the database, update it's amount, or remove it new amount is 0 */
            let old_amount = U256::from_dec_str(&tk.balance)?;
            let new_amount = old_amount.try_checked_add(trade.amount_in)?;
            db.execute(FunWatcherUpsertExpertListenedWalletAssetBalanceReq {
                address: format!("{:?}", expert_watched_wallet_address),
                blockchain,
                token_id: token_in_id,
                old_balance: tk.balance,
                new_balance: format!("{:?}", new_amount),
            })
            .await?;
        }
        None => {
            let old_amount = U256::from(0);
            let new_amount = trade.amount_in;
            db.execute(FunWatcherUpsertExpertListenedWalletAssetBalanceReq {
                address: format!("{:?}", expert_watched_wallet_address),
                blockchain,
                token_id: token_in_id,
                old_balance: format!("{:?}", old_amount),
                new_balance: format!("{:?}", new_amount),
            })
            .await?;
        }
    };

    Ok(())
}
