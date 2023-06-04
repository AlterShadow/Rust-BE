use std::str::FromStr;

use crate::dex_tracker::PancakePairPathSet;

use eyre::*;

use web3::contract::{Contract, Options};
use web3::ethabi::Token;
use web3::signing::Key;
use web3::types::{Address, H256, U256};
use web3::{Transport, Web3};

use crate::dex_tracker::v3::multi_hop::MultiHopPath;

use crate::evm::DexPath;

const SMART_ROUTER_ABI_JSON: &str =
    include_str!("../../../../../../abi/pancake_swap/smart_router_v3.json");

/**	Contract Wrapper for PancakeSwap Smart Router V3
 *
 *	- has copy trade function to repeat swap calls / pool indexes with different parameters
 *	- simplifies all calls to "exact in" type swaps (only amount_in and amount_out_minimum)
 *	- saves GAS by using multicall for multiple swaps
 *	- saves GAS by calling swap functions directly for single swaps
 *	- saves GAS by using internal flag address to refer to this contract
 */
#[derive(Debug, Clone)]
pub struct PancakeSmartRouterV3Contract<T: Transport> {
    contract: Contract<T>,
    w3: Web3<T>,
    refer_to_self_flag: Address,
}

impl<T: Transport> PancakeSmartRouterV3Contract<T> {
    pub fn new(w3: Web3<T>, address: Address) -> Result<Self> {
        let contract = Contract::from_json(w3.eth(), address, SMART_ROUTER_ABI_JSON.as_bytes())?;
        let refer_to_self_flag = Address::from_str("0x0000000000000000000000000000000000000002")?;
        Ok(Self {
            contract,
            w3,
            refer_to_self_flag,
        })
    }

    pub fn address(&self) -> Address {
        self.contract.address()
    }

    pub async fn copy_trade(
        &self,
        signer: impl Key + Clone,
        paths: PancakePairPathSet,
        amount_in: U256,
        amount_out_minimum: U256,
    ) -> Result<H256> {
        let recipient = signer.address();
        match paths.len() {
            0 => bail!("no swap paths"),
            /* if only one swap call, call swap directly */
            /* saves GAS compared to multicall that would call contract +1 times */
            1 => {
                return Ok(self
                    .single_call(
                        signer,
                        paths.get_func_name(0)?,
                        paths.get_path(0)?,
                        recipient,
                        amount_in,
                        amount_out_minimum,
                    )
                    .await?)
            }
            /* if more than one swap call, call multicall */
            /* saves GAS compared to calling each swap call because it only needs approval once */
            _ => {
                return Ok(self
                    .multi_call(signer, paths, recipient, amount_in, amount_out_minimum)
                    .await?)
            }
        }
    }

