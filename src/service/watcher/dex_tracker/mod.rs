use axum::extract::State;
use axum::http::StatusCode;
use bytes::Bytes;
use eth_sdk::Transaction;
use gen::model::{EnumBlockChain, EnumDex};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use tracing::error;
use web3::types::H160;

mod pancake_swap;
pub use pancake_swap::*;
mod parse;
use crate::evm;
use crate::evm::AppState;
pub use parse::*;

pub struct DexAddresses {
    inner: HashMap<EnumBlockChain, Vec<(EnumDex, H160)>>,
}
impl Default for DexAddresses {
    fn default() -> Self {
        let mut this = DexAddresses {
            inner: HashMap::new(),
        };

        this.inner.insert(
            EnumBlockChain::EthereumMainnet,
            vec![(
                EnumDex::PancakeSwap,
                H160::from_str("0x13f4EA83D0bd40E75C8222255bc855a974568Dd4").unwrap(),
            )],
        );
        this.inner.insert(
            EnumBlockChain::BscMainnet,
            vec![(
                EnumDex::PancakeSwap,
                H160::from_str("0x13f4EA83D0bd40E75C8222255bc855a974568Dd4").unwrap(),
            )],
        );
        this.inner.insert(
            EnumBlockChain::EthereumGoerli,
            vec![(
                EnumDex::PancakeSwap,
                H160::from_str("0x9a489505a00cE272eAa5e07Dba6491314CaE3796").unwrap(),
            )],
        );
        this.inner.insert(
            EnumBlockChain::BscTestnet,
            vec![(
                EnumDex::PancakeSwap,
                H160::from_str("0x9a489505a00cE272eAa5e07Dba6491314CaE3796").unwrap(),
            )],
        );

        this
    }
}
impl DexAddresses {
    pub fn new() -> DexAddresses {
        Default::default()
    }
    pub fn get(&self, chain: &EnumBlockChain) -> Option<&Vec<(EnumDex, H160)>> {
        self.inner.get(chain)
    }
}

pub async fn handle_eth_swap(state: State<Arc<AppState>>, body: Bytes) -> Result<(), StatusCode> {
    let hashes = evm::parse_quickalert_payload(body).map_err(|e| {
        error!("failed to parse QuickAlerts payload: {:?}", e);
        StatusCode::BAD_REQUEST
    })?;

    for hash in hashes {
        let conn = state.eth_pool.get_conn().await.map_err(|err| {
            error!("error fetching connection guard: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        let state = state.clone();
        tokio::spawn(async move {
            let tx = match Transaction::new_and_assume_ready(hash, &conn).await {
                Ok(tx) => tx,
                Err(err) => {
                    error!("error processing tx: {:?}", err);
                    return;
                }
            };
            if let Err(e) =
                evm::cache_ethereum_transaction(&tx, &state.db, EnumBlockChain::EthereumMainnet)
                    .await
            {
                error!("error caching transaction: {:?}", e);
            };
            if let Err(e) = parse_dex_trade(
                EnumBlockChain::EthereumMainnet,
                &tx,
                &state.dex_addresses,
                &state.pancake_swap,
            )
            .await
            {
                error!("error parsing dex trade: {:?}", e);
            };
        });
    }

    Ok(())
}
