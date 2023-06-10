use model::endpoint::*;
use model::types::{Field, Type};

pub fn endpoint_user_follow_strategy() -> EndpointSchema {
    EndpointSchema::new(
        "UserFollowStrategy",
        20040,
        vec![Field::new("strategy_id", Type::BigInt)],
        vec![Field::new("success", Type::Boolean)],
    )
    .with_description("User follows a strategy")
}
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
            Field::new("blockchain", Type::enum_ref("block_chain")),
        ],
    )
}
pub fn endpoint_user_list_followed_strategies() -> EndpointSchema {
    EndpointSchema::new(
        "UserListFollowedStrategies",
        20050,
        vec![],
        vec![Field::new("strategies", list_strategies_datatable())],
    )
    .with_description("User lists followed strategies")
}
pub fn endpoint_user_unfollow_strategy() -> EndpointSchema {
    EndpointSchema::new(
        "UserUnfollowStrategy",
        20060,
        vec![Field::new("strategy_id", Type::BigInt)],
        vec![Field::new("success", Type::Boolean)],
    )
}

pub fn endpoint_user_list_strategies() -> EndpointSchema {
    EndpointSchema::new(
        "UserListStrategies",
        20061,
        vec![],
        vec![Field::new("strategies", list_strategies_datatable())],
    )
    .with_description("User lists strategies")
}

