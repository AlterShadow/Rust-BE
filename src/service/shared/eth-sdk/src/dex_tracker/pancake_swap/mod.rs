pub mod v2;
pub mod v3;

use super::v2::{swap_exact_tokens_for_tokens, swap_tokens_for_exact_tokens};
use super::v3::{
    multi_hop::{exact_input, exact_output},
    single_hop::{exact_input_single, exact_output_single},
};
use crate::dex_tracker::v3::multi_hop::MultiHopPath;
use crate::erc20::build_erc_20;
use crate::evm::DexPath;
use crate::evm::{DexTrade, PancakeV3SingleHopPath};
use crate::{ContractCall, SerializableToken, TransactionReady};
use eyre::bail;
use eyre::*;
use gen::model::{EnumBlockChain, EnumDex, EnumDexVersion};
use std::io::Cursor;
use std::str::FromStr;
use web3::ethabi::Contract;
use web3::types::{Address, H160, H256, U256};

pub struct Swap {
    pub recipient: Address,
    pub token_in: Address,
    pub token_out: Address,
    pub amount_in: Option<U256>,
    pub amount_out: Option<U256>,
    pub amount_out_minimum: Option<U256>,
    pub amount_in_maximum: Option<U256>,
    pub path: DexPath,
}

#[derive(Clone, Debug)]
pub struct PancakeSwap {
    smart_router: Contract,
    transfer_event_signature: H256,
    refer_to_self_flag: H160,
}

impl PancakeSwap {
    /* Parses Calls to the PancakeSwap V3 Smart Router into a Trade */
    /* https://etherscan.io/address/0x13f4EA83D0bd40E75C8222255bc855a974568Dd4#code */

