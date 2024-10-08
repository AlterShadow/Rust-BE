pub fn check_if_user_follows_strategy() -> &'static str {
    "EXISTS(SELECT * FROM tbl.user_follow_strategy AS ufs WHERE ufs.fkey_strategy_id = s.pkey_id AND ufs.fkey_user_id = a_user_id AND ufs.unfollowed = FALSE)"
}
pub fn check_if_user_follows_expert() -> &'static str {
    "EXISTS(SELECT * FROM tbl.user_follow_expert AS ufe WHERE ufe.fkey_expert_id = e.pkey_id AND ufe.fkey_user_id = a_user_id AND unfollowed = FALSE)"
}

pub fn get_strategy_followers_count() -> &'static str {
    "(SELECT count(*) FROM tbl.user_follow_strategy AS ufs WHERE ufs.fkey_strategy_id = s.pkey_id AND ufs.unfollowed = FALSE)"
}
pub fn get_strategy_backers_count() -> &'static str {
    "(SELECT COUNT(DISTINCT h.fkey_user_id) FROM tbl.user_back_exit_strategy_ledger AS h WHERE fkey_strategy_id = s.pkey_id)"
}
pub fn strategy_row_type() -> Type {
    Type::struct_(
        "FunUserStrategyRowType",
        vec![
            Field::new("total", Type::BigInt),
            Field::new("strategy_id", Type::BigInt),
            Field::new("strategy_name", Type::String),
            Field::new("strategy_description", Type::String),
            Field::new("current_usdc", Type::BlockchainDecimal),
            Field::new("total_backed_usdc", Type::BlockchainDecimal),
            Field::new("total_exited_usdc", Type::BlockchainDecimal),
            Field::new("risk_score", Type::optional(Type::Numeric)),
            Field::new("aum", Type::optional(Type::Numeric)),
            Field::new("followers", Type::BigInt),
            Field::new("backers", Type::BigInt),
            Field::new("followed", Type::Boolean),
            Field::new("backed", Type::Boolean),
            Field::new("requested_at", Type::optional(Type::BigInt)),
            Field::new("approved", Type::Boolean),
            Field::new("approved_at", Type::optional(Type::BigInt)),
            Field::new("pending_approval", Type::Boolean),
            Field::new("created_at", Type::BigInt),
            Field::new("creator_public_id", Type::BigInt),
            Field::new("creator_id", Type::BigInt),
            Field::new("creator_username", Type::String),
            Field::new("creator_family_name", Type::optional(Type::String)),
            Field::new("creator_given_name", Type::optional(Type::String)),
            Field::new("social_media", Type::optional(Type::String)),
            Field::new("immutable_audit_rules", Type::Boolean),
            Field::new("strategy_pool_token", Type::optional(Type::String)),
            Field::new("blockchain", Type::enum_ref("block_chain")),
            Field::new(
                "strategy_pool_address",
                Type::optional(Type::BlockchainAddress),
            ),
            Field::new("number_of_tokens", Type::optional(Type::BigInt)),
            Field::new("swap_fee", Type::optional(Type::Numeric)),
            Field::new("platform_fee", Type::optional(Type::Numeric)),
            Field::new("expert_fee", Type::optional(Type::Numeric)),
        ],
    )
}
pub fn get_strategy(followed: &str) -> String {
    format!(
        "count(*) OVER() AS total,
      s.pkey_id AS strategy_id,
      s.name AS strategy_name,
      s.description AS strategy_description,
      s.current_usdc,
      s.total_backed_usdc,
      s.total_exited_usdc,
      s.risk_score as risk_score,
      s.aum as aum,
      {followers} AS followers,
      {backers} AS backers,
      {followed} as followed,
      EXISTS(SELECT 1 FROM tbl.user_back_exit_strategy_ledger AS ubesl WHERE ubesl.fkey_strategy_id = s.pkey_id AND ubesl.fkey_user_id = a_user_id) as backed,
      s.requested_at as requested_at,
      s.approved as approved,
      s.approved_at as approved_at,
      s.pending_approval as pending_approval,
      s.created_at as created_at,
      u.public_id as creator_public_id,
      u.pkey_id as creator_id,
      u.username as creator_username,
      u.family_name as creator_family_name,
      u.given_name as creator_given_name,
      s.social_media as social_media,
      s.immutable_audit_rules as immutable_audit_rules,
			-- sum all strategy pool tokens that user owns for this strategy on all chains
			(SELECT CAST(SUM(CAST(spt.balance AS NUMERIC)) AS VARCHAR)
			FROM tbl.user_strategy_balance AS spt
			JOIN tbl.strategy_pool_contract AS spc
			ON spt.fkey_strategy_pool_contract_id = spc.pkey_id
			JOIN tbl.user_strategy_wallet AS usw
			ON spt.fkey_user_strategy_wallet_id = usw.pkey_id
			WHERE spc.fkey_strategy_id = s.pkey_id AND usw.fkey_user_id = a_user_id) AS strategy_pool_token,
      s.blockchain,
      s.strategy_pool_address,
      (SELECT COUNT(*) FROM tbl.strategy_pool_contract_asset_balance AS sss
        JOIN tbl.strategy_pool_contract AS ss ON ss.pkey_id = sss.fkey_strategy_pool_contract_id
         WHERE ss.fkey_strategy_id = s.pkey_id
        ) AS number_of_tokens,
      s.swap_fee,
      (SELECT conf.platform_fee FROM tbl.system_config AS conf),
      s.expert_fee
      ",
        followers = get_strategy_followers_count(),
        backers = get_strategy_backers_count(),
    )
}

