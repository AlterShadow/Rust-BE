use axum::extract::State;
use axum::http::StatusCode;
use axum::{body::Body, routing::post, Router};
use axum_server::tls_rustls::RustlsConfig;
use bytes::Bytes;
use chrono::Utc;
use eth_sdk::dex_tracker::*;
use eth_sdk::erc20::{approve_and_ensure_success, build_erc_20, Erc20Token};
use eth_sdk::escrow_tracker::escrow::parse_escrow;
use eth_sdk::evm::parse_quickalert_payload;
use eth_sdk::signer::Secp256k1SecretKey;
use eth_sdk::strategy_pool::{
    acquire_asset_before_trade_and_ensure_success, give_back_assets_after_trade_and_ensure_success,
    StrategyPoolContract,
};
use eth_sdk::utils::{get_signed_text, wait_for_confirmations_simple};
use eth_sdk::v3::smart_router::{copy_trade_and_ensure_success, PancakeSmartRouterV3Contract};
use eth_sdk::{
    build_pancake_swap, evm, BlockchainCoinAddresses, DexAddresses, EscrowAddresses, EthereumConns,
    EthereumRpcConnectionPool, PancakeSwap, ScaledMath, TransactionFetcher, TransactionReady,
};
use eyre::*;
use gen::database::*;
use gen::model::*;
use lib::config::load_config;
use lib::database::{connect_to_database, DatabaseConfig, DbClient};
use lib::log::{setup_logs, LogLevel};
use lib::utils::encode_header;
use lib::ws::WsClient;
use mc2fi_auth::endpoints::*;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tracing::*;
use web3::ethabi::{Address, Contract};
use web3::signing::Key;
use web3::types::U256;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub app_db: DatabaseConfig,
    #[serde(default)]
    pub log_level: LogLevel,

    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: u16,
    #[serde(default)]
    pub pub_cert: Option<String>,
    #[serde(default)]
    pub priv_cert: Option<String>,
    pub ethereum_urls: EthereumConns,
    pub god_key: SecretString,
    pub user_url: String,
    pub auth_url: String,
}
pub struct AppState {
    pub dex_addresses: DexAddresses,
    pub eth_pool: EthereumRpcConnectionPool,
    pub pancake_swap: PancakeSwap,
    pub db: DbClient,
    pub token_addresses: BlockchainCoinAddresses,
    pub escrow_addresses: EscrowAddresses,
    pub erc_20: Contract,
    pub master_key: Secp256k1SecretKey,
    pub admin_client: Mutex<WsClient>,
}
impl AppState {
    pub fn new(
        db: DbClient,
        eth_pool: EthereumRpcConnectionPool,
        master_key: Secp256k1SecretKey,
        admin_client: WsClient,
    ) -> Result<Self> {
        Ok(Self {
            dex_addresses: DexAddresses::new(),
            eth_pool,
            pancake_swap: build_pancake_swap()?,
            db,
            token_addresses: BlockchainCoinAddresses::new(),
            erc_20: build_erc_20()?,
            escrow_addresses: EscrowAddresses::new(),
            master_key,
            admin_client: Mutex::new(admin_client),
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config: Config = load_config("watcher".to_owned())?;
    setup_logs(config.log_level)?;

    let master_key = Secp256k1SecretKey::from_str(config.god_key.expose_secret())?;
    if let Err(err) = signup(&config.auth_url, "dev-watcher", &master_key.key).await {
        error!("failed to signup: {}", err);
    }

    let client = connect_user(
        &config.auth_url,
        &config.user_url,
        "dev-watcher",
        &master_key.key,
    )
    .await?;
    let db = connect_to_database(config.app_db).await?;

    let eth_pool = EthereumRpcConnectionPool::from_conns(config.ethereum_urls);
    let app: Router<(), Body> = Router::new()
        .route("/eth-mainnet-swaps", post(handle_eth_swap_mainnet))
        .route("/eth-goerli-swaps", post(handle_eth_swap_goerli))
        .route("/eth-mainnet-escrows", post(handle_eth_escrows_mainnet))
        .route("/eth-goerli-escrows", post(handle_eth_escrows_goerli))
        .route("/bsc-mainnet-swaps", post(handle_bsc_swap_mainnet))
        .route("/bsc-mainnet-escrows", post(handle_bsc_escrows_mainnet))
        .with_state(Arc::new(AppState::new(db, eth_pool, master_key, client)?));

    let addr = tokio::net::lookup_host((config.host.as_ref(), config.port))
        .await?
        .next()
        .context("failed to resolve address")?;
    info!("Trade watcher listening on {}", addr);
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

pub async fn handle_eth_swap_mainnet(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    handle_eth_swap(state.0, body, EnumBlockChain::EthereumMainnet).await
}

pub async fn handle_eth_swap_goerli(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    handle_eth_swap(state.0, body, EnumBlockChain::EthereumGoerli).await
}

pub async fn handle_bsc_swap_mainnet(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    handle_eth_swap(state.0, body, EnumBlockChain::BscMainnet).await
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

pub async fn handle_bsc_escrows_mainnet(
    state: State<Arc<AppState>>,
    body: Bytes,
) -> Result<(), StatusCode> {
    handle_eth_escrows(state.0, body, EnumBlockChain::BscMainnet).await
}

pub async fn handle_eth_swap(
    state: Arc<AppState>,
    body: Bytes,
    blockchain: EnumBlockChain,
) -> Result<(), StatusCode> {
    let hashes = parse_quickalert_payload(body).map_err(|e| {
        error!("failed to parse QuickAlerts payload: {:?}", e);
        StatusCode::BAD_REQUEST
    })?;

    for hash in hashes {
        let conn = state.eth_pool.get(blockchain).await.map_err(|err| {
            error!("error fetching connection guard: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        let state = state.clone();
        tokio::spawn(async move {
            /* the transactions from the quickalerts payload might not be yet mined */
            match wait_for_confirmations_simple(&conn.eth(), hash, Duration::from_secs(10), 10)
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    error!("swap tx was not mined: {:?}", e);
                    return;
                }
            }
            // TODO: wait for confirmations blocks before processing to properly handle ommer blocks & reorgs
            let tx = match TransactionFetcher::new_and_assume_ready(hash, &conn).await {
                Ok(tx) => tx,
                Err(err) => {
                    error!("error processing tx: {:?}", err);
                    return;
                }
            };
            if let Err(e) = evm::cache_ethereum_transaction(&tx, &state.db, blockchain).await {
                error!("error caching transaction: {:?}", e);
            };
            match handle_swap(state.clone(), blockchain, tx).await {
                Ok(_) => {}
                Err(e) => {
                    error!("error handling swap: {:?}", e);
                }
            }
        });
    }

    Ok(())
}

pub async fn handle_swap(
    state: Arc<AppState>,
    blockchain: EnumBlockChain,
    tx: TransactionReady,
) -> Result<()> {
    /* check if caller is a strategy watching wallet & get strategy id */
    let caller = tx.get_from().context("no from address found")?;
    let strategy_id = get_strategy_id_from_watching_wallet(&state.db, &blockchain, &caller)
        .await
        .context("caller is not a strategy watching wallet")?;

    /* parse trade */
    let trade = parse_dex_trade(blockchain, &tx, &state.dex_addresses, &state.pancake_swap).await?;

    /* get called contract */
    let called_address = tx.get_to().context("no to address found")?;

    /* update wallet activity ledger & make sure this transaction is not a duplicate */
    let saved = state
        .db
        .execute(FunWatcherSaveStrategyWatchingWalletTradeHistoryReq {
            address: format!("{:?}", caller.clone()),
            transaction_hash: format!("{:?}", tx.get_hash()),
            blockchain,
            contract_address: format!("{:?}", called_address),
            dex: Some(EnumDex::PancakeSwap.to_string()),
            token_in_address: Some(format!("{:?}", trade.token_in)),
            token_out_address: Some(format!("{:?}", trade.token_out)),
            amount_in: Some(format!("{:?}", trade.amount_in)),
            amount_out: Some(format!("{:?}", trade.amount_out)),
            happened_at: None,
        })
        .await
        .context("swap transaction is a duplicate")?
        .into_result();

    if let Some(saved) = saved {
        /* check if tokens are known */
        let token_in = state
            .token_addresses
            .get_by_address(blockchain, trade.token_in)
            .context("token in is unknown")?;
        state
            .token_addresses
            .get_by_address(blockchain, trade.token_out)
            .context("token out is unknown")?;

        /* get all strategy tokens */

        let all_strategy_tokens = state
            .db
            .execute(
                // fun_watcher_list_user_strategy_ledger
                FunWatcherListStrategyEscrowPendingWalletLedgerReq {
                    strategy_id: Some(strategy_id),
                },
            )
            .await?;

        /* build up multichain token map */
        let mut strategy_token_ledger: HashMap<EnumBlockchainCoin, U256> = HashMap::new();
        for row in all_strategy_tokens.into_iter() {
            let (token_chain, token_address, token_amount) = (
                row.blockchain,
                row.token_address.parse::<Address>()?,
                row.entry.parse::<U256>()?,
            );
            let strategy_token = state
                .token_addresses
                .get_by_address(token_chain, token_address)
                .context("strategy token is unknown")?;
            if strategy_token_ledger
                .insert(strategy_token, token_amount)
                .is_some()
            {
                bail!(
                    "Duplicate entry in strategy token list for {:?} {:?}",
                    strategy_token,
                    token_address
                );
            }
        }

        /* update database with watched wallet's tokens */
        let conn = state.eth_pool.get(blockchain).await?;
        update_expert_listened_wallet_asset_ledger(
            &state.db,
            strategy_id,
            &trade,
            saved.fkey_token_out,
            saved.fkey_token_in,
            blockchain,
        )
        .await?;

        /* check if token_in was a strategy token */
        if let Some(total_strategy_token_in_amount) = strategy_token_ledger.get(&token_in) {
            /* if token_in was already a strategy token trade it from SPs in ratio traded_amount / old_amount */

            let strategy_pool = state
                .db
                .execute(FunWatcherListStrategyPoolContractReq {
                    limit: 1,
                    offset: 0,
                    strategy_id: Some(strategy_id),
                    blockchain: Some(blockchain),
                    address: None,
                })
                .await?
                .into_result();
            /* if there is an SP contract for this strategy,  */
            if let Some(address_row) = strategy_pool {
                let address = address_row.address;
                /* check if SP contract holds token_in */
                let sp_contract =
                    StrategyPoolContract::new(conn.clone(), Address::from_str(&address)?)?;
                let mut maybe_sp_token_in_amount: Option<U256> = None;
                let mut max_retries = 10;
                while maybe_sp_token_in_amount.is_none() && max_retries > 0 {
                    match sp_contract.asset_balance(trade.token_in).await {
                        Ok(token_in_amount) => {
                            maybe_sp_token_in_amount = Some(token_in_amount);
                        }
                        Err(_) => {
                            /* if we can't query the contract's assets, it's because it is currently trading */
                            /* wait a bit and try again */
                            sleep(Duration::from_secs(30)).await;
                            max_retries -= 1;
                        }
                    }
                }
                let sp_token_in_amount = maybe_sp_token_in_amount
                    .ok_or_else(|| eyre!("failed to query strategy pool token_in amount"))?;

                if sp_token_in_amount == U256::zero() {
                    bail!("strategy pool has no token_in");
                }

                /* calculate how much to spend */
                let amount_to_spend = trade
                    .amount_in
                    .mul_div(sp_token_in_amount, *total_strategy_token_in_amount)?;
                if amount_to_spend == U256::zero() {
                    bail!("spent ratio is too small to be represented in amount of token_in owned by strategy pool");
                }

                /* instantiate token_in and token_out contracts */
                let token_in_contract = Erc20Token::new(conn.clone(), trade.token_in)?;
                let token_out_contract = Erc20Token::new(conn.clone(), trade.token_out)?;

                /* instantiate pancake swap contract */
                let pancake_contract = PancakeSmartRouterV3Contract::new(
                    conn.clone(),
                    state
                        .dex_addresses
                        .get(blockchain, EnumDex::PancakeSwap)
                        .ok_or_else(|| eyre!("pancake swap not available on this chain"))?,
                )?;

                acquire_asset_before_trade_and_ensure_success(
                    sp_contract.clone(),
                    &conn,
                    12,
                    10,
                    Duration::from_secs(10),
                    state.master_key.clone(),
                    trade.token_in,
                    amount_to_spend,
                )
                .await?;

                /* approve pancakeswap to trade token_in */
                approve_and_ensure_success(
                    token_in_contract,
                    &conn,
                    12,
                    10,
                    Duration::from_secs(10),
                    state.master_key.clone(),
                    pancake_contract.address(),
                    amount_to_spend,
                )
                .await?;

                /* trade token_in for token_out */
                let trade_hash = copy_trade_and_ensure_success(
                    pancake_contract,
                    &conn,
                    12,
                    10,
                    Duration::from_secs(10),
                    state.master_key.clone(),
                    trade.get_pancake_pair_paths()?,
                    amount_to_spend,
                    U256::from(1),
                )
                .await?;

                /* parse trade to find amount_out */
                let sp_trade = parse_dex_trade(
                    blockchain,
                    &TransactionFetcher::new_and_assume_ready(trade_hash, &conn).await?,
                    &state.dex_addresses,
                    &state.pancake_swap,
                )
                .await?;

                /* approve strategy pool for amount_out */
                approve_and_ensure_success(
                    token_out_contract,
                    &conn,
                    12,
                    10,
                    Duration::from_secs(10),
                    state.master_key.clone(),
                    sp_contract.address(),
                    sp_trade.amount_out,
                )
                .await?;

                /* give back traded assets */
                give_back_assets_after_trade_and_ensure_success(
                    sp_contract,
                    &conn,
                    12,
                    10,
                    Duration::from_secs(10),
                    state.master_key.clone(),
                    vec![trade.token_out],
                    vec![sp_trade.amount_out],
                )
                .await?;
            }
        }
    }

    Ok(())
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
        let conn = state.eth_pool.get(blockchain).await.map_err(|err| {
            error!("error fetching connection guard: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        let state = state.clone();
        tokio::spawn(async move {
            /* the transactions from the quickalerts payload might not be yet mined */
            match wait_for_confirmations_simple(&conn.eth(), hash, Duration::from_secs(10), 10)
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    error!("escrow tx was not mined: {:?}", e);
                    return;
                }
            }
            // TODO: wait for confirmations blocks before processing to properly handle ommer blocks & reorgs
            let tx = match TransactionFetcher::new_and_assume_ready(hash, &conn).await {
                Ok(tx) => tx,
                Err(err) => {
                    error!("error processing tx: {:?}", err);
                    return;
                }
            };
            if let Err(e) = evm::cache_ethereum_transaction(&tx, &state.db, blockchain).await {
                error!("error caching transaction: {:?}", e);
            };

            /* check if it is an escrow to one of our escrow contracts */
            let escrow = match parse_escrow(blockchain, &tx, &state.token_addresses, &state.erc_20)
            {
                Ok(escrow) => escrow,
                Err(e) => {
                    info!("tx {:?} is not an escrow: {:?}", tx.get_hash(), e);
                    return;
                }
            };
            if state
                .escrow_addresses
                .get_by_address(blockchain, escrow.recipient)
                .is_none()
            {
                warn!(
                    "no transfer to an escrow contract for tx: {:?}",
                    tx.get_hash()
                );
                return;
            }

            /* check if transaction is from one of our users */
            // TODO: handle an escrow made by an unknown user
            let caller = match tx.get_from() {
                Some(caller) => caller,
                None => {
                    error!("no caller found for tx: {:?}", tx.get_hash());
                    return;
                }
            };

            let user = match state
                .db
                .execute(FunUserGetUserByAddressReq {
                    address: format!("{:?}", caller),
                })
                .await
            {
                Ok(user) => match user.into_result() {
                    Some(user) => user,
                    None => {
                        info!("no user has address: {:?}", caller);
                        return;
                    }
                },
                Err(e) => {
                    error!("error getting user by address: {:?}", e);
                    return;
                }
            };

            /* get token address that was transferred */
            let called_address = match tx.get_to() {
                Some(called_address) => called_address,
                None => {
                    error!("no called address found for tx: {:?}", tx.get_hash());
                    return;
                }
            };

            /* insert escrow in ledger */
            match state
                .db
                .execute(FunUserDepositToEscrowReq {
                    user_id: user.user_id,
                    quantity: format!("{:?}", escrow.amount),
                    blockchain,
                    user_address: format!("{:?}", escrow.owner),
                    contract_address: format!("{:?}", called_address),
                    transaction_hash: format!("{:?}", tx.get_hash()),
                    receiver_address: format!("{:?}", escrow.recipient),
                })
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    error!("error inserting escrow in ledger: {:?}", e);
                    return;
                }
            };
            if let Err(err) = state
                .admin_client
                .lock()
                .await
                .request(AdminNotifyEscrowLedgerChangeRequest {
                    pkey_id: 0,
                    user_id: user.user_id,
                    entry: UserListDepositHistoryRow {
                        quantity: format!("{:?}", escrow.amount),
                        blockchain,
                        user_address: format!("{:?}", escrow.owner),
                        contract_address: format!("{:?}", called_address),
                        transaction_hash: format!("{:?}", tx.get_hash()),
                        receiver_address: format!("{:?}", escrow.recipient),
                        created_at: Utc::now().timestamp(),
                    },
                })
                .await
            {
                error!("error notifying admin of escrow ledger change: {:?}", err);
            }
        });
    }

    Ok(())
}

pub async fn signup(
    url: &str,
    username: impl Into<String>,
    signer: impl Key + Clone,
) -> Result<()> {
    let username = username.into();
    let (txt, sig) = get_signed_text(format!("Signup {}", username), signer.clone())?;

    let mut client = get_ws_auth_client(
        url,
        &encode_header(
            SignupRequest {
                address: format!("{:?}", signer.address()),
                signature_text: txt,
                signature: sig,
                email: "qjk2001@gmail.com".to_string(),
                phone: "+00123456".to_string(),
                agreed_tos: true,
                agreed_privacy: true,
                username,
            },
            endpoint_auth_signup(),
        )?,
    )
    .await?;
    let res: SignupResponse = client.recv_resp().await?;
    info!("{:?}", res);
    Ok(())
}
pub async fn login(
    url: &str,
    username: impl Into<String>,
    signer: impl Key + Clone,
) -> Result<LoginResponse> {
    let username = username.into();

    let (txt, sig) = get_signed_text(format!("Login {}", username), signer.clone())?;
    let mut client = get_ws_auth_client(
        url,
        &encode_header(
            LoginRequest {
                address: format!("{:?}", signer.address()),
                signature_text: txt,
                signature: sig,
                service: EnumService::User as _,
                device_id: "24787297130491616".to_string(),
                device_os: "android".to_string(),
            },
            endpoint_auth_login(),
        )?,
    )
    .await?;
    let res: LoginResponse = client.recv_resp().await?;
    info!("{:?}", res);
    Ok(res)
}

pub async fn get_ws_auth_client(url: &str, header: &str) -> Result<WsClient> {
    info!("Connecting to {} with header {}", url, header);
    let ws_stream = WsClient::new(url, header).await?;
    Ok(ws_stream)
}
pub async fn auth_login(url: &str, req: &LoginRequest) -> Result<LoginResponse> {
    let header = encode_header(req, endpoint_auth_login())?;
    let mut client = get_ws_auth_client(url, &header).await?;
    let resp: LoginResponse = client.recv_resp().await?;
    Ok(resp)
}

pub async fn get_ws_user_client(url: &str, req: &AuthorizeRequest) -> Result<WsClient> {
    let header = &encode_header(req, endpoint_auth_authorize())?;

    info!("Connecting to {} with header {}", url, header);
    let mut ws_stream = WsClient::new(url, header).await?;
    let x: AuthorizeResponse = ws_stream.recv_resp().await?;
    info!("AuthorizeResponse {:?}", x);
    Ok(ws_stream)
}

pub async fn connect_user(
    auth_url: &str,
    user_url: &str,
    username: impl Into<String>,
    signer: impl Key + Clone,
) -> Result<WsClient> {
    let login = login(auth_url, username, signer.clone()).await?;
    let client = get_ws_user_client(
        user_url,
        &AuthorizeRequest {
            address: login.address,
            token: login.user_token,
            service: EnumService::User as _,
            device_id: "24787297130491616".to_string(),
            device_os: "android".to_string(),
        },
    )
    .await?;
    Ok(client)
}
