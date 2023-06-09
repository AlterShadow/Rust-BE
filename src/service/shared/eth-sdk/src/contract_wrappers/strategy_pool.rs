use eyre::*;
use tracing::info;

use web3::contract::{Contract, Options};
use web3::signing::Key;
use web3::types::{Address, H256, U256};
use web3::{ethabi, Transport, Web3};

use crate::contract::AbstractContract;
use crate::{deploy_contract, EitherTransport, EthereumRpcConnectionPool, MultiChainAddressTable};
use gen::model::EnumBlockChain;

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
    // only for testing
    pub async fn deploy(w3: Web3<T>, key: impl Key, name: String, symbol: String) -> Result<Self> {
        let params = (name.clone(), symbol.clone(), key.address());
        let contract = deploy_contract(w3.clone(), key, params, "StrategyPool").await?;
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

    pub async fn max_withdraw(&self, owner: Address) -> Result<(Vec<Address>, Vec<U256>)> {
        Ok(self
            .contract
            .query(
                StrategyPoolFunctions::MaxWithdraw.as_str(),
                owner,
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn max_deposit(&self) -> Result<(Vec<Address>, Vec<U256>)> {
        Ok(self
            .contract
            .query(
                StrategyPoolFunctions::MaxDeposit.as_str(),
                (),
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn min_deposit(&self) -> Result<(Vec<Address>, Vec<U256>)> {
        Ok(self
            .contract
            .query(
                StrategyPoolFunctions::MinDeposit.as_str(),
                (),
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn deposit(
        &self,
        signer: impl Key,
        assets: Vec<Address>,
        amounts: Vec<U256>,
        shares: U256,
        receiver: Address,
    ) -> Result<H256> {
        let estimated_gas = self
            .contract
            .estimate_gas(
                StrategyPoolFunctions::Deposit.as_str(),
                (assets.clone(), amounts.clone(), shares, receiver),
                signer.address(),
                Options::default(),
            )
            .await?;

        info!("Depositing amounts {:?} of assets {:?} to mint {:?} shares to receiver {:?} to strategy pool contract {:?} by {:?}",
							amounts.clone(),
							assets.clone(),
							shares,
							receiver,
							self.address(),
							signer.address(),
				);

        Ok(self
            .contract
            .signed_call(
                StrategyPoolFunctions::Deposit.as_str(),
                (assets, amounts, shares, receiver),
                Options::with(|options| options.gas = Some(estimated_gas)),
                signer,
            )
            .await?)
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

    pub async fn min_redeem(&self) -> Result<U256> {
        Ok(self
            .contract
            .query(
                StrategyPoolFunctions::MinRedeem.as_str(),
                (),
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn preview_redeem(&self, shares: U256) -> Result<(Vec<Address>, Vec<U256>)> {
        Ok(self
            .contract
            .query(
                StrategyPoolFunctions::PreviewRedeem.as_str(),
                shares,
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn redeem(
        &self,
        signer: impl Key,
        shares: U256,
        receiver: Address,
        owner: Address,
    ) -> Result<H256> {
        let estimated_gas = self
            .contract
            .estimate_gas(
                StrategyPoolFunctions::Redeem.as_str(),
                (shares, receiver, owner),
                signer.address(),
                Options::default(),
            )
            .await?;

        info!("Redeeming {:?} shares to receiver {:?} from owner {:?} from strategy pool contract {:?} by {:?}",
							shares,
							receiver,
							owner,
							self.address(),
							signer.address(),
				);

        Ok(self
            .contract
            .signed_call(
                StrategyPoolFunctions::Redeem.as_str(),
                (shares, receiver, owner),
                Options::with(|options| options.gas = Some(estimated_gas)),
                signer,
            )
            .await?)
    }

    pub async fn acquire_asset_before_trade(
        &self,
        signer: impl Key,
        asset: Address,
        amount: U256,
    ) -> Result<H256> {
        let estimated_gas = self
            .contract
            .estimate_gas(
                StrategyPoolFunctions::AcquireAssetBeforeTrade.as_str(),
                (asset, amount),
                signer.address(),
                Options::default(),
            )
            .await?;

        info!(
            "Acquiring {:?} amount of asset {:?} before trade from strategy pool contract {:?} by {:?}",
						amount,
            asset,
            self.address(),
						signer.address(),
        );

        Ok(self
            .contract
            .signed_call(
                StrategyPoolFunctions::AcquireAssetBeforeTrade.as_str(),
                (asset, amount),
                Options::with(|options| options.gas = Some(estimated_gas)),
                signer,
            )
            .await?)
    }

    pub async fn give_back_assets_after_trade(
        &self,
        signer: impl Key,
        assets: Vec<Address>,
        amounts: Vec<U256>,
    ) -> Result<H256> {
        let estimated_gas = self
            .contract
            .estimate_gas(
                StrategyPoolFunctions::GiveBackAssetsAfterTrade.as_str(),
                (assets.clone(), amounts.clone()),
                signer.address(),
                Options::default(),
            )
            .await?;

        info!(
						"Giving back {:?} amounts of assets {:?} after trade to strategy pool contract {:?} by {:?}",
						amounts.clone(),
						assets.clone(),
						self.address(),
						signer.address(),
				);

        Ok(self
            .contract
            .signed_call(
                StrategyPoolFunctions::GiveBackAssetsAfterTrade.as_str(),
                (assets, amounts),
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
        let estimated_gas = self
            .contract
            .estimate_gas(
                StrategyPoolFunctions::TransferOwnership.as_str(),
                new_owner,
                by,
                Options::default(),
            )
            .await?;

        info!(
            "Transferring strategy pool contract {:?} ownership from {:?} to {:?} by {:?}",
            self.address(),
            self.owner().await?,
            new_owner,
            signer.address(),
        );

        Ok(self
            .contract
            .signed_call(
                StrategyPoolFunctions::TransferOwnership.as_str(),
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
    MaxWithdraw,
    MaxDeposit,
    MinDeposit,
    Deposit,
    MaxRedeem,
    MinRedeem,
    PreviewRedeem,
    Redeem,
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
            Self::MaxWithdraw => "maxWithdraw",
            Self::MaxDeposit => "maxDeposit",
            Self::MinDeposit => "minDeposit",
            Self::Deposit => "deposit",
            Self::MaxRedeem => "maxRedeem",
            Self::MinRedeem => "minRedeem",
            Self::PreviewRedeem => "previewRedeem",
            Self::Redeem => "redeem",
            Self::AcquireAssetBeforeTrade => "acquireAssetBeforeTrade",
            Self::GiveBackAssetsAfterTrade => "giveBackAssetsAfterTrade",
            Self::TransferOwnership => "transferOwnership",
            Self::Owner => "owner",
            Self::IsPaused => "paused",
        }
    }
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

        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
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

        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
        )
        .await?;

        assert_eq!(strategy_pool.total_supply().await?, U256::zero());

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(god_key.clone(), god_key.address(), U256::from(100))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(god_key.clone(), strategy_pool.address(), U256::from(200))
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
                    god_key,
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
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

        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
        )
        .await?;

        assert_eq!(strategy_pool.balance_of(alice.address).await?, U256::zero());

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(god_key.clone(), god_key.address(), U256::from(100))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(god_key.clone(), strategy_pool.address(), U256::from(200))
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
                    god_key,
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
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

        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
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

        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
        )
        .await?;

        assert_eq!(strategy_pool.owner().await?, god_key.address());

        wait_for_confirmations_simple(
            &tx_conn.eth(),
            strategy_pool
                .transfer_ownership(god_key.clone(), god_key.address(), alice.address())
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

        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
        )
        .await?;

        assert_eq!(strategy_pool.assets().await?, vec![]);

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(god_key.clone(), god_key.address(), U256::from(100))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(god_key.clone(), strategy_pool.address(), U256::from(200))
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
                    god_key,
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
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

        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
        )
        .await?;

        assert_eq!(
            strategy_pool.asset_balance(mock_erc20_a.address).await?,
            U256::zero()
        );

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(god_key.clone(), god_key.address(), U256::from(100))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(god_key.clone(), strategy_pool.address(), U256::from(200))
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
                    god_key,
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
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

        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
        )
        .await?;

        assert_eq!(strategy_pool.assets_and_balances().await?, (vec![], vec![]));

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(god_key.clone(), god_key.address(), U256::from(100))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(god_key.clone(), strategy_pool.address(), U256::from(200))
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
                    god_key,
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
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

        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
        )
        .await?;

        assert_eq!(strategy_pool.max_mint().await?, U256::max_value());

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(god_key.clone(), god_key.address(), U256::from(100))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(god_key.clone(), strategy_pool.address(), U256::from(200))
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
                    god_key,
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
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
    async fn test_max_withdraw() -> Result<()> {
        let tx_conn_wrapper = get_tx_conn().await?;
        let mut tx_conn_guard = tx_conn_wrapper.lock().await;
        let tx_conn = tx_conn_guard.as_mut().unwrap();

        let god_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;

        let mock_erc20_a = deploy_mock_erc20(tx_conn.clone(), god_key.clone()).await?;
        let alice = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;

        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
        )
        .await?;

        assert_eq!(
            strategy_pool.max_withdraw(alice.address).await?,
            (vec![], vec![])
        );

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(god_key.clone(), god_key.address(), U256::from(100))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(god_key.clone(), strategy_pool.address(), U256::from(200))
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
                    god_key,
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(
            strategy_pool.max_withdraw(alice.address).await?,
            (vec![mock_erc20_a.address], vec![U256::from(100)])
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_max_deposit() -> Result<()> {
        let tx_conn_wrapper = get_tx_conn().await?;
        let mut tx_conn_guard = tx_conn_wrapper.lock().await;
        let tx_conn = tx_conn_guard.as_mut().unwrap();

        let god_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;

        let mock_erc20_a = deploy_mock_erc20(tx_conn.clone(), god_key.clone()).await?;
        let alice = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;

        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
        )
        .await?;

        assert_eq!(strategy_pool.max_deposit().await?, (vec![], vec![]));

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(god_key.clone(), god_key.address(), U256::from(100))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(god_key.clone(), strategy_pool.address(), U256::from(200))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        let assets = vec![mock_erc20_a.address];
        let amounts = vec![U256::from(1)];

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            strategy_pool
                .deposit(
                    god_key,
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(
            strategy_pool.max_deposit().await?,
            (
                vec![mock_erc20_a.address],
                vec![U256::max_value() - U256::from(1)]
            )
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_min_deposit() -> Result<()> {
        let tx_conn_wrapper = get_tx_conn().await?;
        let mut tx_conn_guard = tx_conn_wrapper.lock().await;
        let tx_conn = tx_conn_guard.as_mut().unwrap();

        let god_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;

        let mock_erc20_a = deploy_mock_erc20(tx_conn.clone(), god_key.clone()).await?;
        let alice = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;

        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
        )
        .await?;

        assert_eq!(strategy_pool.min_deposit().await?, (vec![], vec![]));

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(god_key.clone(), god_key.address(), U256::from(100))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(god_key.clone(), strategy_pool.address(), U256::from(200))
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
                    god_key,
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(
            strategy_pool.min_deposit().await?,
            (vec![mock_erc20_a.address], vec![U256::from(100)])
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

        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(god_key.clone(), god_key.address(), U256::from(100))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_b
                .mint(god_key.clone(), god_key.address(), U256::from(200))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(god_key.clone(), strategy_pool.address(), U256::from(100))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_b
                .approve(god_key.clone(), strategy_pool.address(), U256::from(200))
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
                    god_key.clone(),
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
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

        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
        )
        .await?;

        assert_eq!(strategy_pool.max_redeem(alice.address).await?, U256::zero());

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(god_key.clone(), god_key.address(), U256::from(100))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(god_key.clone(), strategy_pool.address(), U256::from(200))
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
                    god_key,
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
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
    async fn test_min_redeem() -> Result<()> {
        let tx_conn_wrapper = get_tx_conn().await?;
        let mut tx_conn_guard = tx_conn_wrapper.lock().await;
        let tx_conn = tx_conn_guard.as_mut().unwrap();

        let god_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;

        let mock_erc20_a = deploy_mock_erc20(tx_conn.clone(), god_key.clone()).await?;
        let alice = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;

        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
        )
        .await?;

        assert_eq!(strategy_pool.min_redeem().await?, U256::zero());

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(god_key.clone(), god_key.address(), U256::from(100))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(god_key.clone(), strategy_pool.address(), U256::from(200))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        let assets = vec![mock_erc20_a.address];
        let amounts = vec![U256::from(1)];

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            strategy_pool
                .deposit(
                    god_key,
                    assets.clone(),
                    amounts.clone(),
                    U256::from(100),
                    alice.address(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(strategy_pool.min_redeem().await?, U256::from(100));

        Ok(())
    }

    #[tokio::test]
    async fn test_preview_redeem() -> Result<()> {
        let tx_conn_wrapper = get_tx_conn().await?;
        let mut tx_conn_guard = tx_conn_wrapper.lock().await;
        let tx_conn = tx_conn_guard.as_mut().unwrap();

        let god_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;

        let mock_erc20_a = deploy_mock_erc20(tx_conn.clone(), god_key.clone()).await?;
        let alice = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;

        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
        )
        .await?;

        assert_eq!(
            strategy_pool.preview_redeem(U256::zero()).await?,
            (vec![], vec![])
        );
        assert!(matches!(
            strategy_pool.preview_redeem(U256::from(1)).await,
            Err(_)
        ));

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(god_key.clone(), god_key.address(), U256::from(100))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(god_key.clone(), strategy_pool.address(), U256::from(200))
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
                    god_key,
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(
            strategy_pool.preview_redeem(U256::from(1)).await?,
            (vec![mock_erc20_a.address], vec![U256::from(100)])
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

        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
        )
        .await?;

        assert!(matches!(
            strategy_pool
                .redeem(alice.clone(), U256::zero(), alice.address, alice.address)
                .await,
            Err(_)
        ));
        assert!(matches!(
            strategy_pool
                .redeem(alice.clone(), U256::from(1), alice.address, alice.address)
                .await,
            Err(_)
        ));

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(god_key.clone(), god_key.address(), U256::from(100))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(god_key.clone(), strategy_pool.address(), U256::from(200))
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
                    god_key.clone(),
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
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
                .redeem(alice.clone(), U256::from(1), alice.address, alice.address)
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
            U256::zero()
        );
        assert_eq!(strategy_pool.balance_of(alice.address).await?, U256::zero());
        assert_eq!(strategy_pool.total_supply().await?, U256::zero());
        assert_eq!(
            mock_erc20_a.balance_of(alice.address()).await?,
            U256::from(100)
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

        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(god_key.clone(), god_key.address(), U256::from(100))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(god_key.clone(), strategy_pool.address(), U256::from(100))
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
                    god_key.clone(),
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
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
                .acquire_asset_before_trade(god_key.clone(), mock_erc20_a.address, U256::from(100))
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

        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(god_key.clone(), god_key.address(), U256::from(100))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(god_key.clone(), strategy_pool.address(), U256::from(100))
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
                    god_key.clone(),
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
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
                .acquire_asset_before_trade(god_key.clone(), mock_erc20_a.address, U256::from(100))
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
                .approve(god_key.clone(), strategy_pool.address(), U256::from(100))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            strategy_pool
                .give_back_assets_after_trade(
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

        let strategy_pool = StrategyPoolContract::deploy(
            tx_conn.clone(),
            god_key.clone(),
            "MockShare".to_string(),
            "MOCK".to_string(),
        )
        .await?;

        assert_eq!(strategy_pool.is_paused().await?, false);

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(god_key.clone(), god_key.address(), U256::from(100))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(god_key.clone(), strategy_pool.address(), U256::from(100))
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
                    god_key.clone(),
                    assets.clone(),
                    amounts.clone(),
                    U256::from(1),
                    alice.address(),
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
                .acquire_asset_before_trade(god_key.clone(), mock_erc20_a.address, U256::from(100))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(strategy_pool.is_paused().await?, true);

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .approve(god_key.clone(), strategy_pool.address(), U256::from(100))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            strategy_pool
                .give_back_assets_after_trade(
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
