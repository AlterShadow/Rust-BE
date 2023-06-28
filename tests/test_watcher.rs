use bytes::Bytes;
use eth_sdk::dex_tracker::parse_dex_trade;
use eth_sdk::erc20::{approve_and_ensure_success, build_erc_20, Erc20Token};
use eth_sdk::escrow::EscrowContract;
use eth_sdk::mock_erc20::deploy_mock_erc20;
use eth_sdk::signer::Secp256k1SecretKey;
use eth_sdk::strategy_pool::{sp_deposit_to_and_ensure_success, StrategyPoolContract};
use eth_sdk::utils::wait_for_confirmations_simple;
use eth_sdk::*;
use eyre::*;
use gen::database::*;
use gen::model::{EnumBlockChain, EnumBlockchainCoin, EnumDex, UserGetDepositAddressesRow};
use lib::config::load_config;
use lib::config::WsServerConfig;
use lib::database::*;
use lib::database::{connect_to_database, database_test_config, drop_and_recreate_database};
use lib::log::LogLevel;
use mc2fi_watcher::method::{handle_eth_escrows, handle_pancake_swap_transaction};
use mc2fi_watcher::*;
use serde::Deserialize;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use web3::signing::Key;
use web3::types::{Address, H256, U256};

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
        admin_client: None,
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
async fn test_handle_eth_swap_goerli_testnet() -> Result<()> {
    drop_and_recreate_database()?;

    let fake_backer_strategy_wallet_key = Secp256k1SecretKey::new_random();
    let master_key = Secp256k1SecretKey::from_str(DEV_ACCOUNT_PRIV_KEY)
        .context("failed to parse dev account private key")?;
    let conn_pool = EthereumRpcConnectionPool::new();
    let conn = conn_pool.get(EnumBlockChain::EthereumGoerli).await?;
    let token_addresses = BlockchainCoinAddresses::new();
    let db = connect_to_database(database_test_config()).await?;
    add_tokens_to_database(&db).await?;
    let weth_address_on_goerli = token_addresses
        .get(EnumBlockChain::EthereumGoerli, EnumBlockchainCoin::WETH)
        .ok_or_else(|| eyre!("could not find WBNB address on BSC Testnet"))?;
    let usdc_address_on_goerli = token_addresses
        .get(EnumBlockChain::EthereumGoerli, EnumBlockchainCoin::USDC)
        .ok_or_else(|| eyre!("could not find USDC address on BSC Testnet"))?;
    let usdc_contract = Erc20Token::new(conn.clone(), usdc_address_on_goerli)?;

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
            blockchain: EnumBlockChain::EthereumGoerli,
            wallet_address: format!("{:?}", Address::zero()),
        })
        .await?
        .into_result()
        .context("failed to create strategy")?;

    /* add strategy watching wallet */
    let _watching_wallet = db
        .execute(FunUserAddStrategyWatchWalletReq {
            user_id: expert.user_id,
            strategy_id: strategy.strategy_id,
            blockchain: EnumBlockChain::EthereumGoerli,
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
    let insert_sp_contract = db
        .execute(FunWatcherSaveStrategyPoolContractReq {
            strategy_id: strategy.strategy_id,
            blockchain: EnumBlockChain::EthereumGoerli,
            address: format!("{:?}", sp_contract.address()),
        })
        .await?
        .into_result()
        .context("could not insert sp contract to database")?;

    /* approve strategy pool for 1 USDC deposit */
    /* make sure dev wallet has enough USDC */
    approve_and_ensure_success(
        usdc_contract.clone(),
        &conn,
        4,
        10,
        Duration::from_secs(10),
        master_key.clone(),
        sp_contract.address(),
        U256::from(1).try_checked_mul(U256::exp10(usdc_contract.decimals().await?.as_usize()))?,
    )
    .await?;

    /* deposit 1 USDC to strategy pool */
    /* make sure dev wallet has enough USDC */
    sp_deposit_to_and_ensure_success(
        sp_contract.clone(),
        &conn,
        12,
        10,
        Duration::from_secs(10),
        master_key.clone(),
        vec![usdc_address_on_goerli],
        vec![U256::from(1)
            .try_checked_mul(U256::exp10(usdc_contract.decimals().await?.as_usize()))?],
        U256::from(1),
        fake_backer_strategy_wallet_key.address(),
    )
    .await?;

    /* add deposit to strategy pool balance table */
    db.execute(FunWatcherUpsertStrategyPoolContractAssetBalanceReq {
        strategy_pool_contract_id: insert_sp_contract.pkey_id,
        token_address: format!("{:?}", usdc_address_on_goerli),
        blockchain: EnumBlockChain::EthereumGoerli,
        new_balance: format!(
            "{:?}",
            U256::from(1)
                .try_checked_mul(U256::exp10(usdc_contract.decimals().await?.as_usize()))?
        ),
    })
    .await?;

    /* add tokens sold tokens as strategy tokens */
    /* since we are simulating the strategy owning the tokens beforehand */
    /* and since they were already deposited to the strategy pool contract */
    /* the handler will check if the token sold was previously bought to calculate how much to buy for strategy pools */
    /* without a previous amount, there would be no way to calculate how much to buy */
    // TODO: change this to add to ledger once ledger is implemented
    db.execute(FunWatcherSaveStrategyWatchingWalletTradeLedgerReq {
        address: format!("{:?}", master_key.address()),
        transaction_hash: format!("{:?}", H256::zero()),
        blockchain: EnumBlockChain::EthereumGoerli,
        contract_address: format!("{:?}", Address::zero()),
        dex: None,
        token_in_address: Some(format!("{:?}", weth_address_on_goerli)),
        token_out_address: Some(format!("{:?}", usdc_address_on_goerli)),
        amount_in: Some(format!("{:?}", U256::one())),
        amount_out: Some(format!(
            "{:?}",
            U256::from(1)
                .try_checked_mul(U256::exp10(usdc_contract.decimals().await?.as_usize()))?
        )), // 1 USDC
        happened_at: None,
    })
    .await?;

    /* fetch pancake swap address */
    let dex_addresses = DexAddresses::new();
    let _pancake_swap_address_on_bsc = dex_addresses
        .get(EnumBlockChain::EthereumGoerli, EnumDex::PancakeSwap)
        .context("could not get pancakeswap address on bsc testnet")?;

    /* expert trades 1 USDC for WETH on pancake swap */
    /* this is a previous trade of 1 USDC for WETH on Goerli */
    /* this trade was made from the dev wallet, which is the expert watched wallet for the created strategy */
    let expert_trade_hash =
        H256::from_str("0x305e519ad0f9ac81d9b3b897c252e10875eebfeef6eeae7e3b114ef3709ebfc6")
            .unwrap();

    /* set up app state */
    let app_state = AppState {
        dex_addresses,
        eth_pool: conn_pool.clone(),
        erc_20: build_erc_20()?,
        pancake_swap: build_pancake_swap()?,
        token_addresses: BlockchainCoinAddresses::new(),
        escrow_addresses: EscrowAddresses::new(),
        db: db.clone(),
        master_key: master_key.clone(),
        admin_client: None,
    };

    let expert_trade_tx =
        TransactionFetcher::new_and_assume_ready(expert_trade_hash, &conn).await?;
    /* handle eth swaps */
    match handle_pancake_swap_transaction(
        Arc::new(app_state),
        EnumBlockChain::EthereumGoerli,
        expert_trade_tx,
    )
    .await
    {
        Ok(_) => (),
        Err(e) => bail!("failed to handle eth swap: {}", e),
    };

    /* parse expert trade to check quantities */
    let expert_trade = parse_dex_trade(
        EnumBlockChain::EthereumGoerli,
        &TransactionFetcher::new_and_assume_ready(expert_trade_hash, &conn).await?,
        &DexAddresses::new(),
        &build_pancake_swap()?,
    )
    .await?;

    /* check entry in watched wallet ledger */
    let strategy_tokens_after = db
        .execute(FunWatcherGetStrategyTokensFromLedgerReq {
            strategy_id: strategy.strategy_id,
        })
        .await?
        .into_rows();

    let mut usdc_present = false;
    let mut weth_present = false;
    for row in strategy_tokens_after {
        if row.token_address == format!("{:?}", weth_address_on_goerli) {
            weth_present = true;
            assert!(U256::from_dec_str(&row.amount)? > U256::one());
            assert_eq!(row.blockchain, EnumBlockChain::EthereumGoerli);
        }
        if row.token_address == format!("{:?}", usdc_address_on_goerli) {
            usdc_present = true;
        }
    }
    assert!(weth_present);
    // USDC cannot be present, since watched wallet had 1 as strategy tokens, but sold 1 USDC for WETH
    assert!(!usdc_present);

    /* check pool balance table shows WETH */
    let strategy_pool_contract_weth = db
        .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
            strategy_pool_contract_id: insert_sp_contract.pkey_id,
            blockchain: Some(EnumBlockChain::EthereumGoerli),
            token_address: Some(format!("{:?}", weth_address_on_goerli)),
        })
        .await?
        .into_result()
        .context("no WETH found for strategy pool contract balance")?;
    let strategy_pool_contract_usdc = db
        .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
            strategy_pool_contract_id: insert_sp_contract.pkey_id,
            blockchain: Some(EnumBlockChain::EthereumGoerli),
            token_address: Some(format!("{:?}", usdc_address_on_goerli)),
        })
        .await?
        .into_result()
        .context("no USDC found for strategy pool contract balance")?;
    assert!(U256::from_dec_str(&strategy_pool_contract_weth.balance)? > U256::one());
    assert!(U256::from_dec_str(&strategy_pool_contract_usdc.balance)? == U256::zero());

    /* check sp contract now holds WETH instead of USDC */
    let sp_contract_weth_balance = sp_contract.asset_balance(weth_address_on_goerli).await?;
    assert!(sp_contract_weth_balance > U256::zero());
    /* check the tokens held by the contract are exactly the ones in the balance table */
    assert_eq!(
        U256::from_dec_str(&strategy_pool_contract_weth.balance)?,
        sp_contract_weth_balance
    );

    /* check expert listened wallet balance table now shows WETH */
    let expert_listened_wallet = db
        .execute(FunWatcherListExpertListenedWalletAssetBalanceReq {
            limit: 1,
            offset: 0,
            address: Some(format!("{:?}", master_key.address())),
            blockchain: None,
            token_id: None,
        })
        .await?
        .into_result()
        .context("no entry found in expert listened wallet balance table")?;

    let weth_id_on_goerli = db
        .execute(FunUserListEscrowTokenContractAddressReq {
            limit: 1,
            offset: 0,
            token_id: None,
            blockchain: Some(EnumBlockChain::EthereumGoerli),
            address: Some(format!("{:?}", weth_address_on_goerli)),
            symbol: None,
            is_stablecoin: None,
        })
        .await?
        .into_result()
        .context("no WETH found in escrow token contract address table")?;

    assert_eq!(expert_listened_wallet.token_id, weth_id_on_goerli.token_id);
    assert!(U256::from_dec_str(&expert_listened_wallet.balance)? > U256::one());

    Ok(())
}

