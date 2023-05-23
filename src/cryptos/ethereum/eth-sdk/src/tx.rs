use crate::EthereumRpcConnection;
use eyre::*;
use web3::types::{Transaction as Web3Transaction, TransactionReceipt, H160, H256, U256};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TxStatus {
    Unknown,
    Successful,
    Pending,
    Reverted,
    NotFound,
}

#[derive(Clone, Debug)]
pub struct Transaction {
    hash: H256,
    transaction: Option<Web3Transaction>,
    receipt: Option<TransactionReceipt>,
    status: TxStatus,
    // TODO: add field: EnumBlockchain
}

impl Transaction {
    pub fn new(hash: H256) -> Self {
        Self {
            hash,
            transaction: None,
            receipt: None,
            status: TxStatus::Unknown,
        }
    }
    pub async fn new_and_assume_ready(
        hash: H256,
        conn: &EthereumRpcConnection,
    ) -> Result<TransactionReady> {
        let mut this = Self::new(hash);
        this.update(conn).await?;
        this.assume_ready()
    }

    pub async fn update(&mut self, conn: &EthereumRpcConnection) -> Result<()> {
        // TODO: handle EnumBlockChain connection error
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
    pub fn get_hash(&self) -> H256 {
        self.hash
    }
    pub fn get_transaction(&self) -> Option<&Web3Transaction> {
        self.transaction.as_ref()
    }
    pub fn get_status(&self) -> TxStatus {
        self.status.clone()
    }

    pub fn get_value(&self) -> Option<&U256> {
        self.transaction.as_ref().map(|x| &x.value)
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
    pub fn assume_ready(self) -> Result<TransactionReady> {
        ensure!(
            self.status == TxStatus::Successful,
            "Transaction status {:?}, transaction hash={:?}",
            self.status,
            self.hash
        );
        Ok(TransactionReady {
            hash: self.hash,
            transaction: self.transaction.context("No valid transaction body")?,
            receipt: self.receipt.context("No valid receipt")?,
        })
    }
}
#[derive(Clone, Debug)]
pub struct TransactionReady {
    hash: H256,
    transaction: Web3Transaction,
    receipt: TransactionReceipt,
}
impl TransactionReady {
    pub fn get_hash(&self) -> H256 {
        self.hash
    }
    pub fn get_transaction(&self) -> &Web3Transaction {
        &self.transaction
    }
    pub fn get_status(&self) -> TxStatus {
        TxStatus::Successful
    }

    pub fn get_value(&self) -> U256 {
        self.transaction.value
    }

    pub fn get_input_data(&self) -> &Vec<u8> {
        &self.transaction.input.0
    }

    pub fn get_receipt(&self) -> &TransactionReceipt {
        &self.receipt
    }

    pub fn get_to(&self) -> Option<H160> {
        self.transaction.to
    }

    pub fn get_from(&self) -> Option<H160> {
        self.transaction.from
    }

    pub fn amount_of_token_received(
        &self,
        token_contract: H160,
        recipient: H160,
        transfer_event_signature: H256,
    ) -> Option<U256> {
        let receipt = self.get_receipt();

        for log in &receipt.logs {
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
        None
    }

    pub fn amount_of_token_sent(
        &self,
        token_contract: H160,
        sender: H160,
        transfer_event_signature: H256,
    ) -> Option<U256> {
        let receipt = self.get_receipt();

        for log in &receipt.logs {
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

        None
    }
}
