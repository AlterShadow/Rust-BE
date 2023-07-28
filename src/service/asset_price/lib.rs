use std::collections::HashMap;

use async_trait::async_trait;
use eyre::*;

use api::{AssetInfoClient, AssetPriceByPeriod};
use gen::database::*;
use lib::database::DbClient;

pub struct AssetPriceClient {
    db: DbClient,
}

impl AssetPriceClient {
    pub fn new(db: DbClient) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AssetInfoClient for AssetPriceClient {
    async fn get_usd_price_latest(&self, symbols: &[String]) -> Result<HashMap<String, f64>> {
        let asset_price_rows = self
            .db
            .execute(FunAssetPriceListAssetPricesReq {
                symbols: Some(symbols.to_vec()),
                limit: None,
                offset: None,
            })
            .await?
            .into_rows();

        if asset_price_rows.len() == 0 {
            bail!("no asset prices found");
        }

        let mut asset_prices: HashMap<String, f64> = HashMap::new();
        for row in asset_price_rows {
            asset_prices.insert(row.symbol, row.price_latest);
        }

        Ok(asset_prices)
    }

    async fn get_usd_price_period(
        &self,
        symbols: &[String],
    ) -> Result<HashMap<String, AssetPriceByPeriod>> {
        let asset_price_rows = self
            .db
            .execute(FunAssetPriceListAssetPricesReq {
                symbols: Some(symbols.to_vec()),
                limit: None,
                offset: None,
            })
            .await?
            .into_rows();

        if asset_price_rows.len() == 0 {
            bail!("no asset prices found");
        }

        let mut asset_prices: HashMap<String, AssetPriceByPeriod> = HashMap::new();
        for row in asset_price_rows {
            let asset_price_by_period = AssetPriceByPeriod {
                symbol: row.symbol.clone(),
                price_latest: row.price_latest,
                price_1d: Some(row.price_1d),
                price_7d: Some(row.price_7d),
                price_30d: Some(row.price_30d),
            };
            asset_prices.insert(row.symbol, asset_price_by_period);
        }

        Ok(asset_prices)
    }
}
