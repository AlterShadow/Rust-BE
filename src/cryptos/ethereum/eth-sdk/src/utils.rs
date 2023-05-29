use eyre::*;
use secp256k1::PublicKey;
use std::time::Duration;
use tracing::level_filters::LevelFilter;
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, EnvFilter};
use web3::api::Eth;
use web3::signing::{hash_message, keccak256, recover, RecoveryError, Signature};
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

// for testing only
pub fn setup_logs() -> Result<()> {
    LogTracer::init().context("Cannot setup_logs")?;
    let filter = EnvFilter::from_default_env().add_directive(LevelFilter::TRACE.into());

    let subscriber = fmt()
        .with_thread_names(true)
        .with_env_filter(filter)
        .finish();

    tracing::subscriber::set_global_default(subscriber).context("Cannot setup_logs")?;
    Ok(())
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

pub fn encode_signature(sig: &Signature) -> String {
    let mut sig_bytes = vec![];
    sig_bytes.extend_from_slice(sig.r.as_bytes());
    sig_bytes.extend_from_slice(sig.s.as_bytes());
    sig_bytes.push(sig.v as u8 + 27);
    hex::encode(sig_bytes)
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

    #[test]
    fn test_eth_public_exponent_to_address() -> Result<()> {
        let key = Secp256k1SecretKey::new_random();
        let public_exponent = key.public_exponent()?;
        let address = super::eth_public_exponent_to_address(&public_exponent).unwrap();
        println!("address: {}", address);
        Ok(())
    }
}
