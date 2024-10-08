use std::time::Duration;

use eyre::*;
use web3::api::Eth;
use web3::types::{
    Address, Transaction as Web3Transaction, TransactionId, TransactionReceipt, H160, H256, U256,
};
use web3::Transport;

use crate::utils::{wait_for_confirmations, wait_for_confirmations_simple, ConfirmationError};
use crate::EthereumRpcConnection;
use lib::log::DynLogger;

pub async fn execute_transaction_and_ensure_success<Tx, Fut>(
    transaction: Tx,
    conn: &EthereumRpcConnection,
    confirmations: u64,
    max_retries: u64,
    poll_interval: Duration,
    logger: &DynLogger,
) -> Result<H256>
where
    Tx: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<H256, RpcCallError>>,
{
    for _full_attempt in 0..max_retries {
        let mut maybe_hash: Option<H256> = None;
        for _transaction_attempt in 0..max_retries {
            let transaction_result = transaction().await;

            match transaction_result {
                Ok(successful_hash) => {
                    /* if the transaction was successfully published, attempt confirmation */
                    maybe_hash = Some(successful_hash);
                    break;
                }
                Err(RpcCallError::ProviderError(e)) | Err(RpcCallError::Web3Error(e)) => {
                    /* if the transaction failed to be published because of an external error, replay transaction */
                    logger.log(format!(
                        "provider error {:?} publishing transaction, replaying transaction",
                        e
                    ));
                    continue;
                }
                Err(internal_error) => {
                    /* if the transaction failed to be published because of an internal error, return the error */
                    return Err(internal_error.into());
                }
            }
        }

        if maybe_hash.is_none() {
            bail!(
                "transaction failed to be published after {} attempts",
                max_retries
            );
        }

        let hash = maybe_hash.unwrap();

        logger.log(format!(
            "transaction {:?} sent, waiting for confirmations",
            hash
        ));

        for _confirmation_attempt in 0..max_retries {
            let confirmation_result = wait_for_confirmations(
                &conn.eth(),
                hash,
                poll_interval,
                max_retries,
                confirmations,
            )
            .await;

            match confirmation_result {
                Ok(_) => {
                    /* if the transaction was successfully confirmed, return it's hash */
                    return Ok(hash);
                }
                Err(ConfirmationError::ProviderError(err)) => {
                    /* if the transaction confirmation failed because of a provider error, retry confirmation */
                    logger.log(format!(
                        "provider error {:?} confirming transaction {:?}, retrying confirmation",
                        err, hash
                    ));
                    continue;
                }
                Err(ConfirmationError::TransactionRevertedAfterConfirmations(_))
                | Err(ConfirmationError::TransactionNotFoundAfterConfirmations(_)) => {
                    /* if the transaction was reverted or not found after confirmations, replay transaction */
                    logger.log(format!(
                        "transaction {:?} failed after confirmations, replaying transaction",
                        hash
                    ));
                    break;
                }
                Err(error) => {
                    /* if the transaction was reverted or not found before confirmations, return the error */
                    return Err(error.into());
                }
            }
        }
    }

    bail!(
        "transaction failed to be confirmed after {} attempts",
        max_retries
    )
}

#[derive(Debug)]
pub enum ApiOrContractError {
    ApiError(web3::error::Error),
    ContractError(web3::contract::Error),
}

#[derive(Debug)]
pub enum RpcCallError {
    InternalError(ApiOrContractError),
    ProviderError(ApiOrContractError),
    Web3Error(ApiOrContractError),
    InternalErrorWithMessage(String),
}

impl std::fmt::Display for RpcCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RpcCallError::InternalError(error) => {
                write!(f, "internal error: {:?}", error)
            }
            RpcCallError::ProviderError(error) => {
                write!(f, "provider error: {:?}", error)
            }
            RpcCallError::Web3Error(error) => {
                write!(f, "web3 error: {:?}", error)
            }
            RpcCallError::InternalErrorWithMessage(message) => {
                write!(f, "internal error: {}", message)
            }
        }
    }
}

