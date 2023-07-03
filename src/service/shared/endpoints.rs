// TODO: rework Type::datatable to be more ergonomic

use model::types::*;

pub fn strategy_row() -> Type {
    Type::struct_(
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
            Field::new(
                "strategy_pool_address",
                Type::optional(Type::BlockchainAddress),
            ),
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
            Field::new("strategy_pool_token", Type::String),
            // strategy fee = platform fee + expert fee
            Field::new("strategy_fee", Type::Numeric),
            Field::new("platform_fee", Type::Numeric),
            Field::new("expert_fee", Type::Numeric),
            // total fee = strategy fee + swap fee(gas fee)
            Field::new("swap_fee", Type::Numeric),
            Field::new("total_fee", Type::Numeric),
        ],
    )
}
pub fn list_strategies_datatable() -> Type {
    Type::vec(strategy_row())
}
pub fn expert_row() -> Type {
    Type::struct_(
        "ListExpertsRow",
        vec![
            Field::new("expert_id", Type::BigInt),
            Field::new("user_public_id", Type::BigInt),
            Field::new("linked_wallet", Type::BlockchainAddress),
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
pub fn list_experts_datatable() -> Type {
    Type::vec(expert_row())
}
pub fn user_deposit_ledger_entry() -> Type {
    Type::struct_(
        "UserListDepositLedgerRow",
        vec![
            Field::new("blockchain", Type::enum_ref("block_chain")),
            Field::new("user_address", Type::BlockchainAddress),
            Field::new("contract_address", Type::BlockchainAddress),
            Field::new("receiver_address", Type::BlockchainAddress),
            Field::new("quantity", Type::BlockchainDecimal),
            Field::new("transaction_hash", Type::BlockchainTransactionHash),
            Field::new("created_at", Type::BigInt),
        ],
    )
}
