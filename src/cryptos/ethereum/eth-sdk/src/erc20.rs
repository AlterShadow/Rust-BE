use crate::signer::EthereumSigner;
use crate::utils::{eth_public_exponent_to_address, wait_for_confirmations_simple, wei_to_eth};
use crate::{EitherTransport, EthereumNet};
use crypto::Signer;
use eyre::*;
use std::any::Any;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use token::CryptoToken;
use web3::api::Web3;
use web3::contract::{Contract, Options};
use web3::signing::Key;
use web3::types::{Address, H256, U256};

pub const ERC20_ABI: &'static str = include_str!("erc20.abi.json");

pub struct Erc20Token {
    client: Web3<EitherTransport>,
    net: EthereumNet,
    pub address: Address,
    pub contract: Contract<EitherTransport>,
}

impl Erc20Token {
    pub fn new(client: Web3<EitherTransport>, contract: Contract<EitherTransport>) -> Result<Self> {
        Ok(Self {
            client,
            net: EthereumNet::Mainnet,
            address: contract.address(),
            contract,
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
            .field("net", &self.net)
            .field("address", &self.address)
            .finish()
    }
}

#[async_trait::async_trait]
impl CryptoToken for Erc20Token {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_network_type(&self) -> String {
        match self.net {
            EthereumNet::Mainnet => "ERC20@mainnet",
            EthereumNet::Ropsten => "ERC20@ropsten",
            EthereumNet::Rinkeby => "ERC20@rinkeby",
            EthereumNet::Goerli => "ERC20@goerli",
            EthereumNet::Kovan => "ERC20@kovan",
            EthereumNet::Local => "ERC20@local",
        }
        .to_string()
    }
    fn convert_display_unit_to_internal_unit(&self, amount: &str) -> Result<String> {
        let amount = amount.parse::<f64>()?;
        let amount = (amount * 1e18).to_string();
        Ok(amount)
    }
    fn convert_internal_unit_to_display_unit(&self, amount: &str) -> Result<String> {
        let amount = U256::from_str_radix(amount, 10)?;
        Ok(wei_to_eth(amount).to_string())
    }
    fn public_exponent_to_address(
        &self,
        public_exponent: &crypto::PublicExponent,
    ) -> Result<String> {
        eth_public_exponent_to_address(public_exponent).map(|x| format!("{:?}", x))
    }

    fn address_to_public_exponent(&self, _address: &str) -> Result<crypto::PublicExponent> {
        bail!("not available for ethereum")
    }

    fn get_address_explorer_url(&self, address: &str) -> String {
        match self.net {
            EthereumNet::Mainnet => format!("https://etherscan.io/address/{}", address),
            EthereumNet::Ropsten => format!("https://ropsten.etherscan.io/address/{}", address),
            EthereumNet::Rinkeby => format!("https://rinkeby.etherscan.io/address/{}", address),
            EthereumNet::Goerli => format!("https://goerli.etherscan.io/address/{}", address),
            EthereumNet::Kovan => format!("https://kovan.etherscan.io/address/{}", address),
            EthereumNet::Local => format!("http://localhost:3000/address/{}", address),
        }
    }

    fn get_transaction_explorer_url(&self, address: &str) -> String {
        match self.net {
            EthereumNet::Mainnet => format!("https://etherscan.io/tx/{}", address),
            EthereumNet::Ropsten => format!("https://ropsten.etherscan.io/tx/{}", address),
            EthereumNet::Rinkeby => format!("https://rinkeby.etherscan.io/tx/{}", address),
            EthereumNet::Goerli => format!("https://goerli.etherscan.io/tx/{}", address),
            EthereumNet::Kovan => format!("https://kovan.etherscan.io/tx/{}", address),
            EthereumNet::Local => format!("http://localhost:3000/tx/{}", address),
        }
    }
    async fn get_balance(&self, addr: &str) -> Result<String> {
        let addr = Address::from_str(addr)?;
        let balance: U256 = self
            .contract
            .query("balanceOf", addr, None, Options::default(), None)
            .await?;
        Ok(balance.to_string())
    }
    async fn request_airdrop(&self, _addr: &str, _amount: &str) -> Result<String> {
        bail!("not available for ethereum")
    }
    async fn transfer(
        &self,
        _fee_payer: Arc<dyn Signer>,
        by: Arc<dyn Signer>,
        from: &str,
        to: &str,
        amount: &str,
    ) -> Result<String> {
        let amount = U256::from_str_radix(amount, 10)?;
        let by = EthereumSigner::new(by)?;
        if by.address == Address::from_str(from)? {
            bail!("from address is not match")
        }
        let to = Address::from_str(to)?;
        let nonce = self
            .client
            .eth()
            .transaction_count(by.address, None)
            .await?;
        let gas_price = self.client.eth().gas_price().await?;
        let gas_limit = 21000;
        let options = Options {
            nonce: Some(nonce),
            gas_price: Some(gas_price),
            gas: Some(gas_limit.into()),
            value: amount.into(),
            ..Default::default()
        };
        let tx_hash = self
            .contract
            .signed_call("transfer", (to, amount), options, by)
            .await?;

        Ok(format!("{:?}", tx_hash))
    }
    async fn confirm_transaction(&self, hash: &str) -> Result<()> {
        if hash.is_empty() {
            return Ok(());
        }
        let hash = H256::from_str(hash)?;
        let eth = self.client.eth();

        wait_for_confirmations_simple(&eth, hash, Duration::from_secs_f64(3.0), 10).await?;
        Ok(())
    }
    async fn create_account(
        &self,
        _fee_payer: Arc<dyn Signer>,
        _owner: &str,
        _account: Arc<dyn Signer>,
    ) -> Result<String> {
        Ok("".to_owned())
    }
    async fn get_latest_blockhash(&self) -> Result<String> {
        let block = self.client.eth().block_number().await?;
        Ok(block.to_string())
    }
    async fn mint_to(
        &self,
        fee_payer: Arc<dyn Signer>,
        minter: Arc<dyn Signer>,
        account: &str,
        amount: &str,
    ) -> Result<String> {
        let amount = U256::from_str_radix(amount, 10)?;
        let fee_payer = EthereumSigner::new(fee_payer)?;
        let minter = EthereumSigner::new(minter)?;
        if fee_payer.address == minter.address {
            bail!("minter address is not match")
        }
        let account = Address::from_str(account)?;
        let nonce = self
            .client
            .eth()
            .transaction_count(fee_payer.address, None)
            .await?;
        let gas_price = self.client.eth().gas_price().await?;
        let gas_limit = 21000;
        let options = Options {
            nonce: Some(nonce),
            gas_price: Some(gas_price),
            gas: Some(gas_limit.into()),
            ..Default::default()
        };
        let tx_hash = self
            .contract
            .signed_call("mintTo", (account, amount), options, minter)
            .await?;

        Ok(format!("{:?}", tx_hash))
    }
}

pub fn build_erc_20() -> Result<web3::ethabi::Contract> {
    Ok(web3::ethabi::Contract::load(ERC20_ABI.as_bytes())
        .context("failed to parse contract ABI")?)
}
