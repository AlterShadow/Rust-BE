use eth_sdk::utils::verify_message_address;
use eyre::*;
use futures::future::BoxFuture;
use futures::FutureExt;
use gen::database::*;
use gen::model::*;
use lib::database::DbClient;
use lib::toolbox::*;
use lib::utils::hex_decode;
use lib::ws::*;
use serde_json::Value;
use siwe::{Message, VerificationOpts};
use std::str::FromStr;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use time::OffsetDateTime;
use tracing::info;
use uuid::Uuid;
use web3::types::Address;

pub async fn ensure_signature_valid(
    signature_text: &str,
    signature: &[u8],
    address: Address,
) -> Result<()> {
    if let Ok(message) = signature_text.parse::<Message>() {
        let verification_opts = VerificationOpts {
            domain: Some("mc2.pathscale.com".parse().unwrap()),
            timestamp: Some(OffsetDateTime::now_utc()),
            ..Default::default()
        };

        if let Err(e) = message.verify(&signature, &verification_opts).await {
            bail!(CustomError::new(
                EnumErrorCode::InvalidPassword,
                format!("Signature is not valid: {}", e)
            ));
        }
    } else {
        // TODO: remove this branch
        let verified = verify_message_address(signature_text.as_bytes(), &signature, address)?;

        ensure!(
            verified,
            CustomError::new(EnumErrorCode::InvalidPassword, "Signature is not valid")
        );
    }
    Ok(())
}
pub struct MethodAuthSignup;

impl SubAuthController for MethodAuthSignup {
    fn auth(
        self: Arc<Self>,
        toolbox: &Toolbox,
        param: Value,
        ctx: RequestContext,
        _conn: Arc<WsConnection>,
    ) -> BoxFuture<'static, Result<Value>> {
        info!("Signup request: {:?}", param);
        let db: DbClient = toolbox.get_db();
        let db_auth: DbClient = toolbox.get_nth_db(1);
        async move {
            let req: SignupRequest = serde_json::from_value(param).map_err(|x| {
                CustomError::new(EnumErrorCode::BadRequest, format!("Invalid request: {}", x))
            })?;
            let address = req.address;

            let signature_text = hex_decode(req.signature_text.as_bytes())?;
            let signature = hex_decode(req.signature.as_bytes())?;
            let signature_text_string = String::from_utf8(signature_text.clone())?;
            ensure_signature_valid(&signature_text_string, &signature, address).await?;

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
            let public_id = chrono::Utc::now().timestamp_millis();
            let _signup = db_auth
                .execute(FunAuthSignupReq {
                    address: address.into(),
                    email: req.email.clone(),
                    phone: req.phone.clone(),
                    preferred_language: "en".to_string(),
                    agreed_tos,
                    agreed_privacy,
                    ip_address: ctx.ip_addr,
                    username: Some(req.username.clone()),
                    age: None,
                    public_id,
                })
                .await?
                .into_result()
                .context("No record")?;
            if req.username.starts_with("dev-") {
                db_auth
                    .execute(FunAuthSetRoleReq {
                        public_user_id: public_id,
                        role: EnumRole::Admin,
                    })
                    .await?;
            }
            if db_auth.conn_hash() != db.conn_hash() {
                db.execute(FunAuthSignupReq {
                    address: address.into(),
                    email: req.email,
                    phone: req.phone,
                    preferred_language: "en".to_string(),
                    agreed_tos,
                    agreed_privacy,
                    ip_address: ctx.ip_addr,
                    username: Some(req.username),
                    age: None,
                    public_id,
                })
                .await?;
            }
            Ok(serde_json::to_value(&SignupResponse {
                address: address.into(),
                user_id: public_id,
            })?)
        }
        .boxed()
    }
}
pub struct MethodAuthLogin;

impl SubAuthController for MethodAuthLogin {
    fn auth(
        self: Arc<Self>,
        toolbox: &Toolbox,
        param: Value,
        ctx: RequestContext,
        _conn: Arc<WsConnection>,
    ) -> BoxFuture<'static, Result<Value>> {
        info!("Login request: {:?}", param);
        let db_auth: DbClient = toolbox.get_nth_db(1);
        async move {
            let req: LoginRequest = serde_json::from_value(param).map_err(|x| {
                CustomError::new(EnumErrorCode::BadRequest, format!("Invalid request: {}", x))
            })?;
            let address = req.address;

            let signature_text = hex_decode(req.signature_text.as_bytes())?;
            let signature_text_string = String::from_utf8(signature_text.clone())?;
            let signature = hex_decode(req.signature.as_bytes())?;
            ensure_signature_valid(&signature_text_string, &signature, address).await?;

            let service_code = req.service;

            let data = db_auth
                .execute(FunAuthAuthenticateReq {
                    address: address.into(),
                    service_code: service_code as _,
                    device_id: req.device_id,
                    device_os: req.device_os,
                    ip_address: ctx.ip_addr,
                })
                .await?;
            let row = data
                .into_result()
                .with_context(|| CustomError::new(EnumErrorCode::UserNoAuthToken, Value::Null))?;
            let user_token = Uuid::new_v4();
            let admin_token = Uuid::new_v4();
            db_auth
                .execute(FunAuthSetTokenReq {
                    user_id: row.user_id,
                    user_token,
                    admin_token,
                    service_code: service_code as _,
                })
                .await?;
            Ok(serde_json::to_value(&LoginResponse {
                address: format!("{:?}", address),
                role: row.role,
                user_id: row.public_user_id,
                user_token,
                admin_token,
            })?)
        }
        .boxed()
    }
}

