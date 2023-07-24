use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::time::Duration;

use eyre::*;
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
    let all_token_contract_rows = db
        .execute(FunAdminListEscrowTokenContractAddressReq {
            limit: None,
            offset: None,
            blockchain: None,
            token_address: None,
            token_id: None,
        })
        .await?
        .into_rows();

    let mut unique_token_symbols: HashMap<String, ()> = HashMap::new();
    for token_contract_row in all_token_contract_rows {
        match unique_token_symbols.entry(token_contract_row.symbol) {
            Entry::Occupied(_) => continue,
            Entry::Vacant(entry) => {
                entry.insert(());
            }
        }
    }

    let quoted_prices = cmc
        .get_usd_prices_by_symbol(
            &unique_token_symbols
                .keys()
                .cloned()
                .collect::<Vec<String>>(),
        )
        .await?;

    db.execute(FunAssetPriceInsertAssetPricesReq {
        symbols: quoted_prices.keys().cloned().collect(),
        prices: quoted_prices.values().cloned().collect(),
    })
    .await?;

    Ok(())
}

async fn delete_old_price_entries(db: &DbClient) -> Result<()> {
    db.execute(FunAssetPriceDeleteOldAssetPriceEntriesReq {})
        .await?;
    Ok(())
}
