use eyre::*;
use gen::database::*;
use gen::model::*;
use lib::handler::RequestHandler;
use lib::toolbox::*;
use lib::ws::*;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tracing::{debug, info};
use uuid::Uuid;
use web3::signing::{keccak256, recover, RecoveryError};
use web3::types::Address;

pub struct SignupHandler;

impl RequestHandler for SignupHandler {
    type Request = SignupRequest;
    type Response = SignupResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: Self::Request,
    ) {
        let db: DbClient = toolbox.get_db();
        let db_auth: DbClient = toolbox.get_nth_db(1);
        toolbox.spawn_response(ctx, async move {
            let public_id = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as i64;
            let salt = Uuid::new_v4();
            let password_hash = hash_password(&req.password, salt.as_bytes())?;
            let address = &req.address;

            let agreed_tos = req.agreed_tos;
            let agreed_privacy = req.agreed_privacy;

            if !agreed_tos {
                bail!(CustomError::new(
                    EnumErrorCode::UserMustAgreeTos,
                    Value::Null
                ));
            }
            if !agreed_privacy {
                bail!(CustomError::new(
                    EnumErrorCode::UserMustAgreePrivacyPolicy,
                    Value::Null
                ));
            }

            db_auth
                .fun_auth_signup(FunAuthSignupReq {
                    address: address.to_string(),
                    email: req.email.clone(),
                    phone: req.phone.clone(),
                    password_hash,
                    password_salt: salt.as_bytes().to_vec(),
                    age: 0,
                    preferred_language: "".to_string(),
                    agreed_tos,
                    agreed_privacy,
                    ip_address: conn.address.ip(),
                })
                .await?;
            if db_auth.client.conn_hash() != db.client.conn_hash() {
                db.fun_auth_signup(FunAuthSignupReq {
                    address: address.to_string(),
                    email: req.email,
                    phone: req.phone,
                    password_hash: vec![],
                    password_salt: salt.as_bytes().to_vec(),
                    age: 0,
                    preferred_language: "".to_string(),
                    agreed_tos,
                    agreed_privacy,
                    ip_address: conn.address.ip(),
                })
                .await?;
            }
            Ok(SignupResponse {
                address: address.to_string(),
                user_public_id: public_id,
            })
        });
    }
}

pub struct LoginHandler;

impl RequestHandler for LoginHandler {
    type Request = LoginRequest;
    type Response = LoginResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: Self::Request,
    ) {
        let db_auth: DbClient = toolbox.get_nth_db(1);
        toolbox.spawn_response(ctx, async move {
            let address = req.address;
            let to_sign_text = req.signature_text;
            let to_sign_text_hash = keccak256(to_sign_text.as_bytes());
            let service_code = req.service_code;
            ensure!(
                req.signature.starts_with("0x"),
                CustomError::new(
                    EnumErrorCode::InvalidPassword,
                    "Signature should start with 0x"
                )
            );
            let signature = hex::decode(req.signature)?;

            let recovered = recover(&to_sign_text_hash, &signature, 27)?;
            debug!(
                "Login address: {}, to sign text {}, signature address: {}",
                address, to_sign_text, recovered
            );
            ensure!(
                format!("{:?}", recovered) == address,
                CustomError::new(EnumErrorCode::InvalidPassword, "Signature is not valid")
            );
            let data = db_auth
                .fun_auth_authenticate(FunAuthAuthenticateReq {
                    address: address.clone(),
                    password_hash: vec![],
                    service_code: service_code as _,
                    device_id: req.device_id,
                    device_os: req.device_os,
                    ip_address: conn.address.ip(),
                })
                .await?;
            let row =
                data.rows.into_iter().next().with_context(|| {
                    CustomError::new(EnumErrorCode::UserNoAuthToken, Value::Null)
                })?;
            let user_token = Uuid::new_v4();
            let admin_token = Uuid::new_v4();
            db_auth
                .fun_auth_set_token(FunAuthSetTokenReq {
                    user_id: row.user_id,
                    user_token,
                    admin_token,
                    service_code: service_code as _,
                })
                .await?;
            Ok(LoginResponse {
                address: address.clone(),
                user_public_id: row.user_public_id,
                user_token,
                admin_token,
            })
        })
    }
}
pub fn hash_password(password: &str, salt: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(salt.as_ref());
    Ok(hasher.finalize().to_vec())
}

