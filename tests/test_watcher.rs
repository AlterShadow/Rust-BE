use std::net::Ipv4Addr;
use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use eyre::*;
use web3::signing::Key;
use web3::types::U256;

use eth_sdk::erc20::build_erc_20;
use eth_sdk::erc20::Erc20Token;
use eth_sdk::escrow::EscrowContract;
use eth_sdk::escrow_tracker::handle_eth_escrows;
use eth_sdk::evm::AppState;
use eth_sdk::mock_erc20::deploy_mock_erc20;
use eth_sdk::signer::Secp256k1SecretKey;
use eth_sdk::utils::wait_for_confirmations_simple;
use eth_sdk::*;
use gen::database::*;
use gen::model::*;
use lib::config::load_config;
use lib::database::{connect_to_database, database_test_config, drop_and_recreate_database};

// TODO: import Config used in watcher/main.rs
use lib::config::WsServerConfig;
use lib::database::DatabaseConfig;
use lib::log::LogLevel;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub app_db: DatabaseConfig,
    pub auth_db: DatabaseConfig,
    #[serde(default)]
    pub log_level: LogLevel,
    #[serde(flatten)]
    pub app: WsServerConfig,
    pub ethereum_urls: EthereumConns,
    #[serde(default)]
    pub setup_ethereum_localnet: bool,
    pub escrow_addresses: Vec<UserGetDepositAddressesRow>,
}

#[tokio::test]
async fn test_handle_eth_escrows() -> Result<()> {
    drop_and_recreate_database()?;
    let conn_pool = EthereumRpcConnectionPool::new();
    let conn = conn_pool.get(EnumBlockChain::LocalNet).await?;
    let db = connect_to_database(database_test_config()).await?;
    let secure_eoa_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)
        .context("failed to parse anvil private key")?;
    let user_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)
        .context("failed to parse anvil private key")?;

    println!("dropped and recreated database");

    /* create user */
    let signup_ret = db
        .execute(FunAuthSignupReq {
            address: format!("{:?}", user_key.address()),
            email: "".to_string(),
            phone: "".to_string(),
            preferred_language: "".to_string(),
            agreed_tos: true,
            agreed_privacy: true,
            ip_address: Ipv4Addr::new(127, 0, 0, 1).into(),
            username: Some("TEST".to_string()),
            age: None,
            public_id: 1,
        })
        .await?
        .into_result()
        .context("no user signup resp")?;

    /* deploy escrow contract */
    let fake_escrow_contract = EscrowContract::deploy(conn.clone(), secure_eoa_key.clone()).await?;

    /* deploy fake USDC contract */
    let fake_usdc_contract = deploy_mock_erc20(conn.clone(), secure_eoa_key.clone()).await?;

    /* mint fake USDC for user */
    let mint_hash = fake_usdc_contract
        .mint(
            &conn,
            secure_eoa_key.clone(),
            user_key.address(),
            U256::from_dec_str("1000000000000000")?,
        )
        .await?;
    wait_for_confirmations_simple(&conn.clone().eth(), mint_hash, Duration::from_secs(10), 10)
        .await?;

    /* user transfer fake USDC to escrow contract */
    let transfer_hash = fake_usdc_contract
        .transfer(
            &conn,
            user_key.clone(),
            fake_escrow_contract.address(),
            U256::from_dec_str("1000000000000000")?,
        )
        .await?;
    wait_for_confirmations_simple(
        &conn.clone().eth(),
        transfer_hash,
        Duration::from_secs(10),
        10,
    )
    .await?;

    /* fake busd in blockchain coin addresses table */
    let mut fake_token_addresses = BlockchainCoinAddresses::empty();
    fake_token_addresses.insert(
        EnumBlockChain::LocalNet,
        EnumBlockchainCoin::USDC,
        fake_usdc_contract.address,
    );

    /* fake escrow address */
    let mut fake_escrow_addresses = EscrowAddresses::empty();
    fake_escrow_addresses.insert(EnumBlockChain::LocalNet, fake_escrow_contract.address());

    /* fake AppState */
    let config: Config = load_config("watcher".to_owned())?;
    let eth_pool = EthereumRpcConnectionPool::from_conns(config.ethereum_urls);
    // let db = connect_to_database(config.app_db).await?;
    let fake_app_state = AppState {
        dex_addresses: DexAddresses::new(),
        eth_pool,
        erc_20: build_erc_20()?,
        pancake_swap: build_pancake_swap()?,
        stablecoin_addresses: fake_token_addresses,
        escrow_addresses: fake_escrow_addresses,
        db: db.clone(),
    };

    /* fake QuickAlert payload body */
    let fake_payload_hashes = vec![transfer_hash];
    let fake_payload_json = serde_json::to_string(&fake_payload_hashes)?;
    let fake_payload = Bytes::from(fake_payload_json);

    /* handle escrow */
    match handle_eth_escrows(
        Arc::new(fake_app_state),
        fake_payload,
        EnumBlockChain::LocalNet,
    )
    .await
    {
        Ok(_) => {}
        Err(e) => {
            println!("handle_eth_escrows error: {:?}", e);
            assert!(false);
        }
    }

    /* check database for user_deposit_history */
    let resp = db
        .execute(FunUserListDepositHistoryReq {
            user_id: signup_ret.user_id,
            limit: 1,
            offset: 0,
        })
        .await?
        .into_result()
        .ok_or_else(|| eyre!("no user deposit history"))?;

    assert_eq!(resp.blockchain, EnumBlockChain::LocalNet);
    assert_eq!(resp.quantity, "1000000000000000".to_string());
    assert_eq!(resp.transaction_hash, format!("{:?}", transfer_hash));
    assert_eq!(
        resp.contract_address,
        format!("{:?}", fake_usdc_contract.address)
    );
    assert_eq!(
        resp.receiver_address,
        format!("{:?}", fake_escrow_contract.address())
    );
    assert_eq!(resp.user_address, format!("{:?}", user_key.address()));

    Ok(())
}

