use config::Config;
use networking::{Command, Message};

fn main() {
    let mut msg = Message::new();
    msg.create_header(Command::Verack, &Config::mainnet());
    println!("{}", msg.dump_header());
}
