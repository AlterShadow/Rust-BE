CREATE SCHEMA IF NOT EXISTS api;

CREATE OR REPLACE FUNCTION api.fun_auth_signup(a_public_id bigint, a_username varchar, a_email varchar, a_phone varchar, a_password_hash bytea, a_password_salt bytea, a_age int, a_preferred_language varchar, a_agreed_tos boolean, a_agreed_privacy boolean, a_ip_address inet)
RETURNS table (
    "user_id" bigint
)
LANGUAGE plpgsql
AS $$
    
DECLARE
  id_ bigint;
BEGIN
  IF (a_agreed_tos = FALSE OR a_agreed_privacy = FALSE) THEN
    RAISE SQLSTATE 'R000X'; -- ConsentMissing
  ELSEIF ((SELECT pkey_id
           FROM tbl.user
           WHERE LOWER(username) = LOWER(a_username)) IS NOT NULL) THEN
    RAISE SQLSTATE 'R000Z'; -- UsernameAlreadyRegistered
  END IF;
  INSERT INTO tbl.user (public_id,
                       username,
                       email,
                       phone_number,
                       password_hash,
                       password_salt,
                       age,
                       preferred_language,
                       agreed_tos,
                       agreed_privacy,
                       last_ip)
  VALUES (a_public_id,
          a_username,
          a_email,
          a_phone,
          a_password_hash,
          a_password_salt,
          a_age,
          a_preferred_language,
          a_agreed_tos,
          a_agreed_privacy,
          a_ip_address)
  RETURNING pkey_id INTO STRICT id_;
  RETURN QUERY SELECT id_;
END
        
$$;
        

CREATE OR REPLACE FUNCTION api.fun_auth_authenticate(a_username varchar, a_password_hash bytea, a_service_code int, a_device_id varchar, a_device_os varchar, a_ip_address inet)
RETURNS table (
    "user_id" bigint,
    "user_public_id" bigint
)
LANGUAGE plpgsql
AS $$
    
DECLARE
    is_blocked_     boolean;
    is_password_ok_ boolean;
    _user_id        bigint;
    _user_public_id bigint;
    _role           enum_role;
BEGIN
    ASSERT (a_ip_address NOTNULL AND a_device_id NOTNULL AND a_device_os NOTNULL AND
            a_username NOTNULL AND a_password_hash NOTNULL AND a_service_code NOTNULL);

    -- Looking up the user.
    SELECT pkey_id, u.public_id, is_blocked, (password_hash = a_password_hash), u.role
    INTO _user_id, _user_public_id, is_blocked_, is_password_ok_, _role
    FROM tbl.user u
    WHERE username = a_username;

    -- Log the login attempt.
    INSERT INTO tbl.login_attempt(fkey_user, username, password_hash, ip_address,
                                  is_password_ok)
    VALUES (_user_id, a_username, a_password_hash, a_ip_address, is_password_ok_);
    -- COMMIT;
    -- Checking the block status and password, and updating the login info if ok.
    IF (_user_id ISNULL) THEN
        RAISE SQLSTATE 'R0007'; -- UnknownUser
    END IF;
    IF (is_blocked_) THEN
        RAISE SQLSTATE 'R0008'; -- BlockedUser
    ELSIF (NOT is_password_ok_) THEN
        RAISE SQLSTATE 'R0009';
    ELSEIF (_role NOT IN ('admin', 'developer') AND
            a_service_code = (SELECT code FROM api.ADMIN_SERVICE())) OR
           (_role NOT IN ('user', 'admin', 'developer') AND
            a_service_code = (SELECT code FROM api.USER_SERVICE())) THEN
        RAISE SQLSTATE 'R000S'; -- InvalidRole
    END IF;
    UPDATE tbl.user -- ping
    SET last_ip      = a_ip_address,
        last_login   = EXTRACT(EPOCH FROM (NOW()))::bigint,
        logins_count = logins_count + 1
    WHERE pkey_id = _user_id;

    IF a_service_code = api.USER_SERVICE() THEN
        UPDATE tbl.user SET user_device_id = a_device_id WHERE pkey_id = _user_id;
    END IF;
    IF a_service_code = api.ADMIN_SERVICE() THEN
        UPDATE tbl.user SET admin_device_id = a_device_id WHERE pkey_id = _user_id;
    END IF;
    RETURN QUERY SELECT _user_id, _user_public_id;
END
        
$$;
        

CREATE OR REPLACE FUNCTION api.fun_auth_get_password_salt(a_username varchar)
RETURNS table (
    "salt" bytea
)
LANGUAGE plpgsql
AS $$
    
DECLARE
  user_id bigint;
