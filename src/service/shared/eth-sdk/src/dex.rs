use gen::model::{EnumBlockChain, EnumDex};
use std::collections::HashMap;
use std::str::FromStr;
use web3::types::H160;

pub struct DexAddresses {
    inner: HashMap<EnumBlockChain, Vec<(EnumDex, H160)>>,
}
impl Default for DexAddresses {
    fn default() -> Self {
        let mut this = DexAddresses {
            inner: HashMap::new(),
        };

        this.inner.insert(
            EnumBlockChain::EthereumMainnet,
            vec![(
                EnumDex::PancakeSwap,
                H160::from_str("0x13f4EA83D0bd40E75C8222255bc855a974568Dd4").unwrap(),
            )],
        );
        this.inner.insert(
            EnumBlockChain::BscMainnet,
            vec![(
                EnumDex::PancakeSwap,
                H160::from_str("0x13f4EA83D0bd40E75C8222255bc855a974568Dd4").unwrap(),
            )],
        );
        this.inner.insert(
            EnumBlockChain::EthereumGoerli,
            vec![(
                EnumDex::PancakeSwap,
                H160::from_str("0x9a489505a00cE272eAa5e07Dba6491314CaE3796").unwrap(),
            )],
        );
        this.inner.insert(
            EnumBlockChain::BscTestnet,
            vec![(
                EnumDex::PancakeSwap,
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
    pub fn get(&self, chain: &EnumBlockChain) -> Option<&Vec<(EnumDex, H160)>> {
        self.inner.get(chain)
    }
}
