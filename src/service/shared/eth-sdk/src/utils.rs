use eyre::*;
use secp256k1::PublicKey;
use std::path::PathBuf;
use std::time::Duration;
use web3::api::Eth;
use web3::signing::{hash_message, keccak256, recover, Key, RecoveryError, Signature};
use web3::types::{Address, TransactionReceipt, H256, U256};
use web3::Transport;

pub fn eth_public_exponent_to_address(public_exponent: &crypto::PublicExponent) -> Result<Address> {
    let public_key = PublicKey::from_slice(&public_exponent.content).map_err(|_| {
        eyre!(
            "malformed public key: {}",
            hex::encode(&public_exponent.content)
        )
    })?;
    let public_key = public_key.serialize_uncompressed();

    debug_assert_eq!(public_key[0], 0x04);
    let hash = keccak256(&public_key[1..]);

    Ok(Address::from_slice(&hash[12..]))
}

pub fn wei_to_eth(wei_val: web3::types::U256) -> f64 {
    let u = U256::from_str_radix("1000000000000000000", 10).unwrap();
    let n = wei_val / u;
    let f = wei_val % u;
    (n.as_u128() as f64) + f.as_u128() as f64 / 1e18
}

/// Should be used to wait for confirmations
pub async fn wait_for_confirmations_simple<T>(
    eth: &Eth<T>,
    hash: H256,
    poll_interval: Duration,
    max_retry: usize,
) -> Result<TransactionReceipt>
where
    T: Transport,
{
    for _ in 0..max_retry {
        if let Some(receipt) = eth.transaction_receipt(hash).await? {
            return Ok(receipt);
        }
        tokio::time::sleep(poll_interval).await;
    }
    bail!(
        "Transaction {:?} not found within {} retries",
        hash,
        max_retry
    )
}
pub async fn wait_for_confirmations<T>(
    eth: &Eth<T>,
    hash: H256,
    poll_interval: Duration,
    max_retry: usize,
    confirmations: u64,
) -> Result<TransactionReceipt>
where
    T: Transport,
{
    /* wait for transaction to be mined and produce a receipt */
    let mut receipt_at_beginning: Option<TransactionReceipt> = None;
    for _ in 0..max_retry {
        if let Some(receipt) = eth.transaction_receipt(hash).await? {
            receipt_at_beginning = Some(receipt);
        }
        tokio::time::sleep(poll_interval).await;
    }

    /* if receipt was produced, check it's status & wait for confirmations */
    if let Some(receipt) = receipt_at_beginning {
        if receipt.status == Some(web3::types::U64([0])) {
            bail!("transaction reverted");
        }
        let receipt_block_number = receipt.block_number.unwrap().as_u64();
        let mut current_block_number = eth.block_number().await?.as_u64();
        while current_block_number - receipt_block_number < confirmations {
            current_block_number = eth.block_number().await?.as_u64();
            tokio::time::sleep(poll_interval).await;
        }
    } else {
        bail!(
            "transaction {:?} not found within {} retries",
            hash,
            max_retry
        );
    }

    /* after confirmations, fetch the receipt again, and check it's status */
    let receipt_after_confirmations = eth
        .transaction_receipt(hash)
        .await?
        .context("transaction was ommerized after confirmations")?;
    if receipt_after_confirmations.status == Some(web3::types::U64([1])) {
        return Ok(receipt_after_confirmations);
    } else {
        bail!("transaction reverted after confirmations");
    }
}

pub fn encode_signature(sig: &Signature) -> String {
    let mut sig_bytes = vec![];
    sig_bytes.extend_from_slice(sig.r.as_bytes());
    sig_bytes.extend_from_slice(sig.s.as_bytes());
    sig_bytes.push(sig.v as u8 + 27);
    hex::encode(sig_bytes)
}

pub fn get_signed_text(txt: String, signer: impl Key) -> Result<(String, String)> {
    let signature = signer.sign_message(hash_message(txt.as_bytes()).as_bytes())?;

    Ok((hex::encode(&txt), encode_signature(&signature)))
}

pub fn verify_message_address(
    message: &[u8],
    signature: &[u8],
    expected_address: Address,
) -> Result<bool, RecoveryError> {
    if signature.len() != 65 {
        return Err(RecoveryError::InvalidSignature);
    }
    if signature[64] as i32 != 27 && signature[64] as i32 != 28 {
        // only supports 27/28 recovery id for ethereum
        return Err(RecoveryError::InvalidSignature);
    }
    let message_hash = hash_message(message);
    let recovery_id = signature[64] as i32 - 27;
    // info!("Recovery id: {}", recovery_id);
    let addr = recover(&message_hash.0, &signature[..64], recovery_id)?;
    // info!(
    //     "Expected address: {:?}, Recovered address: {:?}",
    //     expected_address, addr
    // );
    Ok(addr == expected_address)
}
#[cfg(test)]
mod tests {
    use crate::signer::Secp256k1SecretKey;
    use crypto::PublicKey;
    use eyre::*;
    use std::println;

    #[test]
    fn test_eth_public_exponent_to_address() -> Result<()> {
        let key = Secp256k1SecretKey::new_random();
        let public_exponent = key.public_exponent()?;
        let address = super::eth_public_exponent_to_address(&public_exponent).unwrap();
        println!("address: {}", address);
        Ok(())
    }
}

#[cfg(test)]
pub fn get_project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned()
}

#[cfg(not(test))]
pub fn get_project_root() -> PathBuf {
    std::fs::canonicalize(".").unwrap()
}
