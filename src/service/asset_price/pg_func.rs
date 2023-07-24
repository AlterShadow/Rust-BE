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
        ),
        ProceduralFunction::new(
            "fun_asset_price_list_asset_prices",
            vec![
                Field::new("symbols", Type::optional(Type::Vec(Box::new(Type::String)))),
                Field::new("limit", Type::optional(Type::Int)),
                Field::new("offset", Type::optional(Type::Int)),
            ],
            vec![
                Field::new("symbol", Type::String),
                Field::new("price_latest", Type::Numeric),
                Field::new("price_7d", Type::optional(Type::Numeric)),
                Field::new("price_30d", Type::optional(Type::Numeric)),
            ],
            r#"
BEGIN
	RETURN QUERY
	WITH date_ranges AS (
		SELECT
				tp.symbol,
				MAX(tp.created_at) AS latest,
				MAX(tp.created_at) FILTER (WHERE tp.created_at <= (EXTRACT(EPOCH FROM (NOW() - INTERVAL '7 DAYS'))::BIGINT)) AS day_7,
				MAX(tp.created_at) FILTER (WHERE tp.created_at <= (EXTRACT(EPOCH FROM (NOW() - INTERVAL '30 DAYS'))::BIGINT)) AS day_30
		FROM tbl.token_price AS tp
		WHERE (a_symbols IS NULL OR tp.symbol = ANY(a_symbols))
		GROUP BY tp.symbol
	)

	SELECT
		dr.symbol,
		tp_latest.price AS price_latest,
		tp_7d.price AS price_7d,
		tp_30d.price AS price_30d
	FROM date_ranges dr
	LEFT JOIN tbl.token_price AS tp_latest ON tp_latest.symbol = dr.symbol AND tp_latest.created_at = dr.latest
	LEFT JOIN tbl.token_price AS tp_7d ON tp_7d.symbol = dr.symbol AND tp_7d.created_at = dr.day_7
	LEFT JOIN tbl.token_price AS tp_30d ON tp_30d.symbol = dr.symbol AND tp_30d.created_at = dr.day_30
	ORDER BY dr.symbol
	LIMIT a_limit
	OFFSET a_offset;
END
"#,
        ),
    ]
}
