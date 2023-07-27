CREATE SCHEMA IF NOT EXISTS api;

CREATE OR REPLACE FUNCTION api.fun_auth_signup(a_address varchar, a_email varchar, a_phone varchar, a_preferred_language varchar, a_agreed_tos boolean, a_agreed_privacy boolean, a_ip_address inet, a_public_id bigint, a_username varchar DEFAULT NULL, a_age int DEFAULT NULL, a_ens_name varchar DEFAULT NULL, a_ens_avatar varchar DEFAULT NULL)
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
            extract(Epoch FROM (NOW()))::bigint,
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

        
$$;
        

CREATE OR REPLACE FUNCTION api.fun_auth_authenticate(a_address varchar, a_service_code int, a_device_id varchar, a_device_os varchar, a_ip_address inet)
RETURNS table (
    "user_id" bigint,
    "public_user_id" bigint,
    "role" enum_role,
    "ens_name" varchar,
    "ens_avatar" varchar
)
LANGUAGE plpgsql
AS $$
    
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
    RETURN QUERY SELECT u.pkey_id, u.public_id, u.role, u.ens_name, u.ens_avatar
    FROM tbl.user u
    WHERE address = a_address;
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
        

CREATE OR REPLACE FUNCTION api.fun_auth_remove_token(a_user_id bigint)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
  ASSERT (a_user_id NOTNULL);

  -- Setting up the token.
  UPDATE tbl.user
  SET user_token = NULL, admin_token = NULL
  WHERE pkey_id = a_user_id;

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
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_auth_set_role(a_public_user_id bigint, a_role enum_role)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    UPDATE tbl.user SET role = a_role WHERE public_id = a_public_user_id;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_auth_change_login_wallet_address(a_old_wallet_address varchar, a_new_wallet_address varchar)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    UPDATE tbl.user SET address = a_new_wallet_address,
                updated_at = EXTRACT(EPOCH FROM NOW())::bigint
     WHERE address = a_old_wallet_address;
    
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_auth_update_user_table(a_user_id bigint, a_username varchar DEFAULT NULL, a_family_name varchar DEFAULT NULL, a_given_name varchar DEFAULT NULL)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    UPDATE tbl.user 
    SET
        username = COALESCE(a_username, username),
        family_name = COALESCE(a_family_name, family_name),
        given_name = COALESCE(a_given_name, given_name),
        updated_at = EXTRACT(EPOCH FROM NOW())::bigint
     WHERE pkey_id = a_user_id;
    
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
        

