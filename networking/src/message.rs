// use crate::command::Command;
// use crate::header::Header;
// use shared::Bytes;
// use tokio::net::TcpStream;

pub trait Message {
    fn to_bytes(&self) -> Vec<u8>;
}
