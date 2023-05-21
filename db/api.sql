CREATE SCHEMA IF NOT EXISTS api;

CREATE OR REPLACE FUNCTION api.fun_auth_signup(a_address varchar, a_email varchar, a_phone varchar, a_preferred_language varchar, a_agreed_tos boolean, a_agreed_privacy boolean, a_ip_address inet, a_username varchar DEFAULT NULL, a_age int DEFAULT NULL)
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
                          updated_at,
                          created_at)
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
    INSERT INTO tbl.authorization_attempt(fkey_user, ip_address, is_token_ok, moment)
    VALUES (user_id_, a_ip_address, is_token_ok_ NOTNULL AND is_token_ok_, extract(Epoch FROM (NOW()))::bigint);

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
    WHERE address = a_address;
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
        

CREATE OR REPLACE FUNCTION api.fun_admin_list_users(a_offset int, a_limit int, a_user_id bigint DEFAULT NULL, a_email varchar DEFAULT NULL, a_address varchar DEFAULT NULL, a_role enum_role DEFAULT NULL)
RETURNS table (
    "user_id" bigint,
    "email" varchar,
    "address" varchar,
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
        u.address,
        u.role,
        u.updated_at::int,
        u.created_at::int
    FROM tbl.user AS u
    WHERE a_user_id IS NOT NULL OR u.pkey_id = a_user_id
        AND a_email IS NOT NULL OR u.email = a_email
        AND a_address IS NOT NULL OR u.address = a_address
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
        

CREATE OR REPLACE FUNCTION api.fun_user_follow_strategy(a_user_id bigint, a_strategy_id bigint)
RETURNS table (
    "success" boolean
)
LANGUAGE plpgsql
AS $$
    

BEGIN
    IF EXISTS(SELECT 1
              FROM tbl.user_follow_strategy
              WHERE fkey_user_id = a_user_id
                AND fkey_strategy_id = a_strategy_id
                AND unfollowed = FALSE) THEN
        RETURN QUERY SELECT TRUE AS "select";
    END IF;

    INSERT INTO tbl.user_follow_strategy (fkey_user_id, fkey_strategy_id, created_at, updated_at)
    VALUES (a_user_id, a_strategy_id, extract(epoch from now())::bigint, extract(epoch from now())::bigint);

    RETURN QUERY SELECT TRUE AS "select";

END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_unfollow_strategy(a_user_id bigint, a_strategy_id bigint)
RETURNS table (
    "success" boolean
)
LANGUAGE plpgsql
AS $$
    

BEGIN
    UPDATE tbl.user_follow_strategy 
      SET unfollowed = TRUE,
          updated_at = extract(epoch from now())::bigint
      WHERE fkey_user_id = a_user_id
      AND fkey_strategy_id = a_strategy_id
      AND unfollowed = FALSE;
      
    RETURN QUERY SELECT TRUE AS "select";

END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_followed_strategies(a_user_id bigint)
RETURNS table (
    "strategy_id" bigint,
    "strategy_name" varchar,
    "strategy_description" varchar,
    "net_value" real,
    "followers" int,
    "backers" int,
    "risk_score" real,
    "aum" real
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT a.pkey_id AS strategy_id,
                          a.name AS strategy_name,
                          a.description AS strategy_description,
                          0.0 AS net_value,
                          (SELECT COUNT(*) FROM tbl.user_follow_strategy WHERE fkey_strategy_id = a.pkey_id AND unfollowed = FALSE) AS followers,
                          (SELECT COUNT(DISTINCT user_back_strategy_history.fkey_user_id) FROM tbl.user_back_strategy_history WHERE fkey_strategy_id = a.pkey_id) AS followers,
                          a.risk_score as risk_score,
                          a.aum as aum
                 FROM tbl.strategy AS a 
                     JOIN tbl.user_follow_strategy ON fkey_strategy_id = a.pkey_id WHERE fkey_user_id = a_user_id AND unfollowed = FALSE
                    ;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_strategies()
RETURNS table (
    "strategy_id" bigint,
    "strategy_name" varchar,
    "strategy_description" varchar,
    "net_value" real,
    "followers" int,
    "backers" int,
    "risk_score" real,
    "aum" real
)
LANGUAGE plpgsql
AS $$
    
BEGIN

    RETURN QUERY SELECT a.pkey_id AS strategy_id,
                          a.name AS strategy_name,
                          a.description AS strategy_description,
                          NULL AS net_value,
                          (SELECT COUNT(*) FROM tbl.user_follow_strategy WHERE fkey_strategy_id = a.pkey_id AND unfollowed = FALSE) AS followers,
                          (SELECT COUNT(DISTINCT user_back_strategy_history.fkey_user_id) FROM tbl.user_back_strategy_history WHERE fkey_strategy_id = a.pkey_id) AS followers,
                          a.risk_score as risk_score,
                          a.aum as aum
                 FROM tbl.strategy AS a 
                    ;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_get_strategy(a_strategy_id bigint)
RETURNS table (
    "strategy_id" bigint,
    "strategy_name" varchar,
    "strategy_description" varchar,
    "net_value" real,
    "followers" int,
    "backers" int,
    "risk_score" real,
    "aum" real
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT a.pkey_id AS strategy_id,
                          a.name AS strategy_name,
                          a.description AS strategy_description,
                          NULL AS net_value,
                          (SELECT COUNT(*) FROM tbl.user_follow_strategy WHERE fkey_strategy_id = a.pkey_id AND unfollowed = FALSE) AS followers,
                          (SELECT COUNT(DISTINCT user_back_strategy_history.fkey_user_id) FROM tbl.user_back_strategy_history WHERE fkey_strategy_id = a.pkey_id) AS followers,
                          a.risk_score as risk_score,
                          a.aum as aum
                 FROM tbl.strategy AS a
                 WHERE a.pkey_id = a_strategy_id;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_get_strategy_statistics_net_value(a_strategy_id bigint)
RETURNS table (
    "time" bigint,
    "net_value" real
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    -- TODO
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_get_strategy_statistics_follow_history(a_strategy_id bigint)
RETURNS table (
    "time" bigint,
    "follower_count" real
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    -- TODO
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_get_strategy_statistics_back_history(a_strategy_id bigint)
RETURNS table (
    "time" bigint,
    "backer_count" real,
    "backer_quantity_usd" real
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    -- TODO
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_back_strategy(a_user_id bigint, a_strategy_id bigint, a_quantity real, a_purchase_wallet varchar, a_blockchain varchar, a_dex varchar, a_transaction_hash varchar)
RETURNS table (
    "success" boolean
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    INSERT INTO tbl.user_back_strategy_history (fkey_user_id, fkey_strategy_id, quantity, purchase_wallet, blockchain, dex,
                                                transaction_hash, back_time)
    VALUES (a_user_id, a_strategy_id, a_quantity, a_purchase_wallet, a_blockchain, a_dex, a_transaction_hash,
            extract(epoch from now())::bigint);
    RETURN QUERY SELECT TRUE;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_backed_strategies(a_user_id bigint)
RETURNS table (
    "strategy_id" bigint,
    "strategy_name" varchar,
    "strategy_description" varchar,
    "net_value" real,
    "followers" int,
    "backers" int,
    "risk_score" real,
    "aum" real
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT a.pkey_id                            AS strategy_id,
                        a.name                               AS strategy_name,
                        a.description                        AS strategy_description,
                        NULL                                 AS net_value,
                        (SELECT COUNT(*)
                         FROM tbl.user_follow_strategy
                         WHERE fkey_strategy_id = a.pkey_id
                           AND unfollowed = FALSE)           AS followers,
                        (SELECT COUNT(DISTINCT user_back_strategy_history.fkey_user_id)
                         FROM tbl.user_back_strategy_history
                         WHERE fkey_strategy_id = a.pkey_id) AS followers,
                        a.risk_score                         as risk_score,
                        a.aum                                as aum
                 FROM tbl.strategy AS a
                          JOIN tbl.user_follow_strategy AS b ON b.fkey_strategy_id = a.pkey_id
                     AND b.fkey_user_id = a_user_id
                 WHERE unfollowed = FALSE;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_back_strategy_history(a_user_id bigint, a_strategy_id bigint DEFAULT NULL)
RETURNS table (
    "back_history_id" bigint,
    "strategy_id" bigint,
    "quantity" real,
    "wallet_address" varchar,
    "blockchain" varchar,
    "dex" varchar,
    "transaction_hash" varchar,
    "time" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT a.pkey_id          AS back_history_id,
                        a.fkey_strategy_id AS strategy_id,
                        a.quantity         AS quantity,
                        a.purchase_wallet  AS wallet_address,
                        a.blockchain       AS blockchain,
                        a.dex              AS dex,
                        a.transaction_hash AS transaction_hash,
                        a.time             AS time
                 FROM tbl.user_back_strategy_history AS a
                 WHERE a.fkey_user_id = a_user_id
                  AND (a_strategy_id NOTNULL OR a_strategy_id = a.fkey_strategy_id);
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_exit_strategy(a_user_id bigint, a_strategy_id bigint, a_quantity real, a_blockchain varchar, a_dex varchar, a_back_time bigint, a_transaction_hash varchar, a_purchase_wallet varchar)
RETURNS table (
    "success" boolean
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    INSERT INTO tbl.user_exit_strategy_history (fkey_user_id, fkey_strategy_id, exit_quantity, dex, back_time,
                                                exit_time, purchase_wallet, blockchain, transaction_hash)
    VALUES (a_user_id, a_strategy_id, a_quantity, a_dex, a_back_time, extract(epoch from now()), a_purchase_wallet,
            a_blockchain,
            a_transaction_hash);
    RETURN QUERY SELECT TRUE;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_exit_strategy_history(a_user_id bigint, a_strategy_id bigint DEFAULT NULL)
RETURNS table (
    "exit_history_id" bigint,
    "strategy_id" bigint,
    "exit_quantity" real,
    "purchase_wallet_address" varchar,
    "blockchain" varchar,
    "dex" varchar,
    "back_time" bigint,
    "exit_time" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN

    RETURN QUERY SELECT a.pkey_id AS exit_history_id,
                          a.fkey_strategy_id AS strategy_id,
                          a.exit_quantity AS exit_quantity,
                          a.purchase_wallet AS purchase_wallet_address,
                          a.blockchain AS blockchain,
                          a.dex AS dex,
                          a.back_time AS back_time,
                          a.time AS exit_time
                 FROM tbl.user_exit_strategy_history AS a
                 WHERE a.fkey_user_id = a_user_id AND (a.fkey_strategy_id = a_strategy_id OR a_strategy_id IS NULL);
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_follow_expert(a_user_id bigint, a_expert_id bigint)
RETURNS table (
    "success" boolean
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    INSERT INTO tbl.user_follow_expert (fkey_user_id, fkey_expert_id)
    VALUES (a_user_id, a_expert_id);
    RETURN QUERY SELECT TRUE;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_unfollow_expert(a_user_id bigint, a_expert_id bigint)
RETURNS table (
    "success" boolean
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    UPDATE tbl.user_follow_expert
    SET unfollowed = TRUE
    WHERE fkey_user_id = a_user_id AND fkey_expert_id = a_expert_id;
    RETURN QUERY SELECT TRUE;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_followed_experts(a_user_id bigint)
RETURNS table (
    "expert_id" bigint,
    "name" varchar,
    "follower_count" int,
    "description" varchar,
    "social_media" varchar,
    "risk_score" real,
    "reputation_score" real,
    "aum" real
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT a.pkey_id                                                 AS expert_id,
                        a.name                                                    AS name,
                        (SELECT COUNT(*)
                         FROM tbl.user_follow_expert
                         WHERE fkey_expert_id = a.pkey_id AND unfollowed = FALSE) AS follower_count,
                        a.description                                             AS description,
                        a.social_media                                            AS social_media,
                        a.risk_score                                              AS risk_score,
                        a.reputation_score                                        AS reputation_score,
                        a.aum                                                     AS aum
                 FROM tbl.expert_profile AS a
                          JOIN tbl.user_follow_expert AS b ON b.fkey_expert_id = a.pkey_id
                 WHERE b.fkey_user_id = a_user_id
                   AND unfollowed = FALSE;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_experts()
RETURNS table (
    "expert_id" bigint,
    "name" varchar,
    "follower_count" int,
    "description" varchar,
    "social_media" varchar,
    "risk_score" real,
    "reputation_score" real,
    "aum" real
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT a.pkey_id AS expert_id,
                          a.name AS name,
                          (SELECT COUNT(*) FROM tbl.user_follow_expert WHERE fkey_expert_id = a.pkey_id AND unfollowed = FALSE) AS follower_count,
                          a.description AS description,
                          a.social_media AS social_media,
                          a.risk_score AS risk_score,
                          a.reputation_score AS reputation_score,
                          a.aum AS aum
                 FROM tbl.expert_profile AS a;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_get_expert_profile(a_expert_id bigint)
RETURNS table (
    "expert_id" bigint,
    "name" varchar,
    "follower_count" int,
    "description" varchar,
    "social_media" varchar,
    "risk_score" real,
    "reputation_score" real,
    "aum" real
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT a.pkey_id AS expert_id,
                          a.name AS name,
                          (SELECT COUNT(*) FROM tbl.user_follow_expert WHERE fkey_expert_id = a.pkey_id AND unfollowed = FALSE) AS follower_count,
                          a.description AS description,
                          a.social_media AS social_media,
                          a.risk_score AS risk_score,
                          a.reputation_score AS reputation_score,
                          a.aum AS aum
                 FROM tbl.expert_profile AS a 
                 WHERE a.pkey_id = a_expert_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_get_user_profile(a_user_id bigint)
RETURNS table (
    "user_id" bigint,
    "name" varchar,
    "follower_count" int,
    "description" varchar,
    "social_media" varchar,
    "risk_score" real,
    "reputation_score" real,
    "aum" real
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT a.pkey_id                  AS expert_id,
                        a.name                     AS name,
                        (SELECT COUNT(*)
                         FROM tbl.user_follow_expert
                         WHERE fkey_expert_id = a.pkey_id
                           AND unfollowed = FALSE) AS follower_count,
                        ''                         AS description,
                        ''                         AS social_media,
                        0.0                        AS risk_score,
                        0.0                        AS reputation_score,
                        0.0                        AS aum
                 FROM tbl.user AS a
                 WHERE a.pkey_id = a_user_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_register_wallet(a_user_id bigint, a_blockchain varchar, a_wallet_address varchar)
RETURNS table (
    "success" boolean,
    "wallet_id" bigint
)
LANGUAGE plpgsql
AS $$
    
DECLARE
    a_wallet_id bigint;
BEGIN
    INSERT INTO tbl.user_wallet (fkey_user_id, blockchain, address)
    VALUES (a_user_id, a_blockchain, a_wallet_address)
    RETURNING pkey_id INTO a_wallet_id;
    RETURN QUERY SELECT TRUE, a_wallet_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_deregister_wallet(a_user_id bigint, a_wallet_id bigint)
RETURNS table (
    "success" boolean
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    DELETE
    FROM tbl.user_wallet
    WHERE fkey_user_id = a_user_id
      AND pkey_id = a_wallet_id;
    RETURN QUERY SELECT TRUE;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_wallets(a_user_id bigint)
RETURNS table (
    "wallet_id" bigint,
    "blockchain" varchar,
    "wallet_address" varchar,
    "is_default" boolean
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT a.pkey_id             AS wallet_id,
                        a.blockchain          AS blockchain,
                        a.address             AS wallet_address,
                        a.address = b.address AS is_default
                 FROM tbl.user_wallet AS a
                          JOIN tbl."user" AS b ON b.pkey_id = a.fkey_user_id
                 WHERE a.fkey_user_id = a_user_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_apply_become_expert(a_user_id bigint)
RETURNS table (
    "success" boolean
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    UPDATE tbl.user SET pending_expert = TRUE WHERE pkey_id = a_user_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_approve_user_become_admin(a_admin_user_id bigint, a_user_id bigint)
RETURNS table (
    "success" boolean
)
LANGUAGE plpgsql
AS $$
    
BEGIN
-- TODO: check permission and update tbl.user.role to expert
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_reject_user_become_admin(a_admin_user_id bigint, a_user_id bigint)
RETURNS table (
    "success" boolean
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    UPDATE tbl.user SET pending_expert = FALSE WHERE pkey_id = a_user_id;
    RETURN QUERY SELECT TRUE;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_list_pending_user_expert_applications()
RETURNS table (
    "user_id" bigint,
    "name" varchar,
    "follower_count" int,
    "description" varchar,
    "social_media" varchar,
    "risk_score" real,
    "reputation_score" real,
    "aum" real
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT a.pkey_id                  AS expert_id,
                        a.name                     AS name,
                        (SELECT COUNT(*)
                         FROM tbl.user_follow_expert
                         WHERE fkey_expert_id = a.pkey_id
                           AND unfollowed = FALSE) AS follower_count,
                        ''                         AS description,
                        ''                         AS social_media,
                        0.0                        AS risk_score,
                        0.0                        AS reputation_score,
                        0.0                        AS aum
                 FROM tbl."user" AS a
                 WHERE a.pending_expert = TRUE;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_create_strategy(a_user_id bigint, a_name varchar, a_description varchar)
RETURNS table (
    "success" boolean,
    "strategy_id" bigint
)
LANGUAGE plpgsql
AS $$
    
DECLARE
    a_strategy_id BIGINT;
BEGIN
    INSERT INTO tbl.strategy (fkey_user_id, name, description)
    VALUES (a_user_id, a_name, a_description) RETURNING pkey_id INTO a_strategy_id;
    RETURN QUERY SELECT TRUE, a_strategy_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_update_strategy(a_user_id bigint, a_strategy_id bigint, a_name varchar DEFAULT NULL, a_description varchar DEFAULT NULL)
RETURNS table (
    "success" boolean
)
LANGUAGE plpgsql
AS $$
    
            
BEGIN
    UPDATE tbl.strategy
    SET name = COALESCE(a_name, name),
        description = COALESCE(a_description, description)
    WHERE pkey_id = a_strategy_id
      AND fkey_user_id = a_user_id;
    RETURN QUERY SELECT TRUE;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_add_strategy_watch_wallet(a_user_id bigint, a_strategy_id bigint, a_wallet_address varchar, a_blockchain varchar, a_ratio real, a_dex varchar)
RETURNS table (
    "success" boolean,
    "watch_wallet_id" bigint
)
LANGUAGE plpgsql
AS $$
    
DECLARE
    a_watch_wallet_id BIGINT;
BEGIN
    INSERT INTO tbl.strategy_watching_wallet (fkey_user_id, fkey_strategy_id, address, blockchain, ratio_distribution,
                                              dex, created_at, updated_at)
    VALUES (a_user_id, a_strategy_id, a_wallet_address, a_blockchain, a_ratio, a_dex, extract(epoch FROM NOW()),
            extract(epoch from NOW()))
    RETURNING pkey_id INTO a_watch_wallet_id;
    RETURN QUERY SELECT TRUE, a_watch_wallet_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_remove_strategy_watch_wallet(a_user_id bigint, a_watch_wallet_id bigint)
RETURNS table (
    "success" boolean
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    DELETE FROM tbl.strategy_watching_wallet
    WHERE fkey_user_id = a_user_id
      AND pkey_id = a_watch_wallet_id;
    RETURN QUERY SELECT TRUE;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_strategy_watch_wallets(a_strategy_id bigint)
RETURNS table (
    "watch_wallet_id" bigint,
    "wallet_address" varchar,
    "blockchain" varchar,
    "ratio" real
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT a.pkey_id            AS watch_wallet_id,
                        a.address            AS wallet_address,
                        a.blockchain         AS blockchain,
                        a.ratio_distribution AS ratio
                 FROM tbl.strategy_watching_wallet AS a
                 WHERE a.fkey_strategy_id = a_strategy_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_watcher_save_raw_transaction(a_transaction_hash varchar, a_chain varchar, a_raw_transaction varchar, a_dex varchar DEFAULT NULL)
RETURNS table (
    "transaction_cache_id" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY INSERT INTO tbl.transaction_cache(transaction_hash,
                                                   chain,
                                                   dex,
                                                   raw_transaction,
                                                   created_at)
                 VALUES (a_transaction_hash,
                         a_chain,
                         a_dex,
                         a_raw_transaction,
                         extract(Epoch FROM (NOW()))::bigint)
                 RETURNING pkey_id;
END
        
$$;
        

CREATE OR REPLACE FUNCTION api.fun_watcher_get_raw_transaction(a_transaction_hash varchar, a_chain varchar, a_dex varchar DEFAULT NULL)
RETURNS table (
    "transaction_cache_id" bigint,
    "transaction_hash" varchar,
    "chain" varchar,
    "dex" varchar,
    "raw_transaction" varchar,
    "created_at" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT pkey_id,
                      transaction_hash,
                      chain,
                      dex,
                      raw_transaction,
                      created_at
                 FROM tbl.transaction_cache
                 WHERE transaction_hash = a_transaction_hash
                   AND chain = a_chain
                   AND (a_dex ISNULL OR dex = a_dex);
END
        
$$;
        

CREATE OR REPLACE FUNCTION api.fun_watcher_save_wallet_activity_history(a_address varchar, a_transaction_hash varchar, a_chain varchar, a_dex varchar, a_contract_address varchar, a_token_in_address varchar, a_token_out_address varchar, a_caller_address varchar, a_amount_in varchar, a_amount_out varchar, a_swap_calls jsonb, a_paths jsonb, a_dex_versions jsonb, a_created_at bigint DEFAULT NULL)
RETURNS table (
    "wallet_activity_history_id" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY INSERT INTO tbl.wallet_activity_history(
        address,
        transaction_hash,
        chain,
        dex,
        contract_address,
        token_in_address,
        token_out_address,
        caller_address,
        amount_in,
        amount_out,
        swap_calls,
        paths,
        dex_versions,
        created_at
    )
    VALUES (
        a_address,
        a_transaction_hash,
        a_chain,
        a_dex,
        a_contract_address,
        a_token_in_address,
        a_token_out_address,
        a_caller_address,
        a_amount_in,
        a_amount_out,
        a_swap_calls,
        a_paths,
        a_dex_versions,
        COALESCE(a_created_at, extract(Epoch FROM (NOW()))::bigint)
    )
    RETURNING pkey_id;
END
        
$$;
        

CREATE OR REPLACE FUNCTION api.fun_watcher_list_wallet_activity_history(a_address varchar, a_blockchain varchar)
RETURNS table (
    "wallet_activity_history_id" bigint,
    "address" varchar,
    "transaction_hash" varchar,
    "blockchain" varchar,
    "dex" varchar,
    "contract_address" varchar,
    "token_in_address" varchar,
    "token_out_address" varchar,
    "caller_address" varchar,
    "amount_in" varchar,
    "amount_out" varchar,
    "swap_calls" jsonb,
    "paths" jsonb,
    "dex_versions" jsonb,
    "created_at" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT pkey_id,
                      address,
                      transaction_hash,
                      blockchain,
                      dex,
                      contract_address,
                      token_in_address,
                      token_out_address,
                      caller_address,
                      amount_in,
                      amount_out,
                      swap_calls,
                      paths,
                      dex_versions,
                      created_at
                 FROM tbl.wallet_activity_history
                 WHERE address = a_address
                   AND blockchain = a_blockchain;
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
        

CREATE OR REPLACE FUNCTION api.ESCROW_WATCHER_SERVICE()
RETURNS table (
    "code" int
)
LANGUAGE plpgsql
AS $$
    BEGIN RETURN QUERY SELECT 4; END
$$;
        

CREATE OR REPLACE FUNCTION api.TRADE_WATCHER_SERVICE()
RETURNS table (
    "code" int
)
LANGUAGE plpgsql
AS $$
    BEGIN RETURN QUERY SELECT 5; END
$$;
        
