use axum::{
    body::{Body, Bytes},
    extract::State,
    http::StatusCode,
    routing::post,
    Router,
};
use eyre::*;
use lib::config::{load_config, WsServerConfig};
use lib::database::DatabaseConfig;
use lib::log::{setup_logs, LogLevel};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Cursor;
use std::str::FromStr;
use std::sync::Arc;
use tracker::trade::{Chain, Dex};
use web3::types::{H160, H256};

pub mod rpc_provider;
pub mod tracker;

use rpc_provider::pool::ConnectionPool;
use tracker::{
    ethabi_to_web3::convert_h256_ethabi_to_web3,
    pancake_swap::PancakeSwap,
    tx::{Tx, TxStatus},
};

#[derive(Clone, Debug)]
struct AppState {
    dex_addresses: Arc<HashMap<Chain, Vec<(Dex, H160)>>>,
    eth_pool: Arc<ConnectionPool>,
    pancake_swap: PancakeSwap,
}

const PANCAKE_SMART_ROUTER_PATH: &str = "abi/pancake_swap/smart_router_v3.json";
const ERC20_PATH: &str = "abi/generic/erc20.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub app_db: DatabaseConfig,
    #[serde(default)]
    pub log_level: LogLevel,
    pub eth_provider_url: String,

    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: u16,
}
#[tokio::main]
async fn main() -> Result<()> {
    let config: Config = load_config("trade_watcher".to_owned())?;
    setup_logs(config.log_level)?;

    let mut dexes: HashMap<Chain, Vec<(Dex, H160)>> = HashMap::new();
    /* load relevant addresses on startup */
    dexes.insert(
        Chain::EthereumMainnet,
        vec![(
            Dex::PancakeSwap,
            H160::from_str("0x13f4EA83D0bd40E75C8222255bc855a974568Dd4").unwrap(),
        )],
    );
    dexes.insert(
        Chain::BscMainnet,
        vec![(
            Dex::PancakeSwap,
            H160::from_str("0x13f4EA83D0bd40E75C8222255bc855a974568Dd4").unwrap(),
        )],
    );
    dexes.insert(
        Chain::EthereumGoerli,
        vec![(
            Dex::PancakeSwap,
            H160::from_str("0x9a489505a00cE272eAa5e07Dba6491314CaE3796").unwrap(),
        )],
    );
    dexes.insert(
        Chain::BscTestnet,
        vec![(
            Dex::PancakeSwap,
            H160::from_str("0x9a489505a00cE272eAa5e07Dba6491314CaE3796").unwrap(),
        )],
    );

    let pancake_smart_router = ethabi::Contract::load(Cursor::new(
        std::fs::read(PANCAKE_SMART_ROUTER_PATH).expect("failed to read contract ABI"),
    ))
    .expect("failed to parse contract ABI");
    let erc20 = ethabi::Contract::load(Cursor::new(
        std::fs::read(ERC20_PATH).expect("failed to read contract ABI"),
    ))
    .expect("failed to parse contract ABI");

    let transfer_event_signature = convert_h256_ethabi_to_web3(
        erc20
            .event("Transfer")
            .expect("Failed to get Transfer event signature")
            .signature(),
    );
    let eth_pool = ConnectionPool::new(config.eth_provider_url.to_string(), 100, 300, 10).await?;
    let app: Router<(), Body> = Router::new()
        .route("/eth-mainnet-swaps", post(handle_eth_swap))
        .with_state(AppState {
            dex_addresses: Arc::new(dexes),
            eth_pool,
            pancake_swap: PancakeSwap::new(pancake_smart_router, transfer_event_signature),
        });

    let addr = tokio::net::lookup_host((config.host.as_ref(), config.port))
        .await?
        .next()
        .context("failed to resolve address")?;

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn handle_eth_swap(State(state): State<AppState>, body: Bytes) -> Result<(), StatusCode> {
    let hashes = parse_quickalert_payload(body).map_err(|e| {
        println!("failed to parse QuickAlerts payload: {:?}", e);
        StatusCode::BAD_REQUEST
    })?;

    let eth = match state.eth_pool.clone().get_conn().await {
        Ok(eth) => eth,
        Err(e) => {
            println!("error fetching connection guard: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    for hash in hashes {
        let eth = eth.clone();
        let state = state.clone();
        tokio::spawn(async move {
            let tx = Tx::new(hash, eth.clone()).await;

            match tx.get_status() {
                TxStatus::Successful => (),
                TxStatus::Pending => {
                    println!("transaction is pending");
                    /* TODO: handle pending transaction */
                    return Err(StatusCode::UNPROCESSABLE_ENTITY);
                }
                _ => {
                    println!("transaction failed");
                    return Err(StatusCode::UNPROCESSABLE_ENTITY);
                }
            }

            let contract_address = match tx.get_to() {
                Some(address) => address,
                None => {
                    println!("transaction has no contract address");
                    return Err(StatusCode::UNPROCESSABLE_ENTITY);
                }
            };

            let eth_mainnet_dexes = state.dex_addresses.get(&Chain::EthereumMainnet).unwrap();

            for dex in eth_mainnet_dexes {
                let (dex, address) = dex;
                if *address == contract_address {
                    let trade = match dex {
                        Dex::PancakeSwap => {
                            state.pancake_swap.get_trade(&tx, Chain::EthereumMainnet)
                        }
                        Dex::UniSwap => return Ok(()),
                        Dex::SushiSwap => return Ok(()),
                    };
                    println!();
                    println!("tx: {:?}", tx.get_id().unwrap());
                    println!("trade: {:?}", trade);
                    println!();
                }
            }
            Ok(())
        });
    }

    Ok(())
}

fn parse_quickalert_payload(payload: Bytes) -> Result<Vec<H256>> {
    let result: Result<Vec<H256>, _> = serde_json::from_slice(&payload);

    match result {
        Ok(hashes) => Ok(hashes),
        Err(e) => Err(e.into()),
    }
}
