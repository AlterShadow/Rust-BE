use crate::escrow_tracker::StableCoinAddresses;
use crate::evm::StableCoin;
use eth_sdk::{ContractCall, TransactionReady};
use eyre::*;
use gen::model::EnumBlockChain;

use tracing::info;
use web3::ethabi::Contract;
use web3::types::{H160, U256};

#[derive(Clone, Debug)]
pub struct EscrowTransfer {
    pub token: StableCoin,
    pub amount: U256,
    pub recipient: H160,
    pub owner: H160,
}

fn get_method_by_name(name: &str) -> Option<Erc20Method> {
    match name {
        "transfer" => Some(Erc20Method::Transfer),
        "transferFrom" => Some(Erc20Method::TransferFrom),
        _ => None,
    }
}

pub enum Erc20Method {
    Transfer,
    TransferFrom,
}

pub fn parse_escrow(
    chain: EnumBlockChain,
    tx: &TransactionReady,
    stablecoin_addresses: &StableCoinAddresses,
    erc_20: &Contract,
) -> Result<EscrowTransfer> {
    let called_contract = tx.get_to().context("missing called contract")?;
    let eth_mainnet_stablecoins = stablecoin_addresses.get(chain).unwrap();
    let token: StableCoin = eth_mainnet_stablecoins
        .iter()
        .find(|(_, address)| *address == called_contract)
        .map(|x| x.0)
        .context("Unsupported coin")?;

    let sender = tx.get_from().context("No sender")?;

    let input_data = tx.get_input_data();

    let call = ContractCall::from_inputs(erc_20, &input_data)?;

    let method = get_method_by_name(&call.get_name()).context("call is not an escrow")?;
    let escrow: EscrowTransfer = match method {
        Erc20Method::Transfer => {
            let recipient = call
                .get_param("_to")
                .or_else(|_| call.get_param("to"))
                .or_else(|_| Err(eyre!("no recipient address")))?
                .get_value()
                .into_address()?;

            let amount = call
                .get_param("_value")
                .or_else(|_| call.get_param("value"))
                .or_else(|_| call.get_param("_amount"))
                .or_else(|_| call.get_param("amount"))
                .or_else(|_| Err(eyre!("no amount")))?
                .get_value()
                .into_uint()?;

            EscrowTransfer {
                token,
                amount,
                recipient,
                owner: sender,
            }
        }
        Erc20Method::TransferFrom => {
            let owner = call
                .get_param("_from")
                .or_else(|_| call.get_param("from"))
                .or_else(|_| Err(eyre!("no owner address")))?
                .get_value()
                .into_address()?;

            let recipient = call
                .get_param("_to")
                .or_else(|_| call.get_param("to"))
                .or_else(|_| Err(eyre!("no recipient address")))?
                .get_value()
                .into_address()?;

            let amount = call
                .get_param("_value")
                .or_else(|_| call.get_param("value"))
                .or_else(|_| call.get_param("_amount"))
                .or_else(|_| call.get_param("amount"))
                .or_else(|_| Err(eyre!("no amount")))?
                .get_value()
                .into_uint()?;

            EscrowTransfer {
                token,
                amount,
                recipient,
                owner,
            }
        }
    };

    info!("parsed escrow: {:?} {:?}", tx.get_hash(), escrow);
    Ok(escrow)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::escrow_tracker::escrow::{parse_escrow, StableCoinAddresses};
    use eth_sdk::erc20::build_erc_20;
    use eth_sdk::{EthereumRpcConnectionPool, Transaction};
    use gen::model::EnumBlockChain;
    use lib::log::{setup_logs, LogLevel};
    use tracing::info;

    #[tokio::test]
    pub async fn test_usdt_transfer() -> Result<()> {
        let _ = setup_logs(LogLevel::Trace);

        let conn_pool = EthereumRpcConnectionPool::mainnet();
        let conn = conn_pool.get_conn().await?;
        let tx = Transaction::new_and_assume_ready(
            "0x977939d69a0826a6ef1e94ccfe76a2c2d87bac1d3fce53669b5c637435fd23c1".parse()?,
            &conn,
        )
        .await?;
        let erc20 = build_erc_20()?;
        let trade = parse_escrow(
            EnumBlockChain::EthereumMainnet,
            &tx,
            &StableCoinAddresses::default(),
            &erc20,
        )?;
        info!("trade: {:?}", trade);
        Ok(())
    }
    #[tokio::test]
    pub async fn test_usdc_transfer() -> Result<()> {
        let _ = setup_logs(LogLevel::Trace);
        let conn_pool = EthereumRpcConnectionPool::mainnet();
        let conn = conn_pool.get_conn().await?;
        let tx = Transaction::new_and_assume_ready(
            "0x1f716239290641ad0121814df498e5e04c3759bf6d22c9c89a6aa5175a3ce4c6".parse()?,
            &conn,
        )
        .await?;

        let erc20 = build_erc_20()?;
        let trade = parse_escrow(
            EnumBlockChain::EthereumMainnet,
            &tx,
            &StableCoinAddresses::default(),
            &erc20,
        )?;
        info!("trade: {:?}", trade);
        Ok(())
    }
    #[tokio::test]
    pub async fn test_busd_transfer() -> Result<()> {
        let _ = setup_logs(LogLevel::Trace);
        let conn_pool = EthereumRpcConnectionPool::mainnet();
        let conn = conn_pool.get_conn().await?;
        let tx = Transaction::new_and_assume_ready(
            "0x27e801a5735e5b530535165a18754c074c673263470fc1fad32cca5eb1bc9fea".parse()?,
            &conn,
        )
        .await?;

        let erc20 = build_erc_20()?;
        let trade = parse_escrow(
            EnumBlockChain::EthereumMainnet,
            &tx,
            &StableCoinAddresses::default(),
            &erc20,
        )?;
        info!("trade: {:?}", trade);
        Ok(())
    }
}
