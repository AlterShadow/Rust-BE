use crate::escrow_tracker::escrow::{parse_escrow, EscrowTransfer};
use crate::escrow_tracker::StableCoinAddresses;
use crypto::Signer;
use eth_sdk::erc20::Erc20Contract;
use eth_sdk::signer::EthereumSigner;
use eth_sdk::*;
use eyre::*;
use gen::database::{FunUserBackStrategyReq, FunUserGetStrategyFromWalletReq};
use gen::model::EnumBlockChain;
use lib::database::DbClient;
use lib::toolbox::RequestContext;

use std::sync::Arc;
use tracing::info;
use web3::signing::Key;
use web3::types::{Address, U256};
use web3::Transport;
/*
1. User will decides which strategy S to back with his wallet address A
2. He will transfer tokens C to escrow address B
3. We track his transfer and save the "deposit" information to database (this is for multi chain support)
*/
pub async fn on_user_deposit(
    conn: &EthereumRpcConnection,
    ctx: &RequestContext,
    db: &DbClient,
    chain: EnumBlockChain,
    tx: &TransactionReady,
    stablecoin_addresses: &StableCoinAddresses,
    erc_20: &Erc20Contract,
    signer: impl Key,
) -> Result<()> {
    let esc = parse_escrow(chain, tx, stablecoin_addresses, erc_20)?;
    // TODO: let our_valid_address = esc.recipient == "0x000".parse()?;
    let our_valid_address = true;
    ensure!(
        our_valid_address,
        "is not our valid address {:?}",
        esc.recipient
    );
    // USER just deposits to our service
    db.execute(FunUserDepositToEscrowReq {
        user_id: ctx.user_id,
        quantity: format!("{:?}", esc.amount),
        purchase_wallet: format!("{:?}", user_wallet_address),
        blockchain: chain.to_string(),
        transaction_hash: format!("{:?}", tx.get_hash()),
    })
    .await?;
    Ok(())
}
pub async fn on_user_back_strategy(
    conn: &EthereumRpcConnection,
    ctx: &RequestContext,
    db: &DbClient,
    chain: EnumBlockChain,
    user_wallet_address: Address,
    strategy_address: Address,
    amount: Address,

    stablecoin_addresses: &StableCoinAddresses,
    erc_20: &Erc20Contract,
    signer: impl Key,
) -> Result<()> {
    let mut user_registered_strategy = db
        .execute(FunUserGetStrategyFromWalletReq {
            wallet_address: format!("{:?}", user_wallet_address),
            blockchain: chain.to_string(),
        })
        .await?
        .into_result()
        .context("user_registered_strategy")?;
    if user_registered_strategy.evm_contract_address.is_none() {
        user_registered_strategy.evm_contract_address = Some(
            deploy_strategy_contract(
                &conn,
                "".parse()?,
                &signer,
                "name".to_string(),
                "token".to_string(),
                "address".parse()?,
            )
            .await?,
        );
    }
    let transaction_hash = conn
        .transfer(
            signer,
            strategy_address,
            user_registered_strategy
                .evm_contract_address
                .unwrap()
                .parse()?,
        )
        .await?;
    // TODO: verify transaction_hash
    // TODO: calculate SP tokens based current price
    db.execute(FunUserBackStrategyReq {
        user_id: ctx.user_id,
        strategy_id: user_registered_strategy.strategy_id,
        quantity: format!("{:?}", amount),
        purchase_wallet: format!("{:?}", user_wallet_address),
        blockchain: chain.to_string(),
        transaction_hash: format!("{:?}", transaction_hash),
        earn_sp_tokens: sp_tokens,
    })
    .await?;

    info!("Transfer token to strategy contract {:?}", transaction);
}

use crate::contract_wrappers::strategy_pool_factory::StrategyPoolFactoryContract;
pub async fn deploy_strategy_contract(
    conn: &EthereumRpcConnection,
    factory_address: Address,
    signer: &EthereumSigner,
    strategy_token_name: String,
    strategy_token_symbol: String,
    expert_wallet_address: Address,
) -> Result<String> {
    info!("Deploying strategy contract");

    let factory = StrategyPoolFactoryContract::new(conn.clone().into_raw().eth(), factory_address)?;

    let backer_deposit_value = U256::from(1);

    let tx_hash = factory
        .create_pool(
            signer.clone(),
            signer.address,
            strategy_token_name,
            strategy_token_symbol,
            backer_deposit_value,
        )
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

    Ok(format!("{:?}", signer.address))
}

