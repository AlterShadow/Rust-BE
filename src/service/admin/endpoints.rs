use model::endpoint::*;
use model::types::{Field, Type};

pub fn get_admin_endpoints() -> Vec<EndpointSchema> {
    vec![
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
        ),
        EndpointSchema::new(
            "AdminSetUserRole",
            30020,
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("role", Type::enum_ref("role")),
            ],
            vec![],
        ),
        EndpointSchema::new(
            "AdminSetBlockUser",
            30030,
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("blocked", Type::Boolean),
            ],
            vec![],
        ),
        EndpointSchema::new(
            "AdminListPendingExpertApplications",
            30060,
            vec![
                Field::new("offset", Type::optional(Type::BigInt)),
                Field::new("limit", Type::optional(Type::BigInt)),
            ],
            vec![Field::new(
                "users",
                Type::datatable(
                    "ListPendingExpertApplicationsRow",
                    vec![
                        Field::new("user_id", Type::BigInt),
                        Field::new("name", Type::String),
                        Field::new("linked_wallet", Type::String),
                        Field::new("joined_at", Type::BigInt),
                        Field::new("requested_at", Type::BigInt),
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
        .with_description("Admin approves a user to become an expert"),
        EndpointSchema::new(
            "AdminApproveUserBecomeExpert",
            30040,
            vec![Field::new("user_id", Type::BigInt)],
            vec![Field::new("success", Type::Boolean)],
        )
        .with_description("Admin approves a user to become an expert"),
        EndpointSchema::new(
            "AdminRejectUserBecomeExpert",
            30050,
            vec![Field::new("user_id", Type::BigInt)],
            vec![Field::new("success", Type::Boolean)],
        )
        .with_description("Admin approves a user to become an expert"),
        EndpointSchema::new(
            "AdminGetSystemConfig",
            30070,
            vec![],
            vec![
                Field::new("config_placeholder_1", Type::BigInt),
                Field::new("config_placeholder_2", Type::BigInt),
            ],
        )
        .with_description("Admin get system config"),
        EndpointSchema::new(
            "AdminUpdateSystemConfig",
            30080,
            vec![
                Field::new("config_placeholder_1", Type::optional(Type::BigInt)),
                Field::new("config_placeholder_2", Type::optional(Type::BigInt)),
            ],
            vec![Field::new("success", Type::Boolean)],
        )
        .with_description("Admin updates system config"),
        EndpointSchema::new(
            "AdminListExperts",
            30090,
            vec![
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
                Field::new("expert_id", Type::optional(Type::BigInt)),
                Field::new("user_id", Type::optional(Type::BigInt)),
                Field::new("user_public_id", Type::optional(Type::BigInt)),
                Field::new("username", Type::optional(Type::String)),
                Field::new("family_name", Type::optional(Type::String)),
                Field::new("given_name", Type::optional(Type::String)),
                Field::new("description", Type::optional(Type::String)),
                Field::new("social_media", Type::optional(Type::String)),
            ],
            vec![Field::new(
                "experts",
                Type::datatable(
                    "AdminListExpertsRow",
                    vec![
                        Field::new("expert_id", Type::BigInt),
                        Field::new("user_public_id", Type::BigInt),
                        Field::new("linked_wallet", Type::String),
                        Field::new("name", Type::String),
                        Field::new("family_name", Type::optional(Type::String)),
                        Field::new("given_name", Type::optional(Type::String)),
                        Field::new("follower_count", Type::BigInt),
                        Field::new("description", Type::String),
                        Field::new("social_media", Type::String),
                        Field::new("risk_score", Type::Numeric),
                        Field::new("reputation_score", Type::Numeric),
                        Field::new("aum", Type::Numeric),
                        Field::new("joined_at", Type::BigInt),
                        Field::new("requested_at", Type::BigInt),
                        Field::new("approved_at", Type::optional(Type::BigInt)),
                        Field::new("pending_expert", Type::Boolean),
                        Field::new("approved_expert", Type::Boolean),
                    ],
                ),
            )],
        )
        .with_description("Admin lists experts"),
        EndpointSchema::new(
            "AdminListBackers",
            30100,
            vec![
                Field::new("offset", Type::optional(Type::BigInt)),
                Field::new("limit", Type::optional(Type::BigInt)),
            ],
            vec![Field::new(
                "backers",
                Type::datatable(
                    "AdminListBackersRow",
                    vec![
                        Field::new("username", Type::String),
                        Field::new("user_id", Type::BigInt),
                        Field::new("login_wallet_address", Type::String),
                        Field::new("joined_at", Type::BigInt),
                        Field::new("total_platform_fee_paid", Type::Numeric),
                        Field::new("total_strategy_fee_paid", Type::Numeric),
                        Field::new("total_backing_amount", Type::Numeric),
                    ],
                ),
            )],
        ),
        EndpointSchema::new(
            "AdminListStrategies",
            30110,
            vec![
                Field::new("offset", Type::optional(Type::BigInt)),
                Field::new("limit", Type::optional(Type::BigInt)),
            ],
            vec![Field::new(
                "strategies",
                Type::datatable(
                    "AdminListStrategiesRow",
                    vec![
                        Field::new("strategy_id", Type::BigInt),
                        Field::new("strategy_name", Type::String),
                        // Field::new("expert_id", Type::BigInt),
                        Field::new("expert_public_id", Type::BigInt),
                        Field::new("expert_name", Type::String),
                        Field::new("description", Type::optional(Type::String)),
                        Field::new("created_at", Type::BigInt),
                        Field::new("approved_at", Type::optional(Type::BigInt)),
                        Field::new("pending_strategy", Type::Boolean),
                        Field::new("approved_strategy", Type::Boolean),
                    ],
                ),
            )],
        ),
        EndpointSchema::new(
            "AdminAddWalletActivityHistory",
            31001,
            vec![
                Field::new("wallet_address", Type::String),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("transaction_hash", Type::String),
                Field::new("dex", Type::optional(Type::String)),
                Field::new("contract_address", Type::String),
                Field::new("token_in_address", Type::optional(Type::String)),
                Field::new("token_out_address", Type::optional(Type::String)),
                Field::new("caller_address", Type::String),
                Field::new("amount_in", Type::optional(Type::String)),
                Field::new("amount_out", Type::optional(Type::String)),
                Field::new("swap_calls", Type::optional(Type::Object)),
                Field::new("paths", Type::optional(Type::Object)),
                Field::new("dex_versions", Type::optional(Type::Object)),
                Field::new("created_at", Type::optional(Type::BigInt)),
            ],
            vec![],
        )
        .with_description("Admin adds wallet activity history. for mocking purpose"),
    ]
}
