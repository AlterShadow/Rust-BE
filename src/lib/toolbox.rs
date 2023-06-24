use crate::database::DbClient;
use crate::error_code::ErrorCode;
use crate::log::LogLevel;
use crate::ws::*;
use crossbeam::queue::SegQueue;
use dashmap::DashMap;
use eyre::*;
use serde::*;
use serde_json::Value;
use std::fmt::{Debug, Display, Formatter};
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoResponseError;

impl Display for NoResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("NoResp")
    }
}

impl std::error::Error for NoResponseError {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomError {
    pub code: ErrorCode,
    pub params: Value,
}

impl CustomError {
    pub fn new(code: impl Into<ErrorCode>, reason: impl Serialize) -> Self {
        Self {
            code: code.into(),
            params: serde_json::to_value(reason)
                .context("Failed to serialize error reason")
                .unwrap(),
        }
    }
    pub fn from_sql_error(err: &str, msg: impl Display) -> Result<Self> {
        let code = u32::from_str_radix(err, 36)?;
        let error_code = ErrorCode::new(code);
        let this = Self {
            code: error_code,
            params: msg.to_string().into(),
        };

        Ok(this)
    }
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.params.to_string())
    }
}

impl std::error::Error for CustomError {}

#[derive(Copy, Clone)]
pub struct RequestContext {
    pub connection_id: ConnectionId,
    pub user_id: i64,
    pub seq: u32,
    pub method: u32,
    pub log_id: u64,
    pub role: u32,
    pub ip_addr: IpAddr,
}

// TODO: make it shared and cloneable
#[derive(Clone)]
pub struct Toolbox {
    db: Vec<DbClient>,
    pub send_msg: Arc<dyn Fn(ConnectionId, WsResponseValue) -> bool + Send + Sync>,
}

impl Toolbox {
    pub fn new() -> Self {
        Self {
            db: vec![],
            send_msg: Arc::new(|_conn_id, _msg| false),
        }
    }

    pub fn set_ws_states(
        &mut self,
        states: Arc<DashMap<ConnectionId, Arc<WsStreamState>>>,
        trigger: mpsc::Sender<ConnectionId>,
        oneshot: bool,
    ) {
        self.send_msg = Arc::new(move |conn_id, msg| {
            let state = if let Some(state) = states.get(&conn_id) {
                state
            } else {
                return false;
            };
            Self::send_ws_msg(&state.message_queue, &trigger, conn_id, msg, oneshot);
            true
        });
    }

    pub fn add_db(&mut self, db: DbClient) {
        self.db.push(db);
    }
    pub fn get_db(&self) -> DbClient {
        self.get_nth_db(0)
    }
    pub fn get_nth_db(&self, index: usize) -> DbClient {
        self.db.get(index).expect("Db not Initialized").clone()
    }

    pub fn send_ws_msg(
        sender: &SegQueue<WsResponseValue>,
        trigger: &mpsc::Sender<ConnectionId>,
        connection_id: ConnectionId,
        resp: WsResponseValue,
        oneshot: bool,
    ) {
        sender.push(resp);
        if oneshot {
            sender.push(WsResponseGeneric::Close);
        }
        trigger
            .try_send(connection_id)
            .unwrap_or_else(|_| error!("Failed to trigger flush: sender full"));
    }
    pub fn send(&self, conn_id: ConnectionId, resp: WsResponseValue) -> bool {
        (self.send_msg)(conn_id, resp)
    }
    pub fn send_response(&self, ctx: &RequestContext, resp: impl Serialize) {
        self.send(
            ctx.connection_id,
            WsResponseValue::Immediate(WsSuccessResponse {
                method: ctx.method,
                seq: ctx.seq,
                params: serde_json::to_value(&resp).unwrap(),
            }),
        );
    }
    pub fn send_internal_error(&self, ctx: &RequestContext, code: ErrorCode, err: Error) {
        self.send(ctx.connection_id, internal_error_to_resp(ctx, code, err));
    }
    pub fn send_request_error(&self, ctx: &RequestContext, code: ErrorCode, err: impl Into<Value>) {
        self.send(ctx.connection_id, request_error_to_resp(ctx, code, err));
    }
    pub fn send_log(&self, ctx: &RequestContext, level: LogLevel, msg: impl Into<String>) {
        self.send(
            ctx.connection_id,
            WsResponseValue::Log(WsLogResponse {
                seq: ctx.seq,
                log_id: ctx.log_id,
                level,
                message: msg.into(),
            }),
        );
    }
    pub fn encode_ws_response<Resp: Serialize>(
        ctx: RequestContext,
        resp: Result<Resp>,
    ) -> Option<WsResponseValue> {
        #[allow(unused_variables)]
        let RequestContext {
            connection_id,
            user_id,
            seq,
            method,
            log_id,
            ..
        } = ctx;
        let resp = match resp {
            Ok(ok) => WsResponseValue::Immediate(WsSuccessResponse {
                method,
                seq,
                params: serde_json::to_value(ok).expect("Failed to serialize response"),
            }),
            Err(err) if err.is::<NoResponseError>() => {
                return None;
            }

            Err(err0) if err0.is::<tokio_postgres::Error>() => {
                let err = err0.downcast_ref::<tokio_postgres::Error>().unwrap();
                if let Some(code) = err.code() {
                    if let Ok(err) = CustomError::from_sql_error(code.code(), &err0) {
                        request_error_to_resp(&ctx, err.code, err.params)
                    } else {
                        internal_error_to_resp(
                            &ctx,
                            ErrorCode::new(100601), // Database Error,
                            err0,
                        )
                    }
                } else {
                    internal_error_to_resp(
                        &ctx,
                        ErrorCode::new(100601), // Database Error,
                        err0,
                    )
                }
            }
            Err(err) if err.is::<CustomError>() => {
                let err = err.downcast::<CustomError>().unwrap();
                request_error_to_resp(&ctx, err.code, err.params)
            }
            Err(err) => internal_error_to_resp(
                &ctx,
                ErrorCode::new(100500), // Internal Error
                err,
            ),
        };
        Some(resp)
    }
}
