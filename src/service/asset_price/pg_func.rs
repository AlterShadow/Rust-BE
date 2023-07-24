use model::pg_func::ProceduralFunction;
use model::types::*;

pub fn get_asset_price_pg_func() -> Vec<ProceduralFunction> {
    vec![ProceduralFunction::new(
        "fun_asset_price_insert_asset_prices",
        vec![
            Field::new("symbols", Type::Vec(Box::new(Type::String))),
            Field::new("prices", Type::Vec(Box::new(Type::Numeric))),
        ],
        vec![Field::new("success", Type::Boolean)],
        r#"
BEGIN
	INSERT INTO tbl.token_price (symbol, price, created_at)
	SELECT a_symbol, a_price, EXTRACT(EPOCH FROM NOW())::bigint
	FROM UNNEST(a_symbols, a_prices) AS u(a_symbol, a_price);

	RETURN QUERY SELECT true AS "success";
END
"#,
    )]
}
