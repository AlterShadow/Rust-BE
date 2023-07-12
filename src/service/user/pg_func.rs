use model::pg_func::ProceduralFunction;
use model::types::*;
include!("../shared/pg_func.rs");
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
        ProceduralFunction::new_with_row_type(
            "fun_user_list_followed_strategies",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
            ],
            strategy_row_type(),
            format!(
                r#"
BEGIN
    RETURN QUERY SELECT {strategy}
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
            "#,
                strategy = get_strategy("TRUE"),
            ),
        ),
        ProceduralFunction::new_with_row_type(
            "fun_user_list_strategies",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
                Field::new("strategy_id", Type::optional(Type::BigInt)),
                Field::new("strategy_name", Type::optional(Type::String)),
                Field::new("expert_id", Type::optional(Type::BigInt)),
                Field::new("expert_public_id", Type::optional(Type::BigInt)),
                Field::new("expert_name", Type::optional(Type::String)),
                Field::new("description", Type::optional(Type::String)),
                Field::new("blockchain", Type::optional(Type::enum_ref("block_chain"))),
                Field::new(
                    "strategy_pool_address",
                    Type::optional(Type::BlockchainAddress),
                ),
                Field::new("approved", Type::optional(Type::Boolean)),
            ],
            strategy_row_type(),
            format!(
                r#"
BEGIN
    RETURN QUERY SELECT {strategy}
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
            "#,
                strategy = get_strategy(check_if_user_follows_strategy()),
            ),
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
            "fun_user_get_strategy_statistics_follow_ledger",
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
            "fun_user_get_strategy_statistics_back_ledger",
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
            "fun_user_back_strategy",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("strategy_id", Type::BigInt),
                Field::new("quantity", Type::BlockchainDecimal),
                Field::new("new_total_backed_quantity", Type::BlockchainDecimal),
                Field::new("old_total_backed_quantity", Type::BlockchainDecimal),
                Field::new("new_current_quantity", Type::BlockchainDecimal),
                Field::new("old_current_quantity", Type::BlockchainDecimal),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("transaction_hash", Type::BlockchainTransactionHash),
                Field::new("earn_sp_tokens", Type::BlockchainDecimal),
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
            "#,
        ),
        ProceduralFunction::new_with_row_type(
            "fun_user_list_backed_strategies",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("offset", Type::BigInt),
                Field::new("limit", Type::BigInt),
            ],
            strategy_row_type(),
            format!(
                r#"
BEGIN
    RETURN QUERY SELECT DISTINCT ON (s.pkey_id) {strategy}
                 FROM tbl.strategy AS s
                      JOIN tbl.user_back_exit_strategy_ledger AS b ON b.fkey_strategy_id = s.pkey_id AND b.fkey_user_id = a_user_id
                      JOIN tbl.user AS u ON u.pkey_id = s.fkey_user_id
                      JOIN tbl.expert_watched_wallet AS w ON w.fkey_user_id = u.pkey_id
                 WHERE b.fkey_user_id = a_user_id
                 ORDER BY s.pkey_id
                 LIMIT a_limit
                 OFFSET a_offset;
END
"#,
                strategy = get_strategy(check_if_user_follows_strategy()),
            ),
        ),
        ProceduralFunction::new(
            "fun_user_list_back_strategy_ledger",
            vec![
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
                Field::new("user_id", Type::optional(Type::BigInt)),
                Field::new("strategy_id", Type::optional(Type::BigInt)),
            ],
            vec![
                Field::new("total", Type::BigInt),
                Field::new("back_ledger_id", Type::BigInt),
                Field::new("user_id", Type::BigInt),
                Field::new("strategy_id", Type::BigInt),
                Field::new("quantity", Type::BlockchainDecimal),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("transaction_hash", Type::BlockchainTransactionHash),
                Field::new("happened_at", Type::BigInt),
            ],
            r#"
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
"#,
        ),
        ProceduralFunction::new(
            "fun_user_list_exit_strategy_ledger",
            vec![
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
                Field::new("user_id", Type::optional(Type::BigInt)),
                Field::new("strategy_id", Type::optional(Type::BigInt)),
            ],
            vec![
                Field::new("total", Type::BigInt),
                Field::new("back_ledger_id", Type::BigInt),
                Field::new("user_id", Type::BigInt),
                Field::new("strategy_id", Type::BigInt),
                Field::new("quantity", Type::BlockchainDecimal),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("transaction_hash", Type::BlockchainTransactionHash),
                Field::new("happened_at", Type::BigInt),
            ],
            r#"
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
"#,
        ),
        ProceduralFunction::new(
            "fun_user_exit_strategy",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("strategy_id", Type::BigInt),
                Field::new("quantity", Type::BlockchainDecimal),
                Field::new("redeem_sp_tokens", Type::BlockchainDecimal),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("transaction_hash", Type::BlockchainTransactionHash),
            ],
            vec![Field::new("success", Type::Boolean)],
            r#"
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
"#,
        ),
        ProceduralFunction::new(
            "fun_user_list_user_strategy_pool_contract_asset_ledger_entries",
            vec![
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
                Field::new("user_id", Type::BigInt),
                Field::new("strategy_pool_contract_id", Type::BigInt),
            ],
            vec![
                Field::new("user_strategy_pool_contract_asset_ledger_id", Type::BigInt),
                Field::new("strategy_pool_contract_id", Type::BigInt),
                Field::new("strategy_id", Type::BigInt),
                Field::new("strategy_wallet_id", Type::BigInt),
                Field::new("strategy_wallet_address", Type::BlockchainAddress),
                Field::new("is_strategy_wallet_managed", Type::Boolean),
                Field::new("token_id", Type::BigInt),
                Field::new("token_symbol", Type::String),
                Field::new("token_name", Type::String),
                Field::new("token_address", Type::BlockchainAddress),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("amount", Type::BlockchainDecimal),
                Field::new("happened_at", Type::BigInt),
                Field::new("is_add", Type::Boolean),
            ],
            r#"
DECLARE
	_strategy_id BIGINT;
BEGIN
		SELECT fkey_strategy_id INTO _strategy_id
		FROM tbl.strategy_pool_contract
		WHERE pkey_id = a_strategy_pool_contract_id;
		
		ASSERT _strategy_id IS NOT NULL;

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
			_strategy_id AS strategy_id,
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
		WHERE uspcal.fkey_strategy_pool_contract_id = a_strategy_pool_contract_id
		ORDER BY uspcal.happened_at DESC
		LIMIT a_limit
		OFFSET a_offset;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_add_user_strategy_pool_contract_asset_ledger_entry",
            vec![
                Field::new("strategy_wallet_id", Type::BigInt),
                Field::new("strategy_pool_contract_id", Type::BigInt),
                Field::new("token_address", Type::BlockchainAddress),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("amount", Type::BlockchainDecimal),
                Field::new("is_add", Type::Boolean),
            ],
            vec![Field::new("success", Type::Boolean)],
            r#"
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
"#,
        ),
        ProceduralFunction::new(
            "fun_user_list_user_strategy_pool_contract_asset_balances",
            vec![
                Field::new("strategy_pool_contract_id", Type::optional(Type::BigInt)),
                Field::new("user_id", Type::optional(Type::BigInt)),
                Field::new("strategy_wallet_id", Type::optional(Type::BigInt)),
                Field::new("token_address", Type::optional(Type::BlockchainAddress)),
                Field::new("blockchain", Type::optional(Type::enum_ref("block_chain"))),
            ],
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("strategy_wallet_id", Type::BigInt),
                Field::new("strategy_wallet_address", Type::BlockchainAddress),
                Field::new("is_strategy_wallet_managed", Type::Boolean),
                Field::new("token_id", Type::BigInt),
                Field::new("token_name", Type::String),
                Field::new("token_symbol", Type::String),
                Field::new("token_address", Type::BlockchainAddress),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("balance", Type::BlockchainDecimal),
            ],
            r#"
