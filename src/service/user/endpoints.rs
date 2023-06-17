use model::endpoint::*;
use model::types::{Field, Type};

fn list_strategies_datatable() -> Type {
    Type::datatable(
        "ListStrategiesRow",
        vec![
            Field::new("strategy_id", Type::BigInt),
            Field::new("strategy_name", Type::String),
            Field::new("strategy_description", Type::String),
            Field::new("net_value", Type::Numeric),
            Field::new("followers", Type::Int),
            Field::new("backers", Type::Int),
            Field::new("risk_score", Type::Numeric),
            Field::new("aum", Type::Numeric),
            Field::new("followed", Type::Boolean),
            Field::new("swap_price", Type::Numeric),
            Field::new("price_change", Type::Numeric),
            Field::new("wallet_address", Type::String),
            Field::new("approved", Type::Boolean),
            Field::new("approved_at", Type::optional(Type::BigInt)),
            Field::new("blockchain", Type::enum_ref("block_chain")),
        ],
    )
}

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
            vec![Field::new("strategies", list_strategies_datatable())],
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
            ],
            vec![Field::new("strategies", list_strategies_datatable())],
        )
        .with_description("User lists strategies"),
        EndpointSchema::new(
            "UserListTopPerformingStrategies",
            20063,
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
            ],
            vec![Field::new("strategies", list_strategies_datatable())],
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
            vec![Field::new(
                "backers",
                Type::datatable(
                    "ListStrategyBackersRow",
                    vec![
                        Field::new("user_id", Type::BigInt),
                        Field::new("name", Type::String),
                        Field::new("linked_wallet", Type::String),
                        Field::new("backed_date", Type::BigInt),
                    ],
                ),
            )],
        ),
        EndpointSchema::new(
            "UserListStrategyFollowers",
            20065,
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
            ],
            vec![Field::new(
                "followers",
                Type::datatable(
                    "ListStrategyFollowersRow",
                    vec![
                        Field::new("user_id", Type::BigInt),
                        Field::new("name", Type::String),
                        Field::new("linked_wallet", Type::String),
                        Field::new("followed_date", Type::BigInt),
                    ],
                ),
            )],
        ),
        EndpointSchema::new(
            "UserGetStrategy",
            20062,
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("strategy_name", Type::String),
                Field::new("strategy_description", Type::String),
                Field::new("creator_user_id", Type::BigInt),
                Field::new("social_media", Type::String),
                Field::new("historical_return", Type::Numeric),
                Field::new("inception_time", Type::BigInt),
                Field::new("total_amount", Type::Numeric),
                Field::new("token_allocation", Type::BigInt),
                Field::new("reputation", Type::Int),
                Field::new("risk_score", Type::Numeric),
                Field::new("aum", Type::Numeric),
                Field::new("net_value", Type::Numeric),
                Field::new("followers", Type::Int),
                Field::new("approved", Type::Boolean),
                Field::new("approved_at", Type::optional(Type::BigInt)),
                Field::new("backers", Type::Int),
                Field::new(
                    "watching_wallets",
                    Type::datatable(
                        "WatchingWalletRow",
                        vec![
                            Field::new("watching_wallet_id", Type::BigInt),
                            Field::new("wallet_address", Type::String),
                            Field::new("blockchain", Type::enum_ref("block_chain")),
                            Field::new("dex", Type::String),
                            Field::new("ratio_distribution", Type::Numeric),
                        ],
                    ),
                ),
                Field::new(
                    "aum_history",
                    Type::datatable(
                        "AumHistoryRow",
                        vec![
                            Field::new("aum_history_id", Type::BigInt),
                            Field::new("base_token", Type::String),
                            Field::new("quote_token", Type::String),
                            Field::new("blockchain", Type::enum_ref("block_chain")),
                            Field::new("dex", Type::String),
                            Field::new("action", Type::String),
                            Field::new("wallet_address", Type::String),
                            Field::new("price", Type::Numeric),
                            Field::new("current_price", Type::Numeric),
                            Field::new("quantity", Type::Numeric),
                            Field::new("yield_7d", Type::Numeric),
                            Field::new("yield_30d", Type::Numeric),
                        ],
                    ),
                ),
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
                    "follow_history",
                    Type::datatable(
                        "FollowHistoryPoint",
                        vec![
                            Field::new("time", Type::BigInt),
                            Field::new("follower_count", Type::Numeric),
                        ],
                    ),
                ),
                Field::new(
                    "back_history",
                    Type::datatable(
                        "BackHistoryPoint",
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
                Field::new("quantity", Type::String),
                Field::new("blockchain", Type::enum_ref("block_chain")),
            ],
            vec![Field::new("success", Type::Boolean)],
        ),
        EndpointSchema::new(
            "UserExitStrategy",
            20110,
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("quantity", Type::String),
            ],
            vec![
                Field::new("success", Type::Boolean),
                Field::new("transaction_hash", Type::String),
            ],
        ),
        EndpointSchema::new(
            "UserRequestRefund",
            20081,
            vec![
                Field::new("quantity", Type::String),
                Field::new("wallet_address", Type::String),
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
            vec![Field::new("strategies", list_strategies_datatable())],
        ),
        EndpointSchema::new(
            "UserListBackStrategyHistory",
            20100,
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
            ],
            vec![Field::new(
                "back_history",
                Type::datatable(
                    "BackStrategyHistoryRow",
                    vec![
                        Field::new("back_history_id", Type::BigInt),
                        Field::new("strategy_id", Type::BigInt),
                        Field::new("quantity", Type::String),
                        Field::new("blockchain", Type::enum_ref("block_chain")),
                        Field::new("dex", Type::String),
                        Field::new("transaction_hash", Type::String),
                        Field::new("time", Type::BigInt),
                    ],
                ),
            )],
        ),
        // endpoint_user_exit_strategy(),
        EndpointSchema::new(
            "UserListExitStrategyHistory",
            20120,
            vec![
                Field::new("strategy_id", Type::optional(Type::BigInt)),
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
            ],
            vec![Field::new(
                "exit_history",
                Type::datatable(
                    "ExitStrategyHistoryRow",
                    vec![
                        Field::new("exit_history_id", Type::BigInt),
                        Field::new("strategy_id", Type::BigInt),
                        Field::new("exit_quantity", Type::String),
                        Field::new("purchase_wallet_address", Type::String),
                        Field::new("blockchain", Type::enum_ref("block_chain")),
                        Field::new("dex", Type::String),
                        Field::new("back_time", Type::BigInt),
                        Field::new("exit_time", Type::BigInt),
                    ],
                ),
            )],
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
            vec![Field::new(
                "experts",
                Type::datatable(
                    "UserListFollowedExpertsRow",
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
                        Field::new("followed", Type::Boolean),
                    ],
                ),
            )],
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
            ],
            vec![Field::new(
                "experts",
                Type::datatable(
                    "UserListExpertsRow",
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
                        Field::new("followed", Type::Boolean),
                    ],
                ),
            )],
        )
        .with_description("User lists experts"),
        EndpointSchema::new(
            "UserListTopPerformingExperts",
            20161,
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
            ],
            vec![Field::new(
                "experts",
                Type::datatable(
                    "UserListExpertsRow",
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
                        Field::new("followed", Type::Boolean),
                    ],
                ),
            )],
        )
        .with_description("User lists experts"),
        EndpointSchema::new(
            "UserListFeaturedExperts",
            20162,
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
            ],
            vec![Field::new(
                "experts",
                Type::datatable(
                    "ListFeaturedExpertsRow",
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
                        Field::new("backer_count", Type::BigInt),
                        Field::new("consistent_score", Type::Numeric),
                        Field::new("followed", Type::Boolean),
                    ],
                ),
            )],
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
                Field::new(
                    "followed_experts",
                    Type::datatable(
                        "ListExpertsRow",
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
                ),
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
                Field::new("wallet_address", Type::String),
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
            vec![],
            vec![Field::new(
                "wallets",
                Type::datatable(
                    "ListWalletsRow",
                    vec![
                        Field::new("wallet_id", Type::BigInt),
                        Field::new("blockchain", Type::enum_ref("block_chain")),
                        Field::new("wallet_address", Type::String),
                        Field::new("is_default", Type::Boolean),
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
                Field::new("minimum_backing_amount_usd", Type::Numeric),
                Field::new("strategy_fee", Type::Numeric),
                Field::new("expert_fee", Type::Numeric),
                Field::new("agreed_tos", Type::Boolean),
                Field::new("wallet_address", Type::String),
                // Field::new(
                //     "linked_wallets",
                //     Type::datatable(
                //         "LinkedWallet",
                //         vec![
                //             Field::new("wallet_address", Type::String),
                //             TODO: verify ownership of the wallet by requiring signature
                // ],
                // ),
                // ),
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
                Field::new("risk_score", Type::optional(Type::Numeric)),
                Field::new("reputation_score", Type::optional(Type::Numeric)),
                Field::new("aum", Type::optional(Type::Numeric)),
            ],
            vec![Field::new("success", Type::Boolean)],
        )
        .with_description("User updates a strategy"),
        EndpointSchema::new(
            "ExpertAddStrategyWatchingWallet",
            20270,
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("wallet_address", Type::String),
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
            vec![Field::new("wallet_id", Type::BigInt)],
            vec![Field::new("success", Type::Boolean)],
        ),
        EndpointSchema::new(
            "UserListStrategyWatchingWallets",
            20290,
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![Field::new(
                "wallets",
                Type::datatable(
                    "ListStrategyWatchingWalletsRow",
                    vec![
                        Field::new("wallet_id", Type::BigInt),
                        Field::new("blockchain", Type::enum_ref("block_chain")),
                        Field::new("wallet_address", Type::String),
                        Field::new("ratio", Type::Numeric),
                    ],
                ),
            )],
        ),
        EndpointSchema::new(
            "UserListWalletActivityHistory",
            20300,
            vec![
                Field::new("wallet_address", Type::String),
                Field::new("blockchain", Type::enum_ref("block_chain")),
            ],
            vec![Field::new(
                "wallet_activities",
                Type::datatable(
                    "ListWalletActivityHistoryRow",
                    vec![
                        Field::new("record_id", Type::BigInt),
                        Field::new("wallet_address", Type::String),
                        Field::new("transaction_hash", Type::String),
                        Field::new("dex", Type::String),
                        Field::new("blockchain", Type::enum_ref("block_chain")),
                        Field::new("contract_address", Type::String),
                        Field::new("token_in_address", Type::String),
                        Field::new("token_out_address", Type::String),
                        Field::new("caller_address", Type::String),
                        Field::new("amount_in", Type::String),
                        Field::new("amount_out", Type::String),
                        Field::new("swap_calls", Type::Object),
                        Field::new("paths", Type::Object),
                        Field::new("dex_versions", Type::Object),
                        Field::new("created_at", Type::BigInt),
                    ],
                ),
            )],
        ),
        EndpointSchema::new(
            "ExpertAddStrategyInitialTokenRatio",
            20310,
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("token_name", Type::String),
                Field::new("token_address", Type::String),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("quantity", Type::String),
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
            vec![Field::new(
                "token_ratios",
                Type::datatable(
                    "ListStrategyInitialTokenRatioRow",
                    vec![
                        Field::new("token_id", Type::BigInt),
                        Field::new("token_name", Type::String),
                        Field::new("token_address", Type::String),
                        Field::new("quantity", Type::String),
                        Field::new("updated_at", Type::BigInt),
                        Field::new("created_at", Type::BigInt),
                    ],
                ),
            )],
        ),
        EndpointSchema::new(
            "ExpertListFollowers",
            20340,
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
            ],
            vec![Field::new(
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
            )],
        ),
        EndpointSchema::new(
            "ExpertListBackers",
            20350,
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
            ],
            vec![Field::new(
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
            )],
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
                        Field::new("address", Type::String),
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
                        Field::new("address", Type::String),
                        Field::new("short_name", Type::String),
                    ],
                ),
            )],
        ),
        EndpointSchema::new(
            "UserListDepositHistory",
            20380,
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
            ],
            vec![Field::new(
                "history",
                Type::datatable(
                    "UserListDepositHistoryRow",
                    vec![
                        Field::new("blockchain", Type::enum_ref("block_chain")),
                        Field::new("user_address", Type::String),
                        Field::new("contract_address", Type::String),
                        Field::new("receiver_address", Type::String),
                        Field::new("quantity", Type::String),
                        Field::new("transaction_hash", Type::String),
                        Field::new("created_at", Type::BigInt),
                    ],
                ),
            )],
        ),
        EndpointSchema::new(
            "UserListStrategyWallets",
            20390,
            vec![Field::new(
                "blockchain",
                Type::optional(Type::enum_ref("block_chain")),
            )],
            vec![Field::new(
                "wallets",
                Type::datatable(
                    "UserListStrategyWalletsRow",
                    vec![
                        Field::new("blockchain", Type::enum_ref("block_chain")),
                        Field::new("address", Type::String),
                        Field::new("created_at", Type::BigInt),
                    ],
                ),
            )],
        ),
    ]
}
