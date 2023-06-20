use crate::audit::AuditData;

pub const AUDIT_TOKENS_NO_MORE_THAN_10_PERCENT: AuditData = AuditData {
    id: 3,
    name: "10% TOKENS",
    description: "No asset allowed should be more than 10% of the total portfolio",
};