BEGIN
	RETURN QUERY
		WITH tokens AS (
		SELECT etca.pkey_id, etca.address, etca.symbol, etca.short_name, etca.blockchain
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
"#,
        ),
        ProceduralFunction::new(
            "fun_user_upsert_user_strategy_pool_contract_asset_balance",
            vec![
                Field::new("strategy_wallet_id", Type::BigInt),
                Field::new("strategy_pool_contract_id", Type::BigInt),
                Field::new("token_address", Type::BlockchainAddress),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("old_balance", Type::BlockchainDecimal),
                Field::new("new_balance", Type::BlockchainDecimal),
                Field::new("amount", Type::BlockchainDecimal),
                Field::new("is_add", Type::Boolean),
                Field::new("transaction_hash", Type::BlockchainTransactionHash),
            ],
            vec![Field::new(
                "user_strategy_pool_contract_asset_balance_id",
                Type::BigInt,
            )],
            r#"
DECLARE
	_token_id BIGINT;
    _strategy_id BIGINT;
	_blockchain enum_block_chain;
	_user_strategy_pool_contract_asset_balance_id BIGINT;
	_user_strategy_pool_contract_asset_balance_old_balance VARCHAR;
	_pkey_id BIGINT;
BEGIN
	SELECT etca.pkey_id, etca.blockchain INTO _token_id, _blockchain
	FROM tbl.escrow_token_contract_address AS etca
	WHERE etca.address = a_token_address AND etca.blockchain = a_blockchain;
    
	ASSERT _token_id IS NOT NULL;
	
	SELECT usw.fkey_strategy_id INTO _strategy_id
	FROM tbl.strategy_pool_contract AS usw
	WHERE usw.pkey_id = a_strategy_pool_contract_id;
	ASSERT _strategy_id IS NOT NULL;
    INSERT INTO tbl.strategy_pool_contract_asset_ledger (
        fkey_strategy_id,
        fkey_token_id,
        blockchain,
        amount,
        is_add,
        happened_at,
        transaction_hash
    ) VALUES (
        _strategy_id,
        _token_id,
        _blockchain,
        a_amount,
        a_is_add,
        EXTRACT(EPOCH FROM NOW()),
        a_transaction_hash
    );
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
"#,
        ),
        ProceduralFunction::new(
            "fun_user_add_strategy_pool_contract_asset_ledger_entry",
            vec![
                Field::new("strategy_pool_contract_id", Type::BigInt),
                Field::new("token_address", Type::BlockchainAddress),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("transaction_hash", Type::BlockchainTransactionHash),
                Field::new("amount", Type::BlockchainDecimal),
                Field::new("is_add", Type::Boolean),
            ],
            vec![Field::new("success", Type::Boolean)],
            r#"
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
"#,
        ),
        ProceduralFunction::new(
            "fun_user_list_strategy_pool_contract_asset_ledger",
            vec![
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
                Field::new("strategy_id", Type::optional(Type::BigInt)),
                Field::new("token_id", Type::optional(Type::BigInt)),
                Field::new("blockchain", Type::optional(Type::enum_ref("block_chain"))),
            ],
            vec![
                Field::new("entry_id", Type::BigInt),
                Field::new("strategy_id", Type::BigInt),
                Field::new("token_id", Type::BigInt),
                Field::new("token_symbol", Type::String),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("transaction_hash", Type::BlockchainTransactionHash),
                Field::new("dex", Type::optional(Type::String)),
                Field::new("amount", Type::BlockchainDecimal),
                Field::new("is_add", Type::Boolean),
                Field::new("happened_at", Type::BigInt),
            ],
            r#"
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
        ProceduralFunction::new_with_row_type(
            "fun_user_list_followed_experts",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("offset", Type::BigInt),
                Field::new("limit", Type::BigInt),
            ],
            expert_row_type(),
            format!(
                r#"
    BEGIN
        RETURN QUERY SELECT {expert}
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
    "#,
                expert = get_expert("TRUE"),
            ),
        ),
        ProceduralFunction::new_with_row_type(
            "fun_user_list_experts",
            vec![
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
                Field::new("user_id", Type::BigInt),
                Field::new("sort_by_followers", Type::Boolean),
                Field::new("expert_id", Type::optional(Type::BigInt)),
                Field::new("expert_user_id", Type::optional(Type::BigInt)),
                Field::new("expert_user_public_id", Type::optional(Type::BigInt)),
                Field::new("username", Type::optional(Type::String)),
                Field::new("family_name", Type::optional(Type::String)),
                Field::new("given_name", Type::optional(Type::String)),
                Field::new("description", Type::optional(Type::String)),
                Field::new("social_media", Type::optional(Type::String)),
            ],
            expert_row_type(),
            format!(
                r#"
BEGIN
    RETURN QUERY SELECT {expert}
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
"#,
                expert = get_expert(check_if_user_follows_expert())
            ),
        ),
        ProceduralFunction::new_with_row_type(
            "fun_user_get_expert_profile",
            vec![
                Field::new("expert_id", Type::BigInt),
                Field::new("user_id", Type::BigInt),
            ],
            expert_row_type(),
            format!(
                r#"
BEGIN

    RETURN QUERY SELECT {expert}
                 FROM tbl.expert_profile AS e
                 JOIN tbl.user AS u ON u.pkey_id = e.fkey_user_id
                 WHERE e.pkey_id = a_expert_id
                 ;

END
"#,
                expert = get_expert(check_if_user_follows_expert())
            ),
        ),
        ProceduralFunction::new(
            "fun_user_get_user_profile",
            vec![Field::new("user_id", Type::BigInt)],
            vec![
                Field::new("expert_id", Type::optional(Type::BigInt)),
                Field::new("user_public_id", Type::BigInt),
                Field::new("name", Type::String),
                Field::new("login_wallet", Type::String),
                Field::new("joined_at", Type::BigInt),
                Field::new("follower_count", Type::optional(Type::BigInt)),
                Field::new("description", Type::optional(Type::String)),
                Field::new("social_media", Type::optional(Type::String)),
                Field::new("risk_score", Type::optional(Type::Numeric)),
                Field::new("reputation_score", Type::optional(Type::Numeric)),
                Field::new("aum", Type::optional(Type::Numeric)),
            ],
            r#"
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
                Field::new("swap_fee", Type::Numeric),
                Field::new("expert_fee", Type::Numeric),
                Field::new("agreed_tos", Type::Boolean),
                Field::new("wallet_address", Type::BlockchainAddress),
                Field::new("blockchain", Type::enum_ref("block_chain")),
            ],
            vec![
                Field::new("success", Type::Boolean),
                Field::new("strategy_id", Type::BigInt),
            ],
            r#"
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
"#,
        ),
        ProceduralFunction::new(
            "fun_user_update_strategy",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("strategy_id", Type::BigInt),
                Field::new("name", Type::optional(Type::String)),
                Field::new("description", Type::optional(Type::String)),
                Field::new("social_media", Type::optional(Type::String)),
            ],
            vec![Field::new("success", Type::Boolean)],
            r#"
            
