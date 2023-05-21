use bytes::Bytes;
use eyre::*;
use gen::database::{FunWatcherSaveRawTransactionReq, FunWatcherSaveWalletActivityHistoryReq};
use gen::model::{EnumBlockChain, EnumDex, EnumDexVersion};
use lib::database::DbClient;
use serde::{Deserialize, Serialize};
use tracing::error;
use web3::types::{H160, H256, U256};

mod calldata;
mod ethabi_to_web3;
mod rpc_provider;
mod tx;

pub use calldata::*;
pub use ethabi_to_web3::*;
pub use rpc_provider::*;
pub use tx::*;

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
            chain: EnumBlockChain::EthereumMainnet.to_string(),
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
    hash: &H256,
    tx: &Transaction,
    db: &DbClient,
) -> Result<()> {
    if let Err(err) = async {
        if let Some(content) = tx.get_transaction() {
            db.execute(FunWatcherSaveRawTransactionReq {
                transaction_hash: format!("{:?}", hash),
                chain: EnumBlockChain::EthereumMainnet.to_string(),
                dex: None,
                raw_transaction: serde_json::to_string(content).context("transaction")?,
            })
            .await?;
        }
        Ok::<_, Error>(())
    }
    .await
    {
        error!("failed to save raw transaction: {:?}", err);
    }

    Ok(())
}
