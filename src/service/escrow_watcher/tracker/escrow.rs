use std::collections::HashMap;
use std::str::FromStr;

use ethabi::{Contract, Token};
use eyre::*;
use gen::model::EnumBlockChain;
use tracing::info;
use web3::types::{H160, U256};

use crate::evm::{
    convert_h160_ethabi_to_web3, convert_u256_ethabi_to_web3, ContractCall, Transaction,
};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum StableCoin {
    Usdc,
    Usdt,
    Busd,
}

pub struct StableCoinAddresses {
    inner: HashMap<EnumBlockChain, Vec<(StableCoin, H160)>>,
}

impl Default for StableCoinAddresses {
    fn default() -> Self {
        let mut this = StableCoinAddresses {
            inner: HashMap::new(),
        };

        this.inner.insert(
            EnumBlockChain::EthereumMainnet,
            vec![
                (
                    StableCoin::Usdc,
                    H160::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap(),
                ),
                (
                    StableCoin::Usdt,
                    H160::from_str("0xdac17f958d2ee523a2206206994597c13d831ec7").unwrap(),
                ),
                (
                    StableCoin::Busd,
                    H160::from_str("0x4Fabb145d64652a948d72533023f6E7A623C7C53").unwrap(),
                ),
            ],
        );
        this.inner.insert(
            EnumBlockChain::BscMainnet,
            vec![
                (
                    StableCoin::Usdc,
                    H160::from_str("0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d").unwrap(),
                ),
                (
                    StableCoin::Usdt,
                    H160::from_str("0x55d398326f99059ff775485246999027b3197955").unwrap(),
                ),
                (
                    StableCoin::Busd,
                    H160::from_str("0xe9e7cea3dedca5984780bafc599bd69add087d56").unwrap(),
                ),
            ],
        );
        this.inner.insert(
            EnumBlockChain::EthereumGoerli,
            vec![(
                StableCoin::Usdc,
                H160::from_str("0x07865c6E87B9F70255377e024ace6630C1Eaa37F").unwrap(),
            )],
        );
        this.inner.insert(
            EnumBlockChain::BscTestnet,
            vec![(
                StableCoin::Busd,
                H160::from_str("0xaB1a4d4f1D656d2450692D237fdD6C7f9146e814").unwrap(),
            )],
        );

        this
    }
}

impl StableCoinAddresses {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn get(&self, chain: &EnumBlockChain) -> Option<&Vec<(StableCoin, H160)>> {
        self.inner.get(chain)
    }
}

#[derive(Clone, Debug)]
pub struct Escrow {
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
    tx: &Transaction,
    stablecoin_addresses: &StableCoinAddresses,
    erc_20: &Erc20,
) -> Result<Escrow> {
    let called_contract = tx.get_to().context("missing called contract")?;
    let eth_mainnet_stablecoins = stablecoin_addresses.get(&chain).unwrap();
    let token: StableCoin = eth_mainnet_stablecoins
        .iter()
        .find(|(_, address)| *address == called_contract)
        .map(|x| x.0)
        .context("Unsupported coin")?;

    let sender = tx.get_from().context("No sender")?;

    let input_data = tx.get_input_data().context("No input data")?;

    let call = ContractCall::from_inputs(&erc_20.inner, &input_data)?;
    let method = get_method_by_name(&call.get_name()).context("call is not an escrow")?;
    let escrow: Escrow = match method {
        Erc20Method::Transfer => {
            let to_param = call.get_param("_to").context("no to address")?;
            let recipient = match to_param.get_value() {
                Token::Address(value) => convert_h160_ethabi_to_web3(value),
                x => {
                    bail!("to is not an address: {:?}", x);
                }
            };
            let value_param = call.get_param("_value").context("no value")?;
            let value = match value_param.get_value() {
                Token::Uint(value) => convert_u256_ethabi_to_web3(value),
                x => {
                    bail!("value is not a uint {:?}", x);
                }
            };
            Escrow {
                token,
                amount: value,
                recipient,
                owner: sender,
            }
        }
        Erc20Method::TransferFrom => {
            let from_param = call.get_param("_from").context("no from param")?;
            let owner = match from_param.get_value() {
                Token::Address(value) => convert_h160_ethabi_to_web3(value),
                x => {
                    bail!("from is not an address {:?}", x);
                }
            };

            let to_param = call.get_param("_to").context("no to address")?;
            let recipient = match to_param.get_value() {
                Token::Address(value) => convert_h160_ethabi_to_web3(value),
                x => {
                    bail!("to is not an address: {:?}", x);
                }
            };

            let value_param = call.get_param("_value").context("no value")?;
            let amount = match value_param.get_value() {
                Token::Uint(value) => convert_u256_ethabi_to_web3(value),
                x => {
                    bail!("value is not a uint {:?}", x);
                }
            };
            Escrow {
                token,
                amount,
                recipient,
                owner,
            }
        }
    };

    info!("tx: {:?}", tx.get_id().unwrap());
    info!("escrow: {:?}", escrow);
    Ok(escrow)
}

