use eyre::*;
use tracing::info;
use web3::contract::{Contract, Options};
use web3::signing::Key;
use web3::types::{Address, H256, U256};
use web3::{ethabi, Transport, Web3};

use crate::contract::AbstractContract;
use crate::logger::get_blockchain_logger;
use crate::{
    deploy_contract, EitherTransport, EthereumRpcConnection, EthereumRpcConnectionPool,
    MultiChainAddressTable,
};
use gen::model::EnumBlockChain;
use lib::log::DynLogger;
use lib::types::amount_to_display;

const STRATEGY_WALLET_ABI_JSON: &str = include_str!("strategy_wallet.json");
pub struct AbstractStrategyWalletContract(AbstractContract<()>);
impl AbstractStrategyWalletContract {
    pub fn new(name: String, table: MultiChainAddressTable<()>) -> Self {
        let abi = ethabi::Contract::load(STRATEGY_WALLET_ABI_JSON.as_bytes()).unwrap();
        Self(AbstractContract {
            name,
            abi,
            contract_addresses: table,
        })
    }

    pub async fn get(
        &self,
        pool: &EthereumRpcConnectionPool,
        blockchain: EnumBlockChain,
    ) -> Result<StrategyWalletContract<EitherTransport>> {
        let contract = self.0.get(pool, blockchain, ()).await?;
        Ok(StrategyWalletContract { contract })
    }
}

#[derive(Debug, Clone)]
pub struct StrategyWalletContract<T: Transport> {
    contract: Contract<T>,
}

impl<T: Transport> StrategyWalletContract<T> {
    pub async fn deploy(
        w3: Web3<T>,
        key: impl Key + Clone,
        backer: Address,
        admin: Address,
        logger: DynLogger,
    ) -> Result<Self> {
        let contract =
            deploy_contract(w3.clone(), key, (backer, admin), "StrategyWallet", logger).await?;

        Ok(Self { contract })
    }

    pub fn new(w3: Web3<T>, address: Address) -> Result<Self> {
        let contract = Contract::from_json(w3.eth(), address, STRATEGY_WALLET_ABI_JSON.as_bytes())?;
        Ok(Self { contract })
    }

    pub fn address(&self) -> Address {
        self.contract.address()
    }

    pub async fn backer(&self) -> Result<Address> {
        Ok(self
            .contract
            .query(
                StrategyWalletFunctions::Backer.as_str(),
                (),
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn admin(&self) -> Result<Address> {
        Ok(self
            .contract
            .query(
                StrategyWalletFunctions::Admin.as_str(),
                (),
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn redeem_from_strategy(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key + Clone,
        strategy: Address,
        shares: U256,
    ) -> Result<H256> {
        info!(
					"Redeeming {:?} shares from strategy pool contract {:?} using strategy wallet contract {:?} by {:?}",
					amount_to_display(shares),
					strategy,
					self.address(),
					signer.address(),
				);

        let estimated_gas = self
            .contract
            .estimate_gas(
                StrategyWalletFunctions::RedeemFromStrategy.as_str(),
                (strategy, shares),
                signer.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        let tx_hash = self
            .contract
            .signed_call(
                StrategyWalletFunctions::RedeemFromStrategy.as_str(),
                (strategy, shares),
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer.clone(),
            )
            .await?;
        get_blockchain_logger().log(format!(
            "Redeeming {:?} shares from strategy pool contract {:?} using strategy wallet contract {:?} by {:?}",
            amount_to_display(shares),
            strategy,
            self.address(),
            signer.address(),
        ), tx_hash)?;
        Ok(tx_hash)
    }

    pub async fn full_redeem_from_strategy(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key + Clone,
        strategy: Address,
    ) -> Result<H256> {
        info!(
					"Redeeming all shares from strategy pool contract {:?} using strategy wallet contract {:?} by {:?}",
					strategy,
					self.address(),
					signer.address(),
				);

        let estimated_gas = self
            .contract
            .estimate_gas(
                StrategyWalletFunctions::FullRedeemFromStrategy.as_str(),
                strategy,
                signer.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        let tx_hash = self
            .contract
            .signed_call(
                StrategyWalletFunctions::FullRedeemFromStrategy.as_str(),
                strategy,
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer.clone(),
            )
            .await?;
        get_blockchain_logger().log(
            format!(
            "Redeeming all shares from strategy pool contract {:?} using strategy wallet contract {:?} by {:?}",
            strategy,
            self.address(),
            signer.address(),
        ),
            tx_hash,
        )?;
        Ok(tx_hash)
    }

    pub async fn transfer_adminship(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key,
        new_admin: Address,
    ) -> Result<H256> {
        info!(
            "Transferring adminship of strategy wallet contract {:?} to {:?} by {:?}",
            self.address(),
            new_admin,
            signer.address(),
        );

        let estimated_gas = self
            .contract
            .estimate_gas(
                StrategyWalletFunctions::TransferAdminship.as_str(),
                new_admin,
                signer.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        let tx_hash = self
            .contract
            .signed_call(
                StrategyWalletFunctions::TransferAdminship.as_str(),
                new_admin,
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer,
            )
            .await?;
        get_blockchain_logger().log(
            format!("TransferAdminship new_admin={:?}", new_admin),
            tx_hash,
        )?;
        Ok(tx_hash)
    }

    pub async fn revoke_adminship(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key,
    ) -> Result<H256> {
        info!(
            "Revoking adminship of strategy wallet contract {:?} by {:?}",
            self.address(),
            signer.address(),
        );

        let estimated_gas = self
            .contract
            .estimate_gas(
                StrategyWalletFunctions::RevokeAdminship.as_str(),
                (),
                signer.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        let tx_hash = self
            .contract
            .signed_call(
                StrategyWalletFunctions::RevokeAdminship.as_str(),
                (),
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer,
            )
            .await?;
        Ok(tx_hash)
    }
}

enum StrategyWalletFunctions {
    Backer,
    Admin,
    RedeemFromStrategy,
    FullRedeemFromStrategy,
    TransferAdminship,
    RevokeAdminship,
}

impl StrategyWalletFunctions {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Backer => "backer",
            Self::Admin => "admin",
            Self::RedeemFromStrategy => "redeemFromStrategy",
            Self::FullRedeemFromStrategy => "fullRedeemFromStrategy",
            Self::TransferAdminship => "transferAdminship",
            Self::RevokeAdminship => "revokeAdminship",
        }
    }
}
