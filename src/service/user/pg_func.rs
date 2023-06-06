use model::types::*;

pub fn get_user_pg_func() -> Vec<ProceduralFunction> {
    vec![
        ProceduralFunction::new(
            "fun_user_follow_strategy",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("strategy_id", Type::BigInt),
            ],
            vec![Field::new("success", Type::Boolean)],
            r#"

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
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_unfollow_strategy",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("strategy_id", Type::BigInt),
            ],
            vec![Field::new("success", Type::Boolean)],
            r#"

BEGIN
    UPDATE tbl.user_follow_strategy 
      SET unfollowed = TRUE,
          updated_at = extract(epoch from now())::bigint
      WHERE fkey_user_id = a_user_id
      AND fkey_strategy_id = a_strategy_id
      AND unfollowed = FALSE;
      
    RETURN QUERY SELECT TRUE AS "select";

END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_list_followed_strategies",
            vec![Field::new("user_id", Type::BigInt)],
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("strategy_name", Type::String),
                Field::new("strategy_description", Type::String),
                Field::new("net_value", Type::Numeric),
                Field::new("followers", Type::Int),
                Field::new("backers", Type::Int),
                Field::new("risk_score", Type::Numeric),
                Field::new("aum", Type::Numeric),
            ],
            r#"
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
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_list_strategies",
            vec![],
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("strategy_name", Type::String),
                Field::new("strategy_description", Type::String),
                Field::new("net_value", Type::Numeric),
                Field::new("followers", Type::Int),
                Field::new("backers", Type::Int),
                Field::new("risk_score", Type::Numeric),
                Field::new("aum", Type::Numeric),
            ],
            r#"
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
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_get_strategy",
            // TODO search options
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("strategy_name", Type::String),
                Field::new("strategy_description", Type::String),
                Field::new("current_usdc", Type::String),
                Field::new("total_backed_usdc", Type::String),
                Field::new("total_exited_usdc", Type::String),
                Field::new("followers", Type::BigInt),
                Field::new("backers", Type::BigInt),
                Field::new("risk_score", Type::optional(Type::Numeric)),
                Field::new("aum", Type::optional(Type::Numeric)),
                Field::new("evm_contract_address", Type::optional(Type::String)),
                // TODO more fields
            ],
            r#"
BEGIN
    RETURN QUERY SELECT a.pkey_id AS strategy_id,
                          a.name AS strategy_name,
                          a.description AS strategy_description,
                          current_usdc,
                          total_backed_usdc,
                          total_exited_usdc,
                          (SELECT COUNT(*) FROM tbl.user_follow_strategy WHERE fkey_strategy_id = a.pkey_id AND unfollowed = FALSE) AS followers,
                          (SELECT COUNT(DISTINCT user_back_strategy_history.fkey_user_id) FROM tbl.user_back_strategy_history WHERE fkey_strategy_id = a.pkey_id) AS followers,
                          a.risk_score as risk_score,
                          a.aum as aum,
                          a.evm_contract_address
                 FROM tbl.strategy AS a
                 WHERE a.pkey_id = a_strategy_id;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_get_strategy_statistics_net_value",
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![
                Field::new("time", Type::BigInt),
                Field::new("net_value", Type::Numeric),
            ],
            r#"
BEGIN
    -- TODO
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_get_strategy_statistics_follow_history",
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![
                Field::new("time", Type::BigInt),
                Field::new("follower_count", Type::Numeric),
            ],
            r#"
BEGIN
    -- TODO
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_get_strategy_statistics_back_history",
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![
                Field::new("time", Type::BigInt),
                Field::new("backer_count", Type::Numeric),
                Field::new("backer_quantity_usd", Type::Numeric),
            ],
            r#"
BEGIN
    -- TODO
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_deposit_to_escrow",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("user_address", Type::String),
                Field::new("contract_address", Type::String),
                Field::new("receiver_address", Type::String),
                Field::new("quantity", Type::String),
                Field::new("transaction_hash", Type::String),
            ],
            vec![Field::new("success", Type::Boolean)],
            r#"
 
