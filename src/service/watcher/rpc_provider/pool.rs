use super::connection::Connection;
use crate::rpc_provider::EitherTransport;
use deadpool::managed::{Manager, Object, RecycleResult};
use eyre::*;
use std::ops::Deref;
use std::sync::Arc;

use web3::transports::{Http, WebSocket};
#[derive(Clone, Debug)]
pub struct ConnectionManager {
    provider_url: String,
    max_concurrent_requests: usize,
}
#[async_trait::async_trait]
impl Manager for ConnectionManager {
    type Type = Connection;
    type Error = Error;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        let transport = new_transport(&self.provider_url).await?;
        let web3 = web3::Web3::new(transport);
        let conn = Connection::new(Arc::new(web3), self.max_concurrent_requests);
        Ok(conn)
    }

    async fn recycle(&self, _obj: &mut Self::Type) -> RecycleResult<Self::Error> {
        Ok(())
    }
}
#[derive(Clone, Debug)]
pub struct ConnectionPool {
    pool: deadpool::managed::Pool<ConnectionManager>,
}
async fn new_transport(url: &str) -> Result<EitherTransport> {
    let transport = match url {
        x if x.starts_with("http") => {
            EitherTransport::Right(Http::new(&url).context(url.to_owned())?)
        }
        x if x.starts_with("ws") => {
            EitherTransport::Left(WebSocket::new(&url).await.context(url.to_owned())?)
        }
        _ => bail!("Invalid provider url: {}", url),
    };
    Ok(transport)
}

impl ConnectionPool {
    pub async fn new(provider_url: String, max_concurrent_requests: usize) -> Result<Arc<Self>> {
        let pool = deadpool::managed::Pool::builder(ConnectionManager {
            provider_url,
            max_concurrent_requests,
        })
        .build()
        .unwrap();
        Ok(Arc::new(Self { pool }))
    }

    pub async fn get_conn(&self) -> Result<ConnectionGuard> {
        let conn = match self.pool.get().await {
            Ok(conn) => conn,
            Err(e) => {
                bail!("Failed to get connection from pool: {:?}", e);
            }
        };
        Ok(ConnectionGuard::new(conn))
    }
}

#[derive(Debug)]
pub struct ConnectionGuard {
    inner: Object<ConnectionManager>,
}

impl ConnectionGuard {
    pub fn new(conn: Object<ConnectionManager>) -> Self {
        Self { inner: conn }
    }
}

impl Deref for ConnectionGuard {
    type Target = Connection;
    /* allows for calls to the Connection directly */
    fn deref(&self) -> &Self::Target {
        &self.inner.as_ref()
    }
}
