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
            "fun_watcher_save_wallet_activity_history",
            vec![
                Field::new("address", Type::String),
                Field::new("transaction_hash", Type::String),
                Field::new("blockchain", Type::enum_ref("block_chain")),
                Field::new("contract_address", Type::String),
                Field::new("caller_address", Type::String),
                Field::new("dex", Type::optional(Type::String)),
                Field::new("token_in_address", Type::optional(Type::String)),
                Field::new("token_out_address", Type::optional(Type::String)),
                Field::new("amount_in", Type::optional(Type::String)),
                Field::new("amount_out", Type::optional(Type::String)),
                Field::new("swap_calls", Type::optional(Type::Object)),
                Field::new("paths", Type::optional(Type::Object)),
                Field::new("dex_versions", Type::optional(Type::Object)),
                Field::new("created_at", Type::optional(Type::BigInt)),
            ],
            vec![Field::new("wallet_activity_history_id", Type::BigInt)],
            r#"
BEGIN
    RETURN QUERY INSERT INTO tbl.wallet_activity_history(
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
    )
    VALUES (
        a_address,
        a_transaction_hash,
        a_blockchain,
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
        "#,
        ),
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
        "#,
        ),
    ]
}