#[tokio::test]
async fn test_handle_eth_swap_bsc_mainnet() -> Result<()> {
    drop_and_recreate_database()?;

    let fake_backer_strategy_wallet_key = Secp256k1SecretKey::new_random();
    let master_key = Secp256k1SecretKey::from_str(DEV_ACCOUNT_PRIV_KEY)
        .context("failed to parse dev account private key")?;
    let conn_pool = EthereumRpcConnectionPool::new();
    let conn = conn_pool.get(EnumBlockChain::BscMainnet).await?;
    let token_addresses = BlockchainCoinAddresses::new();
    let db = connect_to_database(database_test_config()).await?;
    add_tokens_to_database(&db).await?;
    let wbnb_address_on_bsc = token_addresses
        .get(EnumBlockChain::BscMainnet, EnumBlockchainCoin::WBNB)
        .ok_or_else(|| eyre!("could not find WBNB address on BSC Testnet"))?;
    let busd_address_on_bsc = token_addresses
        .get(EnumBlockChain::BscMainnet, EnumBlockchainCoin::BUSD)
        .ok_or_else(|| eyre!("could not find BUSD address on BSC Testnet"))?;
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
    let _watching_wallet = db
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
    let insert_sp_contract = db
        .execute(FunWatcherSaveStrategyPoolContractReq {
            strategy_id: strategy.strategy_id,
            blockchain: EnumBlockChain::BscMainnet,
            address: format!("{:?}", sp_contract.address()),
        })
        .await?
        .into_result()
        .context("could not insert sp contract to database")?;

    /* approve strategy pool for 0.0001 BUSD deposit */
    /* make sure dev wallet has enough BUSD */
    let how_much_busd_to_spend = U256::from(1).try_checked_mul(U256::exp10(
        busd_contract.decimals().await?.as_usize() - 4 as usize,
    ))?;
    approve_and_ensure_success(
        busd_contract.clone(),
        &conn,
        4,
        10,
        Duration::from_secs(10),
        master_key.clone(),
        sp_contract.address(),
        how_much_busd_to_spend,
    )
    .await?;

    /* deposit 0.0001 BUSD to strategy pool */
    /* make sure dev wallet has enough BUSD */
    sp_deposit_to_and_ensure_success(
        sp_contract.clone(),
        &conn,
        12,
        10,
        Duration::from_secs(10),
        master_key.clone(),
        vec![busd_address_on_bsc],
        vec![how_much_busd_to_spend],
        U256::from(1),
        fake_backer_strategy_wallet_key.address(),
    )
    .await?;

    /* add deposit to strategy pool balance table */
    db.execute(FunWatcherUpsertStrategyPoolContractAssetBalanceReq {
        strategy_pool_contract_id: insert_sp_contract.pkey_id,
        token_address: format!("{:?}", busd_address_on_bsc),
        blockchain: EnumBlockChain::BscMainnet,
        new_balance: format!("{:?}", how_much_busd_to_spend),
    })
    .await?;

    /* add tokens sold tokens as strategy tokens */
    /* since we are simulating the strategy owning the tokens beforehand */
    /* and since they were already deposited to the strategy pool contract */
    /* the handler will check if the token sold was previously bought to calculate how much to buy for strategy pools */
    /* without a previous amount, there would be no way to calculate how much to buy */
    // TODO: change this to add to ledger once ledger is implemented
    db.execute(FunWatcherSaveStrategyWatchingWalletTradeLedgerReq {
        address: format!("{:?}", master_key.address()),
        transaction_hash: format!("{:?}", H256::zero()),
        blockchain: EnumBlockChain::BscMainnet,
        contract_address: format!("{:?}", Address::zero()),
        dex: None,
        token_in_address: Some(format!("{:?}", wbnb_address_on_bsc)),
        token_out_address: Some(format!("{:?}", busd_address_on_bsc)),
        amount_in: Some(format!("{:?}", U256::one())),
        amount_out: Some(format!("{:?}", how_much_busd_to_spend)), // 0.0001 BUSD
        happened_at: None,
    })
    .await?;

    /* fetch pancake swap address */
    let dex_addresses = DexAddresses::new();
    let _pancake_swap_address_on_bsc = dex_addresses
        .get(EnumBlockChain::BscMainnet, EnumDex::PancakeSwap)
        .context("could not get pancakeswap address on bsc testnet")?;

    /* expert trades 0.0001 BUSD for WBNB on pancake swap */
    /* this is a previous trade of 0.0001 BUSD for WBNB on BSC */
    /* this trade was made from the dev wallet, which is the expert watched wallet for the created strategy */
    let expert_trade_hash =
        H256::from_str("0x1d5af5eb81d7e6b1ce417a44072902b661b024ea04e58890e9bec2a4b5b9a423")
            .unwrap();

    /* set up app state */
    let app_state = AppState {
        dex_addresses,
        eth_pool: conn_pool.clone(),
        erc_20: build_erc_20()?,
        pancake_swap: build_pancake_swap()?,
        token_addresses: BlockchainCoinAddresses::new(),
        escrow_addresses: EscrowAddresses::new(),
        db: db.clone(),
        master_key: master_key.clone(),
        admin_client: None,
    };

    let expert_trade_tx =
        TransactionFetcher::new_and_assume_ready(expert_trade_hash, &conn).await?;
    /* handle eth swaps */
    match handle_pancake_swap_transaction(
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

    /* check entry in watched wallet ledger */
    let strategy_tokens_after = db
        .execute(FunWatcherGetStrategyTokensFromLedgerReq {
            strategy_id: strategy.strategy_id,
        })
        .await?
        .into_rows();

    let mut busd_present = false;
    let mut wbnb_present = false;
    for row in strategy_tokens_after {
        if row.token_address == format!("{:?}", wbnb_address_on_bsc) {
            wbnb_present = true;
            assert!(U256::from_dec_str(&row.amount)? > U256::one());
            assert_eq!(row.blockchain, EnumBlockChain::BscMainnet);
        }
        if row.token_address == format!("{:?}", busd_address_on_bsc) {
            busd_present = true;
        }
    }
    assert!(wbnb_present);
    // BUSD cannot be present, since watched wallet had 0.0001 as strategy tokens, but sold 0.0001 BUSD for WBNB
    assert!(!busd_present);

    /* check pool balance table shows WBNB */
    let strategy_pool_contract_wbnb = db
        .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
            strategy_pool_contract_id: insert_sp_contract.pkey_id,
            blockchain: Some(EnumBlockChain::BscMainnet),
            token_address: Some(format!("{:?}", wbnb_address_on_bsc)),
        })
        .await?
        .into_result()
        .context("no WBNB found for strategy pool contract balance")?;
    let strategy_pool_contract_busd = db
        .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
            strategy_pool_contract_id: insert_sp_contract.pkey_id,
            blockchain: Some(EnumBlockChain::BscMainnet),
            token_address: Some(format!("{:?}", busd_address_on_bsc)),
        })
        .await?
        .into_result()
        .context("no BUSD found for strategy pool contract balance")?;
    assert!(U256::from_dec_str(&strategy_pool_contract_wbnb.balance)? > U256::one());
    assert!(U256::from_dec_str(&strategy_pool_contract_busd.balance)? == U256::zero());

    /* check sp contract now holds WBNB instead of BUSD */
    let sp_contract_wbnb_balance = sp_contract.asset_balance(wbnb_address_on_bsc).await?;
    assert!(sp_contract_wbnb_balance > U256::zero());
    /* check the tokens held by the contract are exactly the ones in the balance table */
    assert_eq!(
        U256::from_dec_str(&strategy_pool_contract_wbnb.balance)?,
        sp_contract_wbnb_balance
    );

    let wbnd_id_on_bsc = db
        .execute(FunUserListEscrowTokenContractAddressReq {
            limit: 1,
            offset: 0,
            token_id: None,
            blockchain: Some(EnumBlockChain::BscMainnet),
            address: Some(format!("{:?}", wbnb_address_on_bsc)),
            symbol: None,
            is_stablecoin: None,
        })
        .await?
        .into_result()
        .context("no WBNB found in escrow token contract address table")?;

    /* check expert listened wallet balance table now shows WBNB */
    let expert_listened_wallet = db
        .execute(FunWatcherListExpertListenedWalletAssetBalanceReq {
            limit: 1,
            offset: 0,
            address: Some(format!("{:?}", master_key.address())),
            blockchain: None,
            token_id: None,
        })
        .await?
        .into_result()
        .context("no entry found in expert listened wallet balance table")?;

    assert_eq!(expert_listened_wallet.token_id, wbnd_id_on_bsc.token_id);
    assert!(U256::from_dec_str(&expert_listened_wallet.balance)? > U256::one());

    Ok(())
}

