use crate::types::{Field, Type};
use convert_case::Case;
use convert_case::Casing;
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct ProceduralFunction {
    pub name: String,
    pub parameters: Vec<Field>,
    pub return_row_type: Type,
    pub body: String,
}
fn sort_parameters(parameters: Vec<Field>) -> Vec<Field> {
    parameters
        .into_iter()
        .sorted_by_cached_key(|x| matches!(x.ty, Type::Optional(_)))
        .collect()
}

impl ProceduralFunction {
    pub fn new(
        name: impl Into<String>,
        parameters: Vec<Field>,
        returns: Vec<Field>,
        body: impl Into<String>,
    ) -> Self {
        let name = name.into();
        Self {
            name: name.clone(),
            parameters: sort_parameters(parameters),
            return_row_type: Type::struct_(
                format!("{}RespRow", name.to_case(Case::Pascal)),
                returns,
            ),
            body: body.into(),
        }
    }
    pub fn new_with_row_type(
        name: impl Into<String>,
        parameters: Vec<Field>,
        return_row_type: Type,
        body: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            parameters: sort_parameters(parameters),
            return_row_type,
            body: body.into(),
        }
    }
}
