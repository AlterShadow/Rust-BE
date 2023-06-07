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