async fn add_tokens_to_database(db: &DbClient) -> Result<()> {
    let token_addresses = BlockchainCoinAddresses::new();
    db.query(
        "
			INSERT INTO tbl.escrow_token_contract_address (
				pkey_id,
				blockchain,
				symbol,
				short_name,
				description,
				address,
				is_stablecoin
			) VALUES ($1, $2, $3, $4, $5, $6, $7)
	",
        &[
            &i64::from(1) as &(dyn ToSql + Sync),
            &EnumBlockChain::EthereumMainnet as &(dyn ToSql + Sync),
            &"USDC" as &(dyn ToSql + Sync),
            &"USDC" as &(dyn ToSql + Sync),
            &"USDC" as &(dyn ToSql + Sync),
            &format!(
                "{:?}",
                token_addresses
                    .get(EnumBlockChain::EthereumMainnet, EnumBlockchainCoin::USDC)
                    .context("could not get token address for test insertion in database")?
            ) as &(dyn ToSql + Sync),
            &true as &(dyn ToSql + Sync),
        ],
    )
    .await?;

    db.query(
        "
		INSERT INTO tbl.escrow_token_contract_address (
			pkey_id,
			blockchain,
			symbol,
			short_name,
			description,
			address,
			is_stablecoin
		) VALUES ($1, $2, $3, $4, $5, $6, $7)
",
        &[
            &i64::from(2) as &(dyn ToSql + Sync),
            &EnumBlockChain::EthereumMainnet as &(dyn ToSql + Sync),
            &"USDT" as &(dyn ToSql + Sync),
            &"USDT" as &(dyn ToSql + Sync),
            &"USDT" as &(dyn ToSql + Sync),
            &format!(
                "{:?}",
                token_addresses
                    .get(EnumBlockChain::EthereumMainnet, EnumBlockchainCoin::USDT)
                    .context("could not get token address for test insertion in database")?
            ) as &(dyn ToSql + Sync),
            &true as &(dyn ToSql + Sync),
        ],
    )
    .await?;

    db.query(
        "
	INSERT INTO tbl.escrow_token_contract_address (
		pkey_id,
		blockchain,
		symbol,
		short_name,
		description,
		address,
		is_stablecoin
	) VALUES ($1, $2, $3, $4, $5, $6, $7)
",
        &[
            &i64::from(3) as &(dyn ToSql + Sync),
            &EnumBlockChain::EthereumMainnet as &(dyn ToSql + Sync),
            &"BUSD" as &(dyn ToSql + Sync),
            &"BUSD" as &(dyn ToSql + Sync),
            &"BUSD" as &(dyn ToSql + Sync),
            &format!(
                "{:?}",
                token_addresses
                    .get(EnumBlockChain::EthereumMainnet, EnumBlockchainCoin::BUSD)
                    .context("could not get token address for test insertion in database")?
            ) as &(dyn ToSql + Sync),
            &true as &(dyn ToSql + Sync),
        ],
    )
    .await?;

    db.query(
        "
INSERT INTO tbl.escrow_token_contract_address (
	pkey_id,
	blockchain,
	symbol,
	short_name,
	description,
	address,
	is_stablecoin
) VALUES ($1, $2, $3, $4, $5, $6, $7)
",
        &[
            &i64::from(4) as &(dyn ToSql + Sync),
            &EnumBlockChain::EthereumMainnet as &(dyn ToSql + Sync),
            &"WETH" as &(dyn ToSql + Sync),
            &"WETH" as &(dyn ToSql + Sync),
            &"WETH" as &(dyn ToSql + Sync),
            &format!(
                "{:?}",
                token_addresses
                    .get(EnumBlockChain::EthereumMainnet, EnumBlockchainCoin::WETH)
                    .context("could not get token address for test insertion in database")?
            ) as &(dyn ToSql + Sync),
            &true as &(dyn ToSql + Sync),
        ],
    )
    .await?;

    db.query(
        "
INSERT INTO tbl.escrow_token_contract_address (
	pkey_id,
	blockchain,
	symbol,
	short_name,
	description,
	address,
	is_stablecoin
) VALUES ($1, $2, $3, $4, $5, $6, $7)
",
        &[
            &i64::from(5) as &(dyn ToSql + Sync),
            &EnumBlockChain::EthereumGoerli as &(dyn ToSql + Sync),
            &"USDC" as &(dyn ToSql + Sync),
            &"USDC" as &(dyn ToSql + Sync),
            &"USDC" as &(dyn ToSql + Sync),
            &format!(
                "{:?}",
                token_addresses
                    .get(EnumBlockChain::EthereumGoerli, EnumBlockchainCoin::USDC)
                    .context("could not get token address for test insertion in database")?
            ) as &(dyn ToSql + Sync),
            &true as &(dyn ToSql + Sync),
        ],
    )
    .await?;

    db.query(
        "
INSERT INTO tbl.escrow_token_contract_address (
	pkey_id,
	blockchain,
	symbol,
	short_name,
	description,
	address,
	is_stablecoin
) VALUES ($1, $2, $3, $4, $5, $6, $7)
",
        &[
            &i64::from(6) as &(dyn ToSql + Sync),
            &EnumBlockChain::EthereumGoerli as &(dyn ToSql + Sync),
            &"WETH" as &(dyn ToSql + Sync),
            &"WETH" as &(dyn ToSql + Sync),
            &"WETH" as &(dyn ToSql + Sync),
            &format!(
                "{:?}",
                token_addresses
                    .get(EnumBlockChain::EthereumGoerli, EnumBlockchainCoin::WETH)
                    .context("could not get token address for test insertion in database")?
            ) as &(dyn ToSql + Sync),
            &true as &(dyn ToSql + Sync),
        ],
    )
    .await?;

    db.query(
        "
INSERT INTO tbl.escrow_token_contract_address (
	pkey_id,
	blockchain,
	symbol,
	short_name,
	description,
	address,
	is_stablecoin
) VALUES ($1, $2, $3, $4, $5, $6, $7)
",
        &[
            &i64::from(7) as &(dyn ToSql + Sync),
            &EnumBlockChain::BscMainnet as &(dyn ToSql + Sync),
            &"USDC" as &(dyn ToSql + Sync),
            &"USDC" as &(dyn ToSql + Sync),
            &"USDC" as &(dyn ToSql + Sync),
            &format!(
                "{:?}",
                token_addresses
                    .get(EnumBlockChain::BscMainnet, EnumBlockchainCoin::USDC)
                    .context("could not get token address for test insertion in database")?
            ) as &(dyn ToSql + Sync),
            &true as &(dyn ToSql + Sync),
        ],
    )
    .await?;

    db.query(
        "
INSERT INTO tbl.escrow_token_contract_address (
	pkey_id,
	blockchain,
	symbol,
	short_name,
	description,
	address,
	is_stablecoin
) VALUES ($1, $2, $3, $4, $5, $6, $7)
",
        &[
            &i64::from(8) as &(dyn ToSql + Sync),
            &EnumBlockChain::BscMainnet as &(dyn ToSql + Sync),
            &"USDT" as &(dyn ToSql + Sync),
            &"USDT" as &(dyn ToSql + Sync),
            &"USDT" as &(dyn ToSql + Sync),
            &format!(
                "{:?}",
                token_addresses
                    .get(EnumBlockChain::BscMainnet, EnumBlockchainCoin::USDT)
                    .context("could not get token address for test insertion in database")?
            ) as &(dyn ToSql + Sync),
            &true as &(dyn ToSql + Sync),
        ],
    )
    .await?;

    db.query(
        "
INSERT INTO tbl.escrow_token_contract_address (
	pkey_id,
	blockchain,
	symbol,
	short_name,
	description,
	address,
	is_stablecoin
) VALUES ($1, $2, $3, $4, $5, $6, $7)
",
        &[
            &i64::from(9) as &(dyn ToSql + Sync),
            &EnumBlockChain::BscMainnet as &(dyn ToSql + Sync),
            &"BUSD" as &(dyn ToSql + Sync),
            &"BUSD" as &(dyn ToSql + Sync),
            &"BUSD" as &(dyn ToSql + Sync),
            &format!(
                "{:?}",
                token_addresses
                    .get(EnumBlockChain::BscMainnet, EnumBlockchainCoin::BUSD)
                    .context("could not get token address for test insertion in database")?
            ) as &(dyn ToSql + Sync),
            &true as &(dyn ToSql + Sync),
        ],
    )
    .await?;

    db.query(
        "
INSERT INTO tbl.escrow_token_contract_address (
	pkey_id,
	blockchain,
	symbol,
	short_name,
	description,
	address,
	is_stablecoin
) VALUES ($1, $2, $3, $4, $5, $6, $7)
",
        &[
            &i64::from(10) as &(dyn ToSql + Sync),
            &EnumBlockChain::BscMainnet as &(dyn ToSql + Sync),
            &"WBNB" as &(dyn ToSql + Sync),
            &"WBNB" as &(dyn ToSql + Sync),
            &"WBNB" as &(dyn ToSql + Sync),
            &format!(
                "{:?}",
                token_addresses
                    .get(EnumBlockChain::BscMainnet, EnumBlockchainCoin::WBNB)
                    .context("could not get token address for test insertion in database")?
            ) as &(dyn ToSql + Sync),
            &true as &(dyn ToSql + Sync),
        ],
    )
    .await?;

    db.query(
        "
INSERT INTO tbl.escrow_token_contract_address (
	pkey_id,
	blockchain,
	symbol,
	short_name,
	description,
	address,
	is_stablecoin
) VALUES ($1, $2, $3, $4, $5, $6, $7)
",
        &[
            &i64::from(11) as &(dyn ToSql + Sync),
            &EnumBlockChain::BscTestnet as &(dyn ToSql + Sync),
            &"BUSD" as &(dyn ToSql + Sync),
            &"BUSD" as &(dyn ToSql + Sync),
            &"BUSD" as &(dyn ToSql + Sync),
            &format!(
                "{:?}",
                token_addresses
                    .get(EnumBlockChain::BscTestnet, EnumBlockchainCoin::BUSD)
                    .context("could not get token address for test insertion in database")?
            ) as &(dyn ToSql + Sync),
            &true as &(dyn ToSql + Sync),
        ],
    )
    .await?;

    db.query(
        "
INSERT INTO tbl.escrow_token_contract_address (
	pkey_id,
	blockchain,
	symbol,
	short_name,
	description,
	address,
	is_stablecoin
) VALUES ($1, $2, $3, $4, $5, $6, $7)
",
        &[
            &i64::from(12) as &(dyn ToSql + Sync),
            &EnumBlockChain::BscTestnet as &(dyn ToSql + Sync),
            &"WBNB" as &(dyn ToSql + Sync),
            &"WBNB" as &(dyn ToSql + Sync),
            &"WBNB" as &(dyn ToSql + Sync),
            &format!(
                "{:?}",
                token_addresses
                    .get(EnumBlockChain::BscTestnet, EnumBlockchainCoin::WBNB)
                    .context("could not get token address for test insertion in database")?
            ) as &(dyn ToSql + Sync),
            &true as &(dyn ToSql + Sync),
        ],
    )
    .await?;
    Ok(())
}
