pub mod tools;
use tools::*;

use eth_sdk::erc20::Erc20Token;
use eth_sdk::escrow::EscrowContract;
use eth_sdk::signer::Secp256k1SecretKey;
use eth_sdk::strategy_pool::StrategyPoolContract;
use eth_sdk::utils::wait_for_confirmations_simple;
use eth_sdk::DexAddresses;
use eth_sdk::ScaledMath;
use eth_sdk::{BlockchainCoinAddresses, EthereumRpcConnectionPool};
use gen::database::{
    FunAdminAddEscrowContractAddressReq, FunAdminAddEscrowTokenContractAddressReq,
    FunAuthSignupReq, FunUserAddStrategyInitialTokenRatioReq, FunUserAddStrategyPoolContractReq,
    FunUserCreateStrategyReq, FunUserListEscrowTokenContractAddressReq,
    FunUserListStrategyWalletsReq, FunWatcherListStrategyPoolContractAssetBalancesReq,
    FunWatcherListUserStrategyBalanceReq, FunWatcherUpsertStrategyPoolContractAssetBalanceReq,
    FunWatcherUpsertUserDepositWithdrawBalanceReq,
};
use gen::model::{EnumBlockchainCoin, EnumRole};
use lib::database::{connect_to_database, database_test_config, drop_and_recreate_database};
use lib::toolbox::RequestContext;

use std::net::Ipv4Addr;
use std::time::Duration;
use web3::signing::Key;
use web3::types::{Address, U256};

use eyre::*;
use gen::model::*;
use lib::log::{setup_logs, LogLevel};

use tracing::*;

#[tokio::test]
async fn test_create_update_strategy() -> Result<()> {
    let _ = setup_logs(LogLevel::Info);
    drop_and_recreate_database()?;
    let (_admin, _admin_client, user, mut client) = prepare_expert().await?;
    let resp = client
        .request(ExpertCreateStrategyRequest {
            name: "test_strategy".to_string(),
            description: "this is a test strategy".to_string(),
            strategy_thesis_url: "".to_string(),
            minimum_backing_amount_usd: None,
            strategy_fee: 0.0,
            expert_fee: 0.0,
            agreed_tos: true,
            wallet_address: user.address.into(),
            wallet_blockchain: EnumBlockChain::EthereumMainnet,
            initial_tokens: vec![],
            audit_rules: Some(vec![1]),
        })
        .await?;
    info!("Register wallet {:?}", resp);
    client
        .request(ExpertUpdateStrategyRequest {
            strategy_id: resp.strategy_id,
            name: None,
            description: None,
            social_media: None,
        })
        .await?;
    let wallet = client
        .request(ExpertAddStrategyWatchingWalletRequest {
            strategy_id: resp.strategy_id,
            blockchain: EnumBlockChain::LocalNet,
            wallet_address: user.address().into(),
            ratio: 1.0,
        })
        .await?;
    info!("Add wallet {:?}", wallet);
    let remove_wallet = client
        .request(ExpertRemoveStrategyWatchingWalletRequest {
            strategy_id: resp.strategy_id,
            wallet_id: wallet.wallet_id,
        })
        .await?;
    info!("Remove wallet {:?}", remove_wallet);
    Ok(())
}

#[tokio::test]
async fn test_user_follow_strategy() -> Result<()> {
    let _ = setup_logs(LogLevel::Info);
    drop_and_recreate_database()?;
    let (_admin, _admin_client, user, mut client) = prepare_expert().await?;

    let create_strategy_resp = client
        .request(ExpertCreateStrategyRequest {
            name: "test_strategy".to_string(),
            description: "this is a test strategy".to_string(),
            strategy_thesis_url: "".to_string(),
            minimum_backing_amount_usd: None,
            strategy_fee: 0.0,
            expert_fee: 0.0,
            agreed_tos: true,
            wallet_address: user.address.into(),
            wallet_blockchain: EnumBlockChain::EthereumMainnet,
            initial_tokens: vec![],
            audit_rules: Some(vec![1]),
        })
        .await?;
    info!("User Create Strategy {:?}", create_strategy_resp);

    let resp = client
        .request(UserFollowStrategyRequest {
            strategy_id: create_strategy_resp.strategy_id,
        })
        .await?;
    info!("User Follow Strategy {:?}", resp);
    let resp = client
        .request(UserListFollowedStrategiesRequest {
            limit: None,
            offset: None,
        })
        .await?;
    assert_eq!(resp.strategies.len(), 1);
    info!("User List Followed Strategies {:?}", resp);
    let resp = client
        .request(UserUnfollowStrategyRequest {
            strategy_id: create_strategy_resp.strategy_id,
        })
        .await?;
    info!("User Unfollow Strategy {:?}", resp);
    let resp = client
        .request(UserListFollowedStrategiesRequest {
            limit: None,
            offset: None,
        })
        .await?;
    assert_eq!(resp.strategies.len(), 0);
    let resp = client
        .request(UserListStrategiesRequest {
            limit: None,
            offset: None,
            strategy_id: None,
            strategy_name: None,
            expert_public_id: None,
            expert_name: None,
            description: None,
            blockchain: None,
            wallet_address: None,
        })
        .await?;
    assert_eq!(resp.strategies.len(), 1);
    info!("User List Strategies {:?}", resp);
    Ok(())
}

