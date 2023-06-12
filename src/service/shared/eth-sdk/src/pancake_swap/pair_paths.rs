use std::collections::HashMap;
use std::str::FromStr;

use eyre::*;
use web3::types::{Address, U256};

use crate::evm::DexPath;
use crate::pancake_swap::PancakeV3SingleHopPath;
use crate::BlockchainCoinAddresses;
use crate::PancakePairPathSet;
use gen::model::{EnumBlockChain, EnumBlockchainCoin};

pub struct WorkingPancakePairPaths {
    inner: HashMap<
        EnumBlockChain,
        HashMap<EnumBlockchainCoin, HashMap<EnumBlockchainCoin, PancakePairPathSet>>,
    >,
    addresses: BlockchainCoinAddresses,
}

impl WorkingPancakePairPaths {
    pub fn new() -> Result<Self> {
        let mut this = HashMap::new();
        this.insert(EnumBlockChain::EthereumMainnet, HashMap::new());
        this.insert(EnumBlockChain::EthereumGoerli, HashMap::new());
        this.insert(EnumBlockChain::BscMainnet, HashMap::new());
        this.insert(EnumBlockChain::BscTestnet, HashMap::new());

        let ethereum_mainnet_pairs = this.get_mut(&EnumBlockChain::EthereumMainnet).unwrap();
        ethereum_mainnet_pairs.insert(EnumBlockchainCoin::USDC, HashMap::new());
        ethereum_mainnet_pairs.insert(EnumBlockchainCoin::USDT, HashMap::new());
        ethereum_mainnet_pairs.insert(EnumBlockchainCoin::BUSD, HashMap::new());
        ethereum_mainnet_pairs.insert(EnumBlockchainCoin::WETH, HashMap::new());

        let ethereum_mainnet_usdc_pairs = ethereum_mainnet_pairs
            .get_mut(&EnumBlockchainCoin::USDC)
            .unwrap();
        ethereum_mainnet_usdc_pairs.insert(
            EnumBlockchainCoin::USDT,
            PancakePairPathSet::new(
                Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")?,
                Address::from_str("0xdac17f958d2ee523a2206206994597c13d831ec7")?,
                vec![(
                    "exactInputSingle".to_string(),
                    DexPath::PancakeV3SingleHop(PancakeV3SingleHopPath {
                        token_in: Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")?,
                        token_out: Address::from_str("0xdac17f958d2ee523a2206206994597c13d831ec7")?,
                        fee: U256::from(100),
                    }),
                )],
            )?,
        );
        ethereum_mainnet_usdc_pairs.insert(
            EnumBlockchainCoin::BUSD,
            PancakePairPathSet::new(
                Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")?,
                Address::from_str("0x4fabb145d64652a948d72533023f6e7a623c7c53")?,
                vec![(
                    "swapExactTokensForTokens".to_string(),
                    DexPath::PancakeV2(vec![
                        Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")?,
                        Address::from_str("0x4fabb145d64652a948d72533023f6e7a623c7c53")?,
                    ]),
                )],
            )?,
        );
        ethereum_mainnet_usdc_pairs.insert(
            EnumBlockchainCoin::WETH,
            PancakePairPathSet::new(
                Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")?,
                Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")?,
                vec![(
                    "swapExactTokensForTokens".to_string(),
                    DexPath::PancakeV2(vec![
                        Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")?,
                        Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")?,
                    ]),
                )],
            )?,
        );

        let ethereum_mainnet_usdt_pairs = ethereum_mainnet_pairs
            .get_mut(&EnumBlockchainCoin::USDT)
            .unwrap();
        ethereum_mainnet_usdt_pairs.insert(
            EnumBlockchainCoin::USDC,
            PancakePairPathSet::new(
                Address::from_str("0xdac17f958d2ee523a2206206994597c13d831ec7")?,
                Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")?,
                vec![(
                    "exactInputSingle".to_string(),
                    DexPath::PancakeV3SingleHop(PancakeV3SingleHopPath {
                        token_in: Address::from_str("0xdac17f958d2ee523a2206206994597c13d831ec7")?,
                        token_out: Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")?,
                        fee: U256::from(100),
                    }),
                )],
            )?,
        );
        ethereum_mainnet_usdt_pairs.insert(
            EnumBlockchainCoin::BUSD,
            PancakePairPathSet::new(
                Address::from_str("0xdac17f958d2ee523a2206206994597c13d831ec7")?,
                Address::from_str("0x4fabb145d64652a948d72533023f6e7a623c7c53")?,
                vec![(
                    "swapExactTokensForTokens".to_string(),
                    DexPath::PancakeV2(vec![
                        Address::from_str("0xdac17f958d2ee523a2206206994597c13d831ec7")?,
                        Address::from_str("0x4fabb145d64652a948d72533023f6e7a623c7c53")?,
                    ]),
                )],
            )?,
        );
        ethereum_mainnet_usdt_pairs.insert(
            EnumBlockchainCoin::WETH,
            PancakePairPathSet::new(
                Address::from_str("0xdac17f958d2ee523a2206206994597c13d831ec7")?,
                Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")?,
                vec![(
                    "exactInputSingle".to_string(),
                    DexPath::PancakeV3SingleHop(PancakeV3SingleHopPath {
                        token_in: Address::from_str("0xdac17f958d2ee523a2206206994597c13d831ec7")?,
                        token_out: Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")?,
                        fee: U256::from(500),
                    }),
                )],
            )?,
        );
        let ethereum_mainnet_busd_pairs = ethereum_mainnet_pairs
            .get_mut(&EnumBlockchainCoin::BUSD)
            .unwrap();
        ethereum_mainnet_busd_pairs.insert(
            EnumBlockchainCoin::USDC,
            PancakePairPathSet::new(
                Address::from_str("0x4fabb145d64652a948d72533023f6e7a623c7c53")?,
                Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")?,
                vec![(
                    "swapExactTokensForTokens".to_string(),
                    DexPath::PancakeV2(vec![
                        Address::from_str("0x4fabb145d64652a948d72533023f6e7a623c7c53")?,
                        Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")?,
                    ]),
                )],
            )?,
        );
        ethereum_mainnet_busd_pairs.insert(
            EnumBlockchainCoin::USDT,
            PancakePairPathSet::new(
                Address::from_str("0x4fabb145d64652a948d72533023f6e7a623c7c53")?,
                Address::from_str("0xdac17f958d2ee523a2206206994597c13d831ec7")?,
                vec![(
                    "swapExactTokensForTokens".to_string(),
                    DexPath::PancakeV2(vec![
                        Address::from_str("0x4fabb145d64652a948d72533023f6e7a623c7c53")?,
                        Address::from_str("0xdac17f958d2ee523a2206206994597c13d831ec7")?,
                    ]),
                )],
            )?,
        );
        ethereum_mainnet_busd_pairs.insert(
            EnumBlockchainCoin::WETH,
            PancakePairPathSet::new(
                Address::from_str("0x4fabb145d64652a948d72533023f6e7a623c7c53")?,
                Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")?,
                vec![(
                    "swapExactTokensForTokens".to_string(),
                    DexPath::PancakeV2(vec![
                        Address::from_str("0x4fabb145d64652a948d72533023f6e7a623c7c53")?,
                        Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")?,
                    ]),
                )],
            )?,
        );
        let ethereum_mainnet_weth_pairs = ethereum_mainnet_pairs
            .get_mut(&EnumBlockchainCoin::WETH)
            .unwrap();
        ethereum_mainnet_weth_pairs.insert(
            EnumBlockchainCoin::USDC,
            PancakePairPathSet::new(
                Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")?,
                Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")?,
                vec![(
                    "swapExactTokensForTokens".to_string(),
                    DexPath::PancakeV2(vec![
                        Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")?,
                        Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")?,
                    ]),
                )],
            )?,
        );
        ethereum_mainnet_weth_pairs.insert(
            EnumBlockchainCoin::USDT,
            PancakePairPathSet::new(
                Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")?,
                Address::from_str("0xdac17f958d2ee523a2206206994597c13d831ec7")?,
                vec![(
                    "swapExactTokensForTokens".to_string(),
                    DexPath::PancakeV2(vec![
                        Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")?,
                        Address::from_str("0xdac17f958d2ee523a2206206994597c13d831ec7")?,
                    ]),
                )],
            )?,
        );
        ethereum_mainnet_weth_pairs.insert(
            EnumBlockchainCoin::BUSD,
            PancakePairPathSet::new(
                Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")?,
                Address::from_str("0x4fabb145d64652a948d72533023f6e7a623c7c53")?,
                vec![(
                    "swapExactTokensForTokens".to_string(),
                    DexPath::PancakeV2(vec![
                        Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")?,
                        Address::from_str("0x4fabb145d64652a948d72533023f6e7a623c7c53")?,
                    ]),
                )],
            )?,
        );

        let ethereum_goerli_pairs = this.get_mut(&EnumBlockChain::EthereumGoerli).unwrap();
        ethereum_goerli_pairs.insert(EnumBlockchainCoin::USDC, HashMap::new());
        ethereum_goerli_pairs.insert(EnumBlockchainCoin::WETH, HashMap::new());

        let ethereum_goerli_usdc_pairs = ethereum_goerli_pairs
            .get_mut(&EnumBlockchainCoin::USDC)
            .unwrap();
        ethereum_goerli_usdc_pairs.insert(
            EnumBlockchainCoin::WETH,
            PancakePairPathSet::new(
                Address::from_str("0x07865c6e87b9f70255377e024ace6630c1eaa37f")?,
                Address::from_str("0xb4fbf271143f4fbf7b91a5ded31805e42b2208d6")?,
                vec![(
                    "exactInputSingle".to_string(),
                    DexPath::PancakeV3SingleHop(PancakeV3SingleHopPath {
                        token_in: Address::from_str("0x07865c6e87b9f70255377e024ace6630c1eaa37f")?,
                        token_out: Address::from_str("0xb4fbf271143f4fbf7b91a5ded31805e42b2208d6")?,
                        fee: U256::from(10000),
                    }),
                )],
            )?,
        );

        let ethereum_goerli_weth_pairs = ethereum_goerli_pairs
            .get_mut(&EnumBlockchainCoin::WETH)
            .unwrap();
        ethereum_goerli_weth_pairs.insert(
            EnumBlockchainCoin::USDC,
            PancakePairPathSet::new(
                Address::from_str("0xb4fbf271143f4fbf7b91a5ded31805e42b2208d6")?,
                Address::from_str("0x07865c6e87b9f70255377e024ace6630c1eaa37f")?,
                vec![(
                    "swapExactTokensForTokens".to_string(),
                    DexPath::PancakeV2(vec![
                        Address::from_str("0xb4fbf271143f4fbf7b91a5ded31805e42b2208d6")?,
                        Address::from_str("0x07865c6e87b9f70255377e024ace6630c1eaa37f")?,
                    ]),
                )],
            )?,
        );

        let bsc_mainnet_pairs = this.get_mut(&EnumBlockChain::BscMainnet).unwrap();
        bsc_mainnet_pairs.insert(EnumBlockchainCoin::USDC, HashMap::new());
        bsc_mainnet_pairs.insert(EnumBlockchainCoin::USDT, HashMap::new());
        bsc_mainnet_pairs.insert(EnumBlockchainCoin::BUSD, HashMap::new());
        bsc_mainnet_pairs.insert(EnumBlockchainCoin::WBNB, HashMap::new());

        let bsc_mainnet_usdc_pairs = bsc_mainnet_pairs
            .get_mut(&EnumBlockchainCoin::USDC)
            .unwrap();

        bsc_mainnet_usdc_pairs.insert(
            EnumBlockchainCoin::USDT,
            PancakePairPathSet::new(
                Address::from_str("0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d")?,
                Address::from_str("0x55d398326f99059ff775485246999027b3197955")?,
                vec![(
                    "swapExactTokensForTokens".to_string(),
                    DexPath::PancakeV2(vec![
                        Address::from_str("0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d")?,
                        Address::from_str("0x55d398326f99059ff775485246999027b3197955")?,
                    ]),
                )],
            )?,
        );
        bsc_mainnet_usdc_pairs.insert(
            EnumBlockchainCoin::BUSD,
            PancakePairPathSet::new(
                Address::from_str("0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d")?,
                Address::from_str("0xe9e7cea3dedca5984780bafc599bd69add087d56")?,
                vec![(
                    "exactInputSingle".to_string(),
                    DexPath::PancakeV3SingleHop(PancakeV3SingleHopPath {
                        token_in: Address::from_str("0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d")?,
                        token_out: Address::from_str("0xe9e7cea3dedca5984780bafc599bd69add087d56")?,
                        fee: U256::from(100),
                    }),
                )],
            )?,
        );
        bsc_mainnet_usdc_pairs.insert(
            EnumBlockchainCoin::WBNB,
            PancakePairPathSet::new(
                Address::from_str("0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d")?,
                Address::from_str("0xbb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c")?,
                vec![(
                    "exactInputSingle".to_string(),
                    DexPath::PancakeV3SingleHop(PancakeV3SingleHopPath {
                        token_in: Address::from_str("0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d")?,
                        token_out: Address::from_str("0xbb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c")?,
                        fee: U256::from(100),
                    }),
                )],
            )?,
        );

        let bsc_mainnet_usdt_pairs = bsc_mainnet_pairs
            .get_mut(&EnumBlockchainCoin::USDT)
            .unwrap();
        bsc_mainnet_usdt_pairs.insert(
            EnumBlockchainCoin::USDC,
            PancakePairPathSet::new(
                Address::from_str("0x55d398326f99059ff775485246999027b3197955")?,
                Address::from_str("0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d")?,
                vec![(
                    "exactInputSingle".to_string(),
                    DexPath::PancakeV3SingleHop(PancakeV3SingleHopPath {
                        token_in: Address::from_str("0x55d398326f99059ff775485246999027b3197955")?,
                        token_out: Address::from_str("0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d")?,
                        fee: U256::from(100),
                    }),
                )],
            )?,
        );
        bsc_mainnet_usdt_pairs.insert(
            EnumBlockchainCoin::BUSD,
            PancakePairPathSet::new(
                Address::from_str("0x55d398326f99059ff775485246999027b3197955")?,
                Address::from_str("0xe9e7cea3dedca5984780bafc599bd69add087d56")?,
                vec![(
                    "exactInputSingle".to_string(),
                    DexPath::PancakeV3SingleHop(PancakeV3SingleHopPath {
                        token_in: Address::from_str("0x55d398326f99059ff775485246999027b3197955")?,
                        token_out: Address::from_str("0xe9e7cea3dedca5984780bafc599bd69add087d56")?,
                        fee: U256::from(100),
                    }),
                )],
            )?,
        );
        bsc_mainnet_usdt_pairs.insert(
            EnumBlockchainCoin::WBNB,
            PancakePairPathSet::new(
                Address::from_str("0x55d398326f99059ff775485246999027b3197955")?,
                Address::from_str("0xbb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c")?,
                vec![(
                    "exactInputSingle".to_string(),
                    DexPath::PancakeV3SingleHop(PancakeV3SingleHopPath {
                        token_in: Address::from_str("0x55d398326f99059ff775485246999027b3197955")?,
                        token_out: Address::from_str("0xbb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c")?,
                        fee: U256::from(100),
                    }),
                )],
            )?,
        );

        let bsc_mainnet_busd_pairs = bsc_mainnet_pairs
            .get_mut(&EnumBlockchainCoin::BUSD)
            .unwrap();
        bsc_mainnet_busd_pairs.insert(
            EnumBlockchainCoin::USDC,
            PancakePairPathSet::new(
                Address::from_str("0xe9e7cea3dedca5984780bafc599bd69add087d56")?,
                Address::from_str("0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d")?,
                vec![(
                    "exactInputSingle".to_string(),
                    DexPath::PancakeV3SingleHop(PancakeV3SingleHopPath {
                        token_in: Address::from_str("0xe9e7cea3dedca5984780bafc599bd69add087d56")?,
                        token_out: Address::from_str("0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d")?,
                        fee: U256::from(100),
                    }),
                )],
            )?,
        );
        bsc_mainnet_busd_pairs.insert(
            EnumBlockchainCoin::USDT,
            PancakePairPathSet::new(
                Address::from_str("0xe9e7cea3dedca5984780bafc599bd69add087d56")?,
                Address::from_str("0x55d398326f99059ff775485246999027b3197955")?,
                vec![(
                    "exactInputSingle".to_string(),
                    DexPath::PancakeV3SingleHop(PancakeV3SingleHopPath {
                        token_in: Address::from_str("0xe9e7cea3dedca5984780bafc599bd69add087d56")?,
                        token_out: Address::from_str("0x55d398326f99059ff775485246999027b3197955")?,
                        fee: U256::from(100),
                    }),
                )],
            )?,
        );
        bsc_mainnet_busd_pairs.insert(
            EnumBlockchainCoin::WBNB,
            PancakePairPathSet::new(
                Address::from_str("0xe9e7cea3dedca5984780bafc599bd69add087d56")?,
                Address::from_str("0xbb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c")?,
                vec![(
                    "exactInputSingle".to_string(),
                    DexPath::PancakeV3SingleHop(PancakeV3SingleHopPath {
                        token_in: Address::from_str("0xe9e7cea3dedca5984780bafc599bd69add087d56")?,
                        token_out: Address::from_str("0xbb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c")?,
                        fee: U256::from(100),
                    }),
                )],
            )?,
        );
        let bsc_mainnet_wbnb_pairs = bsc_mainnet_pairs
            .get_mut(&EnumBlockchainCoin::WBNB)
            .unwrap();
        bsc_mainnet_wbnb_pairs.insert(
            EnumBlockchainCoin::USDC,
            PancakePairPathSet::new(
                Address::from_str("0xbb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c")?,
                Address::from_str("0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d")?,
                vec![(
                    "swapExactTokensForTokens".to_string(),
                    DexPath::PancakeV2(vec![
                        Address::from_str("0xbb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c")?,
                        Address::from_str("0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d")?,
                    ]),
                )],
            )?,
        );
        bsc_mainnet_wbnb_pairs.insert(
            EnumBlockchainCoin::USDT,
            PancakePairPathSet::new(
                Address::from_str("0xbb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c")?,
                Address::from_str("0x55d398326f99059ff775485246999027b3197955")?,
                vec![(
                    "swapExactTokensForTokens".to_string(),
                    DexPath::PancakeV2(vec![
                        Address::from_str("0xbb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c")?,
                        Address::from_str("0x55d398326f99059ff775485246999027b3197955")?,
                    ]),
                )],
            )?,
        );
        bsc_mainnet_wbnb_pairs.insert(
            EnumBlockchainCoin::BUSD,
            PancakePairPathSet::new(
                Address::from_str("0xbb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c")?,
                Address::from_str("0xe9e7cea3dedca5984780bafc599bd69add087d56")?,
                vec![(
                    "swapExactTokensForTokens".to_string(),
                    DexPath::PancakeV2(vec![
                        Address::from_str("0xbb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c")?,
                        Address::from_str("0xe9e7cea3dedca5984780bafc599bd69add087d56")?,
                    ]),
                )],
            )?,
        );

        let bsc_testnet_pairs = this.get_mut(&EnumBlockChain::BscTestnet).unwrap();
        bsc_testnet_pairs.insert(EnumBlockchainCoin::BUSD, HashMap::new());
        bsc_testnet_pairs.insert(EnumBlockchainCoin::WBNB, HashMap::new());

        let bsc_testnet_busd_pairs = bsc_testnet_pairs
            .get_mut(&EnumBlockchainCoin::BUSD)
            .unwrap();
        bsc_testnet_busd_pairs.insert(
            EnumBlockchainCoin::WBNB,
            PancakePairPathSet::new(
                Address::from_str("0xab1a4d4f1d656d2450692d237fdd6c7f9146e814")?,
                Address::from_str("0xae13d989dac2f0debff460ac112a837c89baa7cd")?,
                vec![(
                    "exactInputSingle".to_string(),
                    DexPath::PancakeV3SingleHop(PancakeV3SingleHopPath {
                        token_in: Address::from_str("0xab1a4d4f1d656d2450692d237fdd6c7f9146e814")?,
                        token_out: Address::from_str("0xae13d989dac2f0debff460ac112a837c89baa7cd")?,
                        fee: U256::from(10000),
                    }),
                )],
            )?,
        );
        let bsc_testnet_wbnb_pairs = bsc_testnet_pairs
            .get_mut(&EnumBlockchainCoin::WBNB)
            .unwrap();
        bsc_testnet_wbnb_pairs.insert(
            EnumBlockchainCoin::BUSD,
            PancakePairPathSet::new(
                Address::from_str("0xae13d989dac2f0debff460ac112a837c89baa7cd")?,
                Address::from_str("0xab1a4d4f1d656d2450692d237fdd6c7f9146e814")?,
                vec![(
                    "exactInputSingle".to_string(),
                    DexPath::PancakeV3SingleHop(PancakeV3SingleHopPath {
                        token_in: Address::from_str("0xae13d989dac2f0debff460ac112a837c89baa7cd")?,
                        token_out: Address::from_str("0xab1a4d4f1d656d2450692d237fdd6c7f9146e814")?,
                        fee: U256::from(10000),
                    }),
                )],
            )?,
        );

        Ok(Self {
            inner: this,
            addresses: BlockchainCoinAddresses::new(),
        })
    }

