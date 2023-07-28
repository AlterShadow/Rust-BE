use std::time::Duration;

use eyre::*;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use tokio::time::sleep;
use tracing::error;

use api::cmc::CoinMarketCap;
use lib::config::load_config;
use lib::database::{connect_to_database, DatabaseConfig};
use lib::log::{setup_logs, LogLevel};
use mc2fi_asset_price::update_price::{
    delete_old_price_entries, fill_asset_price_cache, fill_past_prices_on_startup,
};

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