#[tokio::test]
async fn test_user_follow_strategy_get_user_profile() -> Result<()> {
    let _ = setup_logs(LogLevel::Info);
    drop_and_recreate_database()?;
    let (_admin, _admin_client, user, mut client) = prepare_expert().await?;

    let create_strategy_resp = client
        .request(ExpertCreateStrategyRequest {
            name: "test_strategy".to_string(),
            description: "this is a test strategy".to_string(),
            strategy_thesis_url: "".to_string(),
            minimum_backing_amount_usd: None,
            strategy_fee: 0.0,
            expert_fee: 0.0,
            agreed_tos: true,
            wallet_address: user.address.into(),
            wallet_blockchain: EnumBlockChain::EthereumMainnet,
            initial_tokens: vec![],
            audit_rules: Some(vec![1]),
        })
        .await?;
    info!("User Create Strategy {:?}", create_strategy_resp);

    let resp = client
        .request(UserFollowStrategyRequest {
            strategy_id: create_strategy_resp.strategy_id,
        })
        .await?;
    info!("User Follow Strategy {:?}", resp);
    let resp = client
        .request(UserListFollowedStrategiesRequest {
            limit: None,
            offset: None,
        })
        .await?;
    assert_eq!(resp.strategies.len(), 1);

    let resp = client
        .request(UserUnfollowStrategyRequest {
            strategy_id: create_strategy_resp.strategy_id,
        })
        .await?;
    info!("User Unfollow Strategy {:?}", resp);
    let resp = client.request(UserGetUserProfileRequest {}).await?;
    info!("User Profile {:?}", resp);
    Ok(())
}
#[tokio::test]
async fn test_user_list_strategies() -> Result<()> {
    let _ = setup_logs(LogLevel::Info);
    drop_and_recreate_database()?;
    let (_admin, _admin_client, user, mut client) = prepare_expert().await?;

    let create_strategy_resp = client
        .request(ExpertCreateStrategyRequest {
            name: "test_strategy".to_string(),
            description: "this is a test strategy".to_string(),
            strategy_thesis_url: "".to_string(),
            minimum_backing_amount_usd: None,
            strategy_fee: 0.0,
            expert_fee: 0.0,
            agreed_tos: true,
            wallet_address: user.address.into(),
            wallet_blockchain: EnumBlockChain::EthereumMainnet,
            initial_tokens: vec![],
            audit_rules: None,
        })
        .await?;
    info!("User Create Strategy {:?}", create_strategy_resp);

    let resp = client
        .request(UserListStrategiesRequest {
            limit: None,
            offset: None,
            strategy_id: None,
            strategy_name: None,
            expert_public_id: None,
            expert_name: None,
            description: None,
            blockchain: None,
            wallet_address: None,
        })
        .await?;
    info!("User List Strategies {:?}", resp);
    assert_eq!(resp.strategies.len(), 1);
    // TODO: should be non zero to test pg functions
    let resp = client
        .request(UserListFollowedStrategiesRequest {
            limit: None,
            offset: None,
        })
        .await?;
    assert_eq!(resp.strategies.len(), 0);
    let resp = client
        .request(UserListTopPerformingStrategiesRequest {
            limit: None,
            offset: None,
        })
        .await?;
    info!("User List Top Performing Strategies {:?}", resp);
    assert_eq!(resp.strategies.len(), 1);
    let resp = client
        .request(UserListBackedStrategiesRequest {
            limit: None,
            offset: None,
        })
        .await?;
    info!("User List Backed Strategies {:?}", resp);
    assert_eq!(resp.strategies.len(), 0);
    Ok(())
}
#[tokio::test]
async fn test_user_become_expert() -> Result<()> {
    let _ = setup_logs(LogLevel::Info);
    drop_and_recreate_database()?;
    prepare_expert().await?;

    Ok(())
}

