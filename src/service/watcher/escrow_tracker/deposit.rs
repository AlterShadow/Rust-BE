use crate::escrow_tracker::escrow::{parse_escrow, Escrow};
use crate::escrow_tracker::StableCoinAddresses;
use crypto::Signer;
use eth_sdk::erc20::{Erc20Contract, Erc20Token};
use eth_sdk::signer::EthereumSigner;
use eth_sdk::{EthereumNet, Transaction, TransactionReady};
use eyre::*;
use gen::database::{FunUserBackStrategyReq, FunUserGetStrategyFromWalletReq};
use gen::model::EnumBlockChain;
use lib::database::DbClient;
use lib::toolbox::RequestContext;
use std::str::FromStr;
use std::sync::Arc;
use token::CryptoToken;
use tracing::info;
use web3::ethabi::Hash;

pub async fn on_user_deposit(
    ctx: &RequestContext,
    db: &DbClient,
    chain: EnumBlockChain,
    tx: &TransactionReady,
    stablecoin_addresses: &StableCoinAddresses,
    erc_20: &Erc20Contract,
    signer: Arc<dyn Signer>,
) -> Result<()> {
    let signer = EthereumSigner::new(signer)?;

    let user_wallet_address = tx.get_from().context("missing user wallet address")?;
    let esc = parse_escrow(chain, tx, stablecoin_addresses, erc_20)?;
    // let our_valid_address = esc.recipient == "0x000".parse()?;
    let our_valid_address = true;
    ensure!(
        our_valid_address,
        "is not our valid address {:?}",
        esc.recipient
    );
    let mut user_registered_strategy = db
        .execute(FunUserGetStrategyFromWalletReq {
            wallet_address: format!("{:?}", user_wallet_address),
            blockchain: chain.to_string(),
        })
        .await?
        .into_result()
        .context("user_registered_strategy")?;
    db.execute(FunUserBackStrategyReq {
        user_id: ctx.user_id,
        strategy_id: user_registered_strategy.strategy_id,
        quantity: format!("{:?}", esc.amount),
        purchase_wallet: format!("{:?}", user_wallet_address),
        blockchain: chain.to_string(),
        transaction_hash: format!("{:?}", tx.get_hash()),
    })
    .await?;
    if user_registered_strategy.evm_contract_address.is_none() {
        user_registered_strategy.evm_contract_address =
            Some(deploy_strategy_contract(&signer).await?);
    }

    // TODO: use a different signer because our escrow tracker is different from strategy address
    let transaction = transfer_token_to_strategy_contract(
        signer.clone(),
        Escrow {
            token: esc.token,
            amount: esc.amount,
            recipient: user_registered_strategy
                .evm_contract_address
                .unwrap()
                .parse()?,
            owner: signer.address,
        },
        stablecoin_addresses,
    )
    .await?;
    /* Actually, on deposit we'll just transfer to a TBD account and add to his balance */
    /**
     * But, after he decides which strategy to back, we will:
     * 1- make trades with the deposited tokens for the strategy's tokens in correct ratios
     * 2- call "deposit" on the chosen strategy contract with the backer address as receiver of the minted shares
     */
    info!("Transfer token to strategy contract {:?}", transaction);
    Ok(())
}

use crate::contract_wrappers::strategy_pool_factory::StrategyPoolFactoryContract;
use eth_sdk::tx::TxStatus;
pub async fn deploy_strategy_contract(signer: &EthereumSigner) -> Result<String> {
    info!("Deploying strategy contract");

    let conn: web3::eth::Eth<web3::Transport> =
        web3::eth::Eth::new(web3::transports::Http::new("http://localhost:8545").unwrap());
    let factory_address = Address::from_str("strategy pool factory contract address");

    let factory = StrategyPoolFactoryContract::new(conn, factory_address);

    let expert_wallet_address =
        Address::from_str("expert's EOA address corresponding to strategy").unwrap();
    let strategy_token_name = "Strategy Pool Token Name".to_owned();
    let strategy_token_symbol = "Strategy Pool Token SYMBOL".to_owned();
    let backer_deposit_value = U256::from(1);

    let tx_hash = factory
        .create_pool(
            signer,
            signer.address,
            expert_wallet_address,
            strategy_token_name,
            strategy_token_symbol,
            backer_deposit_value,
        )
        .await?;

    let tx = Transaction::new(tx_hash);
    tx.update(conn).await?;

    let mut pool_address: Address;
    match tx.get_status() {
        TxStatus::Successful => {
            info!("Deploy strategy contract success");
            pool_address = factory.get_pool(expert_wallet_address).await?;
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
    signer: EthereumSigner,
    escrow: Escrow,
    stablecoin_addresses: &StableCoinAddresses,
) -> Result<Transaction> {
    let token = Erc20Token::new(
        EthereumNet::Mainnet,
        stablecoin_addresses
            .get_by_chain_and_token(EnumBlockChain::EthereumMainnet, escrow.token)
            .context("No token address registered")?,
    )
    .await?;
    info!(
        "Transferring token from {:?} to strategy contract {:?}",
        escrow.owner, escrow.recipient
    );

    /* the escrow contract holds the tokens, so the transfer comes from it */
    let conn: web3::eth::Eth<web3::Transport> =
        web3::eth::Eth::new(web3::transports::Http::new("http://localhost:8545").unwrap());
    let escrow_address = Address::from_str("escrow contract address");

    let escrow_contract = EscrowContract::new(conn, escrow_address);

    let token_address = Address::from_str("address of token contract that escrow holds");
    let recipient = Address::from_str(
        "receiver of the transfer, still being decided, pending contract architecture refactor",
    );
    let amount = escrow.amount;

    let tx_hash = escrow_contract
        .transfer_token_to(signer, signer.address, token_address, recipient, amount)
        .await?;

    let tx = Transaction::new(tx_hash);
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
    use tracing::info;

    #[tokio::test]
    async fn test_on_user_deposit() -> Result<()> {
        let _ = setup_logs(LogLevel::Trace);
        let key = Secp256k1SecretKey::new_random();
        let conn_pool = EthereumRpcConnectionPool::mainnet();
        let conn = conn_pool.get_conn().await?;
        let tx = Transaction::new_and_assume_ready(
            "0x27e801a5735e5b530535165a18754c074c673263470fc1fad32cca5eb1bc9fea".parse()?,
            &conn,
        )
        .await?;
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
            ..Default::default()
        })
        .await?;

        on_user_deposit(
            &ctx,
            &db,
            EnumBlockChain::EthereumMainnet,
            &tx,
            &StableCoinAddresses::new(),
            &erc20,
            Arc::new(key),
        )
        .await?;
        let trade = parse_escrow(
            EnumBlockChain::EthereumMainnet,
            &tx,
            &StableCoinAddresses::new(),
            &erc20,
        )?;
        info!("trade: {:?}", trade);
        Ok(())
    }
}
