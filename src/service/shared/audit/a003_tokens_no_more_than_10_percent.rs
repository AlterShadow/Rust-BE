use crate::audit::AuditRule;

pub const AUDIT_TOKENS_NO_MORE_THAN_10_PERCENT: AuditRule = AuditRule {
    id: 3,
    name: "10% TOKENS",
    description: "No asset allowed should be more than 10% of the total portfolio",
};
