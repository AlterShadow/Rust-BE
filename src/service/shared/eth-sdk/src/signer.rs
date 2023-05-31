use crate::utils;
use crypto::{sign_sync_compact, DerPublicKey, PrivateExponent, PublicExponent, PublicKey, Signer};
use eyre::*;
use secp256k1::{Message, SecretKey, SECP256K1};
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{info, warn};
use web3::signing::{keccak256, recover, Key, SigningError};
use web3::types::{Address, H256};

#[derive(Clone)]
pub struct EthereumSigner {
    pub inner: Arc<dyn Signer>,
    pub address: Address,
}

impl EthereumSigner {
    pub fn new(inner: Arc<dyn Signer>) -> Result<Self> {
        let address = utils::eth_public_exponent_to_address(&inner.public_exponent()?)?;
        Ok(Self { inner, address })
    }
}

fn get_recovery_id(msg: &[u8], s: &[u8], address: Address) -> Result<i32> {
    if recover(msg, s, 0) == Ok(address) {
        Ok(0)
    } else if recover(msg, s, 1) == Ok(address) {
        Ok(1)
    } else {
        Err(eyre!("Failed to recover address"))
    }
}

impl Key for EthereumSigner {
    fn sign(
        &self,
        message: &[u8],
        chain_id: Option<u64>,
    ) -> Result<web3::signing::Signature, SigningError> {
        if message.len() != 32 {
            return Err(SigningError::InvalidMessage);
        }
        let signature = sign_sync_compact(&*self.inner, message).map_err(|x| {
            warn!("sign error: {:?}", x);
            SigningError::InvalidMessage
        })?;
        let recovery_id = get_recovery_id(message, &signature, self.address).map_err(|x| {
            warn!("get_recovery_id error: {:?}", x);
            SigningError::InvalidMessage
        })?;
        let standard_v = recovery_id as u64;
        let v = if let Some(chain_id) = chain_id {
            // When signing with a chain ID, add chain replay protection.
            standard_v + 35 + chain_id * 2
        } else {
            // Otherwise, convert to 'Electrum' notation.
            standard_v + 27
        };
        let r = H256::from_slice(&signature[..32]);
        let s = H256::from_slice(&signature[32..]);

        Ok(web3::signing::Signature { v, r, s })
    }

    fn sign_message(&self, message: &[u8]) -> Result<web3::signing::Signature, SigningError> {
        if message.len() != 32 {
            return Err(SigningError::InvalidMessage);
        }
        let signature = sign_sync_compact(&*self.inner, message).map_err(|x| {
            warn!("sign error: {:?}", x);
            SigningError::InvalidMessage
        })?;
        info!("Signature {}", hex::encode(&signature));
        let recovery_id = get_recovery_id(message, &signature, self.address).map_err(|x| {
            warn!("get_recovery_id error: {:?}", x);
            SigningError::InvalidMessage
        })?;
        let v = recovery_id as u64;
        let r = H256::from_slice(&signature[..32]);
        let s = H256::from_slice(&signature[32..]);

        Ok(web3::signing::Signature { v, r, s })
    }

    fn address(&self) -> Address {
        self.address
    }
}
#[derive(Clone)]
pub struct Secp256k1SecretKey {
    pub key: SecretKey,
    pub pubkey: secp256k1::PublicKey,
    pub address: Address,
}