BEGIN
    IF EXISTS( SELECT * FROM  tbl.user_deposit_history WHERE transaction_hash = a_transaction_hash) THEN
        RETURN QUERY SELECT FALSE;
    END IF;
    INSERT INTO tbl.user_deposit_history (
        fkey_user_id,
        blockchain,
        user_address,
        contract_address,
        receiver_address,
        quantity,
        transaction_hash,
        created_at
    ) VALUES (
     a_user_id,
     a_blockchain,
     a_user_address,
     a_contract_address,
     a_receiver_address,
     a_quantity,
     a_transaction_hash,
     EXTRACT(EPOCH FROM NOW())::bigint
    );
    RETURN QUERY SELECT TRUE;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_back_strategy",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("strategy_id", Type::BigInt),
                Field::new("quantity", Type::String),
                Field::new("new_total_backed_quantity", Type::String),
                Field::new("old_total_backed_quantity", Type::String),
                Field::new("new_current_quantity", Type::String),
                Field::new("old_current_quantity", Type::String),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("transaction_hash", Type::String),
                Field::new("earn_sp_tokens", Type::String),
            ],
            vec![Field::new("success", Type::Boolean)],
            r#"
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
    INSERT INTO tbl.user_back_strategy_history (fkey_user_id, fkey_strategy_id, quantity, blockchain,
                                                transaction_hash, earn_sp_tokens, back_time)
    VALUES (a_user_id, a_strategy_id, a_quantity, a_blockchain, a_transaction_hash, a_earn_sp_tokens,
            extract(epoch from now())::bigint);
    RETURN QUERY SELECT TRUE;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_list_backed_strategies",
            vec![Field::new("user_id", Type::BigInt)],
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("strategy_name", Type::String),
                Field::new("strategy_description", Type::String),
                Field::new("net_value", Type::Numeric),
                Field::new("followers", Type::Int),
                Field::new("backers", Type::Int),
                Field::new("risk_score", Type::Numeric),
                Field::new("aum", Type::Numeric),
            ],
            r#"
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
"#,
        ),
        ProceduralFunction::new(
            "fun_user_list_back_strategy_history",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("strategy_id", Type::optional(Type::BigInt)),
            ],
            vec![
                Field::new("back_history_id", Type::BigInt),
                Field::new("strategy_id", Type::BigInt),
                Field::new("quantity", Type::String),
                Field::new("wallet_address", Type::String),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("transaction_hash", Type::String),
                Field::new("time", Type::BigInt),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT a.pkey_id          AS back_history_id,
                        a.fkey_strategy_id AS strategy_id,
                        a.quantity         AS quantity,
                        a.blockchain       AS blockchain,
                        a.transaction_hash AS transaction_hash,
                        a.time             AS time
                 FROM tbl.user_back_strategy_history AS a
                 WHERE a.fkey_user_id = a_user_id
                  AND (a_strategy_id NOTNULL OR a_strategy_id = a.fkey_strategy_id);
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_exit_strategy",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("strategy_id", Type::BigInt),
                Field::new("quantity", Type::String),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("dex", Type::String),
                Field::new("back_time", Type::BigInt),
                Field::new("transaction_hash", Type::String),
                Field::new("purchase_wallet", Type::String),
            ],
            vec![Field::new("success", Type::Boolean)],
            r#"
BEGIN
    INSERT INTO tbl.user_exit_strategy_history (fkey_user_id, fkey_strategy_id, exit_quantity, dex, back_time,
                                                exit_time, purchase_wallet, blockchain, transaction_hash)
    VALUES (a_user_id, a_strategy_id, a_quantity, a_dex, a_back_time, extract(epoch from now()), a_purchase_wallet,
            a_blockchain,
            a_transaction_hash);
    RETURN QUERY SELECT TRUE;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_list_exit_strategy_history",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("strategy_id", Type::optional(Type::BigInt)),
            ],
            vec![
                Field::new("exit_history_id", Type::BigInt),
                Field::new("strategy_id", Type::BigInt),
                Field::new("exit_quantity", Type::String),
                Field::new("purchase_wallet_address", Type::String),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("dex", Type::String),
                Field::new("back_time", Type::BigInt),
                Field::new("exit_time", Type::BigInt),
            ],
            r#"
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
"#,
        ),
        ProceduralFunction::new(
            "fun_user_follow_expert",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("expert_id", Type::BigInt),
            ],
            vec![Field::new("success", Type::Boolean)],
            r#"
BEGIN
    INSERT INTO tbl.user_follow_expert (fkey_user_id, fkey_expert_id, updated_at, created_at)
    VALUES (a_user_id, a_expert_id, extract(epoch from now())::bigint, extract(epoch from now())::bigint);
    RETURN QUERY SELECT TRUE;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_unfollow_expert",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("expert_id", Type::BigInt),
            ],
            vec![Field::new("success", Type::Boolean)],
            r#"