#[tokio::test]
async fn test_handle_eth_escrows_testnet() -> Result<()> {
    drop_and_recreate_database()?;
    let conn_pool = EthereumRpcConnectionPool::new();
    let conn = conn_pool.get(EnumBlockChain::BscTestnet).await?;
    let token_addresses = BlockchainCoinAddresses::new();
    let db = connect_to_database(database_test_config()).await?;
    let user_key = Secp256k1SecretKey::from_str(DEV_ACCOUNT_PRIV_KEY)
        .context("failed to parse dev account private key")?;
    let busd_address_on_bsc_testnet = token_addresses
        .get(EnumBlockChain::BscTestnet, EnumBlockchainCoin::BUSD)
        .ok_or_else(|| eyre!("could not find USDC address on BSC Testnet"))?;

    /* create user */
    let signup_ret = db
        .execute(FunAuthSignupReq {
            address: format!("{:?}", user_key.address()),
            email: "".to_string(),
            phone: "".to_string(),
            preferred_language: "".to_string(),
            agreed_tos: true,
            agreed_privacy: true,
            ip_address: Ipv4Addr::new(127, 0, 0, 1).into(),
            username: Some("TEST".to_string()),
            age: None,
            public_id: 1,
        })
        .await?
        .into_result()
        .context("no user signup resp")?;

    /* get escrow contract addresses */
    let escrow_address_bsc_testnet = EscrowAddresses::new()
        .get(EnumBlockChain::BscTestnet)
        .ok_or_else(|| eyre!("could not find escrow contract address on bsc testnet"))?;

    /* instantiate Erc20Token for BUSD contract address */
    let busd_contract = Erc20Token::new(conn.clone(), busd_address_on_bsc_testnet)?;

    /* transfer BUSD from dev wallet to escrow contract on bsc testnet */
    /* make sure dev wallet has enough BUSD */
    let transfer_hash = busd_contract
        .transfer(
            &conn,
            user_key.clone(),
            escrow_address_bsc_testnet,
            U256::from(1000),
        )
        .await?;
    wait_for_confirmations_simple(
        &conn.clone().eth(),
        transfer_hash,
        Duration::from_secs(10),
        10,
    )
    .await?;

    /* fake QuickAlert payload body */
    let fake_payload_hashes = vec![transfer_hash];
    let fake_payload_json = serde_json::to_string(&fake_payload_hashes)?;
    let fake_payload = Bytes::from(fake_payload_json);

    /* handle escrow */
    let config: Config = load_config("watcher".to_owned())?;
    let eth_pool = EthereumRpcConnectionPool::from_conns(config.ethereum_urls);
    match handle_eth_escrows(
        Arc::new(AppState::new(db.clone(), eth_pool)?),
        fake_payload,
        EnumBlockChain::BscTestnet,
    )
    .await
    {
        Ok(_) => {}
        Err(e) => {
            println!("handle_eth_escrows error: {:?}", e);
            assert!(false);
        }
    }

    /* check database for user_deposit_history */
    let resp = db
        .execute(FunUserListDepositHistoryReq {
            user_id: signup_ret.user_id,
            limit: 1,
            offset: 0,
        })
        .await?
        .into_result()
        .ok_or_else(|| eyre!("no user deposit history"))?;

    assert_eq!(resp.blockchain, EnumBlockChain::BscTestnet);
    assert_eq!(resp.quantity, "1000".to_string());
    assert_eq!(resp.transaction_hash, format!("{:?}", transfer_hash));
    assert_eq!(
        resp.contract_address,
        format!("{:?}", busd_address_on_bsc_testnet)
    );
    assert_eq!(
        resp.receiver_address,
        format!("{:?}", escrow_address_bsc_testnet)
    );
    assert_eq!(resp.user_address, format!("{:?}", user_key.address()));

    Ok(())
}