pub fn expert_row_type() -> Type {
    Type::struct_(
        "FunUserExpertRowType",
        vec![
            Field::new("total", Type::BigInt),
            Field::new("expert_id", Type::BigInt),
            Field::new("user_id", Type::BigInt),
            Field::new("user_public_id", Type::BigInt),
            Field::new("listening_wallet", Type::String),
            Field::new("username", Type::String),
            Field::new("family_name", Type::optional(Type::String)),
            Field::new("given_name", Type::optional(Type::String)),
            Field::new("follower_count", Type::BigInt),
            Field::new("backer_count", Type::BigInt),
            Field::new("strategy_count", Type::BigInt),
            Field::new("description", Type::optional(Type::String)),
            Field::new("social_media", Type::optional(Type::String)),
            Field::new("risk_score", Type::optional(Type::Numeric)),
            Field::new("reputation_score", Type::optional(Type::Numeric)),
            Field::new("aum", Type::optional(Type::Numeric)),
            Field::new("joined_at", Type::BigInt),
            Field::new("requested_at", Type::optional(Type::BigInt)),
            Field::new("approved_at", Type::optional(Type::BigInt)),
            Field::new("pending_expert", Type::Boolean),
            Field::new("approved_expert", Type::Boolean),
            Field::new("followed", Type::Boolean),
            Field::new("linked_wallet", Type::BlockchainAddress),
        ],
    )
}

pub fn get_expert(followed: &str) -> String {
    format!(
        "count(*) OVER() AS total,
        e.pkey_id                                                 AS expert_id,
        e.fkey_user_id                                            AS user_id,
        u.public_id                                               AS user_public_id,
        u.address                                                 AS listening_wallet,
        u.username                                                AS username,
        u.family_name                                             AS family_name,
        u.given_name                                              AS given_name,
        (SELECT COUNT(DISTINCT d.fkey_user_id) FROM tbl.user_follow_expert AS d WHERE d.fkey_expert_id = e.pkey_id AND unfollowed = FALSE) AS follower_count,
        (SELECT COUNT(DISTINCT d.fkey_user_id) FROM tbl.user_back_exit_strategy_ledger AS d JOIN tbl.strategy AS e ON e.pkey_id = d.fkey_strategy_id WHERE e.fkey_user_id = u.pkey_id) AS backer_count,
        (SELECT COUNT(DISTINCT d.fkey_user_id) FROM tbl.strategy AS d WHERE d.fkey_user_id = u.pkey_id) AS strategy_count,
        e.description                                             AS description,
        e.social_media                                            AS social_media,
        e.risk_score                                              AS risk_score,
        e.reputation_score                                        AS reputation_score,
        e.aum                                                     AS aum,
        u.created_at                                              AS joined_at,
        e.requested_at                                            AS requested_at,
        e.approved_at                                             AS approved_at,
        e.pending_expert                                          AS pending_expert,
        e.approved_expert                                         AS approved_expert,
        {followed}                                                AS followed,
        u.address                                                 AS linked_wallet
        "
    )
}