    pub fn new(smart_router: Contract, transfer_event_signature: H256) -> Self {
        Self {
            smart_router,
            transfer_event_signature,
            refer_to_self_flag: H160::from_str("0x0000000000000000000000000000000000000002")
                .unwrap(),
        }
    }
    pub fn parse_trade2(
        &self,
        _value: U256,
        caller: Address,
        contract: Address,
        input_data: &[u8],
        chain: EnumBlockChain,
    ) -> Result<DexTrade> {
        /* if tx is successful, all of the following should be Some */

        let function_called = ContractCall::from_inputs(&self.smart_router, input_data)?;
        let function_calls: Vec<ContractCall>;
        if function_called.get_name() == "multicall" {
            /* swaps go through the "multicall" smart router function if: */
            /* the caller pays or receives native tokens, so the swap includes other calls like unwrapETH or refundETH */
            /* the swap requires calls to both v2 and v3 pools to be completed */
            function_calls = self.get_multicall_funcs_and_params(&function_called)?;
        } else {
            /* swaps call a swap function directly instead of multicall if it's token to token and a single pool version is enough */
            function_calls = vec![function_called];
        }

        let mut swap_infos: Vec<(Swap, EnumDexVersion, ContractCall)> = Vec::new();
        for call in function_calls {
            let method_name = call.get_name();
            if let Some(method) = self.get_method_by_name(&method_name) {
                swap_infos.push(match method {
                    /* V2 */
                    PancakeSwapMethod::SwapExactTokensForTokens => {
                        let swap_exact_tokens_for_tokens_params =
                            swap_exact_tokens_for_tokens(&call)?;
                        let swap = Swap {
                            recipient: swap_exact_tokens_for_tokens_params.to,
                            token_in: swap_exact_tokens_for_tokens_params.path[0],
                            token_out: swap_exact_tokens_for_tokens_params.path
                                [swap_exact_tokens_for_tokens_params.path.len() - 1],
                            amount_in: Some(swap_exact_tokens_for_tokens_params.amount_in),
                            amount_out: None,
                            amount_out_minimum: Some(
                                swap_exact_tokens_for_tokens_params.amount_out_min,
                            ),
                            amount_in_maximum: None,
                            path: DexPath::PancakeV2(swap_exact_tokens_for_tokens_params.path),
                        };
                        (swap, EnumDexVersion::V2, call)
                    }
                    PancakeSwapMethod::SwapTokensForExactTokens => {
                        let swap_tokens_for_exact_tokens_params =
                            swap_tokens_for_exact_tokens(&call)?;
                        let swap = Swap {
                            recipient: swap_tokens_for_exact_tokens_params.to,
                            token_in: swap_tokens_for_exact_tokens_params.path[0],
                            token_out: swap_tokens_for_exact_tokens_params.path
                                [swap_tokens_for_exact_tokens_params.path.len() - 1],
                            amount_in: None,
                            amount_out: Some(swap_tokens_for_exact_tokens_params.amount_out),
                            amount_out_minimum: None,
                            amount_in_maximum: Some(
                                swap_tokens_for_exact_tokens_params.amount_in_max,
                            ),
                            path: DexPath::PancakeV2(swap_tokens_for_exact_tokens_params.path),
                        };

                        (swap, EnumDexVersion::V2, call)
                    }
                    /* V3 */
                    PancakeSwapMethod::ExactInputSingle => {
                        let exact_input_single_params = exact_input_single(&call)?;
                        // FIXME: for V3, exact_input_single_params.token_in is a Smart Router V3, not the token itself
                        let swap = Swap {
                            recipient: exact_input_single_params.recipient,
                            token_in: exact_input_single_params.token_in,
                            token_out: exact_input_single_params.token_out,
                            amount_in: Some(exact_input_single_params.amount_in),
                            amount_out: None,
                            amount_out_minimum: Some(exact_input_single_params.amount_out_minimum),
                            amount_in_maximum: None,
                            path: DexPath::PancakeV3SingleHop(PancakeV3SingleHopPath {
                                token_in: exact_input_single_params.token_in,
                                token_out: exact_input_single_params.token_out,
                                fee: exact_input_single_params.fee,
                            }),
                        };
                        (swap, EnumDexVersion::V3, call)
                    }
                    PancakeSwapMethod::ExactOutputSingle => {
                        let exact_output_single_params = exact_output_single(&call)?;
                        let swap = Swap {
                            recipient: exact_output_single_params.recipient,
                            token_in: exact_output_single_params.token_in,
                            token_out: exact_output_single_params.token_out,
                            amount_in: None,
                            amount_out: Some(exact_output_single_params.amount_out),
                            amount_out_minimum: None,
                            amount_in_maximum: Some(exact_output_single_params.amount_in_maximum),
                            path: DexPath::PancakeV3SingleHop(PancakeV3SingleHopPath {
                                token_in: exact_output_single_params.token_in,
                                token_out: exact_output_single_params.token_out,
                                fee: exact_output_single_params.fee,
                            }),
                        };
                        (swap, EnumDexVersion::V3, call)
                    }
                    PancakeSwapMethod::ExactInput => {
                        let exact_input_params = exact_input(&call)?;
                        let full_path = MultiHopPath::from_bytes(&exact_input_params.path)?;
                        let swap = Swap {
                            recipient: exact_input_params.recipient,
                            token_in: full_path[0].first_token,
                            token_out: full_path[full_path.len() - 1].second_token,
                            amount_in: Some(exact_input_params.amount_in),
                            amount_out: None,
                            amount_out_minimum: Some(exact_input_params.amount_out_minimum),
                            amount_in_maximum: None,
                            path: DexPath::PancakeV3MultiHop(exact_input_params.path.to_vec()),
                        };
                        (swap, EnumDexVersion::V3, call)
                    }
                    PancakeSwapMethod::ExactOutput => {
                        let exact_output_params = exact_output(&call)?;
                        let full_path = MultiHopPath::from_bytes(&exact_output_params.path)?;
                        let swap = Swap {
                            recipient: exact_output_params.recipient,
                            token_in: full_path[full_path.len() - 1].second_token,
                            token_out: full_path[0].first_token,
                            amount_in: None,
                            amount_out: Some(exact_output_params.amount_out),
                            amount_out_minimum: None,
                            amount_in_maximum: Some(exact_output_params.amount_in_maximum),
                            path: DexPath::PancakeV3MultiHop(exact_output_params.path.to_vec()),
                        };
                        (swap, EnumDexVersion::V3, call)
                    }
                });
            }
        }
        ensure!(swap_infos.len() > 0, "no suitable method found");

        let mut paths: Vec<DexPath> = Vec::new();
        let mut versions: Vec<EnumDexVersion> = Vec::new();
        let mut calls: Vec<ContractCall> = Vec::new();
        for (swap, version, call) in &swap_infos {
            paths.push(swap.path.clone());
            versions.push(*version);
            calls.push(call.clone());
        }
        Ok(DexTrade {
            chain,
            contract,
            dex: EnumDex::PancakeSwap,
            token_in: swap_infos[0].0.token_in,
            token_out: swap_infos[swap_infos.len() - 1].0.token_out,
            caller,
            amount_in: 0.into(),
            amount_out: 0.into(),
            swap_calls: calls,
            paths,
            dex_versions: versions,
        })
    }

