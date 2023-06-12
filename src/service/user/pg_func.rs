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
                Field::new("followers", Type::BigInt),
                Field::new("backers", Type::BigInt),
                Field::new("risk_score", Type::optional(Type::Numeric)),
                Field::new("aum", Type::optional(Type::Numeric)),
                Field::new("followed", Type::Boolean),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT a.pkey_id AS strategy_id,
                          a.name AS strategy_name,
                          a.description AS strategy_description,
                          0.0::double precision AS net_value,
                          (SELECT COUNT(*) FROM tbl.user_follow_strategy WHERE fkey_strategy_id = a.pkey_id AND unfollowed = FALSE) AS followers,
                          (SELECT COUNT(DISTINCT h.fkey_user_id) FROM tbl.user_back_strategy_history AS h WHERE fkey_strategy_id = a.pkey_id) AS backers,
                          a.risk_score as risk_score,
                          a.aum as aum,
                          TRUE as followed
                 FROM tbl.strategy AS a 
                     JOIN tbl.user_follow_strategy AS b ON b.fkey_strategy_id = a.pkey_id WHERE b.fkey_user_id = a_user_id AND unfollowed = FALSE
                    ;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_list_strategies",
            vec![Field::new("user_id", Type::BigInt)],
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("strategy_name", Type::String),
                Field::new("strategy_description", Type::String),
                Field::new("net_value", Type::Numeric),
                Field::new("followers", Type::BigInt),
                Field::new("backers", Type::BigInt),
                Field::new("risk_score", Type::optional(Type::Numeric)),
                Field::new("aum", Type::optional(Type::Numeric)),
                Field::new("followed", Type::Boolean),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT a.pkey_id AS strategy_id,
                          a.name AS strategy_name,
                          a.description AS strategy_description,
                          0.0::double precision AS net_value,
                          (SELECT COUNT(*) FROM tbl.user_follow_strategy WHERE fkey_strategy_id = a.pkey_id AND unfollowed = FALSE) AS followers,
                          (SELECT COUNT(DISTINCT h.fkey_user_id) FROM tbl.user_back_strategy_history AS h WHERE fkey_strategy_id = a.pkey_id) AS backers,
                          a.risk_score as risk_score,
                          a.aum as aum,
                          EXISTS(SELECT * FROM tbl.user_follow_strategy AS b WHERE b.fkey_strategy_id = a.pkey_id AND b.fkey_user_id = a_user_id) as followed
                 FROM tbl.strategy AS a
                    ;

END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_list_top_performing_strategies",
            vec![],
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("strategy_name", Type::String),
                Field::new("strategy_description", Type::String),
                Field::new("net_value", Type::Numeric),
                Field::new("followers", Type::BigInt),
                Field::new("backers", Type::BigInt),
                Field::new("risk_score", Type::optional(Type::Numeric)),
                Field::new("aum", Type::optional(Type::Numeric)),
            ],
            r#"
