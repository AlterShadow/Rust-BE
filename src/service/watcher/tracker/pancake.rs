use std::io::Cursor;
use std::str::FromStr;

use ethabi::{Contract, Token};
use web3::ethabi::token;
use web3::types::{H160, H256, U256};

use super::trade::{Chain, Dex, DexVersion, Path, Trade};

use super::super::rpc_provider::connection::Connection;
use super::calldata::{CallParameter, ContractCall};
use super::tx::{Tx, TxStatus};

const SMART_ROUTER_PATH: &str = "abi/pancake_swap/smart_router_v3.json";
const ERC20_PATH: &str = "abi/generic/erc20.json";
const WETH_PATH: &str = "abi/weth.json";

pub struct Swap {
    recipient: H160,
    token_in: H160,
    token_out: H160,
    amount_in: Option<U256>,
    amount_out: Option<U256>,
    amount_in_maximum: Option<U256>,
    amount_out_minimum: Option<U256>,
    path: Path,
}

#[derive(Clone, Debug)]
pub struct PancakeSwap {
    smart_router: Contract,
    transfer_event_signature: H256,
}

impl PancakeSwap {
    /* Parses Calls to the PancakeSwap V3 Smart Router into a Trade */
    /* https://etherscan.io/address/0x13f4EA83D0bd40E75C8222255bc855a974568Dd4#code */

    pub fn new(smart_router: Contract, transfer_event_signature: H256) -> Self {
        Self {
            smart_router: smart_router,
            transfer_event_signature: transfer_event_signature,
        }
    }

