use crate::pancake_swap::parse::build_pancake_swap_parser;
use crate::pancake_swap::PancakePairPathSet;
use crate::{EthereumRpcConnection, TransactionFetcher, TransactionReady};
use bytes::Bytes;
use eyre::*;
use gen::database::*;
use gen::model::*;
use lib::database::DbClient;
use lib::utils::hex_decode;
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

async fn get_pair_path_from_db(
    db: &DbClient,
    conn: &EthereumRpcConnection,
    blockchain: EnumBlockChain,
    token_in_address: Address,
    token_out_address: Address,
    dex: Option<EnumDex>,
    format: Option<EnumDexPathFormat>,
) -> Result<PancakePairPathSet> {
    // TODO: return DexPairPathSet once we support multiple dexes
    let pair_path_row = db
        .execute(FunWatcherListDexPathForPairReq {
            blockchain,
            token_in_address: token_in_address.into(),
            token_out_address: token_out_address.into(),
            dex,
            format,
        })
        .await?
        .into_result()
        .context("could not find dex pair path in test")?;

    let pair_path = match pair_path_row.format {
        EnumDexPathFormat::Json => serde_json::from_str(&pair_path_row.path_data)?,
        EnumDexPathFormat::TransactionData => {
            let pancake_parser = build_pancake_swap_parser()?;
            pancake_parser
                .parse_paths_from_inputs(&hex_decode(&pair_path_row.path_data.as_bytes())?)?
        }
        EnumDexPathFormat::TransactionHash => {
            let tx_hash: H256 = pair_path_row.path_data.parse()?;
            let tx_ready = TransactionFetcher::new_and_assume_ready(tx_hash, &conn).await?;
            let pancake_parser = build_pancake_swap_parser()?;
            pancake_parser.parse_paths_from_inputs(&tx_ready.get_input_data())?
        }
    };

    Ok(pair_path)
}
