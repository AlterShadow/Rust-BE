use eyre::*;
use gen::model::EnumBlockChain;
use lib::database::{DbClient, ToSql};
use web3::types::Address;

pub async fn add_strategy_initial_token_ratio(
    db: &DbClient,
    strategy_id: i64,
    wbnb_address_on_bsc_testnet: Address,
    ts: i64,
) -> Result<()> {
    db.query(
        "
			INSERT INTO tbl.strategy_initial_token_ratio
			(fkey_strategy_id, blockchain, token_name, token_address, quantity, updated_at, created_at)
			VALUES
			($1, $2, $3, $4, $5, $6, $7);
			",
        &[
            &strategy_id as &(dyn ToSql + Sync),
            &EnumBlockChain::BscTestnet as &(dyn ToSql + Sync),
            &"WBNB".to_string() as &(dyn ToSql + Sync),
            &format!("{:?}", wbnb_address_on_bsc_testnet) as &(dyn ToSql + Sync),
            &"100000000".to_string() as &(dyn ToSql + Sync),
            &ts as &(dyn ToSql + Sync),
            &ts as &(dyn ToSql + Sync),
        ],
    )
    .await?;
    Ok(())
}
