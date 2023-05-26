use crate::escrow_tracker::escrow::parse_escrow;
use crate::evm::{parse_quickalert_payload, StableCoin};
use crate::{evm, AppState};
use axum::extract::State;
use axum::http::StatusCode;
use bytes::Bytes;
use eth_sdk::Transaction;
use eyre::*;

use gen::model::EnumBlockChain;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::Arc;
use tracing::error;
use web3::types::{Address, H160};

pub mod deposit;
pub mod escrow;

pub struct StableCoinAddresses {
    inner: HashMap<EnumBlockChain, Vec<(StableCoin, H160)>>,
}

impl Default for StableCoinAddresses {
    fn default() -> Self {
        let mut this = StableCoinAddresses {
            inner: HashMap::new(),
        };

        this.inner.insert(
            EnumBlockChain::EthereumMainnet,
            vec![
                (
                    StableCoin::Usdc,
                    H160::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap(),
                ),
                (
                    StableCoin::Usdt,
                    H160::from_str("0xdac17f958d2ee523a2206206994597c13d831ec7").unwrap(),
                ),
                (
                    StableCoin::Busd,
                    H160::from_str("0x4Fabb145d64652a948d72533023f6E7A623C7C53").unwrap(),
                ),
            ],
        );
        this.inner.insert(
            EnumBlockChain::BscMainnet,
            vec![
                (
                    StableCoin::Usdc,
                    H160::from_str("0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d").unwrap(),
                ),
                (
                    StableCoin::Usdt,
                    H160::from_str("0x55d398326f99059ff775485246999027b3197955").unwrap(),
                ),
                (
                    StableCoin::Busd,
                    H160::from_str("0xe9e7cea3dedca5984780bafc599bd69add087d56").unwrap(),
                ),
            ],
        );
        this.inner.insert(
            EnumBlockChain::EthereumGoerli,
            vec![(
                StableCoin::Usdc,
                H160::from_str("0x07865c6E87B9F70255377e024ace6630C1Eaa37F").unwrap(),
            )],
        );
        this.inner.insert(
            EnumBlockChain::BscTestnet,
            vec![(
                StableCoin::Busd,
                H160::from_str("0xaB1a4d4f1D656d2450692D237fdD6C7f9146e814").unwrap(),
            )],
        );

        this
    }
}

impl StableCoinAddresses {
    pub fn new_from_addresses(
        chains: Vec<EnumBlockChain>,
        coins: Vec<Vec<(StableCoin, H160)>>,
    ) -> Result<Self> {
        if chains.len() != coins.len() {
            return Err(eyre!("chains and coins must have the same length"));
        }

        let mut chain_set = HashSet::new();
        for chain in &chains {
            if !chain_set.insert(*chain) {
                return Err(eyre!("duplicate chain detected"));
            }
        }

        let mut this = StableCoinAddresses {
            inner: HashMap::new(),
        };

        for (chain, chain_coins) in chains.into_iter().zip(coins.into_iter()) {
            let mut coin_set = HashSet::new();
            let mut address_set = HashSet::new();

            for (coin, address) in &chain_coins {
                if !coin_set.insert(*coin) {
                    return Err(eyre!("duplicate coin detected for a chain"));
                }
                if !address_set.insert(*address) {
                    return Err(eyre!("duplicate address detected for a chain"));
                }
            }

            this.inner.insert(chain, chain_coins);
        }

        Ok(this)
    }
    pub fn get(&self, chain: EnumBlockChain) -> Option<&Vec<(StableCoin, Address)>> {
        self.inner.get(&chain)
    }
    pub fn get_by_chain_and_token(
        &self,
        chain: EnumBlockChain,
        coin: StableCoin,
    ) -> Option<Address> {
        let list = self.inner.get(&chain)?;
        list.iter().find(|(x, _)| *x == coin).map(|(_, a)| *a)
    }
}
pub async fn handle_eth_escrows(
    state: Arc<AppState>,
    body: Bytes,
    blockchain: EnumBlockChain,
) -> Result<(), StatusCode> {
    let hashes = parse_quickalert_payload(body).map_err(|e| {
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
            if let Err(e) = evm::cache_ethereum_transaction(&tx, &state.db, blockchain).await {
                error!("error caching transaction: {:?}", e);
            };
            if let Err(e) = parse_escrow(
                EnumBlockChain::EthereumMainnet,
                &tx,
                &state.stablecoin_addresses,
                &state.erc_20,
            ) {
                error!("error parsing escrow trade: {:?}", e);
            };
        });
    }

    Ok(())
}
pub async fn handle_eth_escrows_mainnet(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    handle_eth_escrows(state.0, body, EnumBlockChain::EthereumMainnet).await
}

pub async fn handle_eth_escrows_goerli(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    handle_eth_escrows(state.0, body, EnumBlockChain::EthereumGoerli).await
}
