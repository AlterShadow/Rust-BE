use crate::contract_wrappers::escrow::EscrowContract;
use crate::contract_wrappers::strategy_pool::StrategyPoolContract;
use crate::escrow_tracker::escrow::{parse_escrow, EscrowTransfer};
use crate::escrow_tracker::StableCoinAddresses;
use crate::evm::StableCoin;
use eth_sdk::*;
use eyre::*;
use gen::database::*;
use gen::model::EnumBlockChain;
use lib::database::DbClient;
use lib::toolbox::RequestContext;
use tracing::info;
use web3::ethabi::Contract;
use web3::signing::Key;
use web3::types::{Address, U256};

/*
1. He will transfer tokens C of USDC to escrow address B
2. We track his transfer, calculate how much SP token user will have, and save the "deposit" information to database (this is for multi chain support)
*/
pub async fn on_user_deposit(
    _conn: &EthereumRpcConnection,
    ctx: &RequestContext,
    db: &DbClient,
    chain: EnumBlockChain,
    tx: &TransactionReady,
    stablecoin_addresses: &StableCoinAddresses,
    erc_20: &Contract,
) -> Result<()> {
    let esc = parse_escrow(chain, tx, stablecoin_addresses, erc_20)?;
    // TODO: let our_valid_address = esc.recipient == "0x000".parse()?;
    let our_valid_address = true;
    ensure!(
        our_valid_address,
        "is not our valid address {:?}",
        esc.recipient
    );

    //TODO: call "transferTokenTo" on escrow contract wrapper and transfer tokens to our EOA

    // USER just deposits to our service
    db.execute(FunUserDepositToEscrowReq {
        user_id: ctx.user_id,
        quantity: format!("{:?}", esc.amount),
        blockchain: chain.to_string(),
        user_address: format!("{:?}", esc.owner),
        contract_address: format!("{:?}", tx.get_to().context("no to")?),
        transaction_hash: format!("{:?}", tx.get_hash()),
        receiver_address: format!("{:?}", esc.recipient),
    })
    .await?;
    Ok(())
}
/*
1. User will decides which strategy S to back with his wallet address A
2. Backend will save his backing decision in database, and transfer his USDC to strategy for copy trading(in this step it may involve auto token conversion)

 */
pub async fn on_user_back_strategy(
    conn: &EthereumRpcConnection,
    ctx: &RequestContext,
    db: &DbClient,
    chain: EnumBlockChain,
    user_wallet_address: Address,
    amount: U256,
    stablecoin_addresses: &StableCoinAddresses,
    strategy_id: i64,
    strategy_pool_signer: impl Key,
    escrow_signer: impl Key,
    stablecoin: StableCoin,
) -> Result<()> {
    let mut user_registered_strategy = db
        .execute(FunUserGetStrategyReq { strategy_id })
        .await?
        .into_result()
        .context("user_registered_strategy")?;
    if user_registered_strategy.evm_contract_address.is_none() {
        let contract = deploy_strategy_contract(
            &conn,
            strategy_pool_signer,
            user_registered_strategy.strategy_name.clone(),
            user_registered_strategy.strategy_name, // strategy symbol
        )
        .await?;
        user_registered_strategy.evm_contract_address = Some(format!("{:?}", contract.address()));
    }
    let sp_tokens = calculate_sp_tokens().await;
    let strategy_address: Address = user_registered_strategy
        .evm_contract_address
        .unwrap()
        .parse()?;

    let escrow_signer_address = escrow_signer.address();
    // we need to trade, not transfer, and then we need to call deposit on the strategy contract
    let transaction = transfer_token_to_strategy_contract(
        conn,
        escrow_signer,
        EscrowTransfer {
            token: stablecoin,
            amount: sp_tokens,
            recipient: strategy_address,
            owner: escrow_signer_address,
        },
        chain,
        stablecoin_addresses,
    )
    .await?;
    // TODO: need to trade deposit token for strategy's tokens and call "deposit" on the strategy contract wrapper
    db.execute(FunUserBackStrategyReq {
        user_id: ctx.user_id,
        strategy_id: user_registered_strategy.strategy_id,
        quantity: format!("{:?}", amount),
        purchase_wallet: format!("{:?}", user_wallet_address),
        blockchain: chain.to_string(),
        transaction_hash: format!("{:?}", transaction.get_hash()),
        earn_sp_tokens: format!("{:?}", sp_tokens),
    })
    .await?;
    info!(
        "Transfer token to strategy contract {:?}",
        transaction.get_hash()
    );

    let _tx = Transaction::new_and_assume_ready(transaction.get_hash(), conn).await?;
    Ok(())
}

