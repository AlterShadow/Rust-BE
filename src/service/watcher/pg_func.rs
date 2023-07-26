use model::pg_func::ProceduralFunction;
use model::types::*;

pub fn get_trade_watcher_pg_func() -> Vec<ProceduralFunction> {
    vec![
        ProceduralFunction::new(
            "fun_watcher_save_raw_transaction",
            vec![
                Field::new("transaction_hash", Type::BlockchainTransactionHash),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("raw_transaction", Type::String),
                Field::new("dex", Type::optional(Type::String)),
            ],
            vec![Field::new("transaction_cache_id", Type::BigInt)],
            r#"
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
        "#,
        ),
        ProceduralFunction::new(
            "fun_watcher_get_raw_transaction",
            vec![
                Field::new("transaction_hash", Type::BlockchainTransactionHash),
                Field::new("chain", Type::String),
                Field::new("dex", Type::optional(Type::String)),
            ],
            vec![
                Field::new("transaction_cache_id", Type::BigInt),
                Field::new("transaction_hash", Type::BlockchainTransactionHash),
                Field::new("chain", Type::String),
                Field::new("dex", Type::optional(Type::String)),
                Field::new("raw_transaction", Type::String),
                Field::new("created_at", Type::BigInt),
            ],
            r#"
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
        "#,
        ),
        ProceduralFunction::new(
            "fun_watcher_save_strategy_watching_wallet_trade_ledger",
            vec![
                Field::new("address", Type::BlockchainAddress),
                Field::new("transaction_hash", Type::BlockchainTransactionHash),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("contract_address", Type::BlockchainAddress),
                Field::new("dex", Type::optional(Type::String)),
                Field::new("token_in_address", Type::optional(Type::BlockchainAddress)),
                Field::new("token_out_address", Type::optional(Type::BlockchainAddress)),
                Field::new("amount_in", Type::optional(Type::BlockchainDecimal)),
                Field::new("amount_out", Type::optional(Type::BlockchainDecimal)),
                Field::new("happened_at", Type::optional(Type::BigInt)),
            ],
            vec![
                Field::new("strategy_watching_wallet_trade_ledger_id", Type::BigInt),
                Field::new("expert_watched_wallet_id", Type::BigInt),
                Field::new("fkey_token_in", Type::BigInt),
                Field::new("fkey_token_in_name", Type::String),
                Field::new("fkey_token_out", Type::BigInt),
                Field::new("fkey_token_out_name", Type::String),
            ],
            r#"
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
        "#,
        ),
        ProceduralFunction::new(
            "fun_watcher_list_dex_path_for_pair",
            vec![
                Field::new("token_in_address", Type::BlockchainAddress),
                Field::new("token_out_address", Type::BlockchainAddress),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("dex", Type::optional(Type::enum_ref("dex"))),
                Field::new("format", Type::optional(Type::enum_ref("dex_path_format"))),
            ],
            vec![
                Field::new("pkey_id", Type::BigInt),
                Field::new("token_in_id", Type::BigInt),
                Field::new("token_out_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("dex", Type::enum_ref("dex")),
                Field::new("format", Type::enum_ref("dex_path_format")),
                Field::new("path_data", Type::String),
                Field::new("updated_at", Type::BigInt),
            ],
            r#"
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
"#,
        ),
        ProceduralFunction::new(
            "fun_watcher_upsert_dex_path_for_pair",
            vec![
                Field::new("token_in_address", Type::BlockchainAddress),
                Field::new("token_out_address", Type::BlockchainAddress),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("dex", Type::enum_ref("dex")),
                Field::new("format", Type::enum_ref("dex_path_format")),
                Field::new("path_data", Type::String),
            ],
            vec![Field::new("dex_path_for_pair_id", Type::BigInt)],
            r#"
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
"#,
        ),
        ProceduralFunction::new(
            "fun_watcher_list_last_dex_trades_for_pair",
            vec![
                Field::new("token_in_address", Type::BlockchainAddress),
                Field::new("token_out_address", Type::BlockchainAddress),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("dex", Type::optional(Type::enum_ref("dex"))),
            ],
            vec![
                Field::new("transaction_hash", Type::BlockchainTransactionHash),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("dex", Type::enum_ref("dex")),
                Field::new("token_in_id", Type::BigInt),
                Field::new("token_out_id", Type::BigInt),
                Field::new("amount_in", Type::BlockchainDecimal),
                Field::new("amount_out", Type::BlockchainDecimal),
                Field::new("happened_at", Type::BigInt),
            ],
            r#"
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
"#,
        ),
        ProceduralFunction::new(
            "fun_watcher_upsert_last_dex_trade_for_pair",
            vec![
                Field::new("transaction_hash", Type::BlockchainTransactionHash),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("dex", Type::enum_ref("dex")),
                Field::new("token_in_address", Type::BlockchainAddress),
                Field::new("token_out_address", Type::BlockchainAddress),
                Field::new("amount_in", Type::BlockchainDecimal),
                Field::new("amount_out", Type::BlockchainDecimal),
            ],
            vec![Field::new("last_dex_trade_for_pair_id", Type::BigInt)],
            r#"
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
"#,
        ),
        ProceduralFunction::new(
            "fun_watcher_upsert_strategy_pool_contract_asset_balance",
            vec![
                Field::new("strategy_pool_contract_id", Type::BigInt),
                Field::new("token_address", Type::BlockchainAddress),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                // TODO: old_balance
                Field::new("new_balance", Type::BlockchainDecimal),
            ],
            vec![Field::new(
                "strategy_contract_asset_balance_id",
                Type::BigInt,
            )],
            r#"
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
"#,
        ),
        ProceduralFunction::new(
            "fun_watcher_list_strategy_pool_contract_asset_balances",
            vec![
                Field::new("strategy_pool_contract_id", Type::optional(Type::BigInt)),
                Field::new("strategy_id", Type::optional(Type::BigInt)),
                Field::new("blockchain", Type::optional(Type::enum_ref("block_chain"))),
                Field::new("token_address", Type::optional(Type::BlockchainAddress)),
            ],
            vec![
                Field::new("token_id", Type::BigInt),
                Field::new("token_name", Type::String),
                Field::new("token_symbol", Type::String),
                Field::new("token_address", Type::BlockchainAddress),
                Field::new("token_decimals", Type::Int),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("balance", Type::BlockchainDecimal),
            ],
            r#"
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
			AND (a_token_address ISNULL OR tc.address = a_token_address);
END
"#,
        ),
        ProceduralFunction::new(
            "fun_watcher_list_strategy_escrow_pending_wallet_balance",
            vec![
                Field::new("strategy_id", Type::optional(Type::BigInt)),
                Field::new("token_address", Type::optional(Type::BlockchainAddress)),
            ],
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("address", Type::BlockchainAddress),
                Field::new("token_id", Type::BigInt),
                Field::new("token_address", Type::BlockchainAddress),
                Field::new("token_name", Type::String),
                Field::new("token_symbol", Type::String),
                Field::new("balance", Type::BlockchainDecimal),
            ],
            r#"
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
        "#,
        ),
        ProceduralFunction::new(
            "fun_watcher_list_user_strategy_balance",
            vec![
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
                Field::new("strategy_id", Type::optional(Type::BigInt)),
                Field::new("user_id", Type::optional(Type::BigInt)),
                Field::new("blockchain", Type::optional(Type::enum_ref("block_chain"))),
            ],
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("user_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("strategy_pool_contract_address", Type::BlockchainAddress),
                Field::new("user_strategy_wallet_address", Type::BlockchainAddress),
                Field::new("balance", Type::BlockchainDecimal),
            ],
            r#"
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
        "#,
        ),
        ProceduralFunction::new(
            "fun_watcher_upsert_expert_listened_wallet_asset_balance",
            vec![
                Field::new("address", Type::BlockchainAddress),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("token_id", Type::BigInt),
                Field::new("old_balance", Type::BlockchainDecimal),
                Field::new("new_balance", Type::BlockchainDecimal),
            ],
            vec![Field::new(
                "expert_listened_wallet_asset_balance_id",
                Type::BigInt,
            )],
            r#"
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
        "#,
        ),
        ProceduralFunction::new(
            "fun_watcher_list_expert_listened_wallet_asset_balance",
            vec![
                Field::new("limit", Type::optional(Type::BigInt)),
                Field::new("offset", Type::optional(Type::BigInt)),
                Field::new("strategy_id", Type::optional(Type::BigInt)),
                Field::new("address", Type::optional(Type::BlockchainAddress)),
                Field::new("blockchain", Type::optional(Type::enum_ref("block_chain"))),
                Field::new("token_id", Type::optional(Type::BigInt)),
            ],
            vec![
                Field::new("pkey_id", Type::BigInt),
                Field::new("address", Type::BlockchainAddress),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("token_id", Type::BigInt),
                Field::new("token_address", Type::BlockchainAddress),
                Field::new("token_symbol", Type::String),
                Field::new("token_decimals", Type::Int),
                Field::new("balance", Type::BlockchainDecimal),
            ],
            r#"
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
     ORDER BY elwab.pkey_id DESC
    LIMIT a_limit
    OFFSET a_offset;
END

        "#,
        ),
        ProceduralFunction::new(
            "fun_watcher_save_strategy_pool_contract",
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("address", Type::BlockchainAddress),
            ],
            vec![Field::new("pkey_id", Type::BigInt)],
            r#"
BEGIN
    RETURN QUERY INSERT INTO tbl.strategy_pool_contract AS spc (fkey_strategy_id, blockchain, address, created_at)
    VALUES (a_strategy_id, a_blockchain, a_address, EXTRACT(EPOCH FROM NOW()))
    RETURNING spc.pkey_id;
END
"#,
        ),
        ProceduralFunction::new(
            "fun_watcher_upsert_user_strategy_balance",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("strategy_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("old_balance", Type::BlockchainDecimal),
                Field::new("new_balance", Type::BlockchainDecimal),
            ],
            vec![Field::new("ret_pkey_id", Type::BigInt)],
            r#"
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
            "#,
        ),
        ProceduralFunction::new(
            "fun_watcher_upsert_user_deposit_withdraw_balance",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("token_address", Type::BlockchainAddress),
                Field::new("escrow_contract_address", Type::BlockchainAddress),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("old_balance", Type::BlockchainDecimal),
                Field::new("new_balance", Type::BlockchainDecimal),
            ],
            vec![Field::new("ret_pkey_id", Type::BigInt)],
            r#"
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
      AND elwal.fkey_escrow_contract_address_id = _escrow_contract_address_id;

    -- insert new entry if not exist
    IF _user_deposit_withdraw_balance_id ISNULL THEN
        INSERT INTO tbl.user_deposit_withdraw_balance (fkey_user_id, fkey_escrow_contract_address_id, fkey_token_id, balance)
        VALUES (a_user_id, _escrow_contract_address_id, _token_id, a_new_balance)
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
            "#,
        ),
        ProceduralFunction::new(
            "fun_watcher_list_strategy_pool_contract",
            vec![
                Field::new("limit", Type::BigInt),
                Field::new("offset", Type::BigInt),
                Field::new("strategy_id", Type::optional(Type::BigInt)),
                Field::new("blockchain", Type::optional(Type::enum_ref("block_chain"))),
                Field::new("address", Type::optional(Type::BlockchainAddress)),
            ],
            vec![
                Field::new("pkey_id", Type::BigInt),
                Field::new("strategy_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("address", Type::BlockchainAddress),
                Field::new("created_at", Type::BigInt),
            ],
            r#"
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
        "#,
        ),
        ProceduralFunction::new(
            "fun_user_list_escrow_contract_address_req",
            vec![Field::new(
                "blockchain",
                Type::optional(Type::enum_ref("block_chain")),
            )],
            vec![
                Field::new("pkey_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("address", Type::BlockchainAddress),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT
        eca.pkey_id,
        eca.blockchain,
        eca.address
    FROM tbl.escrow_contract_address AS eca
    WHERE (a_blockchain ISNULL OR eca.blockchain = a_blockchain)
    ORDER BY eca.pkey_id;
END
        "#,
        ),
    ]
}
