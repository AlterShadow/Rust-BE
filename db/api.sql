CREATE SCHEMA IF NOT EXISTS api;

CREATE OR REPLACE FUNCTION api.fun_auth_signup(a_address varchar, a_email varchar, a_phone varchar, a_age int, a_preferred_language varchar, a_agreed_tos boolean, a_agreed_privacy boolean, a_ip_address inet)
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
             WHERE LOWER(address) = LOWER(a_address)) IS NOT NULL) THEN
        RAISE SQLSTATE 'R000Z'; -- UsernameAlreadyRegistered
    END IF;
    INSERT INTO tbl.user (address,
                          email,
                          phone_number,
                          role,
                          age,
                          preferred_language,
                          agreed_tos,
                          agreed_privacy,
                          last_ip,
                          updated_at,
                          created_at)
    VALUES (a_address,
            a_email,
            a_phone,
            'user'::enum_role,
            a_age,
            a_preferred_language,
            a_agreed_tos,
            a_agreed_privacy,
            a_ip_address,
            extract(Epoch FROM (NOW()))::bigint,
            extract(Epoch FROM (NOW()))::bigint)
    RETURNING pkey_id INTO STRICT id_;
    RETURN QUERY SELECT id_;
END

        
$$;
        

CREATE OR REPLACE FUNCTION api.fun_auth_authenticate(a_address varchar, a_service_code int, a_device_id varchar, a_device_os varchar, a_ip_address inet)
RETURNS table (
    "user_id" bigint
)
LANGUAGE plpgsql
AS $$
    
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
        

CREATE OR REPLACE FUNCTION api.fun_auth_authorize(a_address varchar, a_token uuid, a_service enum_service, a_device_id varchar, a_device_os varchar, a_ip_address inet)
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
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_auth_basic_authenticate(a_address varchar, a_device_id varchar, a_device_os varchar, a_ip_address inet)
RETURNS table (
    "user_id" inet
)
LANGUAGE plpgsql
AS $$
    
DECLARE
  is_blocked_ boolean;
  user_id_    bigint;
BEGIN
    ASSERT (a_address NOTNULL AND a_device_id NOTNULL AND a_device_os NOTNULL AND
            a_ip_address NOTNULL);
    SELECT pkey_id, is_blocked
    INTO user_id_, is_blocked_
    FROM tbl.user
    WHERE address = LOWER(a_address);
    INSERT INTO tbl.login_attempt(fkey_user, address, ip_address,
                                  device_id, device_os, moment)
    VALUES (user_id_, a_address, a_ip_address, a_device_id, a_device_os, extract(epoch from now())::bigint);
    -- COMMIT;
    IF (user_id_ ISNULL) THEN
        RAISE SQLSTATE 'R0007'; -- UnknownUser
    ELSEIF (is_blocked_) THEN
        RAISE SQLSTATE 'R0008'; -- BlockedUser
    END IF;
    RETURN QUERY SELECT user_id_;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_list_users(a_offset int, a_limit int, a_user_id bigint DEFAULT NULL, a_email varchar DEFAULT NULL, a_username varchar DEFAULT NULL, a_role enum_role DEFAULT NULL)
RETURNS table (
    "user_id" bigint,
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
        
$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_assign_role(a_operator_user_id bigint, a_user_id bigint, a_new_role enum_role)
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
    UPDATE tbl.user SET role = a_new_role WHERE pkey_id = a_user_id;
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
        
