use model::types::*;
fn get_first_linked_wallet() -> &'static str {
    "(SELECT distinct on(1) w.pkey_id FROM tbl.strategy_watching_wallet AS w WHERE w.fkey_strategy_id = s.pkey_id ORDER BY w.pkey_id)"
}
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
                Field::new("linked_wallet", Type::String),
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
                Field::new("user_id", Type::BigInt),
                Field::new("user_public_id", Type::BigInt),
                Field::new("linked_wallet", Type::String),
                Field::new("name", Type::String),
                Field::new("family_name", Type::optional(Type::String)),
                Field::new("given_name", Type::optional(Type::String)),
                Field::new("follower_count", Type::optional(Type::BigInt)),
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
                 WHERE (a_expert_id ISNULL OR a.pkey_id = a_expert_id)
                        AND (a_user_id ISNULL OR c.pkey_id = a_user_id)
                        AND (a_user_public_id ISNULL OR c.public_id = a_user_public_id)
                        AND (a_username ISNULL OR c.username ILIKE a_username || '%')
                        AND (a_family_name ISNULL OR c.family_name ILIKE a_family_name || '%')
                        AND (a_given_name ISNULL OR c.given_name ILIKE a_given_name || '%')
                        AND (a_description ISNULL OR a.description ILIKE a_description || '%')
                        AND (a_social_media ISNULL OR a.social_media ILIKE a_social_media || '%')
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
                Field::new("user_id", Type::optional(Type::BigInt)),
                Field::new("user_public_id", Type::optional(Type::BigInt)),
                Field::new("username", Type::optional(Type::String)),
                Field::new("family_name", Type::optional(Type::String)),
                Field::new("given_name", Type::optional(Type::String)),
            ],
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("user_public_id", Type::BigInt),
                Field::new("username", Type::String),
                Field::new("login_wallet_address", Type::String),
                Field::new("joined_at", Type::BigInt),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT a.pkey_id AS user_id,
                        a.public_id AS user_public_id,
                        a.username AS username,
                        a.address AS login_wallet_address,
                        a.created_at AS joined_at
                 FROM tbl.user AS a
                 JOIN tbl.user_back_strategy_history AS b ON b.fkey_user_id = a.pkey_id
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
        ProceduralFunction::new(
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
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("strategy_name", Type::String),
                Field::new("expert_id", Type::BigInt),
                Field::new("expert_public_id", Type::BigInt),
                Field::new("expert_name", Type::String),
                Field::new("description", Type::optional(Type::String)),
                Field::new("created_at", Type::BigInt),
                Field::new("pending_approval", Type::Boolean),
                Field::new("approved", Type::Boolean),
                Field::new("approved_at", Type::optional(Type::BigInt)),
                Field::new("linked_wallet", Type::optional(Type::String)),
                Field::new(
                    "linked_wallet_blockchain",
                    Type::optional(Type::enum_ref("block_chain")),
                ),
            ],
            format!(
                r#"
BEGIN
    RETURN QUERY SELECT s.pkey_id AS strategy_id,
                        s.name AS strategy_name, 
                        b.pkey_id AS expert_id,
                        b.public_id AS expert_public_id,
                        b.username AS expert_name,
                        s.description AS description,
                        s.created_at AS created_at,
                        s.pending_approval AS pending_approval,
                        s.approved AS approved,
                        s.approved_at AS approved_at,
                        w.address AS linked_wallet,
                        w.blockchain AS linked_wallet_blockchain
                 FROM tbl.strategy AS s
                      LEFT JOIN tbl.strategy_watching_wallet AS w ON w.pkey_id = {linked_wallet}
                      JOIN tbl.user AS b ON b.pkey_id = s.fkey_user_id
                          
                WHERE (a_strategy_id ISNULL OR s.pkey_id = a_strategy_id)
                    AND (a_strategy_name ISNULL OR s.name ILIKE a_strategy_name || '%')
                    AND (a_expert_public_id ISNULL OR b.public_id = a_expert_public_id)
                    AND (a_expert_name ISNULL OR b.username ILIKE a_expert_name || '%')
                    AND (a_description ISNULL OR s.description ILIKE a_description || '%')
                 ORDER BY s.pkey_id
                 OFFSET a_offset
                 LIMIT a_limit;
END
            "#,
                linked_wallet = get_first_linked_wallet()
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
            "fun_admin_reject_strategies",
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
    ]
}