impl std::error::Error for RpcCallError {}

impl From<web3::Error> for RpcCallError {
    fn from(error: web3::Error) -> Self {
        use web3::Error;
        match error {
            /* server is unreachable */
            /* if it is because the server is offline is certainly a provider error */
            /* if it is because we can't establish a connection to the internet, it is our error */
            /* either way the best approach is to retry, so we classify it as a provider error */
            Error::Unreachable => {
                RpcCallError::ProviderError(ApiOrContractError::ApiError(Error::Unreachable))
            }
            /* decoder error */
            /* for now assume they are rarely our fault */
            // TODO: deep dive into possible decoder errors and perhaps add more variants for error handling
            Error::Decoder(message) => {
                RpcCallError::Web3Error(ApiOrContractError::ApiError(Error::Decoder(message)))
            }
            /* invalid response means web3 could not parse the response from the RPC provider */
            /* e.g. can happen when using a public node to call "eth_blockNumber" */
            /* this can be classified as a provider error with a reasonable degree of certainty */
            Error::InvalidResponse(message) => RpcCallError::ProviderError(
                ApiOrContractError::ApiError(Error::InvalidResponse(message)),
            ),
            /* transport error */
            /* for now assume they rarely are our fault */
            // TODO: deep dive into possible transport errors and perhaps add more variants for error handling
            Error::Transport(transport_error) => RpcCallError::ProviderError(
                ApiOrContractError::ApiError(Error::Transport(transport_error)),
            ),
            /* RPC errors are returned from the RPC provider */
            Error::Rpc(rpc_error) => {
                match rpc_error.code.code() {
                    /* Parse error */
                    /* Invalid JSON was received by the server */
                    /* An error occurred on the server while parsing the JSON text. */
                    -32700 => RpcCallError::InternalError(ApiOrContractError::ApiError(
                        Error::Rpc(rpc_error),
                    )),
                    /* Invalid request */
                    /* The JSON sent is not a valid Request object */
                    -32600 => RpcCallError::InternalError(ApiOrContractError::ApiError(
                        Error::Rpc(rpc_error),
                    )),
                    /* Method not found */
                    /* The method does not exist / is not available */
                    -32601 => RpcCallError::InternalError(ApiOrContractError::ApiError(
                        Error::Rpc(rpc_error),
                    )),
                    /* Invalid params */
                    /* Invalid method parameter(s) */
                    -32602 => RpcCallError::InternalError(ApiOrContractError::ApiError(
                        Error::Rpc(rpc_error),
                    )),
                    /* Internal error */
                    /* Internal JSON-RPC error */
                    -32603 => RpcCallError::ProviderError(ApiOrContractError::ApiError(
                        Error::Rpc(rpc_error),
                    )),
                    /* Server error */
                    /* Reserved for implementation-defined server-errors */
                    _ => RpcCallError::ProviderError(ApiOrContractError::ApiError(Error::Rpc(
                        rpc_error,
                    ))),
                }
            }
            /* std::io::error::Error */
            Error::Io(io_error) => {
                RpcCallError::Web3Error(ApiOrContractError::ApiError(Error::Io(io_error)))
            }
            /* web3::signing::RecoveryError */
            /* indicates either an invalid message, or invalid signature, both should be internal errors */
            Error::Recovery(recovery_error) => RpcCallError::InternalError(
                ApiOrContractError::ApiError(Error::Recovery(recovery_error)),
            ),
            /* web3 internal error */
            Error::Internal => {
                RpcCallError::Web3Error(ApiOrContractError::ApiError(Error::Internal))
            }
        }
    }
}