BEGIN
    UPDATE tbl.strategy
    SET name = COALESCE(a_name, name),
        description = COALESCE(a_description, description),
        social_media = COALESCE(a_social_media, social_media)
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
                Field::new("wallet_address", Type::BlockchainAddress),
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
"#,
        ),
        ProceduralFunction::new(
            "fun_user_remove_strategy_watch_wallet",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("strategy_id", Type::BigInt),
                Field::new("watch_wallet_id", Type::BigInt),
            ],
            vec![Field::new("success", Type::Boolean)],
            r#"
BEGIN
    DELETE FROM tbl.strategy_watched_wallet AS sww
    WHERE (SELECT fkey_user_id from tbl.expert_watched_wallet WHERE pkey_id = sww.fkey_expert_watched_wallet_id) = a_user_id
      AND pkey_id = a_watch_wallet_id
			AND fkey_strategy_id = a_strategy_id;
    RETURN QUERY SELECT TRUE;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_list_strategy_watch_wallets",
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![
                Field::new("watch_wallet_id", Type::BigInt),
                Field::new("wallet_address", Type::BlockchainAddress),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("ratio", Type::Numeric), // TODO: insert ratio into database
            ],
            r#"
BEGIN
		RETURN QUERY
		SELECT
				sw.pkey_id AS watch_wallet_id,
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
"#,
        ),
        ProceduralFunction::new(
            "fun_user_list_strategy_followers",
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![
                Field::new("total", Type::BigInt),
                Field::new("user_id", Type::BigInt),
                Field::new("user_public_id", Type::BigInt),
                Field::new("username", Type::String),
                Field::new("wallet_address", Type::BlockchainAddress),
                Field::new("followed_at", Type::BigInt),
            ],
            r#"
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
"#,
        ),
        ProceduralFunction::new(
            "fun_user_list_strategy_backers",
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("total", Type::BigInt),
                Field::new("user_public_id", Type::BigInt),
                Field::new("username", Type::String),
                Field::new("wallet_address", Type::BlockchainAddress),
                Field::new("backed_at", Type::BigInt),
            ],
            r#"
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
"#,
        ),
        ProceduralFunction::new(
            "fun_user_add_whitelisted_wallet",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("address", Type::BlockchainAddress),
            ],
            vec![Field::new("whitelisted_wallet_id", Type::BigInt)],
            r#"