const ERC20_PATH: &str = "abi/generic/erc20.json";

pub struct Erc20 {
    pub inner: Contract,
}

impl Erc20 {
    pub fn new(erc_20: Contract) -> Self {
        Self { inner: erc_20 }
    }
}

pub fn build_erc_20() -> Result<Erc20> {
    let erc20 =
        Contract::load(std::fs::File::open(ERC20_PATH).context("failed to read contract ABI")?)
            .context("failed to parse contract ABI")?;
    Ok(Erc20::new(erc20))
}

#[cfg(test)]
mod tests {
    use crate::evm::{EthereumRpcConnectionPool, Transaction};
    use crate::tracker::escrow::{build_erc_20, parse_escrow, StableCoinAddresses};
    use eyre::*;
    use gen::model::EnumBlockChain;
    use lib::log::{setup_logs, LogLevel};
    use tracing::info;

    #[tokio::test]
    pub async fn test_usdt_transfer() -> Result<()> {
        let _ = setup_logs(LogLevel::Trace);

        let mut tx = Transaction::new(
            "0x977939d69a0826a6ef1e94ccfe76a2c2d87bac1d3fce53669b5c637435fd23c1".parse()?,
        );
        let conn_pool =
            EthereumRpcConnectionPool::new("https://ethereum.publicnode.com".to_string(), 10)
                .await?;
        let conn = conn_pool.get_conn().await?;
        tx.update(&conn).await?;
        let erc20 = build_erc_20()?;
        let trade = parse_escrow(
            EnumBlockChain::EthereumMainnet,
            &tx,
            &StableCoinAddresses::new(),
            &erc20,
        )?;
        info!("trade: {:?}", trade);
        Ok(())
    }
    #[tokio::test]
    pub async fn test_usdc_transfer() -> Result<()> {
        let _ = setup_logs(LogLevel::Trace);

        let mut tx = Transaction::new(
            "0x1f716239290641ad0121814df498e5e04c3759bf6d22c9c89a6aa5175a3ce4c6".parse()?,
        );
        let conn_pool =
            EthereumRpcConnectionPool::new("https://ethereum.publicnode.com".to_string(), 10)
                .await?;
        let conn = conn_pool.get_conn().await?;
        tx.update(&conn).await?;
        let erc20 = build_erc_20()?;
        let trade = parse_escrow(
            EnumBlockChain::EthereumMainnet,
            &tx,
            &StableCoinAddresses::new(),
            &erc20,
        )?;
        info!("trade: {:?}", trade);
        Ok(())
    }
    #[tokio::test]
    pub async fn test_busd_transfer() -> Result<()> {
        let _ = setup_logs(LogLevel::Trace);

        let mut tx = Transaction::new(
            "0x27e801a5735e5b530535165a18754c074c673263470fc1fad32cca5eb1bc9fea".parse()?,
        );
        let conn_pool =
            EthereumRpcConnectionPool::new("https://ethereum.publicnode.com".to_string(), 10)
                .await?;
        let conn = conn_pool.get_conn().await?;
        tx.update(&conn).await?;
        let erc20 = build_erc_20()?;
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