BEGIN
  ASSERT (a_username NOTNULL);

  -- Looking up the user.
  SELECT pkey_id, u.password_salt
  INTO user_id, salt
  FROM tbl.user u
  WHERE username = a_username;

  IF (user_id ISNULL) THEN
    RAISE SQLSTATE 'R0007'; -- UnknownUser
  END IF;
  RETURN QUERY SELECT salt;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_auth_set_token(a_user_id bigint, a_user_token uuid, a_admin_token uuid, a_service_code int)
RETURNS void
LANGUAGE plpgsql
AS $$
    
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
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_auth_authorize(a_username varchar, a_token uuid, a_service enum_service, a_device_id varchar, a_device_os varchar, a_ip_address inet)
RETURNS table (
    "user_id" bigint,
    "role" enum_role
)
LANGUAGE plpgsql
AS $$
    
DECLARE
    rc_          integer;
    is_token_ok_ boolean;
    user_id_     bigint;
    role_        enum_role;

BEGIN
    ASSERT (a_username NOTNULL AND a_token NOTNULL AND a_service NOTNULL AND
            a_device_id NOTNULL AND a_device_os NOTNULL);

    -- Looking up the user
    CASE a_service
        WHEN 'user'::enum_service
            THEN SELECT pkey_id, u.role, (user_token = a_token)
                 INTO user_id_, role_, is_token_ok_
                 FROM tbl.user AS u
                 WHERE username = a_username;
        WHEN 'admin'::enum_service
            THEN SELECT pkey_id, u.role, (admin_token = a_token)
                 INTO user_id_, role_, is_token_ok_
                 FROM tbl.user AS u
                 WHERE username = a_username;
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
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_auth_change_password(a_username varchar, a_old_password_hash bytea, a_new_password_hash bytea, a_device_id varchar, a_device_os varchar, a_ip_address inet)
RETURNS void
LANGUAGE plpgsql
AS $$
    
DECLARE
  is_blocked_     boolean;
  is_password_ok_ boolean;
  user_id_        bigint;
BEGIN
  ASSERT (a_ip_address NOTNULL AND a_device_id NOTNULL AND a_device_os NOTNULL AND
          a_username NOTNULL AND a_old_password_hash NOTNULL AND
          a_new_password_hash NOTNULL);
  -- Looking up the user.
  SELECT pkey_id, is_blocked, (password_hash = a_old_password_hash)
  INTO user_id_, is_blocked_, is_password_ok_
  FROM tbl.user u
  WHERE username = a_username;

  -- Log the login attempt.
  INSERT INTO tbl.login_attempt(fkey_user, username, password_hash, ip_address,
                                device_id, device_os, is_password_ok)
  VALUES (user_id_, a_username, a_old_password_hash, a_ip_address, a_device_id,
          a_device_os, is_password_ok_);
  -- COMMIT;
  -- Checking the block status and password, and updating the login info if ok.
  IF (user_id_ NOTNULL) THEN
    IF (is_blocked_) THEN
      RAISE SQLSTATE 'R0008'; -- BlockedUser
    ELSIF (NOT is_password_ok_) THEN
      RAISE SQLSTATE 'R0009'; -- InvalidPassword
    END IF;
    ASSERT (a_old_password_hash <> a_new_password_hash);

    UPDATE tbl.user
    SET password_hash = a_new_password_hash
    WHERE username = a_username;
  ELSE
      RAISE SQLSTATE 'R0007'; -- UnknownUser
  END IF;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_get_recovery_question_data()
RETURNS table (
    "question_id" int,
    "content" varchar,
    "category" enum_recovery_question_category
)
LANGUAGE plpgsql
AS $$
    
BEGIN
  RETURN QUERY SELECT q.pkey_id,
                      q.content,
                      q.category
               FROM tbl.recovery_question_data q;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_auth_set_recovery_questions(a_user_id bigint, a_question_ids int[], a_answers int[])
RETURNS void
LANGUAGE plpgsql
AS $$
    
DECLARE
  rc_ integer;
BEGIN
  ASSERT (a_user_id NOTNULL AND a_question_ids NOTNULL AND a_answers NOTNULL);
  DELETE FROM tbl.recovery_question WHERE fkey_user = a_user_id;
  INSERT INTO tbl.recovery_question(fkey_user, fkey_question, answer)
  VALUES (a_user_id, UNNEST(a_question_ids), UNNEST(a_answers));
  GET DIAGNOSTICS rc_ := ROW_COUNT;
  ASSERT (rc_ > 0);
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_auth_basic_authenticate(a_username varchar, a_device_id varchar, a_device_os varchar, a_ip_address inet)
RETURNS table (
    "user_id" inet
)
LANGUAGE plpgsql
AS $$
    
DECLARE
  is_blocked_ boolean;
  user_id_    bigint;
