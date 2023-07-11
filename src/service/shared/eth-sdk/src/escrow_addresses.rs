use std::str::FromStr;

use web3::types::Address;

use crate::MultiChainAddressTable;
use gen::model::EnumBlockChain;
use std::ops::{Deref, DerefMut};

pub struct EscrowAddresses(pub MultiChainAddressTable<()>);
impl EscrowAddresses {
    pub fn empty() -> Self {
        Self(MultiChainAddressTable::empty())
    }
    pub fn new() -> Self {
        let mut this = MultiChainAddressTable::empty();

        this.insert(
            EnumBlockChain::EthereumMainnet,
            (),
            Address::from_str("0x708a6759da29d3a5D243D7426578d29Edd9Df974").unwrap(),
        );
        this.insert(
            EnumBlockChain::EthereumGoerli,
            (),
            Address::from_str("0x3289004284864183cd59151067c66cd028BEbA35").unwrap(),
        );
        this.insert(
            EnumBlockChain::BscMainnet,
            (),
            Address::from_str("0x551f5868572bc1d43daa6BCB32aCDAa52451EF6c").unwrap(),
        );
        this.insert(
            EnumBlockChain::BscTestnet,
            (),
            Address::from_str("0xF4516FE3b3C0068a988D7CE3982499EecE9b4833").unwrap(),
        );

        Self(this)
    }
}

impl Deref for EscrowAddresses {
    type Target = MultiChainAddressTable<()>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EscrowAddresses {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
