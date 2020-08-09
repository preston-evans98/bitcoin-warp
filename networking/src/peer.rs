use std::net::{ SocketAddr};

pub enum VersionMessageType{
    
}
pub struct Peer{
    pub peer_id: usize,
    pub services: u32,
    pub version: u32,
    pub version_type: VersionMessageType,
    pub ip_address: SocketAddr,
    pub nonce: u64,
}

impl Peer{
    pub fn send(&self,command: Command){
        let msg = Message::new();
        match command {
            Command::Version => {
                msg.create_version_body();
            }
            Command:Verack => {
                
            }
        }
        msg.create_header_for_body();
    }

    pub fn recieve(&self) -> Command{
        //deserialization call here and return the message
    }
} 