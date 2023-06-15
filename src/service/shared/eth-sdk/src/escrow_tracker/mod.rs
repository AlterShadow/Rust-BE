use crate::escrow_tracker::escrow::parse_escrow;
use crate::evm::{parse_quickalert_payload, AppState};
use crate::EscrowAddresses;
use crate::{evm, TransactionFetcher};
use bytes::Bytes;
use eyre::*;
use gen::database::*;
use gen::model::EnumBlockChain;
use http::StatusCode;
use std::sync::Arc;
use tracing::error;

pub mod deposit;
pub mod escrow;

pub async fn handle_eth_escrows(
    state: Arc<AppState>,
    body: Bytes,
    blockchain: EnumBlockChain,
) -> Result<(), StatusCode> {
    let hashes = parse_quickalert_payload(body).map_err(|e| {
        error!("failed to parse QuickAlerts payload: {:?}", e);
        StatusCode::BAD_REQUEST
    })?;

    for hash in hashes {
        let conn = state.eth_pool.get(blockchain).await.map_err(|err| {
            error!("error fetching connection guard: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        let state = state.clone();
        tokio::spawn(async move {
            let tx = match TransactionFetcher::new_and_assume_ready(hash, &conn).await {
                Ok(tx) => tx,
                Err(err) => {
                    error!("error processing tx: {:?}", err);
                    return;
                }
            };
            if let Err(e) = evm::cache_ethereum_transaction(&tx, &state.db, blockchain).await {
                error!("error caching transaction: {:?}", e);
            };

            /* check if it is an escrow to one of our escrow contracts */
            let escrow =
                match parse_escrow(blockchain, &tx, &state.stablecoin_addresses, &state.erc_20) {
                    Ok(escrow) => escrow,
                    Err(e) => {
                        error!("error parsing escrow: {:?}", e);
                        return;
                    }
                };

            let escrow_addresses = EscrowAddresses::new();
            let called_address = match tx.get_to() {
                Some(called_address) => called_address,
                None => {
                    error!("no called address found for tx: {:?}", tx.get_hash());
                    return;
                }
            };
            match escrow_addresses.get_by_address(called_address) {
                Some(_) => {}
                None => {
                    error!("no call to an escrow contract for tx: {:?}", tx.get_hash());
                    return;
                }
            }

            /* check if transaction is from one of our users */
            // TODO: handle an escrow made by an unknown user
            let caller = match tx.get_from() {
                Some(caller) => caller,
                None => {
                    error!("no caller found for tx: {:?}", tx.get_hash());
                    return;
                }
            };

            let user = match state
                .db
                .execute(FunUserGetUserByAddressReq {
                    address: format!("{:?}", caller),
                })
                .await
            {
                Ok(user) => match user.into_result() {
                    Some(user) => user,
                    None => {
                        error!("no user found for address: {:?}", caller);
                        return;
                    }
                },
                Err(e) => {
                    error!("error getting user by address: {:?}", e);
                    return;
                }
            };

            /* insert escrow in ledger */
            match state
                .db
                .execute(FunUserDepositToEscrowReq {
                    user_id: user.user_id,
                    quantity: format!("{:?}", escrow.amount),
                    blockchain: blockchain,
                    user_address: format!("{:?}", escrow.owner),
                    contract_address: format!("{:?}", called_address),
                    transaction_hash: format!("{:?}", tx.get_hash()),
                    receiver_address: format!("{:?}", escrow.recipient),
                })
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    error!("error inserting escrow in ledger: {:?}", e);
                    return;
                }
            };
        });
    }

    Ok(())
}