BEGIN
    RETURN QUERY INSERT INTO tbl.user_whitelisted_wallet (fkey_user_id, blockchain, address, created_at)
            VALUES ( a_user_id, a_blockchain, a_address, EXTRACT(EPOCH FROM NOW())::bigint) RETURNING pkey_id;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_remove_whitelisted_wallet",
            vec![
                Field::new("whitelisted_wallet_id", Type::BigInt),
                Field::new("user_id", Type::BigInt),
            ],
            vec![],
            r#"
BEGIN
    DELETE FROM tbl.user_whitelisted_wallet WHERE pkey_id = a_whitelisted_wallet_id AND fkey_user_id = a_user_id;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_list_whitelisted_wallets",
            vec![
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
                Field::new("user_id", Type::optional(Type::BigInt)),
                Field::new("blockchain", Type::optional(Type::enum_ref("block_chain"))),
                Field::new("address", Type::optional(Type::BlockchainAddress)),
            ],
            vec![
                Field::new("registered_wallet_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("address", Type::BlockchainAddress),
            ],
            r#"
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
"#,
        ),
        ProceduralFunction::new(
            "fun_user_reduce_quantity_from_user_deposit_withdraw_ledger",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("token_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("user_address", Type::BlockchainAddress),
                Field::new("contract_address", Type::BlockchainAddress),
                Field::new("contract_address_id", Type::BigInt),
                Field::new("receiver_address", Type::BlockchainAddress),
                Field::new("quantity", Type::BlockchainDecimal),
                Field::new("transaction_hash", Type::BlockchainTransactionHash),
            ],
            vec![Field::new("request_refund_id", Type::BigInt)],
            r#"
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
                  a_token_id,
                  a_blockchain,
                  a_user_address,
                  a_contract_address,
                  a_contract_address_id,
                  a_receiver_address,
                  a_quantity,
                  a_transaction_hash,
                  FALSE,
                  FALSE,
                  TRUE,
                  EXTRACT(EPOCH FROM NOW())::bigint
        ) RETURNING pkey_id;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_request_refund",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("token_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("user_address", Type::BlockchainAddress),
                Field::new("contract_address", Type::BlockchainAddress),
                Field::new("contract_address_id", Type::BigInt),
                Field::new("receiver_address", Type::BlockchainAddress),
                Field::new("quantity", Type::BlockchainDecimal),
                Field::new("transaction_hash", Type::BlockchainTransactionHash),
            ],
            vec![Field::new("request_refund_id", Type::BigInt)],
            r#"
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
"#,
        ),
        ProceduralFunction::new(
            "fun_user_list_request_refund_ledger",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
            ],
            vec![
                Field::new("request_refund_id", Type::BigInt),
                Field::new("user_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("quantity", Type::BlockchainDecimal),
                Field::new("wallet_address", Type::BlockchainAddress),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT a.pkey_id, a.fkey_user_id, a.blockchain, a.quantity, a.user_address
		FROM tbl.user_deposit_withdraw_ledger AS a
		WHERE fkey_user_id = a_user_id AND is_withdraw = FALSE
		ORDER BY a.pkey_id DESC
		LIMIT a_limit
		OFFSET a_offset;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_add_strategy_initial_token_ratio",
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("token_id", Type::BigInt),
                Field::new("quantity", Type::BlockchainDecimal),
            ],
            vec![Field::new("strategy_initial_token_ratio_id", Type::BigInt)],
            r#"
BEGIN
    RETURN QUERY INSERT INTO tbl.strategy_initial_token_ratio (fkey_strategy_id, token_id, quantity, created_at, updated_at)
            VALUES ( a_strategy_id, a_token_id, a_quantity, EXTRACT(EPOCH FROM NOW())::bigint, EXTRACT(EPOCH FROM NOW())::bigint) RETURNING pkey_id;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_update_strategy_initial_token_ratio",
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("token_id", Type::BigInt),
                Field::new("new_quantity", Type::BlockchainDecimal),
            ],
            vec![],
            r#"
