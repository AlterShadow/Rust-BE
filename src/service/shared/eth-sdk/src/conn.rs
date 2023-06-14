use eyre::*;

use web3::transports::{Either, Http, WebSocket};

use web3::Web3;

pub type EitherTransport = Either<WebSocket, Http>;

pub type EthereumRpcConnection = Web3<EitherTransport>;

pub async fn new_transport(url: &str) -> Result<EitherTransport> {
    let transport = match url {
        x if x.starts_with("http") => {
            EitherTransport::Right(Http::new(&url).context(url.to_owned())?)
        }
        x if x.starts_with("ws") => {
            EitherTransport::Left(WebSocket::new(&url).await.context(url.to_owned())?)
        }
        _ => bail!("Invalid provider url: {}", url),
    };
    Ok(transport)
}
