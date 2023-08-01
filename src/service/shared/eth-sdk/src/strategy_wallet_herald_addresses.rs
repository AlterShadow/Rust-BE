use std::str::FromStr;

use web3::types::Address;

use crate::MultiChainAddressTable;
use gen::model::EnumBlockChain;
use std::ops::{Deref, DerefMut};

pub struct StrategyWalletHeraldAddresses(pub MultiChainAddressTable<()>);
impl StrategyWalletHeraldAddresses {
    pub fn empty() -> Self {
        Self(MultiChainAddressTable::empty())
    }
    pub fn new() -> Self {
        let mut this = MultiChainAddressTable::empty();

        this.insert(
            EnumBlockChain::EthereumMainnet,
            (),
            Address::from_str("0x998FEfd555Ee7B4d7177FCA9eA738006B42bFaf3").unwrap(),
        );
        this.insert(
            EnumBlockChain::EthereumGoerli,
            (),
            Address::from_str("0x0a774e2412D10DFa754Eb969d79157FF81939C96").unwrap(),
        );
        this.insert(
            EnumBlockChain::BscMainnet,
            (),
            Address::from_str("0x783eE283715F15Ec61fBE2233C47225364acd63b").unwrap(),
        );
        this.insert(
            EnumBlockChain::BscTestnet,
            (),
            Address::from_str("0x9BBd6eE629d3A28bbeAf5f8Bf9554137fDCE2700").unwrap(),
        );

        Self(this)
    }
}

impl Deref for StrategyWalletHeraldAddresses {
    type Target = MultiChainAddressTable<()>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StrategyWalletHeraldAddresses {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
