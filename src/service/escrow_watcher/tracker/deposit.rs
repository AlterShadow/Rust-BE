use crate::evm::TransactionReady;
use crate::tracker::escrow::{parse_escrow, Erc20, StableCoinAddresses};
use eyre::*;
use gen::database::{FunUserBackStrategyReq, FunUserGetStrategyFromWalletReq};
use gen::model::EnumBlockChain;
use lib::database::DbClient;
use lib::toolbox::RequestContext;

pub async fn on_user_deposit(
    ctx: &RequestContext,
    db: &DbClient,
    chain: EnumBlockChain,
    tx: &TransactionReady,
    stablecoin_addresses: &StableCoinAddresses,
    erc_20: &Erc20,
) -> Result<()> {
    let user_wallet_address = tx.get_from().context("missing user wallet address")?;
    let esc = parse_escrow(chain, tx, stablecoin_addresses, erc_20)?;
    // let our_valid_address = esc.recipient == "0x000".parse()?;
    let our_valid_address = true;
    ensure!(
        our_valid_address,
        "is not our valid address {:?}",
        esc.recipient
    );
    let user_registered_strategy = db
        .execute(FunUserGetStrategyFromWalletReq {
            wallet_address: format!("{:?}", user_wallet_address),
            blockchain: chain.to_string(),
        })
        .await?
        .into_result()
        .context("user_registered_strategy")?;
    db.execute(FunUserBackStrategyReq {
        user_id: ctx.user_id,
        strategy_id: user_registered_strategy.strategy_id,
        quantity: format!("{:?}", esc.amount),
        purchase_wallet: format!("{:?}", user_wallet_address),
        blockchain: chain.to_string(),
        transaction_hash: format!("{:?}", tx.get_hash()),
    })
    .await?;
    // TODO: call strategy contract to save above user back strategy on chain
    // TODO: invoke escrow wallet transfer to actually move asset to strategy

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evm::{EthereumRpcConnectionPool, Transaction};
    use crate::tracker::escrow::build_erc_20;
    use itertools::Itertools;
    use lib::database::{connect_to_database, DatabaseConfig};
    use lib::log::{setup_logs, LogLevel};
    use tracing::info;
    #[tokio::test]
    async fn test_on_user_deposit() -> Result<()> {
        let _ = setup_logs(LogLevel::Trace);

        let conn_pool = EthereumRpcConnectionPool::mainnet();
        let conn = conn_pool.get_conn().await?;
        let tx = Transaction::new_and_assume_ready(
            "0x27e801a5735e5b530535165a18754c074c673263470fc1fad32cca5eb1bc9fea".parse()?,
            &conn,
        )
        .await?;
        let erc20 = build_erc_20()?;
        let ctx = RequestContext {
            connection_id: 0,
            user_id: 0,
            seq: 0,
            method: 0,
            log_id: 0,
        };
        let db = connect_to_database(DatabaseConfig {
            user: Some("postgres".to_string()),
            password: Some("123456".to_string()),
            ..Default::default()
        })
        .await?;

        on_user_deposit(
            &ctx,
            &db,
            EnumBlockChain::EthereumMainnet,
            &tx,
            &StableCoinAddresses::new(),
            &erc20,
        )
        .await?;
        let trade = parse_escrow(
            EnumBlockChain::EthereumMainnet,
            &tx,
            &StableCoinAddresses::new(),
            &erc20,
        )?;
        info!("trade: {:?}", trade);
        Ok(())
    }
}
