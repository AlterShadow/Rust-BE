use model::endpoint::*;
use model::types::{Field, Type};

pub fn endpoint_auth_signup() -> EndpointSchema {
    EndpointSchema::new(
        "Signup",
        10010,
        vec![
            Field::new("address", Type::BlockchainAddress),
            Field::new("signature_text", Type::String),
            Field::new("signature", Type::String),
            Field::new("email", Type::String),
            Field::new("phone", Type::String),
            Field::new("agreed_tos", Type::Boolean),
            Field::new("agreed_privacy", Type::Boolean),
            Field::new("username", Type::String),
        ],
        vec![
            Field::new("address", Type::BlockchainAddress),
            Field::new("user_id", Type::BigInt),
        ],
    )
}
pub fn endpoint_auth_login() -> EndpointSchema {
    EndpointSchema::new(
        "Login",
        10020,
        vec![
            Field::new("address", Type::BlockchainAddress),
            Field::new("signature_text", Type::String),
            Field::new("signature", Type::String),
            Field::new("service", Type::enum_ref("service")),
            Field::new("device_id", Type::String),
            Field::new("device_os", Type::String),
        ],
        vec![
            Field::new("address", Type::BlockchainAddress),
            Field::new("display_name", Type::String),
            Field::new("avatar", Type::optional(Type::String)),
            Field::new("role", Type::enum_ref("role")),
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
            Field::new("address", Type::BlockchainAddress),
            Field::new("token", Type::UUID),
            Field::new("service", Type::enum_ref("service")),
            Field::new("device_id", Type::String),
            Field::new("device_os", Type::String),
        ],
        vec![Field::new("success", Type::Boolean)],
    )
}
pub fn endpoint_auth_logout() -> EndpointSchema {
    EndpointSchema::new("Logout", 10040, vec![], vec![])
}
pub fn endpoint_auth_change_login_wallet() -> EndpointSchema {
    EndpointSchema::new(
        "ChangeLoginWallet",
        10050,
        vec![
            Field::new("old_address", Type::BlockchainAddress),
            Field::new("old_signature_text", Type::String),
            Field::new("old_signature", Type::String),
            Field::new("new_address", Type::BlockchainAddress),
            Field::new("new_signature_text", Type::String),
            Field::new("new_signature", Type::String),
        ],
        vec![],
    )
}

pub fn get_auth_endpoints() -> Vec<EndpointSchema> {
    vec![
        endpoint_auth_login(),
        endpoint_auth_signup(),
        endpoint_auth_authorize(),
        endpoint_auth_logout(),
        endpoint_auth_change_login_wallet(),
    ]
}
