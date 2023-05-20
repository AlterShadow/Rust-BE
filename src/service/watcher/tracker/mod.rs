use crate::tracker::trade::{Chain, Dex};
use std::collections::HashMap;
use std::str::FromStr;
use web3::types::H160;

pub mod calldata;
pub mod escrow;
pub mod ethabi_to_web3;
pub mod pancake_swap;
pub mod trade;
pub mod tx;

pub struct DexAddresses {
    inner: HashMap<Chain, Vec<(Dex, H160)>>,
}
impl Default for DexAddresses {
    fn default() -> Self {
        let mut this = DexAddresses {
            inner: HashMap::new(),
        };

        this.inner.insert(
            Chain::EthereumMainnet,
            vec![(
                Dex::PancakeSwap,
                H160::from_str("0x13f4EA83D0bd40E75C8222255bc855a974568Dd4").unwrap(),
            )],
        );
        this.inner.insert(
            Chain::BscMainnet,
            vec![(
                Dex::PancakeSwap,
                H160::from_str("0x13f4EA83D0bd40E75C8222255bc855a974568Dd4").unwrap(),
            )],
        );
        this.inner.insert(
            Chain::EthereumGoerli,
            vec![(
                Dex::PancakeSwap,
                H160::from_str("0x9a489505a00cE272eAa5e07Dba6491314CaE3796").unwrap(),
            )],
        );
        this.inner.insert(
            Chain::BscTestnet,
            vec![(
                Dex::PancakeSwap,
                H160::from_str("0x9a489505a00cE272eAa5e07Dba6491314CaE3796").unwrap(),
            )],
        );

        this
    }
}
impl DexAddresses {
    pub fn new() -> DexAddresses {
        Default::default()
    }
    pub fn get(&self, chain: &Chain) -> Option<&Vec<(Dex, H160)>> {
        self.inner.get(chain)
    }
}
