use crate::contract::AbstractContract;
use crate::logger::get_blockchain_logger;
use crate::utils::wait_for_confirmations;
use crate::{
    deploy_contract, EitherTransport, EthereumRpcConnection, EthereumRpcConnectionPool,
    MultiChainAddressTable,
};
use eyre::*;
use gen::model::EnumBlockChain;
use lib::log::DynLogger;
use lib::types::amount_to_display;
use std::collections::HashMap;
use std::time::Duration;
use tracing::info;
use web3::contract::{Contract, Options};
use web3::signing::Key;
use web3::types::TransactionReceipt;
use web3::types::{Address, H256, U256};
use web3::{ethabi, Transport, Web3};

const POOL_ABI_JSON: &str = include_str!("strategy_pool.json");
pub struct AbstractStrategyPoolContract(AbstractContract<()>);
impl AbstractStrategyPoolContract {
    pub fn new(name: String, table: MultiChainAddressTable<()>) -> Self {
        let abi = ethabi::Contract::load(POOL_ABI_JSON.as_bytes()).unwrap();
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
    ) -> Result<StrategyPoolContract<EitherTransport>> {
        let contract = self.0.get(pool, blockchain, ()).await?;
        Ok(StrategyPoolContract { contract })
    }
}
#[derive(Debug, Clone)]
pub struct StrategyPoolContract<T: Transport> {
    contract: Contract<T>,
}

impl<T: Transport> StrategyPoolContract<T> {
    pub async fn deploy(
        w3: Web3<T>,
        key: impl Key + Clone,
        name: String,
        symbol: String,
        herald: Address,
        logger: DynLogger,
    ) -> Result<Self> {
        let params = (name.clone(), symbol.clone(), key.address(), herald);
        let contract = deploy_contract(w3.clone(), key, params, "StrategyPool", logger).await?;
        Ok(Self { contract })
    }

    pub fn new(w3: Web3<T>, address: Address) -> Result<Self> {
        let contract = Contract::from_json(w3.eth(), address, POOL_ABI_JSON.as_bytes())?;
        Ok(Self { contract })
    }

    pub fn address(&self) -> Address {
        self.contract.address()
    }