    pub fn get_pair(
        &self,
        chain: EnumBlockChain,
        token_in: EnumBlockchainCoin,
        token_out: EnumBlockchainCoin,
    ) -> Result<PancakePairPathSet> {
        Ok(self
            .inner
            .get(&chain)
            .ok_or_else(|| eyre!("chain not found"))?
            .get(&token_in)
            .ok_or_else(|| eyre!("token_in not found"))?
            .get(&token_out)
            .ok_or_else(|| eyre!("token_out not found"))?
            .clone())
    }

    pub fn get_pair_by_address(
        &self,
        chain: EnumBlockChain,
        token_in: Address,
        token_out: Address,
    ) -> Result<PancakePairPathSet> {
        let token_in_enum = self
            .addresses
            .get_by_address(chain, token_in)
            .ok_or_else(|| eyre!("token_in address not found"))?;
        let token_out_enum = self
            .addresses
            .get_by_address(chain, token_out)
            .ok_or_else(|| eyre!("token_out address not found"))?;
        Ok(self
            .inner
            .get(&chain)
            .ok_or_else(|| eyre!("chain not found"))?
            .get(&token_in_enum)
            .ok_or_else(|| eyre!("token_in not found"))?
            .get(&token_out_enum)
            .ok_or_else(|| eyre!("token_out not found"))?
            .clone())
    }
}
