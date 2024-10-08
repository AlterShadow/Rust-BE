use eyre::*;
use futures::stream::SplitStream;
use futures::SinkExt;
use futures::StreamExt;
use itertools::Itertools;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::error::ProtocolError;
use tokio_tungstenite::tungstenite::Error as WsError;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use tracing::*;

use crate::database::DbClient;
use crate::error_code::ErrorCode;
use crate::handler::*;
use crate::listener::{ConnectionListener, TcpListener, TlsListener};
use crate::toolbox::{RequestContext, Toolbox};
use crate::utils::{get_conn_id, get_log_id};
use crate::ws::basics::{WsConnection, WsRequestValue};
use crate::ws::VerifyProtocol;
use crate::ws::WebsocketStates;
use crate::ws::WsEndpoint;
use crate::ws::WsResponseValue;
use crate::ws::WsStreamSink;
use crate::ws::{request_error_to_resp, WsStreamState};
use crate::ws::{AuthController, ConnectionId};
use crate::ws::{SimpleAuthContoller, WsRequest};
use model::endpoint::EndpointSchema;
use serde::{Deserialize, Serialize};

pub struct WebsocketServer {
    pub auth_controller: Arc<dyn AuthController>,
    pub handlers: HashMap<u32, WsEndpoint>,
    pub message_receiver: Option<mpsc::Receiver<ConnectionId>>,
    toolbox: Toolbox,
    pub config: WsServerConfig,
}

impl WebsocketServer {
    pub fn new(config: WsServerConfig) -> Self {
        Self {
            auth_controller: Arc::new(SimpleAuthContoller),
            handlers: Default::default(),
            message_receiver: None,
            toolbox: Toolbox::new(),
            config,
        }
    }
    pub fn add_auth_controller(&mut self, controller: impl AuthController + 'static) {
        self.auth_controller = Arc::new(controller);
    }
    pub fn add_database(&mut self, db: DbClient) {
        self.toolbox.add_db(db);
    }

