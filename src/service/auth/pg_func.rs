use model::pg_func::ProceduralFunction;
use model::types::*;

pub fn get_auth_pg_func() -> Vec<ProceduralFunction> {
    vec![
        ProceduralFunction::new(
            "fun_auth_signup",
            vec![
                Field::new("address", Type::BlockchainAddress),
                Field::new("email", Type::String),
                Field::new("phone", Type::String),
                Field::new("preferred_language", Type::String),
                Field::new("agreed_tos", Type::Boolean),
                Field::new("agreed_privacy", Type::Boolean),
                Field::new("ip_address", Type::Inet),
                Field::new("public_id", Type::BigInt),
                Field::new("username", Type::optional(Type::String)),
                Field::new("age", Type::optional(Type::Int)),
                Field::new("ens_name", Type::optional(Type::String)),
                Field::new("ens_avatar", Type::optional(Type::String)),
            ],
            vec![Field::new("user_id", Type::BigInt)],
            r#"

DECLARE
    id_ bigint;
BEGIN
    IF (a_agreed_tos = FALSE OR a_agreed_privacy = FALSE) THEN
        RAISE SQLSTATE 'R000X'; -- ConsentMissing
    ELSEIF ((SELECT pkey_id
             FROM tbl.user
             WHERE address = a_address) IS NOT NULL) THEN
        RAISE SQLSTATE 'R000Z'; -- UsernameAlreadyRegistered
    END IF;
    INSERT INTO tbl.user (address,
                          username,
                          email,
                          phone_number,
                          role,
                          age,
                          preferred_language,
                          agreed_tos,
                          agreed_privacy,
                          last_ip,
                          public_id,
                          updated_at,
                          created_at,
                          ens_name,
                          ens_avatar
      )
    VALUES (a_address,
            a_username,
            a_email,
            a_phone,
            'user'::enum_role,
            a_age,
            a_preferred_language,
            a_agreed_tos,
            a_agreed_privacy,
            a_ip_address,
            a_public_id,
            extract(Epoch FROM (NOW()))::bigint,
            extract(Epoch FROM (NOW()))::bigint
            a_ens_name,
            a_ens_avatar
        )
    RETURNING pkey_id INTO STRICT id_;
    INSERT INTO tbl.user_whitelisted_wallet(fkey_user_id,
                                           blockchain,
                                           address,
                                           created_at)
    VALUES (id_,
            'EthereumMainnet'::enum_block_chain,
            a_address,
            extract(Epoch FROM (NOW()))::bigint),
            (id_,
            'BscMainnet'::enum_block_chain,
            a_address,
            extract(Epoch FROM (NOW()))::bigint);
            
    RETURN QUERY SELECT id_;
END

        "#,
        ),
        ProceduralFunction::new(
            "fun_auth_authenticate",
            vec![
                Field::new("address", Type::BlockchainAddress),
                Field::new("service_code", Type::Int),
                Field::new("device_id", Type::String),
                Field::new("device_os", Type::String),
                Field::new("ip_address", Type::Inet),
            ],
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("public_user_id", Type::BigInt),
                Field::new("role", Type::enum_ref("role")),
                Field::new("ens_name", Type::optional(Type::String)),
                Field::new("ens_avatar", Type::optional(Type::String)),
            ],
            r#"
DECLARE
    is_blocked_     boolean;
    _user_id        bigint;
    _public_user_id bigint;
    _role           enum_role;
BEGIN
    ASSERT (a_ip_address NOTNULL AND a_device_id NOTNULL AND a_device_os NOTNULL AND
            a_address NOTNULL AND a_service_code NOTNULL);

    -- Looking up the user.
    SELECT pkey_id, is_blocked, u.role, u.public_id
    INTO _user_id, is_blocked_, _role, _public_user_id
    FROM tbl.user u
    WHERE address = a_address;

    -- Log the login attempt. 
    INSERT INTO tbl.login_attempt(fkey_user, address, ip_address, is_password_ok, moment)
    VALUES (_user_id, a_address, a_ip_address, TRUE, extract(Epoch FROM (NOW()))::bigint);
    -- TODO: is_password_ok should be passed from the backend.
    -- COMMIT;
    -- Checking the block status and password, and updating the login info if ok.
    IF (_user_id ISNULL) THEN
        RAISE SQLSTATE 'R0007'; -- UnknownUser
    END IF;
    IF (is_blocked_) THEN
        RAISE SQLSTATE 'R0008'; -- BlockedUser
    ELSEIF (_role NOT IN ('admin', 'developer') AND
            a_service_code = (SELECT code FROM api.ADMIN_SERVICE())) OR
           (_role NOT IN ('user', 'expert', 'admin', 'developer') AND
            a_service_code = (SELECT code FROM api.USER_SERVICE())) THEN
        RAISE SQLSTATE 'R000S'; -- InvalidRole
    END IF;
    UPDATE tbl.user -- ping
    SET last_ip      = a_ip_address,
        last_login_at   = EXTRACT(EPOCH FROM (NOW()))::bigint,
        login_count = login_count + 1
    WHERE pkey_id = _user_id;

    IF a_service_code = api.USER_SERVICE() THEN
        UPDATE tbl.user SET user_device_id = a_device_id WHERE pkey_id = _user_id;
    END IF;
    IF a_service_code = api.ADMIN_SERVICE() THEN
        UPDATE tbl.user SET admin_device_id = a_device_id WHERE pkey_id = _user_id;
    END IF;
    RETURN QUERY SELECT pkey_id, u.public_id, u.role, ens_name, ens_avatar
    FROM tbl.user u
    WHERE address = a_address;;
END
        "#,
        ),
        ProceduralFunction::new(
            "fun_auth_set_token",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("user_token", Type::UUID),
                Field::new("admin_token", Type::UUID),
                Field::new("service_code", Type::Int),
            ],
            vec![],
            r#"
DECLARE
  rc_         integer;
  is_blocked_ boolean;
BEGIN
  ASSERT (a_user_id NOTNULL AND a_service_code NOTNULL AND a_user_token NOTNULL AND
          a_admin_token NOTNULL);
  -- Looking up the user.
  SELECT is_blocked INTO is_blocked_ FROM tbl.user WHERE pkey_id = a_user_id;
  IF (is_blocked_ ISNULL) THEN
    RAISE SQLSTATE 'R0007'; -- UnknownUser
  ELSIF (is_blocked_) THEN
    RAISE SQLSTATE 'R0008'; -- BlockedUser
  END IF;

  -- Setting up the token.
  IF a_service_code = (SELECT code FROM api.USER_SERVICE()) THEN
    UPDATE tbl.user
    SET user_token = a_user_token
    WHERE pkey_id = a_user_id;
  END IF;
  IF a_service_code = (SELECT code FROM api.ADMIN_SERVICE())  THEN
    UPDATE tbl.user
    SET admin_token = a_admin_token
    WHERE pkey_id = a_user_id;
  END IF;

  GET DIAGNOSTICS rc_ := ROW_COUNT;
  ASSERT (rc_ = 1);
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_auth_remove_token",
            vec![Field::new("user_id", Type::BigInt)],
            vec![],
            r#"
BEGIN
  ASSERT (a_user_id NOTNULL);

  -- Setting up the token.
  UPDATE tbl.user
  SET user_token = NULL, admin_token = NULL
  WHERE pkey_id = a_user_id;

END
            "#,
        ),
        ProceduralFunction::new(
            "fun_auth_authorize",
            vec![
                Field::new("address", Type::BlockchainAddress),
                Field::new("token", Type::UUID),
                Field::new("service", Type::enum_ref("service")),
                Field::new("device_id", Type::String),
                Field::new("device_os", Type::String),
                Field::new("ip_address", Type::Inet),
            ],
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("role", Type::enum_ref("role")),
            ],
            r#"
