use std::io::Cursor;

use ethabi::{Contract, Token};
use web3::types::{H160, H256, U256};

use super::trading_pair::{Chain, Dex, DexVersion, Swap, TradingPair};

use super::super::rpc_provider::connection::Conn;
use super::calldata::{CallParameter, ContractCall};
use super::tx::{Tx, TxStatus};

const SMART_ROUTER_PATH: &str = "abi/pancake_swap/smart_router_v3.json";
const ERC20_PATH: &str = "abi/generic/erc20.json";
const WETH_PATH: &str = "abi/weth.json";

#[derive(Clone, Debug)]
pub struct PancakeSwap {
    smart_router: Contract,
    weth: Contract,
    erc20: Contract,
    transfer_event_signature: H256,
}

impl PancakeSwap {
    /* Parses Calls to the PancakeSwap V3 Smart Router into a TradingPair */
    /* https://etherscan.io/address/0x13f4EA83D0bd40E75C8222255bc855a974568Dd4#code */

    pub async fn new() -> Self {
        let smart_router = ethabi::Contract::load(Cursor::new(
            std::fs::read(SMART_ROUTER_PATH).expect("Failed to read contract ABI"),
        ))
        .expect("Failed to parse contract ABI");
        let erc20 = ethabi::Contract::load(Cursor::new(
            std::fs::read(ERC20_PATH).expect("Failed to read contract ABI"),
        ))
        .expect("Failed to parse contract ABI");
        let weth = ethabi::Contract::load(Cursor::new(
            std::fs::read(WETH_PATH).expect("Failed to read contract ABI"),
        ))
        .expect("Failed to parse contract ABI");

        let transfer_event_signature = convert_h256_ethabi_to_web3(
            erc20
                .event("Transfer")
                .expect("Failed to get Transfer event signature")
                .signature(),
        );

        Self {
            smart_router: smart_router,
            weth: weth,
            erc20: erc20,
            transfer_event_signature: transfer_event_signature,
        }
    }

    pub fn get_swap(&self, tx: Tx) -> Option<Swap> {
        match tx.get_status() {
            TxStatus::Successful => (),
            /* TODO: handle pending transaction */
            TxStatus::Pending => return None,
            _ => return None,
        }

        /* if tx is successful, all of the following should be Some */
        let value = match tx.get_value() {
            Some(value) => value,
            None => return None,
        };

        /* all swaps go through the "multicall" smart router function */
        let function_calls = match self.get_multicall_funcs_and_params(&tx) {
            Some(functions_and_params) => functions_and_params,
            None => return None,
        };

        for call in function_calls {
            let method_name = call.get_name();
            if let Some(method) = self.get_method_by_name(&method_name) {
                match method {
                    /* V2 */
                    PancakeSwapMethod::SwapExactTokensForTokens => {
                        return self.swap_exact_tokens_for_tokens(&tx, &call)
                    }
                    PancakeSwapMethod::SwapTokensForExactTokens => {
                        return self.swap_tokens_for_exact_tokens(&tx, &call)
                    }
                    /* V3 */
                    PancakeSwapMethod::ExactInputSingle => return None,
                    PancakeSwapMethod::ExactOutputSingle => return None,
                    PancakeSwapMethod::ExactInput => return None,
                    PancakeSwapMethod::ExactOutput => return None,
                }
            }
        }

        None
    }

    fn get_multicall_funcs_and_params(&self, tx: &Tx) -> Option<Vec<ContractCall>> {
        /*
                        function multicall(
                                bytes[] calldata data
                        ) public payable override returns (bytes[] memory results);
        */
        let multicall_input_data = match tx.get_input_data() {
            Some(input_data) => input_data,
            None => return None,
        };

        let multicall = match ContractCall::from_inputs(&self.smart_router, &multicall_input_data) {
            Some(multicall) => multicall,
            None => return None,
        };

        let mut actual_function_calls: Vec<ContractCall> = Vec::new();
        /* the single parameter from "multicall" is ambiguously called "data" */
        if let Some(param) = multicall.get_param("data") {
            /* data is an unsized array of byte arrays */
            let value_array = match param.get_value() {
                Token::Array(value) => value,
                _ => return None,
            };

            for token in value_array {
                /* each byte array is a nested function call */
                let input_data = token.into_bytes()?;
                let function_call = ContractCall::from_inputs(&self.smart_router, &input_data)?;
                actual_function_calls.push(function_call);
            }
        }

        Some(actual_function_calls)
    }

