use crate::erc20::parse_erc20_transfer_event;
use crate::{BlockchainCoinAddresses, EscrowTransfer, TransactionReady};
use eyre::*;
use gen::model::EnumBlockChain;
use tracing::info;

pub fn parse_escrow_transfer(
    chain: EnumBlockChain,
    tx: &TransactionReady,
    stablecoin_addresses: &BlockchainCoinAddresses,
) -> Result<EscrowTransfer> {
    let called_contract = tx.get_to().context("missing called contract")?;
    let token = stablecoin_addresses
        .get_by_address(chain, called_contract)
        .context("unsupported coin")?;

    match token {
        "USDC" => {}
        "USDT" => {}
        "BUSD" => {}
        _ => bail!("unsupported coin: {:?}", token),
    }

    let transfer_event =
        parse_erc20_transfer_event(called_contract, tx.get_receipt().clone(), None, None)?;

    let escrow = EscrowTransfer {
        token: token.to_string(),
        token_address: called_contract,
        amount: transfer_event.value,
        recipient: transfer_event.to,
        owner: transfer_event.from,
    };

    info!("parsed escrow: {:?} {:?}", tx.get_hash(), escrow);
    Ok(escrow)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BlockchainCoinAddresses, EthereumRpcConnectionPool, TransactionFetcher};
    use gen::model::EnumBlockChain;
    use lib::log::{setup_logs, LogLevel};
    use tracing::info;

    #[tokio::test]
    pub async fn test_usdt_transfer() -> Result<()> {
        let _ = setup_logs(LogLevel::Trace);

        let conn_pool = EthereumRpcConnectionPool::new();
        let conn = conn_pool.get(EnumBlockChain::EthereumMainnet).await?;
        let tx = TransactionFetcher::new_and_assume_ready(
            "0x977939d69a0826a6ef1e94ccfe76a2c2d87bac1d3fce53669b5c637435fd23c1".parse()?,
            &conn,
        )
        .await?;
        let escrow_transfer = parse_escrow_transfer(
            EnumBlockChain::EthereumMainnet,
            &tx,
            &BlockchainCoinAddresses::new(),
        )?;
        info!("escrow: {:?}", escrow_transfer);
        Ok(())
    }
    #[tokio::test]
    pub async fn test_usdc_transfer() -> Result<()> {
        let _ = setup_logs(LogLevel::Trace);
        let conn_pool = EthereumRpcConnectionPool::new();
        let conn = conn_pool.get(EnumBlockChain::EthereumMainnet).await?;
        let tx = TransactionFetcher::new_and_assume_ready(
            "0x1f716239290641ad0121814df498e5e04c3759bf6d22c9c89a6aa5175a3ce4c6".parse()?,
            &conn,
        )
        .await?;

        let escrow_transfer = parse_escrow_transfer(
            EnumBlockChain::EthereumMainnet,
            &tx,
            &BlockchainCoinAddresses::new(),
        )?;
        info!("escrow: {:?}", escrow_transfer);
        Ok(())
    }
    #[tokio::test]
    pub async fn test_busd_transfer() -> Result<()> {
        let _ = setup_logs(LogLevel::Trace);
        let conn_pool = EthereumRpcConnectionPool::new();
        let conn = conn_pool.get(EnumBlockChain::EthereumMainnet).await?;
        let tx = TransactionFetcher::new_and_assume_ready(
            "0x27e801a5735e5b530535165a18754c074c673263470fc1fad32cca5eb1bc9fea".parse()?,
            &conn,
        )
        .await?;

        let escrow_transfer = parse_escrow_transfer(
            EnumBlockChain::EthereumMainnet,
            &tx,
            &BlockchainCoinAddresses::new(),
        )?;
        info!("escrow: {:?}", escrow_transfer);
        Ok(())
    }
}
