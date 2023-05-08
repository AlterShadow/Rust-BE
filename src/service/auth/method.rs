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
use uuid::Uuid;

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
            let username = req.username.trim().to_ascii_lowercase();

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
                    public_id,
                    username: username.to_string(),
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
                    public_id,
                    username: username.to_string(),
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
                username: username.to_string(),
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
            let username = req.username.trim().to_ascii_lowercase();
            let service_code = req.service_code;
            let password = req.password;
            let data = db_auth
                .fun_auth_get_password_salt(FunAuthGetPasswordSaltReq {
                    username: username.clone(),
                })
                .await?;
            let salt = data
                .rows
                .into_iter()
                .next()
                .with_context(|| {
                    CustomError::new(EnumErrorCode::UserNoValidSalt, "No salt found for user")
                })?
                .salt
                .clone();

            let mut hasher = Sha256::new();

            hasher.update(password.as_bytes());
            hasher.update(salt.as_slice());
            let password_hash = hasher.finalize().to_vec();
            let data = db_auth
                .fun_auth_authenticate(FunAuthAuthenticateReq {
                    username: username.clone(),
                    password_hash,
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
                username: username.clone(),
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
                    username: req.username.to_string(),
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
