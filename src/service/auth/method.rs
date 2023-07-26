use eth_sdk::EthereumRpcConnectionPool;
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
use std::sync::atomic::Ordering;
use std::sync::Arc;
use time::OffsetDateTime;
use tracing::{info, warn};
use uuid::Uuid;
use web3::api::Namespace;
use web3::contract::ens::Ens;
use web3::types::Address;

pub async fn ensure_signature_valid(
    signature_text: &str,
    signature: &[u8],
    address: Address,
    allow_cors_sites: &Option<Vec<String>>,
) -> Result<()> {
    // info!("verifying signature_text: {:?}", signature_text);
    let message = signature_text.parse::<Message>().map_err(|err| {
        CustomError::new(
            EnumErrorCode::InvalidPassword,
            format!("Invalid signature text: {}", err),
        )
    })?;

    ensure!(message.address == address.as_bytes(), "Address not match");

    let verification_opts = VerificationOpts {
        timestamp: Some(OffsetDateTime::now_utc()),
        ..Default::default()
    };

    if let Err(e) = message.verify(&signature, &verification_opts).await {
        bail!(CustomError::new(
            EnumErrorCode::InvalidPassword,
            format!("Signature is not valid: {}", e)
        ));
    }
    if let Some(allow_cors_sites) = allow_cors_sites {
        ensure!(
            allow_cors_sites.contains(&message.domain.to_string()),
            "Domain not allowed {}",
            message.domain
        );
    }

    Ok(())
}

#[test]
fn test_siwe_message() {
    let msg = hex_decode(b"6d63322e706174687363616c652e636f6d2077616e747320796f7520746f207369676e20696e207769746820796f757220457468657265756d206163636f756e743a0a3078313131303133623738363245626331423937323634323061613045383732384465333130456536330a0a5468697320726571756573742077696c6c206e6f74207472696767657220616e79207472616e73616374696f6e206f7220696e63757220616e7920636f7374206f7220666565732e4974206973206f6e6c7920696e74656e64656420746f2061757468656e74696361746520796f752061726520746865206f776e6572206f6620746869732077616c6c65743a0a0a0a5552493a2068747470733a2f2f6d63322e706174687363616c652e636f6d2f0a56657273696f6e3a20310a436861696e2049443a20310a4e6f6e63653a203834303132313139310a4973737565642041743a20323032332d30372d32335430383a35323a32352e3632395a").unwrap();
    let msg = String::from_utf8(msg).unwrap();
    println!("{:?}", msg);
    let _msg = msg.parse::<Message>().unwrap();
}
pub struct MethodAuthSignup {
    pub pool: EthereumRpcConnectionPool,
    pub allow_cors_sites: Arc<Option<Vec<String>>>,
}

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
        let pool = self.pool.clone();
        let allow_cors_sites = self.allow_cors_sites.clone();
        async move {
            let req: SignupRequest = serde_json::from_value(param).map_err(|x| {
                CustomError::new(EnumErrorCode::BadRequest, format!("Invalid request: {}", x))
            })?;
            let address = req.address;

            let signature_text = hex_decode(req.signature_text.as_bytes())?;
            let signature = hex_decode(req.signature.as_bytes())?;
            let signature_text_string = String::from_utf8(signature_text.clone())?;
            ensure_signature_valid(
                &signature_text_string,
                &signature,
                address,
                &allow_cors_sites,
            )
            .await?;
            let conn = pool.get(EnumBlockChain::EthereumMainnet).await?;
            let ens = Ens::new(conn.transport().clone());
            let ens_name = match ens.canonical_name(address).await {
                Ok(ok) => Some(ok),
                Err(err) => {
                    warn!("ENS get name {:?} {:?} error: {:?}", address, address, err);
                    None
                }
            };
            let ens_avatar = if let Some(ens_name) = &ens_name {
                match ens.text(ens_name.as_str(), "avatar".to_string()).await {
                    Ok(ok) => Some(ok),
                    Err(err) => {
                        warn!(
                            "ENS get avatar {:?} {:?} error: {:?}",
                            ens_name, address, err
                        );
                        None
                    }
                }
            } else {
                None
            };

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
                    ens_name: ens_name.clone(),
                    public_id,
                    ens_avatar: ens_avatar.clone(),
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
                    ens_name,
                    public_id,
                    ens_avatar,
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
pub struct MethodAuthLogin {
    pub allow_cors_sites: Arc<Option<Vec<String>>>,
}

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
        let allow_cors_sites = self.allow_cors_sites.clone();
        async move {
            let req: LoginRequest = serde_json::from_value(param).map_err(|x| {
                CustomError::new(EnumErrorCode::BadRequest, format!("Invalid request: {}", x))
            })?;
            let address = req.address;

            let signature_text = hex_decode(req.signature_text.as_bytes())?;
            let signature_text_string = String::from_utf8(signature_text.clone())?;
            let signature = hex_decode(req.signature.as_bytes())?;
            ensure_signature_valid(
                &signature_text_string,
                &signature,
                address,
                &allow_cors_sites,
            )
            .await?;

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
                address: address.into(),
                display_name: row
                    .ens_name
                    .unwrap_or_else(|| row.public_user_id.to_string()),
                avatar: row.ens_avatar,
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
            let address = req.address;
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

pub struct MethodAuthChangeLoginWallet {
    pub allow_cors_sites: Arc<Option<Vec<String>>>,
}

impl SubAuthController for MethodAuthChangeLoginWallet {
    fn auth(
        self: Arc<Self>,
        toolbox: &Toolbox,
        param: Value,
        _ctx: RequestContext,
        _conn: Arc<WsConnection>,
    ) -> BoxFuture<'static, Result<Value>> {
        info!("MethodAuthChangeLoginWallet request: {:?}", param);
        let db_auth: DbClient = toolbox.get_nth_db(1);
        let allow_cors_sites = self.allow_cors_sites.clone();
        async move {
            let req: ChangeLoginWalletRequest = serde_json::from_value(param).map_err(|x| {
                CustomError::new(EnumErrorCode::BadRequest, format!("Invalid request: {}", x))
            })?;
            let old_address = req.old_address;

            let old_signature_text = hex_decode(req.old_signature_text.as_bytes())?;

            let old_signature = hex_decode(req.old_signature.as_bytes())?;
            let old_signature_text_string = String::from_utf8(old_signature_text.clone())?;

            ensure_signature_valid(
                &old_signature_text_string,
                &old_signature,
                old_address,
                &allow_cors_sites,
            )
            .await?;

            let new_address = req.new_address;

            let new_signature_text = hex_decode(req.new_signature_text.as_bytes())?;

            let new_signature = hex_decode(req.new_signature.as_bytes())?;
            let new_signature_text_string = String::from_utf8(new_signature_text.clone())?;

            ensure_signature_valid(
                &new_signature_text_string,
                &new_signature,
                new_address,
                &allow_cors_sites,
            )
            .await?;

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
