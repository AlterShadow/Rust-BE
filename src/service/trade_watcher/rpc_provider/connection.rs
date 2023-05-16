use eyre::*;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::{OwnedSemaphorePermit, Semaphore, SemaphorePermit};
use web3::types::{Transaction, TransactionId, TransactionReceipt, H256};
use web3::{transports::WebSocket, Web3};

#[derive(Clone, Debug)]
pub struct Connection {
    inner: Arc<Web3<WebSocket>>,
    semaphore: Arc<Semaphore>,
}

impl Connection {
    pub fn new(connection: Arc<Web3<WebSocket>>, max_concurrent_requests: usize) -> Self {
        Self {
            inner: connection,
            semaphore: Arc::new(Semaphore::new(max_concurrent_requests)),
        }
    }

    pub async fn get_permit(&self) -> Result<ConnectionPermitGuard> {
        /* used to call web3 directly while handling the limit of concurrent requests */
        let permit = self.semaphore.clone().acquire_owned().await?;
        Ok(ConnectionPermitGuard::new(self.inner.clone(), permit))
    }

    pub async fn get_tx(&self, tx_hash: H256) -> Result<Option<Transaction>> {
        let permit = self.semaphore.acquire().await?;
        let tx_result = self
            .inner
            .eth()
            .transaction(TransactionId::Hash(tx_hash))
            .await?;
        drop(permit);
        Ok(tx_result)
    }

    pub async fn get_receipt(&self, tx_hash: H256) -> Result<Option<TransactionReceipt>> {
        let permit = self.semaphore.acquire().await?;
        let receipt_result = self.inner.eth().transaction_receipt(tx_hash).await?;
        drop(permit);
        Ok(receipt_result)
    }

    pub async fn ping(&self) -> Result<()> {
        let _ = self.inner.eth().block_number().await?;
        Ok(())
    }
}

pub struct ConnectionPermitGuard {
    inner: Arc<Web3<WebSocket>>,
    /* permit will be dropped automatically when the guard goes out of scope */
    permit: OwnedSemaphorePermit,
}

impl ConnectionPermitGuard {
    pub fn new(inner: Arc<Web3<WebSocket>>, permit: OwnedSemaphorePermit) -> Self {
        Self { inner, permit }
    }
    pub async fn is_permitted(&self) -> Option<SemaphorePermit> {
        self.permit.semaphore().acquire().await.ok()
    }
}

impl Deref for ConnectionPermitGuard {
    type Target = Web3<WebSocket>;
    /* allows for calls to web3 directly */
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