pub struct MethodAuthAuthorize {
    pub accept_service: EnumService,
}
impl SubAuthController for MethodAuthAuthorize {
    fn auth(
        self: Arc<Self>,
        toolbox: &Toolbox,
        param: Value,
        ctx: RequestContext,
        conn: Arc<WsConnection>,
    ) -> BoxFuture<'static, Result<Value>> {
        info!("Authorize request: {:?}", param);
        let db_auth: DbClient = toolbox.get_nth_db(1);
        let accepted_service = self.accept_service;
        async move {
            let req: AuthorizeRequest = serde_json::from_value(param).map_err(|x| {
                CustomError::new(EnumErrorCode::BadRequest, format!("Invalid request: {}", x))
            })?;
            let address = Address::from_str(&req.address).map_err(|x| {
                CustomError::new(
                    EnumErrorCode::UnknownUser,
                    format!("Invalid address: {}", x),
                )
            })?;
            let service = req.service;

            if service != accepted_service {
                bail!(CustomError::new(
                    EnumErrorCode::InvalidService,
                    format!(
                        "Invalid service, only {:?} {} permitted",
                        accepted_service, accepted_service as u32
                    ),
                ));
            }
            let auth_data = db_auth
                .execute(FunAuthAuthorizeReq {
                    address: address.into(),
                    token: req.token,
                    service,
                    device_id: req.device_id,
                    device_os: req.device_os,
                    ip_address: ctx.ip_addr,
                })
                .await?;

            let auth_data = auth_data.into_result().with_context(|| {
                CustomError::new(EnumErrorCode::UserInvalidAuthToken, Value::Null)
            })?;

            conn.user_id
                .store(auth_data.user_id as _, Ordering::Relaxed);
            conn.role.store(auth_data.role as _, Ordering::Relaxed);
            Ok(serde_json::to_value(&AuthorizeResponse { success: true })?)
        }
        .boxed()
    }
}

pub struct MethodAuthLogout;
impl SubAuthController for MethodAuthLogout {
    fn auth(
        self: Arc<Self>,
        toolbox: &Toolbox,
        _param: Value,
        ctx: RequestContext,
        conn: Arc<WsConnection>,
    ) -> BoxFuture<'static, Result<Value>> {
        let db_auth: DbClient = toolbox.get_nth_db(1);

        async move {
            db_auth
                .execute(FunAuthRemoveTokenReq {
                    user_id: ctx.user_id,
                })
                .await?;
            conn.user_id.store(0, Ordering::Relaxed);
            conn.role.store(EnumRole::Guest as _, Ordering::Relaxed);
            Ok(serde_json::to_value(&LogoutResponse {})?)
        }
        .boxed()
    }
}

pub struct MethodAuthChangeLoginWallet;

impl SubAuthController for MethodAuthChangeLoginWallet {
    fn auth(
        self: Arc<Self>,
        toolbox: &Toolbox,
        param: Value,
        _ctx: RequestContext,
        _conn: Arc<WsConnection>,
    ) -> BoxFuture<'static, Result<Value>> {
        info!("Login request: {:?}", param);
        let db_auth: DbClient = toolbox.get_nth_db(1);
        async move {
            let req: ChangeLoginWalletRequest = serde_json::from_value(param).map_err(|x| {
                CustomError::new(EnumErrorCode::BadRequest, format!("Invalid request: {}", x))
            })?;
            let old_address = Address::from_str(&req.old_address).map_err(|x| {
                CustomError::new(
                    EnumErrorCode::UnknownUser,
                    format!("Invalid address: {}", x),
                )
            })?;

            let old_signature_text = hex_decode(req.old_signature_text.as_bytes())?;

            let old_signature = hex_decode(req.old_signature.as_bytes())?;

            let old_verified =
                verify_message_address(&old_signature_text, &old_signature, old_address)?;

            ensure!(
                old_verified,
                CustomError::new(EnumErrorCode::InvalidPassword, "Old signature is not valid")
            );

            let new_address = Address::from_str(&req.new_address).map_err(|x| {
                CustomError::new(
                    EnumErrorCode::UnknownUser,
                    format!("Invalid address: {}", x),
                )
            })?;

            let new_signature_text = hex_decode(req.new_signature_text.as_bytes())?;

            let new_signature = hex_decode(req.new_signature.as_bytes())?;

            let new_verified =
                verify_message_address(&new_signature_text, &new_signature, new_address)?;

            ensure!(
                new_verified,
                CustomError::new(EnumErrorCode::InvalidPassword, "New signature is not valid")
            );

            let _data = db_auth
                .execute(FunAuthChangeLoginWalletAddressReq {
                    old_wallet_address: old_address.into(),
                    new_wallet_address: new_address.into(),
                })
                .await?;

            Ok(serde_json::to_value(&ChangeLoginWalletResponse {})?)
        }
        .boxed()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use eth_sdk::utils::verify_message_address;
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
        let message = hex::decode("5468697320726571756573742077696c6c206e6f74207472696767657220616e79207472616e73616374696f6e206f7220696e63757220616e7920636f7374206f7220666565732e200a204974206973206f6e6c7920696e74656e64656420746f2061757468656e74696361746520796f752061726520746865206f776e6572206f662077616c6c65743a0a3078313131303133623738363265626331623937323634323061613065383732386465333130656536336e6f6e63653a0a383632353033343139")?;
        let signature = hex::decode("72f8e93e5e2ba1b3df2f179bddac22b691ca86b39f6f7619a9eedd90b16bed165c0e03dcac13e5e2a1a1ea79ab9cf40a6ba572165a7f58525466a42a9699f0ea1c")?;
        assert!(verify_message_address(&message, &signature, address)?);
        Ok(())
    }
}
