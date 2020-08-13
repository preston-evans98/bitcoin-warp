use crate::command::Command;
use crate::message::Message;
use crate::peer::Peer;
use crate::peer::PeerError;
use futures::Future;
use std::fmt;
// use tokio_io::{AsyncRead, AsyncWrite};

pub enum Version {
    V70015, //TODO add the rest of the versions eventually
}

pub async fn outbound_connection(peer: &mut Peer) -> Result<(), PeerError> {
    peer.send(Command::Version); //sending version message
    let version_response = peer.receive(None).await.unwrap();
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
    let verack_response = peer.receive(None).await.unwrap();
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
