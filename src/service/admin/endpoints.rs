use model::endpoint::*;
use model::types::{Field, Type};

pub fn endpoint_admin_list_users() -> EndpointSchema {
    EndpointSchema::new(
        "AdminListUsers",
        30010,
        vec![
            Field::new("limit", Type::BigInt),
            Field::new("offset", Type::BigInt),
            Field::new("user_id", Type::optional(Type::BigInt)),
            Field::new("address", Type::optional(Type::String)),
            Field::new("username", Type::optional(Type::String)),
            Field::new("email", Type::optional(Type::String)),
            Field::new("role", Type::optional(Type::enum_ref("role"))),
        ],
        vec![Field::new(
            "users",
            Type::datatable(
                "ListUserRow",
                vec![
                    Field::new("user_id", Type::BigInt),
                    Field::new("public_user_id", Type::BigInt),
                    Field::new("username", Type::optional(Type::String)),
                    Field::new("address", Type::String),
                    Field::new("last_ip", Type::Inet),
                    Field::new("last_login_at", Type::BigInt),
                    Field::new("login_count", Type::Int),
                    Field::new("role", Type::enum_ref("role")),
                    Field::new("email", Type::optional(Type::String)),
                    Field::new("updated_at", Type::BigInt),
                    Field::new("created_at", Type::BigInt),
                ],
            ),
        )],
    )
}
pub fn endpoint_admin_set_user_role() -> EndpointSchema {
    EndpointSchema::new(
        "AdminSetUserRole",
        30020,
        vec![
            Field::new("user_id", Type::BigInt),
            Field::new("role", Type::enum_ref("role")),
        ],
        vec![],
    )
}
pub fn endpoint_admin_set_block_user() -> EndpointSchema {
    EndpointSchema::new(
        "AdminSetBlockUser",
        30030,
        vec![
            Field::new("user_id", Type::BigInt),
            Field::new("blocked", Type::Boolean),
        ],
        vec![],
    )
}
pub fn endpoint_admin_approve_user_become_expert() -> EndpointSchema {
    EndpointSchema::new(
        "AdminApproveUserBecomeExpert",
        30040,
        vec![Field::new("user_id", Type::BigInt)],
        vec![Field::new("success", Type::Boolean)],
    )
    .with_description("Admin approves a user to become an expert")
}
pub fn endpoint_admin_reject_user_become_expert() -> EndpointSchema {
    EndpointSchema::new(
        "AdminRejectUserBecomeExpert",
        30050,
        vec![Field::new("user_id", Type::BigInt)],
        vec![Field::new("success", Type::Boolean)],
    )
    .with_description("Admin approves a user to become an expert")
}
pub fn endpoint_admin_list_pending_expert_applications() -> EndpointSchema {
    EndpointSchema::new(
        "AdminListPendingExpertApplications",
        30060,
        vec![],
        vec![Field::new(
            "users",
            Type::datatable(
                "ListPendingExpertApplicationsRow",
                vec![
                    Field::new("user_id", Type::BigInt),
                    Field::new("name", Type::String),
                    Field::new("follower_count", Type::Int),
                    Field::new("description", Type::String),
                    Field::new("social_media", Type::String),
                    Field::new("risk_score", Type::Numeric),
                    Field::new("reputation_score", Type::Numeric),
                    Field::new("aum", Type::Numeric),
                ],
            ),
        )],
    )
    .with_description("Admin approves a user to become an expert")
}
pub fn get_admin_endpoints() -> Vec<EndpointSchema> {
    vec![
        endpoint_admin_list_users(),
        endpoint_admin_set_user_role(),
        endpoint_admin_set_block_user(),
        endpoint_admin_list_pending_expert_applications(),
        endpoint_admin_approve_user_become_expert(),
        endpoint_admin_reject_user_become_expert(),
    ]
}
