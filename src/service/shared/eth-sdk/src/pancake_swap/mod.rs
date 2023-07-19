pub mod execute;
pub mod pair_paths;
pub mod parse;

pub const SMART_ROUTER_ABI_JSON: &str = include_str!("smart_router_v3.json");

use eyre::*;
use serde::{Deserialize, Serialize};
use web3::types::{Address, H160, U256};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PancakePairPathSet {
    /* some trades go through multiple swap calls because of pool availability */
    /* this means that for some pairs, we must keep track of all swap calls made in order and their paths */
    func_names_and_paths: Vec<(String, PancakePoolIndex)>,
}

impl PancakePairPathSet {
    pub fn new(func_names_and_paths: Vec<(String, PancakePoolIndex)>) -> Result<Self> {
        if func_names_and_paths.len() == 0 {
            bail!("empty names and paths");
        }
        Ok(Self {
            func_names_and_paths,
        })
    }

    pub fn len(&self) -> usize {
        self.func_names_and_paths.len()
    }

    pub fn get_func_name(&self, idx: usize) -> Result<String> {
        if idx >= self.len() {
            bail!("index out of bounds");
        }
        Ok(self.func_names_and_paths[idx].0.clone())
    }

    pub fn get_path(&self, idx: usize) -> Result<PancakePoolIndex> {
        if idx >= self.len() {
            bail!("index out of bounds");
        }
        Ok(self.func_names_and_paths[idx].1.clone())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PancakePoolIndex {
    /* every path for every token_in token_out pair in every dex in every chain must be recorded in the database */
    /* so that we can trigger our own trades in the futures */
    /* note that reciprocals are different pairs with different paths */
    /* i.e. the path for token_in x and token_out y is different from token_in y and token_out x */
    PancakeV2(Vec<H160>),
    PancakeV3SingleHop(PancakeV3SingleHopPath),
    PancakeV3MultiHop(Vec<u8>),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PancakeV3SingleHopPath {
    pub token_in: Address,
    pub token_out: Address,
    pub fee: U256,
}

#[derive(Debug)]
pub struct MultiHopPath {
    pub first_token: H160,
    pub fee: U256,
    pub second_token: H160,
}

impl MultiHopPath {
    pub fn from_bytes(path: &[u8]) -> Result<Vec<Self>> {
        if path.len() < 43 {
            /* 20 bytes for address, 3 bytes for uint24, 20 bytes for address */
            bail!("path is too short");
        }

        let mut full_path: Vec<MultiHopPath> = Vec::new();
        let mut first_token: H160 = H160::from_slice(&path[0..20]);
        for i in 0..((path.len() - 20) / 23) {
            let start = 20 + i * 23;
            if start + 23 > path.len() {
                bail!("path does not have enough bytes for reading next path entry");
            }

            let fee_bytes: [u8; 3] = match path[start..start + 3].try_into() {
                Ok(bytes) => bytes,
                Err(e) => {
                    bail!(
                        "error parsing 'path' from PancakeSwap exactInput call: {}",
                        e
                    );
                }
            };
            let fee = U256::from(u32::from_be_bytes([
                0,
                fee_bytes[0],
                fee_bytes[1],
                fee_bytes[2],
            ]));
            let second_token: H160 = H160::from_slice(&path[start + 3..start + 23]);
            full_path.push(MultiHopPath {
                first_token,
                fee,
                second_token,
            });
            first_token = second_token;
        }
        Ok(full_path)
    }

    pub fn get_fee(&self) -> U256 {
        self.fee
    }

    pub fn invert(paths: &Vec<Self>) -> Vec<Self> {
        let mut inverted_paths: Vec<Self> = Vec::with_capacity(paths.len());
        for path in paths.iter().rev() {
            inverted_paths.push(Self {
                first_token: path.second_token,
                fee: path.fee,
                second_token: path.first_token,
            });
        }
        inverted_paths
    }

    pub fn to_bytes(paths: &Vec<Self>) -> Result<Vec<u8>> {
        if paths.is_empty() {
            bail!("paths is empty");
        }
        let max_fee = U256::from(2).pow(U256::from(24)) - U256::from(1);
        let mut res: Vec<u8> = Vec::new();
        for (i, path) in paths.iter().enumerate() {
            if path.fee > max_fee {
                // fee can't be larger than max value for uint24
                bail!("Fee is larger than the maximum value for uint24");
            }
            if i == 0 {
                res.extend_from_slice(&path.first_token.0);
            }
            let mut buffer = [0u8; 32];
            path.fee.to_big_endian(&mut buffer);
            let fee_bytes = &buffer[29..32];
            res.extend_from_slice(fee_bytes);
            res.extend_from_slice(&path.second_token.0);
        }
        Ok(res)
    }
}