    pub async fn decimals(&self) -> Result<U256> {
        Ok(self
            .contract
            .query(
                StrategyPoolFunctions::Decimals.as_str(),
                (),
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn total_supply(&self) -> Result<U256> {
        Ok(self
            .contract
            .query(
                StrategyPoolFunctions::TotalSupply.as_str(),
                (),
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn balance_of(&self, owner: Address) -> Result<U256> {
        Ok(self
            .contract
            .query(
                StrategyPoolFunctions::BalanceOf.as_str(),
                owner,
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn assets(&self) -> Result<Vec<Address>> {
        Ok(self
            .contract
            .query(
                StrategyPoolFunctions::Assets.as_str(),
                (),
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn asset_balance(&self, asset: Address) -> Result<U256> {
        Ok(self
            .contract
            .query(
                StrategyPoolFunctions::AssetBalance.as_str(),
                asset,
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn assets_and_balances(&self) -> Result<(Vec<Address>, Vec<U256>)> {
        Ok(self
            .contract
            .query(
                StrategyPoolFunctions::AssetsAndBalances.as_str(),
                (),
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn max_mint(&self) -> Result<U256> {
        Ok(self
            .contract
            .query(
                StrategyPoolFunctions::MaxMint.as_str(),
                (),
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn deposit(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key + Clone,
        assets: Vec<Address>,
        amounts: Vec<U256>,
        pool_tokens: U256,
        receiver: Address,
        logger: DynLogger,
    ) -> Result<H256> {
        info!("Depositing amounts {:?} of assets {:?} to mint {:?} pool tokens to receiver {:?} to strategy pool contract {:?} by {:?}",
						amounts.iter().cloned().map(amount_to_display).collect::<Vec<_>>(),
						assets.clone(),
						pool_tokens,
						receiver,
						self.address(),
						signer.address(),
				);

        let estimated_gas = self
            .contract
            .estimate_gas(
                StrategyPoolFunctions::Deposit.as_str(),
                (assets.clone(), amounts.clone(), pool_tokens, receiver),
                signer.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        logger.log(
            format!(
                "Depositing amounts {:?} of assets {:?} to mint {:?} pool tokens to receiver {:?} to strategy pool contract {:?} by {:?}",
                amounts.iter().cloned().map(amount_to_display).collect::<Vec<_>>(),
                assets.clone(),
                pool_tokens,
                receiver,
                self.address(),
                signer.address(),
            ),
        );
        let tx_hash = self
            .contract
            .signed_call(
                StrategyPoolFunctions::Deposit.as_str(),
                (assets.clone(), amounts.clone(), pool_tokens, receiver),
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer.clone(),
            )
            .await?;
        get_blockchain_logger().log(
            format!(
                "Depositing amounts {:?} of assets {:?} to mint {:?} pool tokens to receiver {:?} to strategy pool contract {:?} by {:?}",
                amounts.iter().cloned().map(amount_to_display).collect::<Vec<_>>(),
                assets.clone(),
                pool_tokens,
                receiver,
                self.address(),
                signer.address(),
            ),
        tx_hash
        )?;
        Ok(tx_hash)
    }

    pub async fn max_redeem(&self, owner: Address) -> Result<U256> {
        Ok(self
            .contract
            .query(
                StrategyPoolFunctions::MaxRedeem.as_str(),
                owner,
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn redeem(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key + Clone,
        pool_tokens: U256,
        receiver: Address,
        owner: Address,
    ) -> Result<H256> {
        info!("Redeeming {:?} pool tokens to receiver {:?} from owner {:?} from strategy pool contract {:?} by {:?}",
					amount_to_display(pool_tokens),
					receiver,
					owner,
					self.address(),
					signer.address(),
			);

        let estimated_gas = self
            .contract
            .estimate_gas(
                StrategyPoolFunctions::Redeem.as_str(),
                (owner, receiver, pool_tokens),
                signer.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        let tx_hash = self
            .contract
            .signed_call(
                StrategyPoolFunctions::Redeem.as_str(),
                (owner, receiver, pool_tokens),
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer.clone(),
            )
            .await?;
        get_blockchain_logger().log(
            format!(
                "Redeeming {:?} pool tokens to receiver {:?} from owner {:?} from strategy pool contract {:?} by {:?}",
                amount_to_display(pool_tokens),
                receiver,
                owner,
                self.address(),
                signer.address(),
            ),
            tx_hash,
        )?;
        Ok(tx_hash)
    }

    pub async fn withdraw(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key + Clone,
        receiver: Address,
        assets: Vec<Address>,
        amounts: Vec<U256>,
        logger: DynLogger,
    ) -> Result<H256> {
        info!("Withdrawing {:?} amounts of {:?} assets to receiver {:?} from strategy pool contract {:?} by {:?}",
						amounts.iter().cloned().map(amount_to_display).collect::<Vec<_>>(),
						assets,
						receiver,
						self.address(),
						signer.address(),
				);

        let estimated_gas = self
            .contract
            .estimate_gas(
                StrategyPoolFunctions::Withdraw.as_str(),
                (receiver, assets.clone(), amounts.clone()),
                signer.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        logger.log(
				format!("Withdrawing {:?} amounts of {:?} assets to receiver {:?} from strategy pool contract {:?} by {:?}",
						amounts.iter().cloned().map(amount_to_display).collect::<Vec<_>>(),
						assets,
						receiver,
						self.address(),
						signer.address(),
					),
			);
        let tx_hash = self
            .contract
            .signed_call(
                StrategyPoolFunctions::Withdraw.as_str(),
                (receiver.clone(), assets.clone(), amounts.clone()),
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer.clone(),
            )
            .await?;
        get_blockchain_logger().log(
                format!("Withdrawing {:?} amounts of {:?} assets to receiver {:?} from strategy pool contract {:?} by {:?}",
                        amounts.iter().cloned().map(amount_to_display).collect::<Vec<_>>(),
                        assets,
                        receiver,
                        self.address(),
                        signer.address(),
                    ),
                tx_hash,
            )?;
        Ok(tx_hash)
    }

    pub async fn acquire_asset_before_trade(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key + Clone,
        asset: Address,
        amount: U256,
    ) -> Result<H256> {
        info!(
						"Acquiring {:?} amount of asset {:?} before trade from strategy pool contract {:?} by {:?}",
						amount_to_display(amount),
						asset,
						self.address(),
						signer.address(),
				);

        let estimated_gas = self
            .contract
            .estimate_gas(
                StrategyPoolFunctions::AcquireAssetBeforeTrade.as_str(),
                (asset, amount),
                signer.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        let tx_hash = self
            .contract
            .signed_call(
                StrategyPoolFunctions::AcquireAssetBeforeTrade.as_str(),
                (asset, amount),
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer.clone(),
            )
            .await?;
        get_blockchain_logger().log(
            format!(
                "Acquiring {:?} amount of asset {:?} before trade from strategy pool contract {:?} by {:?}",
                amount_to_display(amount),
                asset,
                self.address(),
                signer.address(),
            ),
            tx_hash,
        )?;
        Ok(tx_hash)
    }

    pub async fn give_back_assets_after_trade(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key + Clone,
        assets: Vec<Address>,
        amounts: Vec<U256>,
    ) -> Result<H256> {
        info!(
						"Giving back {:?} amounts of assets {:?} after trade to strategy pool contract {:?} by {:?}",
						amounts.iter().cloned().map(amount_to_display).collect::<Vec<_>>(),
						assets.clone(),
						self.address(),
						signer.address(),
				);

        let estimated_gas = self
            .contract
            .estimate_gas(
                StrategyPoolFunctions::GiveBackAssetsAfterTrade.as_str(),
                (assets.clone(), amounts.clone()),
                signer.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        let tx_hash = self
            .contract
            .signed_call(
                StrategyPoolFunctions::GiveBackAssetsAfterTrade.as_str(),
                (assets.clone(), amounts.clone()),
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer.clone(),
            )
            .await?;
        get_blockchain_logger().log(
            format!(
                "Giving back {:?} amounts of assets {:?} after trade to strategy pool contract {:?} by {:?}",
                amounts.iter().cloned().map(amount_to_display).collect::<Vec<_>>(),
                assets,
                self.address(),
                signer.address(),
            ),
            tx_hash,
        )?;
        Ok(tx_hash)
    }

    pub async fn transfer_ownership(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key + Clone,
        new_owner: Address,
    ) -> Result<H256> {
        info!(
            "Transferring strategy pool contract {:?} ownership from {:?} to {:?} by {:?}",
            self.address(),
            self.owner().await?,
            new_owner,
            signer.address(),
        );

        let estimated_gas = self
            .contract
            .estimate_gas(
                StrategyPoolFunctions::TransferOwnership.as_str(),
                new_owner,
                signer.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        let tx_hash = self
            .contract
            .signed_call(
                StrategyPoolFunctions::TransferOwnership.as_str(),
                new_owner,
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer.clone(),
            )
            .await?;
        get_blockchain_logger().log(
            format!(
                "Transferring strategy pool contract {:?} ownership from {:?} to {:?} by {:?}",
                self.address(),
                self.owner().await?,
                new_owner,
                signer.address(),
            ),
            tx_hash,
        )?;
        Ok(tx_hash)
    }

    pub async fn owner(&self) -> Result<Address> {
        Ok(self
            .contract
            .query(
                StrategyPoolFunctions::Owner.as_str(),
                (),
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn is_paused(&self) -> Result<bool> {
        Ok(self
            .contract
            .query(
                StrategyPoolFunctions::IsPaused.as_str(),
                (),
                None,
                Options::default(),
                None,
            )
            .await?)
    }
}

enum StrategyPoolFunctions {
    Decimals,
    TotalSupply,
    BalanceOf,
    Assets,
    AssetBalance,
    AssetsAndBalances,
    MaxMint,
    Deposit,
    MaxRedeem,
    Redeem,
    Withdraw,
    AcquireAssetBeforeTrade,
    GiveBackAssetsAfterTrade,
    TransferOwnership,
    Owner,
    IsPaused,
}

impl StrategyPoolFunctions {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Decimals => "decimals",
            Self::TotalSupply => "totalSupply",
            Self::BalanceOf => "balanceOf",
            Self::Assets => "assets",
            Self::AssetBalance => "assetBalance",
            Self::AssetsAndBalances => "assetsAndBalances",
            Self::MaxMint => "maxMint",
            Self::Deposit => "deposit",
            Self::MaxRedeem => "maxRedeem",
            Self::Redeem => "redeem",
            Self::Withdraw => "withdraw",
            Self::AcquireAssetBeforeTrade => "acquireAssetBeforeTrade",
            Self::GiveBackAssetsAfterTrade => "giveBackAssetsAfterTrade",
            Self::TransferOwnership => "transferOwnership",
            Self::Owner => "owner",
            Self::IsPaused => "paused",
        }
    }
}

pub async fn sp_deposit_to_and_ensure_success(
    contract: StrategyPoolContract<EitherTransport>,
    conn: &EthereumRpcConnection,
    confirmations: u64,
    max_retry: u64,
    poll_interval: Duration,
    signer: impl Key + Clone,
    mut assets_amounts: HashMap<Address, U256>,
    strategy_tokens: U256,
    receiver: Address,
    logger: DynLogger,
) -> Result<H256> {
    assets_amounts = assets_amounts
        .into_iter()
        .filter(|x| !x.1.is_zero())
        .collect();
    /* publish transaction */
    let tx_hash = contract
        .deposit(
            &conn,
            signer.clone(),
            assets_amounts.keys().cloned().collect(),
            assets_amounts.values().cloned().collect(),
            strategy_tokens,
            receiver,
            logger,
        )
        .await?;
    let _tx_receipt = wait_for_confirmations(
        &conn.eth(),
        tx_hash,
        poll_interval,
        max_retry,
        confirmations,
    )
    .await?;

    Ok(tx_hash)
}

pub async fn withdraw_and_ensure_success(
    contract: StrategyPoolContract<EitherTransport>,
    conn: &EthereumRpcConnection,
    confirmations: u64,
    max_retry: u64,
    poll_interval: Duration,
    signer: impl Key + Clone,
    assets: Vec<Address>,
    amounts: Vec<U256>,
    receiver: Address,
    logger: DynLogger,
) -> Result<H256> {
    /* publish transaction */
    let tx_hash = contract
        .withdraw(&conn, signer.clone(), receiver, assets, amounts, logger)
        .await?;
    let _tx_receipt = wait_for_confirmations(
        &conn.eth(),
        tx_hash,
        poll_interval,
        max_retry,
        confirmations,
    )
    .await?;

    Ok(tx_hash)
}

pub async fn acquire_asset_before_trade_and_ensure_success(
    contract: StrategyPoolContract<EitherTransport>,
    conn: &EthereumRpcConnection,
    confirmations: u64,
    max_retry: u64,
    poll_interval: Duration,
    signer: impl Key + Clone,
    asset: Address,
    amount: U256,
) -> Result<H256> {
    /* publish transaction */
    let tx_hash = contract
        .acquire_asset_before_trade(&conn, signer.clone(), asset, amount)
        .await?;
    let _tx_receipt = wait_for_confirmations(
        &conn.eth(),
        tx_hash,
        poll_interval,
        max_retry,
        confirmations,
    )
    .await?;

    Ok(tx_hash)
}

pub async fn give_back_assets_after_trade_and_ensure_success(
    contract: StrategyPoolContract<EitherTransport>,
    conn: &EthereumRpcConnection,
    confirmations: u64,
    max_retry: u64,
    poll_interval: Duration,
    signer: impl Key + Clone,
    assets: Vec<Address>,
    amounts: Vec<U256>,
) -> Result<H256> {
    /* publish transaction */
    let tx_hash = contract
        .give_back_assets_after_trade(&conn, signer.clone(), assets.clone(), amounts.clone())
        .await?;
    let _tx_receipt = wait_for_confirmations(
        &conn.eth(),
        tx_hash,
        poll_interval,
        max_retry,
        confirmations,
    )
    .await?;

    Ok(tx_hash)
}

pub async fn transfer_ownership_and_ensure_success(
    contract: StrategyPoolContract<EitherTransport>,
    conn: &EthereumRpcConnection,
    confirmations: u64,
    max_retry: u64,
    poll_interval: Duration,
    signer: impl Key + Clone,
    new_owner: Address,
) -> Result<H256> {
    /* publish transaction */
    let tx_hash = contract
        .transfer_ownership(&conn, signer.clone(), new_owner)
        .await?;
    let _tx_receipt = wait_for_confirmations(
        &conn.eth(),
        tx_hash,
        poll_interval,
        max_retry,
        confirmations,
    )
    .await?;

    Ok(tx_hash)
}

#[derive(Debug, Clone)]
pub struct StrategyPoolWithdrawEvent {
    pub receiver: Address,
    pub strategy_pool_assets: Vec<Address>,
    pub strategy_pool_asset_amounts: Vec<U256>,
}

pub fn parse_strategy_pool_withdraw_event(
    strategy_pool_address: Address,
    receipt: TransactionReceipt,
) -> Result<StrategyPoolWithdrawEvent> {
    let strategy_pool = web3::ethabi::Contract::load(POOL_ABI_JSON.as_bytes())?;
    let withdraw_event = strategy_pool
        .event("Withdraw")
        .context("Failed to get Withdraw event from strategy pool")?;

    for log in receipt.logs {
        /* there can only be 4 indexed (topic) values in a event log */
        /* 1st topic is always the hash of the event signature */
        if log.topics[0] == withdraw_event.signature()
						/* address of the contract that fired the event */
						&& log.address == strategy_pool_address
        {
            /* 2nd topic is receiver of the assets, should be backer address */
            /* topics have 32 bytes, so we must fetch the last 20 bytes for an address */
            let receiver_bytes = log.topics[1].as_bytes();
            if receiver_bytes.len() < 32 {
                return Err(eyre!("invalid topic length"));
            }
            let receiver = Address::from_slice(&receiver_bytes[12..]);

            /* instantiate an ethabi::Log from raw log to enable access to non indexed data */
            let parsed_log = withdraw_event.parse_log(web3::ethabi::RawLog {
                topics: log.topics.clone(),
                data: log.data.0.clone(),
            })?;

            /* parse non indexed event data from event log */
            /* ethabi::Log params ignore the first topic, so params[0] is not the event signature */
            let strategy_pool_assets = parsed_log.params[1]
                .value
                .clone()
                .into_array()
                .context("could not parse assets from event log")?
                .into_iter()
                .map(|val| {
                    val.into_address()
                        .ok_or_else(|| eyre!("could not parse asset address from event log"))
                })
                .collect::<Result<Vec<Address>, _>>()?;
            let strategy_pool_asset_amounts = parsed_log.params[2]
                .value
                .clone()
                .into_array()
                .context("could not parse amounts array from event log")?
                .into_iter()
                .map(|val| {
                    val.into_uint()
                        .ok_or_else(|| eyre!("could not parse amount from event log"))
                })
                .collect::<Result<Vec<U256>, _>>()?;

            return Ok(StrategyPoolWithdrawEvent {
                receiver,
                strategy_pool_assets,
                strategy_pool_asset_amounts,
            });
        }
    }
    Err(eyre!("could not find withdraw event in receipt"))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use once_cell::sync::Lazy;
    use tokio::sync::Mutex;

    use super::*;
    use crate::contract_wrappers::mock_erc20::deploy_mock_erc20;
    use crate::signer::Secp256k1SecretKey;
    use crate::{
        wait_for_confirmations_simple, EthereumRpcConnectionGuard, EthereumRpcConnectionPool,
        ANVIL_PRIV_KEY_1, ANVIL_PRIV_KEY_2,
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
    async fn test_decimals() -> Result<()> {
        let tx_conn_wrapper = get_tx_conn().await?;
        let mut tx_conn_guard = tx_conn_wrapper.lock().await;
        let tx_conn = tx_conn_guard.as_mut().unwrap();

        let god_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;

        let fake_strategy_pool_herald_address = god_key.address();
        let logger = DynLogger::new(Arc::new(move |msg| {
            println!("{}", msg);
        }));
        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
            fake_strategy_pool_herald_address,
            logger,
        )
        .await?;

        assert_eq!(strategy_pool.decimals().await?, U256::from(18));
        Ok(())
    }

    #[tokio::test]
    async fn test_total_supply() -> Result<()> {
        let tx_conn_wrapper = get_tx_conn().await?;
        let mut tx_conn_guard = tx_conn_wrapper.lock().await;
        let tx_conn = tx_conn_guard.as_mut().unwrap();

        let god_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;

        let mock_erc20_a = deploy_mock_erc20(tx_conn.clone(), god_key.clone()).await?;
        let alice = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;

        let fake_strategy_pool_herald_address = god_key.address();
        let logger = DynLogger::new(Arc::new(move |msg| {
            println!("{}", msg);
        }));
        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
            fake_strategy_pool_herald_address,
            logger.clone(),
        )
        .await?;

        assert_eq!(strategy_pool.total_supply().await?, U256::zero());

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(
                    &tx_conn,
                    god_key.clone(),
                    god_key.address(),
                    U256::from(100),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(
                    &tx_conn,
                    god_key.clone(),
                    strategy_pool.address(),
                    U256::from(200),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        let assets = vec![mock_erc20_a.address];
        let amounts = vec![U256::from(100)];

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            strategy_pool
                .deposit(
                    &tx_conn,
                    god_key,
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(strategy_pool.total_supply().await?, U256::from(1));

        Ok(())
    }

    #[tokio::test]
    async fn test_balance_of() -> Result<()> {
        let tx_conn_wrapper = get_tx_conn().await?;
        let mut tx_conn_guard = tx_conn_wrapper.lock().await;
        let tx_conn = tx_conn_guard.as_mut().unwrap();

        let god_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;

        let mock_erc20_a = deploy_mock_erc20(tx_conn.clone(), god_key.clone()).await?;
        let alice = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;

        let fake_strategy_pool_herald_address = god_key.address();
        let logger = DynLogger::new(Arc::new(move |msg| {
            println!("{}", msg);
        }));
        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
            fake_strategy_pool_herald_address,
            logger.clone(),
        )
        .await?;

        assert_eq!(strategy_pool.balance_of(alice.address).await?, U256::zero());

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(
                    &tx_conn,
                    god_key.clone(),
                    god_key.address(),
                    U256::from(100),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(
                    &tx_conn,
                    god_key.clone(),
                    strategy_pool.address(),
                    U256::from(200),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        let assets = vec![mock_erc20_a.address];
        let amounts = vec![U256::from(100)];

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            strategy_pool
                .deposit(
                    &tx_conn,
                    god_key,
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(
            strategy_pool.balance_of(alice.address).await?,
            U256::from(1)
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_owner() -> Result<()> {
        let tx_conn_wrapper = get_tx_conn().await?;
        let mut tx_conn_guard = tx_conn_wrapper.lock().await;
        let tx_conn = tx_conn_guard.as_mut().unwrap();

        let god_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;

        let fake_strategy_pool_herald_address = god_key.address();
        let logger = DynLogger::new(Arc::new(move |msg| {
            println!("{}", msg);
        }));
        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
            fake_strategy_pool_herald_address,
            logger,
        )
        .await?;

        assert_eq!(strategy_pool.owner().await?, god_key.address());
        Ok(())
    }

    #[tokio::test]
    async fn test_transfer_ownership() -> Result<()> {
        let tx_conn_wrapper = get_tx_conn().await?;
        let mut tx_conn_guard = tx_conn_wrapper.lock().await;
        let tx_conn = tx_conn_guard.as_mut().unwrap();

        let god_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;

        let alice = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;

        let fake_strategy_pool_herald_address = god_key.address();
        let logger = DynLogger::new(Arc::new(move |msg| {
            println!("{}", msg);
        }));
        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
            fake_strategy_pool_herald_address,
            logger,
        )
        .await?;

        assert_eq!(strategy_pool.owner().await?, god_key.address());

        wait_for_confirmations_simple(
            &tx_conn.eth(),
            strategy_pool
                .transfer_ownership(&tx_conn, god_key.clone(), alice.address())
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(strategy_pool.owner().await?, alice.address());
        Ok(())
    }

    #[tokio::test]
    async fn test_assets() -> Result<()> {
        let tx_conn_wrapper = get_tx_conn().await?;
        let mut tx_conn_guard = tx_conn_wrapper.lock().await;
        let tx_conn = tx_conn_guard.as_mut().unwrap();

        let god_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;

        let mock_erc20_a = deploy_mock_erc20(tx_conn.clone(), god_key.clone()).await?;
        let alice = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;

        let fake_strategy_pool_herald_address = god_key.address();
        let logger = DynLogger::new(Arc::new(move |msg| {
            println!("{}", msg);
        }));
        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
            fake_strategy_pool_herald_address,
            logger.clone(),
        )
        .await?;

        assert_eq!(strategy_pool.assets().await?, vec![]);

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(
                    &tx_conn,
                    god_key.clone(),
                    god_key.address(),
                    U256::from(100),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(
                    &tx_conn,
                    god_key.clone(),
                    strategy_pool.address(),
                    U256::from(200),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        let assets = vec![mock_erc20_a.address];
        let amounts = vec![U256::from(100)];

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            strategy_pool
                .deposit(
                    &tx_conn,
                    god_key,
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(strategy_pool.assets().await?, assets);

        Ok(())
    }

    #[tokio::test]
    async fn test_asset_balance() -> Result<()> {
        let tx_conn_wrapper = get_tx_conn().await?;
        let mut tx_conn_guard = tx_conn_wrapper.lock().await;
        let tx_conn = tx_conn_guard.as_mut().unwrap();

        let god_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;

        let mock_erc20_a = deploy_mock_erc20(tx_conn.clone(), god_key.clone()).await?;
        let alice = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;

        let fake_strategy_pool_herald_address = god_key.address();
        let logger = DynLogger::new(Arc::new(move |msg| {
            println!("{}", msg);
        }));
        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
            fake_strategy_pool_herald_address,
            logger.clone(),
        )
        .await?;

        assert_eq!(
            strategy_pool.asset_balance(mock_erc20_a.address).await?,
            U256::zero()
        );

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(
                    &tx_conn,
                    god_key.clone(),
                    god_key.address(),
                    U256::from(100),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(
                    &tx_conn,
                    god_key.clone(),
                    strategy_pool.address(),
                    U256::from(200),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        let assets = vec![mock_erc20_a.address];
        let amounts = vec![U256::from(100)];

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            strategy_pool
                .deposit(
                    &tx_conn,
                    god_key,
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(
            strategy_pool.asset_balance(mock_erc20_a.address).await?,
            U256::from(100)
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_assets_and_balances() -> Result<()> {
        let tx_conn_wrapper = get_tx_conn().await?;
        let mut tx_conn_guard = tx_conn_wrapper.lock().await;
        let tx_conn = tx_conn_guard.as_mut().unwrap();

        let god_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;

        let mock_erc20_a = deploy_mock_erc20(tx_conn.clone(), god_key.clone()).await?;
        let alice = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;

        let fake_strategy_pool_herald_address = god_key.address();
        let logger = DynLogger::new(Arc::new(move |msg| {
            println!("{}", msg);
        }));
        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
            fake_strategy_pool_herald_address,
            logger.clone(),
        )
        .await?;

        assert_eq!(strategy_pool.assets_and_balances().await?, (vec![], vec![]));

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(
                    &tx_conn,
                    god_key.clone(),
                    god_key.address(),
                    U256::from(100),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(
                    &tx_conn,
                    god_key.clone(),
                    strategy_pool.address(),
                    U256::from(200),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        let assets = vec![mock_erc20_a.address];
        let amounts = vec![U256::from(100)];

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            strategy_pool
                .deposit(
                    &tx_conn,
                    god_key,
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(
            strategy_pool.assets_and_balances().await?,
            (vec![mock_erc20_a.address], vec![U256::from(100)])
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_max_mint() -> Result<()> {
        let tx_conn_wrapper = get_tx_conn().await?;
        let mut tx_conn_guard = tx_conn_wrapper.lock().await;
        let tx_conn = tx_conn_guard.as_mut().unwrap();

        let god_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;

        let mock_erc20_a = deploy_mock_erc20(tx_conn.clone(), god_key.clone()).await?;
        let alice = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;

        let fake_strategy_pool_herald_address = god_key.address();
        let logger = DynLogger::new(Arc::new(move |msg| {
            println!("{}", msg);
        }));
        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
            fake_strategy_pool_herald_address,
            logger.clone(),
        )
        .await?;

        assert_eq!(strategy_pool.max_mint().await?, U256::max_value());

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(
                    &tx_conn,
                    god_key.clone(),
                    god_key.address(),
                    U256::from(100),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(
                    &tx_conn,
                    god_key.clone(),
                    strategy_pool.address(),
                    U256::from(200),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        let assets = vec![mock_erc20_a.address];
        let amounts = vec![U256::from(100)];

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            strategy_pool
                .deposit(
                    &tx_conn,
                    god_key,
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(
            strategy_pool.max_mint().await?,
            U256::max_value() - U256::from(1)
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_deposit() -> Result<()> {
        let tx_conn_wrapper = get_tx_conn().await?;
        let mut tx_conn_guard = tx_conn_wrapper.lock().await;
        let tx_conn = tx_conn_guard.as_mut().unwrap();

        let god_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;

        let mock_erc20_a = deploy_mock_erc20(tx_conn.clone(), god_key.clone()).await?;
        let mock_erc20_b = deploy_mock_erc20(tx_conn.clone(), god_key.clone()).await?;

        let alice = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;

        let fake_strategy_pool_herald_address = god_key.address();
        let logger = DynLogger::new(Arc::new(move |msg| {
            println!("{}", msg);
        }));
        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
            fake_strategy_pool_herald_address,
            logger.clone(),
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(
                    &tx_conn,
                    god_key.clone(),
                    god_key.address(),
                    U256::from(100),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_b
                .mint(
                    &tx_conn,
                    god_key.clone(),
                    god_key.address(),
                    U256::from(200),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(
                    &tx_conn,
                    god_key.clone(),
                    strategy_pool.address(),
                    U256::from(100),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_b
                .approve(
                    &tx_conn,
                    god_key.clone(),
                    strategy_pool.address(),
                    U256::from(200),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(
            mock_erc20_a.balance_of(god_key.address()).await?,
            U256::from(100)
        );
        assert_eq!(
            mock_erc20_b.balance_of(god_key.address()).await?,
            U256::from(200)
        );
        assert_eq!(strategy_pool.balance_of(alice.address).await?, U256::zero());

        let assets = vec![mock_erc20_a.address, mock_erc20_b.address];
        let amounts = vec![U256::from(100), U256::from(200)];

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            strategy_pool
                .deposit(
                    &tx_conn,
                    god_key.clone(),
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(
            mock_erc20_a.balance_of(god_key.address()).await?,
            U256::zero()
        );
        assert_eq!(
            mock_erc20_b.balance_of(god_key.address()).await?,
            U256::zero()
        );
        assert_eq!(strategy_pool.assets().await?, assets.clone());
        assert_eq!(
            strategy_pool.asset_balance(mock_erc20_a.address).await?,
            U256::from(100)
        );
        assert_eq!(
            strategy_pool.asset_balance(mock_erc20_b.address).await?,
            U256::from(200)
        );
        assert_eq!(
            strategy_pool.assets_and_balances().await?,
            (assets.clone(), amounts.clone())
        );
        assert_eq!(
            strategy_pool.balance_of(alice.address).await?,
            U256::from(1)
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_max_redeem() -> Result<()> {
        let tx_conn_wrapper = get_tx_conn().await?;
        let mut tx_conn_guard = tx_conn_wrapper.lock().await;
        let tx_conn = tx_conn_guard.as_mut().unwrap();

        let god_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;

        let mock_erc20_a = deploy_mock_erc20(tx_conn.clone(), god_key.clone()).await?;
        let alice = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;

        let fake_strategy_pool_herald_address = god_key.address();
        let logger = DynLogger::new(Arc::new(move |msg| {
            println!("{}", msg);
        }));
        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
            fake_strategy_pool_herald_address,
            logger.clone(),
        )
        .await?;

        assert_eq!(strategy_pool.max_redeem(alice.address).await?, U256::zero());

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(
                    &tx_conn,
                    god_key.clone(),
                    god_key.address(),
                    U256::from(100),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(
                    &tx_conn,
                    god_key.clone(),
                    strategy_pool.address(),
                    U256::from(200),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        let assets = vec![mock_erc20_a.address];
        let amounts = vec![U256::from(100)];

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            strategy_pool
                .deposit(
                    &tx_conn,
                    god_key,
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(
            strategy_pool.max_redeem(alice.address).await?,
            U256::from(1)
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_redeem() -> Result<()> {
        let tx_conn_wrapper = get_tx_conn().await?;
        let mut tx_conn_guard = tx_conn_wrapper.lock().await;
        let tx_conn = tx_conn_guard.as_mut().unwrap();

        let god_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;

        let mock_erc20_a = deploy_mock_erc20(tx_conn.clone(), god_key.clone()).await?;
        let alice = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;

        let fake_strategy_pool_herald_address = god_key.address();
        let logger = DynLogger::new(Arc::new(move |msg| {
            println!("{}", msg);
        }));
        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
            fake_strategy_pool_herald_address,
            logger.clone(),
        )
        .await?;

        assert!(matches!(
            strategy_pool
                .redeem(
                    &tx_conn,
                    alice.clone(),
                    U256::zero(),
                    alice.address,
                    alice.address
                )
                .await,
            Err(_)
        ));
        assert!(matches!(
            strategy_pool
                .redeem(
                    &tx_conn,
                    alice.clone(),
                    U256::from(1),
                    alice.address,
                    alice.address
                )
                .await,
            Err(_)
        ));

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(
                    &tx_conn,
                    god_key.clone(),
                    god_key.address(),
                    U256::from(100),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(
                    &tx_conn,
                    god_key.clone(),
                    strategy_pool.address(),
                    U256::from(200),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(
            mock_erc20_a.balance_of(god_key.address()).await?,
            U256::from(100)
        );
        assert_eq!(
            strategy_pool.asset_balance(mock_erc20_a.address).await?,
            U256::zero()
        );
        assert_eq!(strategy_pool.balance_of(alice.address).await?, U256::zero());
        assert_eq!(strategy_pool.total_supply().await?, U256::zero());

        let assets = vec![mock_erc20_a.address];
        let amounts = vec![U256::from(100)];

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            strategy_pool
                .deposit(
                    &tx_conn,
                    god_key.clone(),
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(
            mock_erc20_a.balance_of(god_key.address()).await?,
            U256::zero()
        );
        assert_eq!(
            strategy_pool.asset_balance(mock_erc20_a.address).await?,
            U256::from(100)
        );
        assert_eq!(
            strategy_pool.balance_of(alice.address).await?,
            U256::from(1)
        );
        assert_eq!(strategy_pool.total_supply().await?, U256::from(1));

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            strategy_pool
                .redeem(
                    &tx_conn,
                    alice.clone(),
                    U256::from(1),
                    alice.address,
                    alice.address,
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(
            mock_erc20_a.balance_of(god_key.address()).await?,
            U256::zero()
        );
        assert_eq!(
            strategy_pool.asset_balance(mock_erc20_a.address).await?,
            U256::from(100)
        );
        assert_eq!(strategy_pool.balance_of(alice.address).await?, U256::zero());
        assert_eq!(strategy_pool.total_supply().await?, U256::zero());
        assert_eq!(
            mock_erc20_a.balance_of(alice.address()).await?,
            U256::zero()
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_acquire_asset_before_trade() -> Result<()> {
        let tx_conn_wrapper = get_tx_conn().await?;
        let mut tx_conn_guard = tx_conn_wrapper.lock().await;
        let tx_conn = tx_conn_guard.as_mut().unwrap();

        let god_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;

        let mock_erc20_a = deploy_mock_erc20(tx_conn.clone(), god_key.clone()).await?;
        let alice = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;

        let fake_strategy_pool_herald_address = god_key.address();
        let logger = DynLogger::new(Arc::new(move |msg| {
            println!("{}", msg);
        }));
        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
            fake_strategy_pool_herald_address,
            logger.clone(),
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(
                    &tx_conn,
                    god_key.clone(),
                    god_key.address(),
                    U256::from(100),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(
                    &tx_conn,
                    god_key.clone(),
                    strategy_pool.address(),
                    U256::from(100),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(
            mock_erc20_a.balance_of(god_key.address()).await?,
            U256::from(100)
        );
        assert_eq!(
            strategy_pool.asset_balance(mock_erc20_a.address).await?,
            U256::zero()
        );
        assert_eq!(strategy_pool.balance_of(alice.address).await?, U256::zero());
        assert_eq!(strategy_pool.total_supply().await?, U256::zero());

        let assets = vec![mock_erc20_a.address];
        let amounts = vec![U256::from(100)];

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            strategy_pool
                .deposit(
                    &tx_conn,
                    god_key.clone(),
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(
            mock_erc20_a.balance_of(god_key.address()).await?,
            U256::zero()
        );
        assert_eq!(
            strategy_pool.asset_balance(mock_erc20_a.address).await?,
            U256::from(100)
        );
        assert_eq!(
            strategy_pool.balance_of(alice.address).await?,
            U256::from(1)
        );
        assert_eq!(strategy_pool.total_supply().await?, U256::from(1));
        assert_eq!(
            mock_erc20_a.balance_of(alice.address()).await?,
            U256::zero()
        );

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            strategy_pool
                .acquire_asset_before_trade(
                    &tx_conn,
                    god_key.clone(),
                    mock_erc20_a.address,
                    U256::from(100),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(
            mock_erc20_a.balance_of(god_key.address()).await?,
            U256::from(100)
        );
        // can't query assets, SP is trading
        assert!(matches!(
            strategy_pool.asset_balance(mock_erc20_a.address).await, // = 0
            Err(_)
        ));
        assert_eq!(
            strategy_pool.balance_of(alice.address).await?,
            U256::from(1)
        );
        assert_eq!(strategy_pool.total_supply().await?, U256::from(1));
        assert_eq!(
            mock_erc20_a.balance_of(alice.address()).await?,
            U256::zero()
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_give_back_assets_after_trade() -> Result<()> {
        let tx_conn_wrapper = get_tx_conn().await?;
        let mut tx_conn_guard = tx_conn_wrapper.lock().await;
        let tx_conn = tx_conn_guard.as_mut().unwrap();

        let god_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;

        let mock_erc20_a = deploy_mock_erc20(tx_conn.clone(), god_key.clone()).await?;
        let alice = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;

        let fake_strategy_pool_herald_address = god_key.address();
        let logger = DynLogger::new(Arc::new(move |msg| {
            println!("{}", msg);
        }));
        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
            fake_strategy_pool_herald_address,
            logger.clone(),
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(
                    &tx_conn,
                    god_key.clone(),
                    god_key.address(),
                    U256::from(100),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(
                    &tx_conn,
                    god_key.clone(),
                    strategy_pool.address(),
                    U256::from(100),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(
            mock_erc20_a.balance_of(god_key.address()).await?,
            U256::from(100)
        );
        assert_eq!(
            strategy_pool.asset_balance(mock_erc20_a.address).await?,
            U256::zero()
        );
        assert_eq!(strategy_pool.balance_of(alice.address).await?, U256::zero());
        assert_eq!(strategy_pool.total_supply().await?, U256::zero());

        let assets = vec![mock_erc20_a.address];
        let amounts = vec![U256::from(100)];

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            strategy_pool
                .deposit(
                    &tx_conn,
                    god_key.clone(),
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(
            mock_erc20_a.balance_of(god_key.address()).await?,
            U256::zero()
        );
        assert_eq!(
            strategy_pool.asset_balance(mock_erc20_a.address).await?,
            U256::from(100)
        );
        assert_eq!(
            strategy_pool.balance_of(alice.address).await?,
            U256::from(1)
        );
        assert_eq!(strategy_pool.total_supply().await?, U256::from(1));
        assert_eq!(
            mock_erc20_a.balance_of(alice.address()).await?,
            U256::zero()
        );

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            strategy_pool
                .acquire_asset_before_trade(
                    &tx_conn,
                    god_key.clone(),
                    mock_erc20_a.address,
                    U256::from(100),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(
            mock_erc20_a.balance_of(god_key.address()).await?,
            U256::from(100)
        );
        // can't query assets, SP is trading
        assert!(matches!(
            strategy_pool.asset_balance(mock_erc20_a.address).await, // = 0
            Err(_)
        ));
        assert_eq!(
            strategy_pool.balance_of(alice.address).await?,
            U256::from(1)
        );
        assert_eq!(strategy_pool.total_supply().await?, U256::from(1));
        assert_eq!(
            mock_erc20_a.balance_of(alice.address()).await?,
            U256::zero()
        );

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(
                    &tx_conn,
                    god_key.clone(),
                    strategy_pool.address(),
                    U256::from(100),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            strategy_pool
                .give_back_assets_after_trade(
                    &tx_conn,
                    god_key.clone(),
                    vec![mock_erc20_a.address],
                    vec![U256::from(100)],
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(
            mock_erc20_a.balance_of(god_key.address()).await?,
            U256::zero()
        );
        assert_eq!(
            strategy_pool.asset_balance(mock_erc20_a.address).await?,
            U256::from(100)
        );
        assert_eq!(
            strategy_pool.balance_of(alice.address).await?,
            U256::from(1)
        );
        assert_eq!(strategy_pool.total_supply().await?, U256::from(1));
        assert_eq!(
            mock_erc20_a.balance_of(alice.address()).await?,
            U256::zero()
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_is_paused() -> Result<()> {
        let tx_conn_wrapper = get_tx_conn().await?;
        let mut tx_conn_guard = tx_conn_wrapper.lock().await;
        let tx_conn = tx_conn_guard.as_mut().unwrap();

        let god_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;

        let mock_erc20_a = deploy_mock_erc20(tx_conn.clone(), god_key.clone()).await?;
        let alice = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;

        let fake_strategy_pool_herald_address = god_key.address();
        let logger = DynLogger::new(Arc::new(move |msg| {
            println!("{}", msg);
        }));
        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
            fake_strategy_pool_herald_address,
            logger.clone(),
        )
        .await?;

        assert_eq!(strategy_pool.is_paused().await?, false);

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(
                    &tx_conn,
                    god_key.clone(),
                    god_key.address(),
                    U256::from(100),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(
                    &tx_conn,
                    god_key.clone(),
                    strategy_pool.address(),
                    U256::from(100),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        let assets = vec![mock_erc20_a.address];
        let amounts = vec![U256::from(100)];

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            strategy_pool
                .deposit(
                    &tx_conn,
                    god_key.clone(),
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(strategy_pool.is_paused().await?, false);

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            strategy_pool
                .acquire_asset_before_trade(
                    &tx_conn,
                    god_key.clone(),
                    mock_erc20_a.address,
                    U256::from(100),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(strategy_pool.is_paused().await?, true);

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(
                    &tx_conn,
                    god_key.clone(),
                    strategy_pool.address(),
                    U256::from(100),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            strategy_pool
                .give_back_assets_after_trade(
                    &tx_conn,
                    god_key.clone(),
                    vec![mock_erc20_a.address],
                    vec![U256::from(100)],
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(strategy_pool.is_paused().await?, false);

        Ok(())
    }
}
