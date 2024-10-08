pub mod method;

use api::cmc::CoinMarketCap;
use eth_sdk::erc20::build_erc_20;
use eth_sdk::pancake_swap::parse::{get_pancake_swap_parser, PancakeSwapParser};
use eth_sdk::signer::Secp256k1SecretKey;
use eth_sdk::{
    BlockchainCoinAddresses, DexAddresses, EscrowAddresses, EthereumRpcConnectionPool,
    StrategyPoolHeraldAddresses, StrategyWalletHeraldAddresses,
};
use eyre::*;
use lib::database::DbClient;
use lib::ws::WsClient;
use mc2fi_user::shared_method::load_escrow_address;
use std::sync::Arc;
use tokio::sync::Mutex;
use web3::ethabi::Contract;

pub struct AppState {
    pub dex_addresses: DexAddresses,
    pub eth_pool: EthereumRpcConnectionPool,
    pub pancake_swap_parser: &'static PancakeSwapParser,
    pub db: DbClient,
    pub token_addresses: Arc<BlockchainCoinAddresses>,
    pub escrow_addresses: Arc<EscrowAddresses>,
    pub pool_herald_addresses: Arc<StrategyPoolHeraldAddresses>,
    pub wallet_herald_addresses: Arc<StrategyWalletHeraldAddresses>,
    pub erc_20: Contract,
    pub master_key: Secp256k1SecretKey,
    pub admin_client: Option<Mutex<WsClient>>,
    pub cmc_client: CoinMarketCap,
}
impl AppState {
    pub async fn new(
        db: DbClient,
        eth_pool: EthereumRpcConnectionPool,
        master_key: Secp256k1SecretKey,
        admin_client: WsClient,
        cmc_client: CoinMarketCap,
        token_addresses: Arc<BlockchainCoinAddresses>,
    ) -> Result<Self> {
        Ok(Self {
            dex_addresses: DexAddresses::new(),
            eth_pool,
            pancake_swap_parser: get_pancake_swap_parser(),
            escrow_addresses: load_escrow_address(&db).await?,
            db,
            token_addresses,
            erc_20: build_erc_20()?,
            master_key,
            admin_client: Some(Mutex::new(admin_client)),
            cmc_client,
            pool_herald_addresses: Arc::new(StrategyPoolHeraldAddresses::new()),
            wallet_herald_addresses: Arc::new(StrategyWalletHeraldAddresses::new()),
        })
    }
}