    pub fn parse_trade(&self, tx: &TransactionReady, chain: EnumBlockChain) -> Result<DexTrade> {
        /* if tx is successful, all of the following should be Some */
        let value = tx.get_value();

        let caller = tx.get_from().context("Failed to get caller")?;

        let contract = tx.get_to().context("Failed to get contract")?;
        let input_data = tx.get_input_data();
        /* if tx is successful, all of the following should be Some */

        let function_called = ContractCall::from_inputs(&self.smart_router, input_data)?;
        let function_calls: Vec<ContractCall>;
        if function_called.get_name() == "multicall" {
            /* swaps go through the "multicall" smart router function if: */
            /* the caller pays or receives native tokens, so the swap includes other calls like unwrapETH or refundETH */
            /* the swap requires calls to both v2 and v3 pools to be completed */
            function_calls = self.get_multicall_funcs_and_params(&function_called)?;
        } else {
            /* swaps call a swap function directly instead of multicall if it's token to token and a single pool version is enough */
            function_calls = vec![function_called];
        }

        let mut swap_infos: Vec<(Swap, EnumDexVersion, ContractCall)> = Vec::new();
        for call in function_calls {
            let method_name = call.get_name();
            if let Some(method) = self.get_method_by_name(&method_name) {
                swap_infos.push(match method {
                    /* V2 */
                    PancakeSwapMethod::SwapExactTokensForTokens => {
                        let swap_exact_tokens_for_tokens_params =
                            swap_exact_tokens_for_tokens(&call)?;
                        let swap = Swap {
                            recipient: swap_exact_tokens_for_tokens_params.to,
                            token_in: swap_exact_tokens_for_tokens_params.path[0],
                            token_out: swap_exact_tokens_for_tokens_params.path
                                [swap_exact_tokens_for_tokens_params.path.len() - 1],
                            amount_in: Some(swap_exact_tokens_for_tokens_params.amount_in),
                            amount_out: None,
                            amount_out_minimum: Some(
                                swap_exact_tokens_for_tokens_params.amount_out_min,
                            ),
                            amount_in_maximum: None,
                            path: DexPath::PancakeV2(swap_exact_tokens_for_tokens_params.path),
                        };
                        (swap, EnumDexVersion::V2, call)
                    }
                    PancakeSwapMethod::SwapTokensForExactTokens => {
                        let swap_tokens_for_exact_tokens_params =
                            swap_tokens_for_exact_tokens(&call)?;
                        let swap = Swap {
                            recipient: swap_tokens_for_exact_tokens_params.to,
                            token_in: swap_tokens_for_exact_tokens_params.path[0],
                            token_out: swap_tokens_for_exact_tokens_params.path
                                [swap_tokens_for_exact_tokens_params.path.len() - 1],
                            amount_in: None,
                            amount_out: Some(swap_tokens_for_exact_tokens_params.amount_out),
                            amount_out_minimum: None,
                            amount_in_maximum: Some(
                                swap_tokens_for_exact_tokens_params.amount_in_max,
                            ),
                            path: DexPath::PancakeV2(swap_tokens_for_exact_tokens_params.path),
                        };

                        (swap, EnumDexVersion::V2, call)
                    }
                    /* V3 */
                    PancakeSwapMethod::ExactInputSingle => {
                        let exact_input_single_params = exact_input_single(&call)?;
                        let swap = Swap {
                            recipient: exact_input_single_params.recipient,
                            token_in: exact_input_single_params.token_in,
                            token_out: exact_input_single_params.token_out,
                            amount_in: Some(exact_input_single_params.amount_in),
                            amount_out: None,
                            amount_out_minimum: Some(exact_input_single_params.amount_out_minimum),
                            amount_in_maximum: None,
                            path: DexPath::PancakeV3SingleHop(PancakeV3SingleHopPath {
                                token_in: exact_input_single_params.token_in,
                                token_out: exact_input_single_params.token_out,
                                fee: exact_input_single_params.fee,
                            }),
                        };
                        (swap, EnumDexVersion::V3, call)
                    }
                    PancakeSwapMethod::ExactOutputSingle => {
                        let exact_output_single_params = exact_output_single(&call)?;
                        let swap = Swap {
                            recipient: exact_output_single_params.recipient,
                            token_in: exact_output_single_params.token_in,
                            token_out: exact_output_single_params.token_out,
                            amount_in: None,
                            amount_out: Some(exact_output_single_params.amount_out),
                            amount_out_minimum: None,
                            amount_in_maximum: Some(exact_output_single_params.amount_in_maximum),
                            path: DexPath::PancakeV3SingleHop(PancakeV3SingleHopPath {
                                token_in: exact_output_single_params.token_in,
                                token_out: exact_output_single_params.token_out,
                                fee: exact_output_single_params.fee,
                            }),
                        };
                        (swap, EnumDexVersion::V3, call)
                    }
                    PancakeSwapMethod::ExactInput => {
                        let exact_input_params = exact_input(&call)?;
                        let full_path = MultiHopPath::from_bytes(&exact_input_params.path)?;
                        let swap = Swap {
                            recipient: exact_input_params.recipient,
                            token_in: full_path[0].first_token,
                            token_out: full_path[full_path.len() - 1].second_token,
                            amount_in: Some(exact_input_params.amount_in),
                            amount_out: None,
                            amount_out_minimum: Some(exact_input_params.amount_out_minimum),
                            amount_in_maximum: None,
                            path: DexPath::PancakeV3MultiHop(exact_input_params.path.to_vec()),
                        };
                        (swap, EnumDexVersion::V3, call)
                    }
                    PancakeSwapMethod::ExactOutput => {
                        let exact_output_params = exact_output(&call)?;
                        let full_path = MultiHopPath::from_bytes(&exact_output_params.path)?;
                        let swap = Swap {
                            recipient: exact_output_params.recipient,
                            token_in: full_path[full_path.len() - 1].second_token,
                            token_out: full_path[0].first_token,
                            amount_in: None,
                            amount_out: Some(exact_output_params.amount_out),
                            amount_out_minimum: None,
                            amount_in_maximum: Some(exact_output_params.amount_in_maximum),
                            path: DexPath::PancakeV3MultiHop(exact_output_params.path.to_vec()),
                        };
                        (swap, EnumDexVersion::V3, call)
                    }
                });
            }
        }
        ensure!(swap_infos.len() > 0, "no suitable method found");

        let mut paths: Vec<DexPath> = Vec::new();
        let mut versions: Vec<EnumDexVersion> = Vec::new();
        let mut calls: Vec<ContractCall> = Vec::new();
        for (swap, version, call) in &mut swap_infos {
            paths.push(swap.path.clone());
            versions.push(*version);
            calls.push(call.clone());
            if swap.amount_out.is_none() {
                /* "exact in" type swap, find amount out */
                if swap.recipient == self.refer_to_self_flag {
                    /* if the recipient is the router, it's either: */
                    /* 1- one of n swaps, and the intermitent amount out goes to the router, so that it can execute next swap */
                    /* 2- not the final call, which means there is a call to "unwrap" and the caller gets paid in native tokens */
                    /* we know the call is to "unwrap" and not "refund" because the amount in was exact */
                    /* in both cases the swap's amount_out is in the transfer of token_out to router */
                    /* in case "1" it's useful to find out the amount out of the individual swap */
                    /* in case "2" it's useful to know the amount of unwrapped native tokens, and use the wrapped token as token_out */
                    /* differentiation between native and non-native is meaningless since only the wrapped version supports trading */
                    let amount_out = tx
                        .amount_of_token_received(
                            swap.token_out,
                            contract,
                            self.transfer_event_signature,
                        )
                        .context("failed to get amount_out")?;
                    swap.amount_out = Some(amount_out);
                } else {
                    /* if the recipient is the caller, it's always the final call */
                    /* since we are only looping through swap calls, the final call is a swap */
                    /* it means that the caller gets paid in non-native tokens */
                    /* swap's amount_out is the transfer of token_out to recipient */
                    let amount_out = tx
                        .amount_of_token_received(
                            swap.token_out,
                            swap.recipient,
                            self.transfer_event_signature,
                        )
                        .context("failed to get amount_out")?;
                    swap.amount_out = Some(amount_out);
                }
            } else {
                /* "exact out" type swap, find amount in */
                /* "exact out" swaps always include a single swap call */
                /* only "exact in" swaps can be chained since the router only spends its balance when "amount_in" is zero */
                /* this means we can refer to "tx.value" with confidence to infer if the sender of token_in is the caller or the router */
                /* if this was part of a chain of swap calls, this would not be possible */
                if value != 0.into() {
                    /* if the call has value, the caller paid in native tokens */
                    /* the router has to wrap native tokens before swapping */
                    /* swap's amount_in is in the transfer of token_in from router */
                    /* TODO: fix possible bug where caller sends value without a purpose */
                    let amount_in = tx
                        .amount_of_token_sent(
                            swap.token_in,
                            contract,
                            self.transfer_event_signature,
                        )
                        .context("failed to get amount_in")?;

                    swap.amount_in = Some(amount_in);
                } else {
                    /* if the call has no value, the caller paid in non-native tokens */
                    /* swap's amount_in is in the transfer of token_in from caller */
                    let amount_in = tx
                        .amount_of_token_sent(swap.token_in, caller, self.transfer_event_signature)
                        .context("failed to get amount_in")?;
                    swap.amount_in = Some(amount_in);
                }
            }
        }

        Ok(DexTrade {
            chain,
            contract,
            dex: EnumDex::PancakeSwap,
            token_in: swap_infos[0].0.token_in,
            token_out: swap_infos[swap_infos.len() - 1].0.token_out,
            caller,
            amount_in: swap_infos[0].0.amount_in.unwrap(),
            amount_out: swap_infos[swap_infos.len() - 1].0.amount_out.unwrap(),
            swap_calls: calls,
            paths,
            dex_versions: versions,
        })
    }

