use crate::MultiChainAddressTable;
use gen::model::{EnumBlockChain, EnumDex};
use std::ops::{Deref, DerefMut};

pub struct DexAddresses(MultiChainAddressTable<EnumDex>);
impl DexAddresses {
    pub fn new() -> Self {
        let mut this = MultiChainAddressTable::empty();

        this.insert(
            EnumBlockChain::EthereumMainnet,
            EnumDex::PancakeSwap,
            "0x13f4EA83D0bd40E75C8222255bc855a974568Dd4"
                .parse()
                .unwrap(),
        );
        this.insert(
            EnumBlockChain::BscMainnet,
            EnumDex::PancakeSwap,
            "0x13f4EA83D0bd40E75C8222255bc855a974568Dd4"
                .parse()
                .unwrap(),
        );
        this.insert(
            EnumBlockChain::EthereumGoerli,
            EnumDex::PancakeSwap,
            "0x9a489505a00cE272eAa5e07Dba6491314CaE3796"
                .parse()
                .unwrap(),
        );
        this.insert(
            EnumBlockChain::BscTestnet,
            EnumDex::PancakeSwap,
            "0x9a489505a00cE272eAa5e07Dba6491314CaE3796"
                .parse()
                .unwrap(),
        );

        Self(this)
    }
}
impl Deref for DexAddresses {
    type Target = MultiChainAddressTable<EnumDex>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DexAddresses {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