pub fn endpoint_user_list_top_performing_strategies() -> EndpointSchema {
    EndpointSchema::new(
        "UserListTopPerformingStrategies",
        20063,
        vec![],
        vec![Field::new("strategies", list_strategies_datatable())],
    )
    .with_description("User lists top performing strategies")
}
pub fn endpoint_user_list_strategy_backers() -> EndpointSchema {
    EndpointSchema::new(
        "UserListStrategyBackers",
        20064,
        vec![
            Field::new("strategy_id", Type::BigInt),
            Field::new("page", Type::Int),
            Field::new("page_size", Type::Int),
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
    )
}
pub fn endpoint_user_list_strategy_followers() -> EndpointSchema {
    EndpointSchema::new(
        "UserListStrategyFollowers",
        20065,
        vec![
            Field::new("strategy_id", Type::BigInt),
            Field::new("page", Type::Int),
            Field::new("page_size", Type::Int),
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
    )
}
pub fn endpoint_user_get_strategy() -> EndpointSchema {
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
    .with_description("User gets a strategy")
}
pub fn endpoint_user_get_strategy_statistics() -> EndpointSchema {
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
    .with_description("User gets a strategy statistics")
}
pub fn endpoint_user_get_strategies_statistics() -> EndpointSchema {
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
    .with_description("User gets statistics of all strategies related to the user")
}
pub fn endpoint_user_back_strategy() -> EndpointSchema {
    EndpointSchema::new(
        "UserBackStrategy",
        20080,
        vec![
            Field::new("strategy_id", Type::BigInt),
            Field::new("quantity", Type::String),
            Field::new("blockchain", Type::enum_ref("block_chain")),
        ],
        vec![Field::new("success", Type::Boolean)],
    )
}
pub fn endpoint_user_request_refund() -> EndpointSchema {
    EndpointSchema::new(
        "UserRequestRefund",
        20081,
        vec![
            Field::new("quantity", Type::String),
            Field::new("wallet_address", Type::String),
            Field::new("blockchain", Type::enum_ref("block_chain")),
        ],
        vec![Field::new("success", Type::Boolean)],
    )
}

pub fn endpoint_user_list_backed_strategies() -> EndpointSchema {
    EndpointSchema::new(
        "UserListBackedStrategies",
        20090,
        vec![],
        vec![Field::new("strategies", list_strategies_datatable())],
    )
}

pub fn endpoint_user_list_back_strategy_history() -> EndpointSchema {
    EndpointSchema::new(
        "UserListBackStrategyHistory",
        20100,
        vec![],
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
    )
}

pub fn endpoint_user_exit_strategy() -> EndpointSchema {
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
    )
}

pub fn endpoint_user_list_exit_strategy_history() -> EndpointSchema {
    EndpointSchema::new(
        "UserListExitStrategyHistory",
        20120,
        vec![Field::new("strategy_id", Type::optional(Type::BigInt))],
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
    )
}
pub fn endpoint_user_follow_expert() -> EndpointSchema {
    EndpointSchema::new(
        "UserFollowExpert",
        20130,
        vec![Field::new("expert_id", Type::BigInt)],
        vec![Field::new("success", Type::Boolean)],
    )
    .with_description("User follows an expert")
}
fn list_experts_datatable() -> Type {
    Type::datatable(
        "ListExpertsRow",
        vec![
            Field::new("expert_id", Type::BigInt),
            Field::new("user_public_id", Type::BigInt),
            Field::new("name", Type::String),
            Field::new("linked_wallet", Type::String),
            Field::new("family_name", Type::String),
            Field::new("given_name", Type::String),
            Field::new("follower_count", Type::Int),
            Field::new("description", Type::String),
            Field::new("social_media", Type::String),
            Field::new("risk_score", Type::Numeric),
            Field::new("reputation_score", Type::Numeric),
            Field::new("aum", Type::Numeric),
            Field::new("joined_at", Type::BigInt),
            Field::new("requested_at", Type::BigInt),
        ],
    )
}
pub fn endpoint_user_list_followed_experts() -> EndpointSchema {
    EndpointSchema::new(
        "UserListFollowedExperts",
        20140,
        vec![],
        vec![Field::new("experts", list_experts_datatable())],
    )
    .with_description("User lists followed experts")
}
pub fn endpoint_user_unfollow_expert() -> EndpointSchema {
    EndpointSchema::new(
        "UserUnfollowExpert",
        20150,
        vec![Field::new("expert_id", Type::BigInt)],
        vec![Field::new("success", Type::Boolean)],
    )
    .with_description("User unfollows an expert")
}
pub fn endpoint_user_list_experts() -> EndpointSchema {
    EndpointSchema::new(
        "UserListExperts",
        20160,
        vec![],
        vec![Field::new("experts", list_experts_datatable())],
    )
    .with_description("User lists experts")
}

pub fn endpoint_user_list_top_performing_experts() -> EndpointSchema {
    EndpointSchema::new(
        "UserListTopPerformingExperts",
        20161,
        vec![],
        vec![Field::new("experts", list_experts_datatable())],
    )
    .with_description("User lists experts")
}
pub fn endpoint_user_list_featured_experts() -> EndpointSchema {
    EndpointSchema::new(
        "UserListFeaturedExperts",
        20162,
        vec![],
        vec![Field::new("experts", list_experts_datatable())],
    )
    .with_description("User lists experts")
}
pub fn endpoint_user_get_expert_profile() -> EndpointSchema {
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
    .with_description("User gets an expert profile")
}

pub fn endpoint_user_update_user_profile() -> EndpointSchema {
    EndpointSchema::new(
        "UserUpdateUserProfile",
        20172,
        vec![
            Field::new("description", Type::optional(Type::String)),
            Field::new("social_media", Type::optional(Type::String)),
        ],
        vec![],
    )
    .with_description("User update its expert profile")
}

pub fn endpoint_user_get_user_profile() -> EndpointSchema {
    EndpointSchema::new(
        "UserGetUserProfile",
        20180,
        vec![Field::new("user_id", Type::BigInt)],
        vec![
            Field::new("name", Type::String),
            Field::new("follower_count", Type::Int),
            Field::new("description", Type::String),
            Field::new("social_media", Type::String),
            Field::new("followed_experts", list_experts_datatable()),
            Field::new("followed_strategies", list_strategies_datatable()),
            Field::new("backed_strategies", list_strategies_datatable()),
        ],
    )
    .with_description("User gets an user profile")
}
pub fn endpoint_user_register_wallet() -> EndpointSchema {
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
    .with_description("User registers a wallet")
}
pub fn endpoint_user_list_registered_wallets() -> EndpointSchema {
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
    .with_description("User lists wallets")
}
pub fn endpoint_user_deregister_wallet() -> EndpointSchema {
    EndpointSchema::new(
        "UserDeregisterWallet",
        20210,
        vec![Field::new("wallet_id", Type::BigInt)],
        vec![Field::new("success", Type::Boolean)],
    )
    .with_description("User deregisters a wallet")
}
pub fn endpoint_user_apply_become_expert() -> EndpointSchema {
    EndpointSchema::new(
        "UserApplyBecomeExpert",
        20220,
        vec![],
        vec![Field::new("success", Type::Boolean)],
    )
    .with_description("User applies to become an expert")
}

pub fn endpoint_user_create_strategy() -> EndpointSchema {
    EndpointSchema::new(
        "UserCreateStrategy",
        20250,
        vec![
            Field::new("name", Type::String),
            Field::new("description", Type::String),
        ],
        vec![
            Field::new("success", Type::Boolean),
            Field::new("strategy_id", Type::BigInt),
        ],
    )
    .with_description("User makes a strategy")
}
pub fn endpoint_user_update_strategy() -> EndpointSchema {
    EndpointSchema::new(
        "UserUpdateStrategy",
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
    .with_description("User updates a strategy")
}

pub fn endpoint_user_add_strategy_watching_wallet() -> EndpointSchema {
    EndpointSchema::new(
        "UserAddStrategyWatchingWallet",
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
    )
}
pub fn endpoint_user_remove_strategy_watching_wallet() -> EndpointSchema {
    EndpointSchema::new(
        "UserRemoveStrategyWatchingWallet",
        20280,
        vec![Field::new("wallet_id", Type::BigInt)],
        vec![Field::new("success", Type::Boolean)],
    )
}
pub fn endpoint_user_list_strategy_watching_wallets() -> EndpointSchema {
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
    )
}
pub fn endpoint_user_list_wallet_activity_history() -> EndpointSchema {
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
    )
}
pub fn endpoint_user_add_strategy_initial_token_ratio() -> EndpointSchema {
    EndpointSchema::new(
        "UserAddStrategyInitialTokenRatio",
        20310,
        vec![
            Field::new("strategy_id", Type::BigInt),
            Field::new("token_name", Type::String),
            Field::new("token_address", Type::String),
            Field::new("quantity", Type::String),
        ],
        vec![
            Field::new("success", Type::Boolean),
            Field::new("token_id", Type::BigInt),
        ],
    )
}
pub fn endpoint_user_remove_strategy_initial_token_ratio() -> EndpointSchema {
    EndpointSchema::new(
        "UserRemoveStrategyInitialTokenRatio",
        20320,
        vec![
            Field::new("strategy_id", Type::BigInt),
            Field::new("token_id", Type::BigInt),
        ],
        vec![Field::new("success", Type::Boolean)],
    )
}
pub fn endpoint_user_list_strategy_initial_token_ratio() -> EndpointSchema {
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
    )
}

