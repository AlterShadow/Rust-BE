use model::types::*;

pub fn get_user_pg_func() -> Vec<ProceduralFunction> {
    vec![
        ProceduralFunction::new(
            "fun_admin_list_users",
            vec![
                Field::new("offset", Type::Int),
                Field::new("limit", Type::Int),
                Field::new("user_id", Type::optional(Type::BigInt)),
                Field::new("email", Type::optional(Type::String)),
                Field::new("username", Type::optional(Type::String)),
                Field::new("role", Type::optional(Type::enum_ref("role"))),
            ],
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("email", Type::String),
                Field::new("username", Type::String),
                Field::new("role", Type::enum_ref("role")),
                Field::new("updated_at", Type::Second),
                Field::new("created_at", Type::Second),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT
        u.pkey_id,
        u.email,
        u.username,
        u.role,
        u.updated_at::int,
        u.created_at::int
    FROM tbl.user AS u
    WHERE a_user_id IS NOT NULL OR u.pkey_id = a_user_id
        AND a_email IS NOT NULL OR u.email = a_email
        AND a_username IS NOT NULL OR u.username = a_username
        AND a_role IS NOT NULL OR u.role = a_role
    ORDER BY user_id
    OFFSET a_offset
    LIMIT a_limit;
END
        "#,
        ),
        ProceduralFunction::new(
            "fun_admin_assign_role",
            vec![
                Field::new("operator_user_id", Type::BigInt),
                Field::new("user_id", Type::BigInt),
                Field::new("new_role", Type::enum_ref("role")),
            ],
            vec![],
            r#"
DECLARE
    _operator_role enum_role;
BEGIN
    SELECT role FROM tbl.user WHERE pkey_id = a_operator_user_id INTO STRICT _operator_role;
    IF _operator_role <> 'admin' THEN
        RAISE SQLSTATE 'R000S'; -- InvalidRole
    END IF;
    UPDATE tbl.user SET role = a_new_role WHERE pkey_id = a_user_id;
END
        "#,
        ),
        ProceduralFunction::new(
            "fun_user_create_organization",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("name", Type::String),
                Field::new("country", Type::String),
                Field::new("tax_id", Type::String),
                Field::new("address", Type::String),
                Field::new("note", Type::String),
                Field::new("approved", Type::Boolean),
            ],
            vec![Field::new("organization_id", Type::BigInt)],
            r#"
DECLARE
    organization_id BIGINT;
BEGIN
    INSERT INTO tbl.organization (name, country, tax_id, address, note, approved)
    VALUES (a_name, a_country, a_tax_id, a_address, a_note, a_approved)
    RETURNING pkey_id INTO organization_id;
    
    INSERT INTO tbl.organization_membership (fkey_user, fkey_organization, role, accepted, created_at)
        VALUES (a_user_id, organization_id, 'owner', true, EXTRACT(EPOCH FROM NOW())::bigint);
    
    
    RETURN QUERY SELECT organization_id;

END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_lookup_user_by_email_or_username",
            vec![
                Field::new("email", Type::String),
                Field::new("username", Type::String),
            ],
            vec![Field::new("user_id", Type::BigInt)],
            r#"
BEGIN
    RETURN QUERY SELECT pkey_id
    FROM tbl.user
    WHERE email = a_email OR username = a_username;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_get_organization_membership",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("organization_id", Type::BigInt),
            ],
            vec![Field::new("role", Type::enum_ref("role"))],
            r#"
BEGIN
    RETURN QUERY SELECT a.role
               FROM tbl.organization_membership AS a
               WHERE fkey_organization = a_organization_id
                 AND fkey_user = a_user_id;
END
        "#,
        ),
        ProceduralFunction::new(
            "fun_user_list_organizations",
            vec![Field::new("user_id", Type::BigInt)],
            vec![
                Field::new("organization_id", Type::BigInt),
                Field::new("name", Type::String),
                Field::new("role", Type::enum_ref("role")),
                Field::new("accepted", Type::Boolean),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT a.pkey_id, a.name, b.role, b.accepted
                 FROM tbl.organization AS a
                          INNER JOIN tbl.organization_membership AS b ON a.pkey_id = b.fkey_organization
                 WHERE b.fkey_user = a_user_id;
END
"#,
        ),
    ]
}
