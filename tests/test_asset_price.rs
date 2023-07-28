use std::str::FromStr;

use eyre::*;
use web3::types::Address;

use api::cmc::CoinMarketCap;
use api::AssetInfoClient;
use gen::database::FunAdminAddEscrowTokenContractAddressReq;
use gen::model::*;
use lib::database::{connect_to_database, database_test_config, drop_and_recreate_database};
use mc2fi_asset_price::update_price::{
    delete_old_price_entries, fill_asset_price_cache, fill_past_prices_on_startup,
};
use mc2fi_asset_price::AssetPriceClient;

#[tokio::test]
async fn test_this_db_func() -> Result<()> {
    drop_and_recreate_database()?;
    let db = connect_to_database(database_test_config()).await?;
    let cmc = CoinMarketCap::new("cd270acf-14f5-4fa2-8787-4fdafc8570f0")?;

    db.execute(FunAdminAddEscrowTokenContractAddressReq {
        pkey_id: Some(i64::from(1)),
        symbol: "WBTC".to_string(),
        short_name: "WBTC".to_string(),
        description: "WBTC".to_string(),
        address: Address::from_str("0x2260fac5e5542a773aa44fbcfedf7c193bc2c599")?.into(),
        blockchain: EnumBlockChain::EthereumMainnet,
        decimals: 18,
        is_stablecoin: false,
        is_wrapped: true,
    })
    .await?;

    db.execute(FunAdminAddEscrowTokenContractAddressReq {
        pkey_id: Some(i64::from(2)),
        symbol: "WETH".to_string(),
        short_name: "WETH".to_string(),
        description: "WETH".to_string(),
        address: Address::from_str("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2")?.into(),
        blockchain: EnumBlockChain::EthereumMainnet,
        decimals: 18,
        is_stablecoin: false,
        is_wrapped: true,
    })
    .await?;

    fill_past_prices_on_startup(&db, &cmc).await?;
    fill_asset_price_cache(&db, &cmc).await?;
    delete_old_price_entries(&db).await?;

    let asset_client = AssetPriceClient::new(db.clone());
    let latest_prices = asset_client
        .get_usd_price_latest(&["WBTC".to_string(), "WETH".to_string()])
        .await?;

    println!("latest prices: {:?}", latest_prices);

    let prices_by_period = asset_client
        .get_usd_price_period(&["WBTC".to_string(), "WETH".to_string()])
        .await?;

    println!("prices by period: {:?}", prices_by_period);

    Ok(())
}
