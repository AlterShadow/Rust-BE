use std::collections::HashMap;
use std::ops::Deref;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

use eyre::*;

use super::connection::Connection;
use tokio::sync::{Mutex, RwLock};
use tokio::time::sleep;

#[derive(Clone, Debug)]
pub struct ConnectionPool {
    pool_and_availability: Arc<Mutex<(Vec<Option<Connection>>, HashMap<usize, bool>)>>,
    provider_url: String,
    max_concurrent_requests: usize,
    max_retries: u32,
    backoff: Arc<AtomicU32>,
}

impl ConnectionPool {
    pub async fn new(
        provider_url: String,
        max_connections: usize,
        max_concurrent_requests: usize,
        max_retries: u32,
    ) -> Result<Arc<Self>> {
        let transport = web3::transports::WebSocket::new(&provider_url).await?;
        let web3 = web3::Web3::new(transport);
        let conn = Connection::new(Arc::new(web3), max_concurrent_requests);

        let mut pool: Vec<Option<Connection>> = vec![None; max_connections];
        let mut conns_in_use: HashMap<usize, bool> = HashMap::new();
        pool[0] = Some(conn);
        for i in 0..max_connections {
            conns_in_use.insert(i, false);
        }

        Ok(Arc::new(Self {
            pool_and_availability: Arc::new(Mutex::new((pool, conns_in_use))),
            provider_url,
            max_concurrent_requests,
            max_retries,
            backoff: Arc::new(AtomicU32::new(0)),
        }))
    }

    async fn new_conn(&self) -> Result<Connection> {
        let transport = web3::transports::WebSocket::new(&self.provider_url).await?;
        let web3 = web3::Web3::new(transport);
        Ok(Connection::new(
            Arc::new(web3),
            self.max_concurrent_requests,
        ))
    }

    pub async fn get_conn(self: Arc<Self>) -> Result<ConnectionGuard> {
        let mut backoff = self.backoff.load(Ordering::Relaxed);
        let mut retries = 0;

        while retries < self.max_retries {
            let mut pool_and_availability = self.pool_and_availability.lock().await;
            let (ref mut connections, ref mut conns_in_use) = *pool_and_availability;
            for i in 0..connections.len() {
                match &connections[i] {
                    Some(conn) => {
                        if let Some(true) = conns_in_use.get(&i) {
                            continue;
                        } else {
                            if conn.ping().await.is_ok() {
                                /* if there is a good connection, decrease backoff period */
                                backoff = self.backoff.fetch_sub(1, Ordering::Relaxed);
                                conns_in_use.insert(i, true);
                                return Ok(ConnectionGuard::new(
                                    conn.clone(),
                                    i,
                                    Arc::clone(&self),
                                ));
                            } else {
                                /* if there is a bad connection, increase backoff period */
                                backoff = self.backoff.fetch_add(1, Ordering::Relaxed);
                                let new_conn = self.new_conn().await?;
                                connections[i] = Some(new_conn.clone());
                                conns_in_use.insert(i, true);
                                return Ok(ConnectionGuard::new(new_conn, i, Arc::clone(&self)));
                            }
                        }
                    }
                    None => {
                        let new_conn = self.new_conn().await?;
                        connections[i] = Some(new_conn.clone());
                        conns_in_use.insert(i, true);
                        return Ok(ConnectionGuard::new(new_conn, i, Arc::clone(&self)));
                    }
                }
            }

            /* backoff */
            sleep(Duration::from_secs(2_u64.pow(backoff as u32))).await;
            retries += 1;
        }

        Err(eyre!("No available connection"))
    }

    async fn release(&self, index: usize) {
        let mut pool_and_availability = self.pool_and_availability.lock().await;
        let (_, ref mut conns_in_use) = *pool_and_availability;
        conns_in_use.insert(index, false);
    }
}

#[derive(Clone, Debug)]
pub struct ConnectionGuard {
    inner: Connection,
    index: usize,
    pool: Arc<ConnectionPool>,
}

impl ConnectionGuard {
    pub fn new(conn: Connection, index: usize, pool: Arc<ConnectionPool>) -> Self {
        Self {
            inner: conn,
            index: index,
            pool: pool,
        }
    }

    pub async fn release(self) {
        self.pool.release(self.index).await;
    }
}

impl Deref for ConnectionGuard {
    type Target = Connection;
    /* allows for calls to the Connection directly */
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Drop for ConnectionGuard {
    /* connection will automatically be released when it goes out of scope */
    /* DANGER WARNING IMPORTANT: drop might not happen if the client thread runs in a different runtime */
    fn drop(&mut self) {
        let conn = self.clone();
        tokio::spawn(async move {
            conn.release().await;
        });
    }
}
