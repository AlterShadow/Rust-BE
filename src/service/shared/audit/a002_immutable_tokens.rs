use crate::audit::{AuditLogger, AuditRule};
use eyre::*;
use gen::database::FunUserListStrategyAuditRulesReq;
use gen::model::EnumErrorCode;
use lib::database::DbClient;
use lib::toolbox::CustomError;

pub const AUDIT_IMMUTABLE_TOKENS: AuditRule = AuditRule {
    id: 2,
    name: "IMMUTABLE TOKENS",
    description: "Watched wallet addresses and token ratio cannot be changed after creation",
};

pub async fn validate_audit_rule_immutable_tokens(
    logger: &AuditLogger,
    db: &DbClient,
    strategy_id: i64,
) -> Result<()> {
    logger.log(
        AUDIT_IMMUTABLE_TOKENS,
        &format!("auditing strategy_id={strategy_id}"),
    )?;
    let immutable = db
        .execute(FunUserListStrategyAuditRulesReq {
            strategy_id,
            audit_rule_id: Some(AUDIT_IMMUTABLE_TOKENS.id),
        })
        .await?
        .into_result()
        .is_some();
    if immutable {
        logger.log(
            AUDIT_IMMUTABLE_TOKENS,
            &format!("audit FAILED strategy_id={strategy_id}"),
        )?;
        bail!(CustomError::new(
            EnumErrorCode::ImmutableStrategy,
            "Strategy is immutable"
        ));
    }
    logger.log(
        AUDIT_IMMUTABLE_TOKENS,
        &format!("audit FAILED strategy_id={strategy_id}"),
    )?;
    Ok(())
}
