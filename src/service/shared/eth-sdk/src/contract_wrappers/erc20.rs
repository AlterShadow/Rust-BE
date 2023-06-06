use crate::contract::AbstractContract;
use crate::{EitherTransport, EthereumRpcConnectionPool, MultiChainAddressTable};
use eyre::*;
use gen::model::EnumBlockChain;
use std::fmt::{Debug, Formatter};

use web3::api::Web3;
use web3::contract::{Contract, Options};
use web3::signing::Key;
use web3::types::{Address, H256, U256};

pub const ERC20_ABI: &'static str = include_str!("erc20.abi.json");

pub struct AbstractErc20Token(AbstractContract<()>);
impl AbstractErc20Token {
    pub fn new(name: String, table: MultiChainAddressTable<()>) -> Self {
        Self(AbstractContract {
            name,
            abi: build_erc_20().unwrap(),
            contract_addresses: table,
        })
    }
    pub async fn get(
        &self,
        pool: &EthereumRpcConnectionPool,
        blockchain: EnumBlockChain,
    ) -> Result<Erc20Token> {
        let contract = self.0.get(pool, blockchain, ()).await?;
        Ok(Erc20Token {
            address: contract.address(),
            contract,
        })
    }
}
pub struct Erc20Token {
    pub address: Address,
    pub contract: Contract<EitherTransport>,
}

impl Erc20Token {
    pub fn new(client: Web3<EitherTransport>, address: Address) -> Result<Self> {
        Ok(Self {
            address,
            contract: Contract::new(client.eth(), address, build_erc_20()?),
        })
    }

    pub async fn mint(&self, secret: impl Key, to: Address, amount: U256) -> Result<H256> {
        Ok(self
            .contract
            .signed_call("mint", (to, amount), Options::default(), secret)
            .await?)
    }

    pub async fn burn(&self, secret: impl Key, from: Address, amount: U256) -> Result<H256> {
        Ok(self
            .contract
            .signed_call("burn", (from, amount), Options::default(), secret)
            .await?)
    }

    pub async fn transfer(&self, secret: impl Key, to: Address, amount: U256) -> Result<H256> {
        Ok(self
            .contract
            .signed_call("transfer", (to, amount), Options::default(), secret)
            .await?)
    }

    pub async fn transfer_from(
        &self,
        secret: impl Key,
        from: Address,
        to: Address,
        amount: U256,
    ) -> Result<H256> {
        Ok(self
            .contract
            .signed_call(
                "transferFrom",
                (from, to, amount),
                Options::default(),
                secret,
            )
            .await?)
    }

    pub async fn approve(&self, secret: impl Key, spender: Address, amount: U256) -> Result<H256> {
        Ok(self
            .contract
            .signed_call("approve", (spender, amount), Options::default(), secret)
            .await?)
    }

    pub async fn balance_of(&self, owner: Address) -> Result<U256> {
        Ok(self
            .contract
            .query("balanceOf", owner, None, Options::default(), None)
            .await?)
    }

    pub async fn allowance(&self, owner: Address, spender: Address) -> Result<U256> {
        Ok(self
            .contract
            .query(
                "allowance",
                (owner, spender),
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn total_supply(&self) -> Result<U256> {
        Ok(self
            .contract
            .query("totalSupply", (), None, Options::default(), None)
            .await?)
    }
}

impl Debug for Erc20Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ERC20Token")
            .field("address", &self.address)
            .finish()
    }
}

pub fn build_erc_20() -> Result<web3::ethabi::Contract> {
    Ok(web3::ethabi::Contract::load(ERC20_ABI.as_bytes())
        .context("failed to parse contract ABI")?)
}
