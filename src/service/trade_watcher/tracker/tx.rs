use web3::types::{Transaction, TransactionReceipt, H256, H160, U256};

use super::super::rpc_provider::pool::ConnectionGuard;

#[derive(Clone, Debug)]
pub enum TxStatus {
    Successful,
    Pending,
    Reverted,
    NotFound,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct Tx {
    conn: ConnectionGuard,
    hash: H256,
    transaction: Option<Transaction>,
    receipt: Option<TransactionReceipt>,
    status: TxStatus,
}

impl Tx {
    pub async fn new(hash: H256, conn: ConnectionGuard) -> Self {
        let mut new_tx = Self {
            conn,
            hash,
            transaction: None,
            receipt: None,
            status: TxStatus::Unknown,
        };
        new_tx.update().await;
        new_tx
    }

    pub async fn update(&mut self) {
        // TODO: handle blockchain connection error
        let tx = match self.conn.get_tx(self.hash).await {
            Ok(maybe_tx) => match maybe_tx {
                Some(tx) => tx,
                None => {
                    self.status = TxStatus::NotFound;
                    return;
                }
            },
            Err(e) => {
                println!("Error getting transaction: {:?}", e);
                return;
            }
        };

        self.transaction = Some(tx.clone());

        if tx.block_number.is_none() {
            self.status = TxStatus::Pending;
            return;
        }

        let receipt = match self.conn.get_receipt(self.hash).await {
            Ok(maybe_receipt) => match maybe_receipt {
                Some(receipt) => receipt,
                None => {
                    self.status = TxStatus::NotFound;
                    return;
                }
            },
            Err(e) => {
                println!("Error getting transaction receipt: {:?}", e);
                return;
            }
        };

        self.receipt = Some(receipt.clone());

        if receipt.status == Some(web3::types::U64([1])) {
            self.status = TxStatus::Successful;
        } else {
            self.status = TxStatus::Reverted;
        }
    }

    pub fn get_status(&self) -> TxStatus {
        self.status.clone()
    }

    pub fn get_value(&self) -> Option<u128> {
        match &self.transaction {
            Some(tx) => Some(tx.value.as_u128()),
            None => None,
        }
    }

    pub fn get_input_data(&self) -> Option<Vec<u8>> {
        match &self.transaction {
            Some(tx) => Some(tx.input.0.to_vec()),
            None => None,
        }
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
				match &self.transaction {
						Some(tx) => Some(tx.hash),
						None => None,
				}
		}

		pub fn amount_of_token_received(&self, token_contract: H160, recipient: H160, transfer_event_signature: H256) -> Option<U256> {
			if let Some(receipt) = self.get_receipt() {
					for log in receipt.logs {
							/* there can only be 4 indexed (topic) values in a event log */
							if log.topics.len() >= 3 
							/* 1st topic is always the hash of the event signature */
							&& log.topics[0] == transfer_event_signature
							/* address of the contract that fired the event */
							&& log.address == token_contract {
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

	pub fn amount_of_token_sent(&self, token_contract: H160, sender: H160, transfer_event_signature: H256) -> Option<U256> {
			if let Some(receipt) = self.get_receipt() {
					for log in receipt.logs {
							/* there can only be 4 indexed (topic) values in a event log */
							if log.topics.len() >= 3 
							/* 1st topic is always the hash of the event signature */
							&& log.topics[0] == transfer_event_signature
							/* address of the contract that fired the event */
							&& log.address == token_contract {
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