impl Secp256k1SecretKey {
    pub fn new_from_private_exponent(key: &PrivateExponent) -> Result<Self> {
        let key = SecretKey::from_slice(&key.content)?;
        let pubkey = secp256k1::PublicKey::from_secret_key(SECP256K1, &key);
        let address = utils::eth_public_exponent_to_address(&PublicExponent {
            content: pubkey.serialize().to_vec(),
        })?;
        Ok(Self {
            key,
            pubkey,
            address,
        })
    }
    pub fn new_random() -> Self {
        let secret_key = SecretKey::new(&mut rand::thread_rng());
        Self::new(secret_key)
    }
    pub fn from_str(s: &str) -> Result<Self> {
        let key = if s.starts_with("0x") {
            SecretKey::from_str(&s[2..])?
        } else {
            SecretKey::from_str(s)?
        };
        Ok(Self::new(key))
    }
    pub fn new(key: SecretKey) -> Self {
        let pubkey = secp256k1::PublicKey::from_secret_key(SECP256K1, &key);
        let address = utils::eth_public_exponent_to_address(&PublicExponent {
            content: pubkey.serialize().to_vec(),
        })
        .unwrap();
        Self {
            key,
            pubkey,
            address,
        }
    }
}
impl Deref for Secp256k1SecretKey {
    type Target = secp256k1::SecretKey;
    fn deref(&self) -> &Self::Target {
        &self.key
    }
}
impl PublicKey for Secp256k1SecretKey {
    fn public_key(&self) -> Result<DerPublicKey> {
        todo!()
    }

    fn public_exponent(&self) -> Result<PublicExponent> {
        self.pubkey
            .serialize_uncompressed()
            .to_vec()
            .try_into()
            .map_err(|_| eyre!("Failed to convert public exponent"))
    }

    fn verify(&self, data: &[u8], signature: &[u8]) -> Result<bool> {
        let message = Message::from_slice(data).unwrap();
        let signature = secp256k1::ecdsa::Signature::from_compact(signature).unwrap();
        SECP256K1
            .verify_ecdsa(&message, &signature, &self.pubkey)
            .expect("invalid signature");
        Ok(true)
    }
}
#[async_trait::async_trait]
impl Signer for Secp256k1SecretKey {
    async fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        let message = Message::from_slice(data).map_err(|_| SigningError::InvalidMessage)?;
        let signature = SECP256K1.sign_ecdsa(&message, &self.key);
        let signature_compact = signature.serialize_der();
        Ok(signature_compact.to_vec())
    }
}

/// Gets the address of a public key.
///
/// The public address is defined as the low 20 bytes of the keccak hash of
/// the public key. Note that the public key returned from the `secp256k1`
/// crate is 65 bytes long, that is because it is prefixed by `0x04` to
/// indicate an uncompressed public key; this first byte is ignored when
/// computing the hash.
pub fn public_key_address(public_key: &secp256k1::PublicKey) -> Address {
    let public_key = public_key.serialize_uncompressed();

    debug_assert_eq!(public_key[0], 0x04);
    let hash = keccak256(&public_key[1..]);

    Address::from_slice(&hash[12..])
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::eth_public_exponent_to_address;
    use web3::signing::{keccak256, Key};

    #[test]
    fn test_convert_key_to_secp255k1() -> Result<()> {
        let key = Secp256k1SecretKey::new_random();
        let exp = key.public_exponent()?;
        println!("exp len {}", exp.content.len());
        let addr = eth_public_exponent_to_address(&exp)?;
        println!("addr: {:?}", addr);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_btc_sign_messages() -> Result<()> {
        let key_owned = Secp256k1SecretKey::new_random();
        let msg = keccak256(b"hello world");
        // let sig2 = key2.sign_message(&msg)?;
        // println!("sig2: {:?}", sig2.v);
        let sig1 = key_owned.sign_message(&msg)?;
        println!("sig1: {:?}", sig1.v);
        // assert_eq!(sig1.v, sig2.v);
        Ok(())
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn test_sign_messages() -> Result<()> {
        let key = Secp256k1SecretKey::new_random();
        // println!("Private key {}", hex::encode(key.private_key()?.content));
        // let key2 = &SecretKey::from_slice(&key.private_exponent()?.content)?;
        // let key3 = Secp256k1SecretKey::new(key2.clone());
        let key1 = EthereumSigner::new(Arc::new(key))?;
        let msg = keccak256(b"hello world");
        // let sig3 = key3.sign_message(&msg)?;
        // println!("sig3: {:?}", sig3.v);
        // let sig2 = key2.sign_message(&msg)?;
        // println!("sig2: {:?}", sig2.v);
        let sig1 = key1.sign_message(&msg)?;
        println!("sig1: {:?}", sig1.v);
        Ok(())
    }
}
