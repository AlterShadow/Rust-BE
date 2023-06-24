use crate::contract::{read_abi_from_solc_output, ContractDeployer};
use crate::erc20::Erc20Token;
use crate::utils::get_project_root;
use crate::EitherTransport;
use eyre::*;
use web3::contract::Options;
use web3::signing::Key;
use web3::Web3;

pub async fn deploy_mock_erc20(
    conn: Web3<EitherTransport>,
    key: impl Key + Clone,
) -> Result<Erc20Token> {
    let base = get_project_root().parent().unwrap().to_owned();
    let abi_json = read_abi_from_solc_output(
        &base.join("app.mc2.fi-solidity/out/MockToken.sol/MockToken.json"),
    )?;
    let bin =
        std::fs::read_to_string(base.join("app.mc2.fi-solidity/out/MockToken.sol/MockToken.bin"))?;
    // web3::contract::web3 never worked: Abi error: Invalid data for ABI json
    let deployer = ContractDeployer::new(conn.eth(), abi_json.clone())?
        .code(bin)
        .options(Options {
            gas: Some(1000000.into()),
            gas_price: None,
            value: None,
            nonce: None,
            condition: None,
            transaction_type: None,
            access_list: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
        });
    Ok(Erc20Token::new_with_abi(
        conn,
        deployer.sign_with_key_and_execute((), key).await?.address(),
        web3::ethabi::Contract::load(abi_json.to_string().as_bytes())?,
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signer::Secp256k1SecretKey;
    use crate::{
        EthereumRpcConnectionPool, TxChecker, ANVIL_PRIV_KEY_1, ANVIL_PRIV_KEY_2,
        ANVIL_PRIV_KEY_3, ANVIL_PRIV_KEY_4,
    };
    use gen::model::EnumBlockChain;
    use web3::types::U256;

    #[tokio::test]
    async fn test_mock_erc20_contract() -> Result<()> {
        let key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;
        let conn_pool = EthereumRpcConnectionPool::new();
        let conn = conn_pool.get(EnumBlockChain::LocalNet).await?;
        let mock_erc20 = deploy_mock_erc20(conn.clone(), key.clone()).await?;
        let _tx_checker = TxChecker::new(conn.eth());

        let alice = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;
        let bob = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_3)?;
        let charlie = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_4)?;

        /* positive assertions */
        assert_eq!(mock_erc20.balance_of(alice.address).await?, U256::from(0));
        assert_eq!(mock_erc20.total_supply().await?, U256::from(0));
        mock_erc20
            .mint(&conn, key.clone(), alice.address, U256::from(10))
            .await?;
        assert_eq!(mock_erc20.balance_of(alice.address).await?, U256::from(10));
        assert_eq!(mock_erc20.total_supply().await?, U256::from(10));
        mock_erc20
            .burn(&conn, key.clone(), alice.address, U256::from(5))
            .await?;
        assert_eq!(mock_erc20.balance_of(alice.address).await?, U256::from(5));
        assert_eq!(mock_erc20.total_supply().await?, U256::from(5));
        mock_erc20
            .transfer(&conn, alice.clone(), bob.address, U256::from(5))
            .await?;
        assert_eq!(mock_erc20.balance_of(alice.address).await?, U256::from(0));
        assert_eq!(mock_erc20.balance_of(bob.address).await?, U256::from(5));

        mock_erc20
            .approve(&conn, bob.clone(), charlie.address, U256::from(5))
            .await?;
        assert_eq!(mock_erc20.balance_of(bob.address).await?, U256::from(5));
        assert_eq!(mock_erc20.balance_of(charlie.address).await?, U256::from(0));
        assert_eq!(
            mock_erc20.allowance(bob.address, charlie.address).await?,
            U256::from(5),
        );
        assert_eq!(mock_erc20.total_supply().await?, U256::from(5));

        mock_erc20
            .transfer_from(
                &conn,
                charlie.clone(),
                bob.address,
                alice.address,
                U256::from(5),
            )
            .await?;
        assert_eq!(
            mock_erc20.allowance(bob.address, charlie.address).await?,
            U256::from(0),
        );
        assert_eq!(mock_erc20.balance_of(alice.address).await?, U256::from(5));
        assert_eq!(mock_erc20.balance_of(bob.address).await?, U256::from(0));
        assert_eq!(mock_erc20.balance_of(charlie.address).await?, U256::from(0));
        assert_eq!(mock_erc20.total_supply().await?, U256::from(5));

        /* reset */
        mock_erc20
            .burn(&conn, key.clone(), alice.address, U256::from(5))
            .await?;
        assert_eq!(mock_erc20.total_supply().await?, U256::from(0));

        /* negative assertions */
        assert!(matches!(
            mock_erc20
                .transfer(&conn, alice.clone(), bob.address, U256::from(1))
                .await,
            Err(_)
        ));
        assert!(matches!(
            mock_erc20
                .transfer(&conn, bob.clone(), alice.address, U256::from(1))
                .await,
            Err(_)
        ));
        assert!(matches!(
            mock_erc20
                .transfer(&conn, charlie.clone(), alice.address, U256::from(1))
                .await,
            Err(_)
        ));
        mock_erc20
            .mint(&conn, key.clone(), alice.address, U256::from(10))
            .await?;
        mock_erc20
            .approve(&conn, alice.clone(), bob.address, U256::from(5))
            .await?;
        assert!(matches!(
            mock_erc20
                .transfer_from(
                    &conn,
                    bob.clone(),
                    alice.address,
                    charlie.address,
                    U256::from(6),
                )
                .await,
            Err(_)
        ));
        assert!(matches!(
            mock_erc20
                .transfer(&conn, alice.clone(), charlie.address, U256::from(11))
                .await,
            Err(_)
        ));
        Ok(())
    }
}