    pub async fn single_call(
        &self,
        signer: impl Key + Clone,
        func_name: String,
        path: DexPath,
        recipient: Address,
        amount_in: U256,
        amount_out_minimum: U256,
    ) -> Result<H256> {
        match PancakeSmartRouterV3Functions::from_str(&func_name)? {
            PancakeSmartRouterV3Functions::SwapExactTokensForTokens => {
                /* path is the same on V2 pools, regardless of exact in or out */
                /* path[0] is tokenIn, path[path.len()-1] is tokenOut */
                Ok(self
                    .swap_exact_tokens_for_tokens(
                        signer.clone(),
                        recipient,
                        amount_in,
                        amount_out_minimum,
                        match path {
                            DexPath::PancakeV2(path) => path,
                            _ => bail!("invalid path for v2"),
                        },
                    )
                    .await?)
            }
            PancakeSmartRouterV3Functions::SwapTokensForExactTokens => {
                /* path is the same on V2 pools, regardless of exact in or out */
                /* path[0] is tokenIn, path[path.len()-1] is tokenOut */
                Ok(self
                    .swap_exact_tokens_for_tokens(
                        signer.clone(),
                        recipient,
                        amount_in,
                        amount_out_minimum,
                        match path {
                            DexPath::PancakeV2(path) => path,
                            _ => bail!("invalid path for v2"),
                        },
                    )
                    .await?)
            }
            PancakeSmartRouterV3Functions::ExactInputSingle => {
                /* path is the same on V3 single hop calls */
                /* tokenIn, tokenOut, and fee are passed on every call */
                let v3_single_hop_path = match path {
                    DexPath::PancakeV3SingleHop(path) => path,
                    _ => bail!("invalid path for v3 single hop"),
                };
                Ok(self
                    .exact_input_single(
                        signer.clone(),
                        v3_single_hop_path.token_in,
                        v3_single_hop_path.token_out,
                        v3_single_hop_path.fee,
                        recipient,
                        amount_in,
                        amount_out_minimum,
                    )
                    .await?)
            }
            PancakeSmartRouterV3Functions::ExactOutputSingle => {
                /* path is the same on V3 single hop calls */
                /* tokenIn, tokenOut, and fee are passed on every call */
                let v3_single_hop_path = match path {
                    DexPath::PancakeV3SingleHop(path) => path,
                    _ => bail!("invalid path for v3 single hop"),
                };
                Ok(self
                    .exact_input_single(
                        signer.clone(),
                        v3_single_hop_path.token_in,
                        v3_single_hop_path.token_out,
                        v3_single_hop_path.fee,
                        recipient,
                        amount_in,
                        amount_out_minimum,
                    )
                    .await?)
            }
            PancakeSmartRouterV3Functions::ExactInput => {
                /* path is opposite on V3 multi hop calls */
                /* first address is tokenIn on exact in */
                /* first address is tokenOut on exact out */
                Ok(self
                    .exact_input(
                        signer.clone(),
                        MultiHopPath::from_bytes(&match path {
                            DexPath::PancakeV3MultiHop(path) => path,
                            _ => bail!("invalid path for v3 multi hop"),
                        })?,
                        recipient,
                        amount_in,
                        amount_out_minimum,
                    )
                    .await?)
            }
            PancakeSmartRouterV3Functions::ExactOutput => {
                /* invert the "exactOutput" call path to reuse it in the "exactInput" call */
                Ok(self
                    .exact_input(
                        signer.clone(),
                        MultiHopPath::invert(&MultiHopPath::from_bytes(&match path {
                            DexPath::PancakeV3MultiHop(path) => path,
                            _ => bail!("invalid path for v3 multi hop"),
                        })?),
                        recipient,
                        amount_in,
                        amount_out_minimum,
                    )
                    .await?)
            }
        }
    }

    pub async fn swap_exact_tokens_for_tokens(
        &self,
        signer: impl Key,
        recipient: Address,
        amount_in: U256,
        amount_out_min: U256,
        path: Vec<Address>,
    ) -> Result<H256> {
        let params = (amount_in, amount_out_min, path, recipient);
        let estimated_gas = self
            .contract
            .estimate_gas(
                PancakeSmartRouterV3Functions::SwapExactTokensForTokens.as_str(),
                params.clone(),
                signer.address(),
                Options::default(),
            )
            .await?;

        Ok(self
            .contract
            .signed_call(
                PancakeSmartRouterV3Functions::SwapExactTokensForTokens.as_str(),
                params,
                Options::with(|options| options.gas = Some(estimated_gas)),
                signer,
            )
            .await?)
    }

    pub async fn swap_tokens_for_exact_tokens(
        &self,
        signer: impl Key,
        recipient: Address,
        amount_out: U256,
        amount_in_max: U256,
        path: Vec<Address>,
    ) -> Result<H256> {
        let params = (amount_out, amount_in_max, path, recipient);
        let estimated_gas = self
            .contract
            .estimate_gas(
                PancakeSmartRouterV3Functions::SwapTokensForExactTokens.as_str(),
                params.clone(),
                signer.address(),
                Options::default(),
            )
            .await?;

        Ok(self
            .contract
            .signed_call(
                PancakeSmartRouterV3Functions::SwapTokensForExactTokens.as_str(),
                params,
                Options::with(|options| options.gas = Some(estimated_gas)),
                signer,
            )
            .await?)
    }

