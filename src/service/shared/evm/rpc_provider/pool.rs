use super::connection::EthereumRpcConnection;
use deadpool::managed::{Manager, Object, RecycleResult};
use eyre::*;
use std::sync::Arc;

use crate::evm::rpc_provider::EitherTransport;
use web3::transports::{Http, WebSocket};

#[derive(Clone, Debug)]
pub struct EthereumRpcConnectionManager {
    provider_url: String,
    max_concurrent_requests: usize,
}
#[async_trait::async_trait]
impl Manager for EthereumRpcConnectionManager {
    type Type = EthereumRpcConnection;
    type Error = Error;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        let transport = new_transport(&self.provider_url).await?;
        let web3 = web3::Web3::new(transport);
        let conn = EthereumRpcConnection::new(Arc::new(web3), self.max_concurrent_requests);
        Ok(conn)
    }

    async fn recycle(&self, _obj: &mut Self::Type) -> RecycleResult<Self::Error> {
        Ok(())
    }
}
#[derive(Clone, Debug)]
pub struct EthereumRpcConnectionPool {
    pool: deadpool::managed::Pool<EthereumRpcConnectionManager>,
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

impl EthereumRpcConnectionPool {
    pub fn new(provider_url: String, max_concurrent_requests: usize) -> Result<Self> {
        let pool = deadpool::managed::Pool::builder(EthereumRpcConnectionManager {
            provider_url,
            max_concurrent_requests,
        })
        .build()
        .unwrap();
        Ok(Self { pool })
    }
    pub fn mainnet() -> Self {
        EthereumRpcConnectionPool::new("https://ethereum.publicnode.com".to_string(), 10).unwrap()
    }

    pub async fn get_conn(&self) -> Result<EthereumRpcConnectionGuard> {
        let conn = match self.pool.get().await {
            Ok(conn) => conn,
            Err(e) => {
                bail!("Failed to get connection from pool: {:?}", e);
            }
        };
        Ok(conn)
    }
}
pub type EthereumRpcConnectionGuard = Object<EthereumRpcConnectionManager>;