use crate::contract_wrappers::escrow::EscrowContract;
pub async fn transfer_token_to_strategy_contract(
    conn: &EthereumRpcConnection,
    signer: EthereumSigner,
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

    let tx_hash = escrow_contract
        .transfer_token_to(
            signer.clone(),
            signer.address,
            escrow.owner,
            escrow.recipient,
            escrow.amount,
        )
        .await?;

    let mut tx = Transaction::new(tx_hash);
    tx.update(conn).await?;

    match tx.get_status() {
        TxStatus::Successful => {
            info!("Transfer success");
        }
        TxStatus::Pending => {
            info!("Transfer pending");
        }
        _ => {
            info!("Transfer failed");
        }
    }
    Ok(Transaction::new(tx_hash))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::escrow_tracker::escrow::parse_escrow;
    use crate::escrow_tracker::StableCoinAddresses;
    use eth_sdk::erc20::build_erc_20;
    use eth_sdk::signer::Secp256k1SecretKey;
    use eth_sdk::{EthereumRpcConnectionPool, Transaction};
    use lib::database::{connect_to_database, DatabaseConfig};
    use lib::log::{setup_logs, LogLevel};
    use secp256k1::SecretKey;
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

    struct MockERC20Contract<T: Transport> {
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

        pub async fn approve(
            &self,
            secret: impl Key,
            spender: Address,
            amount: U256,
        ) -> Result<H256> {
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

    async fn deploy_mock_erc20<T: Transport>(
        conn: Eth<T>,
        key: impl Key,
    ) -> Result<MockERC20Contract<T>> {
        let bytecode = include_str!("mock_erc20.bin");
        let abi_file = File::open("abi/test/mock_erc20.json")?;
        let reader = BufReader::new(abi_file);
        let abi_json: serde_json::Value = serde_json::from_reader(reader)?;
        let deployer = ContractDeployer::new(conn, abi_json)?.code(bytecode.to_owned());
        Ok(MockERC20Contract::new(
            deployer.sign_with_key_and_execute((), key).await?,
        )?)
    }

    #[derive(Debug, PartialEq)]
    enum TxStatus {
        Successful,
        Reverted,
        Unknown,
    }

    struct TxChecker<T: Transport> {
        conn: Eth<T>,
    }

    impl<T: Transport> TxChecker<T> {
        fn new(conn: Eth<T>) -> Self {
            Self { conn }
        }

        async fn status(&self, tx_hash: H256) -> Result<TxStatus> {
            /* typically 1 millisecond is enough to guarantee an Anvil receipt */
            sleep(Duration::from_millis(5));
            match self.conn.transaction_receipt(tx_hash).await? {
                Some(receipt) => {
                    if receipt.status == Some(web3::types::U64::from(1)) {
                        Ok(TxStatus::Successful)
                    } else {
                        Ok(TxStatus::Reverted)
                    }
                }
                None => Ok(TxStatus::Unknown),
            }
        }
    }

    #[tokio::test]
    async fn test_mock_erc20_contract() -> Result<()> {
        let key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;
        let conn_pool = EthereumRpcConnectionPool::localnet();
        let conn = conn_pool.get_conn().await?;
        let mock_erc20 = deploy_mock_erc20(conn.into_raw().eth(), &key).await?;
        let tx_checker = TxChecker::new(conn_pool.get_conn().await?.into_raw().eth());

        let alice = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;
        let bob = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_3)?;
        let charlie = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_4)?;

        /* positive assertions */
        assert_eq!(mock_erc20.balance_of(alice.address).await?, U256::from(0));
        assert_eq!(mock_erc20.total_supply().await?, U256::from(0));
        mock_erc20.mint(&key, alice.address, U256::from(10)).await?;
        assert_eq!(mock_erc20.balance_of(alice.address).await?, U256::from(10));
        assert_eq!(mock_erc20.total_supply().await?, U256::from(10));
        mock_erc20.burn(&key, alice.address, U256::from(5)).await?;
        assert_eq!(mock_erc20.balance_of(alice.address).await?, U256::from(5));
        assert_eq!(mock_erc20.total_supply().await?, U256::from(5));

        mock_erc20
            .transfer(&alice, bob.address, U256::from(5))
            .await?;
        assert_eq!(mock_erc20.balance_of(alice.address).await?, U256::from(0));
        assert_eq!(mock_erc20.balance_of(bob.address).await?, U256::from(5));

        mock_erc20
            .approve(&bob, charlie.address, U256::from(5))
            .await?;
        assert_eq!(mock_erc20.balance_of(bob.address).await?, U256::from(5));
        assert_eq!(mock_erc20.balance_of(charlie.address).await?, U256::from(0));
        assert_eq!(
            mock_erc20.allowance(bob.address, charlie.address).await?,
            U256::from(5),
        );
        assert_eq!(mock_erc20.total_supply().await?, U256::from(5));

        mock_erc20
            .transfer_from(&charlie, bob.address, alice.address, U256::from(5))
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
        mock_erc20.burn(&key, alice.address, U256::from(5)).await?;
        assert_eq!(mock_erc20.total_supply().await?, U256::from(0));

        /* negative assertions */
        assert_eq!(
            tx_checker
                .status(
                    mock_erc20
                        .transfer(&alice, bob.address, U256::from(1))
                        .await?
                )
                .await?,
            TxStatus::Reverted,
        );
        assert_eq!(
            tx_checker
                .status(
                    mock_erc20
                        .transfer(&bob, alice.address, U256::from(1))
                        .await?
                )
                .await?,
            TxStatus::Reverted,
        );
        assert_eq!(
            tx_checker
                .status(
                    mock_erc20
                        .transfer(&charlie, alice.address, U256::from(1))
                        .await?
                )
                .await?,
            TxStatus::Reverted,
        );
        mock_erc20.mint(&key, alice.address, U256::from(10)).await?;
        mock_erc20
            .approve(&alice, bob.address, U256::from(5))
            .await?;
        assert_eq!(
            tx_checker
                .status(
                    mock_erc20
                        .transferFrom(&bob, alice.address, charlie.address, U256::from(6),)
                        .await?
                )
                .await?,
            TxStatus::Reverted,
        );
        assert_eq!(
            tx_checker
                .status(
                    mock_erc20
                        .transfer(&alice, charlie.address, U256::from(11))
                        .await?
                )
                .await?,
            TxStatus::Reverted,
        );
        Ok(())
    }

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
        // TODO: transfer should be one of stablecoin ERC20 contract
        let tx_hash = conn
            .transfer(&user_key.key, escrow_key.address, U256::from(20000))
            .await?;
        conn.get_receipt(tx_hash).await?;

        let erc20 = build_erc_20()?;
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
            &StableCoinAddresses::new(),
            &erc20,
            Arc::new(user_key),
        )
        .await?;

        Ok(())
    }
}