    pub async fn exact_input_single(
        &self,
        signer: impl Key,
        token_in: Address,
        token_out: Address,
        fee: U256,
        recipient: Address,
        amount_in: U256,
        amount_out_minimum: U256,
    ) -> Result<H256> {
        /* fee is a Solidity uint24 */
        let max_uint24: U256 = U256::from(2).pow(24.into()) - U256::from(1);
        if fee > max_uint24 {
            bail!("fee exceeds the maximum value for a uint24");
        }

        /* params is a Soldity struct, encode it into a Token::Tuple */
        let params = Token::Tuple(vec![
            Token::Address(token_in),
            Token::Address(token_out),
            Token::Uint(fee),
            Token::Address(recipient),
            Token::Uint(amount_in),
            Token::Uint(amount_out_minimum),
            Token::Uint(U256::from(0)),
        ]);
        let estimated_gas = self
            .contract
            .estimate_gas(
                PancakeSmartRouterV3Functions::ExactInputSingle.as_str(),
                params.clone(),
                signer.address(),
                Options::default(),
            )
            .await?;

        Ok(self
            .contract
            .signed_call(
                PancakeSmartRouterV3Functions::ExactInputSingle.as_str(),
                params,
                Options::with(|options| options.gas = Some(estimated_gas)),
                signer,
            )
            .await?)
    }

    pub async fn exact_output_single(
        &self,
        signer: impl Key,
        token_in: Address,
        token_out: Address,
        fee: U256,
        recipient: Address,
        amount_out: U256,
        amount_in_maximum: U256,
    ) -> Result<H256> {
        /* fee is a Solidity uint24 */
        let max_uint24: U256 = U256::from(2).pow(24.into()) - U256::from(1);
        if fee > max_uint24 {
            bail!("fee exceeds the maximum value for a uint24");
        }

        /* params is a Soldity struct, encode it into a Token::Tuple */
        let params = Token::Tuple(vec![
            Token::Address(token_in),
            Token::Address(token_out),
            Token::Uint(fee),
            Token::Address(recipient),
            Token::Uint(amount_out),
            Token::Uint(amount_in_maximum),
            Token::Uint(U256::from(0)),
        ]);
        let estimated_gas = self
            .contract
            .estimate_gas(
                PancakeSmartRouterV3Functions::ExactOutputSingle.as_str(),
                params.clone(),
                signer.address(),
                Options::default(),
            )
            .await?;

        Ok(self
            .contract
            .signed_call(
                PancakeSmartRouterV3Functions::ExactOutputSingle.as_str(),
                params,
                Options::with(|options| options.gas = Some(estimated_gas)),
                signer,
            )
            .await?)
    }

    pub async fn exact_input(
        &self,
        signer: impl Key,
        path: Vec<MultiHopPath>,
        recipient: Address,
        amount_in: U256,
        amount_out_minimum: U256,
    ) -> Result<H256> {
        /* params is a Soldity struct, encode it into a Token::Tuple */
        let params = Token::Tuple(vec![
            Token::Bytes(MultiHopPath::to_bytes(&path)?),
            Token::Address(recipient),
            Token::Uint(amount_in),
            Token::Uint(amount_out_minimum),
        ]);

        let estimated_gas = self
            .contract
            .estimate_gas(
                PancakeSmartRouterV3Functions::ExactInput.as_str(),
                params.clone(),
                signer.address(),
                Options::default(),
            )
            .await?;

        Ok(self
            .contract
            .signed_call(
                PancakeSmartRouterV3Functions::ExactInput.as_str(),
                params,
                Options::with(|options| options.gas = Some(estimated_gas)),
                signer,
            )
            .await?)
    }