BEGIN

    RETURN QUERY SELECT a.pkey_id AS strategy_id,
                          a.name AS strategy_name,
                          a.description AS strategy_description,
                          0.0::double precision AS net_value,
                          (SELECT COUNT(*) FROM tbl.user_follow_strategy WHERE fkey_strategy_id = a.pkey_id AND unfollowed = FALSE) AS followers,
                          (SELECT COUNT(DISTINCT h.fkey_user_id) FROM tbl.user_back_strategy_history AS h WHERE fkey_strategy_id = a.pkey_id) AS backers,
                          a.risk_score as risk_score,
                          a.aum as aum
                 FROM tbl.strategy AS a 
                 ORDER BY a.aum
                 LIMIT 10
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
                          a.current_usdc,
                          a.total_backed_usdc,
                          a.total_exited_usdc,
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
                Field::new("followers", Type::BigInt),
                Field::new("backers", Type::BigInt),
                Field::new("risk_score", Type::optional(Type::Numeric)),
                Field::new("aum", Type::optional(Type::Numeric)),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT a.pkey_id                            AS strategy_id,
                        a.name                               AS strategy_name,
                        a.description                        AS strategy_description,
                        0.0::double precision                AS net_value,
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
                          a.exit_time AS exit_time
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
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("offset", Type::BigInt),
                Field::new("limit", Type::BigInt),
            ],
            vec![
                Field::new("expert_id", Type::BigInt),
                Field::new("user_id", Type::BigInt),
                Field::new("user_public_id", Type::BigInt),
                Field::new("listening_wallet", Type::String),
                Field::new("username", Type::String),
                Field::new("family_name", Type::optional(Type::String)),
                Field::new("given_name", Type::optional(Type::String)),
                Field::new("follower_count", Type::BigInt),
                Field::new("description", Type::optional(Type::String)),
                Field::new("social_media", Type::optional(Type::String)),
                Field::new("risk_score", Type::optional(Type::Numeric)),
                Field::new("reputation_score", Type::optional(Type::Numeric)),
                Field::new("aum", Type::optional(Type::Numeric)),
                Field::new("joined_at", Type::BigInt),
                Field::new("requested_at", Type::optional(Type::BigInt)),
                Field::new("approved_at", Type::optional(Type::BigInt)),
                Field::new("pending_expert", Type::Boolean),
                Field::new("approved_expert", Type::Boolean),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT a.pkey_id                                                 AS expert_id,
                        a.fkey_user_id                                            AS user_id,
                        c.public_id                                               AS user_public_id,
                        c.address                                                 AS listening_wallet,
                        c.username                                                AS username,
                        c.family_name                                             AS family_name,
                        c.given_name                                               AS given_name,
                        (SELECT COUNT(*)
                         FROM tbl.user_follow_expert
                         WHERE fkey_expert_id = a.pkey_id AND unfollowed = FALSE) AS follower_count,
                        a.description                                             AS description,
                        a.social_media                                            AS social_media,
                        a.risk_score                                              AS risk_score,
                        a.reputation_score                                        AS reputation_score,
                        a.aum                                                     AS aum,
                        c.created_at                                              AS joined_at,
                        a.requested_at                                            AS requested_at,
                        a.approved_at                                             AS approved_at,
                        a.pending_expert                                          AS pending_expert,
                        a.approved_expert                                         AS approved_expert
                FROM tbl.expert_profile AS a
                          JOIN tbl.user_follow_expert AS b ON b.fkey_expert_id = a.pkey_id
                          JOIN tbl.user AS c ON c.pkey_id = a.fkey_user_id
                WHERE b.fkey_user_id = a_user_id
                    AND unfollowed = FALSE
                ORDER BY a.pkey_id
                OFFSET a_offset
                LIMIT a_limit
                ;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_list_experts",
            vec![
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
                Field::new("user_id", Type::BigInt),
            ],
            vec![
                Field::new("expert_id", Type::BigInt),
                Field::new("user_id", Type::BigInt),
                Field::new("user_public_id", Type::BigInt),
                Field::new("listening_wallet", Type::String),
                Field::new("username", Type::String),
                Field::new("family_name", Type::optional(Type::String)),
                Field::new("given_name", Type::optional(Type::String)),
                Field::new("description", Type::optional(Type::String)),
                Field::new("social_media", Type::optional(Type::String)),
                Field::new("risk_score", Type::optional(Type::Numeric)),
                Field::new("reputation_score", Type::optional(Type::Numeric)),
                Field::new("aum", Type::optional(Type::Numeric)),
                Field::new("joined_at", Type::BigInt),
                Field::new("requested_at", Type::optional(Type::BigInt)),
                Field::new("approved_at", Type::optional(Type::BigInt)),
                Field::new("pending_expert", Type::Boolean),
                Field::new("approved_expert", Type::Boolean),
                Field::new("followed", Type::Boolean),
                Field::new("follower_count", Type::BigInt),
                Field::new("backer_count", Type::BigInt),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT a.pkey_id                                                 AS expert_id,
                        a.fkey_user_id                                            AS user_id,
                        c.public_id                                               AS user_public_id,
                        c.address                                                 AS listening_wallet,
                        c.username                                                AS username,
                        c.family_name                                             AS family_name,
                        c.given_name                                               AS given_name,
                        a.description                                             AS description,
                        a.social_media                                            AS social_media,
                        a.risk_score                                              AS risk_score,
                        a.reputation_score                                        AS reputation_score,
                        a.aum                                                     AS aum,
                        c.created_at                                              AS joined_at,
                        a.requested_at                                            AS requested_at,
                        a.approved_at                                             AS approved_at,
                        a.pending_expert                                          AS pending_expert,
                        a.approved_expert                                         AS approved_expert,
                        EXISTS(SELECT COUNT(*) FROM tbl.user_follow_expert WHERE fkey_expert_id = a.pkey_id AND fkey_user_id = a_user_id AND unfollowed = FALSE) AS followed,
                        (SELECT COUNT(DISTINCT d.fkey_user_id) FROM tbl.user_follow_expert AS d WHERE d.fkey_expert_id = a.pkey_id AND unfollowed = FALSE) AS follower_count,
                        (SELECT COUNT(DISTINCT d.fkey_user_id) FROM tbl.user_back_strategy_history AS d JOIN tbl.strategy AS e ON e.pkey_id = d.fkey_strategy_id WHERE e.fkey_user_id = c.pkey_id) AS backer_count
                 FROM tbl.expert_profile AS a
                          JOIN tbl.user AS c ON c.pkey_id = a.fkey_user_id
                 ORDER BY a.pkey_id
                 OFFSET a_offset
                 LIMIT a_limit
                 ;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_get_expert_profile",
            vec![
                Field::new("expert_id", Type::BigInt),
                Field::new("user_id", Type::BigInt),
            ],
            vec![
                Field::new("expert_id", Type::BigInt),
                Field::new("user_id", Type::BigInt),
                Field::new("user_public_id", Type::BigInt),
                // FIXME: right now listening_wallet is the wallet the expert uses to login. Should be multiple wallets
                Field::new("listening_wallet", Type::String),
                Field::new("username", Type::String),
                Field::new("family_name", Type::optional(Type::String)),
                Field::new("given_name", Type::optional(Type::String)),
                Field::new("follower_count", Type::BigInt),
                Field::new("description", Type::String),
                Field::new("social_media", Type::String),
                Field::new("risk_score", Type::Numeric),
                Field::new("reputation_score", Type::Numeric),
                Field::new("aum", Type::Numeric),
                Field::new("joined_at", Type::BigInt),
                Field::new("requested_at", Type::BigInt),
                Field::new("approved_at", Type::optional(Type::BigInt)),
                Field::new("pending_expert", Type::Boolean),
                Field::new("approved_expert", Type::Boolean),
                Field::new("followed", Type::Boolean),
            ],
            r#"
BEGIN

    RETURN QUERY SELECT a.pkey_id                                                 AS expert_id,
                        a.fkey_user_id                                            AS user_id,
                        b.public_id                                               AS user_public_id,
                        b.address                                                 AS listening_wallet,
                        b.username                                                AS username,
                        b.family_name                                             AS family_name,
                        b.given_name                                              AS given_name,
                        (SELECT COUNT(*)
                         FROM tbl.user_follow_expert
                         WHERE fkey_expert_id = a.pkey_id AND unfollowed = FALSE) AS follower_count,
                        a.description                                             AS description,
                        a.social_media                                            AS social_media,
                        a.risk_score                                              AS risk_score,
                        a.reputation_score                                        AS reputation_score,
                        a.aum                                                     AS aum,
                        b.created_at                                              AS joined_at,
                        a.requested_at                                            AS requested_at,
                        a.approved_at                                             AS approved_at,
                        a.pending_expert                                          AS pending_expert,
                        a.approved_expert                                         AS approved_expert,
                        EXISTS(SELECT COUNT(*) FROM tbl.user_follow_expert WHERE fkey_expert_id = a.pkey_id AND fkey_user_id = a_user_id AND unfollowed = FALSE) AS follower_count

                 FROM tbl.expert_profile AS a 
                 JOIN tbl.user AS b ON b.pkey_id = a.fkey_user_id
                 WHERE a.pkey_id = a_expert_id
                 ;

END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_get_user_profile",
            vec![Field::new("user_id", Type::BigInt)],
            vec![
                Field::new("expert_id", Type::optional(Type::BigInt)),
                Field::new("name", Type::String),
                Field::new("follower_count", Type::optional(Type::BigInt)),
                Field::new("description", Type::optional(Type::String)),
                Field::new("social_media", Type::optional(Type::String)),
                Field::new("risk_score", Type::optional(Type::Numeric)),
                Field::new("reputation_score", Type::optional(Type::Numeric)),
                Field::new("aum", Type::optional(Type::Numeric)),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT   a.pkey_id AS expert_id,
                          b.username AS name,
                          (SELECT COUNT(*) FROM tbl.user_follow_expert WHERE fkey_expert_id = a.pkey_id AND unfollowed = FALSE) AS follower_count,
                          a.description AS description,
                          a.social_media AS social_media,
                          a.risk_score AS risk_score,
                          a.reputation_score AS reputation_score,
                          a.aum AS aum
                 FROM tbl.expert_profile AS a
                 RIGHT JOIN tbl.user AS b ON b.pkey_id = a.fkey_user_id
                 WHERE b.pkey_id = a_user_id;

END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_create_expert_profile",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("description", Type::optional(Type::String)),
                Field::new("social_media", Type::optional(Type::String)),
            ],
            vec![Field::new("expert_id", Type::BigInt)],
            r#"
BEGIN
    RETURN QUERY INSERT INTO tbl.expert_profile(fkey_user_id, description, social_media, updated_at, created_at)
    VALUES(a_user_id, a_description, a_social_media, extract(epoch from now())::bigint, extract(epoch from now())::bigint) 
    RETURNING pkey_id;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_update_expert_profile",
            vec![
                Field::new("expert_id", Type::BigInt),
                Field::new("description", Type::optional(Type::String)),
                Field::new("social_media", Type::optional(Type::String)),
            ],
            vec![],
            r#"
BEGIN
    UPDATE tbl.expert_profile
    SET
        description = COALESCE(a_description, description),
        social_media = COALESCE(a_social_media, social_media),
        updated_at = extract(epoch from now())::bigint
     WHERE pkey_id = a_expert_id;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_apply_become_expert",
            vec![Field::new("user_id", Type::BigInt)],
            vec![
                Field::new("success", Type::Boolean),
                Field::new("expert_id", Type::BigInt),
            ],
            r#"
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
"#,
        ),
        ProceduralFunction::new(
            "fun_user_create_strategy",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("name", Type::String),
                Field::new("description", Type::String),
                Field::new("strategy_thesis_url", Type::String),
                Field::new("minimum_backing_amount_usd", Type::Numeric),
                Field::new("strategy_fee", Type::Numeric),
                Field::new("expert_fee", Type::Numeric),
                Field::new("agreed_tos", Type::Boolean),
            ],
            vec![
                Field::new("success", Type::Boolean),
                Field::new("strategy_id", Type::BigInt),
            ],
            r#"
DECLARE
    a_strategy_id BIGINT;
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
        strategy_fee,
        expert_fee,
        agreed_tos,
        updated_at, 
        created_at
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
        a_strategy_fee,
        a_expert_fee,
        a_agreed_tos,
        EXTRACT(EPOCH FROM NOW())::bigint, 
        EXTRACT(EPOCH FROM NOW())::bigint
    ) RETURNING pkey_id INTO a_strategy_id;
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
            "fun_user_list_strategy_followers",
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("user_public_id", Type::BigInt),
                Field::new("username", Type::String),
                Field::new("wallet_address", Type::String),
                Field::new("followed_at", Type::BigInt),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT a.fkey_user_id AS user_id,
                        b.public_id    AS user_public_id,
                        b.username     AS username,
                        b.address      AS wallet_address,
                        a.created_at   AS followed_at
                 FROM tbl.user_follow_strategy AS a
                          INNER JOIN tbl.user AS b ON a.fkey_user_id = b.pkey_id
                 WHERE a.fkey_strategy_id = a_strategy_id
                   AND a.unfollowed = FALSE;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_list_strategy_backers",
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("user_public_id", Type::BigInt),
                Field::new("username", Type::String),
                Field::new("wallet_address", Type::String),
                Field::new("backed_at", Type::BigInt),
            ],
            r#"
