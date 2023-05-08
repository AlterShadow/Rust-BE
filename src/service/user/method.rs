use ::tap::*;
use eyre::*;
use gen::database::*;
use gen::model::*;

use lib::toolbox::*;

use lib::ws::*;
use serde_json::Value;
use std::sync::atomic::Ordering;

pub fn ensure_user_role(conn: &Connection, role: EnumRole) -> Result<()> {
    let user_role = conn.role.load(Ordering::Relaxed);

    ensure!(
        user_role >= (role as u32),
        CustomError::new(EnumErrorCode::InvalidRole, ErrorInvalidRole {})
    );
    Ok(())
}

pub async fn get_user_role_in_organization(
    db: &DbClient,
    user_id: i64,
    organization_id: i64,
) -> Result<EnumRole> {
    let result = db
        .fun_user_get_organization_membership(FunUserGetOrganizationMembershipReq {
            user_id,
            organization_id,
        })
        .await?;
    result
        .rows
        .into_iter()
        .map(|x| x.role)
        .next()
        .unwrap_or(EnumRole::Guest)
        .pipe(Ok)
}

pub async fn ensure_user_role_in_organization(
    db: &DbClient,
    user_id: i64,
    organization_id: i64,
    role: EnumRole,
) -> Result<EnumRole> {
    let current_role = get_user_role_in_organization(&db, user_id, organization_id).await?;
    ensure!(
        (current_role as u32) >= (role as u32),
        CustomError::new(
            EnumErrorCode::OrganizationForbidden,
            ErrorOrganizationForbidden {
                user: user_id.to_string(),
                organization: organization_id.to_string(),
            }
        )
    );
    Ok(current_role)
}

pub async fn verify_organization_status(db: &DbClient, organization_id: i64) -> Result<()> {
    let organization = db
        .fun_admin_get_organization(FunAdminGetOrganizationReq { organization_id })
        .await?;
    ensure!(
        organization.rows.len() > 0,
        CustomError::new(EnumErrorCode::OrganizationNotFound, Value::Null)
    );
    ensure!(
        organization.rows[0].approved,
        CustomError::new(EnumErrorCode::OrganizationForbidden, Value::Null)
    );
    Ok(())
}