    fn get_multicall_funcs_and_params(
        &self,
        multicall: &ContractCall,
    ) -> Result<Vec<ContractCall>> {
        /*
                        function multicall(
                                bytes[] calldata data
                        ) public payable override returns (bytes[] memory results);
        */
        let mut actual_function_calls: Vec<ContractCall> = Vec::new();
        /* the single parameter from "multicall" is ambiguously called "data" */
        if let Ok(param) = multicall.get_param("data") {
            /* data is an unsized array of byte arrays */
            let value_array = match param.get_value() {
                SerializableToken::Array(value) => value,
                _ => {
                    return Err(eyre!("data is not an array"));
                }
            };

            for token in value_array {
                /* each byte array is a nested function call */
                let input_data = match token.into_bytes() {
                    Ok(input_data) => input_data,
                    Err(_) => {
                        return Err(eyre!("failed to get input data"));
                    }
                };
                let function_call = ContractCall::from_inputs(&self.smart_router, &input_data)?;
                actual_function_calls.push(function_call);
            }
        }

        Ok(actual_function_calls)
    }

    fn get_method_by_name(&self, name: &str) -> Option<PancakeSwapMethod> {
        match name {
            "swapExactTokensForTokens" => Some(PancakeSwapMethod::SwapExactTokensForTokens),
            "swapTokensForExactTokens" => Some(PancakeSwapMethod::SwapTokensForExactTokens),
            "exactInputSingle" => Some(PancakeSwapMethod::ExactInputSingle),
            "exactInput" => Some(PancakeSwapMethod::ExactInput),
            "exactOutputSingle" => Some(PancakeSwapMethod::ExactOutputSingle),
            "exactOutput" => Some(PancakeSwapMethod::ExactOutput),
            _ => None,
        }
    }
}

