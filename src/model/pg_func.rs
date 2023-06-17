use crate::types::{Field, Type};
use convert_case::Case;
use convert_case::Casing;

#[derive(Clone, Debug)]
pub struct ProceduralFunction {
    pub name: String,
    pub parameters: Vec<Field>,
    pub return_row_type: Type,
    pub body: String,
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
            parameters,
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
            parameters,
            return_row_type,
            body: body.into(),
        }
    }
}
