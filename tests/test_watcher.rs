use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use eyre::*;
use web3::signing::Key;
use web3::types::{Address, H256, U256};

use eth_sdk::dex_tracker::handle_swap;
use eth_sdk::dex_tracker::parse_dex_trade;
use eth_sdk::erc20::{approve_and_ensure_success, build_erc_20, Erc20Token};
use eth_sdk::escrow::EscrowContract;
use eth_sdk::escrow_tracker::handle_eth_escrows;
use eth_sdk::evm::AppState;
use eth_sdk::mock_erc20::deploy_mock_erc20;
use eth_sdk::pair_paths::WorkingPancakePairPaths;
use eth_sdk::signer::Secp256k1SecretKey;
use eth_sdk::strategy_pool::{deposit_and_ensure_success, StrategyPoolContract};
use eth_sdk::utils::wait_for_confirmations_simple;
use eth_sdk::v3::smart_router::copy_trade_and_ensure_success;
use eth_sdk::v3::smart_router::PancakeSmartRouterV3Contract;
use eth_sdk::*;
use gen::database::*;
use gen::model::{EnumBlockChain, EnumBlockchainCoin, EnumDex, UserGetDepositAddressesRow};
use lib::config::load_config;
use lib::database::{connect_to_database, database_test_config, drop_and_recreate_database};