pub fn get_user_endpoints() -> Vec<EndpointSchema> {
    vec![
        endpoint_user_follow_strategy(),
        endpoint_user_list_followed_strategies(),
        endpoint_user_unfollow_strategy(),
        endpoint_user_list_strategies(),
        endpoint_user_list_top_performing_strategies(),
        endpoint_user_list_strategy_backers(),
        endpoint_user_list_strategy_followers(),
        endpoint_user_get_strategy(),
        endpoint_user_get_strategy_statistics(),
        endpoint_user_get_strategies_statistics(),
        endpoint_user_update_user_profile(),
        endpoint_user_back_strategy(),
        endpoint_user_exit_strategy(),
        endpoint_user_request_refund(),
        endpoint_user_list_backed_strategies(),
        endpoint_user_list_back_strategy_history(),
        // endpoint_user_exit_strategy(),
        endpoint_user_list_exit_strategy_history(),
        endpoint_user_follow_expert(),
        endpoint_user_list_followed_experts(),
        endpoint_user_unfollow_expert(),
        endpoint_user_list_experts(),
        endpoint_user_list_top_performing_experts(),
        endpoint_user_list_featured_experts(),
        endpoint_user_get_expert_profile(),
        endpoint_user_get_user_profile(),
        endpoint_user_register_wallet(),
        endpoint_user_list_registered_wallets(),
        endpoint_user_deregister_wallet(),
        endpoint_user_apply_become_expert(),
        endpoint_user_create_strategy(),
        endpoint_user_update_strategy(),
        endpoint_user_add_strategy_watching_wallet(),
        endpoint_user_remove_strategy_watching_wallet(),
        endpoint_user_list_strategy_watching_wallets(),
        endpoint_user_list_wallet_activity_history(),
        endpoint_user_add_strategy_initial_token_ratio(),
        endpoint_user_remove_strategy_initial_token_ratio(),
        endpoint_user_list_strategy_initial_token_ratio(),
    ]
}
