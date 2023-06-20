use crate::audit::{AuditLogger, AuditRule};
use api::cmc::CoinMarketCap;
use eyre::*;
use gen::database::FunUserCheckIfTokenWhitelistedReq;
use gen::model::EnumErrorCode;
use lib::database::DbClient;
use lib::toolbox::CustomError;
use std::collections::HashSet;

pub const AUDIT_TOP25_TOKENS: AuditRule = AuditRule {
    id: 1,
    name: "TOP 25 TOKENS",
    description: "top 25 TOKENS from CoinMarketCap",
};
pub async fn validate_audit_rule_top25_tokens(
    cmc: &CoinMarketCap,
    cache: &mut HashSet<String>,
    token: &str,
) -> Result<()> {
    if cache.is_empty() {
        let tokens = cmc.get_top_25_coins().await?;
        for t in tokens.data {
            cache.insert(t.symbol);
        }
    }
    ensure!(
        cache.contains(token),
        CustomError::new(EnumErrorCode::TokenNotTop25, "Token is not in top 25")
    );
    Ok(())
}

pub async fn validate_audit_rule_token_whitelisted(
    logger: &AuditLogger,
    db: &DbClient,
    strategy_id: i64,
    token: &str,
) -> Result<()> {
    logger.log(
        AUDIT_TOP25_TOKENS,
        &format!("auditing strategy_id={strategy_id} token={token}"),
    )?;
    let ret = db
        .execute(FunUserCheckIfTokenWhitelistedReq {
            strategy_id,
            token_name: token.to_string(),
        })
        .await?
        .into_result()
        .context("No result")?;
    if !ret.whitelisted {
        logger.log(
            AUDIT_TOP25_TOKENS,
            &format!("audit FAILED strategy_id={strategy_id} token={token}"),
        )?;
        bail!(CustomError::new(
            EnumErrorCode::TokenNotTop25,
            "Token is not in top 25"
        ));
    }
    logger.log(
        AUDIT_TOP25_TOKENS,
        &format!("audit FAILED strategy_id={strategy_id} token={token}"),
    )?;
    Ok(())
}