enum PancakeSwapMethod {
    SwapExactTokensForTokens,
    SwapTokensForExactTokens,
    ExactInputSingle,
    ExactInput,
    ExactOutputSingle,
    ExactOutput,
}

const SMART_ROUTER_ABI_JSON: &str =
    include_str!("../../../../../../../abi/pancake_swap/smart_router_v3.json");

pub fn build_pancake_swap() -> Result<PancakeSwap> {
    let cursor = Cursor::new(SMART_ROUTER_ABI_JSON);
    let pancake_smart_router = Contract::load(cursor).context("failed to read contract ABI")?;
    let erc20 = build_erc_20()?;
    let transfer_event_signature = erc20
        .event("Transfer")
        .context("Failed to get Transfer event signature")?
        .signature();
    let pancake = PancakeSwap::new(pancake_smart_router, transfer_event_signature);
    Ok(pancake)
}

#[derive(Debug, Clone)]
pub struct PancakePairPathSet {
    token_in: Address,
    token_out: Address,
    func_names_and_paths: Vec<(String, DexPath)>,
}

impl PancakePairPathSet {
    pub fn new(
        token_in: Address,
        token_out: Address,
        func_names_and_paths: Vec<(String, DexPath)>,
    ) -> Result<Self> {
        if func_names_and_paths.len() == 0 {
            bail!("empty names and paths");
        }
        Ok(Self {
            token_in,
            token_out,
            func_names_and_paths,
        })
    }