DECLARE
    rc_          integer;
    is_token_ok_ boolean;
    user_id_     bigint;
    role_        enum_role;

BEGIN
    ASSERT (a_address NOTNULL AND a_token NOTNULL AND a_service NOTNULL AND
            a_device_id NOTNULL AND a_device_os NOTNULL);

    -- Looking up the user
    CASE a_service
        WHEN 'user'::enum_service
            THEN SELECT pkey_id, u.role, (user_token = a_token)
                 INTO user_id_, role_, is_token_ok_
                 FROM tbl.user AS u
                 WHERE address = a_address;
        WHEN 'admin'::enum_service
            THEN SELECT pkey_id, u.role, (admin_token = a_token)
                 INTO user_id_, role_, is_token_ok_
                 FROM tbl.user AS u
                 WHERE address = a_address;
        ELSE RAISE SQLSTATE 'R0001'; -- InvalidArgument
        END CASE;
    GET DIAGNOSTICS rc_ := ROW_COUNT;
    IF (rc_ <> 1) THEN
        RAISE SQLSTATE 'R0007'; -- UnknownUser
    END IF;

    -- Log the authorization attempt
    INSERT INTO tbl.authorization_attempt(fkey_user, ip_address, is_token_ok, moment)
    VALUES (user_id_, a_ip_address, is_token_ok_ NOTNULL AND is_token_ok_, extract(Epoch FROM (NOW()))::bigint);
    -- COMMIT;
    -- Validating the token
    IF NOT is_token_ok_ OR is_token_ok_ IS NULL THEN
        RAISE SQLSTATE 'R000A'; -- InvalidToken
    END IF;

    -- Updating the device info
    CASE a_service
        WHEN 'user'::enum_service
            THEN UPDATE tbl.user
                 SET user_device_id = a_device_id
                 WHERE pkey_id = user_id_
                   AND user_token = a_token;
        WHEN 'admin'::enum_service
            THEN UPDATE tbl.user
                 SET admin_device_id = a_device_id
                 WHERE pkey_id = user_id_
                   AND admin_token = a_token;
        END CASE;
    GET DIAGNOSTICS rc_ := ROW_COUNT;
    ASSERT (rc_ = 1);
    RETURN QUERY SELECT user_id_, role_;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_auth_set_role",
            vec![
                Field::new("public_user_id", Type::BigInt),
                Field::new("role", Type::enum_ref("role")),
            ],
            vec![],
            r#"
BEGIN
    UPDATE tbl.user SET role = a_role WHERE public_id = a_public_user_id;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_auth_change_login_wallet_address",
            vec![
                Field::new("old_wallet_address", Type::BlockchainAddress),
                Field::new("new_wallet_address", Type::BlockchainAddress),
            ],
            vec![],
            r#"
BEGIN
    UPDATE tbl.user SET address = a_new_wallet_address,
                updated_at = EXTRACT(EPOCH FROM NOW())::bigint
     WHERE address = a_old_wallet_address;
    
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_auth_update_user_table",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("username", Type::optional(Type::String)),
                Field::new("family_name", Type::optional(Type::String)),
                Field::new("given_name", Type::optional(Type::String)),
            ],
            vec![],
            r#"
BEGIN
    UPDATE tbl.user 
    SET
        username = COALESCE(a_username, username),
        family_name = COALESCE(a_family_name, family_name),
        given_name = COALESCE(a_given_name, given_name),
        updated_at = EXTRACT(EPOCH FROM NOW())::bigint
     WHERE pkey_id = a_user_id;
    
END
            "#,
        ),
    ]
}
