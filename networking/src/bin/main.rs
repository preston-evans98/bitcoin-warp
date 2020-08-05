use config::Config;
use crypto;
use networking::{Command, Message};
use shared::{Bytes, Serializable};

fn main() {
    let mut msg = Message::new();
    msg.create_header(Command::Verack, &Config::mainnet());
    println!("{}", msg.dump_header());
    // println!(
    //     "{}",
    //     Bytes::from(crypto::double_sha256(&b"hello".to_vec())).hex()
    // );
}
