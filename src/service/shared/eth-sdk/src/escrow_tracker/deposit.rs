use crate::escrow_tracker::escrow::parse_escrow;
use crate::{BlockchainCoinAddresses, EthereumRpcConnection, TransactionReady};
use eyre::*;
use gen::database::*;
use gen::model::EnumBlockChain;
use lib::database::DbClient;
use lib::toolbox::RequestContext;
use web3::ethabi::Contract;

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
    stablecoin_addresses: &BlockchainCoinAddresses,
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
        blockchain: chain,
        user_address: format!("{:?}", esc.owner),
        contract_address: format!("{:?}", tx.get_to().context("no to")?),
        transaction_hash: format!("{:?}", tx.get_hash()),
        receiver_address: format!("{:?}", esc.recipient),
    })
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::mock_erc20::deploy_mock_erc20;
    use crate::signer::Secp256k1SecretKey;
    use crate::{
        wait_for_confirmations_simple, BlockchainCoinAddresses, EthereumRpcConnectionPool,
        EthereumToken, TransactionFetcher, ANVIL_PRIV_KEY_1, ANVIL_PRIV_KEY_2,
    };
    use gen::model::{EnumBlockChain, EnumBlockchainCoin};
    use lib::database::{connect_to_database, database_test_config, drop_and_recreate_database};
    use lib::log::{setup_logs, LogLevel};
    use std::net::Ipv4Addr;
    use web3::types::U256;

    #[tokio::test]
    async fn test_user_ethereum_testnet_transfer() -> Result<()> {
        let _ = setup_logs(LogLevel::Trace);
        let key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;
        let conn_pool = EthereumRpcConnectionPool::new();
        let conn = conn_pool.get(EnumBlockChain::LocalNet).await?;
        let airdrop_tx = EthereumToken::new(conn.clone())
            .transfer(&key.key, key.address, U256::from(20000))
            .await?;
        wait_for_confirmations_simple(&conn.clone().eth(), airdrop_tx, Duration::from_secs(1), 10)
            .await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_user_ethereum_deposit() -> Result<()> {
        let _ = setup_logs(LogLevel::Trace);
        drop_and_recreate_database()?;
        let user_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;
        let admin_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;
        let escrow_key = Secp256k1SecretKey::new_random();
        let conn_pool = EthereumRpcConnectionPool::new();
        let conn = conn_pool.get(EnumBlockChain::LocalNet).await?;
        let erc20_mock = deploy_mock_erc20(conn.clone(), admin_key.clone()).await?;
        erc20_mock
            .mint(&admin_key.key, user_key.address, U256::from(20000000))
            .await?;
        let tx_hash = erc20_mock
            .transfer(&user_key.key, escrow_key.address, U256::from(20000))
            .await?;
        let db = connect_to_database(database_test_config()).await?;
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

        let mut stablecoins = BlockchainCoinAddresses::empty();
        stablecoins.insert(
            EnumBlockChain::EthereumGoerli,
            EnumBlockchainCoin::USDC,
            erc20_mock.address,
        );

        // at this step, tx should be passed with quickalert
        let tx = TransactionFetcher::new_and_assume_ready(tx_hash, &conn).await?;
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
}
