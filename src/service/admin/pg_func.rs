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
            "fun_admin_approve_user_become_admin",
            vec![Field::new("user_id", Type::BigInt)],
            vec![Field::new("success", Type::Boolean)],
            r#"
BEGIN
    UPDATE tbl.user SET pending_expert = FALSE AND role = 'expert' WHERE pkey_id = a_user_id AND role = 'user';
    RETURN QUERY SELECT TRUE;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_admin_reject_user_become_admin",
            vec![
                Field::new("admin_user_id", Type::BigInt),
                Field::new("user_id", Type::BigInt),
            ],
            vec![Field::new("success", Type::Boolean)],
            r#"
BEGIN
    UPDATE tbl.user SET pending_expert = FALSE WHERE pkey_id = a_user_id;
    RETURN QUERY SELECT TRUE;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_admin_list_pending_user_expert_applications",
            vec![],
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("name", Type::optional(Type::String)),
                Field::new("follower_count", Type::BigInt),
                Field::new("description", Type::String),
                Field::new("social_media", Type::String),
                Field::new("risk_score", Type::Numeric),
                Field::new("reputation_score", Type::Numeric),
                Field::new("aum", Type::Numeric),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT a.pkey_id                  AS expert_id,
                        a.username                 AS name,
                        (SELECT COUNT(*)
                         FROM tbl.user_follow_expert
                         WHERE fkey_expert_id = a.pkey_id
                           AND unfollowed = FALSE) AS follower_count,
                        ''::varchar                AS description,
                        ''::varchar                AS social_media,
                        0.0::double precision      AS risk_score,
                        0.0::double precision      AS reputation_score,
                        0.0::double precision      AS aum
                 FROM tbl."user" AS a
                 WHERE a.pending_expert = TRUE;
END
"#,
        ),
    ]
}
