use crate::command::Command;
use crate::message::Message;
use crate::peer::Peer;
use futures::Future;
// use tokio_io::{AsyncRead, AsyncWrite};

pub enum Version {
    V70015,
}
pub struct Error {
    message: String,
}
impl Error {
    pub fn new(message: String) -> Error {
        Error { message }
    }
}
pub async fn outbound_connection(peer: Peer) -> Result<(), Error> {
    peer.send(Command::Version); //sending version message
    let version_response = peer.receive().await;
    match version_response {
        Command::Version => {
            peer.send(Command::Verack);
        }
        _ => {
            return Err(Error::new(format!(
                "Expected Version message but got {:?}",
                version_response
            )))
        }
    }
    let verack_response = peer.receive().await;
    match verack_response {
        Command::Verack => return Ok(()),
        _ => {
            return Err(Error::new(format!(
                "Expected Verack message but got {:?}",
                verack_response
            )))
        }
    }
}