BEGIN
    UPDATE tbl.user_follow_expert
    SET unfollowed = TRUE
    WHERE fkey_user_id = a_user_id AND fkey_expert_id = a_expert_id;
    RETURN QUERY SELECT TRUE;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_list_followed_experts",
            vec![Field::new("user_id", Type::BigInt)],
            vec![
                Field::new("expert_id", Type::BigInt),
                Field::new("name", Type::String),
                Field::new("follower_count", Type::BigInt),
                Field::new("description", Type::String),
                Field::new("social_media", Type::String),
                Field::new("risk_score", Type::Numeric),
                Field::new("reputation_score", Type::Numeric),
                Field::new("aum", Type::Numeric),
            ],
            r#"
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
"#,
        ),
        ProceduralFunction::new(
            "fun_user_list_experts",
            vec![],
            vec![
                Field::new("expert_id", Type::BigInt),
                Field::new("name", Type::String),
                Field::new("follower_count", Type::BigInt),
                Field::new("description", Type::String),
                Field::new("social_media", Type::String),
                Field::new("risk_score", Type::Numeric),
                Field::new("reputation_score", Type::Numeric),
                Field::new("aum", Type::Numeric),
            ],
            r#"
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
"#,
        ),
        ProceduralFunction::new(
            "fun_user_get_expert_profile",
            vec![Field::new("expert_id", Type::BigInt)],
            vec![
                Field::new("expert_id", Type::BigInt),
                Field::new("name", Type::String),
                Field::new("follower_count", Type::Int),
                Field::new("description", Type::String),
                Field::new("social_media", Type::String),
                Field::new("risk_score", Type::Numeric),
                Field::new("reputation_score", Type::Numeric),
                Field::new("aum", Type::Numeric),
            ],
            r#"
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
"#,
        ),
        ProceduralFunction::new(
            "fun_user_get_user_profile",
            vec![Field::new("user_id", Type::BigInt)],
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("name", Type::String),
                Field::new("follower_count", Type::Int),
                Field::new("description", Type::String),
                Field::new("social_media", Type::String),
                Field::new("risk_score", Type::Numeric),
                Field::new("reputation_score", Type::Numeric),
                Field::new("aum", Type::Numeric),
            ],
            r#"
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
"#,
        ),
        ProceduralFunction::new(
            "fun_user_apply_become_expert",
            vec![Field::new("user_id", Type::BigInt)],
            vec![Field::new("success", Type::Boolean)],
            r#"
BEGIN
    UPDATE tbl.user SET pending_expert = TRUE WHERE pkey_id = a_user_id AND role = 'user';
    RETURN QUERY SELECT TRUE;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_create_strategy",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("name", Type::String),
                Field::new("description", Type::String),
            ],
            vec![
                Field::new("success", Type::Boolean),
                Field::new("strategy_id", Type::BigInt),
            ],
            r#"
DECLARE
    a_strategy_id BIGINT;
BEGIN
    INSERT INTO tbl.strategy (fkey_user_id, name, description, current_usdc, total_backed_usdc, total_exited_usdc)
    VALUES (a_user_id, a_name, a_description, '0', '0', '0') RETURNING pkey_id INTO a_strategy_id;
    RETURN QUERY SELECT TRUE, a_strategy_id;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_update_strategy",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("strategy_id", Type::BigInt),
                Field::new("name", Type::optional(Type::String)),
                Field::new("description", Type::optional(Type::String)),
            ],
            vec![Field::new("success", Type::Boolean)],
            r#"
            
BEGIN
    UPDATE tbl.strategy
    SET name = COALESCE(a_name, name),
        description = COALESCE(a_description, description)
    WHERE pkey_id = a_strategy_id
      AND fkey_user_id = a_user_id;
    RETURN QUERY SELECT TRUE;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_admin_update_strategy",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("strategy_id", Type::BigInt),
                Field::new("name", Type::optional(Type::String)),
                Field::new("description", Type::optional(Type::String)),
                Field::new("evm_contract_address", Type::optional(Type::String)),
            ],
            vec![Field::new("success", Type::Boolean)],
            r#"
BEGIN
    UPDATE tbl.strategy
    SET name = COALESCE(a_name, name),
        description = COALESCE(a_description, description),
        evm_contract_address = COALESCE(a_evm_contract_address, evm_contract_address)
    WHERE pkey_id = a_strategy_id
      AND fkey_user_id = a_user_id;
    RETURN QUERY SELECT TRUE;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_add_strategy_watch_wallet",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("strategy_id", Type::BigInt),
                Field::new("wallet_address", Type::String),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("ratio", Type::Numeric), // TODO: insert ratio into database
                Field::new("dex", Type::String),
            ],
            vec![
                Field::new("success", Type::Boolean),
                Field::new("watch_wallet_id", Type::BigInt),
            ],
            r#"
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
"#,
        ),
        ProceduralFunction::new(
            "fun_user_remove_strategy_watch_wallet",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("watch_wallet_id", Type::BigInt),
            ],
            vec![Field::new("success", Type::Boolean)],
            r#"