pub async fn calculate_sp_tokens() -> U256 {
    // TODO: calculate SP tokens based current price
    U256::from(123)
}
use crate::contract_wrappers::strategy_pool_factory::StrategyPoolFactoryContract;
pub async fn deploy_strategy_contract(
    conn: &EthereumRpcConnection,
    key: impl Key,
    strategy_token_name: String,
    strategy_token_symbol: String,
) -> Result<StrategyPoolContract<EitherTransport>> {
    info!("Deploying strategy contract");

    let strategy = StrategyPoolContract::deploy(
        conn.clone().into_raw(),
        key,
        strategy_token_name,
        strategy_token_symbol,
    )
    .await?;

    info!("Deploy strategy contract success");
    Ok(strategy)
}

pub async fn transfer_token_to_strategy_contract(
    conn: &EthereumRpcConnection,
    signer: impl Key,
    escrow: EscrowTransfer,
    chain: EnumBlockChain,
    stablecoin_addresses: &StableCoinAddresses,
) -> Result<Transaction> {
    // TODO: use Erc20Token for it?
    info!(
        "Transferring token from {:?} to strategy contract {:?}",
        escrow.owner, escrow.recipient
    );
    let token_address = stablecoin_addresses
        .get_by_chain_and_token(chain, escrow.token)
        .context("Could not find stablecoin address")?;
    let escrow_contract = EscrowContract::new(conn.clone().into_raw().eth(), escrow.owner)?;

    let tx_hash = escrow_contract
        .transfer_token_to(signer, token_address, escrow.recipient, escrow.amount)
        .await?;

    let tx = Transaction::new(tx_hash);
    Ok(tx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::escrow_tracker::StableCoinAddresses;
    use eth_sdk::mock_erc20::deploy_mock_erc20;
    use eth_sdk::signer::Secp256k1SecretKey;
    use eth_sdk::{EthereumRpcConnectionPool, Transaction};
    use lib::database::{connect_to_database, drop_and_recreate_database, DatabaseConfig};
    use lib::log::{setup_logs, LogLevel};
    use std::net::Ipv4Addr;

    const ANVIL_PRIV_KEY_1: &str =
        "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    const ANVIL_PRIV_KEY_2: &str =
        "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
    const ANVIL_PRIV_KEY_3: &str =
        "5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a";
    const ANVIL_PRIV_KEY_4: &str =
        "7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6";

    #[tokio::test]
    async fn test_user_ethereum_testnet_transfer() -> Result<()> {
        let _ = setup_logs(LogLevel::Trace);
        let key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;
        let conn_pool = EthereumRpcConnectionPool::localnet();
        let conn = conn_pool.get_conn().await?;
        let airdrop_tx = conn
            .transfer(&key.key, key.address, U256::from(20000))
            .await?;
        conn.get_receipt(airdrop_tx).await?;
        Ok(())
    }
    #[tokio::test]
    async fn test_user_ethereum_deposit() -> Result<()> {
        let _ = setup_logs(LogLevel::Trace);
        drop_and_recreate_database()?;
        let user_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;
        let admin_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;
        let escrow_key = Secp256k1SecretKey::new_random();
        let conn_pool = EthereumRpcConnectionPool::localnet();
        let conn = conn_pool.get_conn().await?;
        let erc20_mock = deploy_mock_erc20(conn.get_raw().clone(), admin_key.clone()).await?;
        erc20_mock
            .mint(&admin_key.key, user_key.address, U256::from(20000000))
            .await?;
        let tx_hash = erc20_mock
            .transfer(&user_key.key, escrow_key.address, U256::from(20000))
            .await?;
        let db = connect_to_database(DatabaseConfig {
            user: Some("postgres".to_string()),
            password: Some("123456".to_string()),
            dbname: Some("mc2fi".to_string()),
            host: Some("localhost".to_string()),
            ..Default::default()
        })
        .await?;
        let ret = db
            .execute(FunAuthSignupReq {
                address: format!("{:?}", user_key.address),
                email: "".to_string(),
                phone: "".to_string(),
                preferred_language: "".to_string(),
                agreed_tos: true,
                agreed_privacy: true,
                ip_address: Ipv4Addr::new(127, 0, 0, 1).into(),
                username: None,
                age: None,
                public_id: 1,
            })
            .await?
            .into_result()
            .context("No user signup resp")?;
        let ctx = RequestContext {
            connection_id: 0,
            user_id: ret.user_id,
            seq: 0,
            method: 0,
            log_id: 0,
        };

        let mut stablecoins = StableCoinAddresses::default();
        stablecoins.inner.insert(
            EnumBlockChain::EthereumGoerli,
            vec![(StableCoin::Usdc, erc20_mock.address)],
        );

        // at this step, tx should be passed with quickalert
        let tx = Transaction::new_and_assume_ready(tx_hash, &conn).await?;
        on_user_deposit(
            &conn,
            &ctx,
            &db,
            EnumBlockChain::EthereumGoerli,
            &tx,
            &stablecoins,
            &erc20_mock.contract.abi(),
        )
        .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_user_ethereum_back_strategy() -> Result<()> {
        let _ = setup_logs(LogLevel::Trace);
        drop_and_recreate_database()?;
        let user_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;
        let admin_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;
        let escrow_key = Secp256k1SecretKey::new_random();
        let conn_pool = EthereumRpcConnectionPool::localnet();
        let conn = conn_pool.get_conn().await?;
        let erc20_mock = deploy_mock_erc20(conn.get_raw().clone(), admin_key.clone()).await?;
        erc20_mock
            .mint(
                &admin_key.key,
                user_key.address,
                U256::from(200000000000i64),
            )
            .await?;
        let tx_hash = erc20_mock
            .transfer(
                &user_key.key,
                escrow_key.address,
                U256::from(20000000000i64),
            )
            .await?;
        let db = connect_to_database(DatabaseConfig {
            user: Some("postgres".to_string()),
            password: Some("123456".to_string()),
            dbname: Some("mc2fi".to_string()),
            host: Some("localhost".to_string()),
            ..Default::default()
        })
        .await?;
        let ret = db
            .execute(FunAuthSignupReq {
                address: format!("{:?}", user_key.address),
                email: "".to_string(),
                phone: "".to_string(),
                preferred_language: "".to_string(),
                agreed_tos: true,
                agreed_privacy: true,
                ip_address: Ipv4Addr::new(127, 0, 0, 1).into(),
                username: None,
                age: None,
                public_id: 1,
            })
            .await?
            .into_result()
            .context("No user signup resp")?;
        let ctx = RequestContext {
            connection_id: 0,
            user_id: ret.user_id,
            seq: 0,
            method: 0,
            log_id: 0,
        };

        let mut stablecoins = StableCoinAddresses::default();
        stablecoins.inner.insert(
            EnumBlockChain::EthereumGoerli,
            vec![(StableCoin::Usdc, erc20_mock.address)],
        );

        // at this step, tx should be passed with quickalert
        let tx = Transaction::new_and_assume_ready(tx_hash, &conn).await?;
        on_user_deposit(
            &conn,
            &ctx,
            &db,
            EnumBlockChain::EthereumGoerli,
            &tx,
            &stablecoins,
            &erc20_mock.contract.abi(),
        )
        .await?;

        let strategy = db
            .execute(FunUserCreateStrategyReq {
                user_id: ctx.user_id,
                name: "TEST".to_string(),
                description: "TEST".to_string(),
            })
            .await?
            .into_result()
            .context("create strategy")?;

        on_user_back_strategy(
            &conn,
            &ctx,
            &db,
            EnumBlockChain::EthereumGoerli,
            user_key.address,
            U256::from(1000),
            &stablecoins,
            strategy.strategy_id,
            &admin_key.key,
            &escrow_key.key,
            StableCoin::Usdc,
        )
        .await?;
        Ok(())
    }
}