    pub async fn exact_output(
        &self,
        signer: impl Key,
        path: Vec<MultiHopPath>,
        recipient: Address,
        amount_out: U256,
        amount_in_maximum: U256,
    ) -> Result<H256> {
        /* params is a Soldity struct, encode it into a Token::Tuple */
        let params = Token::Tuple(vec![
            Token::Bytes(MultiHopPath::to_bytes(&path)?),
            Token::Address(recipient),
            Token::Uint(amount_out),
            Token::Uint(amount_in_maximum),
        ]);
        let estimated_gas = self
            .contract
            .estimate_gas(
                PancakeSmartRouterV3Functions::ExactOutput.as_str(),
                params.clone(),
                signer.address(),
                Options::default(),
            )
            .await?;

        Ok(self
            .contract
            .signed_call(
                PancakeSmartRouterV3Functions::ExactOutput.as_str(),
                params,
                Options::with(|options| options.gas = Some(estimated_gas)),
                signer,
            )
            .await?)
    }

    pub async fn multi_call(
        &self,
        signer: impl Key,
        paths: PancakePairPathSet,
        recipient: Address,
        amount_in: U256,
        amount_out_minimum: U256,
    ) -> Result<H256> {
        let mut call_data: Vec<Vec<u8>> = Vec::new();
        let mut temp_recipient: Address;
        let mut temp_amount_in: U256;
        let mut temp_amount_out_minimum: U256;
        for i in 0..paths.len() {
            if i == 0 {
                /* first swap, set recipient of tokenOut as the contract itself */
                /* the flag (address 0x2) saves GAS compared to providing the real address of the contract */
                temp_recipient = self.refer_to_self_flag;
                /* set amount_in, which is the amount of the first tokenIn */
                temp_amount_in = amount_in;
                /* no limit on amount out, this limit is for the last tokenOut only */
                temp_amount_out_minimum = U256::from(0);
            } else if i == paths.len() - 1 {
                /* last swap, set recipient of tokenOut as the true recipient */
                temp_recipient = recipient;
                /* set amount_in to zero, will make the contract spend its own balance */
                temp_amount_in = U256::from(0);
                /* set limit to amount out for the last tokenOut */
                /* if after all swaps this minimum is not achieved, the transaction reverts */
                temp_amount_out_minimum = amount_out_minimum;
            } else {
                /* intermitent swap, set recipient of tokenOut as the contract itself */
                temp_recipient = self.refer_to_self_flag;
                /* set amount_in to zero, will make the contract spend its own balance */
                temp_amount_in = U256::from(0);
                /* no limit on amount out, this limit is for the last tokenOut only */
                temp_amount_out_minimum = U256::from(0);
            }
            match PancakeSmartRouterV3Functions::from_str(&paths.get_func_name(i)?)? {
                PancakeSmartRouterV3Functions::SwapExactTokensForTokens => {
                    call_data.push(self.setup_swap_exact_tokens_for_tokens(
                        temp_recipient,
                        temp_amount_in,
                        temp_amount_out_minimum,
                        match paths.get_path(i)? {
                            DexPath::PancakeV2(path) => path,
                            _ => bail!("invalid path for v2"),
                        },
                    )?)
                }
                PancakeSmartRouterV3Functions::SwapTokensForExactTokens => {
                    call_data.push(self.setup_swap_exact_tokens_for_tokens(
                        temp_recipient,
                        temp_amount_in,
                        temp_amount_out_minimum,
                        match paths.get_path(i)? {
                            DexPath::PancakeV2(path) => path,
                            _ => bail!("invalid path for v2"),
                        },
                    )?)
                }
                PancakeSmartRouterV3Functions::ExactInputSingle => {
                    let v3_single_hop_path = match paths.get_path(i)? {
                        DexPath::PancakeV3SingleHop(path) => path,
                        _ => bail!("invalid path for v3 single hop"),
                    };
                    call_data.push(self.setup_exact_input_single(
                        v3_single_hop_path.token_in,
                        v3_single_hop_path.token_out,
                        v3_single_hop_path.fee,
                        temp_recipient,
                        temp_amount_in,
                        temp_amount_out_minimum,
                    )?)
                }
                PancakeSmartRouterV3Functions::ExactOutputSingle => {
                    let v3_single_hop_path = match paths.get_path(i)? {
                        DexPath::PancakeV3SingleHop(path) => path,
                        _ => bail!("invalid path for v3 single hop"),
                    };
                    call_data.push(self.setup_exact_input_single(
                        v3_single_hop_path.token_in,
                        v3_single_hop_path.token_out,
                        v3_single_hop_path.fee,
                        temp_recipient,
                        temp_amount_in,
                        temp_amount_out_minimum,
                    )?)
                }
                PancakeSmartRouterV3Functions::ExactInput => {
                    call_data.push(self.setup_exact_input(
                        MultiHopPath::from_bytes(&match paths.get_path(i)? {
                            DexPath::PancakeV3MultiHop(path) => path,
                            _ => bail!("invalid path for v3 multi hop"),
                        })?,
                        temp_recipient,
                        temp_amount_in,
                        temp_amount_out_minimum,
                    )?)
                }
                PancakeSmartRouterV3Functions::ExactOutput => {
                    call_data.push(self.setup_exact_input(
                        MultiHopPath::invert(&MultiHopPath::from_bytes(
                            &match paths.get_path(i)? {
                                DexPath::PancakeV3MultiHop(path) => path,
                                _ => bail!("invalid path for v3 multi hop"),
                            },
                        )?),
                        temp_recipient,
                        temp_amount_in,
                        temp_amount_out_minimum,
                    )?)
                }
            }
        }

        let params = Token::Array(
            call_data
                .into_iter()
                .map(|data| Token::Bytes(data))
                .collect(),
        );

        let estimated_gas = self
            .contract
            .estimate_gas(
                "multicall",
                params.clone(),
                signer.address(),
                Options::default(),
            )
            .await?;

        Ok(self
            .contract
            .signed_call(
                "multicall",
                params,
                Options::with(|options| options.gas = Some(estimated_gas)),
                signer,
            )
            .await?)
    }

