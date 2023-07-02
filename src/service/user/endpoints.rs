use model::endpoint::*;
use model::types::{Field, Type};
use shared_endpoints::{
    list_experts_datatable, list_strategies_datatable, strategy_row, user_deposit_ledger_entry,
};

#[path = "../shared/endpoints.rs"]
pub mod shared_endpoints;
pub fn get_user_endpoints() -> Vec<EndpointSchema> {
    vec![
        EndpointSchema::new(
            "UserFollowStrategy",
            20040,
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![Field::new("success", Type::Boolean)],
        )
        .with_description("User follows a strategy"),
        EndpointSchema::new(
            "UserListFollowedStrategies",
            20050,
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
            ],
            vec![
                Field::new("strategies_total", Type::BigInt),
                Field::new("strategies", list_strategies_datatable()),
            ],
        )
        .with_description("User lists followed strategies"),
        EndpointSchema::new(
            "UserUnfollowStrategy",
            20060,
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![Field::new("success", Type::Boolean)],
        ),
        EndpointSchema::new(
            "UserListStrategies",
            20061,
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
                Field::new("strategy_id", Type::optional(Type::BigInt)),
                Field::new("strategy_name", Type::optional(Type::String)),
                Field::new("expert_public_id", Type::optional(Type::BigInt)),
                Field::new("expert_name", Type::optional(Type::String)),
                Field::new("description", Type::optional(Type::String)),
                Field::new("blockchain", Type::optional(Type::enum_ref("block_chain"))),
                Field::new("wallet_address", Type::optional(Type::BlockchainAddress)),
            ],
            vec![
                Field::new("strategies_total", Type::BigInt),
                Field::new("strategies", list_strategies_datatable()),
            ],
        )
        .with_description("User lists strategies"),
        EndpointSchema::new(
            "UserListTopPerformingStrategies",
            20063,
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
            ],
            vec![
                Field::new("strategies_total", Type::BigInt),
                Field::new("strategies", list_strategies_datatable()),
            ],
        )
        .with_description("User lists top performing strategies"),
        EndpointSchema::new(
            "UserListStrategyBackers",
            20064,
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
            ],
            vec![
                Field::new("backers_total", Type::BigInt),
                Field::new(
                    "backers",
                    Type::datatable(
                        "ListStrategyBackersRow",
                        vec![
                            Field::new("user_id", Type::BigInt),
                            Field::new("name", Type::String),
                            Field::new("linked_wallet", Type::BlockchainAddress),
                            Field::new("backed_date", Type::BigInt),
                        ],
                    ),
                ),
            ],
        ),
        EndpointSchema::new(
            "UserListStrategyFollowers",
            20065,
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
            ],
            vec![
                Field::new("followers_total", Type::BigInt),
                Field::new(
                    "followers",
                    Type::datatable(
                        "ListStrategyFollowersRow",
                        vec![
                            Field::new("user_id", Type::BigInt),
                            Field::new("name", Type::String),
                            Field::new("linked_wallet", Type::BlockchainAddress),
                            Field::new("followed_date", Type::BigInt),
                        ],
                    ),
                ),
            ],
        ),
        EndpointSchema::new(
            "UserGetStrategy",
            20062,
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![
                Field::new("strategy", strategy_row()),
                Field::new(
                    "watching_wallets",
                    Type::datatable(
                        "WatchingWalletRow",
                        vec![
                            Field::new("watching_wallet_id", Type::BigInt),
                            Field::new("wallet_address", Type::BlockchainAddress),
                            Field::new("blockchain", Type::enum_ref("block_chain")),
                            Field::new("ratio_distribution", Type::Numeric),
                        ],
                    ),
                ),
                Field::new(
                    "aum_ledger",
                    Type::datatable(
                        "AumLedgerRow",
                        vec![
                            Field::new("aum_ledger_id", Type::BigInt),
                            Field::new("base_token", Type::String),
                            Field::new("quote_token", Type::String),
                            Field::new("blockchain", Type::enum_ref("block_chain")),
                            Field::new("dex", Type::String),
                            Field::new("action", Type::String),
                            Field::new("wallet_address", Type::BlockchainAddress),
                            Field::new("price", Type::Numeric),
                            Field::new("current_price", Type::Numeric),
                            Field::new("quantity", Type::BlockchainDecimal),
                            Field::new("yield_7d", Type::Numeric),
                            Field::new("yield_30d", Type::Numeric),
                        ],
                    ),
                ),
                Field::new(
                    "audit_rules",
                    Type::datatable(
                        "UserListStrategyAuditRulesRow",
                        vec![
                            Field::new("rule_id", Type::BigInt),
                            Field::new("rule_name", Type::String),
                            Field::new("rule_description", Type::String),
                            Field::new("created_at", Type::BigInt),
                            Field::new("enabled", Type::Boolean),
                        ],
                    ),
                ),
                Field::new("whitelisted_tokens", Type::vec(Type::String)),
            ],
        )
        .with_description("User gets a strategy"),
        EndpointSchema::new(
            "UserGetStrategyStatistics",
            20070,
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new(
                    "net_value",
                    Type::datatable(
                        "NetValuePoint",
                        vec![
                            Field::new("time", Type::BigInt),
                            Field::new("net_value", Type::Numeric),
                        ],
                    ),
                ),
                Field::new(
                    "follow_ledger",
                    Type::datatable(
                        "FollowLedgerPoint",
                        vec![
                            Field::new("time", Type::BigInt),
                            Field::new("follower_count", Type::Numeric),
                        ],
                    ),
                ),
                Field::new(
                    "back_ledger",
                    Type::datatable(
                        "BackLedgerPoint",
                        vec![
                            Field::new("time", Type::BigInt),
                            Field::new("backer_count", Type::Numeric),
                            Field::new("backer_quantity_usd", Type::Numeric),
                        ],
                    ),
                ),
            ],
        )
        .with_description("User gets a strategy statistics"),
        EndpointSchema::new(
            "UserGetStrategiesStatistics",
            20071,
            vec![],
            vec![
                Field::new("tracking_amount_usd", Type::Numeric),
                Field::new("backing_amount_usd", Type::Numeric),
                Field::new("difference_amount_usd", Type::Numeric),
                Field::new("aum_value_usd", Type::Numeric),
                Field::new("current_value_usd", Type::Numeric),
                Field::new("withdrawable_value_usd", Type::Numeric),
            ],
        )
        .with_description("User gets statistics of all strategies related to the user"),
        EndpointSchema::new(
            "UserUpdateUserProfile",
            20172,
            vec![
                Field::new("username", Type::optional(Type::String)),
                Field::new("family_name", Type::optional(Type::String)),
                Field::new("given_name", Type::optional(Type::String)),
                Field::new("description", Type::optional(Type::String)),
                Field::new("social_media", Type::optional(Type::String)),
            ],
            vec![],
        )
        .with_description("User update its expert profile"),
        EndpointSchema::new(
            "UserBackStrategy",
            20080,
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("quantity", Type::BlockchainDecimal),
                Field::new("token_id", Type::BigInt),
            ],
            vec![Field::new("success", Type::Boolean)],
        ),
        EndpointSchema::new(
            "UserExitStrategy",
            20110,
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("quantity", Type::optional(Type::BlockchainDecimal)),
                Field::new("blockchain", Type::enum_ref("block_chain")),
            ],
            vec![
                Field::new("success", Type::Boolean),
                Field::new("transaction_hash", Type::BlockchainTransactionHash),
            ],
        ),
        EndpointSchema::new(
            "UserRequestRefund",
            20081,
            vec![
                Field::new("quantity", Type::BlockchainDecimal),
                Field::new("wallet_address", Type::BlockchainAddress),
                Field::new("blockchain", Type::enum_ref("block_chain")),
            ],
            vec![Field::new("success", Type::Boolean)],
        ),
        EndpointSchema::new(
            "UserListBackedStrategies",
            20090,
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
            ],
            vec![
                Field::new("strategies_total", Type::BigInt),
                Field::new("strategies", list_strategies_datatable()),
            ],
        ),
        EndpointSchema::new(
            "UserListBackStrategyLedger",
            20100,
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
                        "BackStrategyLedgerRow",
                        vec![
                            Field::new("back_ledger_id", Type::BigInt),
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
        // endpoint_user_exit_strategy(),
        EndpointSchema::new(
            "UserListExitStrategyLedger",
            20120,
            vec![
                Field::new("strategy_id", Type::optional(Type::BigInt)),
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
            ],
            vec![
                Field::new("exit_ledger_total", Type::BigInt),
                Field::new(
                    "exit_ledger",
                    Type::datatable(
                        "ExitStrategyLedgerRow",
                        vec![
                            Field::new("exit_ledger_id", Type::BigInt),
                            Field::new("strategy_id", Type::BigInt),
                            Field::new("exit_quantity", Type::BlockchainDecimal),
                            Field::new("blockchain", Type::enum_ref("block_chain")),
                            Field::new("exit_time", Type::BigInt),
                        ],
                    ),
                ),
            ],
        ),
        EndpointSchema::new(
            "UserFollowExpert",
            20130,
            vec![Field::new("expert_id", Type::BigInt)],
            vec![Field::new("success", Type::Boolean)],
        )
        .with_description("User follows an expert"),
        EndpointSchema::new(
            "UserListFollowedExperts",
            20140,
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
            ],
            vec![
                Field::new("experts_total", Type::BigInt),
                Field::new("experts", list_experts_datatable()),
            ],
        )
        .with_description("User lists followed experts"),
        EndpointSchema::new(
            "UserUnfollowExpert",
            20150,
            vec![Field::new("expert_id", Type::BigInt)],
            vec![Field::new("success", Type::Boolean)],
        )
        .with_description("User unfollows an expert"),
        EndpointSchema::new(
            "UserListExperts",
            20160,
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
                Field::new("sort_by_followers", Type::optional(Type::Boolean)),
            ],
            vec![
                Field::new("experts_total", Type::BigInt),
                Field::new("experts", list_experts_datatable()),
            ],
        )
        .with_description("User lists experts"),
        EndpointSchema::new(
            "UserListTopPerformingExperts",
            20161,
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
            ],
            vec![
                Field::new("experts_total", Type::BigInt),
                Field::new("experts", list_experts_datatable()),
            ],
        )
        .with_description("User lists experts"),
        EndpointSchema::new(
            "UserListFeaturedExperts",
            20162,
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
            ],
            vec![
                Field::new("experts_total", Type::BigInt),
                Field::new("experts", list_experts_datatable()),
            ],
        )
        .with_description("User lists experts"),
        EndpointSchema::new(
            "UserGetExpertProfile",
            20170,
            vec![Field::new("expert_id", Type::BigInt)],
            vec![
                Field::new("expert_id", Type::BigInt),
                Field::new("name", Type::String),
                Field::new("follower_count", Type::Int),
                Field::new("description", Type::String),
                Field::new("social_media", Type::String),
                Field::new("risk_score", Type::Numeric),
                Field::new("reputation_score", Type::Numeric),
                Field::new("aum", Type::Numeric),
                Field::new("strategies_total", Type::BigInt),
                Field::new("strategies", list_strategies_datatable()),
            ],
        )
        .with_description("User gets an expert profile"),
        EndpointSchema::new(
            "UserGetUserProfile",
            20180,
            vec![],
            vec![
                Field::new("name", Type::String),
                Field::new("login_wallet", Type::String),
                Field::new("joined_at", Type::BigInt),
                Field::new("follower_count", Type::Int),
                Field::new("description", Type::String),
                Field::new("social_media", Type::String),
                Field::new("followed_experts", list_experts_datatable()),
                Field::new("followed_strategies", list_strategies_datatable()),
                Field::new("backed_strategies", list_strategies_datatable()),
            ],
        )
        .with_description("User gets an user profile"),
        EndpointSchema::new(
            "UserRegisterWallet",
            20190,
            vec![
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("wallet_address", Type::BlockchainAddress),
                Field::new("message_to_sign", Type::String),
                Field::new("message_signature", Type::String),
            ],
            vec![
                Field::new("success", Type::Boolean),
                Field::new("wallet_id", Type::BigInt),
            ],
        )
        .with_description("User registers a wallet"),
        EndpointSchema::new(
            "UserListRegisteredWallets",
            20200,
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
                Field::new("wallet_id", Type::optional(Type::BigInt)),
                Field::new("blockchain", Type::optional(Type::enum_ref("block_chain"))),
                Field::new("wallet_address", Type::optional(Type::BlockchainAddress)),
                Field::new("strategy_id", Type::optional(Type::BigInt)),
            ],
            vec![Field::new(
                "wallets",
                Type::datatable(
                    "ListWalletsRow",
                    vec![
                        Field::new("wallet_id", Type::BigInt),
                        Field::new("blockchain", Type::enum_ref("block_chain")),
                        Field::new("wallet_address", Type::BlockchainAddress),
                        Field::new("is_default", Type::Boolean),
                        Field::new("is_compatible", Type::Boolean),
                    ],
                ),
            )],
        )
        .with_description("User lists wallets"),
        EndpointSchema::new(
            "UserDeregisterWallet",
            20210,
            vec![Field::new("wallet_id", Type::BigInt)],
            vec![Field::new("success", Type::Boolean)],
        )
        .with_description("User deregisters a wallet"),
        EndpointSchema::new(
            "UserApplyBecomeExpert",
            20220,
            vec![],
            vec![
                Field::new("success", Type::Boolean),
                Field::new("expert_id", Type::BigInt),
            ],
        )
        .with_description("User applies to become an expert"),
        EndpointSchema::new(
            "ExpertCreateStrategy",
            20250,
            vec![
                Field::new("name", Type::String),
                Field::new("description", Type::String),
                Field::new("strategy_thesis_url", Type::String),
                Field::new("minimum_backing_amount_usd", Type::optional(Type::Numeric)),
                Field::new("strategy_fee", Type::Numeric),
                Field::new("expert_fee", Type::Numeric),
                Field::new("agreed_tos", Type::Boolean),
                Field::new("wallet_address", Type::BlockchainAddress),
                Field::new("wallet_blockchain", Type::enum_ref("block_chain")),
                Field::new(
                    "initial_tokens",
                    Type::datatable(
                        "UserCreateStrategyInitialTokenRow",
                        vec![
                            Field::new("token_id", Type::BigInt),
                            Field::new("quantity", Type::BlockchainDecimal),
                        ],
                    ),
                ),
                Field::new("audit_rules", Type::optional(Type::vec(Type::BigInt))),
            ],
            vec![
                Field::new("success", Type::Boolean),
                Field::new("strategy_id", Type::BigInt),
            ],
        )
        .with_description("User makes a strategy"),
        EndpointSchema::new(
            "ExpertUpdateStrategy",
            20260,
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("name", Type::optional(Type::String)),
                Field::new("description", Type::optional(Type::String)),
                Field::new("social_media", Type::optional(Type::String)),
            ],
            vec![Field::new("success", Type::Boolean)],
        )
        .with_description("Expert updates a strategy"),
        EndpointSchema::new(
            "ExpertFreezeStrategy",
            20265,
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![Field::new("success", Type::Boolean)],
        )
        .with_description("Expert freezes a strategy, by making it immutable"),
        EndpointSchema::new(
            "ExpertAddStrategyWatchingWallet",
            20270,
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("wallet_address", Type::BlockchainAddress),
                Field::new("ratio", Type::Numeric),
            ],
            vec![
                Field::new("success", Type::Boolean),
                Field::new("wallet_id", Type::BigInt),
            ],
        ),
        EndpointSchema::new(
            "ExpertRemoveStrategyWatchingWallet",
            20280,
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("wallet_id", Type::BigInt),
            ],
            vec![Field::new("success", Type::Boolean)],
        ),
        EndpointSchema::new(
            "UserListStrategyWatchingWallets",
            20290,
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![
                Field::new("wallets_total", Type::BigInt),
                Field::new(
                    "wallets",
                    Type::datatable(
                        "ListStrategyWatchingWalletsRow",
                        vec![
                            Field::new("wallet_id", Type::BigInt),
                            Field::new("blockchain", Type::enum_ref("block_chain")),
                            Field::new("wallet_address", Type::BlockchainAddress),
                            Field::new("ratio", Type::Numeric),
                        ],
                    ),
                ),
            ],
        ),
        EndpointSchema::new(
            "UserListWalletActivityLedger",
            20300,
            vec![
                Field::new("wallet_address", Type::BlockchainAddress),
                Field::new("blockchain", Type::enum_ref("block_chain")),
            ],
            vec![
                Field::new("wallet_activities_total", Type::BigInt),
                Field::new(
                    "wallet_activities",
                    Type::datatable(
                        "ListWalletActivityLedgerRow",
                        vec![
                            Field::new("record_id", Type::BigInt),
                            Field::new("wallet_address", Type::BlockchainAddress),
                            Field::new("transaction_hash", Type::BlockchainTransactionHash),
                            Field::new("dex", Type::String),
                            Field::new("blockchain", Type::enum_ref("block_chain")),
                            Field::new("contract_address", Type::BlockchainAddress),
                            Field::new("token_in_address", Type::BlockchainAddress),
                            Field::new("token_out_address", Type::BlockchainAddress),
                            Field::new("caller_address", Type::BlockchainAddress),
                            Field::new("amount_in", Type::BlockchainDecimal),
                            Field::new("amount_out", Type::BlockchainDecimal),
                            Field::new("swap_calls", Type::Object),
                            Field::new("paths", Type::Object),
                            Field::new("dex_versions", Type::Object),
                            Field::new("created_at", Type::BigInt),
                        ],
                    ),
                ),
            ],
        ),
        EndpointSchema::new(
            "ExpertAddStrategyInitialTokenRatio",
            20310,
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("token_id", Type::BigInt),
                Field::new("quantity", Type::BlockchainDecimal),
            ],
            vec![
                Field::new("success", Type::Boolean),
                Field::new("token_id", Type::BigInt),
            ],
        ),
        EndpointSchema::new(
            "ExpertRemoveStrategyInitialTokenRatio",
            20320,
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("token_id", Type::BigInt),
            ],
            vec![Field::new("success", Type::Boolean)],
        ),
        EndpointSchema::new(
            "UserListStrategyInitialTokenRatio",
            20330,
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![
                Field::new("token_ratios_total", Type::BigInt),
                Field::new(
                    "token_ratios",
                    Type::datatable(
                        "ListStrategyInitialTokenRatioRow",
                        vec![
                            Field::new("token_id", Type::BigInt),
                            Field::new("token_name", Type::String),
                            Field::new("token_address", Type::BlockchainAddress),
                            Field::new("quantity", Type::BlockchainDecimal),
                            Field::new("updated_at", Type::BigInt),
                            Field::new("created_at", Type::BigInt),
                        ],
                    ),
                ),
            ],
        ),
        EndpointSchema::new(
            "ExpertListFollowers",
            20340,
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
            ],
            vec![
                Field::new("followers_total", Type::BigInt),
                Field::new(
                    "followers",
                    Type::datatable(
                        "ExpertListFollowersRow",
                        vec![
                            Field::new("public_id", Type::BigInt),
                            Field::new("username", Type::String),
                            Field::new("family_name", Type::optional(Type::String)),
                            Field::new("given_name", Type::optional(Type::String)),
                            Field::new("followed_at", Type::BigInt),
                            Field::new("joined_at", Type::BigInt),
                        ],
                    ),
                ),
            ],
        ),
        EndpointSchema::new(
            "ExpertListBackers",
            20350,
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
            ],
            vec![
                Field::new("backers_total", Type::BigInt),
                Field::new(
                    "backers",
                    Type::datatable(
                        "ExpertListBackersRow",
                        vec![
                            Field::new("public_id", Type::BigInt),
                            Field::new("username", Type::String),
                            Field::new("family_name", Type::optional(Type::String)),
                            Field::new("given_name", Type::optional(Type::String)),
                            Field::new("backed_at", Type::BigInt),
                            Field::new("joined_at", Type::BigInt),
                        ],
                    ),
                ),
            ],
        ),
        EndpointSchema::new(
            "UserGetDepositTokens",
            20360,
            vec![],
            vec![Field::new(
                "tokens",
                Type::datatable(
                    "UserGetDepositTokensRow",
                    vec![
                        Field::new("blockchain", Type::enum_ref("block_chain")),
                        Field::new("token", Type::enum_ref("blockchain_coin")),
                        Field::new("address", Type::BlockchainAddress),
                        Field::new("short_name", Type::String),
                        Field::new("icon_url", Type::String),
                        Field::new("conversion", Type::Numeric),
                    ],
                ),
            )],
        ),
        EndpointSchema::new(
            "UserGetDepositAddresses",
            20370,
            vec![],
            vec![Field::new(
                "addresses",
                Type::datatable(
                    "UserGetDepositAddressesRow",
                    vec![
                        Field::new("blockchain", Type::enum_ref("block_chain")),
                        Field::new("address", Type::BlockchainAddress),
                        Field::new("short_name", Type::String),
                    ],
                ),
            )],
        ),
        EndpointSchema::new(
            "UserListDepositLedger",
            20380,
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
                Field::new("blockchain", Type::optional(Type::enum_ref("block_chain"))),
            ],
            vec![
                Field::new("ledger_total", Type::BigInt),
                Field::new("ledger", Type::vec(user_deposit_ledger_entry())),
            ],
        ),
        EndpointSchema::new(
            "UserSubscribeDepositLedger",
            20381,
            vec![
                Field::new("initial_data", Type::optional(Type::BigInt)),
                Field::new("blockchain", Type::optional(Type::enum_ref("block_chain"))),
                Field::new("mock_data", Type::optional(Type::Boolean)),
            ],
            vec![],
        )
        .with_stream_response_type(user_deposit_ledger_entry()),
        EndpointSchema::new("UserUnsubscribeDepositLedger", 20382, vec![], vec![]),
        EndpointSchema::new(
            "UserListStrategyWallets",
            20390,
            vec![Field::new(
                "blockchain",
                Type::optional(Type::enum_ref("block_chain")),
            )],
            vec![
                Field::new("wallets_total", Type::BigInt),
                Field::new(
                    "wallets",
                    Type::datatable(
                        "UserListStrategyWalletsRow",
                        vec![
                            Field::new("blockchain", Type::enum_ref("block_chain")),
                            Field::new("address", Type::BlockchainAddress),
                            Field::new("created_at", Type::BigInt),
                        ],
                    ),
                ),
            ],
        ),
        EndpointSchema::new(
            "UserCreateStrategyWallet",
            20391,
            vec![
                Field::new("wallet_address", Type::BlockchainAddress),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("adminship", Type::Boolean),
            ],
            vec![
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("address", Type::BlockchainAddress),
            ],
        ),
        EndpointSchema::new(
            "UserListStrategyAuditRules",
            20400,
            vec![Field::new("strategy_id", Type::optional(Type::BigInt))],
            vec![Field::new(
                "audit_rules",
                Type::datatable(
                    "UserListStrategyAuditRulesRow",
                    vec![
                        Field::new("rule_id", Type::BigInt),
                        Field::new("rule_name", Type::String),
                        Field::new("rule_description", Type::String),
                        Field::new("created_at", Type::BigInt),
                        Field::new("enabled", Type::Boolean),
                    ],
                ),
            )],
        ),
        EndpointSchema::new(
            "UserAddStrategyAuditRule",
            20410,
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("rule_id", Type::BigInt),
            ],
            vec![],
        ),
        EndpointSchema::new(
            "UserRemoveStrategyAuditRule",
            20420,
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("rule_id", Type::BigInt),
            ],
            vec![],
        ),
        EndpointSchema::new(
            "UserGetEscrowAddressForStrategy",
            20500,
            // will be expanded later
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("token_id", Type::optional(Type::BigInt)),
            ],
            vec![Field::new(
                "tokens",
                Type::datatable(
                    "UserAllowedEscrowTransferInfo",
                    vec![
                        Field::new("receiver_address", Type::BlockchainAddress),
                        Field::new("blockchain", Type::enum_ref("block_chain")),
                        Field::new("token_id", Type::BigInt),
                        Field::new("token_symbol", Type::String),
                        Field::new("token_name", Type::String),
                        Field::new("token_address", Type::BlockchainAddress),
                    ],
                ),
            )],
        ),
        EndpointSchema::new(
            "UserListDepositWithdrawBalances",
            20510,
            vec![],
            vec![Field::new(
                "balances",
                Type::datatable(
                    "UserListDepositWithdrawBalance",
                    vec![
                        Field::new("blockchain", Type::enum_ref("block_chain")),
                        Field::new("token_id", Type::BigInt),
                        Field::new("token_symbol", Type::String),
                        Field::new("token_name", Type::String),
                        Field::new("balance", Type::BlockchainDecimal),
                    ],
                ),
            )],
        ),
        EndpointSchema::new(
            "UserGetDepositWithdrawBalance",
            20511,
            vec![Field::new("token_id", Type::BigInt)],
            vec![Field::new("balance", Type::BlockchainDecimal)],
        ),
        EndpointSchema::new(
            "UserListEscrowTokenContractAddresses",
            20520,
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
                Field::new("blockchain", Type::optional(Type::enum_ref("block_chain"))),
                Field::new("is_stablecoin", Type::optional(Type::Boolean)),
            ],
            vec![
                Field::new("tokens_total", Type::BigInt),
                Field::new(
                    "tokens",
                    Type::datatable(
                        "UserListEscrowTokenContractAddressesRow",
                        vec![
                            Field::new("token_id", Type::BigInt),
                            Field::new("token_symbol", Type::String),
                            Field::new("token_name", Type::String),
                            Field::new("token_address", Type::BlockchainAddress),
                            Field::new("description", Type::String),
                            Field::new("blockchain", Type::enum_ref("block_chain")),
                            Field::new("is_stablecoin", Type::Boolean),
                        ],
                    ),
                ),
            ],
        ),
        EndpointSchema::new(
            "UserListStrategyTokenBalance",
            20530,
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
                Field::new("strategy_id", Type::optional(Type::BigInt)),
            ],
            vec![
                Field::new("tokens_total", Type::BigInt),
                Field::new(
                    "tokens",
                    Type::datatable(
                        "UserListStrategyTokenBalanceRow",
                        vec![
                            Field::new("strategy_id", Type::BigInt),
                            Field::new("strategy_name", Type::String),
                            Field::new("balance", Type::BlockchainDecimal),
                            Field::new("address", Type::BlockchainAddress),
                            Field::new("blockchain", Type::enum_ref("block_chain")),
                        ],
                    ),
                ),
            ],
        ),
        EndpointSchema::new(
            "UserGetBackStrategyReviewDetail",
            20540,
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("token_id", Type::BigInt),
                Field::new("quantity", Type::BlockchainDecimal),
            ],
            vec![
                Field::new("strategy_fee", Type::BlockchainDecimal),
                Field::new("total_amount_to_back", Type::BlockchainDecimal),
                Field::new(
                    "estimated_amount_of_strategy_tokens",
                    Type::BlockchainDecimal,
                ),
            ],
        ),
    ]
}
