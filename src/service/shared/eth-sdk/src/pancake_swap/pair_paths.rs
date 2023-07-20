use super::parse::{build_pancake_swap_parser, PancakeSwapParser};
use super::{PancakePairPathSet, PancakePoolIndex, PancakeV3SingleHopPath};
use crate::{
    BlockchainCoinAddresses, EthereumRpcConnection, EthereumRpcConnectionPool, TransactionFetcher,
};
use eyre::*;
use gen::database::{FunWatcherListDexPathForPairReq, FunWatcherListDexPathForPairRespRow};
use gen::model::{EnumBlockChain, EnumBlockchainCoin, EnumDex, EnumDexPathFormat};
use lib::database::DbClient;
use lib::types::H256;
use lib::utils::hex_decode;
use std::str::FromStr;
use std::sync::Arc;
use web3::types::{Address, U256};

pub struct WorkingPancakePairPaths {
    inner: Vec<(i64, EnumBlockChain, String, String, PancakePairPathSet)>,
    addresses: Arc<BlockchainCoinAddresses>,
    db: Option<DbClient>,
    pancake_swap_parser: PancakeSwapParser,
    pool: EthereumRpcConnectionPool,
}

impl WorkingPancakePairPaths {
    pub fn empty(addresses: Arc<BlockchainCoinAddresses>, pool: EthereumRpcConnectionPool) -> Self {
        Self {
            inner: Default::default(),
            addresses,
            db: None,
            pancake_swap_parser: build_pancake_swap_parser().unwrap(),
            pool,
        }
    }
    fn insert(
        &mut self,
        chain: EnumBlockChain,
        token_in: EnumBlockchainCoin,
        token_out: EnumBlockchainCoin,
        pair_paths: PancakePairPathSet,
    ) {
        let id = self.inner.len() as i64;
        self.inner.push((
            id,
            chain,
            format!("{token_in:?}"),
            format!("{token_out:?}"),
            pair_paths,
        ));
    }
    // TODO: get rid of these hard-coded values
    pub fn new(
        addresses: Arc<BlockchainCoinAddresses>,
        pool: EthereumRpcConnectionPool,
    ) -> Result<Self> {
        let mut this = Self::empty(addresses, pool);

        this.insert(
            EnumBlockChain::EthereumMainnet,
            EnumBlockchainCoin::USDC,
            EnumBlockchainCoin::USDT,
            PancakePairPathSet::new(vec![(
                "exactInputSingle".to_string(),
                PancakePoolIndex::PancakeV3SingleHop(PancakeV3SingleHopPath {
                    token_in: Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")?,
                    token_out: Address::from_str("0xdac17f958d2ee523a2206206994597c13d831ec7")?,
                    fee: U256::from(100),
                }),
            )])?,
        );
        this.insert(
            EnumBlockChain::EthereumMainnet,
            EnumBlockchainCoin::USDC,
            EnumBlockchainCoin::BUSD,
            PancakePairPathSet::new(vec![(
                "swapExactTokensForTokens".to_string(),
                PancakePoolIndex::PancakeV2(vec![
                    Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")?,
                    Address::from_str("0x4fabb145d64652a948d72533023f6e7a623c7c53")?,
                ]),
            )])?,
        );

        this.insert(
            EnumBlockChain::EthereumMainnet,
            EnumBlockchainCoin::USDC,
            EnumBlockchainCoin::WETH,
            PancakePairPathSet::new(vec![(
                "swapExactTokensForTokens".to_string(),
                PancakePoolIndex::PancakeV2(vec![
                    Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")?,
                    Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")?,
                ]),
            )])?,
        );

        this.insert(
            EnumBlockChain::EthereumMainnet,
            EnumBlockchainCoin::USDT,
            EnumBlockchainCoin::USDC,
            PancakePairPathSet::new(vec![(
                "exactInputSingle".to_string(),
                PancakePoolIndex::PancakeV3SingleHop(PancakeV3SingleHopPath {
                    token_in: Address::from_str("0xdac17f958d2ee523a2206206994597c13d831ec7")?,
                    token_out: Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")?,
                    fee: U256::from(100),
                }),
            )])?,
        );
        this.insert(
            EnumBlockChain::EthereumMainnet,
            EnumBlockchainCoin::USDT,
            EnumBlockchainCoin::BUSD,
            PancakePairPathSet::new(vec![(
                "swapExactTokensForTokens".to_string(),
                PancakePoolIndex::PancakeV2(vec![
                    Address::from_str("0xdac17f958d2ee523a2206206994597c13d831ec7")?,
                    Address::from_str("0x4fabb145d64652a948d72533023f6e7a623c7c53")?,
                ]),
            )])?,
        );
        this.insert(
            EnumBlockChain::EthereumMainnet,
            EnumBlockchainCoin::USDT,
            EnumBlockchainCoin::WETH,
            PancakePairPathSet::new(vec![(
                "exactInputSingle".to_string(),
                PancakePoolIndex::PancakeV3SingleHop(PancakeV3SingleHopPath {
                    token_in: Address::from_str("0xdac17f958d2ee523a2206206994597c13d831ec7")?,
                    token_out: Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")?,
                    fee: U256::from(500),
                }),
            )])?,
        );

        this.insert(
            EnumBlockChain::EthereumMainnet,
            EnumBlockchainCoin::BUSD,
            EnumBlockchainCoin::USDC,
            PancakePairPathSet::new(vec![(
                "swapExactTokensForTokens".to_string(),
                PancakePoolIndex::PancakeV2(vec![
                    Address::from_str("0x4fabb145d64652a948d72533023f6e7a623c7c53")?,
                    Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")?,
                ]),
            )])?,
        );
        this.insert(
            EnumBlockChain::EthereumMainnet,
            EnumBlockchainCoin::BUSD,
            EnumBlockchainCoin::USDT,
            PancakePairPathSet::new(vec![(
                "swapExactTokensForTokens".to_string(),
                PancakePoolIndex::PancakeV2(vec![
                    Address::from_str("0x4fabb145d64652a948d72533023f6e7a623c7c53")?,
                    Address::from_str("0xdac17f958d2ee523a2206206994597c13d831ec7")?,
                ]),
            )])?,
        );
        this.insert(
            EnumBlockChain::EthereumMainnet,
            EnumBlockchainCoin::BUSD,
            EnumBlockchainCoin::WETH,
            PancakePairPathSet::new(vec![(
                "swapExactTokensForTokens".to_string(),
                PancakePoolIndex::PancakeV2(vec![
                    Address::from_str("0x4fabb145d64652a948d72533023f6e7a623c7c53")?,
                    Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")?,
                ]),
            )])?,
        );

        this.insert(
            EnumBlockChain::EthereumMainnet,
            EnumBlockchainCoin::WETH,
            EnumBlockchainCoin::USDC,
            PancakePairPathSet::new(vec![(
                "swapExactTokensForTokens".to_string(),
                PancakePoolIndex::PancakeV2(vec![
                    Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")?,
                    Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")?,
                ]),
            )])?,
        );
        this.insert(
            EnumBlockChain::EthereumMainnet,
            EnumBlockchainCoin::WETH,
            EnumBlockchainCoin::USDT,
            PancakePairPathSet::new(vec![(
                "swapExactTokensForTokens".to_string(),
                PancakePoolIndex::PancakeV2(vec![
                    Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")?,
                    Address::from_str("0xdac17f958d2ee523a2206206994597c13d831ec7")?,
                ]),
            )])?,
        );
        this.insert(
            EnumBlockChain::EthereumMainnet,
            EnumBlockchainCoin::WETH,
            EnumBlockchainCoin::BUSD,
            PancakePairPathSet::new(vec![(
                "swapExactTokensForTokens".to_string(),
                PancakePoolIndex::PancakeV2(vec![
                    Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")?,
                    Address::from_str("0x4fabb145d64652a948d72533023f6e7a623c7c53")?,
                ]),
            )])?,
        );

        this.insert(
            EnumBlockChain::EthereumGoerli,
            EnumBlockchainCoin::USDC,
            EnumBlockchainCoin::WETH,
            PancakePairPathSet::new(vec![(
                "exactInputSingle".to_string(),
                PancakePoolIndex::PancakeV3SingleHop(PancakeV3SingleHopPath {
                    token_in: Address::from_str("0x07865c6e87b9f70255377e024ace6630c1eaa37f")?,
                    token_out: Address::from_str("0xb4fbf271143f4fbf7b91a5ded31805e42b2208d6")?,
                    fee: U256::from(10000),
                }),
            )])?,
        );

        this.insert(
            EnumBlockChain::EthereumGoerli,
            EnumBlockchainCoin::WETH,
            EnumBlockchainCoin::USDC,
            PancakePairPathSet::new(vec![(
                "swapExactTokensForTokens".to_string(),
                PancakePoolIndex::PancakeV2(vec![
                    Address::from_str("0xb4fbf271143f4fbf7b91a5ded31805e42b2208d6")?,
                    Address::from_str("0x07865c6e87b9f70255377e024ace6630c1eaa37f")?,
                ]),
            )])?,
        );

        this.insert(
            EnumBlockChain::BscMainnet,
            EnumBlockchainCoin::USDC,
            EnumBlockchainCoin::USDT,
            PancakePairPathSet::new(vec![(
                "swapExactTokensForTokens".to_string(),
                PancakePoolIndex::PancakeV2(vec![
                    Address::from_str("0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d")?,
                    Address::from_str("0x55d398326f99059ff775485246999027b3197955")?,
                ]),
            )])?,
        );
        this.insert(
            EnumBlockChain::BscMainnet,
            EnumBlockchainCoin::USDC,
            EnumBlockchainCoin::BUSD,
            PancakePairPathSet::new(vec![(
                "exactInputSingle".to_string(),
                PancakePoolIndex::PancakeV3SingleHop(PancakeV3SingleHopPath {
                    token_in: Address::from_str("0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d")?,
                    token_out: Address::from_str("0xe9e7cea3dedca5984780bafc599bd69add087d56")?,
                    fee: U256::from(100),
                }),
            )])?,
        );
        this.insert(
            EnumBlockChain::BscMainnet,
            EnumBlockchainCoin::USDC,
            EnumBlockchainCoin::WBNB,
            PancakePairPathSet::new(vec![(
                "exactInputSingle".to_string(),
                PancakePoolIndex::PancakeV3SingleHop(PancakeV3SingleHopPath {
                    token_in: Address::from_str("0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d")?,
                    token_out: Address::from_str("0xbb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c")?,
                    fee: U256::from(100),
                }),
            )])?,
        );

        this.insert(
            EnumBlockChain::BscMainnet,
            EnumBlockchainCoin::USDT,
            EnumBlockchainCoin::USDC,
            PancakePairPathSet::new(vec![(
                "exactInputSingle".to_string(),
                PancakePoolIndex::PancakeV3SingleHop(PancakeV3SingleHopPath {
                    token_in: Address::from_str("0x55d398326f99059ff775485246999027b3197955")?,
                    token_out: Address::from_str("0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d")?,
                    fee: U256::from(100),
                }),
            )])?,
        );
        this.insert(
            EnumBlockChain::BscMainnet,
            EnumBlockchainCoin::USDT,
            EnumBlockchainCoin::BUSD,
            PancakePairPathSet::new(vec![(
                "exactInputSingle".to_string(),
                PancakePoolIndex::PancakeV3SingleHop(PancakeV3SingleHopPath {
                    token_in: Address::from_str("0x55d398326f99059ff775485246999027b3197955")?,
                    token_out: Address::from_str("0xe9e7cea3dedca5984780bafc599bd69add087d56")?,
                    fee: U256::from(100),
                }),
            )])?,
        );
        this.insert(
            EnumBlockChain::BscMainnet,
            EnumBlockchainCoin::USDT,
            EnumBlockchainCoin::WBNB,
            PancakePairPathSet::new(vec![(
                "exactInputSingle".to_string(),
                PancakePoolIndex::PancakeV3SingleHop(PancakeV3SingleHopPath {
                    token_in: Address::from_str("0x55d398326f99059ff775485246999027b3197955")?,
                    token_out: Address::from_str("0xbb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c")?,
                    fee: U256::from(100),
                }),
            )])?,
        );

        this.insert(
            EnumBlockChain::BscMainnet,
            EnumBlockchainCoin::BUSD,
            EnumBlockchainCoin::USDC,
            PancakePairPathSet::new(vec![(
                "exactInputSingle".to_string(),
                PancakePoolIndex::PancakeV3SingleHop(PancakeV3SingleHopPath {
                    token_in: Address::from_str("0xe9e7cea3dedca5984780bafc599bd69add087d56")?,
                    token_out: Address::from_str("0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d")?,
                    fee: U256::from(100),
                }),
            )])?,
        );
        this.insert(
            EnumBlockChain::BscMainnet,
            EnumBlockchainCoin::BUSD,
            EnumBlockchainCoin::USDT,
            PancakePairPathSet::new(vec![(
                "exactInputSingle".to_string(),
                PancakePoolIndex::PancakeV3SingleHop(PancakeV3SingleHopPath {
                    token_in: Address::from_str("0xe9e7cea3dedca5984780bafc599bd69add087d56")?,
                    token_out: Address::from_str("0x55d398326f99059ff775485246999027b3197955")?,
                    fee: U256::from(100),
                }),
            )])?,
        );
        this.insert(
            EnumBlockChain::BscMainnet,
            EnumBlockchainCoin::BUSD,
            EnumBlockchainCoin::WBNB,
            PancakePairPathSet::new(vec![(
                "exactInputSingle".to_string(),
                PancakePoolIndex::PancakeV3SingleHop(PancakeV3SingleHopPath {
                    token_in: Address::from_str("0xe9e7cea3dedca5984780bafc599bd69add087d56")?,
                    token_out: Address::from_str("0xbb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c")?,
                    fee: U256::from(100),
                }),
            )])?,
        );

        this.insert(
            EnumBlockChain::BscMainnet,
            EnumBlockchainCoin::WBNB,
            EnumBlockchainCoin::USDC,
            PancakePairPathSet::new(vec![(
                "swapExactTokensForTokens".to_string(),
                PancakePoolIndex::PancakeV2(vec![
                    Address::from_str("0xbb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c")?,
                    Address::from_str("0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d")?,
                ]),
            )])?,
        );
        this.insert(
            EnumBlockChain::BscMainnet,
            EnumBlockchainCoin::WBNB,
            EnumBlockchainCoin::USDT,
            PancakePairPathSet::new(vec![(
                "swapExactTokensForTokens".to_string(),
                PancakePoolIndex::PancakeV2(vec![
                    Address::from_str("0xbb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c")?,
                    Address::from_str("0x55d398326f99059ff775485246999027b3197955")?,
                ]),
            )])?,
        );
        this.insert(
            EnumBlockChain::BscMainnet,
            EnumBlockchainCoin::WBNB,
            EnumBlockchainCoin::BUSD,
            PancakePairPathSet::new(vec![(
                "swapExactTokensForTokens".to_string(),
                PancakePoolIndex::PancakeV2(vec![
                    Address::from_str("0xbb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c")?,
                    Address::from_str("0xe9e7cea3dedca5984780bafc599bd69add087d56")?,
                ]),
            )])?,
        );

        this.insert(
            EnumBlockChain::BscTestnet,
            EnumBlockchainCoin::BUSD,
            EnumBlockchainCoin::WBNB,
            PancakePairPathSet::new(vec![(
                "exactInputSingle".to_string(),
                PancakePoolIndex::PancakeV3SingleHop(PancakeV3SingleHopPath {
                    token_in: Address::from_str("0xab1a4d4f1d656d2450692d237fdd6c7f9146e814")?,
                    token_out: Address::from_str("0xae13d989dac2f0debff460ac112a837c89baa7cd")?,
                    fee: U256::from(10000),
                }),
            )])?,
        );

        this.insert(
            EnumBlockChain::BscTestnet,
            EnumBlockchainCoin::WBNB,
            EnumBlockchainCoin::BUSD,
            PancakePairPathSet::new(vec![(
                "exactInputSingle".to_string(),
                PancakePoolIndex::PancakeV3SingleHop(PancakeV3SingleHopPath {
                    token_in: Address::from_str("0xae13d989dac2f0debff460ac112a837c89baa7cd")?,
                    token_out: Address::from_str("0xab1a4d4f1d656d2450692d237fdd6c7f9146e814")?,
                    fee: U256::from(10000),
                }),
            )])?,
        );

        Ok(this)
    }
    pub fn load_from_db(&mut self, db: &DbClient) -> Result<()> {
        self.db = Some(db.clone());
        Ok(())
    }

