use std::time::Duration;

use chrono::Utc;
use eyre::*;
use itertools::Itertools;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use tokio::time::sleep;
use tracing::error;

use api::cmc::CoinMarketCap;
use gen::database::*;
use lib::config::load_config;
use lib::database::{connect_to_database, DatabaseConfig, DbClient};
use lib::log::{setup_logs, LogLevel};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub app_db: DatabaseConfig,
    pub log_level: LogLevel,
    pub cmc_api_key: SecretString,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config: Config = load_config("asset_price".to_owned())?;
    setup_logs(config.log_level)?;
    let db = connect_to_database(config.app_db).await?;
    let cmc_client = CoinMarketCap::new(config.cmc_api_key.expose_secret())?;

    fill_past_prices_on_startup(&db, &cmc_client).await?;
    loop {
        match fill_asset_price_cache(&db, &cmc_client).await {
            Ok(_) => {}
            Err(e) => {
                error!("error inserting to price cache: {}", e);
            }
        }
        match delete_old_price_entries(&db).await {
            Ok(_) => {}
            Err(e) => {
                error!("error deleting old price entries: {}", e);
            }
        }
        sleep(Duration::from_secs(60)).await;
    }
}

async fn fill_asset_price_cache(db: &DbClient, cmc: &CoinMarketCap) -> Result<()> {
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

async fn delete_old_price_entries(db: &DbClient) -> Result<()> {
    db.execute(FunAssetPriceDeleteOldAssetPriceEntriesReq {})
        .await?;
    Ok(())
}

async fn get_unique_asset_symbols(db: &DbClient) -> Result<Vec<String>> {
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

async fn fill_past_prices_on_startup(db: &DbClient, cmc: &CoinMarketCap) -> Result<()> {
    let unique_asset_symbols = get_unique_asset_symbols(&db).await?;

    if unique_asset_symbols.len() == 0 {
        bail!("no unique asset symbols found");
    }

    let now = Utc::now();
    for chunk in unique_asset_symbols.chunks(30) {
        for days_ago in [1, 30] {
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
