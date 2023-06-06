use crate::contract::AbstractContract;
use crate::{deploy_contract, EitherTransport, EthereumRpcConnectionPool, MultiChainAddressTable};
use eyre::*;
use gen::model::EnumBlockChain;
use tracing::info;
use web3::api::Eth;
use web3::contract::{Contract, Options};
use web3::signing::Key;
use web3::types::{Address, H256, U256};
use web3::{ethabi, Transport, Web3};

const ESCROW_ABI_JSON: &str = include_str!("escrow.json");

pub struct AbstractEscrowContract(AbstractContract<()>);
impl AbstractEscrowContract {
    pub fn new(table: MultiChainAddressTable<()>) -> Self {
        let abi = ethabi::Contract::load(ESCROW_ABI_JSON.as_bytes()).unwrap();

        Self(AbstractContract {
            name: "Escrow".to_string(),
            abi,
            contract_addresses: table,
        })
    }

    pub async fn get(
        &self,
        pool: &EthereumRpcConnectionPool,
        blockchain: EnumBlockChain,
    ) -> Result<EscrowContract<EitherTransport>> {
        let contract = self.0.get(pool, blockchain, ()).await?;
        Ok(EscrowContract { contract })
    }
}

#[derive(Debug, Clone)]
pub struct EscrowContract<T: Transport> {
    contract: Contract<T>,
}

impl<T: Transport> EscrowContract<T> {
    // only for testing
    pub async fn deploy(w3: Web3<T>, key: impl Key) -> Result<Self> {
        let address = key.address();
        let contract = deploy_contract(w3, key, address, "Escrow").await?;
        Ok(Self { contract })
    }
    pub fn new(eth: Eth<T>, address: Address) -> Result<Self> {
        let contract = Contract::from_json(eth, address, ESCROW_ABI_JSON.as_bytes())?;
        Ok(Self { contract })
    }

    pub async fn transfer_token_to(
        &self,
        signer: impl Key,
        token_address: Address,
        recipient: Address,
        amount: U256,
    ) -> Result<H256> {
        info!(
            "Transferring {:?} amount of token {:?} from escrow contract {:?} to {:?} by {:?}",
            amount,
            token_address,
            self.address(),
            recipient,
            signer.address(),
        );
        let params = (token_address, recipient, amount);
        let estimated_gas = self
            .contract
            .estimate_gas(
                EscrowFunctions::TransferTokenTo.as_str(),
                params,
                signer.address(),
                Options::default(),
            )
            .await?;

        Ok(self
            .contract
            .signed_call(
                EscrowFunctions::TransferTokenTo.as_str(),
                params,
                Options::with(|options| options.gas = Some(estimated_gas)),
                signer,
            )
            .await?)
    }

    pub async fn transfer_ownership(
        &self,
        signer: impl Key,
        by: Address,
        new_owner: Address,
    ) -> Result<H256> {
        info!(
            "Transferring escrow contract {:?} ownership from {:?} to {:?} by {:?}",
            self.address(),
            self.owner().await?,
            new_owner,
            signer.address(),
        );

        let estimated_gas = self
            .contract
            .estimate_gas(
                EscrowFunctions::TransferOwnership.as_str(),
                new_owner,
                by,
                Options::default(),
            )
            .await?;

        Ok(self
            .contract
            .signed_call(
                EscrowFunctions::TransferOwnership.as_str(),
                new_owner,
                Options::with(|options| options.gas = Some(estimated_gas)),
                signer,
            )
            .await?)
    }

    pub async fn owner(&self) -> Result<Address> {
        Ok(self
            .contract
            .query(
                EscrowFunctions::Owner.as_str(),
                (),
                None,
                Options::default(),
                None,
            )
            .await?)
    }
    pub fn address(&self) -> Address {
        self.contract.address()
    }
}

enum EscrowFunctions {
    TransferTokenTo,
    TransferOwnership,
    Owner,
}

impl EscrowFunctions {
    fn as_str(&self) -> &'static str {
        match self {
            Self::TransferTokenTo => "transferTokenTo",
            Self::TransferOwnership => "transferOwnership",
            Self::Owner => "owner",
        }
    }
}
