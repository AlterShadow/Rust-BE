// TODO: rework Type::datatable to be more ergonomic
pub fn list_strategies_datatable() -> Type {
    Type::datatable(
        "ListStrategiesRow",
        vec![
            Field::new("strategy_id", Type::BigInt),
            Field::new("strategy_name", Type::String),
            Field::new("strategy_description", Type::String),
            Field::new("net_value", Type::Numeric),
            Field::new("followers", Type::Int),
            Field::new("backers", Type::Int),
            Field::new("aum", Type::Numeric),
            Field::new("followed", Type::Boolean),
            Field::new("swap_price", Type::Numeric),
            Field::new("price_change", Type::Numeric),
            Field::new("wallet_address", Type::String),
            Field::new("approved", Type::Boolean),
            Field::new("approved_at", Type::optional(Type::BigInt)),
            Field::new("blockchain", Type::enum_ref("block_chain")),
            Field::new("requested_at", Type::optional(Type::BigInt)),
            Field::new("created_at", Type::BigInt),
            Field::new("expert_public_id", Type::BigInt),
            Field::new("expert_username", Type::String),
            Field::new("expert_family_name", Type::String),
            Field::new("expert_given_name", Type::String),
            Field::new("reputation", Type::Int),
            Field::new("risk_score", Type::Numeric),
        ],
    )
}
pub fn list_experts_datatable() -> Type {
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
            Field::new("backer_count", Type::BigInt),
            Field::new("description", Type::String),
            Field::new("social_media", Type::String),
            Field::new("risk_score", Type::Numeric),
            Field::new("reputation_score", Type::Numeric),
            Field::new("consistent_score", Type::Numeric),
            Field::new("aum", Type::Numeric),
            Field::new("joined_at", Type::BigInt),
            Field::new("requested_at", Type::BigInt),
            Field::new("approved_at", Type::optional(Type::BigInt)),
            Field::new("pending_expert", Type::Boolean),
            Field::new("approved_expert", Type::Boolean),
            Field::new("followed", Type::Boolean),
        ],
    )
}
