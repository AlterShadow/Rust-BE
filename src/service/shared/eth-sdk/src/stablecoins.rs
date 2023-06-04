use crate::MultiChainAddressTable;
use gen::model::EnumBlockChain;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum StableCoin {
    Usdc,
    Usdt,
    Busd,
}

pub struct StableCoinAddresses(MultiChainAddressTable<StableCoin>);
impl StableCoinAddresses {
    pub fn empty() -> Self {
        Self(MultiChainAddressTable::empty())
    }
    pub fn new() -> Self {
        let mut this = MultiChainAddressTable::empty();
        this.insert(
            EnumBlockChain::EthereumMainnet,
            StableCoin::Usdc,
            "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
                .parse()
                .unwrap(),
        );
        this.insert(
            EnumBlockChain::EthereumMainnet,
            StableCoin::Usdt,
            "0xdac17f958d2ee523a2206206994597c13d831ec7"
                .parse()
                .unwrap(),
        );
        this.insert(
            EnumBlockChain::EthereumMainnet,
            StableCoin::Busd,
            "0x4Fabb145d64652a948d72533023f6E7A623C7C53"
                .parse()
                .unwrap(),
        );

        this.insert(
            EnumBlockChain::EthereumGoerli,
            StableCoin::Usdc,
            "0x07865c6E87B9F70255377e024ace6630C1Eaa37F"
                .parse()
                .unwrap(),
        );
        this.insert(
            EnumBlockChain::BscTestnet,
            StableCoin::Busd,
            "0xaB1a4d4f1D656d2450692D237fdD6C7f9146e814"
                .parse()
                .unwrap(),
        );

        Self(this)
    }
}
impl Deref for StableCoinAddresses {
    type Target = MultiChainAddressTable<StableCoin>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for StableCoinAddresses {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
