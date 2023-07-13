use chrono::{Duration, Utc};
use eyre::*;
use lru::LruCache;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Response, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::str::FromStr;
use tokio::sync::Mutex;
use tracing::*;
use web3::types::Address;

const API_KEY: &str = "ec6c4b09-03e6-4bd6-84f9-95406fc2ce81";
const BASE_URL: &str = "https://pro-api.coinmarketcap.com";
const LATEST_QUOTES_URL: &str = "/v2/cryptocurrency/quotes/latest";
const HISTORICAL_QUOTE_URL: &str = "/v2/cryptocurrency/quotes/historical";
const METADATA_URL: &str = "/v1/cryptocurrency/info";
const MAP_URL: &str = "/v1/cryptocurrency/map";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EnumBlockChain {
    EthereumMainnet,
    EthereumGoerli,
    BscMainnet,
    BscTestnet,
    LocalNet,
    EthereumSepolia,
}

#[derive(Debug, Clone)]
pub struct TokenAddress {
    pub address: Address,
    pub chain: EnumBlockChain,
}

#[derive(Debug, Clone)]
pub struct CoinMarketCapTokenInfo {
    pub cmc_id: u64,
    pub name: String,
    pub symbol: String,
    pub slug: String,
    pub addresses: Vec<TokenAddress>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapCoinPlatform {
    pub id: u64,
    pub name: String,
    pub symbol: String,
    pub slug: String,
    pub token_address: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MapCoinInfo {
    pub id: i32,
    pub rank: i32,
    pub name: String,
    pub symbol: String,
    pub slug: String,
    pub is_active: i32,
    pub first_historical_data: String,
    pub last_historical_data: String,
    pub platform: Option<MapCoinPlatform>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MapCoinResponse {
    pub data: Vec<MapCoinInfo>,
}

pub struct CoinMarketCap {
    client: Client,
    base_url: String,
    price_cache: Mutex<LruCache<String, f64>>,
}

impl CoinMarketCap {
    pub fn new_debug_key() -> Result<Self> {
        Self::new(API_KEY)
    }
    pub fn new(api_key: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert("X-CMC_PRO_API_KEY", HeaderValue::from_str(api_key)?);
        headers.insert("Accept", HeaderValue::from_static("application/json"));
        headers.insert("Accept-Encoding", HeaderValue::from_static("deflate, gzip"));

        Ok(Self {
            base_url: BASE_URL.to_string(),
            client: Client::builder().default_headers(headers).build()?,
            price_cache: Mutex::new(LruCache::new(NonZeroUsize::new(1000).unwrap())),
        })
    }

    pub async fn get_cmc_token_infos_by_symbol(
        &self,
        symbols: &Vec<String>,
    ) -> Result<Vec<CoinMarketCapTokenInfo>> {
        let mut url = self.metadata_url();
        self.append_url_params(&mut url, "symbol", symbols);
        self.append_url_params(&mut url, "aux", &vec!["status".to_string()]);
        let payload = &self.send_and_parse_response(&url).await?["data"];
        let mut token_infos: Vec<CoinMarketCapTokenInfo> = Vec::new();
        for symbol in symbols {
            let token = &payload[symbol][0];
            if token["status"].as_str().context("status not found")? != "active" {
                bail!("token is not active");
            }
            let mut addresses: Vec<TokenAddress> = Vec::new();
            for address_to_platform in token["contract_address"]
                .as_array()
                .context("contract addresses not found")?
            {
                let symbol: &str = address_to_platform["platform"]["coin"]["symbol"]
                    .as_str()
                    .context("symbol not found")?;
                match self.coin_symbol_to_chain(&symbol) {
                    Ok(chain) => {
                        addresses.push(TokenAddress {
                            address: Address::from_str(
                                match address_to_platform["contract_address"].as_str() {
                                    Some(address) => address,
                                    None => bail!("address not found"),
                                },
                            )?,
                            chain,
                        });
                    }
                    Err(_) => continue,
                }
            }
            token_infos.push(CoinMarketCapTokenInfo {
                cmc_id: token["id"].as_u64().context("id not found")?,
                name: token["name"]
                    .as_str()
                    .context("name not found")?
                    .to_string(),
                symbol: token["symbol"]
                    .as_str()
                    .context("symbol not found")?
                    .to_string(),
                slug: token["slug"]
                    .as_str()
                    .context("slug not found")?
                    .to_string(),
                addresses: addresses,
            })
        }

        Ok(token_infos)
    }

    pub async fn get_usd_prices_by_symbol(&self, symbols: &[String]) -> Result<Vec<f64>> {
        let mut result_index = HashMap::new();
        for (index, symbol) in symbols.iter().enumerate() {
            result_index.insert(symbol, index);
        }
        let mut token_prices: Vec<f64> = Vec::new();
        token_prices.resize(symbols.len(), 0.0);
        let mut new_symbols = Vec::new();
        let mut new_symbols_index = Vec::new();
        for symbol in symbols {
            if let Some(price) = self.price_cache.lock().await.get(symbol) {
                token_prices[result_index[symbol]] = *price;
            } else {
                new_symbols.push(symbol.clone());
                new_symbols_index.push(result_index[symbol]);
            }
        }
        if !new_symbols.is_empty() {
            let mut url = self.price_url();
            self.append_url_params(&mut url, "symbol", &new_symbols);
            let payload = &self.send_and_parse_response(&url).await?["data"];
            for (symbol, i) in new_symbols.into_iter().zip(new_symbols_index.into_iter()) {
                let token = &payload[&symbol][0];
                if token["is_active"].as_u64().context("status not found")? != 1 {
                    bail!("token status not found")
                }
                token_prices[i] = token["quote"]["USD"]["price"]
                    .as_f64()
                    .context("price not found")?;
                self.price_cache.lock().await.put(symbol, token_prices[i]);
            }
        }
        Ok(token_prices)
    }
    /**
        Quotes Historical v2
    Returns an interval of historic market quotes for any cryptocurrency based on time and interval parameters.

    Please note: This documentation relates to our updated V2 endpoint, which may be incompatible with our V1 versions. Documentation for deprecated endpoints can be found here.

    Technical Notes

    A historic quote for every "interval" period between your "time_start" and "time_end" will be returned.
    If a "time_start" is not supplied, the "interval" will be applied in reverse from "time_end".
    If "time_end" is not supplied, it defaults to the current time.
    At each "interval" period, the historic quote that is closest in time to the requested time will be returned.
    If no historic quotes are available in a given "interval" period up until the next interval period, it will be skipped.
    Implementation Tips

    Want to get the last quote of each UTC day? Don't use "interval=daily" as that returns the first quote. Instead use "interval=24h" to repeat a specific timestamp search every 24 hours and pass ex. "time_start=2019-01-04T23:59:00.000Z" to query for the last record of each UTC day.
    This endpoint supports requesting multiple cryptocurrencies in the same call. Please note the API response will be wrapped in an additional object in this case.
    Interval Options
    There are 2 types of time interval formats that may be used for "interval".

    The first are calendar year and time constants in UTC time:
    "hourly" - Get the first quote available at the beginning of each calendar hour.
    "daily" - Get the first quote available at the beginning of each calendar day.
    "weekly" - Get the first quote available at the beginning of each calendar week.
    "monthly" - Get the first quote available at the beginning of each calendar month.
    "yearly" - Get the first quote available at the beginning of each calendar year.

    The second are relative time intervals.
    "m": Get the first quote available every "m" minutes (60 second intervals). Supported minutes are: "5m", "10m", "15m", "30m", "45m".
    "h": Get the first quote available every "h" hours (3600 second intervals). Supported hour intervals are: "1h", "2h", "3h", "4h", "6h", "12h".
    "d": Get the first quote available every "d" days (86400 second intervals). Supported day intervals are: "1d", "2d", "3d", "7d", "14d", "15d", "30d", "60d", "90d", "365d".

    This endpoint is available on the following API plans:

    Basic
    Hobbyist (1 month)
    Startup (1 month)
    Standard (3 month)
    Professional (12 months)
    Enterprise (Up to 6 years)
    Cache / Update frequency: Every 5 minutes.
    Plan credit use: 1 call credit per 100 historical data points returned (rounded up) and 1 call credit per convert option beyond the first.
    CMC equivalent pages: Our historical cryptocurrency charts like coinmarketcap.com/currencies/bitcoin/#charts.


    PARAMETERS
    Query Parameters ?
     id
    string
    One or more comma-separated CoinMarketCap cryptocurrency IDs. Example: "1,2"

     symbol
    string
    Alternatively pass one or more comma-separated cryptocurrency symbols. Example: "BTC,ETH". At least one "id" or "symbol" is required for this request.

     time_start
    string
    Timestamp (Unix or ISO 8601) to start returning quotes for. Optional, if not passed, we'll return quotes calculated in reverse from "time_end".

     time_end
    string
    Timestamp (Unix or ISO 8601) to stop returning quotes for (inclusive). Optional, if not passed, we'll default to the current time. If no "time_start" is passed, we return quotes in reverse order starting from this time.

     count
    number [ 1 .. 10000 ]
    10
    The number of interval periods to return results for. Optional, required if both "time_start" and "time_end" aren't supplied. The default is 10 items. The current query limit is 10000.

     interval
    string
    "5m"
    "yearly""monthly""weekly""daily""hourly""5m""10m""15m""30m""45m""1h""2h""3h""4h""6h""12h""24h""1d""2d""3d""7d""14d""15d""30d""60d""90d""365d"
    Interval of time to return data points for. See details in endpoint description.

     convert
    string
    By default market quotes are returned in USD. Optionally calculate market quotes in up to 3 other fiat currencies or cryptocurrencies.

     convert_id
    string
    Optionally calculate market quotes by CoinMarketCap ID instead of symbol. This option is identical to convert outside of ID format. Ex: convert_id=1,2781 would replace convert=BTC,USD in your query. This parameter cannot be used when convert is used.

     aux
    string
    "price,volume,market_cap,circulating_supply,total_supply,quote_timestamp,is_active,is_fiat"
    Optionally specify a comma-separated list of supplemental data fields to return. Pass price,volume,market_cap,circulating_supply,total_supply,quote_timestamp,is_active,is_fiat,search_interval to include all auxiliary fields.

     skip_invalid
    boolean
    true
    Pass true to relax request validation rules. When requesting records on multiple cryptocurrencies an error is returned if no match is found for 1 or more requested cryptocurrencies. If set to true, invalid lookups will be skipped allowing valid cryptocurrencies to still be returned.


        */
    pub async fn get_usd_price_days_ago(&self, symbol: String, days: u32) -> Result<f64> {
        let mut url = self.quotes_historical_url();
        let today = Utc::now();
        let ago = today - Duration::days(days as i64);

        self.append_url_params(&mut url, "symbol", &[symbol.clone()]);
        self.append_url_params(&mut url, "time_start", &[ago.to_rfc3339()]);
        self.append_url_params(&mut url, "interval", &["daily".to_string()]);
        self.append_url_params(&mut url, "count", &["1".to_string()]);
        let payload = self.send_and_parse_response(&url).await?;
        let payload = &payload["data"];
        let base = &payload[symbol][0];
        let quote = &base["quotes"][0]["quote"]["USD"];
        let price = quote["price"].as_f64().context("price not found")?;
        Ok(price)
    }
    pub async fn get_quote_price_by_symbol(
        &self,
        base_symbol: String,
        quote_symbol: String,
    ) -> Result<f64> {
        let mut url = self.price_url();
        self.append_url_params(&mut url, "symbol", &[base_symbol.clone()]);
        self.append_url_params(&mut url, "convert", &[quote_symbol.clone()]);
        let payload = &self
            .parse_response(self.client.get(url).send().await?)
            .await?["data"];
        let base = &payload[base_symbol][0];
        let quote = &base["quote"][quote_symbol];
        let price = quote["price"].as_f64().context("price not found")?;
        Ok(price)
    }

    pub async fn get_top_25_coins(&self) -> Result<MapCoinResponse> {
        let mut url = self.map_url();
        self.append_url_params(&mut url, "limit", &vec!["25".to_string()]);
        self.append_url_params(&mut url, "sort", &vec!["cmc_rank".to_string()]);
        let result = self.client.get(url).send().await?;
        let msg = result.text().await?;
        let data: MapCoinResponse = serde_json::from_str(&msg)?;
        Ok(data)
    }
    fn price_url(&self) -> Url {
        Url::parse(&format!("{}{}", self.base_url, LATEST_QUOTES_URL)).unwrap()
    }
    fn quotes_historical_url(&self) -> Url {
        Url::parse(&format!("{}{}", self.base_url, HISTORICAL_QUOTE_URL)).unwrap()
    }
    fn metadata_url(&self) -> Url {
        Url::parse(&format!("{}{}", self.base_url, METADATA_URL)).unwrap()
    }
    fn map_url(&self) -> Url {
        Url::parse(&format!("{}{}", self.base_url, MAP_URL)).unwrap()
    }

    fn append_url_params(&self, url: &mut Url, param_key: &str, param_values: &[String]) -> () {
        let mut params = url.query_pairs_mut();
        params.append_pair(param_key, &param_values.join(","));
    }
    pub async fn send_and_parse_response(&self, url: &Url) -> Result<Value> {
        debug!("Request: {}", url);
        let response = self.client.get(url.clone()).send().await?;
        self.parse_response(response).await
    }

    pub async fn parse_response(&self, response: Response) -> Result<Value> {
        let text = response.text().await?;
        debug!("Response: {}", text);

        let json = Value::from_str(&text)?;
        if let Some(err) = json["status"].get("error_message") {
            if !err.is_null() {
                bail!("error_message: {}", err);
            }
        }
        Ok(json)
    }

    fn coin_symbol_to_chain(&self, coin_symbol: &str) -> Result<EnumBlockChain> {
        match coin_symbol {
            "ETH" => Ok(EnumBlockChain::EthereumMainnet),
            "BNB" => Ok(EnumBlockChain::BscMainnet),
            _ => bail!("chain not supported"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lib::log::{setup_logs, LogLevel};
    use tracing::info;

    #[tokio::test]
    async fn test_get_usd_price_by_symbol() -> Result<()> {
        let cmc = CoinMarketCap::new_debug_key().unwrap();
        let prices = cmc
            .get_usd_prices_by_symbol(&vec!["ETH".to_string()])
            .await?;
        assert_eq!(prices.len(), 1);
        assert!(prices[0] > 0.0);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_cmc_id_by_symbol() -> Result<()> {
        let cmc = CoinMarketCap::new_debug_key().unwrap();
        let infos = cmc
            .get_cmc_token_infos_by_symbol(&vec!["ETH".to_string()])
            .await?;
        assert_eq!(infos.len(), 1);
        assert_eq!(infos[0].cmc_id, 1027);
        assert_eq!(infos[0].name, "Ethereum");
        assert_eq!(infos[0].symbol, "ETH");
        assert_eq!(infos[0].slug, "ethereum");
        Ok(())
    }
    #[tokio::test]
    async fn test_get_cmc_top_25_tokens() -> Result<()> {
        setup_logs(LogLevel::Debug)?;
        let cmc = CoinMarketCap::new_debug_key().unwrap();
        let infos = cmc.get_top_25_coins().await?;
        info!("{:?}", infos);
        Ok(())
    }
    #[tokio::test]
    async fn test_get_quote_price_by_symbol() -> Result<()> {
        let cmc = CoinMarketCap::new_debug_key().unwrap();
        let price = cmc
            .get_quote_price_by_symbol("ETH".to_string(), "USDC".to_string())
            .await?;
        println!("PRICE: {:?}", price);
        assert!(price > 0.0);
        Ok(())
    }
    #[tokio::test]
    async fn test_get_price_in_usd_30_days_ago() -> Result<()> {
        setup_logs(LogLevel::Info)?;
        let cmc = CoinMarketCap::new_debug_key().unwrap();
        let price = cmc.get_usd_price_days_ago("ETH".to_string(), 30).await?;
        println!("PRICE: {:?}", price);
        assert!(price > 0.0);
        Ok(())
    }
}