BEGIN
    -- TODO: need to group by user_id
    RETURN QUERY SELECT a.fkey_user_id AS user_id,
                        b.public_id    AS user_public_id,
                        b.address      AS wallet_address,
                        b.username     AS username,
                        a.back_time  AS followed_at
                 FROM tbl.user_back_strategy_history AS a
                          INNER JOIN tbl.user AS b ON a.fkey_user_id = b.pkey_id
                 WHERE a.fkey_strategy_id = a_strategy_id
                 ;
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
    RETURN QUERY SELECT a.pkey_id, a.blockchain, a.address FROM tbl.user_registered_wallet AS a WHERE fkey_user_id = a_user_id;
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
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("token_name", Type::String),
                Field::new("token_address", Type::String),
                Field::new("quantity", Type::String),
                Field::new("strategy_id", Type::BigInt),
                Field::new("created_at", Type::BigInt),
                Field::new("updated_at", Type::BigInt),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT a.pkey_id, a.blockchain, a.token_name, a.token_address, a.quantity, a.fkey_strategy_id, a.updated_at, a.created_at FROM tbl.strategy_initial_token_ratio AS a WHERE fkey_strategy_id = a_strategy_id;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_list_followers",
            vec![Field::new("user_id", Type::BigInt)],
            vec![
                Field::new("public_id", Type::BigInt),
                Field::new("username", Type::String),
                Field::new("family_name", Type::optional(Type::String)),
                Field::new("given_name", Type::optional(Type::String)),
                Field::new("followed_at", Type::BigInt),
                Field::new("joined_at", Type::BigInt),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT b.pkey_id, b.username, b.family_name, b.given_name, a.created_at, b.created_at FROM tbl.user_follow_expert AS a
            INNER JOIN tbl.user AS b ON a.fkey_user_id = b.pkey_id
            WHERE a.fkey_user_id = a_user_id;
END            
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_list_backers",
            vec![Field::new("user_id", Type::BigInt)],
            vec![
                Field::new("public_id", Type::BigInt),
                Field::new("username", Type::String),
                Field::new("family_name", Type::optional(Type::String)),
                Field::new("given_name", Type::optional(Type::String)),
                Field::new("backed_at", Type::BigInt),
                Field::new("joined_at", Type::BigInt),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT b.pkey_id, b.username, b.family_name, b.given_name, a.back_time, b.created_at FROM tbl.user_back_strategy_history AS a
            INNER JOIN tbl.user AS b ON a.fkey_user_id = b.pkey_id
            WHERE a.fkey_user_id = a_user_id;
END
            "#,
        ),
    ]
}
