use axum::{
    body::{Body, Bytes},
    extract::State,
    http::StatusCode,
    routing::post,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use eyre::*;
use gen::database::FunWatcherSaveRawTransactionReq;
use lib::config::load_config;
use lib::database::{connect_to_database, DatabaseConfig, DbClient};
use lib::log::{setup_logs, LogLevel};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Cursor;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{error, info};
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

#[derive(Clone)]
struct AppState {
    dex_addresses: Arc<HashMap<Chain, Vec<(Dex, H160)>>>,
    eth_pool: Arc<ConnectionPool>,
    pancake_swap: PancakeSwap,
    db: DbClient,
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
    #[serde(default)]
    pub pub_cert: Option<String>,
    #[serde(default)]
    pub priv_cert: Option<String>,
}
#[tokio::main]
async fn main() -> Result<()> {
    let config: Config = load_config("watcher".to_owned())?;
    setup_logs(config.log_level)?;
    let db = connect_to_database(config.app_db).await?;
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
        std::fs::read(PANCAKE_SMART_ROUTER_PATH).context("failed to read contract ABI")?,
    ))
    .context("failed to parse contract ABI")?;
    let erc20 = ethabi::Contract::load(Cursor::new(
        std::fs::read(ERC20_PATH).context("failed to read contract ABI")?,
    ))
    .context("failed to parse contract ABI")?;

    let transfer_event_signature = convert_h256_ethabi_to_web3(
        erc20
            .event("Transfer")
            .context("Failed to get Transfer event signature")?
            .signature(),
    );
    let eth_pool = ConnectionPool::new(config.eth_provider_url.to_string(), 10).await?;
    let app: Router<(), Body> = Router::new()
        .route("/eth-mainnet-swaps", post(handle_eth_swap))
        .with_state(AppState {
            dex_addresses: Arc::new(dexes),
            eth_pool,
            pancake_swap: PancakeSwap::new(pancake_smart_router, transfer_event_signature),
            db,
        });

    let addr = tokio::net::lookup_host((config.host.as_ref(), config.port))
        .await?
        .next()
        .context("failed to resolve address")?;
    info!("Watcher listening on {}", addr);
    if let (Some(pub_cert), Some(priv_key)) = (config.pub_cert, config.priv_cert) {
        // configure certificate and private key used by https
        let config = RustlsConfig::from_pem_file(pub_cert, priv_key).await?;
        axum_server::bind_rustls(addr, config)
            .serve(app.into_make_service())
            .await?;
    } else {
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;
    }

    Ok(())
}

async fn handle_eth_swap(State(state): State<AppState>, body: Bytes) -> Result<(), StatusCode> {
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
        let db = state.db.clone();
        tokio::spawn(async move {
            let err = async {
                let mut tx = Tx::new(hash);
                tx.update(&conn).await?;
                if let Err(err) = {
                    if let Some(content) = tx.get_transaction() {
                        db.execute(FunWatcherSaveRawTransactionReq {
                            transaction_hash: format!("{:?}", hash),
                            chain: "ethereum".to_string(),
                            dex: None,
                            raw_transaction: serde_json::to_string(content)
                                .context("transaction")?,
                        })
                        .await?;
                    }
                    Ok::<_, Error>(())
                } {
                    error!("failed to save raw transaction: {}", err);
                }
                match tx.get_status() {
                    TxStatus::Successful => (),
                    TxStatus::Pending => {
                        /* TODO: handle pending transaction */
                        bail!("transaction is pending: {:?}", hash);
                    }
                    err => {
                        bail!("transaction failed: {:?}", err);
                    }
                }

                let contract_address = match tx.get_to() {
                    Some(address) => address,
                    None => {
                        bail!("transaction has no contract address: {:?}", hash);
                    }
                };

                let eth_mainnet_dexes = state.dex_addresses.get(&Chain::EthereumMainnet).unwrap();

                for (dex, address) in eth_mainnet_dexes {
                    if *address == contract_address {
                        let trade = match dex {
                            Dex::PancakeSwap => {
                                state.pancake_swap.get_trade(&tx, Chain::EthereumMainnet)
                            }
                            Dex::UniSwap => {
                                error!("does not support dex type: UniSwap");
                                continue;
                            }
                            Dex::SushiSwap => {
                                error!("does not support dex type: SushiSwap");
                                continue;
                            }
                        };
                        info!("tx: {:?}", tx.get_id().unwrap());
                        info!("trade: {:?}", trade);
                    }
                }
                Ok(())
            }
            .await;
            if let Err(err) = err {
                error!("error processing tx: {:?}", err);
            }
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
