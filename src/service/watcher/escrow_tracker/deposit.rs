use crate::escrow_tracker::escrow::{parse_escrow, EscrowTransfer};
use crate::escrow_tracker::StableCoinAddresses;
use eth_sdk::signer::EthereumSigner;
use eth_sdk::utils::wait_for_confirmations_simple;
use eth_sdk::*;
use eyre::*;
use gen::database::*;
use gen::model::EnumBlockChain;
use lib::database::DbClient;
use lib::toolbox::RequestContext;
use std::time::Duration;
use tracing::info;
use web3::ethabi::Contract;
use web3::signing::Key;
use web3::types::{Address, U256};
use web3::Transport;
/*
1. He will transfer tokens C of USDC to escrow address B
2. We track his transfer, calculate how much SP token user will have, and save the "deposit" information to database (this is for multi chain support)
*/
pub async fn on_user_deposit(
    conn: &EthereumRpcConnection,
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
    amount: Address,
    stablecoin_addresses: &StableCoinAddresses,
    strategy_factory_address: Address,
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
        let address = deploy_strategy_contract(
            &conn,
            strategy_factory_address,
            strategy_pool_signer,
            "name".to_string(),
            "token".to_string(),
        )
        .await?;
        user_registered_strategy.evm_contract_address = Some(format!("{:?}", address));
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
    factory_address: Address,
    key: impl Key,
    strategy_token_name: String,
    strategy_token_symbol: String,
) -> Result<Address> {
    info!("Deploying strategy contract");

    let factory = StrategyPoolFactoryContract::new(conn.clone().into_raw().eth(), factory_address)?;

    let tx_hash = factory
        .create_pool(key, strategy_token_name, strategy_token_symbol)
        .await?;

    let mut tx = Transaction::new(tx_hash);
    tx.update(conn).await?;

    match tx.get_status() {
        TxStatus::Successful => {
            info!("Deploy strategy contract success");
            // TODO: implement a wrapper method to retrieve created pool address from receipt logs
        }
        TxStatus::Pending => {
            info!("Deploy strategy contract pending");
        }
        _ => {
            info!("Deploy strategy contract failed");
        }
    }
    wait_for_confirmations_simple(&conn.get_raw().eth(), tx_hash, Duration::from_secs(1), 15)
        .await?;
    info!("Deploy strategy contract success");
    todo!("Could not get strategy contract address yet")
}

use crate::contract_wrappers::escrow::EscrowContract;
use crate::evm::StableCoin;

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
    let escrow_address = stablecoin_addresses
        .get_by_chain_and_token(chain, escrow.token)
        .context("Could not find stablecoin address")?;
    let escrow_contract = EscrowContract::new(conn.clone().into_raw().eth(), escrow_address)?;

    let signer_address = signer.address();
    let tx_hash = escrow_contract
        .transfer_token_to(
            signer,
            signer_address,
            escrow.owner,
            escrow.recipient,
            escrow.amount,
        )
        .await?;

    let tx = Transaction::new(tx_hash);
    Ok(tx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::escrow_tracker::StableCoinAddresses;
    use eth_sdk::erc20::build_erc_20;
    use eth_sdk::mock_erc20::deploy_mock_erc20;
    use eth_sdk::signer::Secp256k1SecretKey;
    use eth_sdk::{EthereumRpcConnectionPool, Transaction};
    use lib::database::{connect_to_database, DatabaseConfig};
    use lib::log::{setup_logs, LogLevel};
    use std::str::FromStr;
    use std::thread::sleep;
    use std::time::Duration;
    use tracing::info;
    use web3::contract::{Contract, Options};
    use web3::signing::Key;
    use web3::types::{TransactionReceipt, H256, U64};
    use web3::Transport;
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
        let user_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;
        let escrow_key = Secp256k1SecretKey::new_random();
        let conn_pool = EthereumRpcConnectionPool::localnet();
        let conn = conn_pool.get_conn().await?;
        let mock_erc20 = deploy_mock_erc20(conn.clone().into_raw().eth(), user_key.clone()).await?;
        mock_erc20
            .mint(user_key.clone(), user_key.address, U256::from(10000))
            .await?;
        let tx_hash = mock_erc20
            .transfer(user_key.clone(), escrow_key.address, U256::from(10000))
            .await?;

        let ctx = RequestContext {
            connection_id: 0,
            user_id: 0,
            seq: 0,
            method: 0,
            log_id: 0,
        };
        let db = connect_to_database(DatabaseConfig {
            user: Some("postgres".to_string()),
            password: Some("123456".to_string()),
            dbname: Some("mc2fi".to_string()),
            ..Default::default()
        })
        .await?;
        // at this step, tx should be passed with quickalert
        let tx = Transaction::new_and_assume_ready(tx_hash, &conn).await?;
        on_user_deposit(
            &conn,
            &ctx,
            &db,
            EnumBlockChain::EthereumMainnet,
            &tx,
            &StableCoinAddresses::new(
                vec![EnumBlockChain::EthereumMainnet],
                vec![vec![(StableCoin::Usdc, mock_erc20.inner.address())]],
            )?,
            &mock_erc20.inner.abi(),
        )
        .await?;

        Ok(())
    }
}
