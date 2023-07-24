use eyre::*;
use serde::Deserialize;

use api::cmc::CoinMarketCap;
use lib::config::load_config;
use lib::database::{connect_to_database, DatabaseConfig};
use lib::log::{setup_logs, LogLevel};
use secrecy::{ExposeSecret, SecretString};

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

    Ok(())
}

async fn fill_asset_price_cache() -> Result<()> {
    Ok(())
}