    pub fn get_trade(&self, tx: &Tx, chain: Chain) -> Option<Trade> {
        /* if tx is successful, all of the following should be Some */
        let value = match tx.get_value() {
            Some(value) => value,
            None => {
                println!("Failed to get value");
                return None;
            }
        };

        /* all swaps go through the "multicall" smart router function */
        let function_calls = match self.get_multicall_funcs_and_params(&tx) {
            Some(functions_and_params) => functions_and_params,
            None => {
                println!("Failed to get multicall functions and params");
                return None;
            }
        };

        println!("");
        println!("function_calls number: {:?}", function_calls.len());
        for call in function_calls {
            println!("tx hash: {:?}", tx.get_id());
            println!("caller: {:?}", tx.get_from());
            println!("function: {:?}", call.get_name());
            println!("entire call: {:?}", call);
            let method_name = call.get_name();
            if let Some(method) = self.get_method_by_name(&method_name) {
                match method {
                    /* V2 */
                    PancakeSwapMethod::SwapExactTokensForTokens => {
                        return self.swap_exact_tokens_for_tokens(&tx, &call, &chain)
                    }
                    PancakeSwapMethod::SwapTokensForExactTokens => {
                        return self.swap_tokens_for_exact_tokens(&tx, &call, &chain)
                    }
                    /* V3 */
                    PancakeSwapMethod::ExactInputSingle => {
                        return self.exact_input_single(&tx, &call, &chain)
                    }
                    PancakeSwapMethod::ExactOutputSingle => {
                        return self.exact_output_single(&tx, &call, &chain)
                    }
                    PancakeSwapMethod::ExactInput => return self.exact_input(&tx, &call, &chain),
                    PancakeSwapMethod::ExactOutput => return self.exact_output(&tx, &call, &chain),
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
            None => {
                println!("no input data");
                return None;
            }
        };

        let multicall = match ContractCall::from_inputs(&self.smart_router, &multicall_input_data) {
            Some(multicall) => multicall,
            None => {
                println!("failed to parse multicall");
                return None;
            }
        };

        let mut actual_function_calls: Vec<ContractCall> = Vec::new();
        /* the single parameter from "multicall" is ambiguously called "data" */
        if let Some(param) = multicall.get_param("data") {
            /* data is an unsized array of byte arrays */
            let value_array = match param.get_value() {
                Token::Array(value) => value,
                _ => {
                    println!("data is not an array");
                    return None;
                }
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

    fn exact_input_single(&self, tx: &Tx, call: &ContractCall, chain: &Chain) -> Option<Trade> {
        /*
                function exactInputSingle(
                        ExactInputSingleParams memory params
                ) external payable override nonReentrant returns (uint256 amountOut)

                                struct ExactInputSingleParams {
                                        address tokenIn;
                                        address tokenOut;
                                        uint24 fee;
                                        address recipient;
                                        uint256 amountIn;
                                        uint256 amountOutMinimum;
                                        uint160 sqrtPriceLimitX96;
                                }
        */

        let params = match call.get_param("params") {
            Some(param) => match param.get_value() {
                Token::Tuple(value) => value,
                _ => {
                    println!("params is not a tuple");
                    return None;
                }
            },
            None => {
                println!("no params");
                return None;
            }
        };

        let token_in = match params[0] {
            Token::Address(param) => convert_h160_ethabi_to_web3(param),
            _ => {
                println!("token_in is not an address");
                return None;
            }
        };

        let token_out = match params[1] {
            Token::Address(param) => convert_h160_ethabi_to_web3(param),
            _ => {
                println!("token_out is not an address");
                return None;
            }
        };

        let fee = match params[2] {
            Token::Uint(param) => convert_u256_ethabi_to_web3(param),
            _ => {
                println!("fee is not a uint");
                return None;
            }
        };

        let recipient = match params[3] {
            Token::Address(param) => convert_h160_ethabi_to_web3(param),
            _ => {
                println!("recipient is not an address");
                return None;
            }
        };

        let amount_in = match params[4] {
            Token::Uint(param) => convert_u256_ethabi_to_web3(param),
            _ => {
                println!("amount_in is not a uint");
                return None;
            }
        };

        let amount_out_minimum = match params[5] {
            Token::Uint(param) => convert_u256_ethabi_to_web3(param),
            _ => {
                println!("amount_out_minimum is not a uint");
                return None;
            }
        };

        let caller = match tx.get_from() {
            Some(caller) => caller,
            None => {
                println!("failed to get caller");
                return None;
            }
        };

        let contract = match tx.get_to() {
            Some(contract) => contract,
            None => {
                println!("failed to get contract");
                return None;
            }
        };

        let amount_out = if recipient == caller {
            match tx.amount_of_token_received(token_out, recipient, self.transfer_event_signature) {
                Some(amount) => amount,
                None => {
                    println!("failed to get amount_out");
                    return None;
                }
            }
        } else if recipient == H160::from_str("0x0000000000000000000000000000000000000002").unwrap()
        {
            match tx.amount_of_token_received(token_out, contract, self.transfer_event_signature) {
                Some(amount) => amount,
                None => {
                    println!("failed to get amount_out");
                    return None;
                }
            }
        } else {
            println!("recipient is not caller or contract");
            return None;
        };

        Some(Trade::new(
            chain.to_owned(),
            contract,
            Dex::PancakeSwap,
            DexVersion::V3,
            token_in,
            token_out,
            None,
            None,
            caller,
            recipient,
            amount_in,
            amount_out,
        ))
    }

    fn exact_output_single(&self, tx: &Tx, call: &ContractCall, chain: &Chain) -> Option<Trade> {
        /*
                function exactOutputSingle(
                        ExactOutputSingleParams calldata params
                ) external payable override nonReentrant returns (uint256 amountIn)

                                struct ExactOutputSingleParams {
                                        address tokenIn;
                                        address tokenOut;
                                        uint24 fee;
                                        address recipient;
                                        uint256 amountOut;
                                        uint256 amountInMaximum;
                                        uint160 sqrtPriceLimitX96;
                                }
        */

        let params = match call.get_param("params") {
            Some(param) => match param.get_value() {
                Token::Tuple(value) => value,
                _ => {
                    println!("params is not a tuple");
                    return None;
                }
            },
            None => {
                println!("no params");
                return None;
            }
        };

        let token_in = match params[0] {
            Token::Address(param) => convert_h160_ethabi_to_web3(param),
            _ => {
                println!("token_in is not an address");
                return None;
            }
        };

        let token_out = match params[1] {
            Token::Address(param) => convert_h160_ethabi_to_web3(param),
            _ => {
                println!("token_out is not an address");
                return None;
            }
        };

        let fee = match params[2] {
            Token::Uint(param) => convert_u256_ethabi_to_web3(param),
            _ => {
                println!("fee is not a uint");
                return None;
            }
        };

        let recipient = match params[3] {
            Token::Address(param) => convert_h160_ethabi_to_web3(param),
            _ => {
                println!("recipient is not an address");
                return None;
            }
        };

        let amount_out = match params[4] {
            Token::Uint(param) => convert_u256_ethabi_to_web3(param),
            _ => {
                println!("amount_out is not a uint");
                return None;
            }
        };

        let amount_in_maximum = match params[5] {
            Token::Uint(param) => convert_u256_ethabi_to_web3(param),
            _ => {
                println!("amount_in_maximum is not a uint");
                return None;
            }
        };

        let caller = match tx.get_from() {
            Some(caller) => caller,
            None => {
                println!("failed to get caller");
                return None;
            }
        };

        let contract = match tx.get_to() {
            Some(contract) => contract,
            None => {
                println!("failed to get contract");
                return None;
            }
        };

        println!("token_in: {}", token_in);

        let amount_in: U256;
        if let Some(_) = tx.get_value() {
            /* if tx.value user paid in native tokens, the router wraps the tokens and sends it to the pool */
            /* even though token_in came from caller, it was sent by the router */
            amount_in =
                match tx.amount_of_token_sent(token_in, contract, self.transfer_event_signature) {
                    Some(amount) => amount,
                    None => {
                        println!("failed to get amount_in");
                        return None;
                    }
                };
        } else {
            amount_in =
                match tx.amount_of_token_sent(token_in, caller, self.transfer_event_signature) {
                    Some(amount) => amount,
                    None => {
                        println!("failed to get amount_in");
                        return None;
                    }
                };
        }

        Some(Trade::new(
            chain.to_owned(),
            contract,
            Dex::PancakeSwap,
            DexVersion::V3,
            token_in,
            token_out,
            None,
            None,
            caller,
            recipient,
            amount_in,
            amount_out,
        ))
    }

    fn exact_input(&self, tx: &Tx, call: &ContractCall, chain: &Chain) -> Option<Trade> {
        /*
                function exactInput(
                        ExactInputParams memory params
                ) external payable nonReentrant override returns (uint256 amountOut)

                                struct ExactInputParams {
                                        bytes path;
                                        address recipient;
                                        uint256 amountIn;
                                        uint256 amountOutMinimum;
                                }
        */

        let params = match call.get_param("params") {
            Some(param) => match param.get_value() {
                Token::Tuple(value) => value,
                _ => {
                    println!("params is not a tuple");
                    return None;
                }
            },
            None => {
                println!("no params");
                return None;
            }
        };

        let path = match &params[0] {
            Token::Bytes(bytes) => bytes,
            _ => {
                println!("path is not bytes");
                return None;
            }
        };

        if path.len() < 43 {
            /* 20 bytes for address, 3 bytes for uint24, 20 bytes for address */
            println!("path is too short");
            return None;
        }

        #[derive(Debug)]
        struct MultiHopPath {
            token_in: H160,
            fee: U256,
            token_out: H160,
        }

        println!("path length: {:?}", path.len());

        let mut full_path: Vec<MultiHopPath> = Vec::new();
        let mut token_in: H160 = H160::from_slice(&path[0..20]);
        for i in 0..((path.len() - 20) / 23) {
            let start = 20 + i * 23;
            let fee_bytes: [u8; 3] = match path[start..start + 3].try_into() {
                Ok(bytes) => bytes,
                Err(e) => {
                    println!(
                        "Error parsing 'path' from PancakeSwap exactInput call: {}",
                        e
                    );
                    return None;
                }
            };
            let fee = U256::from(u32::from_be_bytes([
                0,
                fee_bytes[0],
                fee_bytes[1],
                fee_bytes[2],
            ]));
            let token_out: H160 = H160::from_slice(&path[start + 3..start + 23]);
            full_path.push(MultiHopPath {
                token_in: token_in.clone(),
                fee,
                token_out: token_out.clone(),
            });
            token_in = token_out;
        }

        let recipient = match &params[1] {
            Token::Address(param) => convert_h160_ethabi_to_web3(*param),
            _ => {
                println!("recipient is not an address");
                return None;
            }
        };

        let amount_in = match &params[2] {
            Token::Uint(param) => convert_u256_ethabi_to_web3(*param),
            _ => {
                println!("amount_in is not a uint");
                return None;
            }
        };

        let amount_out_minimum = match &params[3] {
            Token::Uint(param) => convert_u256_ethabi_to_web3(*param),
            _ => {
                println!("amount_out_minimum is not a uint");
                return None;
            }
        };

        let caller = match tx.get_from() {
            Some(caller) => caller,
            None => {
                println!("failed to get caller");
                return None;
            }
        };

        let contract = match tx.get_to() {
            Some(contract) => contract,
            None => {
                println!("failed to get contract");
                return None;
            }
        };

        println!("recipient: {}", recipient);
        println!("amount_in: {}", amount_in);
        println!("amount_out_minimum: {}", amount_out_minimum);
        println!("caller: {}", caller);
        println!("multihop path: {:?}", full_path);

        let amount_out = if recipient == caller {
            match tx.amount_of_token_received(
                full_path[full_path.len() - 1].token_out,
                recipient,
                self.transfer_event_signature,
            ) {
                Some(amount) => amount,
                None => {
                    println!("failed to get amount_out");
                    return None;
                }
            }
        } else if recipient == H160::from_str("0x0000000000000000000000000000000000000002").unwrap()
        {
            match tx.amount_of_token_received(
                full_path[full_path.len() - 1].token_out,
                contract,
                self.transfer_event_signature,
            ) {
                Some(amount) => amount,
                None => {
                    println!("failed to get amount_out");
                    return None;
                }
            }
        } else {
            println!("recipient is not caller or contract");
            return None;
        };

        Some(Trade::new(
            chain.to_owned(),
            contract,
            Dex::PancakeSwap,
            DexVersion::V3,
            full_path[0].token_in,
            full_path[full_path.len() - 1].token_out,
            None,
            None,
            caller,
            recipient,
            amount_in,
            amount_out,
        ))
    }

    fn exact_output(&self, tx: &Tx, call: &ContractCall, chain: &Chain) -> Option<Trade> {
        /*
                function exactOutput(
                        ExactOutputParams calldata params
                ) external payable override nonReentrant returns (uint256 amountIn)

                                struct ExactOutputParams {
                                        bytes path;
                                        address recipient;
                                        uint256 amountOut;
                                        uint256 amountInMaximum;
                                }
        */

        let params = match call.get_param("params") {
            Some(param) => match param.get_value() {
                Token::Tuple(value) => value,
                _ => {
                    println!("params is not a tuple");
                    return None;
                }
            },
            None => {
                println!("no params");
                return None;
            }
        };

        let path = match &params[0] {
            Token::Bytes(bytes) => bytes,
            _ => {
                println!("path is not bytes");
                return None;
            }
        };

        if path.len() < 43 {
            /* 20 bytes for address, 3 bytes for uint24, 20 bytes for address */
            println!("path is too short");
            return None;
        }

        let token_out: H160 = H160::from_slice(&path[0..20]);
        let fee_bytes: [u8; 3] = match path[20..23].try_into() {
            Ok(bytes) => bytes,
            Err(e) => {
                println!(
                    "Error parsing 'path' from PancakeSwap exactInput call: {}",
                    e
                );
                return None;
            }
        };
        let fee = U256::from(u32::from_be_bytes([
            0,
            fee_bytes[0],
            fee_bytes[1],
            fee_bytes[2],
        ]));
        let token_in: H160 = H160::from_slice(&path[23..43]);

        let recipient = match &params[1] {
            Token::Address(param) => convert_h160_ethabi_to_web3(*param),
            _ => {
                println!("recipient is not an address");
                return None;
            }
        };

        let amount_out = match &params[2] {
            Token::Uint(param) => convert_u256_ethabi_to_web3(*param),
            _ => {
                println!("amount_out is not a uint");
                return None;
            }
        };

        let amount_in_maximum = match &params[3] {
            Token::Uint(param) => convert_u256_ethabi_to_web3(*param),
            _ => {
                println!("amount_in_maximum is not a uint");
                return None;
            }
        };

        let caller = match tx.get_from() {
            Some(caller) => caller,
            None => {
                println!("failed to get caller");
                return None;
            }
        };

        let contract = match tx.get_to() {
            Some(contract) => contract,
            None => {
                println!("failed to get contract");
                return None;
            }
        };

        println!("token_in: {}", token_in);
        println!("token_out: {}", token_out);
        println!("fee: {}", fee);
        println!("recipient: {}", recipient);
        println!("amount_in_maximum: {}", amount_in_maximum);
        println!("amount_out: {}", amount_out);
        println!("caller: {}", caller);

        let amount_in = if recipient == caller {
            match tx.amount_of_token_sent(token_in, recipient, self.transfer_event_signature) {
                Some(amount) => amount,
                None => {
                    println!("failed to get amount_in");
                    return None;
                }
            }
        } else if recipient == H160::from_str("0x0000000000000000000000000000000000000002").unwrap()
        {
            match tx.amount_of_token_sent(token_in, contract, self.transfer_event_signature) {
                Some(amount) => amount,
                None => {
                    println!("failed to get amount_in");
                    return None;
                }
            }
        } else {
            println!("recipient is not caller or contract");
            return None;
        };

        Some(Trade::new(
            chain.to_owned(),
            contract,
            Dex::PancakeSwap,
            DexVersion::V3,
            token_in,
            token_out,
            None,
            None,
            caller,
            recipient,
            amount_in,
            amount_out,
        ))
    }

    fn swap_exact_tokens_for_tokens(
        &self,
        tx: &Tx,
        call: &ContractCall,
        chain: &Chain,
    ) -> Option<Trade> {
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
                _ => {
                    println!("amountIn is not a uint");
                    return None;
                }
            },
            None => {
                println!("no amountIn");
                return None;
            }
        };

        let amount_out_min = match call.get_param("amountOutMin") {
            Some(param) => match param.get_value() {
                Token::Uint(value) => convert_u256_ethabi_to_web3(value),
                _ => {
                    println!("amountOutMin is not a uint");
                    return None;
                }
            },
            None => {
                println!("no amountOutMin");
                return None;
            }
        };

        let path = match call.get_param("path") {
            Some(param) => match param.get_value() {
                Token::Array(value) => value,
                _ => {
                    println!("path is not an array");
                    return None;
                }
            },
            None => {
                println!("no path");
                return None;
            }
        };

        let token_in = match path[0].clone() {
            Token::Address(value) => convert_h160_ethabi_to_web3(value),
            _ => {
                println!("token_in is not an address");
                return None;
            }
        };

        let token_out = match path[path.len() - 1].clone() {
            Token::Address(value) => convert_h160_ethabi_to_web3(value),
            _ => {
                println!("token_out is not an address");
                return None;
            }
        };

        let recipient = match call.get_param("to") {
            Some(param) => match param.get_value() {
                Token::Address(value) => convert_h160_ethabi_to_web3(value),
                _ => {
                    println!("recipient is not an address");
                    return None;
                }
            },
            None => {
                println!("no recipient");
                return None;
            }
        };

        let caller = match tx.get_from() {
            Some(caller) => caller,
            None => {
                println!("failed to get caller");
                return None;
            }
        };

        let contract = match tx.get_to() {
            Some(contract) => contract,
            None => {
                println!("failed to get contract");
                return None;
            }
        };

        println!("token_in: {}", token_in);
        println!("token_out: {}", token_out);
        println!("recipient: {}", recipient);
        println!("amount_in: {}", amount_in);
        println!("amount_out_min: {}", amount_out_min);
        println!("caller: {}", caller);

        let amount_out = if recipient == caller {
            match tx.amount_of_token_received(token_out, recipient, self.transfer_event_signature) {
                Some(amount) => amount,
                None => {
                    println!("failed to get amount_out");
                    return None;
                }
            }
        } else if recipient == H160::from_str("0x0000000000000000000000000000000000000002").unwrap()
        {
            match tx.amount_of_token_received(token_out, contract, self.transfer_event_signature) {
                Some(amount) => amount,
                None => {
                    println!("failed to get amount_out");
                    return None;
                }
            }
        } else {
            println!("recipient is not caller or contract");
            return None;
        };

        Some(Trade::new(
            chain.to_owned(),
            contract,
            Dex::PancakeSwap,
            DexVersion::V2,
            token_in,
            token_out,
            None,
            None,
            caller,
            recipient,
            amount_in,
            amount_out,
        ))
    }

    fn swap_tokens_for_exact_tokens(
        &self,
        tx: &Tx,
        call: &ContractCall,
        chain: &Chain,
    ) -> Option<Trade> {
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
                _ => {
                    println!("amountOut is not a uint");
                    return None;
                }
            },
            None => {
                println!("no amountOut");
                return None;
            }
        };

        let amount_in_max = match call.get_param("amountInMax") {
            Some(param) => match param.get_value() {
                Token::Uint(value) => convert_u256_ethabi_to_web3(value),
                _ => {
                    println!("amountInMax is not a uint");
                    return None;
                }
            },
            None => {
                println!("no amountInMax");
                return None;
            }
        };

        let path = match call.get_param("path") {
            Some(param) => match param.get_value() {
                Token::Array(value) => value,
                _ => {
                    println!("path is not an array");
                    return None;
                }
            },
            None => {
                println!("no path");
                return None;
            }
        };

        let token_in = match path[0].clone() {
            Token::Address(value) => convert_h160_ethabi_to_web3(value),
            _ => {
                println!("token_in is not an address");
                return None;
            }
        };

        let token_out = match path[1].clone() {
            Token::Address(value) => convert_h160_ethabi_to_web3(value),
            _ => {
                println!("token_out is not an address");
                return None;
            }
        };

        let recipient = match call.get_param("to") {
            Some(param) => match param.get_value() {
                Token::Address(value) => convert_h160_ethabi_to_web3(value),
                _ => {
                    println!("recipient is not an address");
                    return None;
                }
            },
            None => {
                println!("no recipient");
                return None;
            }
        };

        let caller = match tx.get_from() {
            Some(caller) => caller,
            None => {
                println!("failed to get caller");
                return None;
            }
        };

        let contract = match tx.get_to() {
            Some(contract) => contract,
            None => {
                println!("failed to get contract");
                return None;
            }
        };

        println!("token_in: {}", token_in);
        println!("token_out: {}", token_out);
        println!("recipient: {}", recipient);
        println!("amount_in_max: {}", amount_in_max);
        println!("amount_out: {}", amount_out);
        println!("caller: {}", caller);

        let amount_in =
            match tx.amount_of_token_sent(token_in, recipient, self.transfer_event_signature) {
                Some(amount) => amount,
                None => {
                    println!("failed to get amount_in");
                    return None;
                }
            };

        Some(Trade::new(
            chain.to_owned(),
            contract,
            Dex::PancakeSwap,
            DexVersion::V2,
            token_in,
            token_out,
            None,
            None,
            caller,
            recipient,
            amount_in,
            amount_out,
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
