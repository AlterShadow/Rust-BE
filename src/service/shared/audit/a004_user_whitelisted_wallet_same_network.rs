use crate::audit::{AuditLogger, AuditRule};
use eyre::*;
use gen::database::{FunUserListRegisteredWalletsReq, FunUserListStrategiesReq};
use gen::model::EnumErrorCode;
use lib::database::DbClient;
use lib::toolbox::CustomError;
use web3::types::Address;

pub const AUDIT_USER_WHITELISTED_WALLET_SAME_NETWORK: AuditRule = AuditRule {
    id: 4,
    name: "User wallet same network",
    description: "User whitelisted wallet should be in the same network as the strategy contract",
};
pub async fn validate_audit_rule_user_whitelisted_wallet_same_network(
    logger: &AuditLogger,
    db: &DbClient,
    user_whitelisted_wallet_address: Address,
    strategy_id: i64,
) -> Result<()> {
    logger.log(
        AUDIT_USER_WHITELISTED_WALLET_SAME_NETWORK,
        &format!(
            "auditing strategy_id={strategy_id} user_wallet={user_whitelisted_wallet_address:?}"
        ),
    )?;
    let strategy = db
        .execute(FunUserListStrategiesReq {
            user_id: 0,
            limit: 1,
            strategy_id: Some(strategy_id),
            strategy_name: None,
            expert_public_id: None,
            expert_name: None,
            description: None,
            blockchain: None,
            offset: 0,
            wallet_address: None,
        })
        .await?
        .into_result()
        .with_context(|| CustomError::new(EnumErrorCode::NotFound, "Strategy not found"))?;
    let wallet = db
        .execute(FunUserListRegisteredWalletsReq {
            limit: 1,
            offset: 0,
            user_id: None,
            blockchain: None,
            address: Some(user_whitelisted_wallet_address.into()),
        })
        .await?
        .into_result()
        .with_context(|| CustomError::new(EnumErrorCode::NotFound, "Wallet not found"))?;
    if strategy.blockchain != wallet.blockchain {
        logger.log(
            AUDIT_USER_WHITELISTED_WALLET_SAME_NETWORK,
            &format!("audit FAILED strategy_id={strategy_id} user_wallet={user_whitelisted_wallet_address:?}"),
        )?;
        bail!(CustomError::new(
            EnumErrorCode::UserWhitelistedWalletNotSameNetworkAsStrategy,
            "User whitelisted wallet not in the same network as the strategy contract"
        ));
    }
    logger.log(
        AUDIT_USER_WHITELISTED_WALLET_SAME_NETWORK,
        &format!(
            "audit FAILED strategy_id={strategy_id} user_wallet={user_whitelisted_wallet_address:?}"
        ),
    )?;
    Ok(())
}
