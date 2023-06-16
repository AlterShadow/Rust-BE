use std::str::FromStr;

use eyre::*;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Response, Url};
use serde_json::Value;
use web3::types::Address;

const API_KEY: &str = "ec6c4b09-03e6-4bd6-84f9-95406fc2ce81";
const BASE_URL: &str = "https://pro-api.coinmarketcap.com";

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

pub struct CoinMarketCap {
    client: Client,
    base_url: String,
    price_path: String,
    metadata_path: String,
}

impl CoinMarketCap {
    pub fn new() -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert("X-CMC_PRO_API_KEY", HeaderValue::from_static(API_KEY));
        headers.insert("Accept", HeaderValue::from_static("application/json"));
        headers.insert("Accept-Encoding", HeaderValue::from_static("deflate, gzip"));

        Ok(Self {
            base_url: BASE_URL.to_string(),
            client: Client::builder().default_headers(headers).build()?,
            price_path: "/v2/cryptocurrency/quotes/latest".to_string(),
            metadata_path: "/v2/cryptocurrency/info".to_string(),
        })
    }

    pub async fn get_cmc_token_infos_by_symbol(
        &self,
        symbols: &Vec<String>,
    ) -> Result<Vec<CoinMarketCapTokenInfo>> {
        let mut url = self.metadata_url()?;
        self.append_url_params(&mut url, "symbol", symbols);
        self.append_url_params(&mut url, "aux", &vec!["status".to_string()]);
        let payload = &self
            .parse_response(self.client.get(url).send().await?)
            .await?["data"];
        let mut token_infos: Vec<CoinMarketCapTokenInfo> = Vec::new();
        for symbol in symbols {
            let token = &payload[symbol][0];
            if token["status"]
                .as_str()
                .ok_or_else(|| eyre!("status not found"))?
                != "active"
            {
                bail!("token is not active");
            }
            let mut addresses: Vec<TokenAddress> = Vec::new();
            for address_to_platform in token["contract_address"]
                .as_array()
                .ok_or_else(|| eyre!("contract addresses not found"))?
            {
                let symbol: &str = address_to_platform["platform"]["coin"]["symbol"]
                    .as_str()
                    .ok_or_else(|| eyre!("symbol not found"))?;
                match self.coin_symbol_to_chain(&symbol) {
                    Ok(chain) => {
                        addresses.push(TokenAddress {
                            address: Address::from_str(
                                match address_to_platform["contract_address"].as_str() {
                                    Some(address) => address,
                                    None => bail!("address not found"),
                                },
                            )?,
                            chain: chain,
                        });
                    }
                    Err(_) => continue,
                }
            }
            token_infos.push(CoinMarketCapTokenInfo {
                cmc_id: token["id"].as_u64().ok_or_else(|| eyre!("id not found"))?,
                name: token["name"]
                    .as_str()
                    .ok_or_else(|| eyre!("name not found"))?
                    .to_string(),
                symbol: token["symbol"]
                    .as_str()
                    .ok_or_else(|| eyre!("symbol not found"))?
                    .to_string(),
                slug: token["slug"]
                    .as_str()
                    .ok_or_else(|| eyre!("slug not found"))?
                    .to_string(),
                addresses: addresses,
            })
        }

        Ok(token_infos)
    }

    pub async fn get_usd_prices_by_symbol(&self, symbols: &Vec<String>) -> Result<Vec<f64>> {
        let mut url = self.price_url()?;
        self.append_url_params(&mut url, "symbol", symbols);
        let payload = &self
            .parse_response(self.client.get(url).send().await?)
            .await?["data"];
        let mut token_prices: Vec<f64> = Vec::new();
        for symbol in symbols {
            let token = &payload[symbol][0];
            if token["is_active"]
                .as_u64()
                .ok_or_else(|| eyre!("status not found"))?
                != 1
            {
                bail!("token status not found")
            }
            token_prices.push(
                token["quote"]["USD"]["price"]
                    .as_f64()
                    .ok_or_else(|| eyre!("price not found"))?,
            );
        }
        Ok(token_prices)
    }

    fn price_url(&self) -> Result<Url> {
        Ok(Url::parse(&format!(
            "{}{}",
            self.base_url, self.price_path
        ))?)
    }

    fn metadata_url(&self) -> Result<Url> {
        Ok(Url::parse(&format!(
            "{}{}",
            self.base_url, self.metadata_path
        ))?)
    }

    fn append_url_params(&self, url: &mut Url, param_key: &str, param_values: &Vec<String>) -> () {
        let mut params = url.query_pairs_mut();
        params.append_pair(param_key, &param_values.join(","));
    }

    pub async fn parse_response(&self, response: Response) -> Result<Value> {
        Ok(Value::from_str(&response.text().await?)?)
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

    #[tokio::test]
    async fn test_get_usd_price_by_symbol() -> Result<()> {
        let cmc = CoinMarketCap::new().unwrap();
        let prices = cmc
            .get_usd_prices_by_symbol(&vec!["ETH".to_string()])
            .await?;
        assert_eq!(prices.len(), 1);
        assert!(prices[0] > 0.0);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_cmc_id_by_symbol() -> Result<()> {
        let cmc = CoinMarketCap::new().unwrap();
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
}
