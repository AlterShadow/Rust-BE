use crate::audit::AuditData;
use eyre::*;
pub const AUDIT_TOP25_TOKENS: AuditData = AuditData {
    id: 1,
    name: "TOP 25 TOKENS",
    description: "top 25 TOKENS from CoinMarketCap",
};
pub fn validate_audit_rule_top25_tokens() -> Result<()> {
    todo!()
}