BEGIN
    DELETE FROM tbl.strategy_watching_wallet
    WHERE fkey_user_id = a_user_id
      AND pkey_id = a_watch_wallet_id;
    RETURN QUERY SELECT TRUE;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_list_strategy_watch_wallets",
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![
                Field::new("watch_wallet_id", Type::BigInt),
                Field::new("wallet_address", Type::String),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("ratio", Type::Numeric), // TODO: insert ratio into database
            ],
            r#"
BEGIN
    RETURN QUERY SELECT a.pkey_id            AS watch_wallet_id,
                        a.address            AS wallet_address,
                        a.blockchain         AS blockchain,
                        a.ratio_distribution AS ratio
                 FROM tbl.strategy_watching_wallet AS a
                 WHERE a.fkey_strategy_id = a_strategy_id;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_add_registered_wallet",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("address", Type::String),
            ],
            vec![Field::new("registered_wallet_id", Type::BigInt)],
            r#"
BEGIN
    RETURN QUERY INSERT INTO tbl.user_registered_wallet (fkey_user_id, blockchain, address, created_at)
            VALUES ( a_user_id, a_blockchain, a_address, EXTRACT(EPOCH FROM NOW())::bigint) RETURNING pkey_id;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_remove_registered_wallet",
            vec![
                Field::new("registered_wallet_id", Type::BigInt),
                Field::new("user_id", Type::BigInt),
            ],
            vec![],
            r#"
BEGIN
    DELETE FROM tbl.user_registered_wallet WHERE pkey_id = a_registered_wallet_id AND fkey_user_id = a_user_id;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_list_registered_wallets",
            vec![Field::new("user_id", Type::BigInt)],
            vec![
                Field::new("registered_wallet_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("address", Type::String),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT pkey_id, blockchain, address FROM tbl.user_registered_wallet WHERE fkey_user_id = a_user_id;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_request_refund",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("quantity", Type::String),
                Field::new("wallet_address", Type::String),
            ],
            vec![Field::new("request_refund_id", Type::BigInt)],
            r#"
BEGIN
    RETURN QUERY INSERT INTO tbl.user_request_refund_history (fkey_user_id, blockchain, quantity, wallet_address, updated_at, created_at)
            VALUES ( a_user_id, a_blockchain, a_quantity, a_wallet_address, EXTRACT(EPOCH FROM NOW())::bigint, EXTRACT(EPOCH FROM NOW())::bigint) RETURNING pkey_id;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_list_request_refund_history",
            vec![],
            vec![
                Field::new("request_refund_id", Type::BigInt),
                Field::new("user_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("quantity", Type::String),
                Field::new("wallet_address", Type::String),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT pkey_id, fkey_user_id, blockchain, quantity, wallet_address FROM tbl.user_request_refund_history;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_update_request_refund_history",
            vec![
                Field::new("request_refund_id", Type::BigInt),
                Field::new("transaction_hash", Type::String),
            ],
            vec![],
            r#"
BEGIN
    UPDATE tbl.user_request_refund_history SET
            transaction_hash = a_transaction_hash, 
            updated_at = EXTRACT(EPOCH FROM NOW())::bigint
    WHERE pkey_id = a_request_refund_id;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_add_strategy_initial_token_ratio",
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("token_name", Type::String),
                Field::new("token_address", Type::String),
                Field::new("quantity", Type::String),
            ],
            vec![Field::new("strategy_initial_token_ratio_id", Type::BigInt)],
            r#"
BEGIN
    RETURN QUERY INSERT INTO tbl.strategy_initial_token_ratio (fkey_strategy_id, token_name, token_address, quantity, created_at, updated_at)
            VALUES ( a_strategy_id, a_token_name, a_token_address, a_quantity, EXTRACT(EPOCH FROM NOW())::bigint, EXTRACT(EPOCH FROM NOW())::bigint) RETURNING pkey_id;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_remove_strategy_initial_token_ratio",
            vec![
                Field::new("strategy_initial_token_ratio_id", Type::BigInt),
                Field::new("strategy_id", Type::BigInt),
            ],
            vec![],
            r#"
BEGIN
    DELETE FROM tbl.strategy_initial_token_ratio WHERE pkey_id = a_strategy_initial_token_ratio_id AND fkey_strategy_id = a_strategy_id;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_list_strategy_initial_token_ratios",
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![
                Field::new("strategy_initial_token_ratio_id", Type::BigInt),
                Field::new("token_name", Type::String),
                Field::new("token_address", Type::String),
                Field::new("quantity", Type::String),
                Field::new("strategy_id", Type::BigInt),
                Field::new("created_at", Type::BigInt),
                Field::new("updated_at", Type::BigInt),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT pkey_id, token_name, token_address, quantity, fkey_strategy_id, updated_at, created_at FROM tbl.strategy_initial_token_ratio WHERE fkey_strategy_id = a_strategy_id;
END
"#,
        ),
    ]
}
