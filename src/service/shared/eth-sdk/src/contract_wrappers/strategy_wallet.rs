use std::time::Duration;

use eyre::*;
use tokio::time::sleep;
use tracing::info;
use web3::contract::{Contract, Options};
use web3::signing::Key;
use web3::types::{Address, H256, U256};
use web3::{ethabi, Transport, Web3};

use crate::contract::AbstractContract;
use crate::{
    deploy_contract, wait_for_confirmations_simple, EitherTransport, EthereumRpcConnection,
    EthereumRpcConnectionPool, MultiChainAddressTable, TransactionFetcher, TxStatus,
};
use gen::model::EnumBlockChain;

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
        key: impl Key,
        backer: Address,
        admin: Address,
    ) -> Result<Self> {
        let contract = deploy_contract(w3.clone(), key, (backer, admin), "StrategyWallet").await?;
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
        signer: impl Key,
        strategy: Address,
        shares: U256,
    ) -> Result<H256> {
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

        info!(
            "Redeeming {:?} shares from strategy pool contract {:?} using strategy wallet contract {:?} by {:?}",
            shares,
            strategy,
            self.address(),
            signer.address(),
        );

        Ok(self
            .contract
            .signed_call(
                StrategyWalletFunctions::RedeemFromStrategy.as_str(),
                (strategy, shares),
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer,
            )
            .await?)
    }

    pub async fn full_redeem_from_strategy(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key,
        strategy: Address,
    ) -> Result<H256> {
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

        info!(
						"Redeeming all shares from strategy pool contract {:?} using strategy wallet contract {:?} by {:?}",
						strategy,
						self.address(),
						signer.address(),
				);

        Ok(self
            .contract
            .signed_call(
                StrategyWalletFunctions::FullRedeemFromStrategy.as_str(),
                strategy,
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer,
            )
            .await?)
    }

    pub async fn transfer_adminship(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key,
        new_admin: Address,
    ) -> Result<H256> {
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

        info!(
            "Transferring adminship of strategy wallet contract {:?} to {:?} by {:?}",
            self.address(),
            new_admin,
            signer.address(),
        );

        Ok(self
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
            .await?)
    }

    pub async fn revoke_adminship(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key,
    ) -> Result<H256> {
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

        info!(
            "Revoking adminship of strategy wallet contract {:?} by {:?}",
            self.address(),
            signer.address(),
        );

        Ok(self
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
            .await?)
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

pub async fn redeem_from_strategy_and_ensure_success(
    contract: StrategyWalletContract<EitherTransport>,
    conn: &EthereumRpcConnection,
    confirmations: u64,
    max_retry: usize,
    wait_timeout: Duration,
    signer: impl Key + Clone,
    strategy: Address,
    shares: U256,
) -> Result<H256> {
    /* publish transaction */
    let mut tx_hash = contract
        .redeem_from_strategy(&conn, signer.clone(), strategy.clone(), shares.clone())
        .await?;
    let mut retries: usize = 0;
    while retries < max_retry {
        /* wait for transaction receipt */
        /* after it has a receipt, it was included in a block */
        let tx_receipt =
            wait_for_confirmations_simple(&conn.eth(), tx_hash, wait_timeout, max_retry).await?;

        /* get receipt block number */
        let tx_block_number = tx_receipt
            .block_number
            .ok_or_else(|| eyre!("transaction has receipt but was not included in a block"))?
            .as_u64();
        let mut current_block_number = conn.eth().block_number().await?.as_u64();

        while current_block_number - tx_block_number < confirmations {
            /* wait for confirmations */
            /* more confirmations = greater probability that the transaction status is canonical */
            current_block_number = conn.eth().block_number().await?.as_u64();
            sleep(wait_timeout).await;
        }

        /* after confirmations, find out transaction status */
        let mut tx = TransactionFetcher::new(tx_hash);
        tx.update(&conn).await?;

        match tx.get_status() {
            TxStatus::Successful => {
                /* transaction is successful after confirmations, consider it canonical*/
                break;
            }
            TxStatus::Pending => {
                /* TODO: check if this is even possible */
                /* transaction had a receipt (was included in a block) but has somehow returned to the mempool */
                /* wait for the new receipt */
                retries += 1;
                continue;
            }
            TxStatus::Reverted | TxStatus::NotFound => {
                /* transaction is reverted or doesn't exist after confirmations, try again */
                retries += 1;
                tx_hash = contract
                    .redeem_from_strategy(&conn, signer.clone(), strategy.clone(), shares.clone())
                    .await?;
            }
            _ => continue,
        }
    }
    Ok(tx_hash)
}

pub async fn full_redeem_from_strategy_and_ensure_success(
    contract: StrategyWalletContract<EitherTransport>,
    conn: &EthereumRpcConnection,
    confirmations: u64,
    max_retry: usize,
    wait_timeout: Duration,
    signer: impl Key + Clone,
    strategy: Address,
) -> Result<H256> {
    /* publish transaction */
    let mut tx_hash = contract
        .full_redeem_from_strategy(&conn, signer.clone(), strategy.clone())
        .await?;
    let mut retries: usize = 0;
    while retries < max_retry {
        /* wait for transaction receipt */
        /* after it has a receipt, it was included in a block */
        let tx_receipt =
            wait_for_confirmations_simple(&conn.eth(), tx_hash, wait_timeout, max_retry).await?;

        /* get receipt block number */
        let tx_block_number = tx_receipt
            .block_number
            .ok_or_else(|| eyre!("transaction has receipt but was not included in a block"))?
            .as_u64();
        let mut current_block_number = conn.eth().block_number().await?.as_u64();

        while current_block_number - tx_block_number < confirmations {
            /* wait for confirmations */
            /* more confirmations = greater probability that the transaction status is canonical */
            current_block_number = conn.eth().block_number().await?.as_u64();
            sleep(wait_timeout).await;
        }

        /* after confirmations, find out transaction status */
        let mut tx = TransactionFetcher::new(tx_hash);
        tx.update(&conn).await?;

        match tx.get_status() {
            TxStatus::Successful => {
                /* transaction is successful after confirmations, consider it canonical*/
                break;
            }
            TxStatus::Pending => {
                /* TODO: check if this is even possible */
                /* transaction had a receipt (was included in a block) but has somehow returned to the mempool */
                /* wait for the new receipt */
                retries += 1;
                continue;
            }
            TxStatus::Reverted | TxStatus::NotFound => {
                /* transaction is reverted or doesn't exist after confirmations, try again */
                retries += 1;
                tx_hash = contract
                    .full_redeem_from_strategy(&conn, signer.clone(), strategy.clone())
                    .await?;
            }
            _ => continue,
        }
    }
    Ok(tx_hash)
}
