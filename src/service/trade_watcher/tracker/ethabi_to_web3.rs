use web3::types::{H160, H256, U256};

pub fn convert_h160_ethabi_to_web3(ethabi_h160: ethabi::ethereum_types::H160) -> H160 {
    H160::from_slice(&ethabi_h160.0)
}

pub fn convert_h256_ethabi_to_web3(ethabi_h256: ethabi::ethereum_types::H256) -> H256 {
    H256::from_slice(&ethabi_h256.0)
}

pub fn convert_u256_ethabi_to_web3(ethabi_u256: ethabi::ethereum_types::U256) -> U256 {
    let mut buffer = [0u8; 32];
    ethabi_u256.to_big_endian(&mut buffer);
    U256::from_big_endian(&buffer)
}
