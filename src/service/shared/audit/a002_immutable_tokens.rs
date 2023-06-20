use crate::audit::AuditRule;
use eyre::*;
use gen::database::{FunUserGetStrategyReq, FunUserStrategyRowType};
use gen::model::EnumErrorCode;
use lib::database::DbClient;
use lib::toolbox::CustomError;

pub const AUDIT_IMMUTABLE_TOKENS: AuditRule = AuditRule {
    id: 2,
    name: "IMMUTABLE TOKENS",
    description: "Watched wallet addresses and token ratio cannot be changed after creation",
};

pub async fn validate_audit_rule_immutable_tokens(
    db: &DbClient,
    strategy_id: i64,
    user_id: i64,
) -> Result<FunUserStrategyRowType> {
    let strategy = db
        .execute(FunUserGetStrategyReq {
            strategy_id,
            user_id,
        })
        .await?
        .into_result()
        .context(CustomError::new(
            EnumErrorCode::NotFound,
            "Could not find strategy",
        ))?;
    ensure!(
        !strategy.immutable,
        CustomError::new(EnumErrorCode::ImmutableStrategy, "Strategy is immutable")
    );
    Ok(strategy)
}
