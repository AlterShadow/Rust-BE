pub mod calldata;
pub mod ethabi_to_web3;
pub mod tx;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Chain {
    EthereumMainnet,
    EthereumGoerli,
    BscMainnet,
    BscTestnet,
}
