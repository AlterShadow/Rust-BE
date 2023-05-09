use model::endpoint::*;
use model::types::{Field, Type};

pub fn endpoint_auth_signup() -> EndpointSchema {
    EndpointSchema::new(
        "Signup",
        10010,
        vec![
            Field::new("address", Type::String),
            Field::new("signature_text", Type::String),
            Field::new("signature", Type::String),
            Field::new("email", Type::String),
            Field::new("phone", Type::String),
            Field::new("agreed_tos", Type::Boolean),
            Field::new("agreed_privacy", Type::Boolean),
        ],
        vec![
            Field::new("address", Type::String),
            Field::new("user_id", Type::BigInt),
        ],
    )
}
pub fn endpoint_auth_login() -> EndpointSchema {
    EndpointSchema::new(
        "Login",
        10020,
        vec![
            Field::new("address", Type::String),
            Field::new("signature_text", Type::String),
            Field::new("signature", Type::String),
            Field::new("service_code", Type::enum_ref("service")),
            Field::new("device_id", Type::String),
            Field::new("device_os", Type::String),
        ],
        vec![
            Field::new("address", Type::String),
            Field::new("user_id", Type::BigInt),
            Field::new("user_token", Type::UUID),
            Field::new("admin_token", Type::UUID),
        ],
    )
}
pub fn endpoint_auth_authorize() -> EndpointSchema {
    EndpointSchema::new(
        "Authorize",
        10030,
        vec![
            Field::new("address", Type::String),
            Field::new("token", Type::UUID),
            Field::new("service_code", Type::enum_ref("service")),
            Field::new("device_id", Type::String),
            Field::new("device_os", Type::String),
        ],
        vec![Field::new("success", Type::Boolean)],
    )
}

pub fn get_auth_endpoints() -> Vec<EndpointSchema> {
    vec![
        endpoint_auth_login(),
        endpoint_auth_signup(),
        endpoint_auth_authorize(),
    ]
}
