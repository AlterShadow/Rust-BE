use chrono::Utc;
use eyre::*;
use itertools::Itertools;

use api::cmc::CoinMarketCap;
use gen::database::*;
use lib::database::DbClient;

pub async fn fill_asset_price_cache(db: &DbClient, cmc: &CoinMarketCap) -> Result<()> {
    let unique_asset_symbols = get_unique_asset_symbols(&db).await?;

    if unique_asset_symbols.len() == 0 {
        bail!("no unique asset symbols found");
    }

    for chunk in unique_asset_symbols.chunks(30) {
        let prices = cmc.get_usd_prices_by_symbol(&chunk).await?;

        db.execute(FunAssetPriceInsertAssetPricesReq {
            symbols: prices.keys().cloned().collect(),
            prices: prices.values().cloned().collect(),
            timestamps: None,
        })
        .await?;
    }

    Ok(())
}

pub async fn delete_old_price_entries(db: &DbClient) -> Result<()> {
    db.execute(FunAssetPriceDeleteOldAssetPriceEntriesReq {})
        .await?;
    Ok(())
}

pub async fn get_unique_asset_symbols(db: &DbClient) -> Result<Vec<String>> {
    let all_token_contract_rows = db
        .execute(FunAdminListEscrowTokenContractAddressReq {
            limit: None,
            offset: None,
            blockchain: None,
            token_address: None,
            symbol: None,
            token_id: None,
        })
        .await?
        .into_rows();

    Ok(all_token_contract_rows
        .into_iter()
        .map(|x| x.symbol)
        .unique()
        .collect::<Vec<String>>())
}

pub async fn fill_past_prices_on_startup(db: &DbClient, cmc: &CoinMarketCap) -> Result<()> {
    let unique_asset_symbols = get_unique_asset_symbols(&db).await?;

    if unique_asset_symbols.len() == 0 {
        bail!("no unique asset symbols found");
    }

    let now = Utc::now();
    for chunk in unique_asset_symbols.chunks(30) {
        for days_ago in [1, 7, 30] {
            let prices = cmc.get_usd_price_days_ago(chunk, days_ago, false).await?;
            let timestamp_days_ago = now
                .checked_sub_signed(chrono::Duration::days(days_ago as i64))
                .context("could not subtract days from current timestamp")?
                .timestamp();
            let timestamps = vec![timestamp_days_ago; prices.len()];

            db.execute(FunAssetPriceInsertAssetPricesReq {
                symbols: prices.keys().cloned().collect(),
                prices: prices.values().cloned().collect(),
                timestamps: Some(timestamps),
            })
            .await?;
        }
    }

    Ok(())
}
