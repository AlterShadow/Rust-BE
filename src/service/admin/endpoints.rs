use model::endpoint::*;
use model::types::{Field, Type};
use shared_endpoints::*;

#[path = "../shared/endpoints.rs"]
mod shared_endpoints;
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
            vec![
                Field::new("users_total", Type::BigInt),
                Field::new(
                    "users",
                    Type::datatable(
                        "ListUserRow",
                        vec![
                            Field::new("user_id", Type::BigInt),
                            Field::new("public_user_id", Type::BigInt),
                            Field::new("username", Type::optional(Type::String)),
                            Field::new("address", Type::BlockchainAddress),
                            Field::new("last_ip", Type::Inet),
                            Field::new("last_login_at", Type::optional(Type::BigInt)),
                            Field::new("login_count", Type::Int),
                            Field::new("role", Type::enum_ref("role")),
                            Field::new("email", Type::optional(Type::String)),
                            Field::new("updated_at", Type::BigInt),
                            Field::new("created_at", Type::BigInt),
                        ],
                    ),
                ),
            ],
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
            vec![
                Field::new("users_total", Type::BigInt),
                Field::new(
                    "users",
                    Type::datatable(
                        "ListPendingExpertApplicationsRow",
                        vec![
                            Field::new("user_id", Type::BigInt),
                            Field::new("name", Type::String),
                            Field::new("linked_wallet", Type::BlockchainAddress),
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
                ),
            ],
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
                Field::new("platform_fee", Type::Numeric),
                Field::new("allow_domain_urls", Type::String),
                Field::new("escrow_contract_address_ethereum", Type::BlockchainAddress),
                Field::new("escrow_contract_address_goerli", Type::BlockchainAddress),
                Field::new("escrow_contract_address_bsc", Type::BlockchainAddress),
                Field::new(
                    "escrow_contract_address_bsc_testnet",
                    Type::BlockchainAddress,
                ),
            ],
        )
        .with_description("Admin get system config"),
        EndpointSchema::new(
            "AdminUpdateSystemConfig",
            30080,
            vec![
                Field::new("platform_fee", Type::optional(Type::Numeric)),
                Field::new("allow_domain_urls", Type::optional(Type::String)),
                Field::new(
                    "escrow_contract_address_ethereum",
                    Type::optional(Type::BlockchainAddress),
                ),
                Field::new(
                    "escrow_contract_address_goerli",
                    Type::optional(Type::BlockchainAddress),
                ),
                Field::new(
                    "escrow_contract_address_bsc",
                    Type::optional(Type::BlockchainAddress),
                ),
                Field::new(
                    "escrow_contract_address_bsc_testnet",
                    Type::optional(Type::BlockchainAddress),
                ),
            ],
            vec![Field::new("success", Type::Boolean)],
        )
        .with_description("Admin updates system config"),
        EndpointSchema::new(
            "AdminListExperts",
            30090,
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
                Field::new("expert_id", Type::optional(Type::BigInt)),
                Field::new("user_id", Type::optional(Type::BigInt)),
                Field::new("user_public_id", Type::optional(Type::BigInt)),
                Field::new("username", Type::optional(Type::String)),
                Field::new("family_name", Type::optional(Type::String)),
                Field::new("given_name", Type::optional(Type::String)),
                Field::new("description", Type::optional(Type::String)),
                Field::new("social_media", Type::optional(Type::String)),
            ],
            vec![
                Field::new("experts_total", Type::BigInt),
                Field::new("experts", list_experts_datatable()),
            ],
        )
        .with_description("Admin lists experts"),
        EndpointSchema::new(
            "AdminListBackers",
            30100,
            vec![
                Field::new("offset", Type::optional(Type::BigInt)),
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("user_id", Type::optional(Type::BigInt)),
                Field::new("user_public_id", Type::optional(Type::BigInt)),
                Field::new("username", Type::optional(Type::String)),
                Field::new("family_name", Type::optional(Type::String)),
                Field::new("given_name", Type::optional(Type::String)),
                // Field::new("description", Type::optional(Type::String)),
                // Field::new("social_media", Type::optional(Type::String)),
            ],
            vec![
                Field::new("backers_total", Type::BigInt),
                Field::new(
                    "backers",
                    Type::datatable(
                        "AdminListBackersRow",
                        vec![
                            Field::new("username", Type::String),
                            Field::new("user_id", Type::BigInt),
                            Field::new("login_wallet_address", Type::BlockchainAddress),
                            Field::new("joined_at", Type::BigInt),
                            Field::new("total_platform_fee_paid", Type::Numeric),
                            Field::new("total_strategy_fee_paid", Type::Numeric),
                            Field::new("total_backing_amount", Type::Numeric),
                        ],
                    ),
                ),
            ],
        ),
        EndpointSchema::new(
            "AdminListStrategies",
            30110,
            vec![
                Field::new("offset", Type::optional(Type::BigInt)),
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("strategy_id", Type::optional(Type::BigInt)),
                Field::new("strategy_name", Type::optional(Type::String)),
                Field::new("expert_id", Type::optional(Type::BigInt)),
                Field::new("expert_name", Type::optional(Type::String)),
                Field::new("description", Type::optional(Type::String)),
                Field::new("pending_approval", Type::optional(Type::Boolean)),
                Field::new("approved", Type::optional(Type::Boolean)),
            ],
            vec![
                Field::new("strategies_total", Type::BigInt),
                Field::new("strategies", list_strategies_datatable()),
            ],
        ),
        EndpointSchema::new(
            "AdminApproveStrategy",
            30120,
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![Field::new("success", Type::Boolean)],
        )
        .with_description("Admin approves strategy"),
        EndpointSchema::new(
            "AdminRefreshExpertWalletBalance",
            30121,
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![Field::new("success", Type::Boolean)],
        ),
        EndpointSchema::new(
            "AdminRejectStrategy",
            30130,
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![Field::new("success", Type::Boolean)],
        ),
        EndpointSchema::new(
            "AdminAddAuditRule",
            31002,
            vec![
                Field::new("rule_id", Type::BigInt),
                Field::new("name", Type::String),
                Field::new("description", Type::String),
            ],
            vec![],
        ),
        EndpointSchema::new(
            "AdminNotifyEscrowLedgerChange",
            32010,
            vec![
                Field::new("pkey_id", Type::BigInt),
                Field::new("user_id", Type::BigInt),
                Field::new("balance", user_deposit_withdraw_ledger_entry()),
            ],
            vec![],
        ),
        EndpointSchema::new(
            "AdminSubscribeDepositLedger",
            32011,
            vec![
                Field::new("initial_data", Type::optional(Type::BigInt)),
                Field::new("blockchain", Type::optional(Type::enum_ref("block_chain"))),
                Field::new("mock_data", Type::optional(Type::Boolean)),
            ],
            vec![],
        )
        .with_stream_response_type(user_deposit_withdraw_ledger_entry()),
        EndpointSchema::new("AdminUnsubscribeDepositLedger", 32012, vec![], vec![]),
        EndpointSchema::new(
            "AdminAddEscrowTokenContractAddress",
            32020,
            vec![
                Field::new("pkey_id", Type::optional(Type::BigInt)),
                Field::new("symbol", Type::String),
                Field::new("short_name", Type::String),
                Field::new("description", Type::String),
                Field::new("address", Type::BlockchainAddress),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("is_stablecoin", Type::Boolean),
                Field::new("is_wrapped", Type::Boolean),
            ],
            vec![],
        ),
        EndpointSchema::new(
            "AdminAddEscrowContractAddress",
            32030,
            vec![
                Field::new("pkey_id", Type::BigInt),
                Field::new("address", Type::BlockchainAddress),
                Field::new("blockchain", Type::enum_ref("block_chain")),
            ],
            vec![],
        ),
        EndpointSchema::new(
            "AdminListBackStrategyLedger",
            32040,
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
                Field::new("strategy_id", Type::optional(Type::BigInt)),
            ],
            vec![
                Field::new("back_ledger_total", Type::BigInt),
                Field::new(
                    "back_ledger",
                    Type::datatable(
                        "AdminBackStrategyLedgerRow",
                        vec![
                            Field::new("back_ledger_id", Type::BigInt),
                            Field::new("user_id", Type::BigInt),
                            Field::new("strategy_id", Type::BigInt),
                            Field::new("quantity", Type::BlockchainDecimal),
                            Field::new("blockchain", Type::enum_ref("block_chain")),
                            Field::new("transaction_hash", Type::BlockchainTransactionHash),
                            Field::new("happened_at", Type::BigInt),
                        ],
                    ),
                ),
            ],
        ),
        EndpointSchema::new(
            "AdminListExitStrategyLedger",
            32041,
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
                Field::new("strategy_id", Type::optional(Type::BigInt)),
            ],
            vec![
                Field::new("exit_ledger_total", Type::BigInt),
                Field::new(
                    "exit_ledger",
                    Type::datatable(
                        "AdminExitStrategyLedgerRow",
                        vec![
                            Field::new("back_ledger_id", Type::BigInt),
                            Field::new("user_id", Type::BigInt),
                            Field::new("strategy_id", Type::BigInt),
                            Field::new("quantity", Type::BlockchainDecimal),
                            Field::new("blockchain", Type::enum_ref("block_chain")),
                            Field::new("transaction_hash", Type::BlockchainTransactionHash),
                            Field::new("happened_at", Type::BigInt),
                        ],
                    ),
                ),
            ],
        ),
        EndpointSchema::new(
            "AdminSetBlockchainLogger",
            32050,
            vec![Field::new("enabled", Type::Boolean)],
            vec![],
        ),
        EndpointSchema::new(
            "AdminListEscrowTokenContractAddresses",
            32060,
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
                Field::new("symbol", Type::optional(Type::String)),
                Field::new("address", Type::optional(Type::BlockchainAddress)),
                Field::new("blockchain", Type::optional(Type::enum_ref("block_chain"))),
            ],
            vec![
                Field::new("addresses_total", Type::BigInt),
                Field::new(
                    "addresses",
                    Type::datatable(
                        "AdminEscrowTokenContractAddressRow",
                        vec![
                            Field::new("pkey_id", Type::BigInt),
                            Field::new("symbol", Type::String),
                            Field::new("short_name", Type::String),
                            Field::new("description", Type::String),
                            Field::new("address", Type::BlockchainAddress),
                            Field::new("blockchain", Type::enum_ref("block_chain")),
                            Field::new("decimals", Type::BigInt),
                            Field::new("is_stablecoin", Type::Boolean),
                            Field::new("is_wrapped", Type::Boolean),
                            Field::new("price", Type::Numeric),
                        ],
                    ),
                ),
            ],
        ),
        EndpointSchema::new(
            "AdminUpdateEscrowTokenContractAddress",
            32080,
            vec![
                Field::new("pkey_id", Type::BigInt),
                Field::new("symbol", Type::optional(Type::String)),
                Field::new("short_name", Type::optional(Type::String)),
                Field::new("description", Type::optional(Type::String)),
                Field::new("is_stablecoin", Type::optional(Type::Boolean)),
                Field::new("is_wrapped", Type::optional(Type::Boolean)),
            ],
            vec![],
        ),
    ]
}