CREATE OR REPLACE FUNCTION api.fun_user_list_followed_strategies(a_user_id bigint, a_limit bigint, a_offset bigint)
RETURNS table (
    "total" bigint,
    "strategy_id" bigint,
    "strategy_name" varchar,
    "strategy_description" varchar,
    "current_usdc" decimal(56, 18),
    "total_backed_usdc" decimal(56, 18),
    "total_exited_usdc" decimal(56, 18),
    "risk_score" double precision,
    "aum" double precision,
    "followers" bigint,
    "backers" bigint,
    "followed" boolean,
    "backed" boolean,
    "requested_at" bigint,
    "approved" boolean,
    "approved_at" bigint,
    "pending_approval" boolean,
    "created_at" bigint,
    "creator_public_id" bigint,
    "creator_id" bigint,
    "creator_username" varchar,
    "creator_family_name" varchar,
    "creator_given_name" varchar,
    "social_media" varchar,
    "immutable_audit_rules" boolean,
    "strategy_pool_token" varchar,
    "blockchain" enum_block_chain,
    "strategy_pool_address" varchar,
    "number_of_tokens" bigint,
    "swap_fee" double precision,
    "platform_fee" double precision,
    "expert_fee" double precision
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT 
                 DISTINCT ON (s.pkey_id)
                 count(*) OVER() AS total,
      s.pkey_id AS strategy_id,
      s.name AS strategy_name,
      s.description AS strategy_description,
      s.current_usdc,
      s.total_backed_usdc,
      s.total_exited_usdc,
      s.risk_score as risk_score,
      s.aum as aum,
      (SELECT count(*) FROM tbl.user_follow_strategy AS ufs WHERE ufs.fkey_strategy_id = s.pkey_id AND ufs.unfollowed = FALSE) AS followers,
      (SELECT COUNT(DISTINCT h.fkey_user_id) FROM tbl.user_back_exit_strategy_ledger AS h WHERE fkey_strategy_id = s.pkey_id) AS backers,
      TRUE as followed,
      EXISTS(SELECT 1 FROM tbl.user_back_exit_strategy_ledger AS ubesl WHERE ubesl.fkey_strategy_id = s.pkey_id AND ubesl.fkey_user_id = a_user_id) as backed,
      s.requested_at as requested_at,
      s.approved as approved,
      s.approved_at as approved_at,
      s.pending_approval as pending_approval,
      s.created_at as created_at,
      u.public_id as creator_public_id,
      u.pkey_id as creator_id,
      u.username as creator_username,
      u.family_name as creator_family_name,
      u.given_name as creator_given_name,
      s.social_media as social_media,
      s.immutable_audit_rules as immutable_audit_rules,
			-- sum all strategy pool tokens that user owns for this strategy on all chains
			(SELECT CAST(SUM(CAST(spt.balance AS NUMERIC)) AS VARCHAR)
			FROM tbl.user_strategy_balance AS spt
			JOIN tbl.strategy_pool_contract AS spc
			ON spt.fkey_strategy_pool_contract_id = spc.pkey_id
			JOIN tbl.user_strategy_wallet AS usw
			ON spt.fkey_user_strategy_wallet_id = usw.pkey_id
			WHERE spc.fkey_strategy_id = s.pkey_id AND usw.fkey_user_id = a_user_id) AS strategy_pool_token,
      s.blockchain,
      s.strategy_pool_address,
      (SELECT COUNT(*) FROM tbl.strategy_pool_contract_asset_balance AS sss
        JOIN tbl.strategy_pool_contract AS ss ON ss.pkey_id = sss.fkey_strategy_pool_contract_id
         WHERE ss.fkey_strategy_id = s.pkey_id
        ) AS number_of_tokens,
      s.swap_fee,
      (SELECT conf.platform_fee FROM tbl.system_config AS conf),
      s.expert_fee
      
                 FROM tbl.strategy AS s
                     JOIN tbl.user_follow_strategy AS b ON b.fkey_strategy_id = s.pkey_id
                     JOIN tbl.user AS u ON u.pkey_id = s.fkey_user_id
										 -- TODO: should "linked_wallet" be the expert watched wallet linked to the strategy?
										 JOIN tbl.expert_watched_wallet AS w ON w.fkey_user_id = u.pkey_id
                 WHERE b.fkey_user_id = a_user_id AND unfollowed = FALSE
                 -- TODO: filter only approved strategies
                ORDER BY s.pkey_id
                LIMIT a_limit
                OFFSET a_offset;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_strategies(a_user_id bigint, a_limit bigint, a_offset bigint, a_strategy_id bigint DEFAULT NULL, a_strategy_name varchar DEFAULT NULL, a_expert_id bigint DEFAULT NULL, a_expert_public_id bigint DEFAULT NULL, a_expert_name varchar DEFAULT NULL, a_description varchar DEFAULT NULL, a_blockchain enum_block_chain DEFAULT NULL, a_strategy_pool_address varchar DEFAULT NULL, a_approved boolean DEFAULT NULL)
RETURNS table (
    "total" bigint,
    "strategy_id" bigint,
    "strategy_name" varchar,
    "strategy_description" varchar,
    "current_usdc" decimal(56, 18),
    "total_backed_usdc" decimal(56, 18),
    "total_exited_usdc" decimal(56, 18),
    "risk_score" double precision,
    "aum" double precision,
    "followers" bigint,
    "backers" bigint,
    "followed" boolean,
    "backed" boolean,
    "requested_at" bigint,
    "approved" boolean,
    "approved_at" bigint,
    "pending_approval" boolean,
    "created_at" bigint,
    "creator_public_id" bigint,
    "creator_id" bigint,
    "creator_username" varchar,
    "creator_family_name" varchar,
    "creator_given_name" varchar,
    "social_media" varchar,
    "immutable_audit_rules" boolean,
    "strategy_pool_token" varchar,
    "blockchain" enum_block_chain,
    "strategy_pool_address" varchar,
    "number_of_tokens" bigint,
    "swap_fee" double precision,
    "platform_fee" double precision,
    "expert_fee" double precision
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT count(*) OVER() AS total,
      s.pkey_id AS strategy_id,
      s.name AS strategy_name,
      s.description AS strategy_description,
      s.current_usdc,
      s.total_backed_usdc,
      s.total_exited_usdc,
      s.risk_score as risk_score,
      s.aum as aum,
      (SELECT count(*) FROM tbl.user_follow_strategy AS ufs WHERE ufs.fkey_strategy_id = s.pkey_id AND ufs.unfollowed = FALSE) AS followers,
      (SELECT COUNT(DISTINCT h.fkey_user_id) FROM tbl.user_back_exit_strategy_ledger AS h WHERE fkey_strategy_id = s.pkey_id) AS backers,
      EXISTS(SELECT * FROM tbl.user_follow_strategy AS ufs WHERE ufs.fkey_strategy_id = s.pkey_id AND ufs.fkey_user_id = a_user_id AND ufs.unfollowed = FALSE) as followed,
      EXISTS(SELECT 1 FROM tbl.user_back_exit_strategy_ledger AS ubesl WHERE ubesl.fkey_strategy_id = s.pkey_id AND ubesl.fkey_user_id = a_user_id) as backed,
      s.requested_at as requested_at,
      s.approved as approved,
      s.approved_at as approved_at,
      s.pending_approval as pending_approval,
      s.created_at as created_at,
      u.public_id as creator_public_id,
      u.pkey_id as creator_id,
      u.username as creator_username,
      u.family_name as creator_family_name,
      u.given_name as creator_given_name,
      s.social_media as social_media,
      s.immutable_audit_rules as immutable_audit_rules,
			-- sum all strategy pool tokens that user owns for this strategy on all chains
			(SELECT CAST(SUM(CAST(spt.balance AS NUMERIC)) AS VARCHAR)
			FROM tbl.user_strategy_balance AS spt
			JOIN tbl.strategy_pool_contract AS spc
			ON spt.fkey_strategy_pool_contract_id = spc.pkey_id
			JOIN tbl.user_strategy_wallet AS usw
			ON spt.fkey_user_strategy_wallet_id = usw.pkey_id
			WHERE spc.fkey_strategy_id = s.pkey_id AND usw.fkey_user_id = a_user_id) AS strategy_pool_token,
      s.blockchain,
      s.strategy_pool_address,
      (SELECT COUNT(*) FROM tbl.strategy_pool_contract_asset_balance AS sss
        JOIN tbl.strategy_pool_contract AS ss ON ss.pkey_id = sss.fkey_strategy_pool_contract_id
         WHERE ss.fkey_strategy_id = s.pkey_id
        ) AS number_of_tokens,
      s.swap_fee,
      (SELECT conf.platform_fee FROM tbl.system_config AS conf),
      s.expert_fee
      
                 FROM tbl.strategy AS s
                        JOIN tbl.user AS u ON u.pkey_id = s.fkey_user_id
                        JOIN tbl.expert_watched_wallet AS w ON w.fkey_user_id = u.pkey_id
                 WHERE (a_strategy_id ISNULL OR s.pkey_id = a_strategy_id)
                    AND (a_strategy_name ISNULL OR s.name ILIKE a_strategy_name || '%')
                    AND (a_expert_id ISNULL OR u.pkey_id = a_expert_id)
                    AND (a_expert_public_id ISNULL OR u.public_id = a_expert_public_id)
                    AND (a_expert_name ISNULL OR u.username ILIKE a_expert_name || '%')
                    AND (a_description ISNULL OR s.description ILIKE a_description || '%')
                    AND (a_blockchain ISNULL OR s.blockchain = a_blockchain)
                    AND (a_strategy_pool_address ISNULL OR s.strategy_pool_address ILIKE a_strategy_pool_address || '%')
                    AND (a_approved ISNULL OR s.approved = a_approved)
                ORDER BY s.pkey_id
                LIMIT a_limit
                OFFSET a_offset;


END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_get_strategy_statistics_net_value(a_strategy_id bigint)
RETURNS table (
    "time" bigint,
    "net_value" double precision
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    -- TODO
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_get_strategy_statistics_follow_ledger(a_strategy_id bigint)
RETURNS table (
    "time" bigint,
    "follower_count" double precision
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    -- TODO
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_get_strategy_statistics_back_ledger(a_strategy_id bigint)
RETURNS table (
    "time" bigint,
    "backer_count" double precision,
    "backer_quantity_usd" double precision
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    -- TODO
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_back_strategy(a_user_id bigint, a_strategy_id bigint, a_quantity decimal(56, 18), a_new_total_backed_quantity decimal(56, 18), a_old_total_backed_quantity decimal(56, 18), a_new_current_quantity decimal(56, 18), a_old_current_quantity decimal(56, 18), a_blockchain enum_block_chain, a_transaction_hash varchar, a_earn_sp_tokens decimal(56, 18))
RETURNS table (
    "success" boolean
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    -- check if old total backed quantity is the one in strategy
    IF NOT EXISTS(SELECT * FROM tbl.strategy WHERE pkey_id = a_strategy_id AND total_backed_usdc = a_old_total_backed_quantity) THEN
        RETURN QUERY SELECT FALSE;
    END IF;
    -- update strategy total backed quantity
    UPDATE tbl.strategy SET total_backed_usdc = a_new_total_backed_quantity WHERE pkey_id = a_strategy_id;
    
    -- check if old current quantity is the one in strategy
    IF NOT EXISTS(SELECT * FROM tbl.strategy WHERE pkey_id = a_strategy_id AND current_usdc = a_old_current_quantity) THEN
        RETURN QUERY SELECT FALSE;
    END IF;
    -- update strategy current quantity
    UPDATE tbl.strategy SET current_usdc = a_new_current_quantity WHERE pkey_id = a_strategy_id;
    
    -- save record
    INSERT INTO tbl.user_back_exit_strategy_ledger (
			fkey_user_id,
			fkey_strategy_id,
			blockchain,
			quantity_of_usdc,
			quantity_sp_tokens,
			transaction_hash,
			happened_at,
			is_back
		) VALUES (
			a_user_id,
			a_strategy_id,
			a_blockchain,
			a_quantity,
			a_earn_sp_tokens,
			a_transaction_hash,
			extract(epoch from now())::bigint,
			TRUE);
    RETURN QUERY SELECT TRUE;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_backed_strategies(a_user_id bigint, a_offset bigint, a_limit bigint)
RETURNS table (
    "total" bigint,
    "strategy_id" bigint,
    "strategy_name" varchar,
    "strategy_description" varchar,
    "current_usdc" decimal(56, 18),
    "total_backed_usdc" decimal(56, 18),
    "total_exited_usdc" decimal(56, 18),
    "risk_score" double precision,
    "aum" double precision,
    "followers" bigint,
    "backers" bigint,
    "followed" boolean,
    "backed" boolean,
    "requested_at" bigint,
    "approved" boolean,
    "approved_at" bigint,
    "pending_approval" boolean,
    "created_at" bigint,
    "creator_public_id" bigint,
    "creator_id" bigint,
    "creator_username" varchar,
    "creator_family_name" varchar,
    "creator_given_name" varchar,
    "social_media" varchar,
    "immutable_audit_rules" boolean,
    "strategy_pool_token" varchar,
    "blockchain" enum_block_chain,
    "strategy_pool_address" varchar,
    "number_of_tokens" bigint,
    "swap_fee" double precision,
    "platform_fee" double precision,
    "expert_fee" double precision
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT DISTINCT ON (s.pkey_id) count(*) OVER() AS total,
      s.pkey_id AS strategy_id,
      s.name AS strategy_name,
      s.description AS strategy_description,
      s.current_usdc,
      s.total_backed_usdc,
      s.total_exited_usdc,
      s.risk_score as risk_score,
      s.aum as aum,
      (SELECT count(*) FROM tbl.user_follow_strategy AS ufs WHERE ufs.fkey_strategy_id = s.pkey_id AND ufs.unfollowed = FALSE) AS followers,
      (SELECT COUNT(DISTINCT h.fkey_user_id) FROM tbl.user_back_exit_strategy_ledger AS h WHERE fkey_strategy_id = s.pkey_id) AS backers,
      EXISTS(SELECT * FROM tbl.user_follow_strategy AS ufs WHERE ufs.fkey_strategy_id = s.pkey_id AND ufs.fkey_user_id = a_user_id AND ufs.unfollowed = FALSE) as followed,
      EXISTS(SELECT 1 FROM tbl.user_back_exit_strategy_ledger AS ubesl WHERE ubesl.fkey_strategy_id = s.pkey_id AND ubesl.fkey_user_id = a_user_id) as backed,
      s.requested_at as requested_at,
      s.approved as approved,
      s.approved_at as approved_at,
      s.pending_approval as pending_approval,
      s.created_at as created_at,
      u.public_id as creator_public_id,
      u.pkey_id as creator_id,
      u.username as creator_username,
      u.family_name as creator_family_name,
      u.given_name as creator_given_name,
      s.social_media as social_media,
      s.immutable_audit_rules as immutable_audit_rules,
			-- sum all strategy pool tokens that user owns for this strategy on all chains
			(SELECT CAST(SUM(CAST(spt.balance AS NUMERIC)) AS VARCHAR)
			FROM tbl.user_strategy_balance AS spt
			JOIN tbl.strategy_pool_contract AS spc
			ON spt.fkey_strategy_pool_contract_id = spc.pkey_id
			JOIN tbl.user_strategy_wallet AS usw
			ON spt.fkey_user_strategy_wallet_id = usw.pkey_id
			WHERE spc.fkey_strategy_id = s.pkey_id AND usw.fkey_user_id = a_user_id) AS strategy_pool_token,
      s.blockchain,
      s.strategy_pool_address,
      (SELECT COUNT(*) FROM tbl.strategy_pool_contract_asset_balance AS sss
        JOIN tbl.strategy_pool_contract AS ss ON ss.pkey_id = sss.fkey_strategy_pool_contract_id
         WHERE ss.fkey_strategy_id = s.pkey_id
        ) AS number_of_tokens,
      s.swap_fee,
      (SELECT conf.platform_fee FROM tbl.system_config AS conf),
      s.expert_fee
      
                 FROM tbl.strategy AS s
                      JOIN tbl.user_back_exit_strategy_ledger AS b ON b.fkey_strategy_id = s.pkey_id AND b.fkey_user_id = a_user_id
                      JOIN tbl.user AS u ON u.pkey_id = s.fkey_user_id
                      JOIN tbl.expert_watched_wallet AS w ON w.fkey_user_id = u.pkey_id
                 WHERE b.fkey_user_id = a_user_id
                 ORDER BY s.pkey_id
                 LIMIT a_limit
                 OFFSET a_offset;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_back_strategy_ledger(a_limit bigint, a_offset bigint, a_user_id bigint DEFAULT NULL, a_strategy_id bigint DEFAULT NULL)
RETURNS table (
    "total" bigint,
    "back_ledger_id" bigint,
    "user_id" bigint,
    "strategy_id" bigint,
    "quantity" decimal(56, 18),
    "blockchain" enum_block_chain,
    "transaction_hash" varchar,
    "happened_at" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT COUNT(*) OVER() AS total,
                        a.pkey_id          AS back_ledger_id,
                        a.fkey_user_id     AS user_id,
                        a.fkey_strategy_id AS strategy_id,
                        a.quantity_of_usdc         AS quantity,
                        a.blockchain       AS blockchain,
                        a.transaction_hash AS transaction_hash,
                        a.happened_at             AS happened_at
                 FROM tbl.user_back_exit_strategy_ledger AS a
                 WHERE (a_user_id ISNULL OR a.fkey_user_id = a_user_id)
                    AND (a_strategy_id ISNULL OR a_strategy_id = a.fkey_strategy_id)
                        AND a.is_back = TRUE
                 ORDER BY a.happened_at
                 LIMIT a_limit
                 OFFSET a_offset;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_exit_strategy_ledger(a_limit bigint, a_offset bigint, a_user_id bigint DEFAULT NULL, a_strategy_id bigint DEFAULT NULL)
RETURNS table (
    "total" bigint,
    "back_ledger_id" bigint,
    "user_id" bigint,
    "strategy_id" bigint,
    "quantity" decimal(56, 18),
    "blockchain" enum_block_chain,
    "transaction_hash" varchar,
    "happened_at" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT COUNT(*) OVER() AS total,
                        a.pkey_id          AS back_ledger_id,
                        a.fkey_user_id     AS user_id,
                        a.fkey_strategy_id AS strategy_id,
                        a.quantity_of_usdc         AS quantity,
                        a.blockchain       AS blockchain,
                        a.transaction_hash AS transaction_hash,
                        a.happened_at             AS happened_at
                 FROM tbl.user_back_exit_strategy_ledger AS a
                 WHERE (a_user_id ISNULL OR a.fkey_user_id = a_user_id)
                    AND (a_strategy_id ISNULL OR a_strategy_id = a.fkey_strategy_id)
                        AND a.is_back = FALSE
                 ORDER BY a.happened_at
                 LIMIT a_limit
                 OFFSET a_offset;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_exit_strategy(a_user_id bigint, a_strategy_id bigint, a_quantity decimal(56, 18), a_redeem_sp_tokens decimal(56, 18), a_blockchain enum_block_chain, a_transaction_hash varchar)
RETURNS table (
    "success" boolean
)
LANGUAGE plpgsql
AS $$
    
BEGIN

		-- update strategy total backed quantity
		UPDATE tbl.strategy AS s SET total_exited_usdc = CAST(s.total_exited_usdc AS NUMERIC) + CAST(a_quantity AS NUMERIC)
		WHERE pkey_id = a_strategy_id;

		-- update strategy current quantity
		UPDATE tbl.strategy AS s SET current_usdc = CAST(s.current_usdc AS NUMERIC) - CAST(a_quantity AS NUMERIC)
		WHERE pkey_id = a_strategy_id;

		-- save record
		INSERT INTO tbl.user_back_exit_strategy_ledger (
			fkey_user_id,
			fkey_strategy_id,
			blockchain,
			quantity_of_usdc,
			quantity_sp_tokens,
			transaction_hash,
			happened_at,
			is_back
		) VALUES (
			a_user_id,
			a_strategy_id,
			a_blockchain,
			a_quantity,
			a_redeem_sp_tokens,
			a_transaction_hash,
			extract(epoch from now())::bigint,
			FALSE);
		RETURN QUERY SELECT TRUE;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_user_strategy_pool_contract_asset_ledger_entries(a_user_id bigint, a_strategy_id bigint, a_blockchain enum_block_chain, a_limit bigint DEFAULT NULL, a_offset bigint DEFAULT NULL)
RETURNS table (
    "user_strategy_pool_contract_asset_ledger_id" bigint,
    "strategy_pool_contract_id" bigint,
    "strategy_id" bigint,
    "strategy_wallet_id" bigint,
    "strategy_wallet_address" varchar,
    "is_strategy_wallet_managed" boolean,
    "token_id" bigint,
    "token_symbol" varchar,
    "token_name" varchar,
    "token_address" varchar,
    "blockchain" enum_block_chain,
    "amount" decimal(56, 18),
    "happened_at" bigint,
    "is_add" boolean
)
LANGUAGE plpgsql
AS $$
    
DECLARE
	_strategy_pool_contract_id BIGINT;
BEGIN		
		SELECT spc.pkey_id INTO _strategy_pool_contract_id
		FROM tbl.strategy_pool_contract AS spc
		WHERE spc.fkey_strategy_id = a_strategy_id AND spc.blockchain = a_blockchain;

		ASSERT _strategy_pool_contract_id IS NOT NULL;

		RETURN QUERY
		WITH tokens AS (
			SELECT etca.pkey_id, etca.address, etca.symbol, etca.short_name, etca.blockchain
			FROM tbl.escrow_token_contract_address AS etca
		),

		strategy_wallets AS (
			SELECT usw.pkey_id, usw.fkey_user_id, usw.address, usw.blockchain, usw.is_platform_managed, usw.created_at
			FROM tbl.user_strategy_wallet as usw
			WHERE usw.fkey_user_id = a_user_id
		)

		SELECT
			uspcal.pkey_id AS user_strategy_pool_contract_asset_ledger_id,
			uspcal.fkey_strategy_pool_contract_id,
			a_strategy_id AS strategy_id,
			strategy_wallets.pkey_id AS strategy_wallet_id,
			strategy_wallets.address AS strategy_wallet_address,
			strategy_wallets.is_platform_managed AS is_strategy_wallet_managed,
			tokens.pkey_id AS token_id,
			tokens.symbol AS token_symbol,
			tokens.short_name AS token_name,
			tokens.address AS token_address,
			tokens.blockchain,
			uspcal.amount,
			uspcal.happened_at,
			uspcal.is_add
		FROM tbl.user_strategy_pool_contract_asset_ledger AS uspcal
		INNER JOIN tokens ON tokens.pkey_id = uspcal.fkey_token_id
		INNER JOIN strategy_wallets ON strategy_wallets.pkey_id = uspcal.fkey_strategy_wallet_id
		WHERE uspcal.fkey_strategy_pool_contract_id = _strategy_pool_contract_id
		ORDER BY uspcal.happened_at DESC
		LIMIT a_limit
		OFFSET a_offset;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_expert_listened_wallet_trade_ledger_entries(a_expert_listened_wallet_id bigint, a_limit bigint DEFAULT NULL, a_offset bigint DEFAULT NULL)
RETURNS table (
    "expert_listened_wallet_trade_ledger_id" bigint,
    "expert_listened_wallet_id" bigint,
    "blockchain" enum_block_chain,
    "transaction_hash" varchar,
    "dex" enum_dex,
    "token_in_id" bigint,
    "token_in_symbol" varchar,
    "token_in_address" varchar,
    "amount_in" decimal(56, 18),
    "token_out_id" bigint,
    "token_out_symbol" varchar,
    "token_out_address" varchar,
    "amount_out" decimal(56, 18),
    "happened_at" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
	RETURN QUERY

	WITH tokens AS (
		SELECT etca.pkey_id, etca.address, etca.symbol, etca.short_name, etca.blockchain
		FROM tbl.escrow_token_contract_address AS etca
	)
	
	SELECT
		swwtl.pkey_id AS expert_listened_wallet_trade_ledger_id,
		swwtl.expert_watched_wallet_id AS expert_listened_wallet_id,
		swwtl.blockchain,
		swwtl.transaction_hash,
		swwtl.dex,
		swwtl.fkey_token_in,
		token_in.symbol AS token_in_symbol,
		token_in.address AS token_in_address,
		swwtl.amount_in,
		swwtl.fkey_token_out,
		token_out.symbol AS token_out_symbol,
		token_out.address AS token_out_address,
		swwtl.amount_out,
		swwtl.heppened_at AS happened_at
	FROM tbl.strategy_watching_wallet_trade_ledger AS swwtl
	JOIN tokens AS token_in ON token_in.pkey_id = swwtl.fkey_token_in
	JOIN tokens AS token_out ON token_out.pkey_id = swwtl.fkey_token_out
	WHERE swwtl.expert_watched_wallet_id = a_expert_listened_wallet_id
	ORDER BY swwtl.heppened_at DESC
	LIMIT a_limit
	OFFSET a_offset;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_add_user_strategy_pool_contract_asset_ledger_entry(a_strategy_wallet_id bigint, a_strategy_pool_contract_id bigint, a_token_address varchar, a_blockchain enum_block_chain, a_amount decimal(56, 18), a_is_add boolean)
RETURNS table (
    "success" boolean
)
LANGUAGE plpgsql
AS $$
    
BEGIN
	INSERT INTO tbl.user_strategy_pool_contract_asset_ledger (
		fkey_strategy_wallet_id,
		fkey_strategy_pool_contract_id,
		fkey_token_id,
		amount,
		happened_at,
		is_add
	) VALUES (
		a_strategy_wallet_id,
		a_strategy_pool_contract_id,
		(SELECT pkey_id FROM tbl.escrow_token_contract_address AS etca WHERE etca.address = a_token_address AND etca.blockchain = a_blockchain),
		a_amount,
		extract(epoch from now())::bigint,
		a_is_add
	);

	RETURN QUERY SELECT TRUE;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_user_strategy_pool_contract_asset_balances(a_strategy_pool_contract_id bigint DEFAULT NULL, a_user_id bigint DEFAULT NULL, a_strategy_wallet_id bigint DEFAULT NULL, a_token_address varchar DEFAULT NULL, a_blockchain enum_block_chain DEFAULT NULL)
RETURNS table (
    "user_id" bigint,
    "strategy_wallet_id" bigint,
    "strategy_wallet_address" varchar,
    "is_strategy_wallet_managed" boolean,
    "token_id" bigint,
    "token_name" varchar,
    "token_symbol" varchar,
    "token_address" varchar,
    "token_decimals" int,
    "blockchain" enum_block_chain,
    "balance" decimal(56, 18)
)
LANGUAGE plpgsql
AS $$
    
BEGIN
	RETURN QUERY
		WITH tokens AS (
		SELECT etca.pkey_id, etca.address, etca.symbol, etca.short_name, etca.decimals, etca.blockchain
		FROM tbl.escrow_token_contract_address AS etca
	),

	strategy_wallets AS (
		SELECT usw.pkey_id, usw.fkey_user_id, usw.address, usw.blockchain, usw.is_platform_managed, usw.created_at
		FROM tbl.user_strategy_wallet as usw
		WHERE (a_user_id ISNULL OR usw.fkey_user_id = a_user_id)
			AND (a_blockchain ISNULL OR usw.blockchain = a_blockchain)
			AND (a_strategy_wallet_id ISNULL OR usw.pkey_id = a_strategy_wallet_id)
	)

	SELECT
		strategy_wallets.fkey_user_id AS user_id,
		strategy_wallets.pkey_id AS strategy_wallet_id,
		strategy_wallets.address AS strategy_wallet_address,
		strategy_wallets.is_platform_managed AS is_strategy_wallet_managed,
		tokens.pkey_id AS token_id,
		tokens.short_name AS token_name,
		tokens.symbol AS token_symbol,
		tokens.address AS token_address,
		tokens.decimals AS token_decimals,
		tokens.blockchain,
		uspcab.balance
	FROM tbl.user_strategy_pool_contract_asset_balance as uspcab
	INNER JOIN tokens ON tokens.pkey_id = uspcab.fkey_token_id
	INNER JOIN strategy_wallets ON strategy_wallets.pkey_id = uspcab.fkey_strategy_wallet_id
	WHERE (a_strategy_pool_contract_id ISNULL OR uspcab.fkey_strategy_pool_contract_id = a_strategy_pool_contract_id)
		AND (a_token_address ISNULL OR tokens.address = a_token_address)
		AND (a_blockchain ISNULL OR tokens.blockchain = a_blockchain)
		AND (a_user_id ISNULL OR strategy_wallets.fkey_user_id = a_user_id)
		AND (a_blockchain ISNULL OR strategy_wallets.blockchain = a_blockchain)
		AND (a_strategy_wallet_id ISNULL OR strategy_wallets.pkey_id = a_strategy_wallet_id);
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_upsert_user_strategy_pool_contract_asset_balance(a_strategy_wallet_id bigint, a_strategy_pool_contract_id bigint, a_token_address varchar, a_blockchain enum_block_chain, a_old_balance decimal(56, 18), a_new_balance decimal(56, 18))
RETURNS table (
    "user_strategy_pool_contract_asset_balance_id" bigint
)
LANGUAGE plpgsql
AS $$
    
DECLARE
		_token_id BIGINT;
		_user_strategy_pool_contract_asset_balance_id BIGINT;
		_user_strategy_pool_contract_asset_balance_old_balance decimal(56, 18);
		_pkey_id BIGINT;
BEGIN
		SELECT etca.pkey_id INTO _token_id
		FROM tbl.escrow_token_contract_address AS etca
		WHERE etca.address = a_token_address AND etca.blockchain = a_blockchain;

		ASSERT _token_id IS NOT NULL;

		SELECT uspcab.pkey_id, uspcab.balance
		INTO _user_strategy_pool_contract_asset_balance_id, _user_strategy_pool_contract_asset_balance_old_balance
		FROM tbl.user_strategy_pool_contract_asset_balance AS uspcab
		WHERE uspcab.fkey_strategy_wallet_id = a_strategy_wallet_id
			AND uspcab.fkey_strategy_pool_contract_id = a_strategy_pool_contract_id
			AND uspcab.fkey_token_id = _token_id;

		-- insert new entry if not exist
		IF _user_strategy_pool_contract_asset_balance_id ISNULL THEN
				INSERT INTO tbl.user_strategy_pool_contract_asset_balance (
					fkey_strategy_wallet_id,
					fkey_strategy_pool_contract_id,
					fkey_token_id,
					balance
				)	VALUES (
					a_strategy_wallet_id,
					a_strategy_pool_contract_id,
					_token_id,
					a_new_balance
				)	RETURNING pkey_id	INTO _pkey_id;
		ELSE
				-- update old balance if exist and equals to old balance
				IF _user_strategy_pool_contract_asset_balance_old_balance NOTNULL AND _user_strategy_pool_contract_asset_balance_old_balance != a_old_balance THEN
						RETURN;
				END IF;
				UPDATE tbl.user_strategy_pool_contract_asset_balance
				SET balance = a_new_balance
				WHERE pkey_id = _user_strategy_pool_contract_asset_balance_id
				RETURNING pkey_id
						INTO _pkey_id;
		END IF;

		RETURN QUERY SELECT _pkey_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_add_strategy_pool_contract_asset_ledger_entry(a_strategy_pool_contract_id bigint, a_token_address varchar, a_blockchain enum_block_chain, a_transaction_hash varchar, a_amount decimal(56, 18), a_is_add boolean)
RETURNS table (
    "success" boolean
)
LANGUAGE plpgsql
AS $$
    
DECLARE
		_strategy_id BIGINT;
		_token_id BIGINT;
BEGIN
		SELECT spc.fkey_strategy_id INTO _strategy_id
		FROM tbl.strategy_pool_contract AS spc
		WHERE spc.pkey_id = a_strategy_pool_contract_id;

		ASSERT _strategy_id IS NOT NULL;

		SELECT etca.pkey_id INTO _token_id
		FROM tbl.escrow_token_contract_address AS etca
		WHERE etca.address = a_token_address AND etca.blockchain = a_blockchain;

		ASSERT _token_id IS NOT NULL;

		INSERT INTO tbl.strategy_pool_contract_asset_ledger (
			fkey_strategy_id,
			fkey_token_id,
			blockchain,
			transaction_hash,
			amount,
			is_add,
			happened_at
	) VALUES (
			_strategy_id,
			_token_id,
			a_blockchain,
			a_transaction_hash,
			a_amount,
			a_is_add,
			EXTRACT(EPOCH FROM NOW())
		);

		RETURN QUERY SELECT TRUE;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_strategy_pool_contract_asset_ledger(a_limit bigint DEFAULT NULL, a_offset bigint DEFAULT NULL, a_strategy_id bigint DEFAULT NULL, a_token_id bigint DEFAULT NULL, a_blockchain enum_block_chain DEFAULT NULL)
RETURNS table (
    "entry_id" bigint,
    "strategy_id" bigint,
    "token_id" bigint,
    "token_symbol" varchar,
    "blockchain" enum_block_chain,
    "transaction_hash" varchar,
    "dex" varchar,
    "amount" decimal(56, 18),
    "is_add" boolean,
    "happened_at" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
RETURN QUERY SELECT
            spcal.pkey_id,
            spcal.fkey_strategy_id,
            spcal.fkey_token_id,
            etca.symbol,
            spcal.blockchain,
            spcal.transaction_hash,
            spcal.dex,
            spcal.amount,
            spcal.is_add,
            spcal.happened_at
        FROM tbl.strategy_pool_contract_asset_ledger AS spcal
        JOIN tbl.escrow_token_contract_address AS etca ON spcal.fkey_token_id = etca.pkey_id
        WHERE (a_strategy_id ISNULL OR spcal.fkey_strategy_id = a_strategy_id)
        AND (a_token_id ISNULL OR spcal.fkey_token_id = a_token_id)
        AND (a_blockchain ISNULL OR spcal.blockchain = a_blockchain)
        ORDER BY spcal.happened_at DESC
        LIMIT a_limit
        OFFSET a_offset;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_follow_expert(a_user_id bigint, a_expert_id bigint DEFAULT NULL, a_expert_public_id bigint DEFAULT NULL)
RETURNS table (
    "success" boolean
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    IF a_expert_id ISNULL THEN
        SELECT pkey_id INTO STRICT a_expert_id FROM tbl.expert_profile AS e
         JOIN tbl.user AS u ON e.pkey_id = e.fkey_user_id
         WHERE u.public_id = a_expert_public_id;
    END IF;
    INSERT INTO tbl.user_follow_expert (fkey_user_id, fkey_expert_id, updated_at, created_at)
    VALUES (a_user_id, a_expert_id, extract(epoch from now())::bigint, extract(epoch from now())::bigint);
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
        

CREATE OR REPLACE FUNCTION api.fun_user_list_followed_experts(a_user_id bigint, a_offset bigint, a_limit bigint)
RETURNS table (
    "total" bigint,
    "expert_id" bigint,
    "user_id" bigint,
    "user_public_id" bigint,
    "listening_wallet" varchar,
    "username" varchar,
    "family_name" varchar,
    "given_name" varchar,
    "follower_count" bigint,
    "backer_count" bigint,
    "strategy_count" bigint,
    "description" varchar,
    "social_media" varchar,
    "risk_score" double precision,
    "reputation_score" double precision,
    "aum" double precision,
    "joined_at" bigint,
    "requested_at" bigint,
    "approved_at" bigint,
    "pending_expert" boolean,
    "approved_expert" boolean,
    "followed" boolean,
    "linked_wallet" varchar
)
LANGUAGE plpgsql
AS $$
    
    BEGIN
        RETURN QUERY SELECT count(*) OVER() AS total,
        e.pkey_id                                                 AS expert_id,
        e.fkey_user_id                                            AS user_id,
        u.public_id                                               AS user_public_id,
        u.address                                                 AS listening_wallet,
        u.username                                                AS username,
        u.family_name                                             AS family_name,
        u.given_name                                              AS given_name,
        (SELECT COUNT(DISTINCT d.fkey_user_id) FROM tbl.user_follow_expert AS d WHERE d.fkey_expert_id = e.pkey_id AND unfollowed = FALSE) AS follower_count,
        (SELECT COUNT(DISTINCT d.fkey_user_id) FROM tbl.user_back_exit_strategy_ledger AS d JOIN tbl.strategy AS e ON e.pkey_id = d.fkey_strategy_id WHERE e.fkey_user_id = u.pkey_id) AS backer_count,
        (SELECT COUNT(DISTINCT d.fkey_user_id) FROM tbl.strategy AS d WHERE d.fkey_user_id = u.pkey_id) AS strategy_count,
        e.description                                             AS description,
        e.social_media                                            AS social_media,
        e.risk_score                                              AS risk_score,
        e.reputation_score                                        AS reputation_score,
        e.aum                                                     AS aum,
        u.created_at                                              AS joined_at,
        e.requested_at                                            AS requested_at,
        e.approved_at                                             AS approved_at,
        e.pending_expert                                          AS pending_expert,
        e.approved_expert                                         AS approved_expert,
        TRUE                                                AS followed,
        u.address                                                 AS linked_wallet
        
                    FROM tbl.expert_profile AS e
                      JOIN tbl.user_follow_expert AS b ON b.fkey_expert_id = e.pkey_id
                      JOIN tbl.user AS u ON u.pkey_id = e.fkey_user_id
                    WHERE b.fkey_user_id = a_user_id
                        AND unfollowed = FALSE
                    ORDER BY e.pkey_id
                    OFFSET a_offset
                    LIMIT a_limit
                    ;
    END
    
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_experts(a_limit bigint, a_offset bigint, a_user_id bigint, a_sort_by_followers boolean, a_expert_id bigint DEFAULT NULL, a_expert_user_id bigint DEFAULT NULL, a_expert_user_public_id bigint DEFAULT NULL, a_username varchar DEFAULT NULL, a_family_name varchar DEFAULT NULL, a_given_name varchar DEFAULT NULL, a_description varchar DEFAULT NULL, a_social_media varchar DEFAULT NULL)
RETURNS table (
    "total" bigint,
    "expert_id" bigint,
    "user_id" bigint,
    "user_public_id" bigint,
    "listening_wallet" varchar,
    "username" varchar,
    "family_name" varchar,
    "given_name" varchar,
    "follower_count" bigint,
    "backer_count" bigint,
    "strategy_count" bigint,
    "description" varchar,
    "social_media" varchar,
    "risk_score" double precision,
    "reputation_score" double precision,
    "aum" double precision,
    "joined_at" bigint,
    "requested_at" bigint,
    "approved_at" bigint,
    "pending_expert" boolean,
    "approved_expert" boolean,
    "followed" boolean,
    "linked_wallet" varchar
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT count(*) OVER() AS total,
        e.pkey_id                                                 AS expert_id,
        e.fkey_user_id                                            AS user_id,
        u.public_id                                               AS user_public_id,
        u.address                                                 AS listening_wallet,
        u.username                                                AS username,
        u.family_name                                             AS family_name,
        u.given_name                                              AS given_name,
        (SELECT COUNT(DISTINCT d.fkey_user_id) FROM tbl.user_follow_expert AS d WHERE d.fkey_expert_id = e.pkey_id AND unfollowed = FALSE) AS follower_count,
        (SELECT COUNT(DISTINCT d.fkey_user_id) FROM tbl.user_back_exit_strategy_ledger AS d JOIN tbl.strategy AS e ON e.pkey_id = d.fkey_strategy_id WHERE e.fkey_user_id = u.pkey_id) AS backer_count,
        (SELECT COUNT(DISTINCT d.fkey_user_id) FROM tbl.strategy AS d WHERE d.fkey_user_id = u.pkey_id) AS strategy_count,
        e.description                                             AS description,
        e.social_media                                            AS social_media,
        e.risk_score                                              AS risk_score,
        e.reputation_score                                        AS reputation_score,
        e.aum                                                     AS aum,
        u.created_at                                              AS joined_at,
        e.requested_at                                            AS requested_at,
        e.approved_at                                             AS approved_at,
        e.pending_expert                                          AS pending_expert,
        e.approved_expert                                         AS approved_expert,
        EXISTS(SELECT * FROM tbl.user_follow_expert AS ufe WHERE ufe.fkey_expert_id = e.pkey_id AND ufe.fkey_user_id = a_user_id AND unfollowed = FALSE)                                                AS followed,
        u.address                                                 AS linked_wallet
        
                 FROM tbl.expert_profile AS e
                   JOIN tbl.user AS u ON u.pkey_id = e.fkey_user_id
                 WHERE (a_expert_id ISNULL OR e.pkey_id = a_expert_id)
                        AND (a_expert_user_id ISNULL OR u.pkey_id = a_expert_user_id)
                        AND (a_expert_user_public_id ISNULL OR u.public_id = a_expert_user_public_id)
                        AND (a_username ISNULL OR u.username ILIKE a_username || '%')
                        AND (a_family_name ISNULL OR u.family_name ILIKE a_family_name || '%')
                        AND (a_given_name ISNULL OR u.given_name ILIKE a_given_name || '%')
                        AND (a_description ISNULL OR e.description ILIKE a_description || '%')
                        AND (a_social_media ISNULL OR e.social_media ILIKE a_social_media || '%')
                 ORDER BY CASE 
                  WHEN a_sort_by_followers = TRUE THEN follower_count 
                  ELSE e.pkey_id
                 END DESC
                 OFFSET a_offset
                 LIMIT a_limit
                 ;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_get_expert_profile(a_user_id bigint, a_expert_id bigint DEFAULT NULL, a_expert_public_id bigint DEFAULT NULL)
RETURNS table (
    "total" bigint,
    "expert_id" bigint,
    "user_id" bigint,
    "user_public_id" bigint,
    "listening_wallet" varchar,
    "username" varchar,
    "family_name" varchar,
    "given_name" varchar,
    "follower_count" bigint,
    "backer_count" bigint,
    "strategy_count" bigint,
    "description" varchar,
    "social_media" varchar,
    "risk_score" double precision,
    "reputation_score" double precision,
    "aum" double precision,
    "joined_at" bigint,
    "requested_at" bigint,
    "approved_at" bigint,
    "pending_expert" boolean,
    "approved_expert" boolean,
    "followed" boolean,
    "linked_wallet" varchar
)
LANGUAGE plpgsql
AS $$
    
BEGIN

    RETURN QUERY SELECT count(*) OVER() AS total,
        e.pkey_id                                                 AS expert_id,
        e.fkey_user_id                                            AS user_id,
        u.public_id                                               AS user_public_id,
        u.address                                                 AS listening_wallet,
        u.username                                                AS username,
        u.family_name                                             AS family_name,
        u.given_name                                              AS given_name,
        (SELECT COUNT(DISTINCT d.fkey_user_id) FROM tbl.user_follow_expert AS d WHERE d.fkey_expert_id = e.pkey_id AND unfollowed = FALSE) AS follower_count,
        (SELECT COUNT(DISTINCT d.fkey_user_id) FROM tbl.user_back_exit_strategy_ledger AS d JOIN tbl.strategy AS e ON e.pkey_id = d.fkey_strategy_id WHERE e.fkey_user_id = u.pkey_id) AS backer_count,
        (SELECT COUNT(DISTINCT d.fkey_user_id) FROM tbl.strategy AS d WHERE d.fkey_user_id = u.pkey_id) AS strategy_count,
        e.description                                             AS description,
        e.social_media                                            AS social_media,
        e.risk_score                                              AS risk_score,
        e.reputation_score                                        AS reputation_score,
        e.aum                                                     AS aum,
        u.created_at                                              AS joined_at,
        e.requested_at                                            AS requested_at,
        e.approved_at                                             AS approved_at,
        e.pending_expert                                          AS pending_expert,
        e.approved_expert                                         AS approved_expert,
        EXISTS(SELECT * FROM tbl.user_follow_expert AS ufe WHERE ufe.fkey_expert_id = e.pkey_id AND ufe.fkey_user_id = a_user_id AND unfollowed = FALSE)                                                AS followed,
        u.address                                                 AS linked_wallet
        
                 FROM tbl.expert_profile AS e
                 JOIN tbl.user AS u ON u.pkey_id = e.fkey_user_id
                 WHERE (a_expert_id ISNULL OR e.pkey_id = a_expert_id)
                    AND (a_expert_public_id ISNULL OR u.public_id = a_expert_public_id)
                 ;

END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_get_user_profile(a_user_id bigint)
RETURNS table (
    "expert_id" bigint,
    "user_public_id" bigint,
    "name" varchar,
    "login_wallet" varchar,
    "joined_at" bigint,
    "follower_count" bigint,
    "description" varchar,
    "social_media" varchar,
    "risk_score" double precision,
    "reputation_score" double precision,
    "aum" double precision
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT   e.pkey_id AS expert_id,
                          b.public_id AS user_public_id,
                          b.username AS name,
                          b.address AS login_wallet,
                          b.created_at AS joined_at,
                          (SELECT COUNT(*) FROM tbl.user_follow_expert WHERE fkey_expert_id = e.pkey_id AND unfollowed = FALSE) AS follower_count,
                          e.description AS description,
                          e.social_media AS social_media,
                          e.risk_score AS risk_score,
                          e.reputation_score AS reputation_score,
                          e.aum AS aum
                 FROM tbl.expert_profile AS e
                 RIGHT JOIN tbl.user AS b ON b.pkey_id = e.fkey_user_id
                 WHERE b.pkey_id = a_user_id;

END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_create_expert_profile(a_user_id bigint, a_description varchar DEFAULT NULL, a_social_media varchar DEFAULT NULL)
RETURNS table (
    "expert_id" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY INSERT INTO tbl.expert_profile(fkey_user_id, description, social_media, updated_at, created_at)
    VALUES(a_user_id, a_description, a_social_media, extract(epoch from now())::bigint, extract(epoch from now())::bigint) 
    RETURNING pkey_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_update_expert_profile(a_expert_id bigint, a_description varchar DEFAULT NULL, a_social_media varchar DEFAULT NULL)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    UPDATE tbl.expert_profile
    SET
        description = COALESCE(a_description, description),
        social_media = COALESCE(a_social_media, social_media),
        updated_at = extract(epoch from now())::bigint
     WHERE pkey_id = a_expert_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_apply_become_expert(a_user_id bigint)
RETURNS table (
    "success" boolean,
    "expert_id" bigint
)
LANGUAGE plpgsql
AS $$
    
DECLARE
    _expert_id bigint;
BEGIN
    IF NOT EXISTS(SELECT * FROM tbl.expert_profile WHERE fkey_user_id = a_user_id) THEN
        INSERT INTO tbl.expert_profile(fkey_user_id, pending_expert, requested_at, updated_at, created_at)
        VALUES(a_user_id, TRUE, extract(epoch from now())::bigint, extract(epoch from now())::bigint, extract(epoch from now())::bigint)
        RETURNING pkey_id INTO _expert_id;
    ELSE
        UPDATE tbl.expert_profile SET 
            pending_expert = TRUE,
            updated_at = extract(epoch from now())::bigint,
            requested_at = extract(epoch from now())::bigint
        WHERE fkey_user_id = a_user_id;
        SELECT pkey_id INTO _expert_id FROM tbl.expert_profile WHERE fkey_user_id = a_user_id;
    END IF;
    RETURN QUERY SELECT TRUE, _expert_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_create_strategy(a_user_id bigint, a_name varchar, a_description varchar, a_strategy_thesis_url varchar, a_minimum_backing_amount_usd double precision, a_swap_fee double precision, a_expert_fee double precision, a_agreed_tos boolean, a_wallet_address varchar, a_blockchain enum_block_chain)
RETURNS table (
    "success" boolean,
    "strategy_id" bigint
)
LANGUAGE plpgsql
AS $$
    
DECLARE
    a_strategy_id BIGINT;
		a_expert_watched_wallet_id BIGINT;
BEGIN
    INSERT INTO tbl.strategy (
        fkey_user_id, 
        name, 
        description,
        current_usdc, 
        total_backed_usdc, 
        total_exited_usdc, 
        strategy_thesis_url,
        minimum_backing_amount_usd,
        swap_fee,
        expert_fee,
        agreed_tos,
        updated_at, 
        created_at,
        pending_approval,
        approved,
        blockchain
    )
    VALUES (
        a_user_id, 
        a_name, 
        a_description, 
        '0', 
        '0', 
        '0', 
        a_strategy_thesis_url,
        a_minimum_backing_amount_usd,
        a_swap_fee,
        a_expert_fee,
        a_agreed_tos,
        EXTRACT(EPOCH FROM NOW())::bigint, 
        EXTRACT(EPOCH FROM NOW())::bigint,
        TRUE,
        FALSE,
        a_blockchain
    ) RETURNING pkey_id INTO a_strategy_id;

    -- if expert watched wallet already exists, fetch it's id
    -- TODO: add unique constraint to blockchain + address
    -- TODO: find out if one expert wallet can be watched for multiple strategies
    SELECT pkey_id
    INTO a_expert_watched_wallet_id
    FROM tbl.expert_watched_wallet
    WHERE fkey_user_id = a_user_id AND blockchain = a_blockchain AND address = a_wallet_address;

    -- if not, insert it and fetch it's id
    IF a_expert_watched_wallet_id IS NULL THEN
        INSERT INTO tbl.expert_watched_wallet(
            fkey_user_id,
            blockchain,
            address,
            created_at
        )
        VALUES (
            a_user_id,
            a_blockchain,
            a_wallet_address,
            EXTRACT(EPOCH FROM NOW())::bigint
        ) RETURNING pkey_id INTO a_expert_watched_wallet_id;
    END IF;

    INSERT INTO tbl.strategy_watched_wallet(
        fkey_expert_watched_wallet_id,
        fkey_strategy_id,
        ratio_distribution,
        updated_at,
        created_at
    ) VALUES (
        a_expert_watched_wallet_id,
        a_strategy_id,
        1.0,
        EXTRACT(EPOCH FROM NOW())::bigint,
        EXTRACT(EPOCH FROM NOW())::bigint
    );
    
    RETURN QUERY SELECT TRUE, a_strategy_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_update_strategy(a_user_id bigint, a_strategy_id bigint, a_name varchar DEFAULT NULL, a_description varchar DEFAULT NULL, a_social_media varchar DEFAULT NULL)
RETURNS table (
    "success" boolean
)
LANGUAGE plpgsql
AS $$
    
            
BEGIN
    UPDATE tbl.strategy
    SET name = COALESCE(a_name, name),
        description = COALESCE(a_description, description),
        social_media = COALESCE(a_social_media, social_media)
    WHERE pkey_id = a_strategy_id
      AND fkey_user_id = a_user_id;
    RETURN QUERY SELECT TRUE;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_add_strategy_watch_wallet(a_user_id bigint, a_strategy_id bigint, a_wallet_address varchar, a_blockchain enum_block_chain, a_ratio double precision, a_dex varchar)
RETURNS table (
    "success" boolean,
    "watch_wallet_id" bigint
)
LANGUAGE plpgsql
AS $$
    
DECLARE
    a_strategy_watched_wallet_id BIGINT;
		a_expert_watched_wallet_id BIGINT;
BEGIN
		-- if expert watched wallet already exists, fetch it's id
		-- TODO: add unique constraint to blockchain + address
		-- TODO: find out if one expert wallet can be watched for multiple strategies
		SELECT pkey_id
		INTO a_expert_watched_wallet_id
		FROM tbl.expert_watched_wallet
		WHERE fkey_user_id = a_user_id AND blockchain = a_blockchain AND address = a_wallet_address;

		-- if not, insert it and fetch it's id
		IF a_expert_watched_wallet_id IS NULL THEN
				INSERT INTO tbl.expert_watched_wallet(
						fkey_user_id,
						blockchain,
						address,
						created_at
				)
				VALUES (
						a_user_id,
						a_blockchain,
						a_wallet_address,
						EXTRACT(EPOCH FROM NOW())::bigint
				) RETURNING pkey_id INTO a_expert_watched_wallet_id;
		END IF;

		INSERT INTO tbl.strategy_watched_wallet (
			fkey_expert_watched_wallet_id,
			fkey_strategy_id,
			ratio_distribution,
			created_at,
			updated_at
		) VALUES (
			a_expert_watched_wallet_id,
			a_strategy_id,
			a_ratio,
			extract(epoch FROM NOW()),
			extract(epoch from NOW()))
		RETURNING pkey_id INTO a_strategy_watched_wallet_id;

    RETURN QUERY SELECT TRUE, a_strategy_watched_wallet_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_remove_strategy_watch_wallet(a_user_id bigint, a_strategy_id bigint, a_watch_wallet_id bigint)
RETURNS table (
    "success" boolean
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    DELETE FROM tbl.strategy_watched_wallet AS sww
    WHERE (SELECT fkey_user_id from tbl.expert_watched_wallet WHERE pkey_id = sww.fkey_expert_watched_wallet_id) = a_user_id
      AND pkey_id = a_watch_wallet_id
			AND fkey_strategy_id = a_strategy_id;
    RETURN QUERY SELECT TRUE;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_strategy_watch_wallets(a_strategy_id bigint)
RETURNS table (
    "strategy_watch_wallet_id" bigint,
    "expert_watch_wallet_id" bigint,
    "wallet_address" varchar,
    "blockchain" enum_block_chain,
    "ratio" double precision
)
LANGUAGE plpgsql
AS $$
    
BEGIN
		RETURN QUERY
		SELECT
				sw.pkey_id AS strategy_watch_wallet_id,
				sw.fkey_expert_watched_wallet_id AS expert_watch_wallet_id,
				ew.address AS wallet_address,
				ew.blockchain AS blockchain,
				sw.ratio_distribution AS ratio
		FROM
				tbl.strategy_watched_wallet AS sw
		JOIN
				tbl.expert_watched_wallet AS ew ON ew.pkey_id = sw.fkey_expert_watched_wallet_id
		WHERE
				sw.fkey_strategy_id = a_strategy_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_strategy_followers(a_strategy_id bigint)
RETURNS table (
    "total" bigint,
    "user_id" bigint,
    "user_public_id" bigint,
    "username" varchar,
    "wallet_address" varchar,
    "followed_at" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT 
                        COUNT(*) OVER () AS total,
                        a.fkey_user_id AS user_id,
                        b.public_id    AS user_public_id,
                        b.username     AS username,
                        b.address      AS wallet_address,
                        a.created_at   AS followed_at
                 FROM tbl.user_follow_strategy AS a
                          INNER JOIN tbl.user AS b ON a.fkey_user_id = b.pkey_id
                 WHERE a.fkey_strategy_id = a_strategy_id
                   AND a.unfollowed = FALSE;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_strategy_backers(a_strategy_id bigint)
RETURNS table (
    "user_id" bigint,
    "total" bigint,
    "user_public_id" bigint,
    "username" varchar,
    "wallet_address" varchar,
    "backed_at" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT 
                        DISTINCT ON(a.fkey_user_id) user_id,
                        COUNT(*) OVER () AS total,
                        b.public_id    AS user_public_id,
                        b.address      AS wallet_address,
                        b.username     AS username,
                        a.happened_at  AS backed_at
                 FROM tbl.user_back_exit_strategy_ledger AS a
                 JOIN tbl.user AS b ON a.fkey_user_id = b.pkey_id
                 WHERE a.fkey_strategy_id = a_strategy_id 
                     AND a.is_back = TRUE;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_add_whitelisted_wallet(a_user_id bigint, a_blockchain enum_block_chain, a_address varchar)
RETURNS table (
    "whitelisted_wallet_id" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY INSERT INTO tbl.user_whitelisted_wallet (fkey_user_id, blockchain, address, created_at)
            VALUES ( a_user_id, a_blockchain, a_address, EXTRACT(EPOCH FROM NOW())::bigint) RETURNING pkey_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_remove_whitelisted_wallet(a_whitelisted_wallet_id bigint, a_user_id bigint)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    DELETE FROM tbl.user_whitelisted_wallet WHERE pkey_id = a_whitelisted_wallet_id AND fkey_user_id = a_user_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_whitelisted_wallets(a_limit bigint, a_offset bigint, a_user_id bigint DEFAULT NULL, a_blockchain enum_block_chain DEFAULT NULL, a_address varchar DEFAULT NULL)
RETURNS table (
    "registered_wallet_id" bigint,
    "blockchain" enum_block_chain,
    "address" varchar
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT
        a.pkey_id,
        a.blockchain,
        a.address 
    FROM tbl.user_whitelisted_wallet AS a 
    WHERE (a.fkey_user_id = a_user_id OR a_user_id IS NULL) AND
          (a.blockchain = a_blockchain OR a_blockchain IS NULL) AND
          (a.address = a_address OR a_address IS NULL)
    ORDER BY a.pkey_id
    LIMIT a_limit
    OFFSET a_offset;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_add_user_deposit_withdraw_ledger_entry(a_user_id bigint, a_token_address varchar, a_blockchain enum_block_chain, a_user_address varchar, a_escrow_contract_address varchar, a_receiver_address varchar, a_quantity decimal(56, 18), a_transaction_hash varchar, a_is_deposit boolean, a_is_back boolean, a_is_withdraw boolean)
RETURNS table (
    "ledger_entry_id" bigint
)
LANGUAGE plpgsql
AS $$
    
DECLARE
	_existing_id bigint;
	_token_id bigint;
	_escrow_contract_id bigint;
BEGIN
	SELECT udwl.pkey_id INTO _existing_id
	FROM tbl.user_deposit_withdraw_ledger AS udwl
	WHERE udwl.transaction_hash = a_transaction_hash AND udwl.blockchain = a_blockchain
	LIMIT 1;

	IF _existing_id IS NOT NULL THEN
			RETURN QUERY SELECT _existing_id;
	END IF;

	SELECT etca.pkey_id INTO _token_id FROM tbl.escrow_token_contract_address AS etca
		WHERE etca.address = a_token_address AND etca.blockchain = a_blockchain;

	SELECT eca.pkey_id INTO _escrow_contract_id FROM tbl.escrow_contract_address AS eca
		WHERE eca.address = a_escrow_contract_address AND eca.blockchain = a_blockchain;

	ASSERT _token_id IS NOT NULL AND _escrow_contract_id IS NOT NULL;

	RETURN QUERY INSERT INTO tbl.user_deposit_withdraw_ledger (
		fkey_user_id,
		fkey_token_id,
		blockchain,
		user_address,
		escrow_contract_address,
		fkey_escrow_contract_address_id,
		receiver_address,
		quantity,
		transaction_hash,
		is_deposit,
		is_back,
		is_withdraw,
		happened_at
		) VALUES (a_user_id,
							_token_id,
							a_blockchain,
							a_user_address,
							a_escrow_contract_address,
							_escrow_contract_id,
							a_receiver_address,
							a_quantity,
							a_transaction_hash,
							a_is_deposit,
							a_is_back,
							a_is_withdraw,
							EXTRACT(EPOCH FROM NOW())::bigint
		) RETURNING pkey_id;

END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_request_refund(a_user_id bigint, a_token_id bigint, a_blockchain enum_block_chain, a_user_address varchar, a_contract_address varchar, a_contract_address_id bigint, a_receiver_address varchar, a_quantity decimal(56, 18), a_transaction_hash varchar)
RETURNS table (
    "request_refund_id" bigint
)
LANGUAGE plpgsql
AS $$
    
DECLARE
    existing_id bigint;
BEGIN
		SELECT pkey_id INTO existing_id
		FROM tbl.user_deposit_withdraw_ledger
		WHERE transaction_hash = a_transaction_hash AND
		blockchain = a_blockchain
		LIMIT 1;

		IF existing_id IS NOT NULL THEN
				RETURN QUERY SELECT existing_id;
		END IF;


    RETURN QUERY INSERT INTO tbl.user_deposit_withdraw_ledger (
        fkey_user_id,
        fkey_token_id,
        fkey_escrow_contract_address_id,
        blockchain,
        user_address,
        escrow_contract_address,
        receiver_address,
        quantity,
        transaction_hash,
        is_deposit,
        is_back,
        is_withdraw,
        happened_at
    ) VALUES (
     a_user_id,
     a_token_id,
     a_contract_address_id,
     a_blockchain,
     a_user_address,
     a_contract_address,
     a_receiver_address,
     a_quantity,
     a_transaction_hash,
     FALSE,
     FALSE,
     TRUE,
     EXTRACT(EPOCH FROM NOW())::bigint
    ) RETURNING pkey_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_request_refund_ledger(a_user_id bigint, a_limit bigint, a_offset bigint)
RETURNS table (
    "request_refund_id" bigint,
    "user_id" bigint,
    "blockchain" enum_block_chain,
    "quantity" decimal(56, 18),
    "wallet_address" varchar
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT a.pkey_id, a.fkey_user_id, a.blockchain, a.quantity, a.user_address
		FROM tbl.user_deposit_withdraw_ledger AS a
		WHERE fkey_user_id = a_user_id AND is_withdraw = FALSE
		ORDER BY a.pkey_id DESC
		LIMIT a_limit
		OFFSET a_offset;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_add_strategy_initial_token_ratio(a_strategy_id bigint, a_token_id bigint, a_quantity decimal(56, 18))
RETURNS table (
    "strategy_initial_token_ratio_id" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY INSERT INTO tbl.strategy_initial_token_ratio (fkey_strategy_id, token_id, quantity, created_at, updated_at)
            VALUES ( a_strategy_id, a_token_id, a_quantity, EXTRACT(EPOCH FROM NOW())::bigint, EXTRACT(EPOCH FROM NOW())::bigint) RETURNING pkey_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_update_strategy_initial_token_ratio(a_strategy_id bigint, a_token_id bigint, a_new_quantity decimal(56, 18))
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
		UPDATE tbl.strategy_initial_token_ratio
				SET quantity = a_new_quantity, updated_at = EXTRACT(EPOCH FROM NOW())::bigint
				WHERE fkey_strategy_id = a_strategy_id AND token_id = a_token_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_remove_strategy_initial_token_ratio(a_strategy_id bigint, a_token_id bigint)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    DELETE FROM tbl.strategy_initial_token_ratio 
    WHERE fkey_strategy_id = a_strategy_id AND token_id = a_token_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_strategy_initial_token_ratios(a_strategy_id bigint, a_token_id bigint DEFAULT NULL, a_token_address varchar DEFAULT NULL, a_blockchain enum_block_chain DEFAULT NULL)
RETURNS table (
    "total" bigint,
    "blockchain" enum_block_chain,
    "token_id" bigint,
    "token_name" varchar,
    "token_address" varchar,
    "token_decimals" int,
    "quantity" decimal(56, 18),
    "strategy_id" bigint,
    "created_at" bigint,
    "updated_at" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT
        COUNT(*) OVER() AS total,
        b.blockchain,
        a.token_id,
        b.short_name,
        b.address,
        b.decimals,
        a.quantity,
        a.fkey_strategy_id,
        a.updated_at,
        a.created_at 
    FROM tbl.strategy_initial_token_ratio AS a
    JOIN tbl.escrow_token_contract_address AS b ON a.token_id = b.pkey_id
    WHERE fkey_strategy_id = a_strategy_id
    AND (b.pkey_id = a_token_id OR a_token_id ISNULL)
    AND (b.address = a_token_address OR a_token_address ISNULL)
    AND (b.blockchain = a_blockchain OR a_blockchain ISNULL);
    
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_expert_list_followers(a_user_id bigint, a_limit bigint, a_offset bigint)
RETURNS table (
    "total" bigint,
    "public_id" bigint,
    "username" varchar,
    "family_name" varchar,
    "given_name" varchar,
    "linked_wallet" varchar,
    "followed_at" bigint,
    "joined_at" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT 
                DISTINCT ON(a.fkey_user_id)
                COUNT(*) OVER() AS total,
                b.pkey_id, 
                b.username, 
                b.family_name,
                b.given_name, 
                b.address,
                a.created_at, 
                b.created_at 
            FROM tbl.user_follow_expert AS a
            INNER JOIN tbl.user AS b ON a.fkey_user_id = b.pkey_id
            WHERE a.fkey_user_id = a_user_id
            ORDER BY a.fkey_user_id
            LIMIT a_limit
            OFFSET a_offset;

END            
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_expert_list_backers(a_user_id bigint, a_limit bigint, a_offset bigint)
RETURNS table (
    "total" bigint,
    "public_id" bigint,
    "username" varchar,
    "family_name" varchar,
    "given_name" varchar,
    "linked_wallet" varchar,
    "backed_at" bigint,
    "joined_at" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT
                DISTINCT ON(a.fkey_user_id)
                COUNT(*) OVER() AS total,
                b.pkey_id, 
                b.username, 
                b.family_name,
                b.given_name,
                b.address,
                a.happened_at,
                b.created_at
            FROM tbl.user_back_exit_strategy_ledger AS a
            INNER JOIN tbl.user AS b ON a.fkey_user_id = b.pkey_id
            WHERE a.fkey_user_id = a_user_id
            ORDER BY a.fkey_user_id
            LIMIT a_limit
            OFFSET a_offset;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_deposit_withdraw_ledger(a_limit bigint, a_offset bigint, a_user_id bigint DEFAULT NULL, a_is_deposit boolean DEFAULT NULL, a_is_back boolean DEFAULT NULL, a_is_withdraw boolean DEFAULT NULL, a_blockchain enum_block_chain DEFAULT NULL)
RETURNS table (
    "total" bigint,
    "transaction_id" bigint,
    "blockchain" enum_block_chain,
    "user_address" varchar,
    "contract_address" varchar,
    "receiver_address" varchar,
    "quantity" decimal(56, 18),
    "transaction_hash" varchar,
    "is_deposit" boolean,
    "happened_at" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT
            COUNT(*) OVER() AS total,
            a.pkey_id,
            a.blockchain, 
            a.user_address, 
            a.escrow_contract_address, 
            a.receiver_address, 
            a.quantity, 
            a.transaction_hash, 
            a.is_deposit,
            a.happened_at
		FROM tbl.user_deposit_withdraw_ledger AS a
		WHERE  (a.is_deposit = a_is_deposit OR a_is_deposit IS NULL)
		        AND (a.is_back = a_is_back OR a_is_back IS NULL)
		        AND (a.is_withdraw = a_is_withdraw OR a_is_withdraw IS NULL)
                AND (a.fkey_user_id = a_user_id OR a_user_id IS NULL)
                AND (a.blockchain = a_blockchain OR a_blockchain IS NULL)
		ORDER BY a.pkey_id DESC
		LIMIT a_limit
		OFFSET a_offset;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_get_user_by_address(a_address varchar, a_blockchain enum_block_chain)
RETURNS table (
    "user_id" bigint,
    "user_public_id" bigint,
    "username" varchar,
    "family_name" varchar,
    "given_name" varchar,
    "joined_at" bigint
)
LANGUAGE plpgsql
AS $$
    
DECLARE
		_user_id BIGINT;
BEGIN
		-- search for user id in user table by address
		SELECT pkey_id INTO _user_id FROM tbl.user AS a WHERE a.address = a_address;

		-- if address is not registered in user table, search for a whitelisted wallet
		IF _user_id IS NULL THEN
					SELECT uww.fkey_user_id INTO _user_id FROM tbl.user_whitelisted_wallet AS uww
							WHERE uww.address = a_address AND uww.blockchain = a_blockchain;
		END IF;

		-- if address not whitelisted, search for previously whitelisted addresses in the ledger
		IF _user_id IS NULL THEN
					SELECT udwl.fkey_user_id INTO _user_id FROM tbl.user_deposit_withdraw_ledger AS udwl
							WHERE udwl.user_address = a_address AND udwl.blockchain = a_blockchain;
		END IF;

		ASSERT _user_id IS NOT NULL;

    RETURN QUERY SELECT 
            a.pkey_id, 
            a.public_id,
            a.username, 
            a.family_name,
            a.given_name, 
            a.created_at 
            FROM tbl.user AS a WHERE a.pkey_id = _user_id;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_add_strategy_wallet(a_user_id bigint, a_blockchain enum_block_chain, a_address varchar, a_is_platform_managed boolean)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    INSERT INTO tbl.user_strategy_wallet (fkey_user_id, blockchain, address, is_platform_managed, created_at) 
    VALUES (a_user_id, a_blockchain, a_address, a_is_platform_managed, EXTRACT(EPOCH FROM NOW())::BIGINT);
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_strategy_wallets(a_user_id bigint DEFAULT NULL, a_strategy_wallet_address varchar DEFAULT NULL, a_blockchain enum_block_chain DEFAULT NULL)
RETURNS table (
    "total" bigint,
    "wallet_id" bigint,
    "user_id" bigint,
    "blockchain" enum_block_chain,
    "address" varchar,
    "is_platform_managed" boolean,
    "created_at" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT 
        COUNT(*) OVER() AS total,
        a.pkey_id,
				a.fkey_user_id,
        a.blockchain,
        a.address, 
        a.is_platform_managed,
        a.created_at 
    FROM tbl.user_strategy_wallet AS a 
    WHERE (a_user_id ISNULL OR a.fkey_user_id = a_user_id)
				AND (a_strategy_wallet_address ISNULL OR a.address = a_strategy_wallet_address)
        AND (a_blockchain ISNULL OR a.blockchain = a_blockchain);
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_strategy_audit_rules(a_strategy_id bigint, a_audit_rule_id bigint DEFAULT NULL)
RETURNS table (
    "rule_id" bigint,
    "created_at" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT a.fkey_audit_rule_id, a.created_at
    FROM tbl.strategy_audit_rule AS a
    WHERE a.fkey_strategy_id = a_strategy_id
    AND (a_audit_rule_id ISNULL OR a.fkey_audit_rule_id = a_audit_rule_id);
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_add_strategy_audit_rule(a_strategy_id bigint, a_audit_rule_id bigint)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    INSERT INTO tbl.strategy_audit_rule (fkey_strategy_id, fkey_audit_rule_id, updated_at, created_at)
    VALUES (a_strategy_id, a_audit_rule_id, EXTRACT(EPOCH FROM NOW())::BIGINT, EXTRACT(EPOCH FROM NOW())::BIGINT);
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_del_strategy_audit_rule(a_strategy_id bigint, a_audit_rule_id bigint)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    DELETE FROM tbl.strategy_audit_rule AS a
    WHERE a.fkey_strategy_id = a_strategy_id
    AND a.fkey_audit_rule_id = a_audit_rule_id;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_add_strategy_whitelisted_token(a_strategy_id bigint, a_token_name varchar)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    INSERT INTO tbl.strategy_whitelisted_token (fkey_strategy_id, token_name)
    VALUES (a_strategy_id, a_token_name);
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_strategy_whitelisted_tokens(a_strategy_id bigint)
RETURNS table (
    "token_name" varchar
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT a.token_name
    FROM tbl.strategy_whitelisted_token AS a
    WHERE a.fkey_strategy_id = a_strategy_id;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_check_if_token_whitelisted(a_strategy_id bigint, a_token_name varchar)
RETURNS table (
    "whitelisted" boolean
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    -- if a strategy does not have whitelist, it is considered as all tokens are whitelisted
    IF NOT EXISTS(
        SELECT 1
        FROM tbl.strategy_whitelisted_token AS a
        WHERE a.fkey_strategy_id = a_strategy_id
    ) THEN
        RETURN QUERY SELECT TRUE;
        RETURN;
    END IF;
    RETURN QUERY SELECT EXISTS(
        SELECT 1
        FROM tbl.strategy_whitelisted_token AS a
        WHERE a.fkey_strategy_id = a_strategy_id
            AND a.token_name = a_token_name
    );
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_audit_rules(a_audit_rule_id bigint DEFAULT NULL)
RETURNS table (
    "rule_id" bigint,
    "name" varchar,
    "description" varchar
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT a.pkey_id, a.name, a.description
    FROM tbl.audit_rule AS a
    WHERE (a_audit_rule_id ISNULL OR a.pkey_id = a_audit_rule_id);
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_get_strategy_id_from_watching_wallet(a_blockchain enum_block_chain, a_address varchar)
RETURNS table (
    "strategy_id" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT sww.fkey_strategy_id
    FROM tbl.strategy_watched_wallet AS sww
    JOIN tbl.expert_watched_wallet AS eww ON sww.fkey_expert_watched_wallet_id = eww.pkey_id
    WHERE eww.blockchain = a_blockchain
        AND eww.address = a_address;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_user_deposit_withdraw_balance(a_user_id bigint, a_limit bigint DEFAULT NULL, a_offset bigint DEFAULT NULL, a_user_address varchar DEFAULT NULL, a_blockchain enum_block_chain DEFAULT NULL, a_token_address varchar DEFAULT NULL, a_token_id bigint DEFAULT NULL, a_escrow_contract_address varchar DEFAULT NULL)
RETURNS table (
    "deposit_withdraw_balance_id" bigint,
    "user_id" bigint,
    "user_address" varchar,
    "blockchain" enum_block_chain,
    "token_id" bigint,
    "token_symbol" varchar,
    "token_name" varchar,
    "balance" decimal(56, 18)
)
LANGUAGE plpgsql
AS $$
    
DECLARE
    _token_id bigint;
BEGIN
    IF a_token_address IS NOT NULL THEN
        SELECT pkey_id INTO _token_id FROM tbl.escrow_token_contract_address AS a WHERE a.address = a_token_address AND a.blockchain = a_blockchain;
    ELSE
        _token_id := a_token_id;
    END IF;
   
    RETURN QUERY SELECT
        a.pkey_id,
        a.fkey_user_id,
				a.user_address,
        etc.blockchain,
        etc.pkey_id,
        etc.symbol,
        etc.short_name,
        a.balance
    FROM tbl.user_deposit_withdraw_balance AS a
    JOIN tbl.escrow_token_contract_address AS etc ON etc.pkey_id = a.fkey_token_id
    JOIN tbl.escrow_contract_address AS eca ON eca.pkey_id = a.fkey_escrow_contract_address_id
    WHERE a.fkey_user_id = a_user_id
				AND (a_user_address ISNULL OR a.user_address = a_user_address)
        AND (a_blockchain ISNULL OR etc.blockchain = a_blockchain)
        AND (a_token_address iSNULL OR etc.address = a_token_address)
        AND (a_escrow_contract_address ISNULL OR eca.address = a_escrow_contract_address)
    ORDER BY a.pkey_id
    LIMIT a_limit
    OFFSET a_offset;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_update_user_deposit_withdraw_balance(a_deposit_withdraw_balance_id bigint, a_old_balance decimal(56, 18), a_new_balance decimal(56, 18))
RETURNS table (
    "updated" boolean
)
LANGUAGE plpgsql
AS $$
    
DECLARE
    _old_balance decimal(56, 18);
BEGIN
    SELECT balance INTO _old_balance FROM tbl.user_deposit_withdraw_balance WHERE pkey_id = a_deposit_withdraw_balance_id;
    IF _old_balance <> a_old_balance THEN
        RETURN QUERY SELECT FALSE;
    END IF;
    UPDATE tbl.user_deposit_withdraw_balance SET balance = a_new_balance WHERE pkey_id = a_deposit_withdraw_balance_id;
    RETURN QUERY SELECT TRUE;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_add_strategy_pool_contract(a_strategy_id bigint, a_blockchain enum_block_chain, a_address varchar)
RETURNS table (
    "strategy_pool_contract_id" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    UPDATE tbl.strategy
    SET strategy_pool_address = a_address
    WHERE pkey_id = a_strategy_id
        AND strategy_pool_address ISNULL;
    RETURN QUERY INSERT INTO tbl.strategy_pool_contract (fkey_strategy_id, blockchain, address, created_at)
    VALUES (a_strategy_id, a_blockchain, a_address, EXTRACT(EPOCH FROM NOW()))
    RETURNING pkey_id;
END
        
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_escrow_token_contract_address(a_limit bigint, a_offset bigint, a_token_id bigint DEFAULT NULL, a_blockchain enum_block_chain DEFAULT NULL, a_address varchar DEFAULT NULL, a_symbol varchar DEFAULT NULL, a_is_stablecoin boolean DEFAULT NULL)
RETURNS table (
    "total" bigint,
    "token_id" bigint,
    "blockchain" enum_block_chain,
    "address" varchar,
    "symbol" varchar,
    "short_name" varchar,
    "description" varchar,
    "is_stablecoin" boolean,
    "decimals" int
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT
        COUNT(*) OVER() AS total,
        a.pkey_id,
        a.blockchain,
        a.address,
        a.symbol,
        a.short_name,
        a.description,
        a.is_stablecoin,
        a.decimals
    FROM tbl.escrow_token_contract_address AS a
    WHERE (a_token_id ISNULL OR a.pkey_id = a_token_id)
        AND (a_blockchain ISNULL OR a.blockchain = a_blockchain)
        AND (a_address ISNULL OR a.address = a_address)
        AND (a_symbol ISNULL OR a.symbol = a_symbol)
        AND (a_is_stablecoin ISNULL OR a.is_stablecoin = a_is_stablecoin)
    ORDER BY a.pkey_id
    LIMIT a_limit
    OFFSET a_offset;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_user_strategy_balance(a_limit bigint, a_offset bigint, a_user_id bigint, a_strategy_id bigint DEFAULT NULL)
RETURNS table (
    "total" bigint,
    "strategy_id" bigint,
    "strategy_name" varchar,
    "balance" decimal(56, 18),
    "user_strategy_wallet_address" varchar,
    "blockchain" enum_block_chain
)
LANGUAGE plpgsql
AS $$
    
BEGIN
            RETURN QUERY SELECT 
                    COUNT(*) OVER() AS total,
                    spc.fkey_strategy_id,
                    s.name,
                    spt.balance,
                    usw.address,
                    s.blockchain
			FROM tbl.user_strategy_balance AS spt
			JOIN tbl.strategy_pool_contract AS spc ON spt.fkey_strategy_pool_contract_id = spc.pkey_id
			JOIN tbl.user_strategy_wallet AS usw ON spt.fkey_user_strategy_wallet_id = usw.pkey_id
			JOIN tbl.strategy AS s on s.pkey_id = spc.fkey_strategy_id
			WHERE (a_strategy_id ISNULL OR spc.fkey_strategy_id = a_strategy_id)
			AND usw.fkey_user_id = a_user_id
			ORDER BY spt.pkey_id
			OFFSET a_offset
			LIMIT a_limit
			;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_save_user_back_strategy_attempt(a_strategy_id bigint, a_user_id bigint, a_token_id bigint, a_back_quantity decimal(56, 18), a_strategy_wallet_address varchar, a_log_id bigint)
RETURNS table (
    "user_back_strategy_attempt_id" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY INSERT INTO tbl.user_back_strategy_attempt (fkey_strategy_id, fkey_user_id, fkey_token_id, back_quantity, strategy_wallet_address, log_id, happened_at)
        VALUES (a_strategy_id, a_user_id, a_token_id, a_back_quantity, a_strategy_wallet_address, a_log_id, EXTRACT(EPOCH FROM NOW()))
        RETURNING pkey_id;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_user_back_strategy_attempt(a_limit bigint, a_offset bigint, a_user_id bigint DEFAULT NULL, a_strategy_id bigint DEFAULT NULL, a_token_id bigint DEFAULT NULL)
RETURNS table (
    "total" bigint,
    "user_back_strategy_attempt_id" bigint,
    "strategy_id" bigint,
    "strategy_name" varchar,
    "token_id" bigint,
    "token_symbol" varchar,
    "back_quantity" decimal(56, 18),
    "strategy_wallet_address" varchar,
    "log_id" bigint,
    "happened_at" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT
        COUNT(*) OVER() AS total,
        a.pkey_id,
        a.fkey_strategy_id,
        s.name,
        a.fkey_token_id,
        t.symbol,
        a.back_quantity,
        a.strategy_wallet_address,
        a.log_id,
        a.happened_at
    FROM tbl.user_back_strategy_attempt AS a
    JOIN tbl.strategy AS s ON a.fkey_strategy_id = s.pkey_id
    JOIN tbl.escrow_token_contract_address AS t ON a.fkey_token_id = t.pkey_id
    WHERE (a_strategy_id ISNULL OR a.fkey_strategy_id = a_strategy_id)
        AND (a_token_id ISNULL OR a.fkey_token_id = a_token_id)
        AND (a_user_id ISNULL OR a.fkey_user_id = a_user_id)
    ORDER BY a.pkey_id
    LIMIT a_limit
    OFFSET a_offset;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_save_user_back_strategy_log(a_user_back_strategy_attempt_id bigint, a_message varchar)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    INSERT INTO tbl.user_back_strategy_log (fkey_user_back_strategy_attempt_id, message, happened_at)
        VALUES (a_user_back_strategy_attempt_id, a_message, EXTRACT(EPOCH FROM NOW()));
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_user_back_strategy_log(a_limit bigint, a_offset bigint, a_user_back_strategy_attempt_id bigint)
RETURNS table (
    "total" bigint,
    "log_entry_id" bigint,
    "message" varchar,
    "happened_at" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT
        COUNT(*) OVER() AS total,
        l.pkey_id,
        l.message,
        l.happened_at
    FROM tbl.user_back_strategy_log AS l
    WHERE l.fkey_user_back_strategy_attempt_id = a_user_back_strategy_attempt_id
    ORDER BY l.pkey_id
    LIMIT a_limit
    OFFSET a_offset;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_list_users(a_limit bigint, a_offset bigint, a_user_id bigint DEFAULT NULL, a_address varchar DEFAULT NULL, a_username varchar DEFAULT NULL, a_email varchar DEFAULT NULL, a_role enum_role DEFAULT NULL)
RETURNS table (
    "total" bigint,
    "user_id" bigint,
    "public_user_id" bigint,
    "username" varchar,
    "address" varchar,
    "last_ip" inet,
    "last_login_at" bigint,
    "login_count" int,
    "role" enum_role,
    "email" varchar,
    "updated_at" bigint,
    "created_at" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY
    SELECT
        count(*) OVER() AS total,
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

        
$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_set_user_role(a_user_id bigint, a_role enum_role)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    UPDATE tbl.user SET role = a_role WHERE pkey_id = a_user_id;
END;
        
$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_set_block_user(a_user_id bigint, a_blocked boolean)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    UPDATE tbl.user SET is_blocked = a_blocked WHERE pkey_id = a_user_id;
END;
        
$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_approve_user_become_expert(a_user_public_id bigint)
RETURNS table (
    "success" boolean
)
LANGUAGE plpgsql
AS $$
    

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

$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_reject_user_become_expert(a_user_public_id bigint)
RETURNS table (
    "success" boolean
)
LANGUAGE plpgsql
AS $$
    

DECLARE
    _user_id bigint;
BEGIN
    SELECT pkey_id INTO _user_id FROM tbl.user WHERE public_id = a_user_public_id;
    UPDATE tbl.expert_profile SET pending_expert = FALSE, approved_expert = FALSE WHERE fkey_user_id = _user_id;
    RETURN QUERY SELECT TRUE;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_list_pending_user_expert_applications(a_limit bigint, a_offset bigint)
RETURNS table (
    "total" bigint,
    "user_public_id" bigint,
    "name" varchar,
    "linked_wallet" varchar,
    "follower_count" bigint,
    "description" varchar,
    "social_media" varchar,
    "risk_score" double precision,
    "reputation_score" double precision,
    "aum" double precision,
    "pending_expert" boolean,
    "approved_expert" boolean,
    "joined_at" bigint,
    "requested_at" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT COUNT(*) OVER() AS total,
                        a.public_id                AS user_public_id,
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

$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_get_system_config(a_config_id bigint)
RETURNS table (
    "platform_fee" double precision,
    "allow_domain_urls" varchar
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT
        a.platform_fee,
        a.allow_domain_urls
    FROM
        tbl.system_config a
    WHERE
        a.pkey_id = a_config_id;
END            

$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_update_system_config(a_config_id bigint, a_platform_fee double precision DEFAULT NULL, a_allow_domain_urls varchar DEFAULT NULL)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    IF NOT EXISTS (SELECT * FROM tbl.system_config WHERE pkey_id = a_config_id) THEN
        INSERT INTO tbl.system_config (pkey_id, platform_fee, allow_domain_urls)
        VALUES (a_config_id, a_platform_fee, a_allow_domain_urls);
    ELSE
        UPDATE tbl.system_config SET
            platform_fee = coalesce(a_platform_fee, platform_fee),
            allow_domain_urls = coalesce(a_allow_domain_urls, allow_domain_urls)
        WHERE
            pkey_id = a_config_id;
    END IF;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_list_experts(a_limit bigint, a_offset bigint, a_expert_id bigint DEFAULT NULL, a_user_id bigint DEFAULT NULL, a_user_public_id bigint DEFAULT NULL, a_username varchar DEFAULT NULL, a_family_name varchar DEFAULT NULL, a_given_name varchar DEFAULT NULL, a_description varchar DEFAULT NULL, a_social_media varchar DEFAULT NULL)
RETURNS table (
    "total" bigint,
    "expert_id" bigint,
    "user_id" bigint,
    "user_public_id" bigint,
    "listening_wallet" varchar,
    "username" varchar,
    "family_name" varchar,
    "given_name" varchar,
    "follower_count" bigint,
    "backer_count" bigint,
    "strategy_count" bigint,
    "description" varchar,
    "social_media" varchar,
    "risk_score" double precision,
    "reputation_score" double precision,
    "aum" double precision,
    "joined_at" bigint,
    "requested_at" bigint,
    "approved_at" bigint,
    "pending_expert" boolean,
    "approved_expert" boolean,
    "followed" boolean,
    "linked_wallet" varchar
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT count(*) OVER() AS total,
        e.pkey_id                                                 AS expert_id,
        e.fkey_user_id                                            AS user_id,
        u.public_id                                               AS user_public_id,
        u.address                                                 AS listening_wallet,
        u.username                                                AS username,
        u.family_name                                             AS family_name,
        u.given_name                                              AS given_name,
        (SELECT COUNT(DISTINCT d.fkey_user_id) FROM tbl.user_follow_expert AS d WHERE d.fkey_expert_id = e.pkey_id AND unfollowed = FALSE) AS follower_count,
        (SELECT COUNT(DISTINCT d.fkey_user_id) FROM tbl.user_back_exit_strategy_ledger AS d JOIN tbl.strategy AS e ON e.pkey_id = d.fkey_strategy_id WHERE e.fkey_user_id = u.pkey_id) AS backer_count,
        (SELECT COUNT(DISTINCT d.fkey_user_id) FROM tbl.strategy AS d WHERE d.fkey_user_id = u.pkey_id) AS strategy_count,
        e.description                                             AS description,
        e.social_media                                            AS social_media,
        e.risk_score                                              AS risk_score,
        e.reputation_score                                        AS reputation_score,
        e.aum                                                     AS aum,
        u.created_at                                              AS joined_at,
        e.requested_at                                            AS requested_at,
        e.approved_at                                             AS approved_at,
        e.pending_expert                                          AS pending_expert,
        e.approved_expert                                         AS approved_expert,
        EXISTS(SELECT * FROM tbl.user_follow_expert AS ufe WHERE ufe.fkey_expert_id = e.pkey_id AND ufe.fkey_user_id = a_user_id AND unfollowed = FALSE)                                                AS followed,
        u.address                                                 AS linked_wallet
        
                 FROM tbl.expert_profile AS e
                   JOIN tbl.user AS u ON u.pkey_id = e.fkey_user_id
                 WHERE (a_expert_id ISNULL OR e.pkey_id = a_expert_id)
                        AND (a_user_id ISNULL OR u.pkey_id = a_user_id)
                        AND (a_user_public_id ISNULL OR u.public_id = a_user_public_id)
                        AND (a_username ISNULL OR u.username ILIKE a_username || '%')
                        AND (a_family_name ISNULL OR u.family_name ILIKE a_family_name || '%')
                        AND (a_given_name ISNULL OR u.given_name ILIKE a_given_name || '%')
                        AND (a_description ISNULL OR e.description ILIKE a_description || '%')
                        AND (a_social_media ISNULL OR e.social_media ILIKE a_social_media || '%')
                        AND e.approved_expert = TRUE
                 ORDER BY e.pkey_id
                 OFFSET a_offset
                 LIMIT a_limit;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_list_backers(a_offset bigint, a_limit bigint, a_user_id bigint DEFAULT NULL, a_user_public_id bigint DEFAULT NULL, a_username varchar DEFAULT NULL, a_family_name varchar DEFAULT NULL, a_given_name varchar DEFAULT NULL)
RETURNS table (
    "total" bigint,
    "user_id" bigint,
    "user_public_id" bigint,
    "username" varchar,
    "login_wallet_address" varchar,
    "joined_at" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT COUNT(*) OVER() AS total,
                        a.pkey_id AS user_id,
                        a.public_id AS user_public_id,
                        a.username AS username,
                        a.address AS login_wallet_address,
                        a.created_at AS joined_at
                 FROM tbl.user AS a
                 JOIN tbl.user_back_exit_strategy_ledger AS b ON b.fkey_user_id = a.pkey_id
                WHERE (a_user_id ISNULL OR a.pkey_id = a_user_id)
                        AND (a_user_public_id ISNULL OR a.public_id = a_user_public_id)
                        AND (a_username ISNULL OR a.username ILIKE a_username || '%')
                        AND (a_family_name ISNULL OR a.family_name ILIKE a_family_name || '%')
                        AND (a_given_name ISNULL OR a.given_name ILIKE a_given_name || '%')
                 ORDER BY a.pkey_id
                 OFFSET a_offset
                 LIMIT a_limit;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_list_strategies(a_limit bigint, a_offset bigint, a_strategy_id bigint DEFAULT NULL, a_strategy_name varchar DEFAULT NULL, a_expert_public_id bigint DEFAULT NULL, a_expert_name varchar DEFAULT NULL, a_description varchar DEFAULT NULL, a_approved boolean DEFAULT NULL, a_pending_approval boolean DEFAULT NULL)
RETURNS table (
    "total" bigint,
    "strategy_id" bigint,
    "strategy_name" varchar,
    "strategy_description" varchar,
    "current_usdc" decimal(56, 18),
    "total_backed_usdc" decimal(56, 18),
    "total_exited_usdc" decimal(56, 18),
    "risk_score" double precision,
    "aum" double precision,
    "followers" bigint,
    "backers" bigint,
    "followed" boolean,
    "backed" boolean,
    "requested_at" bigint,
    "approved" boolean,
    "approved_at" bigint,
    "pending_approval" boolean,
    "created_at" bigint,
    "creator_public_id" bigint,
    "creator_id" bigint,
    "creator_username" varchar,
    "creator_family_name" varchar,
    "creator_given_name" varchar,
    "social_media" varchar,
    "immutable_audit_rules" boolean,
    "strategy_pool_token" varchar,
    "blockchain" enum_block_chain,
    "strategy_pool_address" varchar,
    "number_of_tokens" bigint,
    "swap_fee" double precision,
    "platform_fee" double precision,
    "expert_fee" double precision
)
LANGUAGE plpgsql
AS $$
    
DECLARE
    a_user_id bigint = NULL;
BEGIN
    RETURN QUERY SELECT count(*) OVER() AS total,
      s.pkey_id AS strategy_id,
      s.name AS strategy_name,
      s.description AS strategy_description,
      s.current_usdc,
      s.total_backed_usdc,
      s.total_exited_usdc,
      s.risk_score as risk_score,
      s.aum as aum,
      (SELECT count(*) FROM tbl.user_follow_strategy AS ufs WHERE ufs.fkey_strategy_id = s.pkey_id AND ufs.unfollowed = FALSE) AS followers,
      (SELECT COUNT(DISTINCT h.fkey_user_id) FROM tbl.user_back_exit_strategy_ledger AS h WHERE fkey_strategy_id = s.pkey_id) AS backers,
      EXISTS(SELECT * FROM tbl.user_follow_strategy AS ufs WHERE ufs.fkey_strategy_id = s.pkey_id AND ufs.fkey_user_id = a_user_id AND ufs.unfollowed = FALSE) as followed,
      EXISTS(SELECT 1 FROM tbl.user_back_exit_strategy_ledger AS ubesl WHERE ubesl.fkey_strategy_id = s.pkey_id AND ubesl.fkey_user_id = a_user_id) as backed,
      s.requested_at as requested_at,
      s.approved as approved,
      s.approved_at as approved_at,
      s.pending_approval as pending_approval,
      s.created_at as created_at,
      u.public_id as creator_public_id,
      u.pkey_id as creator_id,
      u.username as creator_username,
      u.family_name as creator_family_name,
      u.given_name as creator_given_name,
      s.social_media as social_media,
      s.immutable_audit_rules as immutable_audit_rules,
			-- sum all strategy pool tokens that user owns for this strategy on all chains
			(SELECT CAST(SUM(CAST(spt.balance AS NUMERIC)) AS VARCHAR)
			FROM tbl.user_strategy_balance AS spt
			JOIN tbl.strategy_pool_contract AS spc
			ON spt.fkey_strategy_pool_contract_id = spc.pkey_id
			JOIN tbl.user_strategy_wallet AS usw
			ON spt.fkey_user_strategy_wallet_id = usw.pkey_id
			WHERE spc.fkey_strategy_id = s.pkey_id AND usw.fkey_user_id = a_user_id) AS strategy_pool_token,
      s.blockchain,
      s.strategy_pool_address,
      (SELECT COUNT(*) FROM tbl.strategy_pool_contract_asset_balance AS sss
        JOIN tbl.strategy_pool_contract AS ss ON ss.pkey_id = sss.fkey_strategy_pool_contract_id
         WHERE ss.fkey_strategy_id = s.pkey_id
        ) AS number_of_tokens,
      s.swap_fee,
      (SELECT conf.platform_fee FROM tbl.system_config AS conf),
      s.expert_fee
      
                 FROM tbl.strategy AS s
                      JOIN tbl.user AS u ON u.pkey_id = s.fkey_user_id
                          
                WHERE (a_strategy_id ISNULL OR s.pkey_id = a_strategy_id)
                    AND (a_strategy_name ISNULL OR s.name ILIKE a_strategy_name || '%')
                    AND (a_expert_public_id ISNULL OR u.public_id = a_expert_public_id)
                    AND (a_expert_name ISNULL OR u.username ILIKE a_expert_name || '%')
                    AND (a_description ISNULL OR s.description ILIKE a_description || '%')
                    AND (a_approved ISNULL OR s.approved = a_approved)
                    AND (a_pending_approval ISNULL OR s.pending_approval = a_pending_approval)
                 ORDER BY s.pkey_id
                 OFFSET a_offset
                 LIMIT a_limit;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_approve_strategy(a_strategy_id bigint)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    UPDATE tbl.strategy
       SET approved = TRUE,
           pending_approval = FALSE,
           approved_at = EXTRACT(EPOCH FROM NOW())::bigint,
           updated_at = EXTRACT(EPOCH FROM NOW())::bigint
     WHERE pkey_id = a_strategy_id;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_reject_strategy(a_strategy_id bigint)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    UPDATE tbl.strategy
       SET approved = FALSE,
           pending_approval = FALSE,
           approved_at = NULL,
           updated_at = EXTRACT(EPOCH FROM NOW())::bigint
     WHERE pkey_id = a_strategy_id;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_add_audit_rule(a_rule_id bigint, a_name varchar, a_description varchar)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    INSERT INTO tbl.audit_rule (pkey_id, name, description)
         VALUES (a_rule_id, a_name, a_description);
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_add_escrow_token_contract_address(a_symbol varchar, a_short_name varchar, a_description varchar, a_address varchar, a_blockchain enum_block_chain, a_decimals int, a_is_stablecoin boolean, a_is_wrapped boolean, a_pkey_id bigint DEFAULT NULL)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    IF EXISTS (SELECT 1 FROM tbl.escrow_token_contract_address WHERE blockchain = a_blockchain AND symbol = a_symbol) THEN
        UPDATE tbl.escrow_token_contract_address 
        SET short_name = a_short_name,
            description = a_description,
            address = a_address,
            is_stablecoin = a_is_stablecoin,
            decimals = a_decimals
        WHERE blockchain = a_blockchain AND symbol = a_symbol;
    ELSE
        INSERT INTO tbl.escrow_token_contract_address (pkey_id, symbol, short_name, description, address, blockchain, is_stablecoin, decimals)
             VALUES (a_pkey_id, a_symbol, a_short_name, a_description, a_address, a_blockchain, a_is_stablecoin, a_decimals);
    END IF;

END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_update_escrow_token_contract_address(a_pkey_id bigint, a_symbol varchar DEFAULT NULL, a_short_name varchar DEFAULT NULL, a_description varchar DEFAULT NULL, a_is_stablecoin boolean DEFAULT NULL, a_is_wrapped boolean DEFAULT NULL)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    UPDATE tbl.escrow_token_contract_address
            SET symbol = coalesce(a_symbol, symbol),
                short_name = coalesce(a_short_name, short_name),
                description = coalesce(a_description, description),
                is_stablecoin = coalesce(a_is_stablecoin, is_stablecoin),
                is_wrapped = coalesce(a_is_wrapped, is_wrapped)
            WHERE pkey_id = a_pkey_id;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_list_escrow_token_contract_address(a_limit bigint DEFAULT NULL, a_offset bigint DEFAULT NULL, a_blockchain enum_block_chain DEFAULT NULL, a_token_address varchar DEFAULT NULL, a_symbol varchar DEFAULT NULL, a_token_id bigint DEFAULT NULL)
RETURNS table (
    "total" bigint,
    "pkey_id" bigint,
    "symbol" varchar,
    "short_name" varchar,
    "description" varchar,
    "address" varchar,
    "blockchain" enum_block_chain,
    "decimals" int,
    "is_stablecoin" boolean,
    "is_wrapped" boolean
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT
                count(*) OVER() AS total,
				etca.pkey_id,
				etca.symbol,
				etca.short_name,
				etca.description,
				etca.address,
				etca.blockchain,
				etca.decimals,
				etca.is_stablecoin,
				FALSE
			FROM tbl.escrow_token_contract_address AS etca
			WHERE (a_blockchain ISNULL OR etca.blockchain = a_blockchain)
			AND (a_token_address ISNULL OR etca.address = a_token_address)
			AND (a_token_id ISNULL OR etca.pkey_id = a_token_id)
			AND (a_symbol ISNULL OR etca.symbol ILIKE a_symbol || '%')
			ORDER BY etca.pkey_id
			OFFSET a_offset
			LIMIT a_limit;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_add_escrow_contract_address(a_pkey_id bigint, a_blockchain enum_block_chain, a_address varchar)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    INSERT INTO tbl.escrow_contract_address (pkey_id, blockchain, address)
         VALUES (a_pkey_id, a_blockchain, a_address);
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_list_escrow_contract_address(a_limit bigint, a_offset bigint, a_blockchain enum_block_chain DEFAULT NULL)
RETURNS table (
    "pkey_id" bigint,
    "blockchain" enum_block_chain,
    "address" varchar
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT pkey_id, blockchain, address
                 FROM tbl.escrow_contract_address
                WHERE (a_blockchain ISNULL OR blockchain = a_blockchain)
                 ORDER BY pkey_id
                 OFFSET a_offset
                 LIMIT a_limit;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_admin_update_escrow_contract_address(a_blockchain enum_block_chain, a_address varchar)
RETURNS void
LANGUAGE plpgsql
AS $$
    
BEGIN
    UPDATE tbl.escrow_contract_address
         SET address = a_address
         WHERE blockchain = a_blockchain;
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_watcher_save_raw_transaction(a_transaction_hash varchar, a_blockchain enum_block_chain, a_raw_transaction varchar, a_dex varchar DEFAULT NULL)
RETURNS table (
    "transaction_cache_id" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY INSERT INTO tbl.transaction_cache(transaction_hash,
                                                   blockchain,
                                                   dex,
                                                   raw_content,
                                                   created_at)
                 VALUES (a_transaction_hash,
                         a_blockchain,
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
        

CREATE OR REPLACE FUNCTION api.fun_watcher_save_strategy_watching_wallet_trade_ledger(a_address varchar, a_transaction_hash varchar, a_blockchain enum_block_chain, a_contract_address varchar, a_dex varchar DEFAULT NULL, a_token_in_address varchar DEFAULT NULL, a_token_out_address varchar DEFAULT NULL, a_amount_in decimal(56, 18) DEFAULT NULL, a_amount_out decimal(56, 18) DEFAULT NULL, a_happened_at bigint DEFAULT NULL)
RETURNS table (
    "strategy_watching_wallet_trade_ledger_id" bigint,
    "expert_watched_wallet_id" bigint,
    "fkey_token_in" bigint,
    "fkey_token_in_name" varchar,
    "fkey_token_out" bigint,
    "fkey_token_out_name" varchar
)
LANGUAGE plpgsql
AS $$
    
DECLARE
    _strategy_watching_wallet_trade_ledger_id bigint;
    _expert_watched_wallet_id bigint;
    _fkey_token_in            bigint;
    _fkey_token_in_name       varchar;
    _fkey_token_out           bigint;
    _fkey_token_out_name      varchar;
BEGIN
    SELECT pkey_id
    INTO _expert_watched_wallet_id
    FROM tbl.expert_watched_wallet
    WHERE address = a_address
      AND blockchain = a_blockchain;
    SELECT pkey_id, symbol
    INTO _fkey_token_in, _fkey_token_in_name
    FROM tbl.escrow_token_contract_address
    WHERE address = a_token_in_address
      AND blockchain = a_blockchain;
    SELECT pkey_id, symbol
    INTO _fkey_token_out, _fkey_token_out_name
    FROM tbl.escrow_token_contract_address
    WHERE address = a_token_out_address
      AND blockchain = a_blockchain;

    IF _expert_watched_wallet_id IS NOT NULL AND _fkey_token_in IS NOT NULL AND _fkey_token_out IS NOT NULL THEN
        INSERT INTO tbl.strategy_watching_wallet_trade_ledger
            (
             expert_watched_wallet_id, blockchain,
             transaction_hash, dex, contract_address,
             fkey_token_in, fkey_token_out, amount_in,
             amount_out, heppened_at
                )
        VALUES (_expert_watched_wallet_id, a_blockchain, a_transaction_hash, a_dex, a_contract_address,
                _fkey_token_in, _fkey_token_out, a_amount_in, a_amount_out, COALESCE(a_happened_at, EXTRACT(EPOCH FROM NOW())::bigint))
        RETURNING pkey_id
        INTO _strategy_watching_wallet_trade_ledger_id;
        RETURN QUERY SELECT _strategy_watching_wallet_trade_ledger_id, _expert_watched_wallet_id,
                            _fkey_token_in, _fkey_token_in_name, _fkey_token_out, _fkey_token_out_name;
    END IF;
END
        
$$;
        

CREATE OR REPLACE FUNCTION api.fun_watcher_list_dex_path_for_pair(a_token_in_address varchar, a_token_out_address varchar, a_blockchain enum_block_chain, a_dex enum_dex DEFAULT NULL, a_format enum_dex_path_format DEFAULT NULL)
RETURNS table (
    "pkey_id" bigint,
    "token_in_id" bigint,
    "token_out_id" bigint,
    "blockchain" enum_block_chain,
    "dex" enum_dex,
    "format" enum_dex_path_format,
    "path_data" varchar,
    "updated_at" bigint
)
LANGUAGE plpgsql
AS $$
    
DECLARE
		_token_in_id BIGINT;
		_token_out_id BIGINT;
BEGIN
		SELECT etca.pkey_id INTO _token_in_id FROM tbl.escrow_token_contract_address AS etca
		WHERE etca.address = a_token_in_address AND etca.blockchain = a_blockchain;

		SELECT etca.pkey_id INTO _token_out_id FROM tbl.escrow_token_contract_address AS etca
		WHERE etca.address = a_token_out_address AND etca.blockchain = a_blockchain;

		ASSERT _token_in_id IS NOT NULL;
		ASSERT _token_out_id IS NOT NULL;

		RETURN QUERY SELECT
			dpfp.pkey_id,
			dpfp.fkey_token_in,
			dpfp.fkey_token_out,
			dpfp.blockchain,
			dpfp.dex,
			dpfp.format,
			dpfp.path_data,
			dpfp.updated_at
		FROM tbl.dex_path_for_pair AS dpfp
		WHERE dpfp.fkey_token_in = _token_in_id AND dpfp.fkey_token_out = _token_out_id
		AND (a_dex ISNULL OR dpfp.dex = a_dex)
		AND (a_format ISNULL OR dpfp.format = a_format);
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_watcher_upsert_dex_path_for_pair(a_token_in_address varchar, a_token_out_address varchar, a_blockchain enum_block_chain, a_dex enum_dex, a_format enum_dex_path_format, a_path_data varchar)
RETURNS table (
    "dex_path_for_pair_id" bigint
)
LANGUAGE plpgsql
AS $$
    
DECLARE
		_token_in_id BIGINT;
		_token_out_id BIGINT;
		_dex_path_for_pair_id BIGINT;
BEGIN

		SELECT etca.pkey_id INTO _token_in_id FROM tbl.escrow_token_contract_address AS etca
		WHERE etca.address = a_token_in_address AND etca.blockchain = a_blockchain;

		SELECT etca.pkey_id INTO _token_out_id FROM tbl.escrow_token_contract_address AS etca
		WHERE etca.address = a_token_out_address AND etca.blockchain = a_blockchain;

		ASSERT _token_in_id IS NOT NULL;
		ASSERT _token_out_id IS NOT NULL;

		SELECT dpfp.pkey_id INTO _dex_path_for_pair_id FROM tbl.dex_path_for_pair AS dpfp
		WHERE dpfp.fkey_token_in = _token_in_id
			AND dpfp.fkey_token_out = _token_out_id
			AND dpfp.blockchain = a_blockchain
			AND dpfp.dex = a_dex
			AND dpfp.format = a_format;

		-- if the dex path for this token_in, token_out, blockchain, dex, and format does not exist, create one
		IF _dex_path_for_pair_id IS NULL THEN
			INSERT INTO tbl.dex_path_for_pair (
				fkey_token_in,
				fkey_token_out,
				blockchain,
				dex,
				format,
				path_data,
				updated_at
			) VALUES (
				_token_in_id,
				_token_out_id,
				a_blockchain,
				a_dex,
				a_format,
				a_path_data,
				EXTRACT(EPOCH FROM NOW())
			) RETURNING pkey_id INTO _dex_path_for_pair_id;
		ELSE
			UPDATE tbl.dex_path_for_pair
			SET path_data = a_path_data,
				updated_at = EXTRACT(EPOCH FROM NOW())
			WHERE pkey_id = _dex_path_for_pair_id;
		END IF;

		RETURN QUERY SELECT _dex_path_for_pair_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_watcher_list_last_dex_trades_for_pair(a_token_in_address varchar, a_token_out_address varchar, a_blockchain enum_block_chain, a_dex enum_dex DEFAULT NULL)
RETURNS table (
    "transaction_hash" varchar,
    "blockchain" enum_block_chain,
    "dex" enum_dex,
    "token_in_id" bigint,
    "token_out_id" bigint,
    "amount_in" decimal(56, 18),
    "amount_out" decimal(56, 18),
    "happened_at" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
		RETURN QUERY
		SELECT
			dex_trade.transaction_hash,
			dex_trade.blockchain,
			dex_trade.dex,
			dex_trade.fkey_token_in,
			dex_trade.fkey_token_out,
			dex_trade.amount_in,
			dex_trade.amount_out,
			dex_trade.happened_at
		FROM tbl.last_dex_trade_for_pair AS dex_trade
		WHERE dex_trade.fkey_token_in = (SELECT etca.pkey_id FROM tbl.escrow_token_contract_address AS etca WHERE etca.address = a_token_in_address AND etca.blockchain = a_blockchain)
		AND dex_trade.fkey_token_out = (SELECT etca.pkey_id FROM tbl.escrow_token_contract_address AS etca WHERE etca.address = a_token_out_address AND etca.blockchain = a_blockchain)
		AND dex_trade.blockchain = a_blockchain
		AND dex_trade.dex = COALESCE(a_dex, dex_trade.dex);
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_watcher_upsert_last_dex_trade_for_pair(a_transaction_hash varchar, a_blockchain enum_block_chain, a_dex enum_dex, a_token_in_address varchar, a_token_out_address varchar, a_amount_in decimal(56, 18), a_amount_out decimal(56, 18))
RETURNS table (
    "last_dex_trade_for_pair_id" bigint
)
LANGUAGE plpgsql
AS $$
    
DECLARE
		_last_dex_trade_for_pair_id BIGINT;
		_token_in_id BIGINT;
		_token_out_id BIGINT;
BEGIN
		SELECT etca.pkey_id INTO _token_in_id
		FROM tbl.escrow_token_contract_address AS etca
		WHERE etca.address = a_token_in_address AND etca.blockchain = a_blockchain;

		SELECT etca.pkey_id INTO _token_out_id
		FROM tbl.escrow_token_contract_address AS etca
		WHERE etca.address = a_token_out_address AND etca.blockchain = a_blockchain;

		ASSERT _token_in_id NOTNULL;
		ASSERT _token_out_id NOTNULL;

		SELECT ldtfp.pkey_id INTO _last_dex_trade_for_pair_id
		FROM tbl.last_dex_trade_for_pair AS ldtfp
		WHERE ldtfp.fkey_token_in = _token_in_id AND ldtfp.fkey_token_out = _token_out_id
		AND ldtfp.blockchain = a_blockchain AND ldtfp.dex = a_dex;

		-- if the trade record for this token_in, token_out, dex, and blockchain does not exist, create one
		IF _last_dex_trade_for_pair_id IS NULL THEN
			INSERT INTO tbl.last_dex_trade_for_pair (
				transaction_hash,
				blockchain,
				dex,
				fkey_token_in,
				fkey_token_out,
				amount_in,
				amount_out,
				happened_at
			) VALUES (
				a_transaction_hash,
				a_blockchain,
				a_dex,
				_token_in_id,
				_token_out_id,
				a_amount_in,
				a_amount_out,
				EXTRACT(EPOCH FROM NOW())
			) RETURNING pkey_id INTO _last_dex_trade_for_pair_id;
		ELSE
			UPDATE tbl.last_dex_trade_for_pair
			SET transaction_hash = a_transaction_hash,
				amount_in = a_amount_in,
				amount_out = a_amount_out,
				happened_at = EXTRACT(EPOCH FROM NOW())
			WHERE pkey_id = _last_dex_trade_for_pair_id;
		END IF;

		RETURN QUERY SELECT _last_dex_trade_for_pair_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_watcher_upsert_strategy_pool_contract_asset_balance(a_strategy_pool_contract_id bigint, a_token_address varchar, a_blockchain enum_block_chain, a_new_balance decimal(56, 18))
RETURNS table (
    "strategy_contract_asset_balance_id" bigint
)
LANGUAGE plpgsql
AS $$
    
DECLARE
		_strategy_contract_asset_balance_id BIGINT;
		_token_id BIGINT;
BEGIN
		SELECT etca.pkey_id INTO _token_id
		FROM tbl.escrow_token_contract_address AS etca
		WHERE etca.address = a_token_address AND etca.blockchain = a_blockchain;

		SELECT spcab.pkey_id INTO _strategy_contract_asset_balance_id
		FROM tbl.strategy_pool_contract_asset_balance AS spcab
		WHERE spcab.fkey_strategy_pool_contract_id = a_strategy_pool_contract_id AND spcab.fkey_token_id = _token_id;
		
		-- if the record for this token and this strategy pool contract does not exit, create one
		IF _strategy_contract_asset_balance_id IS NULL THEN
			INSERT INTO tbl.strategy_pool_contract_asset_balance (fkey_strategy_pool_contract_id, fkey_token_id, balance)
			VALUES (a_strategy_pool_contract_id, _token_id, a_new_balance)
			RETURNING pkey_id INTO _strategy_contract_asset_balance_id;
		ELSE
			UPDATE tbl.strategy_pool_contract_asset_balance
			SET balance = a_new_balance
			WHERE pkey_id = _strategy_contract_asset_balance_id;
		END IF;

		RETURN QUERY SELECT _strategy_contract_asset_balance_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_watcher_list_strategy_pool_contract_asset_balances(a_strategy_pool_contract_id bigint DEFAULT NULL, a_strategy_id bigint DEFAULT NULL, a_blockchain enum_block_chain DEFAULT NULL, a_token_address varchar DEFAULT NULL)
RETURNS table (
    "token_id" bigint,
    "token_name" varchar,
    "token_symbol" varchar,
    "token_address" varchar,
    "token_decimals" int,
    "blockchain" enum_block_chain,
    "balance" decimal(56, 18)
)
LANGUAGE plpgsql
AS $$
    
BEGIN
		RETURN QUERY SELECT
			tc.pkey_id,
			tc.short_name,
			tc.symbol,
			tc.address,
			tc.decimals,
			tc.blockchain,
			spcab.balance AS balance
			FROM tbl.strategy_pool_contract_asset_balance AS spcab
			JOIN tbl.escrow_token_contract_address AS tc ON spcab.fkey_token_id = tc.pkey_id
			JOIN tbl.strategy_pool_contract AS spc ON spc.pkey_id = spcab.fkey_strategy_pool_contract_id 
			WHERE (a_strategy_pool_contract_id ISNULL OR spcab.fkey_strategy_pool_contract_id = a_strategy_pool_contract_id)
			AND (a_strategy_id ISNULL OR spc.fkey_strategy_id = a_strategy_id)
			AND (a_blockchain ISNULL OR tc.blockchain = a_blockchain)
			AND (a_token_address ISNULL OR tc.address = a_token_address)
			AND spcab.balance != 0.0;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_watcher_list_strategy_escrow_pending_wallet_balance(a_strategy_id bigint DEFAULT NULL, a_token_address varchar DEFAULT NULL)
RETURNS table (
    "strategy_id" bigint,
    "blockchain" enum_block_chain,
    "address" varchar,
    "token_id" bigint,
    "token_address" varchar,
    "token_name" varchar,
    "token_symbol" varchar,
    "balance" decimal(56, 18)
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT w.fkey_strategy_id,
                        l.blockchain,
                        w.address,
                        t.pkey_id,
                        t.address,
                        t.name,
                        t.symbol,
                        l.balance
                 FROM tbl.strategy_escrow_pending_wallet_balance AS l
                 JOIN tbl.strategy_escrow_pending_wallet_address AS w ON l.fkey_strategy_pending_wallet_address_id = w.pkey_id
                 JOIN tbl.escrow_token_contract_address AS t ON l.fkey_token_id = t.pkey_id
                 WHERE strategy_id = a_strategy_id
                     AND (a_token_address ISNULL OR t.address = a_token_address);

END
        
$$;
        

CREATE OR REPLACE FUNCTION api.fun_watcher_list_user_strategy_balance(a_limit bigint, a_offset bigint, a_strategy_id bigint DEFAULT NULL, a_user_id bigint DEFAULT NULL, a_blockchain enum_block_chain DEFAULT NULL)
RETURNS table (
    "strategy_id" bigint,
    "user_id" bigint,
    "blockchain" enum_block_chain,
    "strategy_pool_contract_address" varchar,
    "user_strategy_wallet_address" varchar,
    "balance" decimal(56, 18)
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT spc.fkey_strategy_id,
                        usw.fkey_user_id,
                        spc.blockchain,
                        spc.address,
                        usw.address,
                        usl.balance
                 FROM tbl.user_strategy_balance AS usl
                 JOIN tbl.user_strategy_wallet AS usw ON usw.pkey_id = usl.fkey_user_strategy_wallet_id
                 JOIN tbl.strategy_pool_contract AS spc ON spc.pkey_id = usl.fkey_strategy_pool_contract_id
                 WHERE (a_strategy_id ISNULL OR spc.fkey_strategy_id = a_strategy_id)
                   AND (a_user_id ISNULL OR usw.fkey_user_id = a_user_id)
                   AND (a_blockchain ISNULL OR spc.blockchain = a_blockchain)
                 ORDER BY usl.pkey_id DESC
                 LIMIT a_limit
                 OFFSET a_offset;
END
        
$$;
        

CREATE OR REPLACE FUNCTION api.fun_watcher_upsert_expert_listened_wallet_asset_balance(a_address varchar, a_blockchain enum_block_chain, a_token_id bigint, a_old_balance decimal(56, 18), a_new_balance decimal(56, 18))
RETURNS table (
    "expert_listened_wallet_asset_balance_id" bigint
)
LANGUAGE plpgsql
AS $$
    
DECLARE
    _expert_watched_wallet_id                         bigint;
    _expert_listened_wallet_asset_balance_id          bigint;
    _expert_listened_wallet_asset_balance_old_balance decimal(56, 18);
    _pkey_id                                          bigint;
BEGIN
    SELECT pkey_id
    INTO _expert_watched_wallet_id
    FROM tbl.expert_watched_wallet
    WHERE address = a_address
      AND blockchain = a_blockchain;
    ASSERT _expert_watched_wallet_id NOTNULL;
    SELECT elwal.pkey_id, elwal.balance
    INTO _expert_listened_wallet_asset_balance_id, _expert_listened_wallet_asset_balance_old_balance
    FROM tbl.expert_listened_wallet_asset_balance AS elwal
             JOIN tbl.expert_watched_wallet AS eww ON eww.pkey_id = elwal.fkey_expert_watched_wallet_id
    WHERE elwal.fkey_token_id = a_token_id
      AND eww.pkey_id = _expert_watched_wallet_id;

    -- insert new entry if not exist
    IF _expert_listened_wallet_asset_balance_id ISNULL THEN
        INSERT INTO tbl.expert_listened_wallet_asset_balance (fkey_token_id, balance, fkey_expert_watched_wallet_id)
        VALUES (a_token_id, a_new_balance, _expert_watched_wallet_id)
        RETURNING pkey_id
            INTO _pkey_id;
    ELSE
        -- update old balance if exist and equals to old balance
        IF _expert_listened_wallet_asset_balance_old_balance NOTNULL AND _expert_listened_wallet_asset_balance_old_balance != a_old_balance THEN
            RETURN;
        END IF;
        UPDATE tbl.expert_listened_wallet_asset_balance
        SET balance = a_new_balance
        WHERE pkey_id = _expert_listened_wallet_asset_balance_id
        RETURNING pkey_id
            INTO _pkey_id;
    END IF;

    RETURN QUERY SELECT _pkey_id;
END
        
$$;
        

CREATE OR REPLACE FUNCTION api.fun_watcher_list_expert_listened_wallet_asset_balance(a_limit bigint DEFAULT NULL, a_offset bigint DEFAULT NULL, a_strategy_id bigint DEFAULT NULL, a_address varchar DEFAULT NULL, a_blockchain enum_block_chain DEFAULT NULL, a_token_id bigint DEFAULT NULL)
RETURNS table (
    "pkey_id" bigint,
    "address" varchar,
    "blockchain" enum_block_chain,
    "token_id" bigint,
    "token_address" varchar,
    "token_symbol" varchar,
    "token_decimals" int,
    "balance" decimal(56, 18)
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT 
        elwab.pkey_id,
        eww.address,
        eww.blockchain,
        elwab.fkey_token_id,
        etca.address,
        etca.symbol,
        etca.decimals,
        elwab.balance
    FROM tbl.expert_listened_wallet_asset_balance AS elwab
    JOIN tbl.expert_watched_wallet AS eww ON eww.pkey_id = elwab.fkey_expert_watched_wallet_id
    JOIN tbl.escrow_token_contract_address AS etca ON etca.pkey_id = elwab.fkey_token_id
    JOIN tbl.strategy_watched_wallet AS sww ON sww.fkey_expert_watched_wallet_id = eww.pkey_id
    WHERE (a_token_id ISNULL OR elwab.fkey_token_id = a_token_id)
     AND (a_address ISNULL OR eww.address = a_address)
     AND (a_blockchain ISNULL OR eww.blockchain = a_blockchain)
     AND (a_strategy_id ISNULL OR sww.fkey_strategy_id = a_strategy_id)
	 AND elwab.balance != 0.0	 
     ORDER BY elwab.pkey_id DESC
    LIMIT a_limit
    OFFSET a_offset;
END

        
$$;
        

CREATE OR REPLACE FUNCTION api.fun_watcher_save_strategy_pool_contract(a_strategy_id bigint, a_blockchain enum_block_chain, a_address varchar)
RETURNS table (
    "pkey_id" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY INSERT INTO tbl.strategy_pool_contract AS spc (fkey_strategy_id, blockchain, address, created_at)
    VALUES (a_strategy_id, a_blockchain, a_address, EXTRACT(EPOCH FROM NOW()))
    RETURNING spc.pkey_id;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_watcher_upsert_user_strategy_balance(a_user_id bigint, a_strategy_id bigint, a_blockchain enum_block_chain, a_old_balance decimal(56, 18), a_new_balance decimal(56, 18))
RETURNS table (
    "ret_pkey_id" bigint
)
LANGUAGE plpgsql
AS $$
    
DECLARE
    _strategy_pool_contract_id bigint;
    _user_strategy_wallet_id  bigint;
    _user_strategy_wallet_balance_old_balance    decimal(56, 18);
    _user_strategy_wallet_balance_id             bigint;
    _pkey_id                                     bigint;
BEGIN
    SELECT pkey_id INTO _strategy_pool_contract_id FROM tbl.strategy_pool_contract
    WHERE fkey_strategy_id = a_strategy_id
        AND blockchain = a_blockchain;
    ASSERT _strategy_pool_contract_id NOTNULL;
    
    SELECT pkey_id INTO _user_strategy_wallet_id FROM tbl.user_strategy_wallet
    WHERE fkey_user_id = a_user_id
        AND blockchain = a_blockchain;

    ASSERT _user_strategy_wallet_id NOTNULL;
    
    SELECT pkey_id, balance INTO _user_strategy_wallet_balance_id, _user_strategy_wallet_balance_old_balance
    FROM tbl.user_strategy_balance
    WHERE fkey_user_strategy_wallet_id = _user_strategy_wallet_id
        AND fkey_strategy_pool_contract_id = _strategy_pool_contract_id;
    
    -- insert new entry if not exist
    IF _user_strategy_wallet_balance_id ISNULL THEN
        INSERT INTO tbl.user_strategy_balance (fkey_user_strategy_wallet_id, fkey_strategy_pool_contract_id, balance)
        VALUES (_user_strategy_wallet_id, _strategy_pool_contract_id, a_new_balance)
        RETURNING pkey_id
            INTO _pkey_id;
    ELSE
    -- update old balance if exist and equals to old balance
        IF _user_strategy_wallet_balance_old_balance NOTNULL AND _user_strategy_wallet_balance_old_balance != a_old_balance THEN
            RETURN;
        END IF;

        UPDATE tbl.user_strategy_balance
        SET balance = a_new_balance
        WHERE pkey_id = _user_strategy_wallet_balance_id
        RETURNING pkey_id
            INTO _pkey_id;
            
    END IF;
    
    RETURN QUERY SELECT _pkey_id;
    
        
END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_watcher_upsert_user_deposit_withdraw_balance(a_user_id bigint, a_user_address varchar, a_token_address varchar, a_escrow_contract_address varchar, a_blockchain enum_block_chain, a_old_balance decimal(56, 18), a_new_balance decimal(56, 18))
RETURNS table (
    "ret_pkey_id" bigint
)
LANGUAGE plpgsql
AS $$
    
DECLARE
    _token_id bigint;
    _escrow_contract_address_id bigint;
    _user_deposit_withdraw_balance_id          bigint;
    _user_deposit_withdraw_balance_old_balance decimal(56, 18);
    _pkey_id                                   bigint;
BEGIN
    SELECT pkey_id INTO _token_id FROM tbl.escrow_token_contract_address WHERE address = a_token_address AND blockchain = a_blockchain;
    SELECT pkey_id INTO _escrow_contract_address_id FROM tbl.escrow_contract_address WHERE address = a_escrow_contract_address AND blockchain = a_blockchain;
    ASSERT _token_id NOTNULL AND _escrow_contract_address_id NOTNULL;
    SELECT elwal.pkey_id, elwal.balance
    INTO _user_deposit_withdraw_balance_id, _user_deposit_withdraw_balance_old_balance
    FROM tbl.user_deposit_withdraw_balance AS elwal
    WHERE elwal.fkey_token_id = _token_id
      AND elwal.fkey_user_id = a_user_id
			AND elwal.user_address = a_user_address
      AND elwal.fkey_escrow_contract_address_id = _escrow_contract_address_id;

    -- insert new entry if not exist
    IF _user_deposit_withdraw_balance_id ISNULL THEN
        INSERT INTO tbl.user_deposit_withdraw_balance (fkey_user_id, user_address, fkey_escrow_contract_address_id, fkey_token_id, balance)
        VALUES (a_user_id, a_user_address, _escrow_contract_address_id, _token_id, a_new_balance)
        RETURNING pkey_id
            INTO _pkey_id;
    ELSE
        -- update old balance if exist and equals to old balance
        IF _user_deposit_withdraw_balance_old_balance NOTNULL AND _user_deposit_withdraw_balance_old_balance != a_old_balance THEN
            RETURN;
        END IF;
        UPDATE tbl.user_deposit_withdraw_balance
        SET balance = a_new_balance
        WHERE pkey_id = _user_deposit_withdraw_balance_id
        RETURNING pkey_id
            INTO _pkey_id;
    END IF;
    RETURN QUERY SELECT _pkey_id;

END
            
$$;
        

CREATE OR REPLACE FUNCTION api.fun_watcher_list_strategy_pool_contract(a_limit bigint, a_offset bigint, a_strategy_id bigint DEFAULT NULL, a_blockchain enum_block_chain DEFAULT NULL, a_address varchar DEFAULT NULL)
RETURNS table (
    "pkey_id" bigint,
    "strategy_id" bigint,
    "blockchain" enum_block_chain,
    "address" varchar,
    "created_at" bigint
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT
        spc.pkey_id,
        spc.fkey_strategy_id,
        spc.blockchain,
        spc.address,
        spc.created_at
    FROM tbl.strategy_pool_contract AS spc
    WHERE (a_strategy_id ISNULL OR spc.fkey_strategy_id = a_strategy_id)
        AND (a_blockchain ISNULL OR spc.blockchain = a_blockchain)
        AND (a_address ISNULL OR spc.address = a_address)
    ORDER BY spc.pkey_id
    LIMIT a_limit
    OFFSET a_offset;
END
        
$$;
        

CREATE OR REPLACE FUNCTION api.fun_user_list_escrow_contract_address_req(a_blockchain enum_block_chain DEFAULT NULL)
RETURNS table (
    "pkey_id" bigint,
    "blockchain" enum_block_chain,
    "address" varchar
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT
        eca.pkey_id,
        eca.blockchain,
        eca.address
    FROM tbl.escrow_contract_address AS eca
    WHERE (a_blockchain ISNULL OR eca.blockchain = a_blockchain)
    ORDER BY eca.pkey_id;
END
        
$$;
        

CREATE OR REPLACE FUNCTION api.fun_asset_price_insert_asset_prices(a_symbols varchar[], a_prices double precision[], a_timestamps bigint[] DEFAULT NULL)
RETURNS table (
    "success" boolean
)
LANGUAGE plpgsql
AS $$
    
BEGIN
	IF a_timestamps IS NULL THEN
		INSERT INTO tbl.token_price (symbol, price, created_at)
		SELECT
				a_symbol,
				a_price,
				EXTRACT(EPOCH FROM NOW())::bigint
		FROM UNNEST(a_symbols, a_prices) AS u(a_symbol, a_price);
	ELSE
		INSERT INTO tbl.token_price (symbol, price, created_at)
		SELECT
				a_symbol,
				a_price,
				a_timestamp
		FROM UNNEST(a_symbols, a_prices, a_timestamps) AS u(a_symbol, a_price, a_timestamp);
	END IF;

	RETURN QUERY SELECT true AS "success";
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_asset_price_list_asset_prices(a_symbols varchar[] DEFAULT NULL, a_limit int DEFAULT NULL, a_offset int DEFAULT NULL)
RETURNS table (
    "symbol" varchar,
    "price_latest" double precision,
    "price_1d" double precision,
    "price_7d" double precision,
    "price_30d" double precision
)
LANGUAGE plpgsql
AS $$
    
DECLARE
	_now TIMESTAMP := NOW();
	_days_ago_1 BIGINT := EXTRACT(EPOCH FROM (_now - INTERVAL '1 DAYS'))::BIGINT;
	_days_ago_2 BIGINT := EXTRACT(EPOCH FROM (_now - INTERVAL '2 DAYS'))::BIGINT;
	_days_ago_7 BIGINT := EXTRACT(EPOCH FROM (_now - INTERVAL '7 DAYS'))::BIGINT;
	_days_ago_8 BIGINT := EXTRACT(EPOCH FROM (_now - INTERVAL '8 DAYS'))::BIGINT;
	_days_ago_30 BIGINT := EXTRACT(EPOCH FROM (_now - INTERVAL '30 DAYS'))::BIGINT;
	_days_ago_31 BIGINT := EXTRACT(EPOCH FROM (_now - INTERVAL '31 DAYS'))::BIGINT;
BEGIN
	RETURN QUERY
	WITH date_ranges AS (
		SELECT
				tp.symbol,
				MAX(tp.created_at) FILTER (
					WHERE tp.created_at >= _days_ago_1
				) AS latest,
				MAX(tp.created_at) FILTER (
					WHERE tp.created_at <= _days_ago_1
					AND tp.created_at >= _days_ago_2
				) AS day_1,
				MAX(tp.created_at) FILTER (
					WHERE tp.created_at <= _days_ago_7
					AND tp.created_at >= _days_ago_8
				) AS day_7,
				MAX(tp.created_at) FILTER (
					WHERE tp.created_at <= _days_ago_30
					AND tp.created_at >= _days_ago_31
				) AS day_30
		FROM tbl.token_price AS tp
		WHERE (a_symbols IS NULL OR tp.symbol = ANY(a_symbols))
		GROUP BY tp.symbol
	)

	SELECT
		dr.symbol,
		tp_latest.price AS price_latest,
		tp_1d.price AS price_1d,
		tp_7d.price AS price_7d,
		tp_30d.price AS price_30d
	FROM date_ranges dr
	LEFT JOIN tbl.token_price AS tp_latest ON tp_latest.symbol = dr.symbol AND tp_latest.created_at = dr.latest
	LEFT JOIN tbl.token_price AS tp_1d ON tp_1d.symbol = dr.symbol AND tp_1d.created_at = dr.day_1
	LEFT JOIN tbl.token_price AS tp_7d ON tp_7d.symbol = dr.symbol AND tp_7d.created_at = dr.day_7
	LEFT JOIN tbl.token_price AS tp_30d ON tp_30d.symbol = dr.symbol AND tp_30d.created_at = dr.day_30
	ORDER BY dr.symbol
	LIMIT a_limit
	OFFSET a_offset;
END

$$;
        

CREATE OR REPLACE FUNCTION api.fun_asset_price_delete_old_asset_price_entries()
RETURNS table (
    "success" boolean
)
LANGUAGE plpgsql
AS $$
    
DECLARE
	days_ago BIGINT := (EXTRACT(EPOCH FROM (NOW() - INTERVAL '91 DAYS'))::BIGINT);
BEGIN
	DELETE FROM tbl.token_price AS tp
	WHERE tp.created_at <= days_ago;

	RETURN QUERY SELECT true AS "success";
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
        

CREATE OR REPLACE FUNCTION api.WATCHER_SERVICE()
RETURNS table (
    "code" int
)
LANGUAGE plpgsql
AS $$
    BEGIN RETURN QUERY SELECT 4; END
$$;
        

CREATE OR REPLACE FUNCTION api.ASSET_PRICE_SERVICE()
RETURNS table (
    "code" int
)
LANGUAGE plpgsql
AS $$
    BEGIN RETURN QUERY SELECT 5; END
$$;
        