BEGIN
		UPDATE tbl.strategy_initial_token_ratio
				SET quantity = a_new_quantity, updated_at = EXTRACT(EPOCH FROM NOW())::bigint
				WHERE fkey_strategy_id = a_strategy_id AND token_id = a_token_id;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_remove_strategy_initial_token_ratio",
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("token_id", Type::BigInt),
            ],
            vec![],
            r#"
BEGIN
    DELETE FROM tbl.strategy_initial_token_ratio 
    WHERE fkey_strategy_id = a_strategy_id AND token_id = a_token_id;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_list_strategy_initial_token_ratios",
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("token_id", Type::optional(Type::BigInt)),
                Field::new("token_address", Type::optional(Type::BlockchainAddress)),
                Field::new("blockchain", Type::optional(Type::enum_ref("block_chain"))),
            ],
            vec![
                Field::new("total", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("token_id", Type::BigInt),
                Field::new("token_name", Type::String),
                Field::new("token_address", Type::BlockchainAddress),
                Field::new("quantity", Type::BlockchainDecimal),
                Field::new("strategy_id", Type::BigInt),
                Field::new("created_at", Type::BigInt),
                Field::new("updated_at", Type::BigInt),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT
        COUNT(*) OVER() AS total,
        b.blockchain,
        a.token_id,
        b.short_name,
        b.address,
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
"#,
        ),
        ProceduralFunction::new(
            "fun_expert_list_followers",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
            ],
            vec![
                Field::new("total", Type::BigInt),
                Field::new("public_id", Type::BigInt),
                Field::new("username", Type::String),
                Field::new("family_name", Type::optional(Type::String)),
                Field::new("given_name", Type::optional(Type::String)),
                Field::new("linked_wallet", Type::BlockchainAddress),
                Field::new("followed_at", Type::BigInt),
                Field::new("joined_at", Type::BigInt),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT 
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
            ORDER BY a.pkey_id
            LIMIT a_limit
            OFFSET a_offset;

END            
            "#,
        ),
        ProceduralFunction::new(
            "fun_expert_list_backers",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
            ],
            vec![
                Field::new("total", Type::BigInt),
                Field::new("public_id", Type::BigInt),
                Field::new("username", Type::String),
                Field::new("family_name", Type::optional(Type::String)),
                Field::new("given_name", Type::optional(Type::String)),
                Field::new("linked_wallet", Type::BlockchainAddress),
                Field::new("backed_at", Type::BigInt),
                Field::new("joined_at", Type::BigInt),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT
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
            ORDER BY a.pkey_id
            LIMIT a_limit
            OFFSET a_offset;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_list_deposit_withdraw_ledger",
            vec![
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
                Field::new("user_id", Type::optional(Type::BigInt)),
                Field::new("is_deposit", Type::optional(Type::Boolean)),
                Field::new("is_back", Type::optional(Type::Boolean)),
                Field::new("is_withdraw", Type::optional(Type::Boolean)),
                Field::new("blockchain", Type::optional(Type::enum_ref("block_chain"))),
            ],
            vec![
                Field::new("total", Type::BigInt),
                Field::new("transaction_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("user_address", Type::BlockchainAddress),
                Field::new("contract_address", Type::BlockchainAddress),
                Field::new("receiver_address", Type::BlockchainAddress),
                Field::new("quantity", Type::BlockchainDecimal),
                Field::new("transaction_hash", Type::BlockchainTransactionHash),
                Field::new("is_deposit", Type::Boolean),
                Field::new("happened_at", Type::BigInt),
            ],
            r#"
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
"#,
        ),
        ProceduralFunction::new(
            "fun_user_get_user_by_address",
            vec![Field::new("address", Type::BlockchainAddress)],
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("user_public_id", Type::BigInt),
                Field::new("username", Type::String),
                Field::new("family_name", Type::optional(Type::String)),
                Field::new("given_name", Type::optional(Type::String)),
                Field::new("joined_at", Type::BigInt),
            ],
            // TODO: it should later looking up user_registered_wallet table
            r#"
BEGIN
    RETURN QUERY SELECT 
            a.pkey_id, 
            a.public_id,
            a.username, 
            a.family_name,
            a.given_name, 
            a.created_at 
            FROM tbl.user AS a WHERE a.address = a_address;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_add_strategy_wallet",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("address", Type::BlockchainAddress),
                Field::new("is_platform_managed", Type::Boolean),
            ],
            vec![],
            r#"
BEGIN
    INSERT INTO tbl.user_strategy_wallet (fkey_user_id, blockchain, address, is_platform_managed, created_at) 
    VALUES (a_user_id, a_blockchain, a_address, a_is_platform_managed, EXTRACT(EPOCH FROM NOW())::BIGINT);
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_list_strategy_wallets",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("blockchain", Type::optional(Type::enum_ref("block_chain"))),
            ],
            vec![
                Field::new("total", Type::BigInt),
                Field::new("wallet_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("address", Type::BlockchainAddress),
                Field::new("is_platform_managed", Type::Boolean),
                Field::new("created_at", Type::BigInt),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT 
        COUNT(*) OVER() AS total,
        a.pkey_id,
        a.blockchain,
        a.address, 
        a.is_platform_managed,
        a.created_at 
    FROM tbl.user_strategy_wallet AS a 
    WHERE a.fkey_user_id = a_user_id 
        AND (a_blockchain ISNULL OR a.blockchain = a_blockchain);
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_list_strategy_audit_rules",
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("audit_rule_id", Type::optional(Type::BigInt)),
            ],
            vec![
                Field::new("rule_id", Type::BigInt),
                Field::new("created_at", Type::BigInt),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT a.fkey_audit_rule_id, a.created_at
    FROM tbl.strategy_audit_rule AS a
    WHERE a.fkey_strategy_id = a_strategy_id
    AND (a_audit_rule_id ISNULL OR a.fkey_audit_rule_id = a_audit_rule_id);
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_add_strategy_audit_rule",
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("audit_rule_id", Type::BigInt),
            ],
            vec![],
            r#"
BEGIN
    INSERT INTO tbl.strategy_audit_rule (fkey_strategy_id, fkey_audit_rule_id, updated_at, created_at)
    VALUES (a_strategy_id, a_audit_rule_id, EXTRACT(EPOCH FROM NOW())::BIGINT, EXTRACT(EPOCH FROM NOW())::BIGINT);
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_del_strategy_audit_rule",
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("audit_rule_id", Type::BigInt),
            ],
            vec![],
            r#"
