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
BEGIN
    UPDATE tbl.expert_profile SET pending_expert = FALSE, approved_expert = TRUE WHERE public_id = a_user_public_id;
    UPDATE tbl.user SET role = 'expert' WHERE role = 'user';
    RETURN QUERY SELECT TRUE;

END
"#,
        ),
        ProceduralFunction::new(
            "fun_admin_reject_user_become_expert",
            vec![Field::new("user_public_id", Type::BigInt)],
            vec![Field::new("success", Type::Boolean)],
            r#"
BEGIN
    UPDATE tbl.expert_profile SET pending_expert = FALSE, approved_expert = FALSE WHERE public_id = a_user_public_id;
    RETURN QUERY SELECT TRUE;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_admin_list_pending_user_expert_applications",
            vec![],
            vec![
                Field::new("user_public_id", Type::BigInt),
                Field::new("name", Type::optional(Type::String)),
                Field::new("follower_count", Type::BigInt),
                Field::new("description", Type::String),
                Field::new("social_media", Type::String),
                Field::new("risk_score", Type::Numeric),
                Field::new("reputation_score", Type::Numeric),
                Field::new("aum", Type::Numeric),
                Field::new("pending_expert", Type::Boolean),
                Field::new("approved_expert", Type::Boolean),
                Field::new("joined_at", Type::BigInt),
                Field::new("request_at", Type::BigInt),
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
                        b.request_at                AS request_at
                 FROM tbl."user" AS a
                    JOIN tbl.expert_profile AS b ON b.fkey_user_id = a.pkey_id
                 WHERE b.pending_expert = TRUE;
END
"#,
        ),
    ]
}
