use model::types::*;

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
            r#"
BEGIN
    RETURN QUERY
    SELECT
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
                Field::new("user_public_id", Type::BigInt),
                Field::new("name", Type::String),
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
    RETURN QUERY SELECT a.public_id                AS user_public_id,
                        a.username                 AS name,
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
                Field::new("config_placeholder_1", Type::optional(Type::BigInt)),
                Field::new("config_placeholder_2", Type::optional(Type::BigInt)),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT
        a.config_placeholder_1,
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
                Field::new("config_placeholder_1", Type::optional(Type::BigInt)),
                Field::new("config_placeholder_2", Type::optional(Type::BigInt)),
            ],
            vec![],
            r#"
BEGIN
    IF NOT EXISTS (SELECT * FROM tbl.system_config WHERE pkey_id = a_config_id) THEN
        INSERT INTO tbl.system_config (pkey_id, config_placeholder_1, config_placeholder_2)
        VALUES (a_config_id, a_config_placeholder_1, a_config_placeholder_2);
    ELSE
        UPDATE tbl.system_config SET
            config_placeholder_1 = coalesce(a_config_placeholder_1, config_placeholder_1),
            config_placeholder_2 = coalesce(a_config_placeholder_2, config_placeholder_2)
        WHERE
            pkey_id = a_config_id;
    END IF;
END
"#,
        ),
        ProceduralFunction::new(
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
            vec![
                Field::new("expert_id", Type::BigInt),
                Field::new("user_public_id", Type::BigInt),
                // FIXME: right now listening_wallet is the wallet the expert uses to login. Should be multiple wallets
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
            r#"
BEGIN
    RETURN QUERY SELECT a.pkey_id AS expert_id,
                        a.fkey_user_id   AS user_id,
                        c.public_id      AS user_public_id,
                        c.address        AS listening_wallet,
                        c.username                                                AS username,
                        c.family_name                                             AS family_name,
                        c.given_name                                               AS given_name,
                        (SELECT COUNT(*) FROM tbl.user_follow_expert WHERE fkey_expert_id = a.pkey_id AND unfollowed = FALSE) AS follower_count,
                        a.description AS description,
                        a.social_media AS social_media,
                        a.risk_score AS risk_score,
                        a.reputation_score AS reputation_score,
                        a.aum AS aum,
                        c.created_at AS joined_at,
                        a.requested_at AS request_at,
                        a.approved_at AS created_at,
                        a.pending_expert AS pending_expert,
                        a.approved_expert AS approved_expert
                 FROM tbl.expert_profile AS a
                          JOIN tbl.user AS c ON c.pkey_id = a.fkey_user_id
                 WHERE (a_expert_id NOTNULL OR a.pkey_id = a_expert_id)
                        AND (a_user_id NOTNULL OR c.pkey_id = a_user_id)
                        AND (a_user_public_id NOTNULL OR c.public_id = a_user_public_id)
                        AND (a_username NOTNULL OR c.username ILIKE a_username || '%')
                        AND (a_family_name NOTNULL OR c.family_name ILIKE a_family_name || '%')
                        AND (a_given_name NOTNULL OR c.given_name ILIKE a_given_name || '%')
                        AND (a_description NOTNULL OR a.description ILIKE a_description || '%')
                        AND (a_social_media NOTNULL OR a.social_media ILIKE a_social_media || '%')
                 ORDER BY a.pkey_id
                 OFFSET a_offset
                 LIMIT a_limit;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_admin_list_backers",
            vec![
                Field::new("offset", Type::BigInt),
                Field::new("limit", Type::BigInt),
            ],
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("username", Type::String),
                Field::new("login_wallet_address", Type::String),
                Field::new("joined_at", Type::BigInt),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT a.pkey_id AS user_id,
                        a.username AS username,
                        a.address AS login_wallet_address,
                        a.created_at AS joined_at
                 FROM tbl.user AS a
                 ORDER BY a.pkey_id
                 OFFSET a_offset
                 LIMIT a_limit;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_admin_list_strategies",
            vec![
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
            ],
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("strategy_name", Type::String),
                Field::new("expert_id", Type::BigInt),
                Field::new("expert_public_id", Type::BigInt),
                Field::new("expert_name", Type::String),
                Field::new("description", Type::optional(Type::String)),
                Field::new("created_at", Type::BigInt),
                Field::new("approved_at", Type::optional(Type::BigInt)),
                Field::new("pending_strategy", Type::Boolean),
                Field::new("approved_strategy", Type::Boolean),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT a.pkey_id AS strategy_id,
                        a.name AS strategy_name, 
                        b.pkey_id AS expert_id,
                        b.public_id AS expert_public_id,
                        b.username AS expert_name,
                        a.description AS description,
                        a.created_at AS created_at,
                        0::bigint AS approved_at,
                        FALSE AS pending_strategy,
                        TRUE AS approved_strategy
                 FROM tbl.strategy AS a
                          JOIN tbl.user AS b ON b.pkey_id = a.fkey_user_id
                 ORDER BY a.pkey_id
                 OFFSET a_offset
                 LIMIT a_limit;
END
            "#,
        ),
    ]
}
