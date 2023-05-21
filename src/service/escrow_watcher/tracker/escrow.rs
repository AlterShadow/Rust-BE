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

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
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

pub async fn parse_escrow(
    chain: EnumBlockChain,
    tx: &Transaction,
    called_contract: &H160,
    stablecoin_addresses: &StableCoinAddresses,
    erc_20: &Erc20,
) -> Result<()> {
    let eth_mainnet_stablecoins = stablecoin_addresses.get(&chain).unwrap();
    let mut coin: Option<StableCoin> = None;
    for (stablecoin, address) in eth_mainnet_stablecoins {
        if *address == *called_contract {
            coin = Some(match stablecoin {
                StableCoin::Usdc => StableCoin::Usdc,
                StableCoin::Usdt => {
                    bail!("does not support stable coin: USDT");
                }
                StableCoin::Busd => {
                    bail!("does not support stable coin: BUSD");
                }
            });
            break;
        }
    }

    let sender = match tx.get_from() {
        Some(sender) => sender,
        None => {
            return Err(eyre!("no sender"));
        }
    };

    let input_data = match tx.get_input_data() {
        Some(input_data) => input_data,
        None => {
            return Err(eyre!("no input data"));
        }
    };

    let call = ContractCall::from_inputs(&erc_20.inner, &input_data)?;
    let escrow: Escrow;
    if let Some(method) = get_method_by_name(&call.get_name()) {
        escrow = match method {
            Erc20Method::Transfer => {
                let recipient = match call.get_param("_to") {
                    Some(param) => match param.get_value() {
                        Token::Address(value) => convert_h160_ethabi_to_web3(value),
                        _ => {
                            return Err(eyre!("to is not an address"));
                        }
                    },
                    None => {
                        return Err(eyre!("no to"));
                    }
                };

                let value = match call.get_param("_value") {
                    Some(param) => match param.get_value() {
                        Token::Uint(value) => convert_u256_ethabi_to_web3(value),
                        _ => {
                            return Err(eyre!("value is not a uint"));
                        }
                    },
                    None => {
                        return Err(eyre!("no value"));
                    }
                };
                Escrow {
                    token: match coin {
                        Some(coin) => coin,
                        None => return Err(eyre!("no coin")),
                    },
                    amount: value,
                    recipient: recipient,
                    owner: sender,
                }
            }
            Erc20Method::TransferFrom => {
                let owner = match call.get_param("_from") {
                    Some(param) => match param.get_value() {
                        Token::Address(value) => convert_h160_ethabi_to_web3(value),
                        _ => {
                            return Err(eyre!("from is not an address"));
                        }
                    },
                    None => {
                        return Err(eyre!("no from"));
                    }
                };

                let recipient = match call.get_param("_to") {
                    Some(param) => match param.get_value() {
                        Token::Address(value) => convert_h160_ethabi_to_web3(value),
                        _ => {
                            return Err(eyre!("to is not an address"));
                        }
                    },
                    None => {
                        return Err(eyre!("no to"));
                    }
                };

                let value = match call.get_param("_value") {
                    Some(param) => match param.get_value() {
                        Token::Uint(value) => convert_u256_ethabi_to_web3(value),
                        _ => {
                            return Err(eyre!("value is not a uint"));
                        }
                    },
                    None => {
                        return Err(eyre!("no value"));
                    }
                };
                Escrow {
                    token: match coin {
                        Some(coin) => coin,
                        None => return Err(eyre!("no coin")),
                    },
                    amount: value,
                    recipient: recipient,
                    owner: owner,
                }
            }
        }
    } else {
        return Err(eyre!("call is not an escrow"));
    }

    info!("tx: {:?}", tx.get_id().unwrap());
    info!("escrow: {:?}", escrow);
    Ok(())
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
