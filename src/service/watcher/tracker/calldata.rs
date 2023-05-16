use std::collections::HashMap;

use eyre::*;

use ethabi::{Contract, Param, ParamType, StateMutability, Token};

#[derive(Clone, Debug)]
pub struct ContractCall {
    name: String,
    params: HashMap<String, CallParameter>,
    state_mutability: StateMutability,
}

impl ContractCall {
    pub fn new(
        name: String,
        params: HashMap<String, CallParameter>,
        state_mutability: StateMutability,
    ) -> Self {
        Self {
            name,
            params,
            state_mutability: state_mutability,
        }
    }

    pub fn from_inputs(contract: &Contract, input_data: &Vec<u8>) -> Result<ContractCall> {
        let function = match contract
            .functions()
            .find(|function| function.short_signature() == input_data[..4])
        {
            Some(function) => function,
            None => {
                return Err(eyre!("could not find function"));
            }
        };

        let mut parameters: HashMap<String, CallParameter> = HashMap::new();

        let parameter_values = match function.decode_input(&input_data[4..]) {
            Ok(values) => values,
            Err(e) => {
                return Err(eyre!("could not decode input: {:?}", e));
            }
        };

        for (parameter, value) in function.inputs.iter().zip(parameter_values) {
            parameters.insert(
                parameter.name.clone(),
                CallParameter::new(
                    parameter.name.clone(),
                    value,
                    parameter.kind.clone(),
                    parameter.clone(),
                ),
            );
        }

        Ok(Self::new(
            function.name.clone(),
            parameters,
            function.state_mutability.clone(),
        ))
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_params(&self) -> HashMap<String, CallParameter> {
        self.params.clone()
    }

    pub fn get_param(&self, name: &str) -> Option<&CallParameter> {
        self.params.get(name)
    }

    pub fn get_state_mutability(&self) -> StateMutability {
        self.state_mutability.clone()
    }
}

#[derive(Clone, Debug)]
pub struct CallParameter {
    name: String,
    value: Token,
    param_type: ParamType,
    inner: Param,
}

impl CallParameter {
    pub fn new(name: String, value: Token, param_type: ParamType, inner: Param) -> Self {
        Self {
            name,
            value,
            param_type,
            inner,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_value(&self) -> Token {
        self.value.clone()
    }

    pub fn get_param_type(&self) -> ParamType {
        self.param_type.clone()
    }
    pub fn get_inner(&self) -> Param {
        self.inner.clone()
    }
}