    fn swap_exact_tokens_for_tokens(&self, tx: &Tx, call: &ContractCall) -> Option<Swap> {
        /*
            function swapExactTokensForTokens(
                                uint256 amountIn,
                                uint256 amountOutMin,
                                address[] calldata path,
                                address to
                ) external payable returns (uint256 amountOut);
        */
        let amount_in = match call.get_param("amountIn") {
            Some(param) => match param.get_value() {
                Token::Uint(value) => convert_u256_ethabi_to_web3(value),
                _ => return None,
            },
            None => return None,
        };

        let amount_out_min = match call.get_param("amountOutMin") {
            Some(param) => match param.get_value() {
                Token::Uint(value) => convert_u256_ethabi_to_web3(value),
                _ => return None,
            },
            None => return None,
        };

        let path = match call.get_param("path") {
            Some(param) => match param.get_value() {
                Token::Array(value) => value,
                _ => return None,
            },
            None => return None,
        };

        let token_in = match path[0].clone() {
            Token::Address(value) => convert_h160_ethabi_to_web3(value),
            _ => return None,
        };

        let token_out = match path[1].clone() {
            Token::Address(value) => convert_h160_ethabi_to_web3(value),
            _ => return None,
        };

        let recipient = match call.get_param("to") {
            Some(param) => match param.get_value() {
                Token::Address(value) => convert_h160_ethabi_to_web3(value),
                _ => return None,
            },
            None => return None,
        };

        let amount_out = match tx.amount_of_token_received(
            token_out,
            recipient,
            self.transfer_event_signature,
        ) {
            Some(amount) => amount,
            None => return None,
        };

        Some(Swap::new(
            recipient,
            Chain::Ethereum,
            tx.get_to().unwrap(),
            Dex::PancakeSwap,
            DexVersion::V2,
            token_in,
            token_out,
            amount_in,
            amount_out,
            None,
        ))
    }

    fn swap_tokens_for_exact_tokens(&self, tx: &Tx, call: &ContractCall) -> Option<Swap> {
        /*
            function swapTokensForExactTokens(
                        uint256 amountOut,
                        uint256 amountInMax,
                        address[] calldata path,
                        address to
            ) external payable override nonReentrant returns (uint256 amountIn)
        */

        let amount_out = match call.get_param("amountOut") {
            Some(param) => match param.get_value() {
                Token::Uint(value) => convert_u256_ethabi_to_web3(value),
                _ => return None,
            },
            None => return None,
        };

        let amount_in_max = match call.get_param("amountInMax") {
            Some(param) => match param.get_value() {
                Token::Uint(value) => convert_u256_ethabi_to_web3(value),
                _ => return None,
            },
            None => return None,
        };

        let path = match call.get_param("path") {
            Some(param) => match param.get_value() {
                Token::Array(value) => value,
                _ => return None,
            },
            None => return None,
        };

        let token_in = match path[0].clone() {
            Token::Address(value) => convert_h160_ethabi_to_web3(value),
            _ => return None,
        };

        let token_out = match path[1].clone() {
            Token::Address(value) => convert_h160_ethabi_to_web3(value),
            _ => return None,
        };

        let recipient = match call.get_param("to") {
            Some(param) => match param.get_value() {
                Token::Address(value) => convert_h160_ethabi_to_web3(value),
                _ => return None,
            },
            None => return None,
        };

        let amount_in =
            match tx.amount_of_token_sent(token_in, recipient, self.transfer_event_signature) {
                Some(amount) => amount,
                None => return None,
            };

        Some(Swap::new(
            recipient,
            Chain::Ethereum,
            tx.get_to().unwrap(),
            Dex::PancakeSwap,
            DexVersion::V2,
            token_in,
            token_out,
            amount_in,
            amount_out,
            None,
        ))
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

fn convert_h160_ethabi_to_web3(ethabi_h160: ethabi::ethereum_types::H160) -> H160 {
    H160::from_slice(&ethabi_h160.0)
}

fn convert_h256_ethabi_to_web3(ethabi_h256: ethabi::ethereum_types::H256) -> H256 {
    H256::from_slice(&ethabi_h256.0)
}

fn convert_u256_ethabi_to_web3(ethabi_u256: ethabi::ethereum_types::U256) -> U256 {
    let mut buffer = [0u8; 32];
    ethabi_u256.to_big_endian(&mut buffer);
    U256::from_big_endian(&buffer)
}

fn print_token(token: Token) {
    match token {
        Token::Address(_) => println!("Address"),
        Token::FixedBytes(_) => println!("FixedBytes"),
        Token::Bytes(_) => println!("Bytes"),
        Token::Int(_) => println!("Int"),
        Token::Uint(_) => println!("Uint"),
        Token::Bool(_) => println!("Bool"),
        Token::String(_) => println!("String"),
        Token::FixedArray(_) => println!("FixedArray"),
        Token::Array(_) => println!("Array"),
        Token::Tuple(_) => println!("Tuple"),
    }
}
