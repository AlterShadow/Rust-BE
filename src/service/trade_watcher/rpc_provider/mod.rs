use web3::transports::{Either, Http, WebSocket};

pub mod connection;
pub mod pool;

pub type EitherTransport = Either<WebSocket, Http>;