    pub fn get_token_in(&self) -> Address {
        self.token_in
    }

    pub fn get_token_out(&self) -> Address {
        self.token_out
    }

    pub fn len(&self) -> usize {
        self.func_names_and_paths.len()
    }

    pub fn get_func_name(&self, idx: usize) -> Result<String> {
        if idx >= self.len() {
            bail!("index out of bounds");
        }
        Ok(self.func_names_and_paths[idx].0.clone())
    }

    pub fn get_path(&self, idx: usize) -> Result<DexPath> {
        if idx >= self.len() {
            bail!("index out of bounds");
        }
        Ok(self.func_names_and_paths[idx].1.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{EthereumRpcConnectionPool, TransactionFetcher};
    use gen::model::EnumBlockChain;
    use lib::log::{setup_logs, LogLevel};
    use tracing::info;

    #[tokio::test]
    async fn test_pancakeswap() -> Result<()> {
        let _ = setup_logs(LogLevel::Info);

        let pancake = build_pancake_swap()?;
        let conn_pool = EthereumRpcConnectionPool::mainnet();
        let conn = conn_pool.get_conn().await?;
        let tx = TransactionFetcher::new_and_assume_ready(
            "0x750d90bf90ad0fe7d035fbbab41334f6bb10bf7e71246d430cb23ed35d1df7c2".parse()?,
            &conn,
        )
        .await?;

        let trade = pancake.parse_trade(&tx, EnumBlockChain::EthereumMainnet)?;
        info!("trade: {:?}", trade);
        Ok(())
    }
}
