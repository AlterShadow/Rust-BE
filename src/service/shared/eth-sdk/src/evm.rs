use crate::{PancakePairPathSet, TransactionReady};
use bytes::Bytes;
use eyre::*;
use gen::database::FunWatcherSaveRawTransactionReq;
use gen::model::{EnumBlockChain, EnumDex};
use lib::database::DbClient;
use serde::{Deserialize, Serialize};
use tracing::error;

use web3::types::{Address, H256, U256};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DexPairPathSet {
    PancakeSwap(PancakePairPathSet),
}

#[derive(Clone, Debug)]
pub struct DexTrade {
    pub chain: EnumBlockChain,
    pub contract: Address,
    pub dex: EnumDex,
    pub token_in: Address,
    pub token_out: Address,
    pub caller: Address,
    pub amount_in: U256,
    pub amount_out: U256,
    pub paths: DexPairPathSet,
}

impl DexTrade {
    pub fn get_pancake_pair_paths(&self) -> Result<PancakePairPathSet> {
        if self.dex != EnumDex::PancakeSwap {
            bail!("dex is not pancakeswap")
        }
        Ok(match self.paths {
            DexPairPathSet::PancakeSwap(ref paths) => paths.clone(),
        })
    }
}

pub fn parse_quickalert_payload(payload: Bytes) -> Result<Vec<H256>> {
    let result: Result<Vec<H256>, _> = serde_json::from_slice(&payload);

    match result {
        Ok(hashes) => Ok(hashes),
        Err(e) => Err(e.into()),
    }
}

pub async fn cache_ethereum_transaction(
    tx: &TransactionReady,
    db: &DbClient,
    blockchain: EnumBlockChain,
) -> Result<()> {
    if let Err(err) = async {
        db.execute(FunWatcherSaveRawTransactionReq {
            transaction_hash: tx.get_hash().into(),
            blockchain,
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
