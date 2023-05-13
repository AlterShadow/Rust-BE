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
