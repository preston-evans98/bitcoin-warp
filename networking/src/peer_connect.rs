use crate::message::Message;
use crate::peer::Peer;
use futures::{Future, Poll, Async};
// use tokio_io::{AsyncRead, AsyncWrite};

pub enum Version{
    V70015,
}
pub struct Error{
    message: String,
}
impl Error{
    pub fn new(message: &str) -> Error{
        Error{message:String::from(message)};
    }
}
pub async fn outbound_connection(peer: Peer) -> Result<(), Error>{
    peer.send(Command::Version); //sending version message
    response= peer.receive().await
    if (presponse == Command::Version) -> Result<(), io::Error>{
        peer.send(Command::Verack);

        if(peer.receive().await == Command::Verack){
            return Ok(())
        }
    }
    else{
        Err(Error::new(response))
    }

    


}