    fn get_pair(
        &self,
        chain: EnumBlockChain,
        token_in: &str,
        token_out: &str,
    ) -> Result<&PancakePairPathSet> {
        self.inner
            .iter()
            .find(|(_, c, ti, to, _)| *c == chain && *ti == token_in && *to == token_out)
            .map(|(_, _, _, _, p)| p)
            .with_context(|| {
                format!(
                    "pair not found: chain {:?}, token_in {:?}, token_out {:?}",
                    chain, token_in, token_out
                )
            })
    }

    pub async fn get_pair_by_address(
        &self,
        chain: EnumBlockChain,
        token_in: Address,
        token_out: Address,
    ) -> Result<PancakePairPathSet> {
        if let Some(db) = &self.db {
            if let Some(token) = db
                .execute(FunWatcherListDexPathForPairReq {
                    token_in_address: token_in.into(),
                    token_out_address: token_out.into(),
                    blockchain: chain,
                    dex: Some(EnumDex::PancakeSwap),
                    format: None,
                })
                .await?
                .into_result()
            {
                let pool = self.pool.get(chain).await?;
                let path = get_pair_path_from_db(token, &pool).await?;
                return Ok(path);
            }
        }
        let token_in_enum = self
            .addresses
            .get_by_address(chain, token_in)
            .ok_or_else(|| eyre!("token_in {:?} not found", token_out))?;
        let token_out_enum = self
            .addresses
            .get_by_address(chain, token_out)
            .ok_or_else(|| eyre!("token_out {:?} not found", token_out))?;
        self.get_pair(chain, token_in_enum, token_out_enum).cloned()
    }
}

async fn get_pair_path_from_db(
    pair_path_row: FunWatcherListDexPathForPairRespRow,
    conn: &EthereumRpcConnection,
) -> Result<PancakePairPathSet> {
    // TODO: return DexPairPathSet once we support multiple dexes

    let pair_path = match pair_path_row.format {
        EnumDexPathFormat::Json => serde_json::from_str(&pair_path_row.path_data)?,
        EnumDexPathFormat::TransactionData => {
            let pancake_parser = build_pancake_swap_parser()?;
            pancake_parser
                .parse_paths_from_inputs(&hex_decode(&pair_path_row.path_data.as_bytes())?)?
        }
        EnumDexPathFormat::TransactionHash => {
            let tx_hash: H256 = pair_path_row.path_data.parse()?;
            let tx_ready = TransactionFetcher::new_and_assume_ready(tx_hash, &conn).await?;
            let pancake_parser = build_pancake_swap_parser()?;
            pancake_parser.parse_paths_from_inputs(&tx_ready.get_input_data())?
        }
    };

    Ok(pair_path)
}