// TODO: import Config used in watcher/main.rs
use lib::config::WsServerConfig;
use lib::database::*;
use lib::log::LogLevel;
use serde::Deserialize;
#[derive(Debug, Clone, Deserialize)]
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
async fn test_handle_eth_escrows_anvil() -> Result<()> {
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
    fake_escrow_addresses.insert(EnumBlockChain::LocalNet, (), fake_escrow_contract.address());

    /* fake AppState */
    let config: Config = load_config("watcher".to_owned())?;
    let eth_pool = EthereumRpcConnectionPool::from_conns(config.ethereum_urls);
    // let db = connect_to_database(config.app_db).await?;
    let fake_app_state = AppState {
        dex_addresses: DexAddresses::new(),
        eth_pool,
        erc_20: build_erc_20()?,
        pancake_swap: build_pancake_swap()?,
        token_addresses: fake_token_addresses,
        escrow_addresses: fake_escrow_addresses,
        db: db.clone(),
        master_key: secure_eoa_key,
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

    // TODO: refactor handler to use an async function
    // TODO: call async function instead of handler directly and remove this sleep
    tokio::time::sleep(Duration::from_secs(20)).await;

    /* check database for user_deposit_ledger */
    let resp = db
        .execute(FunUserListDepositLedgerReq {
            user_id: signup_ret.user_id,
            limit: 1,
            offset: 0,
        })
        .await?
        .into_result()
        .ok_or_else(|| eyre!("no user deposit Ledger"))?;

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
async fn test_handle_eth_swap_mainnet() -> Result<()> {
    drop_and_recreate_database()?;
    let fake_backer_strategy_wallet_key = Secp256k1SecretKey::new_random();
    let master_key = Secp256k1SecretKey::from_str(DEV_ACCOUNT_PRIV_KEY)
        .context("failed to parse dev account private key")?;
    let conn_pool = EthereumRpcConnectionPool::new();
    let conn = conn_pool.get(EnumBlockChain::BscMainnet).await?;
    let token_addresses = BlockchainCoinAddresses::new();
    let db = connect_to_database(database_test_config()).await?;

    let wbnb_address_on_bsc = token_addresses
        .get(EnumBlockChain::BscMainnet, EnumBlockchainCoin::WBNB)
        .ok_or_else(|| eyre!("could not find WBNB address on BSC Testnet"))?;
    let busd_address_on_bsc = token_addresses
        .get(EnumBlockChain::BscMainnet, EnumBlockchainCoin::BUSD)
        .ok_or_else(|| eyre!("could not find USDC address on BSC Testnet"))?;
    let busd_contract = Erc20Token::new(conn.clone(), busd_address_on_bsc)?;

    /* create expert */
    let expert = db
        .execute(FunAuthSignupReq {
            address: format!("{:?}", master_key.address()),
            email: "".to_string(),
            phone: "".to_string(),
            preferred_language: "".to_string(),
            agreed_tos: true,
            agreed_privacy: true,
            ip_address: Ipv4Addr::new(127, 0, 0, 1).into(),
            username: Some("TEST".to_string()),
            age: None,
            public_id: 2,
        })
        .await?
        .into_result()
        .context("no user signup resp")?;

    /* create strategy */
    let strategy = db
        .execute(FunUserCreateStrategyReq {
            user_id: expert.user_id,
            name: "TEST".to_string(),
            description: "TEST".to_string(),
            strategy_thesis_url: "TEST".to_string(),
            minimum_backing_amount_usd: 1.0,
            strategy_fee: 1.0,
            expert_fee: 1.0,
            agreed_tos: true,
            blockchain: EnumBlockChain::BscMainnet,
            wallet_address: format!("{:?}", Address::zero()),
        })
        .await?
        .into_result()
        .context("failed to create strategy")?;

    /* add strategy watching wallet */
    let watching_wallet = db
        .execute(FunUserAddStrategyWatchWalletReq {
            user_id: expert.user_id,
            strategy_id: strategy.strategy_id,
            blockchain: EnumBlockChain::BscMainnet,
            wallet_address: format!("{:?}", master_key.address()),
            ratio: 1.0,
            dex: EnumDex::PancakeSwap.to_string(),
        })
        .await?
        .into_result()
        .context("failed to add watching wallet")?;

    /* deploy strategy contract */
    let sp_contract = StrategyPoolContract::deploy(
        conn.clone(),
        master_key.clone(),
        "TEST".to_string(),
        "TEST".to_string(),
    )
    .await?;

    /* add strategy contract address to the database */
    db.execute(FunWatcherSaveStrategyPoolContractReq {
        strategy_id: strategy.strategy_id,
        blockchain: EnumBlockChain::BscMainnet,
        address: format!("{:?}", sp_contract.address()),
    })
    .await?;

    /* approve strategy pool for 1 BUSD deposit */
    /* make sure dev wallet has enough BUSD */
    approve_and_ensure_success(
        busd_contract.clone(),
        &conn,
        4,
        10,
        Duration::from_secs(10),
        master_key.clone(),
        sp_contract.address(),
        U256::from(1).try_checked_mul(U256::exp10(busd_contract.decimals().await?.as_usize()))?,
    )
    .await?;

    /* deposit 1 BUSD to strategy pool */
    /* make sure dev wallet has enough BUSD */
    deposit_and_ensure_success(
        sp_contract.clone(),
        &conn,
        12,
        10,
        Duration::from_secs(10),
        master_key.clone(),
        vec![busd_address_on_bsc],
        vec![U256::from(1)
            .try_checked_mul(U256::exp10(busd_contract.decimals().await?.as_usize()))?],
        U256::from(1),
        fake_backer_strategy_wallet_key.address(),
    )
    .await?;

    /* add tokens sold tokens as strategy tokens */
    /* since we are simulating the strategy owning the tokens before hand */
    /* and since they were already deposited to the strategy pool contract */
    /* the handler will check if the token sold was previously bought to calculate how much to buy for strategy pools */
    /* without a previous amount, there would be no way to calculate how much to buy */
    // TODO: change this to add to ledger once ledger is implemented
    db.execute(FunUserAddStrategyInitialTokenRatioReq {
        strategy_id: strategy.strategy_id,
        token_address: format!("{:?}", busd_address_on_bsc),
        token_name: "BUSD".to_string(),
        quantity: format!(
            "{:?}",
            U256::from(1)
                .try_checked_mul(U256::exp10(busd_contract.decimals().await?.as_usize()))?
        ),
        blockchain: EnumBlockChain::BscMainnet,
    })
    .await?;

    /* fetch pancake swap address */
    let dex_addresses = DexAddresses::new();
    let pancake_swap_address_on_bsc = dex_addresses
        .get(EnumBlockChain::BscMainnet, EnumDex::PancakeSwap)
        .context("could not get pancakeswap address on bsc testnet")?;

    /* expert trades 1 BUSD for WBNB on pancake swap */
    /* this is a previous trade of 1 BUSD for WBNB */
    let expert_trade_hash =
        H256::from_str("0xa42409d6beac0c219b957361a94ef9ba042fa274de2ebbd6936af74bec525a3c")
            .unwrap();

    /* set up app state */
    let app_state = AppState {
        dex_addresses: dex_addresses,
        eth_pool: conn_pool.clone(),
        erc_20: build_erc_20()?,
        pancake_swap: build_pancake_swap()?,
        token_addresses: BlockchainCoinAddresses::new(),
        escrow_addresses: EscrowAddresses::new(),
        db: db.clone(),
        master_key: master_key.clone(),
    };

    let expert_trade_tx =
        TransactionFetcher::new_and_assume_ready(expert_trade_hash, &conn).await?;
    /* handle eth swaps */
    match handle_swap(
        Arc::new(app_state),
        EnumBlockChain::BscMainnet,
        expert_trade_tx,
    )
    .await
    {
        Ok(_) => (),
        Err(e) => bail!("failed to handle eth swap: {}", e),
    };

    /* parse expert trade to check quantities */
    let expert_trade = parse_dex_trade(
        EnumBlockChain::BscMainnet,
        &TransactionFetcher::new_and_assume_ready(expert_trade_hash, &conn).await?,
        &DexAddresses::new(),
        &build_pancake_swap()?,
    )
    .await?;

    /* check entry in wallet activity ledger */
    let wallet_activity = db
        .execute(FunWatcherListWalletActivityLedgerReq {
            address: format!("{:?}", master_key.address()),
            blockchain: EnumBlockChain::BscMainnet,
        })
        .await?
        .into_result()
        .context("failed to fetch entry in wallet activity ledger")?;

    assert_eq!(
        wallet_activity.address,
        format!("{:?}", master_key.address())
    );
    assert_eq!(wallet_activity.blockchain, EnumBlockChain::BscMainnet);
    assert_eq!(
        wallet_activity.transaction_hash,
        format!("{:?}", expert_trade_hash)
    );
    assert_eq!(wallet_activity.dex, Some(EnumDex::PancakeSwap.to_string()));
    assert_eq!(
        wallet_activity.contract_address,
        format!("{:?}", pancake_swap_address_on_bsc)
    );
    assert_eq!(
        wallet_activity.token_in_address,
        Some(format!("{:?}", busd_address_on_bsc))
    );
    assert_eq!(
        wallet_activity.token_out_address,
        Some(format!("{:?}", wbnb_address_on_bsc))
    );
    assert_eq!(
        wallet_activity.caller_address,
        format!("{:?}", master_key.address())
    );
    assert_eq!(
        wallet_activity.amount_in,
        Some(format!(
            "{:?}",
            U256::from(1)
                .try_checked_mul(U256::exp10(busd_contract.decimals().await?.as_usize()))?
        ))
    );
    assert_eq!(
        wallet_activity.amount_out,
        Some(format!("{:?}", expert_trade.amount_out))
    );

    /* check strategy_initial_token_ratio now shows wbnb */
    let strategy_tokens = db
        .execute(FunUserListStrategyInitialTokenRatiosReq {
            strategy_id: strategy.strategy_id,
        })
        .await?
        .into_result()
        .context("no tokens")?;
    assert_eq!(
        strategy_tokens.token_address,
        format!("{:?}", wbnb_address_on_bsc)
    );
    assert_eq!(
        strategy_tokens.quantity,
        format!("{:?}", expert_trade.amount_out)
    );
    assert_eq!(strategy_tokens.blockchain, EnumBlockChain::BscMainnet);
    assert_eq!(strategy_tokens.strategy_id, strategy.strategy_id);

    /* check sp contract now holds wbnb instead of busd */
    let sp_contract_wbnb_balance = sp_contract.asset_balance(wbnb_address_on_bsc).await?;
    assert!(sp_contract_wbnb_balance > U256::zero());

    Ok(())
}
