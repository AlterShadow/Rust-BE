use web3::transports::{Either, Http, WebSocket};

mod connection;
mod pool;

pub type EitherTransport = Either<WebSocket, Http>;
pub use connection::*;
pub use pool::*;
