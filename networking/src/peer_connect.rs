use crate::command::Command;
use crate::message::Message;
use crate::peer::Peer;
use futures::Future;
use std::fmt;
// use tokio_io::{AsyncRead, AsyncWrite};

pub enum Version {
    V70015, //TODO add the rest of the versions eventually
}
#[derive(Debug)]
pub struct PeerError {
    message: String,
}
impl PeerError {
    pub fn new(message: String) -> PeerError {
        PeerError { message }
    }
}
impl fmt::Display for PeerError{
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), std::fmt::Error>{
        write!(f,"A peer inccured the following error:{}",self.message);
        Ok(())
    }
}
pub async fn outbound_connection(peer: Peer) -> Result<(), PeerError> {
    peer.send(Command::Version); //sending version message
    let version_response = peer.receive().await;
    match version_response {
        Command::Version => {
            peer.send(Command::Verack);
        }
        _ => {
            return Err(PeerError::new(format!(
                "Expected Version message but got {:?}",
                version_response
            )))
        }
    }
    let verack_response = peer.receive().await;
    match verack_response {
        Command::Verack => return Ok(()),
        _ => {
            return Err(PeerError::new(format!(
                "Expected Verack message but got {:?}",
                verack_response
            )))
        }
    }
}