    pub fn add_handler<T: RequestHandler + 'static>(&mut self, handler: T) {
        let schema = serde_json::from_str(T::Request::SCHEMA).expect("Invalid schema");
        check_handler::<T>(&schema).expect("Invalid handler");
        self.add_handler_erased(schema, Arc::new(handler))
    }
    pub fn add_handler_erased(
        &mut self,
        schema: EndpointSchema,
        handler: Arc<dyn RequestHandlerErased>,
    ) {
        let old = self
            .handlers
            .insert(schema.code, WsEndpoint { schema, handler });
        if let Some(old) = old {
            panic!(
                "Overwriting handler for endpoint {} {}",
                old.schema.code, old.schema.name
            );
        }
    }
    async fn handle_connection<S: AsyncRead + AsyncWrite + Unpin + Send + 'static>(
        self: Arc<Self>,
        addr: SocketAddr,
        states: Arc<WebsocketStates<S>>,
        stream: S,
    ) -> Result<()> {
        let (tx, mut rx) = mpsc::channel(1);
        let hs = tokio_tungstenite::accept_hdr_async(
            stream,
            VerifyProtocol {
                addr,
                tx,
                allow_cors_domains: &self.config.allow_cors_urls,
            },
        )
        .await;
        let stream = wrap_ws_error(hs)?;
        let conn = Arc::new(WsConnection {
            connection_id: get_conn_id(),
            user_id: Default::default(),
            role: AtomicU32::new(0),
            address: addr,
            log_id: get_log_id(),
        });
        debug!(?addr, "New connection handshaken {:?}", conn);
        let headers = rx
            .recv()
            .await
            .ok_or_else(|| eyre!("Failed to receive ws headers"))?;
        let (ws_sink, ws_stream) = stream.split();

        let conn = Arc::clone(&conn);
        states.insert(conn.connection_id, ws_sink, conn.clone());

        let auth_result = Arc::clone(&self.auth_controller)
            .auth(&self.toolbox, headers, Arc::clone(&conn))
            .await;
        let raw_ctx = RequestContext {
            connection_id: conn.connection_id,
            user_id: conn.get_user_id(),
            seq: 0,
            method: 0,
            log_id: conn.log_id.clone(),
            role: conn.role.load(Ordering::Relaxed),
            ip_addr: conn.address.ip(),
        };
        if let Err(err) = auth_result {
            self.toolbox.send_request_error(
                &raw_ctx,
                ErrorCode::new(100400), // BadRequest
                err.to_string(),
            );
            return Err(err);
        }
        if !self.config.header_only {
            debug!(?addr, "Starting ws recv_msg loop");
            self.recv_msg(conn, states, ws_stream).await;
        }
        Ok(())
    }

    pub async fn recv_msg<S: AsyncRead + AsyncWrite + Unpin + Send + 'static>(
        self: Arc<Self>,
        conn: Arc<WsConnection>,
        states: Arc<WebsocketStates<S>>,
        mut reader: SplitStream<WebSocketStream<S>>,
    ) {
        let addr = conn.address;
        let mut context = RequestContext {
            connection_id: conn.connection_id,
            user_id: conn.get_user_id(),
            seq: 0,
            method: 0,
            log_id: conn.log_id.clone(),
            role: conn.role.load(Ordering::Relaxed),
            ip_addr: conn.address.ip(),
        };
        while let Some(msg) = reader.next().await {
            match msg {
                Ok(req) => {
                    let obj: Result<WsRequestValue, _> = match req {
                        Message::Text(t) => {
                            debug!(?addr, "Handling request {}", t);

                            serde_json::from_str(&t)
                        }
                        Message::Binary(b) => {
                            debug!(?addr, "Handling request <BIN>");
                            serde_json::from_slice(&b)
                        }
                        Message::Ping(_) => {
                            continue;
                        }
                        Message::Pong(_) => {
                            continue;
                        }
                        Message::Close(_) => {
                            info!(?addr, "Receive side terminated");
                            break;
                        }
                        _ => {
                            warn!(?addr, "Strange pattern {:?}", req);
                            continue;
                        }
                    };
                    let req = match obj {
                        Ok(req) => req,
                        Err(err) => {
                            self.toolbox.send(
                                context.connection_id,
                                request_error_to_resp(
                                    &context,
                                    ErrorCode::new(100400), // BadRequest
                                    err.to_string(),
                                ),
                            );
                            continue;
                        }
                    };
                    context.seq = req.seq;
                    context.method = req.method;
                    context.user_id = conn.get_user_id();

                    let handler = self.handlers.get(&req.method);
                    let handler = match handler {
                        Some(handler) => handler,
                        None => {
                            self.toolbox.send(
                                context.connection_id,
                                request_error_to_resp(
                                    &context,
                                    ErrorCode::new(100501), // Not Implemented
                                    Value::Null,
                                ),
                            );
                            continue;
                        }
                    };
                    tokio::spawn(handler.handler.handle(&self.toolbox, context, req.params));
                }
                Err(WsError::Protocol(ProtocolError::ResetWithoutClosingHandshake)) => {
                    debug!(?addr, "Receive side terminated");
                    break;
                }
                Err(err) => {
                    error!(?addr, "Error while receiving {:?}", err);
                    break;
                }
            }
        }
        states.remove(context.connection_id);
        debug!(?addr, "Connection closed");
    }
    pub async fn send_msg_single<S: AsyncRead + AsyncWrite + Unpin + Send + 'static>(
        self: Arc<Self>,
        states: Arc<WebsocketStates<S>>,
        conn: Arc<WsStreamSink<S>>,
        state: Arc<WsStreamState>,
    ) {
        let addr = state.conn.address;
        let mut sink = conn.ws_sink.lock().await;
        while let Some(msg) = state.message_queue.pop() {
            let timeout_operation = async {
                match &msg {
                    WsResponseValue::Close => {
                        debug!(?addr, "Closing connection");
                        let _ = sink.send(Message::Close(None)).await;
                        let _ = sink.close().await;
                        states.remove(state.conn.connection_id);
                        debug!(?addr, "Connection closed");
                    }
                    resp => {
                        let resp_str =
                            serde_json::to_string(&resp).expect("Failed to dump json(impossible)");
                        debug!(?addr, "Sending message {}", resp_str);

                        let result = sink.send(Message::Text(resp_str)).await;
                        if let Err(err) = result {
                            error!(?addr, "Error while sending {:?}", err);
                            let _ = sink.send(Message::Close(None)).await;
                            let _ = sink.close().await;
                            states.remove(state.conn.connection_id);
                            debug!(?addr, "Connection closed");
                        }
                    }
                }
            };
            if let Err(err) = tokio::time::timeout(Duration::from_secs(1), timeout_operation).await
            {
                error!(?addr, "Timeout while sending message: {:?}", err);
                states.remove(state.conn.connection_id);
            }
        }
    }
    pub async fn send_msg<S: AsyncRead + AsyncWrite + Unpin + Send + 'static>(
        self: Arc<Self>,
        states: Arc<WebsocketStates<S>>,
        mut message_receiver: mpsc::Receiver<ConnectionId>,
    ) {
        while let Some(conn_id) = message_receiver.recv().await {
            if let Some(conn) = states.get_connection(conn_id) {
                if conn.ws_sink.try_lock().is_ok() {
                    let state = states.get_state(conn_id).unwrap();
                    tokio::spawn(Arc::clone(&self).send_msg_single(
                        Arc::clone(&states),
                        conn,
                        state,
                    ));
                }
            } else {
                warn!(?conn_id, "Connection not found");
            }
        }
    }
    pub async fn listen(self) -> Result<()> {
        let addr = (self.config.host.as_ref(), self.config.port)
            .to_socket_addrs()?
            .next()
            .context("Failed to resolve address")?;
        if self.config.pub_certs.is_none() && self.config.priv_cert.is_none() {
            let listener = TcpListener::bind(addr).await?;
            self.listen_impl(Arc::new(listener), addr).await
        } else if !self.config.pub_certs.is_none() && !self.config.priv_cert.is_none() {
            let listener = TcpListener::bind(addr).await?;

            let listener = TlsListener::bind(
                listener,
                self.config.pub_certs.clone().unwrap(),
                self.config.priv_cert.clone().unwrap(),
            )
            .await?;
            self.listen_impl(Arc::new(listener), addr).await
        } else {
            bail!("pub_cert and priv_cert should be both set or unset")
        }
    }

    async fn listen_impl<T: ConnectionListener + 'static>(
        mut self,
        listener: Arc<T>,
        listen_addr: SocketAddr,
    ) -> Result<()> {
        info!("{} listening on {}", self.config.name, listen_addr);

        let states = Arc::new(WebsocketStates::new());
        let (tx, rx) = mpsc::channel(100);
        let message_receiver = rx;
        self.toolbox
            .set_ws_states(states.clone_states(), tx, self.config.header_only);
        let this = Arc::new(self);
        tokio::spawn(Arc::clone(&this).send_msg(Arc::clone(&states), message_receiver));
        loop {
            let (stream, addr) = match listener.accept().await {
                Ok(x) => x,
                Err(err) => {
                    error!("Error while accepting stream: {:?}", err);
                    continue;
                }
            };
            let listener = Arc::clone(&listener);
            let this = Arc::clone(&this);
            let states = Arc::clone(&states);
            tokio::spawn(async move {
                let stream = match listener.handshake(stream).await {
                    Ok(channel) => {
                        info!("Accepted stream from {}", addr);
                        channel
                    }
                    Err(err) => {
                        error!("Error while handshaking stream: {:?}", err);
                        return;
                    }
                };

                if let Err(err) = this.handle_connection(addr, states, stream).await {
                    error!("Error while handling connection: {:?}", err);
                }
            });
        }
    }
    pub fn dump_schemas(&self) -> Result<()> {
        let _ = std::fs::create_dir_all("docs");
        let file = format!("docs/{}_alive_endpoints.json", self.config.name);
        let available_schemas: Vec<String> = self
            .handlers
            .values()
            .map(|x| x.schema.name.clone())
            .sorted()
            .collect();
        info!(
            "Dumping {} endpoint names to {}",
            available_schemas.len(),
            file
        );
        serde_json::to_writer_pretty(File::create(file)?, &available_schemas)?;
        Ok(())
    }
}

pub fn wrap_ws_error<T>(err: Result<T, WsError>) -> Result<T> {
    err.map_err(|x| eyre!(x))
}

pub fn check_name(cat: &str, be_name: &str, should_name: &str) -> Result<()> {
    if !be_name.contains(&should_name) {
        bail!("{} name should be {} but got {}", cat, should_name, be_name);
    } else {
        Ok(())
    }
}

pub fn check_handler<T: RequestHandler + 'static>(schema: &EndpointSchema) -> Result<()> {
    let handler_name = std::any::type_name::<T>();
    let should_handler_name = format!("Method{}", schema.name);
    check_name("Method", handler_name, &should_handler_name)?;
    let request_name = std::any::type_name::<T::Request>();
    let should_req_name = format!("{}Request", schema.name);
    check_name("Request", request_name, &should_req_name)?;

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsServerConfig {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: u16,
    #[serde(default)]
    pub pub_certs: Option<Vec<String>>,
    #[serde(default)]
    pub priv_cert: Option<String>,
    #[serde(default)]
    pub debug: bool,
    #[serde(skip)]
    pub header_only: bool,
    #[serde(skip)]
    pub allow_cors_urls: Arc<Option<Vec<String>>>,
}
