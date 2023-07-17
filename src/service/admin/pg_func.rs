use model::pg_func::ProceduralFunction;
use model::types::*;
include!("../shared/pg_func.rs");

pub fn get_admin_pg_func() -> Vec<ProceduralFunction> {
    vec![
        ProceduralFunction::new(
            "fun_admin_list_users",
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
                Field::new("total", Type::BigInt),
                Field::new("user_id", Type::BigInt),
                Field::new("public_user_id", Type::BigInt),
                Field::new("username", Type::optional(Type::String)),
                Field::new("address", Type::BlockchainAddress),
                Field::new("last_ip", Type::Inet),
                Field::new("last_login_at", Type::BigInt),
                Field::new("login_count", Type::Int),
                Field::new("role", Type::enum_ref("role")),
                Field::new("email", Type::optional(Type::String)),
                Field::new("updated_at", Type::BigInt),
                Field::new("created_at", Type::BigInt),
            ],
            r#"
BEGIN
    RETURN QUERY
    SELECT
        count(*) OVER() AS total,
        u.pkey_id,
        u.public_id,
        u.username,
        u.address,
        u.last_ip,
        u.last_login_at,
        u.login_count,
        u.role,
        u.email,
        u.updated_at,
        u.created_at
    FROM
        tbl.user u
    WHERE
        (a_user_id ISNULL OR u.pkey_id = a_user_id) AND
        (a_address ISNULL OR u.address ILIKE a_address || '%') AND
        (a_username ISNULL OR u.username ILIKE a_username || '%') AND
        (a_email ISNULL OR u.email ILIKE a_email || '%') AND
        (a_role ISNULL OR u.role = a_role)
    ORDER BY
        u.pkey_id
    LIMIT
        a_limit
    OFFSET
        a_offset;
END;

        "#,
        ),
        ProceduralFunction::new(
            "fun_admin_set_user_role",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("role", Type::enum_ref("role")),
            ],
            vec![],
            r#"
BEGIN
    UPDATE tbl.user SET role = a_role WHERE pkey_id = a_user_id;
END;
        "#,
        ),
        ProceduralFunction::new(
            "fun_admin_set_block_user",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("blocked", Type::Boolean),
            ],
            vec![],
            r#"
BEGIN
    UPDATE tbl.user SET is_blocked = a_blocked WHERE pkey_id = a_user_id;