#[tokio::test]
async fn test_user_follow_expert() -> Result<()> {
    let _ = setup_logs(LogLevel::Info);
    drop_and_recreate_database()?;

    let admin = Secp256k1SecretKey::new_random();
    signup("dev-admin", &admin.key).await?;
    let mut admin_client = connect_user("dev-admin", &admin.key).await?;

    let user = Secp256k1SecretKey::new_random();
    signup("user1", &user.key).await?;

    let mut client = connect_user("user1", &user.key).await?;
    let apply_become_expert_resp = client.request(UserApplyBecomeExpertRequest {}).await?;
    info!("User Apply Become Expert {:?}", apply_become_expert_resp);

    let resp = admin_client
        .request(AdminListPendingExpertApplicationsRequest {
            offset: None,
            limit: None,
        })
        .await?;
    assert_eq!(resp.users.len(), 1);

    let resp = admin_client
        .request(AdminApproveUserBecomeExpertRequest {
            user_id: resp.users[0].user_id,
        })
        .await?;
    info!("Approve {:?}", resp);
    let user = Secp256k1SecretKey::new_random();
    signup("user2", &user.key).await?;

    let mut client = connect_user("user2", &user.key).await?;
    let resp = client
        .request(UserFollowExpertRequest {
            expert_id: apply_become_expert_resp.expert_id,
        })
        .await?;
    info!("Follow {:?}", resp);
    assert!(resp.success);
    let resp = client
        .request(UserListFollowedExpertsRequest {
            limit: None,
            offset: None,
        })
        .await?;
    info!("List followed experts {:?}", resp);
    assert_eq!(resp.experts.len(), 1);
    assert_eq!(
        resp.experts[0].expert_id,
        apply_become_expert_resp.expert_id
    );
    let resp = client
        .request(UserUnfollowExpertRequest {
            expert_id: apply_become_expert_resp.expert_id,
        })
        .await?;
    info!("Unfollow {:?}", resp);
    assert!(resp.success);
    let resp = client
        .request(UserListFollowedExpertsRequest {
            limit: None,
            offset: None,
        })
        .await?;
    info!("List followed experts {:?}", resp);
    assert_eq!(resp.experts.len(), 0);
    Ok(())
}

#[tokio::test]
async fn test_user_list_experts() -> Result<()> {
    let _ = setup_logs(LogLevel::Info);
    drop_and_recreate_database()?;
    prepare_expert().await?;

    let user = Secp256k1SecretKey::new_random();
    signup("user2", &user.key).await?;

    let mut client = connect_user("user2", &user.key).await?;
    let resp = client
        .request(UserListExpertsRequest {
            limit: None,
            offset: None,
            expert_id: None,
            user_id: None,
            user_public_id: None,
            username: None,
            family_name: None,
            given_name: None,
            description: None,
            social_media: None,
            sort_by_followers: None,
        })
        .await?;
    info!("Experts {:?}", resp);
    let resp = client
        .request(UserListFeaturedExpertsRequest {
            limit: None,
            offset: None,
        })
        .await?;
    info!("Featured {:?}", resp);
    let resp = client
        .request(UserListTopPerformingExpertsRequest {
            limit: None,
            offset: None,
        })
        .await?;
    info!("Top performing {:?}", resp);
    Ok(())
}

