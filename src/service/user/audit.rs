pub struct AuditData {
    pub id: i32,
    pub name: &'static str,
    pub description: &'static str,
}

pub const AUDIT_TOP25_TOKENS: AuditData = AuditData {
    id: 1,
    name: "TOP 25 TOKENS",
    description: "top 25 TOKENS from CoinMarketCap",
};

pub const AUDIT_IMMUTABLE_TOKENS: AuditData = AuditData {
    id: 2,
    name: "IMMUTABLE TOKENS",
    description: "Watched wallet addresses and token ratio cannot be changed after creation",
};

pub const AUDIT_TOKENS_NO_MORE_THAN_10_PERCENT: AuditData = AuditData {
    id: 3,
    name: "10% TOKENS",
    description: "No asset allowed should be more than 10% of the total portfolio",
};

pub fn get_audit_rules() -> &'static [AuditData] {
    &[
        AUDIT_TOP25_TOKENS,
        AUDIT_IMMUTABLE_TOKENS,
        AUDIT_TOKENS_NO_MORE_THAN_10_PERCENT,
    ]
}
