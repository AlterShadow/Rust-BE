use crate::contract::ContractDeployer;
use crate::erc20::ERC20_ABI;
use eyre::*;
use web3::api::Eth;
use web3::contract::{Contract, Options};
use web3::signing::Key;
use web3::types::{Address, H256, U256};
use web3::Transport;

const MOCK_ERC20_BYTECODE: &str = include_str!("mock_erc20.bin");

pub struct MockERC20Contract<T: Transport> {
    /* unrestricted mint and burn, but all other restrictions apply */
    pub inner: Contract<T>,
}

impl<T: Transport> MockERC20Contract<T> {
    pub fn new(contract: Contract<T>) -> Result<Self> {
        Ok(Self { inner: contract })
    }

    pub async fn mint(&self, secret: impl Key, to: Address, amount: U256) -> Result<H256> {
        Ok(self
            .inner
            .signed_call("mint", (to, amount), Options::default(), secret)
            .await?)
    }

    pub async fn burn(&self, secret: impl Key, from: Address, amount: U256) -> Result<H256> {
        Ok(self
            .inner
            .signed_call("burn", (from, amount), Options::default(), secret)
            .await?)
    }

    pub async fn transfer(&self, secret: impl Key, to: Address, amount: U256) -> Result<H256> {
        Ok(self
            .inner
            .signed_call("transfer", (to, amount), Options::default(), secret)
            .await?)
    }

    pub async fn transfer_from(
        &self,
        secret: impl Key,
        from: Address,
        to: Address,
        amount: U256,
    ) -> Result<H256> {
        Ok(self
            .inner
            .signed_call(
                "transferFrom",
                (from, to, amount),
                Options::default(),
                secret,
            )
            .await?)
    }

    pub async fn approve(&self, secret: impl Key, spender: Address, amount: U256) -> Result<H256> {
        Ok(self
            .inner
            .signed_call("approve", (spender, amount), Options::default(), secret)
            .await?)
    }

    pub async fn balance_of(&self, owner: Address) -> Result<U256> {
        Ok(self
            .inner
            .query("balanceOf", owner, None, Options::default(), None)
            .await?)
    }

    pub async fn allowance(&self, owner: Address, spender: Address) -> Result<U256> {
        Ok(self
            .inner
            .query(
                "allowance",
                (owner, spender),
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn total_supply(&self) -> Result<U256> {
        Ok(self
            .inner
            .query("totalSupply", (), None, Options::default(), None)
            .await?)
    }
}

pub async fn deploy_mock_erc20<T: Transport>(
    conn: Eth<T>,
    key: impl Key,
) -> Result<MockERC20Contract<T>> {
    let abi_json: serde_json::Value = serde_json::from_str(ERC20_ABI)?;
    let deployer = ContractDeployer::new(conn, abi_json)?.code(MOCK_ERC20_BYTECODE.to_owned());
    Ok(MockERC20Contract::new(
        deployer.sign_with_key_and_execute((), key).await?,
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signer::Secp256k1SecretKey;
    use crate::{EthereumRpcConnectionPool, TxChecker, TxStatus};

    const ANVIL_PRIV_KEY_1: &str =
        "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    const ANVIL_PRIV_KEY_2: &str =
        "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
    const ANVIL_PRIV_KEY_3: &str =
        "5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a";
    const ANVIL_PRIV_KEY_4: &str =
        "7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6";

    #[tokio::test]
    async fn test_mock_erc20_contract() -> Result<()> {
        let key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;
        let conn_pool = EthereumRpcConnectionPool::localnet();
        let conn = conn_pool.get_conn().await?;
        let mock_erc20 = deploy_mock_erc20(conn.get_raw().eth(), key.clone()).await?;
        let tx_checker = TxChecker::new(conn.get_raw().eth());

        let alice = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;
        let bob = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_3)?;
        let charlie = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_4)?;

        /* positive assertions */
        assert_eq!(mock_erc20.balance_of(alice.address).await?, U256::from(0));
        assert_eq!(mock_erc20.total_supply().await?, U256::from(0));
        mock_erc20
            .mint(key.clone(), alice.address, U256::from(10))
            .await?;
        assert_eq!(mock_erc20.balance_of(alice.address).await?, U256::from(10));
        assert_eq!(mock_erc20.total_supply().await?, U256::from(10));
        mock_erc20
            .burn(key.clone(), alice.address, U256::from(5))
            .await?;
        assert_eq!(mock_erc20.balance_of(alice.address).await?, U256::from(5));
        assert_eq!(mock_erc20.total_supply().await?, U256::from(5));

        mock_erc20
            .transfer(alice.clone(), bob.address, U256::from(5))
            .await?;
        assert_eq!(mock_erc20.balance_of(alice.address).await?, U256::from(0));
        assert_eq!(mock_erc20.balance_of(bob.address).await?, U256::from(5));

        mock_erc20
            .approve(bob.clone(), charlie.address, U256::from(5))
            .await?;
        assert_eq!(mock_erc20.balance_of(bob.address).await?, U256::from(5));
        assert_eq!(mock_erc20.balance_of(charlie.address).await?, U256::from(0));
        assert_eq!(
            mock_erc20.allowance(bob.address, charlie.address).await?,
            U256::from(5),
        );
        assert_eq!(mock_erc20.total_supply().await?, U256::from(5));

        mock_erc20
            .transfer_from(charlie.clone(), bob.address, alice.address, U256::from(5))
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
            .burn(key.clone(), alice.address, U256::from(5))
            .await?;
        assert_eq!(mock_erc20.total_supply().await?, U256::from(0));

        /* negative assertions */
        assert_eq!(
            tx_checker
                .status(
                    mock_erc20
                        .transfer(alice.clone(), bob.address, U256::from(1))
                        .await?
                )
                .await?,
            TxStatus::Reverted,
        );
        assert_eq!(
            tx_checker
                .status(
                    mock_erc20
                        .transfer(bob.clone(), alice.address, U256::from(1))
                        .await?
                )
                .await?,
            TxStatus::Reverted,
        );
        assert_eq!(
            tx_checker
                .status(
                    mock_erc20
                        .transfer(charlie.clone(), alice.address, U256::from(1))
                        .await?
                )
                .await?,
            TxStatus::Reverted,
        );
        mock_erc20
            .mint(key.clone(), alice.address, U256::from(10))
            .await?;
        mock_erc20
            .approve(alice.clone(), bob.address, U256::from(5))
            .await?;
        assert_eq!(
            tx_checker
                .status(
                    mock_erc20
                        .transfer_from(bob.clone(), alice.address, charlie.address, U256::from(6),)
                        .await?
                )
                .await?,
            TxStatus::Reverted,
        );
        assert_eq!(
            tx_checker
                .status(
                    mock_erc20
                        .transfer(alice.clone(), charlie.address, U256::from(11))
                        .await?
                )
                .await?,
            TxStatus::Reverted,
        );
        Ok(())
    }
}