impl From<web3::contract::Error> for RpcCallError {
    fn from(error: web3::contract::Error) -> Self {
        use web3::contract::Error;

        match error {
            /* invalid output type requested by the caller */
            Error::InvalidOutputType(message) => RpcCallError::InternalError(
                ApiOrContractError::ContractError(Error::InvalidOutputType(message)),
            ),
            /* eth abi error */
            Error::Abi(eth_error) => RpcCallError::InternalError(
                ApiOrContractError::ContractError(Error::Abi(eth_error)),
            ),
            /* rpc error */
            Error::Api(rpc_error) => RpcCallError::from(rpc_error),
            /* error during deployment */
            Error::Deployment(deployment_error) => {
                use web3::contract::deploy;
                match deployment_error {
                    /* eth abi error during deployment */
                    deploy::Error::Abi(ethabi_error) => {
                        RpcCallError::InternalError(ApiOrContractError::ContractError(
                            Error::Deployment(deploy::Error::Abi(ethabi_error)),
                        ))
                    }
                    /* rpc error during deployment */
                    deploy::Error::Api(rpc_error) => RpcCallError::from(rpc_error),
                    /* general failure during deployment */
                    deploy::Error::ContractDeploymentFailure(tx_hash) => {
                        RpcCallError::ProviderError(ApiOrContractError::ContractError(
                            Error::Deployment(deploy::Error::ContractDeploymentFailure(tx_hash)),
                        ))
                    }
                }
            }
            Error::InterfaceUnsupported => RpcCallError::InternalError(
                ApiOrContractError::ContractError(Error::InterfaceUnsupported),
            ),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TxStatus {
    Unknown,
    Successful,
    Pending,
    Reverted,
    NotFound,
}

#[derive(Clone, Debug)]
pub struct TransactionFetcher {
    hash: H256,
    transaction: Option<Web3Transaction>,
    receipt: Option<TransactionReceipt>,
    status: TxStatus,
    // TODO: add field: EnumBlockchain
}

impl TransactionFetcher {
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

    pub async fn update_retry(&mut self, conn: &EthereumRpcConnection) -> Result<()> {
        // TODO: handle EnumBlockChain connection error
        let maybe_tx = conn
            .eth()
            .transaction(TransactionId::Hash(self.hash))
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
        let receipt =
            wait_for_confirmations_simple(&conn.eth(), self.hash, Duration::from_secs(3), 5)
                .await?;

        self.receipt = Some(receipt.clone());

        if receipt.status == Some(web3::types::U64([1])) {
            self.status = TxStatus::Successful;
        } else {
            self.status = TxStatus::Reverted;
        }
        Ok(())
    }
    pub async fn update(&mut self, conn: &EthereumRpcConnection) -> Result<()> {
        // TODO: handle EnumBlockChain connection error
        let maybe_tx = conn
            .eth()
            .transaction(TransactionId::Hash(self.hash))
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
            .eth()
            .transaction_receipt(self.hash)
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
    ) -> Result<U256> {
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
                    let to_bytes = log.topics[2].as_bytes();
                    if to_bytes.len() < 32 {
                        return Err(eyre!("invalid topic length"));
                    }
                    let to = H160::from_slice(&to_bytes[12..]);

                    if to == recipient {
                        /* transfer value is not indexed according to ERC20, and is stored in log data */
                        let data = log.data.0.as_slice();
                        if data.len() < 32 {
                            return Err(eyre!("invalid data length"));
                        }
                        let amount_out = U256::from_big_endian(&data);
                        return Ok(amount_out);
                    }
                }
            }
            return Err(eyre!("transfer log not found"));
        }

        Err(eyre!("no receipt"))
    }

