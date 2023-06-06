use super::conn::EthereumRpcConnection;
use crate::conn;

use deadpool::managed::{Manager, Object, RecycleResult};
use eyre::*;
use gen::model::EnumBlockChain;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EthereumConns {
    #[serde(flatten)]
    conns: HashMap<EnumBlockChain, String>,
}
impl EthereumConns {
    pub fn new() -> Self {
        let mut this = Self {
            conns: HashMap::new(),
        };
        this.conns.insert(
            EnumBlockChain::EthereumMainnet,
            "https://ethereum.publicnode.com".to_string(),
        );
        this.conns.insert(
            EnumBlockChain::EthereumGoerli,
            "https://ethereum-goerli.publicnode.com".to_string(),
        );
        this.conns.insert(
            EnumBlockChain::EthereumSepolia,
            "https://rpc.sepolia.dev".to_string(),
        );
        this.conns.insert(
            EnumBlockChain::BscMainnet,
            "https://bsc.publicnode.com".to_string(),
        );
        this.conns.insert(
            EnumBlockChain::BscTestnet,
            "https://bsc-testnet.publicnode.com".to_string(),
        );
        this.conns.insert(
            EnumBlockChain::LocalNet,
            "http://127.0.0.1:8545".to_string(),
        );
        this
    }
}
#[derive(Clone, Debug)]
pub struct EthereumRpcConnectionManager {
    provider_url: String,
}
#[async_trait::async_trait]
impl Manager for EthereumRpcConnectionManager {
    type Type = EthereumRpcConnection;
    type Error = Error;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        let transport = conn::new_transport(&self.provider_url).await?;
        let web3 = web3::Web3::new(transport);
        Ok(web3)
    }

    async fn recycle(&self, _obj: &mut Self::Type) -> RecycleResult<Self::Error> {
        Ok(())
    }
}
#[derive(Clone, Debug)]
pub struct EthereumRpcConnectionPoolInner {
    pools: HashMap<EnumBlockChain, deadpool::managed::Pool<EthereumRpcConnectionManager>>,
}
#[derive(Clone, Debug)]
pub struct EthereumRpcConnectionPool(Arc<EthereumRpcConnectionPoolInner>);

impl EthereumRpcConnectionPool {
    pub fn from_conns(conns: EthereumConns) -> Self {
        let mut pools = HashMap::new();
        for (key, val) in conns.conns.iter() {
            let pool = deadpool::managed::Pool::builder(EthereumRpcConnectionManager {
                provider_url: val.to_string(),
            })
            .build()
            .unwrap();
            pools.insert(key.clone(), pool);
        }
        Self(Arc::new(EthereumRpcConnectionPoolInner { pools }))
    }
    pub fn new() -> Self {
        let conns = EthereumConns::new();
        Self::from_conns(conns)
    }

    pub async fn get(&self, chain: EnumBlockChain) -> Result<EthereumRpcConnectionGuard> {
        let conn = self
            .0
            .pools
            .get(&chain)
            .context("No available chain")?
            .get()
            .await
            .map_err(|err| eyre!("Failed to get connection from pool: {:?}", err))?;
        Ok(conn)
    }
}
pub type EthereumRpcConnectionGuard = Object<EthereumRpcConnectionManager>;