BEGIN
  ASSERT (a_username NOTNULL AND a_device_id NOTNULL AND a_device_os NOTNULL AND
          a_ip_address NOTNULL);
  SELECT pkey_id, is_blocked
  INTO user_id_, is_blocked_
  FROM tbl.user
  WHERE username = LOWER(a_username);
  INSERT INTO tbl.login_attempt(fkey_user, username, password_hash, ip_address,
                                device_id, device_os)
  VALUES (user_id_, a_username, '', a_ip_address, a_device_id, a_device_os);
  -- COMMIT;
  IF (user_id_ ISNULL) THEN
    RAISE SQLSTATE 'R0007'; -- UnknownUser
  ELSEIF (is_blocked_) THEN
    RAISE SQLSTATE 'R0008'; -- BlockedUser
  END IF;
  RETURN QUERY SELECT user_id_;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_auth_get_recovery_questions(a_user_id bigint)
RETURNS table (
    "question_id" int,
    "question" varchar
)
LANGUAGE plpgsql
AS $$
    
BEGIN
  ASSERT (a_user_id NOTNULL);
  RETURN QUERY SELECT qd.pkey_id,
                      qd.content
               FROM tbl.recovery_question_data qd
                      JOIN tbl.recovery_question q ON qd.pkey_id = q.fkey_question
               WHERE q.fkey_user = a_user_id;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_submit_recovery_answers(a_user_id bigint, a_question_ids int[], a_answers varchar[], a_password_reset_token uuid, a_token_valid int)
RETURNS void
LANGUAGE plpgsql
AS $$
    
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
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_auth_reset_password(a_user_id bigint, a_new_password_hash bytea, a_new_password_salt bytea, a_reset_token uuid)
RETURNS void
LANGUAGE plpgsql
AS $$
    
DECLARE
  rc_ integer;
BEGIN
  ASSERT (a_user_id NOTNULL AND a_new_password_hash NOTNULL AND a_reset_token NOTNULL);
  UPDATE tbl.user
  SET password_hash        = a_new_password_hash,
      password_salt        = a_new_password_salt,
      password_reset_token = NULL,
      reset_token_valid    = NULL
  WHERE pkey_id = a_user_id
    AND password_reset_token = a_reset_token
    AND reset_token_valid > EXTRACT(EPOCH FROM NOW())::bigint;
  GET DIAGNOSTICS rc_ := ROW_COUNT;
  IF (rc_ <> 1) THEN
    RAISE SQLSTATE 'R0012'; -- InvalidRecoveryToken
  END IF;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_list_users(a_offset int, a_limit int, a_user_id bigint DEFAULT NULL, a_user_public_id bigint DEFAULT NULL, a_email varchar DEFAULT NULL, a_username varchar DEFAULT NULL, a_role enum_role DEFAULT NULL)
RETURNS table (
    "user_id" bigint,
    "user_public_id" bigint,
    "email" varchar,
    "username" varchar,
    "role" enum_role,
    "updated_at" oid,
    "created_at" oid
)
LANGUAGE plpgsql
AS $$
    
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
        
$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_list_organizations(a_offset bigint, a_limit bigint)
RETURNS table (
    "organization_id" bigint,
    "name" varchar,
    "description" varchar,
    "approved" boolean,
    "member_count" bigint
)
LANGUAGE plpgsql
AS $$
    

BEGIN
    RETURN QUERY SELECT a.pkey_id,
                        a.name,
                        a.note,
                        a.approved,
                        (SELECT COUNT(*) FROM tbl.organization_membership WHERE fkey_organization = a.pkey_id)
                 FROM tbl.organization AS a
                 OFFSET a_offset LIMIT a_limit;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_assign_role(a_operator_user_id bigint, a_user_public_id bigint, a_new_role enum_role)
RETURNS void
LANGUAGE plpgsql
AS $$
    
DECLARE
    _operator_role enum_role;
BEGIN
    SELECT role FROM tbl.user WHERE pkey_id = a_operator_user_id INTO STRICT _operator_role;
    IF _operator_role <> 'admin' THEN
        RAISE SQLSTATE 'R000S'; -- InvalidRole
    END IF;
    UPDATE tbl.user SET role = a_new_role WHERE public_id = a_user_public_id;
