use crate::error_code::ErrorCode;
use crate::toolbox::{RequestContext, Toolbox};
use crate::ws::*;
use core::marker::{Send, Sync};
use serde_json::Value;
use std::marker::PhantomData;
pub struct SpawnedResponse<T> {
    _phantom: PhantomData<T>,
}
impl<T> SpawnedResponse<T> {
    pub fn new() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}
pub trait RequestHandler: Send + Sync {
    type Request: WsRequest;
    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> SpawnedResponse<Self::Request>;
}

pub trait RequestHandlerErased: Send + Sync {
    fn handle(&self, toolbox: &Toolbox, ctx: RequestContext, req: Value);
}

impl<T: RequestHandler> RequestHandlerErased for T {
    fn handle(&self, toolbox: &Toolbox, ctx: RequestContext, req: Value) {
        let data: T::Request = match serde_json::from_value(req) {
            Ok(data) => data,
            Err(err) => {
                toolbox.send(
                    ctx.connection_id,
                    request_error_to_resp(
                        &ctx,
                        ErrorCode::new(100400), // Bad Request
                        err.to_string(),
                    ),
                );
                return;
            }
        };

        RequestHandler::handle(self, toolbox, ctx, data);
    }
}