#[tokio::test]
async fn test_user_back_strategy_nth_backer_testnet() -> Result<()> {
    use mc2fi_user::user_back_strategy_sergio_tries_to_help;

    drop_and_recreate_database()?;
    let user_key = Secp256k1SecretKey::new_random();
    let conn_pool = EthereumRpcConnectionPool::new();
    let conn = conn_pool.get(EnumBlockChain::EthereumGoerli).await?;
    let token_addresses = BlockchainCoinAddresses::new();
    let db = connect_to_database(database_test_config()).await?;
    use eth_sdk::DEV_ACCOUNT_PRIV_KEY;
    let master_key = Secp256k1SecretKey::from_str(DEV_ACCOUNT_PRIV_KEY)
        .context("failed to parse dev account private key")?;
    let weth_address_on_goerli = token_addresses
        .get(EnumBlockChain::EthereumGoerli, EnumBlockchainCoin::WETH)
        .ok_or_else(|| eyre!("could not find WETH address on Goerli"))?;

    let usdc_address_on_goerli = token_addresses
        .get(EnumBlockChain::EthereumGoerli, EnumBlockchainCoin::USDC)
        .ok_or_else(|| eyre!("could not find USDC address on Goerli"))?;
    let usdc_decimals = 10u64.pow(
        Erc20Token::new(conn.clone(), usdc_address_on_goerli)?
            .decimals()
            .await?
            .as_u32(),
    ) as i64;

    /* create user */
    let user_signup_ret = db
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

    /* create strategy */
    let strategy_ret = db
        .execute(FunUserCreateStrategyReq {
            user_id: user_signup_ret.user_id,
            name: "TEST".to_string(),
            description: "TEST".to_string(),
            strategy_thesis_url: "TEST".to_string(),
            minimum_backing_amount_usd: 1.0,
            strategy_fee: 1.0,
            expert_fee: 1.0,
            agreed_tos: true,
            blockchain: EnumBlockChain::EthereumGoerli,
            wallet_address: Address::zero().into(),
        })
        .await?
        .into_result()
        .context("failed to create strategy")?;

    /* insert strategy initial token ratio */
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    let ctx = RequestContext {
        connection_id: 0,
        user_id: user_signup_ret.user_id,
        seq: 0,
        method: 0,
        log_id: 0,
        ip_addr: Ipv4Addr::new(127, 0, 0, 1).into(),
        role: EnumRole::Expert as u32,
    };

    /* deploy escrow contract */
    let escrow_contract = EscrowContract::deploy(conn.clone(), master_key.clone()).await?;

    /* instantiate usdc and weth contract wrappers */
    let usdc_contract = Erc20Token::new(conn.clone(), usdc_address_on_goerli)?;
    let weth_contract = Erc20Token::new(conn.clone(), weth_address_on_goerli)?;

    /* make sure dev account has enough USDC on Goerli */
    /* transfer 10 USDC to escrow contract */
    let transfer_tx_hash = usdc_contract
        .transfer(
            &conn,
            master_key.clone(),
            escrow_contract.address(),
            U256::from(10).try_checked_mul(U256::from(usdc_decimals))?,
        )
        .await?;
    wait_for_confirmations_simple(
        &conn.clone().eth(),
        transfer_tx_hash,
        Duration::from_secs(10),
        10,
    )
    .await?;

    /* add escrow contract to database */
    db.execute(FunAdminAddEscrowContractAddressReq {
        pkey_id: 1,
        blockchain: EnumBlockChain::EthereumGoerli,
        address: escrow_contract.address().into(),
    })
    .await?;

    /* add token deposited token to database */
    db.execute(FunAdminAddEscrowTokenContractAddressReq {
        pkey_id: 1,
        blockchain: EnumBlockChain::EthereumGoerli,
        address: usdc_contract.address.into(),
        symbol: "USDC".to_string(),
        short_name: "USDC".to_string(),
        description: "USDC".to_string(),
        is_stablecoin: true,
    })
    .await?
    .into_result()
    .context("failed to add usdc to escrow contract address")?;

    /* add asset held by strategy pool to database */
    db.execute(FunAdminAddEscrowTokenContractAddressReq {
        pkey_id: 2,
        blockchain: EnumBlockChain::EthereumGoerli,
        address: weth_contract.address.into(),
        symbol: "WETH".to_string(),
        short_name: "WETH".to_string(),
        description: "WETH".to_string(),
        is_stablecoin: false,
    })
    .await?
    .into_result()
    .context("failed to add weth to escrow contract address")?;

    /* add strategy initial token ratio */
    db.execute(FunUserAddStrategyInitialTokenRatioReq {
        strategy_id: strategy_ret.strategy_id,
        token_id: 2, // strategy holds weth
        quantity: U256::from_dec_str("100000000")?.into(),
    })
    .await?;

    /* increase user deposit in database (simulating wacher) */
    db.execute(FunWatcherUpsertUserDepositWithdrawBalanceReq {
        user_id: user_signup_ret.user_id,
        token_address: usdc_contract.address.into(),
        escrow_contract_address: escrow_contract.address().into(),
        blockchain: EnumBlockChain::EthereumGoerli,
        old_balance: U256::zero().into(),
        new_balance: U256::from(10)
            .try_checked_mul(U256::from(usdc_decimals))?
            .into(),
    })
    .await?;

    /* get token id */
    let token_ret = db
        .execute(FunUserListEscrowTokenContractAddressReq {
            limit: 1,
            offset: 0,
            blockchain: Some(EnumBlockChain::EthereumGoerli),
            token_id: None,
            address: Some(usdc_address_on_goerli.into()),
            symbol: None,
            is_stablecoin: None,
        })
        .await?
        .into_result()
        .context("no token")?;

    /* deploy strategy pool contract */
    let strategy_pool_contract = StrategyPoolContract::deploy(
        conn.clone(),
        master_key.clone(),
        "TEST".to_string(),
        "TEST".to_string(),
    )
    .await?;

    /* insert strategy pool contract in database */
    let strategy_pool_contract_ret = db
        .execute(FunUserAddStrategyPoolContractReq {
            strategy_id: strategy_ret.strategy_id,
            blockchain: EnumBlockChain::EthereumGoerli,
            address: strategy_pool_contract.address().into(),
        })
        .await?
        .into_result()
        .context("could not add strategy pool contract to database")?;

    /* insert strategy pool contract balance in database */
    /* simulating a previous deposit */
    db.execute(FunWatcherUpsertStrategyPoolContractAssetBalanceReq {
        strategy_pool_contract_id: strategy_pool_contract_ret.strategy_pool_contract_id,
        token_address: weth_contract.address.into(),
        blockchain: EnumBlockChain::EthereumGoerli,
        new_balance: U256::from(10)
            .try_checked_mul(U256::from(usdc_decimals))?
            .into(),
    })
    .await?;

    user_back_strategy_sergio_tries_to_help(
        &conn,
        &ctx,
        &db,
        EnumBlockChain::EthereumGoerli,
        user_signup_ret.user_id,
        U256::from(10).try_checked_mul(U256::from(usdc_decimals))?,
        strategy_ret.strategy_id,
        token_ret.token_id,
        usdc_address_on_goerli,
        escrow_contract,
        &DexAddresses::new(),
        master_key,
    )
    .await?;

    /* check strategy pool contract balance shows more weth than before */
    let strategy_pool_contract_balance_ret = db
        .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
            strategy_pool_contract_id: strategy_pool_contract_ret.strategy_pool_contract_id,
            blockchain: Some(EnumBlockChain::EthereumGoerli),
            token_address: Some(weth_contract.address.into()),
        })
        .await?
        .into_result()
        .context("no strategy pool contract balance")?;

    assert!(strategy_pool_contract_balance_ret.balance > U256::one().into());

    /* check user strategy balance shows strategy tokens from this contract */
    let user_strategy_balance = db
        .execute(FunWatcherListUserStrategyBalanceReq {
            limit: 1,
            offset: 0,
            strategy_id: Some(strategy_ret.strategy_id),
            user_id: Some(user_signup_ret.user_id),
            blockchain: Some(EnumBlockChain::EthereumGoerli),
        })
        .await?
        .first(|x| x.balance)
        .unwrap_or_default();

    assert!(user_strategy_balance > U256::one().into());

    /* fetch user's strategy wallet address on this chain */
    let strategy_wallet_address: Address = db
        .execute(FunUserListStrategyWalletsReq {
            user_id: user_signup_ret.user_id,
            blockchain: Some(EnumBlockChain::EthereumGoerli),
        })
        .await?
        .into_result()
        .context("could not retrieve strategy wallet address")?
        .address
        .into();

    /* check that SP has positive WETH balance */
    let sp_assets = strategy_pool_contract.assets().await?;
    assert_eq!(sp_assets.len(), 1);
    assert_eq!(sp_assets[0], weth_address_on_goerli);
    let (sp_assets_from_another_func, sp_balances) =
        strategy_pool_contract.assets_and_balances().await?;
    assert_eq!(sp_assets_from_another_func.len(), 1);
    assert_eq!(sp_assets_from_another_func[0], weth_address_on_goerli);
    assert_eq!(sp_balances.len(), 1);
    assert!(sp_balances[0] > U256::zero());
    assert!(
        strategy_pool_contract
            .asset_balance(weth_address_on_goerli)
            .await?
            > U256::zero()
    );

    /* check that user's strategy wallet has some strategy tokens */
    assert!(
        strategy_pool_contract
            .balance_of(strategy_wallet_address)
            .await?
            > U256::one()
    );
    /* check that SP has some strategy tokens */
    assert!(strategy_pool_contract.total_supply().await? > U256::one());

    Ok(())
}
