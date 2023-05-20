use crate::rpc_provider::connection::Connection;
use crate::tracker::pancake_swap::PancakeSwap;
use crate::tracker::trade::Dex;
use crate::tracker::Chain;
use crate::tracker::DexAddresses;
use eyre::*;
use gen::database::FunWatcherSaveRawTransactionReq;
use lib::database::DbClient;
use tracing::{error, info};
use web3::types::{Transaction, TransactionReceipt, H160, H256, U256};

#[derive(Clone, Debug)]
pub enum TxStatus {
    Unknown,
    Successful,
    Pending,
    Reverted,
    NotFound,
}

#[derive(Clone, Debug)]
pub struct Tx {
    hash: H256,
    transaction: Option<Transaction>,
    receipt: Option<TransactionReceipt>,
    status: TxStatus,
}

impl Tx {
    pub fn new(hash: H256) -> Self {
        Self {
            hash,
            transaction: None,
            receipt: None,
            status: TxStatus::Unknown,
        }
    }

    pub async fn update(&mut self, conn: &Connection) -> Result<()> {
        // TODO: handle blockchain connection error
        let maybe_tx = conn
            .get_tx(self.hash)
            .await
            .context("getting transaction")?;
        let tx = match maybe_tx {
            Some(tx) => tx,
            None => {
                self.status = TxStatus::NotFound;
                return Ok(());
            }
        };

        self.transaction = Some(tx.clone());

        if tx.block_number.is_none() {
            self.status = TxStatus::Pending;
            return Ok(());
        }
        let maybe_receipt = conn
            .get_receipt(self.hash)
            .await
            .context("getting receipt")?;
        let receipt = match maybe_receipt {
            Some(receipt) => receipt,
            None => {
                self.status = TxStatus::NotFound;
                return Ok(());
            }
        };

        self.receipt = Some(receipt.clone());

        if receipt.status == Some(web3::types::U64([1])) {
            self.status = TxStatus::Successful;
        } else {
            self.status = TxStatus::Reverted;
        }
        Ok(())
    }
    pub fn get_transaction(&self) -> Option<&Transaction> {
        self.transaction.as_ref()
    }
    pub fn get_status(&self) -> TxStatus {
        self.status.clone()
    }

    pub fn get_value(&self) -> Option<u128> {
        self.transaction.as_ref().map(|tx| tx.value.as_u128())
    }

    pub fn get_input_data(&self) -> Option<Vec<u8>> {
        self.transaction.as_ref().map(|tx| tx.input.0.to_vec())
    }

    pub fn get_receipt(&self) -> Option<TransactionReceipt> {
        self.receipt.clone()
    }

    pub fn get_to(&self) -> Option<H160> {
        match &self.transaction {
            Some(tx) => tx.to,
            None => None,
        }
    }

    pub fn get_from(&self) -> Option<H160> {
        match &self.transaction {
            Some(tx) => tx.from,
            None => None,
        }
    }

    pub fn get_id(&self) -> Option<H256> {
        self.transaction.as_ref().map(|tx| tx.hash)
    }

    pub fn amount_of_token_received(
        &self,
        token_contract: H160,
        recipient: H160,
        transfer_event_signature: H256,
    ) -> Option<U256> {
        if let Some(receipt) = self.get_receipt() {
            for log in receipt.logs {
                /* there can only be 4 indexed (topic) values in a event log */
                if log.topics.len() >= 3
                    /* 1st topic is always the hash of the event signature */
                    && log.topics[0] == transfer_event_signature
                    /* address of the contract that fired the event */
                    && log.address == token_contract
                {
                    /* 3rd topic according to ERC20 is the "to" address */
                    /* topics have 32 bytes, so we must fetch the last 20 bytes for an address */
                    let to = H160::from_slice(&log.topics[2].as_bytes()[12..]);

                    if to == recipient {
                        /* transfer value is not indexed according to ERC20, and is stored in log data */
                        let data = log.data.0.as_slice();
                        let amount_out = U256::from_big_endian(&data);
                        return Some(amount_out);
                    }
                }
            }
        }

        None
    }

    pub fn amount_of_token_sent(
        &self,
        token_contract: H160,
        sender: H160,
        transfer_event_signature: H256,
    ) -> Option<U256> {
        if let Some(receipt) = self.get_receipt() {
            for log in receipt.logs {
                /* there can only be 4 indexed (topic) values in a event log */
                if log.topics.len() >= 3
                    /* 1st topic is always the hash of the event signature */
                    && log.topics[0] == transfer_event_signature
                    /* address of the contract that fired the event */
                    && log.address == token_contract
                {
                    /* 2nd topic according to ERC20 is the "from" address */
                    /* topics have 32 bytes, so we must fetch the last 20 bytes for an address */
                    let from = H160::from_slice(&log.topics[1].as_bytes()[12..]);

                    if from == sender {
                        /* transfer value is not indexed according to ERC20, and is stored in log data */
                        let data = log.data.0.as_slice();
                        let amount_out = U256::from_big_endian(&data);
                        return Some(amount_out);
                    }
                }
            }
        }

        None
    }
}
pub async fn parse_ethereum_transaction(
    hash: H256,
    db: &DbClient,
    conn: &Connection,
    dex_addresses: &DexAddresses,
    pancake_swap: &PancakeSwap,
) -> Result<()> {
    let mut tx = Tx::new(hash);
    tx.update(&conn).await?;
    if let Err(err) = {
        if let Some(content) = tx.get_transaction() {
            db.execute(FunWatcherSaveRawTransactionReq {
                transaction_hash: format!("{:?}", hash),
                chain: "ethereum".to_string(),
                dex: None,
                raw_transaction: serde_json::to_string(content).context("transaction")?,
            })
            .await?;
        }
        Ok::<_, Error>(())
    } {
        error!("failed to save raw transaction: {}", err);
    }
    match tx.get_status() {
        TxStatus::Successful => (),
        TxStatus::Pending => {
            /* TODO: handle pending transaction */
            bail!("transaction is pending: {:?}", hash);
        }
        err => {
            bail!("transaction failed: {:?}", err);
        }
    }

    let contract_address = match tx.get_to() {
        Some(address) => address,
        None => {
            bail!("transaction has no contract address: {:?}", hash);
        }
    };

    let eth_mainnet_dexes = dex_addresses.get(&Chain::EthereumMainnet).unwrap();

    for (dex, address) in eth_mainnet_dexes {
        if *address == contract_address {
            let trade = match dex {
                Dex::PancakeSwap => pancake_swap.parse_trade(&tx, Chain::EthereumMainnet),
                Dex::UniSwap => {
                    error!("does not support dex type: UniSwap");
                    continue;
                }
                Dex::SushiSwap => {
                    error!("does not support dex type: SushiSwap");
                    continue;
                }
            };
            info!("tx: {:?}", tx.get_id().unwrap());
            info!("trade: {:?}", trade);
        }
    }
    Ok(())
}
