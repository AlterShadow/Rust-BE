use model::pg_func::ProceduralFunction;
use model::types::*;

pub fn get_trade_watcher_pg_func() -> Vec<ProceduralFunction> {
    vec![
        ProceduralFunction::new(
            "fun_watcher_save_raw_transaction",
            vec![
                Field::new("transaction_hash", Type::String),
                Field::new("chain", Type::String),
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
                         a_chain,
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
                Field::new("transaction_hash", Type::String),
                Field::new("chain", Type::String),
                Field::new("dex", Type::optional(Type::String)),
            ],
            vec![
                Field::new("transaction_cache_id", Type::BigInt),
                Field::new("transaction_hash", Type::String),
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
            "fun_watcher_save_strategy_watching_wallet_trade_history",
            vec![
                Field::new("address", Type::String),
                Field::new("transaction_hash", Type::String),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("contract_address", Type::String),
                Field::new("dex", Type::optional(Type::String)),
                Field::new("token_in_address", Type::optional(Type::String)),
                Field::new("token_out_address", Type::optional(Type::String)),
                Field::new("amount_in", Type::optional(Type::String)),
                Field::new("amount_out", Type::optional(Type::String)),
                Field::new("happened_at", Type::optional(Type::BigInt)),
            ],
            vec![
                Field::new(
                    "strategy_watching_wallet_trade_history_id",
                    Type::optional(Type::BigInt),
                ),
                Field::new("expert_watched_wallet_id", Type::optional(Type::BigInt)),
                Field::new("fkey_token_in", Type::optional(Type::BigInt)),
                Field::new("fkey_token_in_name", Type::optional(Type::String)),
                Field::new("fkey_token_out", Type::optional(Type::BigInt)),
                Field::new("fkey_token_out_name", Type::optional(Type::String)),
            ],
            r#"
DECLARE
    _strategy_watching_wallet_trade_history_id bigint;
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
    SELECT pkey_id
    INTO _fkey_token_in
    FROM tbl.escrow_token_contract_address
    WHERE address = a_token_in_address
      AND blockchain = a_blockchain;
    SELECT pkey_id
    INTO _fkey_token_out
    FROM tbl.escrow_token_contract_address
    WHERE address = a_token_out_address
      AND blockchain = a_blockchain;
    IF _expert_watched_wallet_id ISNULL AND _fkey_token_in ISNULL AND _fkey_token_out ISNULL THEN
        INSERT INTO tbl.strategy_watching_wallet_trade_history
            (
             expert_watched_wallet_id, blockchain,
             transaction_hash, dex, contract_address,
             fkey_token_in, fkey_token_out, amount_in,
             amount_out, heppened_at
                )
        VALUES (_expert_watched_wallet_id, a_blockchain, a_transaction_hash, a_dex, a_contract_address,
                _fkey_token_in, _fkey_token_out, a_amount_in, a_amount_out, a_happened_at)
        RETURNING pkey_id
        INTO _strategy_watching_wallet_trade_history;
        RETURN QUERY SELECT _strategy_watching_wallet_trade_history_id, _expert_watched_wallet_id,
                            _fkey_token_in, _fkey_token_in_name, _fkey_token_out, _fkey_token_out_name;
    END IF;
END
        "#,
        ),
        // depcreated
        ProceduralFunction::new(
            "fun_watcher_list_wallet_activity_history",
            vec![
                Field::new("address", Type::String),
                Field::new("blockchain", Type::enum_ref("block_chain")),
            ],
            vec![
                Field::new("wallet_activity_history_id", Type::BigInt),
                Field::new("address", Type::String),
                Field::new("transaction_hash", Type::String),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("dex", Type::optional(Type::String)),
                Field::new("contract_address", Type::String),
                Field::new("token_in_address", Type::optional(Type::String)),
                Field::new("token_out_address", Type::optional(Type::String)),
                Field::new("caller_address", Type::String),
                Field::new("amount_in", Type::optional(Type::String)),
                Field::new("amount_out", Type::optional(Type::String)),
                Field::new("swap_calls", Type::optional(Type::Object)),
                Field::new("paths", Type::optional(Type::Object)),
                Field::new("dex_versions", Type::optional(Type::Object)),
                Field::new("created_at", Type::optional(Type::BigInt)),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT a.pkey_id,
                      a.address,
                      a.transaction_hash,
                      a.blockchain,
                      a.dex,
                      a.contract_address,
                      a.token_in_address,
                      a.token_out_address,
                      a.caller_address,
                      a.amount_in,
                      a.amount_out,
                      a.swap_calls,
                      a.paths,
                      a.dex_versions,
                      a.created_at
                 FROM tbl.wallet_activity_history AS a
                 WHERE a.address = a_address
                   AND a.blockchain = a_blockchain;
END
        "#,
        ),
        ProceduralFunction::new(
            "fun_watcher_list_strategy_escrow_pending_wallet_ledger",
            vec![Field::new("strategy_id", Type::optional(Type::BigInt))],
            vec![
                Field::new("strategy_id", Type::BigInt),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("address", Type::String),
                Field::new("token_id", Type::BigInt),
                Field::new("token_address", Type::String),
                Field::new("token_name", Type::String),
                Field::new("token_symbol", Type::String),
                Field::new("entry", Type::String),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT w.strategy_id,
                        l.blockchain,
                        w.address,
                        t.pkey_id,
                        t.address,
                        t.name,
                        t.symbol,
                        l.entry
                 FROM tbl.strategy_escrow_pending_wallet_ledger AS l
                 JOIN tbl.strategy_escrow_pending_wallet_address AS w ON l.fkey_strategy_pending_wallet_address_id = w.pkey_id
                 JOIN tbl.escrow_token_contract_address AS t ON l.fkey_token_id = t.pkey_id
                 WHERE strategy_id = a_strategy_id;
END
        "#,
        ),
        ProceduralFunction::new(
            "fun_watcher_list_user_strategy_ledger",
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
                Field::new("strategy_pool_contract_address", Type::String),
                Field::new("user_strategy_wallet_address", Type::String),
                Field::new("entry", Type::String),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT spc.fkey_strategy_id,
                        usw.user_id,
                        spc.blockchain,
                        spc.address,
                        usw.address,
                        usl.entry
                 FROM tbl.user_strategy_ledger AS usl
                 JOIN tbl.user_strategy_wallet AS usw ON usw.pkey_id = usl.fkey_user_strategy_wallet_id
                 JOIN tbl.strategy_pool_contract AS spc ON spc.pkey_id = usl.fkey_strategy_pool_contract_id
                 WHERE (a_strategy_id ISNULL OR spc.fkey_strategy_id = a_strategy_id)
                   AND (a_user_id ISNULL OR usw.user_id = a_user_id)
                   AND (a_blockchain ISNULL OR spc.blockchain = a_blockchain)
                 ORDER BY usl.pkey_id DESC
                 LIMIT a_limit
                 OFFSET a_offset;
END
        "#,
        ),
        ProceduralFunction::new(
            "fun_watcher_upsert_expert_listened_wallet_asset_ledger",
            vec![
                Field::new("address", Type::String),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("token_id", Type::String),
                Field::new("old_entry", Type::String),
                Field::new("new_entry", Type::String),
            ],
            vec![Field::new(
                "expert_listened_wallet_asset_ledger_id",
                Type::BigInt,
            )],
            r#"

DECLARE
    _expert_watched_wallet_id bigint;
    _expert_listened_wallet_asset_ledger_id bigint;
    _expert_listened_wallet_asset_ledger_old_entry bigint;
    _pkey_id bigint;
BEGIN
    SELECT pkey_id INTO _expert_watched_wallet_id
    FROM tbl.expert_watched_wallet
    WHERE address = a_address
      AND blockchain = a_blockchain;
    ASSERT _expert_watched_wallet_id NOTNULL;
    SELECT elwal.pkey_id, elwal.entry INTO _expert_listened_wallet_asset_ledger_id, _expert_listened_wallet_asset_ledger_old_entry
        FROM tbl.expert_listened_wallet_asset_ledger AS elwal
                JOIN tbl.expert_watched_wallet AS eww ON eww.pkey_id = elwal.expert_watched_wallet_pkey_id
        WHERE elwal.fkey_token_id = a_token_id
         AND eww.pkey_id = _expert_watched_wallet_id;
         
    -- insert new entry if not exist
    IF _expert_listened_wallet_asset_ledger_id ISNULL THEN
        INSERT INTO tbl.expert_listened_wallet_asset_ledger (fkey_token_id, entry, expert_watched_wallet_pkey_id)
        VALUES (fkey_token_id, a_new_entry, _expert_listened_wallet_asset_ledger_id)
        RETURNING pkey_id
            INTO _pkey_id;
    END IF;

    -- update old entry if exist and equals to old entry
    IF _expert_listened_wallet_asset_ledger_old_entry != a_old_entry THEN
        RETURN QUERY SELECT _pkey_id;
    END IF;
    UPDATE tbl.expert_listened_wallet_asset_ledger
    SET entry = a_new_entry
    WHERE expert_watched_wallet_pkey_id = _expert_listened_wallet_asset_ledger_id
      AND fkey_token_id = a_token_id
      AND entry = a_old_entry
    RETURNING pkey_id
        INTO _pkey_id;
    RETURN QUERY SELECT _pkey_id;
END

        "#,
        ),
    ]
}
