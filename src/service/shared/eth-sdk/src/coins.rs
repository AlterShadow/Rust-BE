use crate::MultiChainAddressTable;
use gen::model::{EnumBlockChain, EnumBlockchainCoin};
use std::ops::{Deref, DerefMut};

pub struct BlockchainCoinAddresses(MultiChainAddressTable<EnumBlockchainCoin>);
impl BlockchainCoinAddresses {
    pub fn empty() -> Self {
        Self(MultiChainAddressTable::empty())
    }
    pub fn new() -> Self {
        let mut this = MultiChainAddressTable::empty();
        this.insert(
            EnumBlockChain::EthereumMainnet,
            EnumBlockchainCoin::USDC,
            "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
                .parse()
                .unwrap(),
        );
        this.insert(
            EnumBlockChain::EthereumMainnet,
            EnumBlockchainCoin::USDT,
            "0xdac17f958d2ee523a2206206994597c13d831ec7"
                .parse()
                .unwrap(),
        );
        this.insert(
            EnumBlockChain::EthereumMainnet,
            EnumBlockchainCoin::BUSD,
            "0x4Fabb145d64652a948d72533023f6E7A623C7C53"
                .parse()
                .unwrap(),
        );

        this.insert(
            EnumBlockChain::EthereumGoerli,
            EnumBlockchainCoin::USDC,
            "0x07865c6E87B9F70255377e024ace6630C1Eaa37F"
                .parse()
                .unwrap(),
        );
        this.insert(
            EnumBlockChain::BscMainnet,
            EnumBlockchainCoin::USDC,
            "0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d"
                .parse()
                .unwrap(),
        );
        this.insert(
            EnumBlockChain::BscMainnet,
            EnumBlockchainCoin::USDT,
            "0x55d398326f99059ff775485246999027b3197955"
                .parse()
                .unwrap(),
        );
        this.insert(
            EnumBlockChain::BscMainnet,
            EnumBlockchainCoin::BUSD,
            "0xe9e7cea3dedca5984780bafc599bd69add087d56"
                .parse()
                .unwrap(),
        );
        this.insert(
            EnumBlockChain::BscTestnet,
            EnumBlockchainCoin::BUSD,
            "0xaB1a4d4f1D656d2450692D237fdD6C7f9146e814"
                .parse()
                .unwrap(),
        );

        Self(this)
    }
}
impl Deref for BlockchainCoinAddresses {
    type Target = MultiChainAddressTable<EnumBlockchainCoin>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for BlockchainCoinAddresses {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
