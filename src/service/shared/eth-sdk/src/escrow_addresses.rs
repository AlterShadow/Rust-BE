use std::str::FromStr;

use web3::types::Address;

use crate::AddressTable;
use gen::model::EnumBlockChain;
use std::ops::{Deref, DerefMut};

pub struct EscrowAddresses(AddressTable<EnumBlockChain>);
impl EscrowAddresses {
    pub fn empty() -> Self {
        Self(AddressTable::new())
    }
    pub fn new() -> Self {
        let mut this = AddressTable::new();

        this.insert(
            EnumBlockChain::EthereumMainnet,
            Address::from_str("0x0893abEB433C1a3D63C60F7034c2582Fc7dc8c52").unwrap(),
        );
        this.insert(
            EnumBlockChain::EthereumGoerli,
            Address::from_str("0xd74e67AbE5620E7F442DAD04B2bb06ad784633BF").unwrap(),
        );
        this.insert(
            EnumBlockChain::BscMainnet,
            Address::from_str("0x54D4fa025E0239E9BA0c401F8A926b71F804627B").unwrap(),
        );
        this.insert(
            EnumBlockChain::BscTestnet,
            Address::from_str("0x39638cFb8CAcA5aF7E9B5f9ab02Fa0B76B3EAb01").unwrap(),
        );

        Self(this)
    }
}

impl Deref for EscrowAddresses {
    type Target = AddressTable<EnumBlockChain>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EscrowAddresses {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
