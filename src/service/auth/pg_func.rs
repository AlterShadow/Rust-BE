use model::types::*;

pub fn get_auth_pg_func() -> Vec<ProceduralFunction> {
    vec![
        ProceduralFunction::new(
            "fun_auth_signup",
            vec![
                Field::new("address", Type::String),
                Field::new("email", Type::String),
                Field::new("phone", Type::String),
                Field::new("age", Type::Int),
                Field::new("preferred_language", Type::String),
                Field::new("agreed_tos", Type::Boolean),
                Field::new("agreed_privacy", Type::Boolean),
                Field::new("ip_address", Type::Inet),
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
           WHERE LOWER(address) = LOWER(a_address)) IS NOT NULL) THEN
    RAISE SQLSTATE 'R000Z'; -- UsernameAlreadyRegistered
  END IF;
  INSERT INTO tbl.user (address,
                       email,
                       phone_number,
                       age,
                       preferred_language,
                       agreed_tos,
                       agreed_privacy,
                       last_ip)
  VALUES (a_address,
          a_email,
          a_phone,
          a_age,
          a_preferred_language,
          a_agreed_tos,
          a_agreed_privacy,
          a_ip_address)
  RETURNING pkey_id INTO STRICT id_;
  RETURN QUERY SELECT id_;
END
        "#,
        ),
        ProceduralFunction::new(
            "fun_auth_authenticate",
            vec![
                Field::new("address", Type::String),
                Field::new("service_code", Type::Int),
                Field::new("device_id", Type::String),
                Field::new("device_os", Type::String),
                Field::new("ip_address", Type::Inet),
            ],
            vec![Field::new("user_id", Type::BigInt)],
            r#"
DECLARE
    is_blocked_     boolean;
    _user_id        bigint;
    _role           enum_role;
BEGIN
    ASSERT (a_ip_address NOTNULL AND a_device_id NOTNULL AND a_device_os NOTNULL AND
            a_address NOTNULL AND a_service_code NOTNULL);

    -- Looking up the user.
    SELECT pkey_id, is_blocked, u.role
    INTO _user_id, is_blocked_, _role
    FROM tbl.user u
    WHERE address = a_address;

    -- Log the login attempt. 
    INSERT INTO tbl.login_attempt(fkey_user, address, ip_address, is_password_ok)
    VALUES (_user_id, a_address, a_ip_address, TRUE);
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
           (_role NOT IN ('user', 'admin', 'developer') AND
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
    RETURN QUERY SELECT _user_id;
END
        "#,
        ),
        ProceduralFunction::new(
            "fun_auth_get_password_salt",
            vec![Field::new("address", Type::String)],
            vec![Field::new("salt", Type::Bytea)],
            r#"
DECLARE
  user_id bigint;
BEGIN
  ASSERT (a_address NOTNULL);

  -- Looking up the user.
  SELECT pkey_id, u.password_salt
  INTO user_id, salt
  FROM tbl.user u
  WHERE address = a_address;

  IF (user_id ISNULL) THEN
    RAISE SQLSTATE 'R0007'; -- UnknownUser
  END IF;
  RETURN QUERY SELECT salt;
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
            "fun_auth_authorize",
            vec![
                Field::new("address", Type::String),
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
    INSERT INTO tbl.authorization_attempt(fkey_user, ip_address, is_token_ok)
    VALUES (user_id_, a_ip_address, is_token_ok_ NOTNULL AND is_token_ok_);

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
            "fun_auth_basic_authenticate",
            vec![
                Field::new("address", Type::String),
                Field::new("device_id", Type::String),
                Field::new("device_os", Type::String),
                Field::new("ip_address", Type::Inet),
            ],
            vec![Field::new("user_id", Type::Inet)],
            r#"
DECLARE
  is_blocked_ boolean;
  user_id_    bigint;
BEGIN
  ASSERT (a_address NOTNULL AND a_device_id NOTNULL AND a_device_os NOTNULL AND
          a_ip_address NOTNULL);
  SELECT pkey_id, is_blocked
  INTO user_id_, is_blocked_
  FROM tbl.userd
  WHERE address = LOWER(a_address);
  INSERT INTO tbl.login_attempt(fkey_user, address, ip_address,
                                device_id, device_os)
  VALUES (user_id_, a_address,  a_ip_address, a_device_id, a_device_os);
  -- COMMIT;
  IF (user_id_ ISNULL) THEN
    RAISE SQLSTATE 'R0007'; -- UnknownUser
  ELSEIF (is_blocked_) THEN
    RAISE SQLSTATE 'R0008'; -- BlockedUser
  END IF;
  RETURN QUERY SELECT user_id_;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_auth_get_recovery_questions",
            vec![Field::new("user_id", Type::BigInt)],
            vec![
                Field::new("question_id", Type::Int),
                Field::new("question", Type::String),
            ],
            r#"
BEGIN
  ASSERT (a_user_id NOTNULL);
  RETURN QUERY SELECT qd.pkey_id,
                      qd.content
               FROM tbl.recovery_question_data qd
                      JOIN tbl.recovery_question q ON qd.pkey_id = q.fkey_question
               WHERE q.fkey_user = a_user_id;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_submit_recovery_answers",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("question_ids", Type::Vec(Box::new(Type::Int))),
                Field::new("answers", Type::Vec(Box::new(Type::String))),
                Field::new("password_reset_token", Type::UUID),
                Field::new("token_valid", Type::Int),
            ],
            vec![],
            r#"
DECLARE
  correct_answers_ varchar[];
BEGIN
  IF (SELECT COUNT(pkey_id) FROM tbl.recovery_question WHERE fkey_user = a_user_id) !=
     CARDINALITY(a_question_ids) THEN
    RAISE SQLSTATE 'R0011'; -- MustSubmitAllRecoveryQuestions
  END IF;
  SELECT ARRAY_AGG(result.answer)
  INTO correct_answers_
  FROM (SELECT q.answer AS answer
        FROM tbl.recovery_question q
               JOIN UNNEST(a_question_ids) WITH ORDINALITY t(fkey_question, ord)
                    USING (fkey_question)
        WHERE fkey_user = a_user_id
        ORDER BY t.ord) result;
  IF a_answers != correct_answers_ THEN
    RAISE SQLSTATE 'R000T'; -- WrongRecoveryAnswers
  END IF;
  UPDATE tbl.user
  SET password_reset_token = a_password_reset_token,
      reset_token_valid    = a_token_valid
  WHERE pkey_id = a_user_id;
END
            "#,
        ),
    ]
}