END;
        "#,
        ),
        ProceduralFunction::new(
            "fun_admin_approve_user_become_expert",
            vec![Field::new("user_public_id", Type::BigInt)],
            vec![Field::new("success", Type::Boolean)],
            r#"

DECLARE
    _user_id bigint;
BEGIN
    SELECT pkey_id INTO _user_id FROM tbl.user WHERE public_id = a_user_public_id;
    UPDATE tbl.expert_profile 
    SET pending_expert = FALSE,
        approved_expert = TRUE,
        approved_at = EXTRACT(EPOCH FROM NOW()),
        updated_at = EXTRACT(EPOCH FROM NOW())
    WHERE fkey_user_id = _user_id;
    UPDATE tbl.user SET role = 'expert' WHERE role = 'user' AND pkey_id = _user_id;
    RETURN QUERY SELECT TRUE;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_admin_reject_user_become_expert",
            vec![Field::new("user_public_id", Type::BigInt)],
            vec![Field::new("success", Type::Boolean)],
            r#"

DECLARE
    _user_id bigint;
BEGIN
    SELECT pkey_id INTO _user_id FROM tbl.user WHERE public_id = a_user_public_id;
    UPDATE tbl.expert_profile SET pending_expert = FALSE, approved_expert = FALSE WHERE fkey_user_id = _user_id;
    RETURN QUERY SELECT TRUE;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_admin_list_pending_user_expert_applications",
            vec![
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
            ],
            vec![
                Field::new("total", Type::BigInt),
                Field::new("user_public_id", Type::BigInt),
                Field::new("name", Type::String),
                Field::new("linked_wallet", Type::BlockchainAddress),
                Field::new("follower_count", Type::BigInt),
                Field::new("description", Type::optional(Type::String)),
                Field::new("social_media", Type::optional(Type::String)),
                Field::new("risk_score", Type::optional(Type::Numeric)),
                Field::new("reputation_score", Type::optional(Type::Numeric)),
                Field::new("aum", Type::optional(Type::Numeric)),
                Field::new("pending_expert", Type::Boolean),
                Field::new("approved_expert", Type::Boolean),
                Field::new("joined_at", Type::optional(Type::BigInt)),
                Field::new("requested_at", Type::optional(Type::BigInt)),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT COUNT(*) OVER() AS total,
                        a.public_id                AS user_public_id,
                        a.username                 AS name,
                        a.address                  AS linked_wallet,
                        (SELECT COUNT(*)
                         FROM tbl.user_follow_expert
                         WHERE fkey_expert_id = a.pkey_id
                           AND unfollowed = FALSE) AS follower_count,
                        b.description                AS description,
                        b.social_media                AS social_media,
                        b.risk_score      AS risk_score,
                        b.reputation_score      AS reputation_score,
                        b.aum      AS aum,
                        b.pending_expert            AS pending_expert,
                        b.approved_expert           AS approved_expert,
                        a.created_at                AS joined_at,
                        b.requested_at                AS request_at
                 FROM tbl."user" AS a
                    JOIN tbl.expert_profile AS b ON b.fkey_user_id = a.pkey_id
                 WHERE b.pending_expert = TRUE
                 ORDER BY b.pkey_id
                LIMIT a_limit
                OFFSET a_offset;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_admin_get_system_config",
            vec![Field::new("config_id", Type::BigInt)],
            vec![
                Field::new("platform_fee", Type::optional(Type::Numeric)),
                Field::new("config_placeholder_2", Type::optional(Type::BigInt)),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT
        a.platform_fee,
        a.config_placeholder_2
    FROM
        tbl.system_config a
    WHERE
        a.pkey_id = a_config_id;
END            
"#,
        ),
        ProceduralFunction::new(
            "fun_admin_update_system_config",
            vec![
                Field::new("config_id", Type::BigInt),
                Field::new("platform_fee", Type::optional(Type::Numeric)),
                Field::new("config_placeholder_2", Type::optional(Type::BigInt)),
            ],
            vec![],
            r#"
BEGIN
    IF NOT EXISTS (SELECT * FROM tbl.system_config WHERE pkey_id = a_config_id) THEN
        INSERT INTO tbl.system_config (pkey_id, platform_fee, config_placeholder_2)
        VALUES (a_config_id, a_platform_fee, a_config_placeholder_2);
    ELSE
        UPDATE tbl.system_config SET
            platform_fee = coalesce(a_platform_fee, platform_fee),
            config_placeholder_2 = coalesce(a_config_placeholder_2, config_placeholder_2)
        WHERE
            pkey_id = a_config_id;
    END IF;
END
"#,
        ),
        ProceduralFunction::new_with_row_type(
            "fun_admin_list_experts",
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
            expert_row_type(),
            format!(
                r#"
BEGIN
    RETURN QUERY SELECT {expert}
                 FROM tbl.expert_profile AS e
                   JOIN tbl.user AS u ON u.pkey_id = e.fkey_user_id
                 WHERE (a_expert_id ISNULL OR e.pkey_id = a_expert_id)
                        AND (a_user_id ISNULL OR u.pkey_id = a_user_id)
                        AND (a_user_public_id ISNULL OR u.public_id = a_user_public_id)
                        AND (a_username ISNULL OR u.username ILIKE a_username || '%')
                        AND (a_family_name ISNULL OR u.family_name ILIKE a_family_name || '%')
                        AND (a_given_name ISNULL OR u.given_name ILIKE a_given_name || '%')
                        AND (a_description ISNULL OR e.description ILIKE a_description || '%')
                        AND (a_social_media ISNULL OR e.social_media ILIKE a_social_media || '%')
                        AND e.approved_expert = TRUE
                 ORDER BY e.pkey_id
                 OFFSET a_offset
                 LIMIT a_limit;
END
"#,
                expert = get_expert(check_if_user_follows_expert())
            ),
        ),
        ProceduralFunction::new(
            "fun_admin_list_backers",
            vec![
                Field::new("offset", Type::BigInt),
                Field::new("limit", Type::BigInt),
                Field::new("user_id", Type::optional(Type::BigInt)),
                Field::new("user_public_id", Type::optional(Type::BigInt)),
                Field::new("username", Type::optional(Type::String)),
                Field::new("family_name", Type::optional(Type::String)),
                Field::new("given_name", Type::optional(Type::String)),
            ],
            vec![
                Field::new("total", Type::BigInt),
                Field::new("user_id", Type::BigInt),
                Field::new("user_public_id", Type::BigInt),
                Field::new("username", Type::String),
                Field::new("login_wallet_address", Type::BlockchainAddress),
                Field::new("joined_at", Type::BigInt),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT COUNT(*) OVER() AS total,
                        a.pkey_id AS user_id,
                        a.public_id AS user_public_id,
                        a.username AS username,
                        a.address AS login_wallet_address,
                        a.created_at AS joined_at
                 FROM tbl.user AS a
                 JOIN tbl.user_back_exit_strategy_ledger AS b ON b.fkey_user_id = a.pkey_id
                WHERE (a_user_id ISNULL OR a.pkey_id = a_user_id)
                        AND (a_user_public_id ISNULL OR a.public_id = a_user_public_id)
                        AND (a_username ISNULL OR a.username ILIKE a_username || '%')
                        AND (a_family_name ISNULL OR a.family_name ILIKE a_family_name || '%')
                        AND (a_given_name ISNULL OR a.given_name ILIKE a_given_name || '%')
                 ORDER BY a.pkey_id
                 OFFSET a_offset
                 LIMIT a_limit;
END
            "#,
        ),
        ProceduralFunction::new_with_row_type(
            "fun_admin_list_strategies",
            vec![
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
                Field::new("strategy_id", Type::optional(Type::BigInt)),
                Field::new("strategy_name", Type::optional(Type::String)),
                Field::new("expert_public_id", Type::optional(Type::BigInt)),
                Field::new("expert_name", Type::optional(Type::String)),
                Field::new("description", Type::optional(Type::String)),
                Field::new("approved", Type::optional(Type::Boolean)),
                Field::new("pending_approval", Type::optional(Type::Boolean)),
            ],
            strategy_row_type(),
            format!(
                r#"
DECLARE
    a_user_id bigint = NULL;
BEGIN
    RETURN QUERY SELECT {strategy}
                 FROM tbl.strategy AS s
                      JOIN tbl.user AS u ON u.pkey_id = s.fkey_user_id
                          
                WHERE (a_strategy_id ISNULL OR s.pkey_id = a_strategy_id)
                    AND (a_strategy_name ISNULL OR s.name ILIKE a_strategy_name || '%')
                    AND (a_expert_public_id ISNULL OR u.public_id = a_expert_public_id)
                    AND (a_expert_name ISNULL OR u.username ILIKE a_expert_name || '%')
                    AND (a_description ISNULL OR s.description ILIKE a_description || '%')
                    AND (a_approved ISNULL OR s.approved = a_approved)
                    AND (a_pending_approval ISNULL OR s.pending_approval = a_pending_approval)
                 ORDER BY s.pkey_id
                 OFFSET a_offset
                 LIMIT a_limit;
END
            "#,
                strategy = get_strategy(check_if_user_follows_strategy()),
            ),
        ),
        ProceduralFunction::new(
            "fun_admin_approve_strategy",
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![],
            r#"
BEGIN
    UPDATE tbl.strategy
       SET approved = TRUE,
           pending_approval = FALSE,
           approved_at = EXTRACT(EPOCH FROM NOW())::bigint,
           updated_at = EXTRACT(EPOCH FROM NOW())::bigint
     WHERE pkey_id = a_strategy_id;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_admin_reject_strategy",
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![],
            r#"
BEGIN
    UPDATE tbl.strategy
       SET approved = FALSE,
           pending_approval = FALSE,
           approved_at = NULL,
           updated_at = EXTRACT(EPOCH FROM NOW())::bigint
     WHERE pkey_id = a_strategy_id;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_admin_add_audit_rule",
            vec![
                Field::new("rule_id", Type::BigInt),
                Field::new("name", Type::String),
                Field::new("description", Type::String),
            ],
            vec![],
            r#"
BEGIN
    INSERT INTO tbl.audit_rule (pkey_id, name, description)
         VALUES (a_rule_id, a_name, a_description);
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_admin_add_escrow_token_contract_address",
            vec![
                Field::new("pkey_id", Type::BigInt),
                Field::new("symbol", Type::String),
                Field::new("short_name", Type::String),
                Field::new("description", Type::String),
                Field::new("address", Type::BlockchainAddress),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("decimals", Type::Int),
                Field::new("is_stablecoin", Type::Boolean),
            ],
            vec![],
            r#"
BEGIN
    IF EXISTS (SELECT 1 FROM tbl.escrow_token_contract_address WHERE blockchain = a_blockchain AND symbol = a_symbol) THEN
        UPDATE tbl.escrow_token_contract_address 
        SET short_name = a_short_name,
            description = a_description,
            address = a_address,
            is_stablecoin = a_is_stablecoin,
            decimals = a_decimals
        WHERE blockchain = a_blockchain AND symbol = a_symbol;
    ELSE
        INSERT INTO tbl.escrow_token_contract_address (pkey_id, symbol, short_name, description, address, blockchain, is_stablecoin, decimals)
             VALUES (a_pkey_id, a_symbol, a_short_name, a_description, a_address, a_blockchain, a_is_stablecoin, a_decimals);
    END IF;

END
            "#,
        ),
        ProceduralFunction::new(
            "fun_admin_list_escrow_token_contract_address",
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
                Field::new("blockchain", Type::optional(Type::enum_ref("block_chain"))),
            ],
            vec![
                Field::new("pkey_id", Type::BigInt),
                Field::new("symbol", Type::String),
                Field::new("short_name", Type::String),
                Field::new("description", Type::String),
                Field::new("address", Type::BlockchainAddress),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("decimals", Type::Int),
                Field::new("is_stablecoin", Type::Boolean),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT
				etca.pkey_id,
				etca.symbol,
				etca.short_name,
				etca.description,
				etca.address,
				etca.blockchain,
				etca.decimals,
				etca.is_stablecoin
			FROM tbl.escrow_token_contract_address AS etca
			WHERE (a_blockchain ISNULL OR etca.blockchain = a_blockchain)
			ORDER BY etca.pkey_id
			OFFSET a_offset
			LIMIT a_limit;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_admin_add_escrow_contract_address",
            vec![
                Field::new("pkey_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("address", Type::BlockchainAddress),
            ],
            vec![],
            r#"
BEGIN
    INSERT INTO tbl.escrow_contract_address (pkey_id, blockchain, address)
         VALUES (a_pkey_id, a_blockchain, a_address);
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_admin_list_escrow_contract_address",
            vec![
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
                Field::new("blockchain", Type::optional(Type::enum_ref("block_chain"))),
            ],
            vec![
                Field::new("pkey_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("address", Type::BlockchainAddress),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT pkey_id, blockchain, address
                 FROM tbl.escrow_contract_address
                WHERE (a_blockchain ISNULL OR blockchain = a_blockchain)
                 ORDER BY pkey_id
                 OFFSET a_offset
                 LIMIT a_limit;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_admin_update_escrow_contract_address",
            vec![
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("address", Type::BlockchainAddress),
            ],
            vec![],
            r#"
BEGIN
    UPDATE tbl.escrow_contract_address
         SET address = a_address
         WHERE blockchain = a_blockchain;
END
            "#,
        ),
    ]
}