    pub fn amount_of_token_sent(
        &self,
        token_contract: H160,
        sender: H160,
        transfer_event_signature: H256,
    ) -> Result<U256> {
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
                    let from_bytes = log.topics[1].as_bytes();
                    if from_bytes.len() < 32 {
                        return Err(eyre!("invalid topic length"));
                    }
                    let from = H160::from_slice(&from_bytes[12..]);

                    if from == sender {
                        /* transfer value is not indexed according to ERC20, and is stored in log data */
                        let data = log.data.0.as_slice();
                        if data.len() < 32 {
                            return Err(eyre!("invalid data length"));
                        }
                        let amount_out = U256::from_big_endian(&data);
                        return Ok(amount_out);
                    }
                }
            }
            return Err(eyre!("transfer log not found"));
        }

        Err(eyre!("no receipt"))
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

    pub fn get_to(&self) -> Option<Address> {
        self.transaction.to
    }

    pub fn get_from(&self) -> Option<Address> {
        self.transaction.from
    }
    // TODO: move to ERC20, as Transfer event is defined in ERC20
    pub fn parse_amount_of_token_received(
        receipt: &TransactionReceipt,
        token_contract: Address,
        recipient: Address,
        transfer_event_signature: H256,
    ) -> Result<U256> {
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
                let to_bytes = log.topics[2].as_bytes();
                ensure!(to_bytes.len() >= 32, "invalid topic length");

                let to = H160::from_slice(&to_bytes[12..]);

                if to == recipient {
                    /* transfer value is not indexed according to ERC20, and is stored in log data */
                    let data = log.data.0.as_slice();
                    ensure!(data.len() >= 32, "invalid data length");

                    let amount_out = U256::from_big_endian(&data);
                    return Ok(amount_out);
                }
            }
        }

        bail!("transfer log not found")
    }

    pub fn amount_of_token_received(
        &self,
        token_contract: Address,
        recipient: Address,
        transfer_event_signature: H256,
    ) -> Result<U256> {
        Self::parse_amount_of_token_received(
            &self.receipt,
            token_contract,
            recipient,
            transfer_event_signature,
        )
    }
    pub fn parse_amount_of_token_sent(
        receipt: &TransactionReceipt,
        token_contract: H160,
        sender: H160,
        transfer_event_signature: H256,
    ) -> Result<U256> {
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
                let from_bytes = log.topics[1].as_bytes();
                ensure!(from_bytes.len() >= 32, "invalid topic length");

                let from = H160::from_slice(&from_bytes[12..]);

                if from == sender {
                    /* transfer value is not indexed according to ERC20, and is stored in log data */
                    let data = log.data.0.as_slice();
                    ensure!(data.len() >= 32, "invalid data length");

                    let amount_out = U256::from_big_endian(&data);
                    return Ok(amount_out);
                }
            }
        }
        bail!("transfer log not found")
    }

    pub fn amount_of_token_sent(
        &self,
        token_contract: H160,
        sender: H160,
        transfer_event_signature: H256,
    ) -> Result<U256> {
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
                let from_bytes = log.topics[1].as_bytes();
                if from_bytes.len() < 32 {
                    return Err(eyre!("invalid topic length"));
                }
                let from = H160::from_slice(&from_bytes[12..]);

                if from == sender {
                    /* transfer value is not indexed according to ERC20, and is stored in log data */
                    let data = log.data.0.as_slice();
                    if data.len() < 32 {
                        return Err(eyre!("invalid data length"));
                    }
                    let amount_out = U256::from_big_endian(&data);
                    return Ok(amount_out);
                }
            }
        }

        Err(eyre!("transfer log not found"))
    }
}

pub struct TxChecker<T: Transport> {
    conn: Eth<T>,
}

impl<T: Transport> TxChecker<T> {
    pub fn new(conn: Eth<T>) -> Self {
        Self { conn }
    }

    pub async fn status(&self, tx_hash: H256) -> Result<TxStatus> {
        let receipt =
            wait_for_confirmations_simple(&self.conn, tx_hash, Duration::from_secs_f64(3.0), 10)
                .await?;

        if receipt.status == Some(web3::types::U64::from(1)) {
            Ok(TxStatus::Successful)
        } else {
            Ok(TxStatus::Reverted)
        }
    }
}
