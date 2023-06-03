use crate::dex_tracker::pancake::build_pancake_swap;
use crate::dex_tracker::PancakeSwap;
use crate::erc20::build_erc_20;
use crate::{
    ContractCall, DexAddresses, EthereumRpcConnectionPool, StableCoinAddresses, TransactionReady,
};
use bytes::Bytes;
use eyre::*;
use gen::database::{FunWatcherSaveRawTransactionReq, FunWatcherSaveWalletActivityHistoryReq};
use gen::model::{EnumBlockChain, EnumDex, EnumDexVersion};
use lib::database::DbClient;
use serde::{Deserialize, Serialize};
use tracing::error;
use web3::ethabi::Contract;
use web3::types::{H160, H256, U256};

pub struct AppState {
    pub dex_addresses: DexAddresses,
    pub eth_pool: EthereumRpcConnectionPool,
    pub pancake_swap: PancakeSwap,
    pub db: DbClient,
    pub stablecoin_addresses: StableCoinAddresses,
    pub erc_20: Contract,
}
impl AppState {
    pub fn new(db: DbClient, eth_pool: EthereumRpcConnectionPool) -> Result<Self> {
        Ok(Self {
            dex_addresses: DexAddresses::new(),
            eth_pool,
            pancake_swap: build_pancake_swap()?,
            db,
            stablecoin_addresses: StableCoinAddresses::default(),
            erc_20: build_erc_20()?,
        })
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DexPath {
    /* every path for every token_in token_out pair in every dex in every chain must be recorded in the database */
    /* so that we can trigger our own trades in the futures */
    /* note that reciprocals are different pairs with different paths */
    /* i.e. the path for token_in x and token_out y is different from token_in y and token_out x */
    PancakeV2(Vec<H160>),
    PancakeV3SingleHop(PancakeV3SingleHopPath),
    PancakeV3MultiHop(Vec<u8>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PancakeV3SingleHopPath {
    pub token_in: H160,
    pub token_out: H160,
    pub fee: U256,
}

#[derive(Clone, Debug)]
pub struct Trade {
    pub chain: EnumBlockChain,
    pub contract: H160,
    pub dex: EnumDex,
    pub token_in: H160,
    pub token_out: H160,
    pub caller: H160,
    pub amount_in: U256,
    pub amount_out: U256,
    /* some trades go through multiple swap calls because of pool availability */
    /* this means that for some pairs, we must keep track of all swap calls made in order and their paths */
    pub swap_calls: Vec<ContractCall>,
    pub paths: Vec<DexPath>,
    pub dex_versions: Vec<EnumDexVersion>,
}

impl Trade {
    pub fn get_pancake_pair_paths(&self) -> Result<PancakePairPaths> {
        if self.dex != EnumDex::PancakeSwap {
            bail!("dex is not pancakeswap")
        }
        let mut func_names_and_paths = Vec::new();
        for (i, swap_call) in self.swap_calls.iter().enumerate() {
            func_names_and_paths.push((swap_call.get_name(), self.paths[i].clone()));
        }
        Ok(PancakePairPaths::new(
            self.token_in,
            self.token_out,
            func_names_and_paths,
        )?)
    }
}

#[derive(Debug, Clone)]
pub struct PancakePairPaths {
    token_in: H160,
    token_out: H160,
    func_names_and_paths: Vec<(String, DexPath)>,
}

impl PancakePairPaths {
    pub fn new(
        token_in: H160,
        token_out: H160,
        func_names_and_paths: Vec<(String, DexPath)>,
    ) -> Result<Self> {
        if func_names_and_paths.len() == 0 {
            bail!("empty names and paths");
        }
        Ok(Self {
            token_in,
            token_out,
            func_names_and_paths: func_names_and_paths,
        })
    }

    pub fn get_token_in(&self) -> H160 {
        self.token_in
    }

    pub fn get_token_out(&self) -> H160 {
        self.token_out
    }

    pub fn len(&self) -> usize {
        self.func_names_and_paths.len()
    }

    pub fn get_func_name(&self, idx: usize) -> Result<String> {
        if idx >= self.len() {
            bail!("index out of bounds");
        }
        Ok(self.func_names_and_paths[idx].0.clone())
    }

    pub fn get_path(&self, idx: usize) -> Result<DexPath> {
        if idx >= self.len() {
            bail!("index out of bounds");
        }
        Ok(self.func_names_and_paths[idx].1.clone())
    }
}

pub fn parse_quickalert_payload(payload: Bytes) -> Result<Vec<H256>> {
    let result: Result<Vec<H256>, _> = serde_json::from_slice(&payload);

    match result {
        Ok(hashes) => Ok(hashes),
        Err(e) => Err(e.into()),
    }
}

pub async fn save_trade(hash: H256, trade: &Trade, db: &DbClient) -> Result<()> {
    if let Err(err) = async {
        db.execute(FunWatcherSaveWalletActivityHistoryReq {
            address: format!("{:?}", trade.caller),
            transaction_hash: format!("{:?}", hash),
            blockchain: EnumBlockChain::EthereumMainnet.to_string(),
            dex: trade.dex.to_string(),
            contract_address: format!("{:?}", trade.contract),
            token_in_address: format!("{:?}", trade.token_in),
            token_out_address: format!("{:?}", trade.token_out),
            caller_address: format!("{:?}", trade.caller),
            amount_in: format!("{:?}", trade.amount_in),
            amount_out: format!("{:?}", trade.amount_out),
            swap_calls: serde_json::to_value(&trade.swap_calls)?,
            paths: serde_json::to_value(&trade.paths)?,
            dex_versions: serde_json::to_value(&trade.dex_versions)?,
            // TODO: fetch block time
            created_at: None,
        })
        .await?;

        Ok::<_, Error>(())
    }
    .await
    {
        return Err(eyre!("failed to save trade: {:?}", err));
    }

    Ok(())
}
pub async fn cache_ethereum_transaction(
    tx: &TransactionReady,
    db: &DbClient,
    blockchain: EnumBlockChain,
) -> Result<()> {
    if let Err(err) = async {
        db.execute(FunWatcherSaveRawTransactionReq {
            transaction_hash: format!("{:?}", tx.get_hash()),
            chain: blockchain.to_string(),
            dex: None,
            raw_transaction: serde_json::to_string(tx.get_transaction()).context("transaction")?,
        })
        .await?;

        Ok::<_, Error>(())
    }
    .await
    {
        error!("failed to save raw transaction: {:?}", err);
    }

    Ok(())
}
