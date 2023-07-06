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
            Address::from_str("0x2096ccFc2EfE5dF7Cc838cF39aa5528891666e51").unwrap(),
        );
        this.insert(
            EnumBlockChain::EthereumGoerli,
            (),
            Address::from_str("0x64c1241aE01b245FBA90BA88e15DF78Da2a6a2D9").unwrap(),
        );
        this.insert(
            EnumBlockChain::BscMainnet,
            (),
            Address::from_str("0x115932B4D979E3d7b2b18066Af444663E7F25478").unwrap(),
        );
        this.insert(
            EnumBlockChain::BscTestnet,
            (),
            Address::from_str("0x85fC1F9EC12e16DA681EEd853464F9E162e3C036").unwrap(),
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
