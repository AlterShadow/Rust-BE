use eyre::*;

use lib::database::{DbClient, ToSql};

use web3::types::{Address, U256};

mod parse;
use crate::calc::ScaledMath;
use crate::evm::DexTrade;
use gen::database::*;
use gen::model::EnumBlockChain;
pub use parse::*;

pub async fn get_strategy_id_from_watching_wallet(
    db: &DbClient,
    chain: &EnumBlockChain,
    wallet: &Address,
) -> Result<i64> {
    let strategy_id: i64 = db
        .query(
            "
				SELECT fkey_strategy_id
				FROM tbl.strategy_watching_wallet
				WHERE address = $1 AND blockchain = $2
			",
            &vec![
                &format!("{:?}", wallet) as &(dyn ToSql + Sync),
                chain as &(dyn ToSql + Sync),
            ],
        )
        .await?
        .first()
        .context("error fetching fkey_strategy_id from tbl.strategy_watching_wallet")?
        .try_get("fkey_strategy_id")
        .context("error parsing fkey_strategy_id from tbl.strategy_watching_wallet")?;

    Ok(strategy_id)
}

pub async fn get_user_id_from_strategy(db: &DbClient, strategy_id: i64) -> Result<i64> {
    let strategy_id: i64 = db
        .query(
            "
				SELECT fkey_user_id
				FROM tbl.strategy
				WHERE pkey_id = $1
			",
            &vec![&strategy_id as &(dyn ToSql + Sync)],
        )
        .await?
        .first()
        .context("error fetching fkey_user_id from tbl.strategy")?
        .try_get("fkey_user_id")
        .context("error parsing fkey_user_id from tbl.strategy")?;

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
            let old_amount = U256::from_dec_str(&tk.entry)?;
            let new_amount = old_amount.try_checked_sub(trade.amount_out)?;
            db.execute(FunWatcherUpsertExpertListenedWalletAssetBalanceReq {
                address: format!("{:?}", expert_watched_wallet_address),
                blockchain,
                token_id: token_out_id,
                old_entry: tk.entry,
                new_entry: format!("{:?}", new_amount),
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
            let old_amount = U256::from_dec_str(&tk.entry)?;
            let new_amount = old_amount.try_checked_add(trade.amount_in)?;
            db.execute(FunWatcherUpsertExpertListenedWalletAssetBalanceReq {
                address: format!("{:?}", expert_watched_wallet_address),
                blockchain,
                token_id: token_in_id,
                old_entry: tk.entry,
                new_entry: format!("{:?}", new_amount),
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
                old_entry: format!("{:?}", old_amount),
                new_entry: format!("{:?}", new_amount),
            })
            .await?;
        }
    };

    Ok(())
}