pub struct AuthorizeHandler {
    pub accept_service: EnumService,
}
impl RequestHandler for AuthorizeHandler {
    type Request = AuthorizeRequest;
    type Response = AuthorizeResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: Self::Request,
    ) {
        let db_auth: DbClient = toolbox.get_nth_db(1);
        let accept_srv = self.accept_service;
        toolbox.spawn_response(ctx, async move {
            let srv = req.service_code;

            if srv != accept_srv {
                bail!(CustomError::new(
                    EnumErrorCode::InvalidService,
                    format!(
                        "Invalid service, only {:?} {} permitted",
                        accept_srv, accept_srv as u32
                    ),
                ));
            }
            let auth_data = db_auth
                .fun_auth_authorize(FunAuthAuthorizeReq {
                    address: req.address.to_string(),
                    token: req.token,
                    service: srv,
                    device_id: req.device_id,
                    device_os: req.device_os,
                    ip_address: conn.address.ip(),
                })
                .await?;

            let auth_data = auth_data.rows.into_iter().next().with_context(|| {
                CustomError::new(EnumErrorCode::UserInvalidAuthToken, Value::Null)
            })?;

            conn.user_id
                .store(auth_data.user_id as _, Ordering::Relaxed);
            conn.role.store(auth_data.role as _, Ordering::Relaxed);
            Ok(AuthorizeResponse { success: true })
        })
    }
}
pub fn hash_eth_message(message: &[u8]) -> [u8; 32] {
    let mut data = vec![];
    data.extend(b"\x19Ethereum Signed Message:\n");
    data.extend(format!("{}", message.len()).as_bytes());
    data.extend(message);
    keccak256(&data)
}
fn verify_message_address(
    message: &[u8],
    signature: &[u8],
    expected_address: Address,
) -> Result<bool, RecoveryError> {
    if signature.len() != 65 {
        return Err(RecoveryError::InvalidSignature);
    }
    if signature[64] as i32 != 27 && signature[64] as i32 != 28 {
        // only supports 27/28 recovery id for ethereum
        return Err(RecoveryError::InvalidSignature);
    }
    let message_hash = hash_eth_message(message);
    let recovery_id = signature[64] as i32 - 27;
    info!("Recovery id: {}", recovery_id);
    let addr = recover(&message_hash, &signature[..64], recovery_id)?;
    info!(
        "Expected address: {:?}, Recovered address: {:?}",
        expected_address, addr
    );
    return Ok(addr == expected_address);
}
#[cfg(test)]
mod tests {
    use super::*;
    use lib::log::{setup_logs, LogLevel};
    use std::str::FromStr;

    #[test]
    fn test_personal_sign_recover() -> Result<()> {
        let _ = setup_logs(LogLevel::Trace);
        let address = Address::from_str("0x63f9a92d8d61b48a9fff8d58080425a3012d05c8")?;
        let message = b"0x63f9a92d8d61b48a9fff8d58080425a3012d05c8igwyk4r1o7o";
        let signature = hex::decode("382a3e04daf88f322730f6a2972475fc5646ea8c4a7f3b5e83a90b10ba08a7364cd2f55348f2b6d210fbed7fc485abf19ecb2f3967e410d6349dd7dd1d4487751b")?;
        assert!(verify_message_address(message, &signature, address)?);
        Ok(())
    }
    #[test]
    fn test_personal_sign_recover_real_data() -> Result<()> {
        let _ = setup_logs(LogLevel::Trace);
        let address = Address::from_str("0x111013b7862ebc1b9726420aa0e8728de310ee63")?;
        let message = hex::decode("005400680069007300200072006500710075006500730074002000770069006c006c0020006e006f00740020007400720069006700670065007200200061006e00790020007400720061006e00730061006300740069006f006e0020006f007200200069006e00630075007200200061006e007900200063006f007300740020006f007200200066006500650073002e0020000a0020004900740020006900730020006f006e006c007900200069006e00740065006e00640065006400200074006f002000610075007400680065006e00740069006300610074006500200079006f0075002000610072006500200074006800650020006f0077006e006500720020006f0066002000770061006c006c00650074003a000a003000780031003100310030003100330062003700380036003200650062006300310062003900370032003600340032003000610061003000650038003700320038006400650033003100300065006500360033006e006f006e00630065003a000a00310032003800370037003000380033")?;
        let signature = hex::decode("aef85890188d4cdc88b329c6aeeae24ab917600c23e85e196717a22153ca352b1f81cd3378c4a53d44ec8fcfe320a505e93cb08fe0635bc0ed5bf0b771502d921c")?;
        assert!(verify_message_address(&message, &signature, address)?);
        Ok(())
    }
}
