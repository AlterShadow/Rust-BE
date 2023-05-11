use std::ops::Deref;
use std::sync::Arc;

use eyre::*;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};
use web3::types::{Transaction, TransactionId, TransactionReceipt, H256};
use web3::{transports::WebSocket, Web3};

// TODO: handle blockchain connection error

#[derive(Clone, Debug)]
pub struct Conn {
    _inner: Arc<Web3<WebSocket>>,
    _semaphore: Arc<Semaphore>,
}

pub type ConnError = web3::Error;

impl Conn {
    pub async fn new(provider_url: &str, max_concurrent_requests: usize) -> Result<Self> {
        let transport = web3::transports::WebSocket::new(provider_url).await?;
        let web3 = web3::Web3::new(transport);

        Ok(Self {
            _inner: Arc::new(web3),
            _semaphore: Arc::new(Semaphore::new(max_concurrent_requests)),
        })
    }

    pub async fn get_conn(&self) -> Result<ConnGuard> {
        // used to call web3 directly while handling the limit of concurrent requests
        let permit = self._semaphore.clone().acquire_owned().await?;
        Ok(ConnGuard {
            _inner: self._inner.clone(),
            _permit: permit,
        })
    }

    pub async fn get_tx(&self, tx_hash: H256) -> Result<Option<Transaction>> {
        let permit = self._semaphore.acquire().await?;
        let tx_result = self
            ._inner
            .eth()
            .transaction(TransactionId::Hash(tx_hash))
            .await?;
        drop(permit);
        Ok(tx_result)
    }

    pub async fn get_receipt(&self, tx_hash: H256) -> Result<Option<TransactionReceipt>> {
        let permit = self._semaphore.acquire().await?;
        let receipt_result = self._inner.eth().transaction_receipt(tx_hash).await?;
        drop(permit);
        Ok(receipt_result)
    }
}

pub struct ConnGuard {
    _inner: Arc<Web3<WebSocket>>,
    // permit will be dropped automatically when the guard goes out of scope
    _permit: OwnedSemaphorePermit,
}

impl Deref for ConnGuard {
    type Target = Web3<WebSocket>;

    fn deref(&self) -> &Self::Target {
        &self._inner
    }
}
