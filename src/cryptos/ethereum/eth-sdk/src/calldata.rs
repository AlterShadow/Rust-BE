use ethabi::{Contract, Param, ParamType, StateMutability, Token};
use eyre::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::mem::transmute;

#[derive(Clone, Debug, Serialize, Deserialize)]
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
            state_mutability,
        }
    }

    pub fn from_inputs(contract: &Contract, input_data: &[u8]) -> Result<ContractCall> {
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
                    SerializableParamType::from_ethabi(parameter.kind.clone()),
                    SerializableParam::from_ethabi(parameter.clone()),
                ),
            );
        }

        Ok(Self::new(
            function.name.clone(),
            parameters,
            function.state_mutability,
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
/// Function and event param types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SerializableParamType {
    /// Address.
    Address,
    /// Bytes.
    Bytes,
    /// Signed integer.
    Int(usize),
    /// Unsigned integer.
    Uint(usize),
    /// Boolean.
    Bool,
    /// String.
    String,
    /// Array of unknown size.
    Array(Box<SerializableParamType>),
    /// Vector of bytes with fixed size.
    FixedBytes(usize),
    /// Array with fixed size.
    FixedArray(Box<SerializableParamType>, usize),
    /// Tuple containing different types
    Tuple(Vec<SerializableParamType>),
}
impl SerializableParamType {
    pub fn from_ethabi(param_type: ParamType) -> Self {
        unsafe { transmute(param_type) }
    }
}
/// Function param.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SerializableParam {
    /// Param name.
    pub name: String,
    /// Param type.
    pub kind: SerializableParamType,
    /// Additional Internal type.
    pub internal_type: Option<String>,
}
impl SerializableParam {
    pub fn from_ethabi(param: Param) -> Self {
        unsafe { transmute(param) }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CallParameter {
    name: String,
    value: Token,
    param_type: SerializableParamType,
    inner: SerializableParam,
}

impl CallParameter {
    pub fn new(
        name: String,
        value: Token,
        param_type: SerializableParamType,
        inner: SerializableParam,
    ) -> Self {
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

    pub fn get_param_type(&self) -> SerializableParamType {
        self.param_type.clone()
    }
    pub fn get_inner(&self) -> SerializableParam {
        self.inner.clone()
    }
}
