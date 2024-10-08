use std::sync::Arc;

use crate::contract::AbstractContract;
use crate::logger::get_blockchain_logger;
use crate::RpcCallError;
use crate::{
    deploy_contract, EitherTransport, EscrowAddresses, EthereumRpcConnection,
    EthereumRpcConnectionPool, MultiChainAddressTable,
};
use eyre::*;
use gen::model::EnumBlockChain;
use lib::log::DynLogger;
use lib::types::amount_to_display;
use tracing::info;
use web3::api::Eth;
use web3::contract::{Contract, Options};
use web3::signing::Key;
use web3::types::{Address, TransactionReceipt, H256, U256};
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
    pub fn new2(table: Arc<EscrowAddresses>) -> Self {
        let abi = ethabi::Contract::load(ESCROW_ABI_JSON.as_bytes()).unwrap();

        Self(AbstractContract {
            name: "Escrow".to_string(),
            abi,
            contract_addresses: table.0.clone(),
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
    pub async fn deploy(w3: Web3<T>, key: impl Key + Clone) -> Result<Self> {
        let address = key.address();
        let contract = deploy_contract(w3, key, address, "Escrow", DynLogger::empty()).await?;
        Ok(Self { contract })
    }

    pub fn new(eth: Eth<T>, address: Address) -> Result<Self> {
        let contract = Contract::from_json(eth, address, ESCROW_ABI_JSON.as_bytes())?;
        Ok(Self { contract })
    }

    pub fn address(&self) -> Address {
        self.contract.address()
    }

    pub async fn assets(&self, proprietor: Address) -> Result<Vec<Address>, RpcCallError> {
        Ok(self
            .contract
            .query(
                EscrowFunctions::Assets.as_str(),
                proprietor,
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn asset_balance(
        &self,
        proprietor: Address,
        asset: Address,
    ) -> Result<U256, RpcCallError> {
        Ok(self
            .contract
            .query(
                EscrowFunctions::AssetBalance.as_str(),
                (proprietor, asset),
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn assets_and_balances(
        &self,
        proprietor: Address,
    ) -> Result<(Vec<Address>, Vec<U256>), RpcCallError> {
        Ok(self
            .contract
            .query(
                EscrowFunctions::AssetsAndBalances.as_str(),
                proprietor,
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn accept_deposit(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key,
        proprietor: Address,
        asset: Address,
        amount: U256,
        logger: DynLogger,
    ) -> Result<H256, RpcCallError> {
        info!(
					"Accepting {:?} amount of asset {:?} from proprietor {:?} from escrow contract {:?} by {:?}",
					amount_to_display(amount),
					asset,
					proprietor,
					self.address(),
					signer.address(),
			);

        let estimated_gas = self
            .contract
            .estimate_gas(
                EscrowFunctions::AcceptDeposit.as_str(),
                (proprietor, asset, amount),
                signer.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        logger.log(format!(
            "Accepting {:?} amount of asset {:?} from proprietor {:?} from escrow contract {:?} by {:?}",
            amount_to_display(amount),
            asset,
            proprietor,
            self.address(),
            signer.address(),
        ));

        let tx_hash = self
            .contract
            .signed_call(
                EscrowFunctions::AcceptDeposit.as_str(),
                (proprietor, asset, amount),
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer,
            )
            .await?;

        get_blockchain_logger().log(
            format!(
                "Accepting {:?} amount of asset {:?} from proprietor {:?} tx_hash {:?}",
                amount_to_display(amount),
                asset,
                proprietor,
                tx_hash
            ),
            tx_hash,
        );

        Ok(tx_hash)
    }

    pub async fn estimate_gas_reject_deposit(
        &self,
        signer: impl Key,
        proprietor: Address,
        asset: Address,
        deposit_amount: U256,
        fee_recipient: Address,
        fee_amount: U256,
    ) -> Result<U256, RpcCallError> {
        let estimated_gas = self
            .contract
            .estimate_gas(
                EscrowFunctions::RejectDeposit.as_str(),
                (proprietor, asset, deposit_amount, fee_recipient, fee_amount),
                signer.address(),
                Options::default(),
            )
            .await?;

        Ok(estimated_gas)
    }

    pub async fn reject_deposit(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key,
        proprietor: Address,
        asset: Address,
        deposit_amount: U256,
        fee_recipient: Address,
        fee_amount: U256,
        logger: DynLogger,
    ) -> Result<H256, RpcCallError> {
        info!(
					"Rejecting {:?} amount of asset {:?} from proprietor {:?}, and transferring fee amount {:?} to fee recipient {:?} from escrow contract {:?} by {:?}",
					amount_to_display(deposit_amount),
					asset,
					proprietor,
					fee_amount,
					fee_recipient,
					self.address(),
					signer.address(),
			);

        let estimated_gas = self
            .contract
            .estimate_gas(
                EscrowFunctions::RejectDeposit.as_str(),
                (proprietor, asset, deposit_amount, fee_recipient, fee_amount),
                signer.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        logger.log(format!(
					"Rejecting {:?} amount of asset {:?} from proprietor {:?}, and transferring fee amount {:?} to fee recipient {:?} from escrow contract {:?} by {:?}",
					amount_to_display(deposit_amount),
					asset,
					proprietor,
					fee_amount,
					fee_recipient,
					self.address(),
					signer.address(),
        ));

        let tx_hash = self
            .contract
            .signed_call(
                EscrowFunctions::RejectDeposit.as_str(),
                (proprietor, asset, deposit_amount, fee_recipient, fee_amount),
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer,
            )
            .await?;

        get_blockchain_logger().log(
            format!(
                "Reject deposit amount {:?} of asset {:?} from proprietor {:?} and send {:?} as fees to {:?} tx_hash {:?}",
                amount_to_display(deposit_amount), asset, proprietor, fee_amount, fee_recipient, tx_hash
            ),
            tx_hash,
        );

        Ok(tx_hash)
    }

    pub async fn withdraw(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key,
        asset: Address,
        amount: U256,
    ) -> Result<H256, RpcCallError> {
        let estimated_gas = self
            .contract
            .estimate_gas(
                EscrowFunctions::Withdraw.as_str(),
                (asset, amount),
                signer.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        let tx_hash = self
            .contract
            .signed_call(
                EscrowFunctions::Withdraw.as_str(),
                (asset, amount),
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer,
            )
            .await?;

        Ok(tx_hash)
    }

    pub async fn transfer_asset_from(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key,
        proprietor: Address,
        asset: Address,
        amount: U256,
        recipient: Address,
        logger: DynLogger,
    ) -> Result<H256, RpcCallError> {
        info!(
            "Transferring {} amount of {:?} asset from {:?} proprietor to {:?} recipient from escrow contract {:?} by {:?}",
            amount_to_display(amount),
            asset,
            proprietor,
            recipient,
            self.address(),
            signer.address(),
        );

        let estimated_gas = self
            .contract
            .estimate_gas(
                EscrowFunctions::TransferAssetFrom.as_str(),
                (proprietor, asset, amount, recipient),
                signer.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        logger.log(format!(
            "Transferring {} amount of {:?} asset from {:?} proprietor to {:?} recipient from escrow contract {:?} by {:?}",
            amount_to_display(amount),
            asset,
            proprietor,
            recipient,
            self.address(),
            signer.address(),
        ));

        let tx_hash = self
            .contract
            .signed_call(
                EscrowFunctions::TransferAssetFrom.as_str(),
                (proprietor, asset, amount, recipient),
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer,
            )
            .await?;

        get_blockchain_logger().log(
            format!(
                "Transfer {:?} amount of {:?} asset from {:?} proprietor to {:?} tx_hash {:?}",
                amount_to_display(amount),
                asset,
                proprietor,
                recipient,
                tx_hash,
            ),
            tx_hash,
        );
        Ok(tx_hash)
    }

    pub async fn refund_asset(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key,
        proprietor: Address,
        asset: Address,
        amount: U256,
        logger: DynLogger,
    ) -> Result<H256, RpcCallError> {
        info!(
					"Refunding {:?} amount of {:?} asset to {:?} proprietor from escrow contract {:?} by {:?}",
					amount_to_display(amount),
					asset,
					proprietor,
					self.address(),
					signer.address(),
			);

        let estimated_gas = self
            .contract
            .estimate_gas(
                EscrowFunctions::RefundAsset.as_str(),
                (proprietor, asset, amount),
                signer.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        logger.log(format!(
					"Refunding {:?} amount of {:?} asset to {:?} proprietor from escrow contract {:?} by {:?}",
					amount_to_display(amount),
					asset,
					proprietor,
					self.address(),
					signer.address(),
				));

        let tx_hash = self
            .contract
            .signed_call(
                EscrowFunctions::RefundAsset.as_str(),
                (proprietor, asset, amount),
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer,
            )
            .await?;

        get_blockchain_logger().log(
            format!(
                "Refund {:?} amount of {:?} asset to {:?} proprietor tx_hash {:?}",
                amount_to_display(amount),
                asset,
                proprietor,
                tx_hash,
            ),
            tx_hash,
        );
        Ok(tx_hash)
    }

    pub async fn rescue_assets(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key,
        recipient: Address,
        assets: Vec<Address>,
        amounts: Vec<U256>,
        logger: DynLogger,
    ) -> Result<H256, RpcCallError> {
        info!(
            "Rescuing {:?} amounts of {:?} assets to {:?} from escrow contract {:?} by {:?}",
            amounts
                .iter()
                .cloned()
                .map(amount_to_display)
                .collect::<Vec<_>>(),
            assets.clone(),
            recipient,
            self.address(),
            signer.address(),
        );

        let estimated_gas = self
            .contract
            .estimate_gas(
                EscrowFunctions::RescueAssets.as_str(),
                (recipient, assets.clone(), amounts.clone()),
                signer.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        logger.log(format!(
            "Rescuing {:?} amounts of {:?} assets to {:?} from escrow contract {:?} by {:?}",
            amounts
                .iter()
                .cloned()
                .map(amount_to_display)
                .collect::<Vec<_>>(),
            assets.clone(),
            recipient,
            self.address(),
            signer.address(),
        ));

        let tx_hash = self
            .contract
            .signed_call(
                EscrowFunctions::RescueAssets.as_str(),
                (recipient, assets.clone(), amounts.clone()),
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer,
            )
            .await?;

        get_blockchain_logger().log(
            format!(
                "Rescue {:?} amounts of {:?} assets to {:?} proprietor tx_hash {:?}",
                amounts
                    .iter()
                    .cloned()
                    .map(amount_to_display)
                    .collect::<Vec<_>>(),
                assets.clone(),
                recipient,
                tx_hash,
            ),
            tx_hash,
        );
        Ok(tx_hash)
    }

    pub async fn add_blacklisted_account(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key,
        blacklisted_account: Address,
        logger: DynLogger,
    ) -> Result<H256, RpcCallError> {
        info!(
            "Adding {:?} blacklisted account to escrow contract {:?} by {:?}",
            blacklisted_account,
            self.address(),
            signer.address(),
        );

        let estimated_gas = self
            .contract
            .estimate_gas(
                EscrowFunctions::AddBlacklistedAccount.as_str(),
                blacklisted_account,
                signer.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        logger.log(format!(
            "Adding {:?} blacklisted account to escrow contract {:?} by {:?}",
            blacklisted_account,
            self.address(),
            signer.address(),
        ));

        let tx_hash = self
            .contract
            .signed_call(
                EscrowFunctions::AddBlacklistedAccount.as_str(),
                blacklisted_account,
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer,
            )
            .await?;

        get_blockchain_logger().log(
            format!(
                "Add {:?} blacklisted account to escrow contract {:?} tx_hash {:?}",
                blacklisted_account,
                self.address(),
                tx_hash,
            ),
            tx_hash,
        );
        Ok(tx_hash)
    }

    pub async fn remove_blacklisted_account(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key,
        blacklisted_account: Address,
        logger: DynLogger,
    ) -> Result<H256, RpcCallError> {
        info!(
            "Removing {:?} blacklisted account to escrow contract {:?} by {:?}",
            blacklisted_account,
            self.address(),
            signer.address(),
        );

        let estimated_gas = self
            .contract
            .estimate_gas(
                EscrowFunctions::RemoveBlacklistedAccount.as_str(),
                blacklisted_account,
                signer.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        logger.log(format!(
            "Removing {:?} blacklisted account to escrow contract {:?} by {:?}",
            blacklisted_account,
            self.address(),
            signer.address(),
        ));

        let tx_hash = self
            .contract
            .signed_call(
                EscrowFunctions::RemoveBlacklistedAccount.as_str(),
                blacklisted_account,
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer,
            )
            .await?;

        get_blockchain_logger().log(
            format!(
                "Remove {:?} blacklisted account to escrow contract {:?} tx_hash {:?}",
                blacklisted_account,
                self.address(),
                tx_hash,
            ),
            tx_hash,
        );
        Ok(tx_hash)
    }

    pub async fn blacklisted_accounts(&self) -> Result<Address, RpcCallError> {
        Ok(self
            .contract
            .query(
                EscrowFunctions::BlacklistedAccounts.as_str(),
                (),
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn account_is_blacklisted(&self, account: Address) -> Result<bool, RpcCallError> {
        Ok(self
            .contract
            .query(
                EscrowFunctions::AccountIsBlacklisted.as_str(),
                account,
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn transfer_ownership(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key,
        new_owner: Address,
    ) -> Result<H256, RpcCallError> {
        let estimated_gas = self
            .contract
            .estimate_gas(
                EscrowFunctions::TransferOwnership.as_str(),
                new_owner,
                signer.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        info!(
            "Transferring ownership from {:?} to {:?} of escrow contract {:?} by {:?}",
            self.owner().await?,
            new_owner,
            self.address(),
            signer.address(),
        );

        Ok(self
            .contract
            .signed_call(
                EscrowFunctions::TransferOwnership.as_str(),
                new_owner,
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer,
            )
            .await?)
    }

    pub async fn owner(&self) -> Result<Address, RpcCallError> {
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
}

enum EscrowFunctions {
    Assets,
    AssetBalance,
    AssetsAndBalances,
    AcceptDeposit,
    RejectDeposit,
    Withdraw,
    TransferAssetFrom,
    RefundAsset,
    RescueAssets,
    AddBlacklistedAccount,
    RemoveBlacklistedAccount,
    BlacklistedAccounts,
    AccountIsBlacklisted,
    TransferOwnership,
    Owner,
}

impl EscrowFunctions {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Assets => "assets",
            Self::AssetBalance => "assetBalance",
            Self::AssetsAndBalances => "assetsAndBalances",
            Self::AcceptDeposit => "acceptDeposit",
            Self::RejectDeposit => "rejectDeposit",
            Self::Withdraw => "withdraw",
            Self::TransferAssetFrom => "transferAssetFrom",
            Self::RefundAsset => "refundAsset",
            Self::RescueAssets => "rescueAssets",
            Self::AddBlacklistedAccount => "addBlacklistedAccount",
            Self::RemoveBlacklistedAccount => "removeBlacklistedAccount",
            Self::BlacklistedAccounts => "blacklistedAccounts",
            Self::AccountIsBlacklisted => "accountIsBlacklisted",
            Self::TransferOwnership => "transferOwnership",
            Self::Owner => "owner",
        }
    }
}

#[derive(Debug, Clone)]
pub struct EscrowWithdrawEvent {
    pub proprietor: Address,
    pub asset: Address,
    pub amount: U256,
}

pub fn parse_escrow_withdraw_event(
    escrow_address: Address,
    receipt: TransactionReceipt,
) -> Result<EscrowWithdrawEvent> {
    let escrow = web3::ethabi::Contract::load(ESCROW_ABI_JSON.as_bytes())?;
    let withdraw_event = escrow
        .event("Withdraw")
        .context("failed to get Withdraw event from escrow")?;

    for log in receipt.logs {
        /* there can only be 4 indexed (topic) values in a event log */
        /* 1st topic is always the hash of the event signature */
        if log.topics[0] == withdraw_event.signature()
						/* address of the contract that fired the event */
						&& log.address == escrow_address
        {
            /* 2nd topic is sender of the call, should be the depositor address */
            /* topics have 32 bytes, so we must fetch the last 20 bytes for an address */
            let proprietor_bytes = log.topics[1].as_bytes();
            if proprietor_bytes.len() < 32 {
                return Err(eyre!("invalid topic length"));
            }
            let proprietor = Address::from_slice(&proprietor_bytes[12..]);

            /* 3rd topic is the asset that was withdrawn (the previously depositted asset) */
            /* topics have 32 bytes, so we must fetch the last 20 bytes for an address */
            let asset_bytes = log.topics[2].as_bytes();
            if asset_bytes.len() < 32 {
                return Err(eyre!("invalid topic length"));
            }
            let asset = Address::from_slice(&asset_bytes[12..]);

            /* 4th topic is the amount of assets withdrawn */
            /* topics have 32 bytes, and so does uint256, so we fetch the entire topic for the amount */
            let amount_bytes = log.topics[3].as_bytes();
            if amount_bytes.len() < 32 {
                return Err(eyre!("invalid topic length"));
            }
            let amount = U256::from_big_endian(&amount_bytes);

            return Ok(EscrowWithdrawEvent {
                proprietor,
                asset,
                amount,
            });
        }
    }
    Err(eyre!("could not find Withdraw event in receipt"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::time::Duration;

    use once_cell::sync::Lazy;
    use tokio::sync::Mutex;

    use crate::contract_wrappers::mock_erc20::deploy_mock_erc20;
    use crate::signer::Secp256k1SecretKey;
    use crate::{
        wait_for_confirmations_simple, EthereumRpcConnectionGuard, EthereumRpcConnectionPool,
        ANVIL_PRIV_KEY_1,
    };
    use gen::model::EnumBlockChain;

    static TX_CONN: Lazy<Arc<Mutex<Option<EthereumRpcConnectionGuard>>>> =
        Lazy::new(|| Arc::new(Mutex::new(None)));

    async fn get_tx_conn() -> Result<Arc<Mutex<Option<EthereumRpcConnectionGuard>>>> {
        /* since tests are parallel and use a single key, ensure only one test publishes transactions at a time */
        /* to avoid "nonce too low" errors */
        let tx_conn_arc = TX_CONN.clone();
        let mut tx_conn = tx_conn_arc.lock().await;
        if tx_conn.is_none() {
            *tx_conn = Some(
                EthereumRpcConnectionPool::new()
                    .get(EnumBlockChain::LocalNet)
                    .await?,
            );
        }
        Ok(tx_conn_arc.clone())
    }

    #[tokio::test]
    async fn test_parse_escrow_withdraw_event() -> Result<()> {
        let tx_conn_wrapper = get_tx_conn().await?;
        let mut tx_conn_guard = tx_conn_wrapper.lock().await;
        let tx_conn = tx_conn_guard.as_mut().unwrap();

        let god_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;

        let escrow_contract = EscrowContract::deploy(tx_conn.clone(), god_key.clone()).await?;

        let mock_erc20 = deploy_mock_erc20(tx_conn.clone(), god_key.clone()).await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20
                .mint(
                    &tx_conn,
                    god_key.clone(),
                    god_key.address(),
                    U256::from(100),
                )
                .await?,
            Duration::from_secs(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20
                .transfer(
                    &tx_conn,
                    god_key.clone(),
                    escrow_contract.address(),
                    U256::from(100),
                )
                .await?,
            Duration::from_secs(1),
            10,
        )
        .await?;

        let logger = DynLogger::new(Arc::new(move |msg| {
            println!("{}", msg);
        }));
        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            escrow_contract
                .accept_deposit(
                    &tx_conn,
                    god_key.clone(),
                    god_key.address(),
                    mock_erc20.address,
                    U256::from(100),
                    logger,
                )
                .await?,
            Duration::from_secs(1),
            10,
        )
        .await?;

        let withdraw_receipt = wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            escrow_contract
                .withdraw(
                    &tx_conn,
                    god_key.clone(),
                    mock_erc20.address,
                    U256::from(100),
                )
                .await?,
            Duration::from_secs(1),
            10,
        )
        .await?;

        let withdraw_event =
            parse_escrow_withdraw_event(escrow_contract.address(), withdraw_receipt)?;

        assert_eq!(withdraw_event.proprietor, god_key.address());
        assert_eq!(withdraw_event.asset, mock_erc20.address);
        assert_eq!(withdraw_event.amount, U256::from(100));

        Ok(())
    }
}
