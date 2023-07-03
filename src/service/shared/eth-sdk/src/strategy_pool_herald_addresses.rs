use std::str::FromStr;

use web3::types::Address;

use crate::MultiChainAddressTable;
use gen::model::EnumBlockChain;
use std::ops::{Deref, DerefMut};

pub struct StrategyPoolHeraldAddresses(pub MultiChainAddressTable<()>);
impl StrategyPoolHeraldAddresses {
    pub fn empty() -> Self {
        Self(MultiChainAddressTable::empty())
    }
    pub fn new() -> Self {
        let mut this = MultiChainAddressTable::empty();

        this.insert(
            EnumBlockChain::EthereumMainnet,
            (),
            Address::from_str("0x8C47839e82243cF5E2EE784B115F68e95f3C2ce1").unwrap(),
        );
        this.insert(
            EnumBlockChain::EthereumGoerli,
            (),
            Address::from_str("0x14837279c5FC572B0175e078732fb0694287bf53").unwrap(),
        );
        this.insert(
            EnumBlockChain::BscMainnet,
            (),
            Address::from_str("0x52f47C22F0138f8c6251b6A4dD6E93ee693116e1").unwrap(),
        );
        this.insert(
            EnumBlockChain::BscTestnet,
            (),
            Address::from_str("0x0893abEB433C1a3D63C60F7034c2582Fc7dc8c52").unwrap(),
        );

        Self(this)
    }
}

impl Deref for StrategyPoolHeraldAddresses {
    type Target = MultiChainAddressTable<()>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StrategyPoolHeraldAddresses {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
