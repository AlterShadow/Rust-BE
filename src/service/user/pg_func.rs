use model::types::*;

pub fn get_user_pg_func() -> Vec<ProceduralFunction> {
    vec![
        ProceduralFunction::new(
            "fun_admin_list_users",
            vec![
                Field::new("offset", Type::Int),
                Field::new("limit", Type::Int),
                Field::new("user_id", Type::optional(Type::BigInt)),
                Field::new("user_public_id", Type::optional(Type::BigInt)),
                Field::new("email", Type::optional(Type::String)),
                Field::new("username", Type::optional(Type::String)),
                Field::new("role", Type::optional(Type::enum_ref("role"))),
            ],
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("user_public_id", Type::BigInt),
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
        u.public_id,
        u.email,
        u.username,
        u.role,
        u.updated_at::int,
        u.created_at::int
    FROM tbl.user AS u
    WHERE a_user_id IS NOT NULL OR u.pkey_id = a_user_id
        AND a_user_public_id IS NOT NULL OR u.public_id = a_user_public_id
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
            "fun_admin_list_organizations",
            vec![
                Field::new("offset", Type::BigInt),
                Field::new("limit", Type::BigInt),
            ],
            vec![
                Field::new("organization_id", Type::BigInt),
                Field::new("name", Type::String),
                Field::new("description", Type::String),
                Field::new("approved", Type::Boolean),
                Field::new("member_count", Type::BigInt),
            ],
            r#"

BEGIN
    RETURN QUERY SELECT a.pkey_id,
                        a.name,
                        a.note,
                        a.approved,
                        (SELECT COUNT(*) FROM tbl.organization_membership WHERE fkey_organization = a.pkey_id)
                 FROM tbl.organization AS a
                 OFFSET a_offset LIMIT a_limit;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_admin_assign_role",
            vec![
                Field::new("operator_user_id", Type::BigInt),
                Field::new("user_public_id", Type::BigInt),
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
    UPDATE tbl.user SET role = a_new_role WHERE public_id = a_user_public_id;
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
        ProceduralFunction::new(
            "fun_user_invite_user_to_organization",
            vec![
                Field::new("organization_id", Type::BigInt),
                Field::new("target_user_id", Type::BigInt),
            ],
            vec![],
            r#"
BEGIN
    INSERT INTO tbl.organization_membership (fkey_user, fkey_organization, role, accepted, created_at)
    VALUES (a_target_user_id, a_organization_id, 'user', FALSE, EXTRACT(EPOCH FROM NOW())::bigint);    
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_accept_organization_invitation",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("organization_id", Type::BigInt),
            ],
            vec![Field::new("membership_id", Type::BigInt)],
            r#"
BEGIN
    RETURN QUERY UPDATE tbl.organization_membership
    SET accepted = TRUE
    WHERE fkey_user = a_user_id AND fkey_organization = a_organization_id
    RETURNING pkey_id;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_list_organization_invitations_by_organization",
            vec![Field::new("organization_id", Type::BigInt)],
            vec![
                Field::new("membership_id", Type::BigInt),
                Field::new("organization_id", Type::BigInt),
                Field::new("organization_name", Type::String),
                Field::new("user_id", Type::BigInt),
                Field::new("username", Type::String),
                Field::new("email", Type::String),
                Field::new("created_at", Type::Second),
                // Field::new("updated_at", Type::Second),
                // Field::new("accepted_at", Type::Second),
                Field::new("accepted", Type::Boolean),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT a.pkey_id, c.pkey_id, c.name, b.pkey_id, b.username, b.email,
           a.created_at, a.accepted -- a.updated_at, a.accepted_at
    FROM tbl.organization_membership AS a
    INNER JOIN tbl.user AS b ON b.pkey_id = a.fkey_user
    INNER JOIN tbl.organization AS c ON c.pkey_id = a.fkey_organization
         WHERE a.fkey_organization = a_organization_id AND a.accepted = FALSE;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_list_organization_invitations_by_user",
            vec![Field::new("user_id", Type::BigInt)],
            vec![
                Field::new("membership_id", Type::BigInt),
                Field::new("organization_id", Type::BigInt),
                Field::new("organization_name", Type::String),
                Field::new("user_id", Type::BigInt),
                Field::new("username", Type::String),
                Field::new("email", Type::String),
                Field::new("created_at", Type::Second),
                // Field::new("updated_at", Type::Second),
                // Field::new("accepted_at", Type::Second),
                Field::new("accepted", Type::Boolean),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT a.pkey_id, c.pkey_id, c.name, b.pkey_id, b.username, b.email,
           a.created_at, a.accepted -- a.updated_at, a.accepted_at
    FROM tbl.organization_membership AS a
    INNER JOIN tbl.user AS b ON b.pkey_id = a.fkey_user
    INNER JOIN tbl.organization AS c ON c.pkey_id = a.fkey_organization
         WHERE a.fkey_user = a_user_id AND a.accepted = FALSE;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_set_user_role_in_organization",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("organization_id", Type::BigInt),
                Field::new("new_role", Type::enum_ref("role")),
            ],
            vec![],
            r#"
BEGIN
    UPDATE tbl.organization_membership
    SET role = a_new_role
    WHERE fkey_user = a_user_id AND fkey_organization = a_organization_id;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_delete_user_from_organization",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("organization_id", Type::BigInt),
            ],
            vec![],
            r#"
BEGIN
    DELETE FROM tbl.organization_membership
    WHERE fkey_user = a_user_id AND fkey_organization = a_organization_id;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_admin_get_organization",
            vec![Field::new("organization_id", Type::BigInt)],
            vec![
                Field::new("organization_id", Type::BigInt),
                Field::new("name", Type::String),
                Field::new("description", Type::String),
                Field::new("approved", Type::Boolean),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT a.pkey_id, a.name, a.note, a.approved
        FROM tbl.organization AS a
        WHERE pkey_id = a_organization_id;
  
END
        "#,
        ),
        ProceduralFunction::new(
            "fun_admin_approve_organization",
            vec![Field::new("organization_id", Type::BigInt)],
            vec![],
            r#"
BEGIN
    UPDATE tbl.organization SET approved = TRUE
        WHERE pkey_id = a_organization_id;
END
        "#,
        ),
        ProceduralFunction::new(
            "fun_admin_disapprove_organization",
            vec![Field::new("organization_id", Type::BigInt)],
            vec![],
            r#"
BEGIN
    UPDATE tbl.organization SET approved = FALSE
        WHERE pkey_id = a_organization_id;
END
        "#,
        ),
    ]
}
