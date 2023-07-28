pub mod cmc;

use std::collections::HashMap;

use async_trait::async_trait;
use eyre::*;

#[derive(Debug, Clone)]
pub struct AssetPriceByPeriod {
    pub symbol: String,
    pub price_latest: f64,
    pub price_1d: f64,
    pub price_7d: f64,
    pub price_30d: f64,
}

#[async_trait]
pub trait AssetInfoClient: Sync + Send {
    async fn get_usd_price_latest(&self, symbols: &[String]) -> Result<HashMap<String, f64>>;
    async fn get_usd_price_period(
        &self,
        symbols: &[String],
    ) -> Result<HashMap<String, AssetPriceByPeriod>>;
}