    fn setup_exact_input(
        &self,
        path: Vec<MultiHopPath>,
        recipient: Address,
        amount_in: U256,
        amount_out_minimum: U256,
    ) -> Result<Vec<u8>> {
        let params = Token::Tuple(vec![
            Token::Bytes(MultiHopPath::to_bytes(&path)?),
            Token::Address(recipient),
            Token::Uint(amount_in),
            Token::Uint(amount_out_minimum),
        ]);
        Ok(self
            .contract
            .abi()
            .function(PancakeSmartRouterV3Functions::ExactInput.as_str())?
            .encode_input(&vec![params])?)
    }

    fn setup_exact_input_single(
        &self,
        token_in: Address,
        token_out: Address,
        fee: U256,
        recipient: Address,
        amount_in: U256,
        amount_out_minimum: U256,
    ) -> Result<Vec<u8>> {
        let params = Token::Tuple(vec![
            Token::Address(token_in),
            Token::Address(token_out),
            Token::Uint(fee),
            Token::Address(recipient),
            Token::Uint(amount_in),
            Token::Uint(amount_out_minimum),
            Token::Uint(U256::from(0)),
        ]);
        Ok(self
            .contract
            .abi()
            .function(PancakeSmartRouterV3Functions::ExactInputSingle.as_str())?
            .encode_input(&vec![params])?)
    }

    fn setup_swap_exact_tokens_for_tokens(
        &self,
        recipient: Address,
        amount_in: U256,
        amount_out_min: U256,
        path: Vec<Address>,
    ) -> Result<Vec<u8>> {
        let params = vec![
            Token::Uint(amount_in),
            Token::Uint(amount_out_min),
            Token::Array(path.into_iter().map(|p| Token::Address(p)).collect()),
            Token::Address(recipient),
        ];
        Ok(self
            .contract
            .abi()
            .function(PancakeSmartRouterV3Functions::SwapExactTokensForTokens.as_str())?
            .encode_input(&params)?)
    }
}

