use super::calldata::ContractCall;
use web3::types::{H160, U256};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Chain {
    EthereumMainnet,
    EthereumGoerli,
    BscMainnet,
    BscTestnet,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Dex {
    UniSwap,
    PancakeSwap,
    SushiSwap,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum DexVersion {
    V1,
    V2,
    V3,
}

#[derive(Clone, Debug)]
pub enum Path {
    /* every path for every token_in token_out pair in every dex in every chain must be recorded in the database */
    /* so that we can trigger our own trades in the futures */
    /* note that reciprocals are different pairs with different paths */
    /* i.e. the path for token_in x and token_out y is different from token_in y and token_out x */
    PancakeV2(Vec<H160>),
    PancakeV3SingleHop(PancakeV3SingleHopPath),
    PancakeV3MultiHop(Vec<u8>),
}

#[derive(Clone, Debug)]
pub struct PancakeV3SingleHopPath {
    pub token_in: H160,
    pub token_out: H160,
    pub fee: U256,
}

#[derive(Clone, Debug)]
pub struct Trade {
    pub chain: Chain,
    pub contract: H160,
    pub dex: Dex,
    pub token_in: H160,
    pub token_out: H160,
    pub caller: H160,
    pub amount_in: U256,
    pub amount_out: U256,
    /* some trades go through multiple swap calls because of pool availability */
    /* this means that for some pairs, we must keep track of all swap calls made in order and their paths */
    pub swap_calls: Vec<ContractCall>,
    pub paths: Vec<Path>,
    pub dex_versions: Vec<DexVersion>,
}
