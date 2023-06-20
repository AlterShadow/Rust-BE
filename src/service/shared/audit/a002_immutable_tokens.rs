use crate::audit::AuditData;
use eyre::*;

pub const AUDIT_IMMUTABLE_TOKENS: AuditData = AuditData {
    id: 2,
    name: "IMMUTABLE TOKENS",
    description: "Watched wallet addresses and token ratio cannot be changed after creation",
};

pub fn validate_audit_rule_immutable_tokens() -> Result<()> {
    todo!()
}