END
        
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_create_organization(a_user_id bigint, a_name varchar, a_country varchar, a_tax_id varchar, a_address varchar, a_note varchar, a_approved boolean)
RETURNS table (
    "organization_id" bigint
)
LANGUAGE plpgsql
AS $$
    
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
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_lookup_user_by_email_or_username(a_email varchar, a_username varchar)
RETURNS table (
    "user_id" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT pkey_id
    FROM tbl.user
    WHERE email = a_email OR username = a_username;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_get_organization_membership(a_user_id bigint, a_organization_id bigint)
RETURNS table (
    "role" enum_role
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT a.role
               FROM tbl.organization_membership AS a
               WHERE fkey_organization = a_organization_id
                 AND fkey_user = a_user_id;
END
        
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_organizations(a_user_id bigint)
RETURNS table (
    "organization_id" bigint,
    "name" varchar,
    "role" enum_role,
    "accepted" boolean
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT a.pkey_id, a.name, b.role, b.accepted
                 FROM tbl.organization AS a
                          INNER JOIN tbl.organization_membership AS b ON a.pkey_id = b.fkey_organization
                 WHERE b.fkey_user = a_user_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_invite_user_to_organization(a_organization_id bigint, a_target_user_id bigint)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    INSERT INTO tbl.organization_membership (fkey_user, fkey_organization, role, accepted, created_at)
    VALUES (a_target_user_id, a_organization_id, 'user', FALSE, EXTRACT(EPOCH FROM NOW())::bigint);    
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_accept_organization_invitation(a_user_id bigint, a_organization_id bigint)
RETURNS table (
    "membership_id" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY UPDATE tbl.organization_membership
    SET accepted = TRUE
    WHERE fkey_user = a_user_id AND fkey_organization = a_organization_id
    RETURNING pkey_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_organization_invitations_by_organization(a_organization_id bigint)
RETURNS table (
    "membership_id" bigint,
    "organization_id" bigint,
    "organization_name" varchar,
    "user_id" bigint,
    "username" varchar,
    "email" varchar,
    "created_at" oid,
    "accepted" boolean
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT a.pkey_id, c.pkey_id, c.name, b.pkey_id, b.username, b.email,
           a.created_at, a.accepted -- a.updated_at, a.accepted_at
    FROM tbl.organization_membership AS a
    INNER JOIN tbl.user AS b ON b.pkey_id = a.fkey_user
    INNER JOIN tbl.organization AS c ON c.pkey_id = a.fkey_organization
         WHERE a.fkey_organization = a_organization_id AND a.accepted = FALSE;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_organization_invitations_by_user(a_user_id bigint)
RETURNS table (
    "membership_id" bigint,
    "organization_id" bigint,
    "organization_name" varchar,
    "user_id" bigint,
    "username" varchar,
    "email" varchar,
    "created_at" oid,
    "accepted" boolean
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT a.pkey_id, c.pkey_id, c.name, b.pkey_id, b.username, b.email,
           a.created_at, a.accepted -- a.updated_at, a.accepted_at
    FROM tbl.organization_membership AS a
    INNER JOIN tbl.user AS b ON b.pkey_id = a.fkey_user
    INNER JOIN tbl.organization AS c ON c.pkey_id = a.fkey_organization
         WHERE a.fkey_user = a_user_id AND a.accepted = FALSE;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_set_user_role_in_organization(a_user_id bigint, a_organization_id bigint, a_new_role enum_role)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    UPDATE tbl.organization_membership
    SET role = a_new_role
    WHERE fkey_user = a_user_id AND fkey_organization = a_organization_id;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_delete_user_from_organization(a_user_id bigint, a_organization_id bigint)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    DELETE FROM tbl.organization_membership
    WHERE fkey_user = a_user_id AND fkey_organization = a_organization_id;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_get_organization(a_organization_id bigint)
RETURNS table (
    "organization_id" bigint,
    "name" varchar,
    "description" varchar,
    "approved" boolean
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT a.pkey_id, a.name, a.note, a.approved
        FROM tbl.organization AS a
        WHERE pkey_id = a_organization_id;
  
END
        
$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_approve_organization(a_organization_id bigint)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    UPDATE tbl.organization SET approved = TRUE
        WHERE pkey_id = a_organization_id;
END
        
$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_disapprove_organization(a_organization_id bigint)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    UPDATE tbl.organization SET approved = FALSE
        WHERE pkey_id = a_organization_id;
END
        
$$;
        

CREATE OR REPLACE FUNCTION api.AUTH_SERVICE()
RETURNS table (
    "code" int
)
LANGUAGE plpgsql
AS $$
    BEGIN RETURN QUERY SELECT 1; END
$$;
        

CREATE OR REPLACE FUNCTION api.USER_SERVICE()
RETURNS table (
    "code" int
)
LANGUAGE plpgsql
AS $$
    BEGIN RETURN QUERY SELECT 2; END
$$;
        

CREATE OR REPLACE FUNCTION api.ADMIN_SERVICE()
RETURNS table (
    "code" int
)
LANGUAGE plpgsql
AS $$
    BEGIN RETURN QUERY SELECT 3; END
$$;
        