enum PancakeSmartRouterV3Functions {
    SwapExactTokensForTokens,
    SwapTokensForExactTokens,
    ExactInputSingle,
    ExactInput,
    ExactOutputSingle,
    ExactOutput,
}

impl PancakeSmartRouterV3Functions {
    fn as_str(&self) -> &'static str {
        match self {
            Self::SwapExactTokensForTokens => "swapExactTokensForTokens",
            Self::SwapTokensForExactTokens => "swapTokensForExactTokens",
            Self::ExactInputSingle => "exactInputSingle",
            Self::ExactInput => "exactInput",
            Self::ExactOutputSingle => "exactOutputSingle",
            Self::ExactOutput => "exactOutput",
        }
    }

    fn from_str(function: &str) -> Result<Self> {
        match function {
            "swapExactTokensForTokens" => Ok(Self::SwapExactTokensForTokens),
            "swapTokensForExactTokens" => Ok(Self::SwapTokensForExactTokens),
            "exactInputSingle" => Ok(Self::ExactInputSingle),
            "exactInput" => Ok(Self::ExactInput),
            "exactOutputSingle" => Ok(Self::ExactOutputSingle),
            "exactOutput" => Ok(Self::ExactOutput),
            _ => bail!("invalid function name"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::time::Duration;
    use tracing::info;

    use super::*;
    use crate::contract_wrappers::wrapped_token::WrappedTokenContract;
    use crate::dex::DexAddresses;
    use crate::dex_tracker::build_pancake_swap;
    use crate::erc20::Erc20Token;
    use crate::signer::Secp256k1SecretKey;
    use crate::tx::{TransactionFetcher, TransactionReady};
    use crate::wait_for_confirmations_simple;
    use crate::{EthereumRpcConnectionPool, TxStatus};
    use crate::{StableCoin, StableCoinAddresses};
    use crate::{DEV_ACCOUNT_ADDRESS, DEV_ACCOUNT_PRIV_KEY};
    use gen::model::{EnumBlockChain, EnumDex};
    use lib::log::{setup_logs, LogLevel};
    use lib::utils::hex_decode;

    pub struct WorkingPairPaths {
        inner: HashMap<EnumBlockChain, Vec<PancakePairPathSet>>,
    }

    impl WorkingPairPaths {
        pub fn new() -> Result<Self> {
            let mut this: HashMap<EnumBlockChain, Vec<PancakePairPathSet>> = HashMap::new();

            this.insert(EnumBlockChain::BscTestnet, Vec::new());
            this.insert(EnumBlockChain::EthereumGoerli, Vec::new());

            this.get_mut(&EnumBlockChain::BscTestnet)
                .unwrap()
                /* tx: 0x272919df10865fbb8ea14df513772a853d2e1a2457f1b7dae186b1fb59630089 */
                /* amount_in: 1000 */
                .push(PancakePairPathSet::new(
                    /* token_in is wrapped Testnet BNB */
                    Address::from_str("0xae13d989dac2f0debff460ac112a837c89baa7cd")?,
                    /* token_out is Testnet BUSD */
                    Address::from_str("0xab1a4d4f1d656d2450692d237fdd6c7f9146e814")?,
                    /* path is one v2 function call */
                    vec![(
                        "swapExactTokensForTokens".to_string(),
                        DexPath::PancakeV2(vec![
                            Address::from_str("0xae13d989dac2f0debff460ac112a837c89baa7cd")?,
                            Address::from_str("0xab1a4d4f1d656d2450692d237fdd6c7f9146e814")?,
                        ]),
                    )],
                )?);

            /* tx: 0x99cd5746eb0aa3ce832e149f9c7b70b7db95c932d0534354c9f0230a56d5b8e6 */
            /* amount_in: 10000000000 */
            this.get_mut(&EnumBlockChain::EthereumGoerli)
                .unwrap()
                .push(PancakePairPathSet::new(
                    /* token_in is wrapped Goerli ETH */
                    Address::from_str("0xb4fbf271143f4fbf7b91a5ded31805e42b2208d6")?,
                    /* token_out is Goerli USDC */
                    Address::from_str("0x07865c6e87b9f70255377e024ace6630c1eaa37f")?,
                    /* path is one v2 function call */
                    vec![(
                        "swapExactTokensForTokens".to_string(),
                        DexPath::PancakeV2(vec![
                            Address::from_str("0xb4fbf271143f4fbf7b91a5ded31805e42b2208d6")?,
                            Address::from_str("0x07865c6e87b9f70255377e024ace6630c1eaa37f")?,
                        ]),
                    )],
                )?);

            Ok(Self { inner: this })
        }

        fn get_pair_paths_by_chain(
            &self,
            chain: EnumBlockChain,
        ) -> Result<Vec<PancakePairPathSet>> {
            match self.inner.get(&chain) {
                Some(paths) => Ok(paths.clone()),
                None => bail!("no paths for chain"),
            }
        }
    }

    const WBNB_TESTNET_ADDRESS: &str = "0xae13d989dac2f0debff460ac112a837c89baa7cd";
    const WETH_GOERLI_ADDRESS: &str = "0xb4fbf271143f4fbf7b91a5ded31805e42b2208d6";
    const BNB_TEST_SWAP_CONTRACT_ADDRESS: &str = "0x13f4EA83D0bd40E75C8222255bc855a974568Dd4";
    const BNB_TEST_SWAP_INPUT_DATA: &str = "0x5ae401dc00000000000000000000000000000000000000000000000000000000647c9d9c00000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000e4472b43f30000000000000000000000000000000000000000000000004563918244f4000000000000000000000000000000000000000000000000000044e00b38052cbb9a0000000000000000000000000000000000000000000000000000000000000080000000000000000000000000111013b7862ebc1b9726420aa0e8728de310ee63000000000000000000000000000000000000000000000000000000000000000200000000000000000000000055d398326f99059ff775485246999027b31979550000000000000000000000008ac76a51cc950d9822d68b83fe1ad97b32cd580d00000000000000000000000000000000000000000000000000000000";
    #[tokio::test]
    async fn test_copy_trade_testnet() -> Result<()> {
        let _ = setup_logs(LogLevel::Debug);
        let conn_pool = EthereumRpcConnectionPool::bsc_testnet();
        let conn = conn_pool.get_conn().await?;

        /* dev account must have native tokens */
        let key = Secp256k1SecretKey::from_str(DEV_ACCOUNT_PRIV_KEY)?;

        /* wbnb contract so we can wrap and approve tokens to the router */
        let wbnb = WrappedTokenContract::new(
            conn.clone().into_raw(),
            Address::from_str(WBNB_TESTNET_ADDRESS)?,
        )?;

        /* busd contract so we can check balances */
        let bsc_testnet_busd_address = StableCoinAddresses::default()
            .get_by_chain_and_token(EnumBlockChain::BscTestnet, StableCoin::Busd)
            .unwrap();
        let busd = Erc20Token::new(conn.clone().into_raw(), bsc_testnet_busd_address)?;

        let pancake_swap = PancakeSmartRouterV3Contract::new(
            conn.clone().into_raw(),
            DexAddresses::default()
                .get_by_chain_and_dex(EnumBlockChain::BscTestnet, EnumDex::PancakeSwap)
                .unwrap(),
        )?;

        info!("Wrapping BNB");
        /* wrap BNB so we can use it to trade tokens */
        /* assert transaction is successful */
        let wrap_tx_hash = wbnb.wrap(key.clone(), U256::from(1000)).await?;

        wait_for_confirmations_simple(
            &conn.clone().into_raw().eth(),
            wrap_tx_hash,
            Duration::from_secs(3),
            5,
        )
        .await?;

        let mut tx = TransactionFetcher::new(wrap_tx_hash);
        tx.update(&conn).await?;

        match tx.get_status() {
            TxStatus::Successful => {}
            TxStatus::Reverted => {
                bail!("wrap transaction reverted")
            }
            _ => {
                unreachable!()
            }
        }
        info!("Approving PancakeSwap to spend wrapped BNB");
        /* approve the pancake swap smart router for wrapped tokens, so it can trade them */
        /* assert transaction is successful */
        let approve_tx_hash = wbnb
            .approve(key.clone(), pancake_swap.address(), U256::from(1000))
            .await?;
        wait_for_confirmations_simple(
            &conn.clone().into_raw().eth(),
            approve_tx_hash,
            Duration::from_secs(3),
            5,
        )
        .await?;

        let mut tx = TransactionFetcher::new(approve_tx_hash);
        tx.update(&conn).await?;

        match tx.get_status() {
            TxStatus::Successful => {}
            TxStatus::Reverted => {
                bail!("approve transaction reverted")
            }
            _ => unreachable!(),
        }

        let balance_wbnb_before_copy_trade = wbnb.balance_of(key.address()).await?;
        let balance_busd_before_copy_trade = busd.balance_of(key.address()).await?;
        /* fetch previous trade from wbnb to busd */
        let working_pair_paths =
            WorkingPairPaths::new()?.get_pair_paths_by_chain(EnumBlockChain::BscTestnet)?;
        /* copy trade */
        /* assert transaction is successful */
        let mut copied_trade_tx: Option<TransactionReady> = None;
        let copy_trade_tx_hash = pancake_swap
            .copy_trade(
                key.clone(),
                working_pair_paths[0].clone(),
                U256::from(1000),
                U256::from(1),
            )
            .await?;
        wait_for_confirmations_simple(
            &conn.clone().into_raw().eth(),
            copy_trade_tx_hash,
            Duration::from_secs(3),
            5,
        )
        .await?;

        let mut tx = TransactionFetcher::new(copy_trade_tx_hash);
        tx.update(&conn).await?;

        match tx.get_status() {
            TxStatus::Successful => {
                copied_trade_tx = Some(tx.assume_ready()?);
            }
            TxStatus::Reverted | TxStatus::NotFound => {}
            _ => unreachable!(),
        }

        let balance_wbnb_after_copy_trade = wbnb.balance_of(key.address()).await?;
        let balance_busd_after_copy_trade = busd.balance_of(key.address()).await?;

        /* parse copied trade */
        let trade_tracker = build_pancake_swap()?;
        let copied_trade_tx = copied_trade_tx.context("no copied trade tx")?;
        let copied_trade =
            trade_tracker.parse_trade(&copied_trade_tx, EnumBlockChain::BscTestnet)?;

        /* check amounts and addresses */
        assert_eq!(copied_trade.amount_in, U256::from(1000));
        assert_eq!(copied_trade.caller, Address::from_str(DEV_ACCOUNT_ADDRESS)?);
        assert_eq!(
            copied_trade.token_in,
            Address::from_str(WBNB_TESTNET_ADDRESS)?
        );
        assert_eq!(copied_trade.token_out, bsc_testnet_busd_address);
        assert_eq!(
            balance_wbnb_before_copy_trade,
            balance_wbnb_after_copy_trade + U256::from(1000)
        );
        assert!(balance_busd_before_copy_trade < balance_busd_after_copy_trade);
        Ok(())
    }
    #[tokio::test]
    async fn test_parse_trade_from_raw_data() -> Result<()> {
        let _ = setup_logs(LogLevel::Debug);

        /* parse copied trade */
        let pancake = build_pancake_swap()?;
        let caller = DEV_ACCOUNT_ADDRESS.parse()?;
        let copied_trade = pancake.parse_trade2(
            1.into(),
            caller,
            BNB_TEST_SWAP_CONTRACT_ADDRESS.parse()?,
            &hex_decode(BNB_TEST_SWAP_INPUT_DATA.as_bytes())?,
            EnumBlockChain::BscMainnet,
        )?;

        let paths = copied_trade.get_pancake_pair_paths()?;
        assert_eq!(paths.len(), 1);
        info!("Paths: {:?}", paths);
        Ok(())
    }
}