BEGIN
    DELETE FROM tbl.strategy_audit_rule AS a
    WHERE a.fkey_strategy_id = a_strategy_id
    AND a.fkey_audit_rule_id = a_audit_rule_id;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_add_strategy_whitelisted_token",
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("token_name", Type::String),
            ],
            vec![],
            r#"
BEGIN
    INSERT INTO tbl.strategy_whitelisted_token (fkey_strategy_id, token_name)
    VALUES (a_strategy_id, a_token_name);
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_list_strategy_whitelisted_tokens",
            vec![Field::new("strategy_id", Type::BigInt)],
            vec![Field::new("token_name", Type::String)],
            r#"
BEGIN
    RETURN QUERY SELECT a.token_name
    FROM tbl.strategy_whitelisted_token AS a
    WHERE a.fkey_strategy_id = a_strategy_id;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_check_if_token_whitelisted",
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("token_name", Type::String),
            ],
            vec![Field::new("whitelisted", Type::Boolean)],
            r#"
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
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_list_audit_rules",
            vec![Field::new("audit_rule_id", Type::optional(Type::BigInt))],
            vec![
                Field::new("rule_id", Type::BigInt),
                Field::new("name", Type::String),
                Field::new("description", Type::String),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT a.pkey_id, a.name, a.description
    FROM tbl.audit_rule AS a
    WHERE (a_audit_rule_id ISNULL OR a.pkey_id = a_audit_rule_id);
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_get_strategy_id_from_watching_wallet",
            vec![
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("address", Type::BlockchainAddress),
            ],
            vec![Field::new("strategy_id", Type::BigInt)],
            r#"
BEGIN
    RETURN QUERY SELECT sww.fkey_strategy_id
    FROM tbl.strategy_watched_wallet AS sww
    JOIN tbl.expert_watched_wallet AS eww ON sww.fkey_expert_watched_wallet_id = eww.pkey_id
    WHERE eww.blockchain = a_blockchain
        AND eww.address = a_address;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_list_user_deposit_withdraw_balance",
            vec![
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
                Field::new("user_id", Type::BigInt),
                Field::new("blockchain", Type::optional(Type::enum_ref("block_chain"))),
                Field::new("token_address", Type::optional(Type::BlockchainAddress)),
                Field::new("token_id", Type::optional(Type::BigInt)),
                Field::new(
                    "escrow_contract_address",
                    Type::optional(Type::BlockchainAddress),
                ),
            ],
            vec![
                Field::new("deposit_withdraw_balance_id", Type::BigInt),
                Field::new("user_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("token_id", Type::BigInt),
                Field::new("token_symbol", Type::String),
                Field::new("token_name", Type::String),
                Field::new("balance", Type::BlockchainDecimal),
            ],
            r#"
DECLARE
    _token_id bigint;
BEGIN
    IF a_token_address ISNULL THEN
        SELECT pkey_id INTO _token_id FROM tbl.escrow_token_contract_address AS a WHERE a.address = a_token_address AND a.blockchain = a_blockchain;
    ELSE
        _token_id := a_token_id;
    END IF;
   
    RETURN QUERY SELECT
        a.pkey_id,
        a.fkey_user_id,
        etc.blockchain,
        etc.pkey_id,
        etc.symbol,
        etc.short_name,
        a.balance
    FROM tbl.user_deposit_withdraw_balance AS a
    JOIN tbl.escrow_token_contract_address AS etc ON etc.pkey_id = a.fkey_token_id
    JOIN tbl.escrow_contract_address AS eca ON eca.pkey_id = a.fkey_escrow_contract_address_id
    WHERE a.fkey_user_id = a_user_id
        AND (a_blockchain ISNULL OR etc.blockchain = a_blockchain)
        AND (a_token_address iSNULL OR etc.address = a_token_address)
        AND (a_escrow_contract_address ISNULL OR eca.address = a_escrow_contract_address)
    ORDER BY a.pkey_id
    LIMIT a_limit
    OFFSET a_offset;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_update_user_deposit_withdraw_balance",
            vec![
                Field::new("deposit_withdraw_balance_id", Type::BigInt),
                Field::new("old_balance", Type::BlockchainDecimal),
                Field::new("new_balance", Type::BlockchainDecimal),
            ],
            vec![Field::new("updated", Type::Boolean)],
            r#"
DECLARE
    _old_balance varchar;
BEGIN
    SELECT balance INTO _old_balance FROM tbl.user_deposit_withdraw_balance WHERE pkey_id = a_deposit_withdraw_balance_id;
    IF _old_balance <> a_old_balance THEN
        RETURN QUERY SELECT FALSE;
    END IF;
    UPDATE tbl.user_deposit_withdraw_balance SET balance = a_new_balance WHERE pkey_id = a_deposit_withdraw_balance_id;
    RETURN QUERY SELECT TRUE;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_user_add_strategy_pool_contract",
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("address", Type::BlockchainAddress),
            ],
            vec![Field::new("strategy_pool_contract_id", Type::BigInt)],
            r#"
BEGIN
    UPDATE tbl.strategy
    SET strategy_pool_address = a_address
    WHERE pkey_id = a_strategy_id
        AND strategy_pool_address ISNULL;
    RETURN QUERY INSERT INTO tbl.strategy_pool_contract (fkey_strategy_id, blockchain, address, created_at)
    VALUES (a_strategy_id, a_blockchain, a_address, EXTRACT(EPOCH FROM NOW()))
    RETURNING pkey_id;
END
        "#,
        ),
        ProceduralFunction::new(
            "fun_user_list_escrow_token_contract_address",
            vec![
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
                Field::new("token_id", Type::optional(Type::BigInt)),
                Field::new("blockchain", Type::optional(Type::enum_ref("block_chain"))),
                Field::new("address", Type::optional(Type::BlockchainAddress)),
                Field::new("symbol", Type::optional(Type::String)),
                Field::new("is_stablecoin", Type::optional(Type::Boolean)),
            ],
            vec![
                Field::new("total", Type::BigInt),
                Field::new("token_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("address", Type::BlockchainAddress),
                Field::new("symbol", Type::String),
                Field::new("short_name", Type::String),
                Field::new("description", Type::String),
                Field::new("is_stablecoin", Type::Boolean),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT
        COUNT(*) OVER() AS total,
        a.pkey_id,
        a.blockchain,
        a.address,
        a.symbol,
        a.short_name,
        a.description,
        a.is_stablecoin
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
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_list_user_strategy_balance",
            vec![
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
                Field::new("user_id", Type::BigInt),
                Field::new("strategy_id", Type::optional(Type::BigInt)),
            ],
            vec![
                Field::new("total", Type::BigInt),
                Field::new("strategy_id", Type::BigInt),
                Field::new("strategy_name", Type::String),
                Field::new("balance", Type::BlockchainDecimal),
                Field::new("user_strategy_wallet_address", Type::BlockchainAddress),
                Field::new("blockchain", Type::enum_ref("block_chain")),
            ],
            r#"
BEGIN
            SELECT 
                    COUNT(*) OVER() AS total,
                    spc.fkey_strategy_id,
                    s.pkey_id,
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
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_save_user_back_strategy_attempt",
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("user_id", Type::BigInt),
                Field::new("token_id", Type::BigInt),
                Field::new("back_quantity", Type::BlockchainDecimal),
                Field::new("strategy_wallet_address", Type::BlockchainAddress),
                Field::new("log_id", Type::BigInt),
            ],
            vec![Field::new("user_back_strategy_attempt_id", Type::BigInt)],
            r#"
BEGIN
    RETURN QUERY INSERT INTO tbl.user_back_strategy_attempt (fkey_strategy_id, fkey_user_id, fkey_token_id, back_quantity, strategy_wallet_address, log_id, happened_at)
        VALUES (a_strategy_id, a_user_id, a_token_id, a_back_quantity, a_strategy_wallet_address, a_log_id, EXTRACT(EPOCH FROM NOW()))
        RETURNING pkey_id;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_list_user_back_strategy_attempt",
            vec![
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
                Field::new("user_id", Type::optional(Type::BigInt)),
                Field::new("strategy_id", Type::optional(Type::BigInt)),
                Field::new("token_id", Type::optional(Type::BigInt)),
            ],
            vec![
                Field::new("total", Type::BigInt),
                Field::new("user_back_strategy_attempt_id", Type::BigInt),
                Field::new("strategy_id", Type::BigInt),
                Field::new("strategy_name", Type::String),
                Field::new("token_id", Type::BigInt),
                Field::new("token_symbol", Type::String),
                Field::new("back_quantity", Type::BlockchainDecimal),
                Field::new("strategy_wallet_address", Type::BlockchainAddress),
                Field::new("log_id", Type::BigInt),
                Field::new("happened_at", Type::BigInt),
            ],
            r#"
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
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_save_user_back_strategy_log",
            vec![
                Field::new("user_back_strategy_attempt_id", Type::BigInt),
                Field::new("message", Type::String),
            ],
            vec![],
            r#"
BEGIN
    INSERT INTO tbl.user_back_strategy_log (fkey_user_back_strategy_attempt_id, message, happened_at)
        VALUES (a_user_back_strategy_attempt_id, a_message, EXTRACT(EPOCH FROM NOW()));
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_list_user_back_strategy_log",
            vec![
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
                Field::new("user_back_strategy_attempt_id", Type::BigInt),
            ],
            vec![
                Field::new("total", Type::BigInt),
                Field::new("log_entry_id", Type::BigInt),
                Field::new("message", Type::String),
                Field::new("happened_at", Type::BigInt),
            ],
            r#"
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
            "#,
        ),
        ProceduralFunction::new(
            "fun_user_calculate_user_escrow_balance_from_ledger",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("token_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("wallet_address", Type::optional(Type::BlockchainAddress)),
            ],
            vec![
                Field::new("wallet_address", Type::BlockchainAddress),
                Field::new("balance", Type::BlockchainDecimal),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT
						a.user_address,
            CAST(SUM(CAST(a.quantity AS NUMERIC) 
                * CASE
                     WHEN a.is_deposit THEN 1
                     ELSE -1 
                 END
            ) AS VARCHAR)
		FROM tbl.user_deposit_withdraw_ledger AS a
		WHERE a.blockchain = a_blockchain
		    AND a.fkey_user_id = a_user_id
            AND a.fkey_token_id = a_token_id
            AND  (a_wallet_address ISNULL OR a.user_address = a_wallet_address)
						GROUP BY a.user_address
        ;
END
            "#,
        ),
    ]
}
