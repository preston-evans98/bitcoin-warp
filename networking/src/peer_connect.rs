use crate::command::Command;
use crate::peer::Peer;
use crate::peer::PeerError;
// use tokio_io::{AsyncRead, AsyncWrite};

pub enum Version {
    V70015, //TODO add the rest of the versions eventually
}

pub async fn outbound_connection<'a>(peer: &'a mut Peer<'a>) -> Result<(), PeerError> {
    peer.send(Command::Version).await?; //sending version message
    let version_response = peer.receive(None).await?;
    match version_response {
        Command::Version => {
            peer.send(Command::Verack).await?;
        }
        _ => {
            return Err(PeerError::Message(format!(
                "Expected Version but got {:?}",
                version_response
            )))
        }
    }
    let verack_response = peer.receive(None).await.unwrap();
    match verack_response {
        Command::Verack => return Ok(()),
        _ => {
            return Err(PeerError::Message(format!(
                "Expected Verack message but got {:?}",
                verack_response
            )))
        }
    }
}
pub async fn inbound_connection<'a>(peer: &'a mut Peer<'a>) -> Result<(), PeerError> {
    let version_response = peer.receive(None).await.unwrap();
    match version_response {
        Command::Version => {
            peer.send(Command::Version).await; //sending version message
        }
        _ => {
            return Err(PeerError::Message(format!(
                "Expected Version message but got {:?}",
                version_response
            )))
        }
    }
    peer.send(Command::Verack).await;
    let verack_response = peer.receive(None).await.unwrap();
    match verack_response {
        Command::Verack => return Ok(()),
        _ => {
            return Err(PeerError::Message(format!(
                "Expected Verack message but got {:?}",
                verack_response
            )))
        }
    }
}
