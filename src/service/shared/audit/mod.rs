mod a001_top25;
pub use a001_top25::*;
mod a002_immutable_tokens;
pub use a002_immutable_tokens::*;
mod a003_tokens_no_more_than_10_percent;
pub use a003_tokens_no_more_than_10_percent::*;

pub struct AuditData {
    pub id: i32,
    pub name: &'static str,
    pub description: &'static str,
}

pub fn get_audit_rules() -> &'static [AuditData] {
    &[
        AUDIT_TOP25_TOKENS,
        AUDIT_IMMUTABLE_TOKENS,
        AUDIT_TOKENS_NO_MORE_THAN_10_PERCENT,
    ]
}
