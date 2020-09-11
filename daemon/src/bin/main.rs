extern crate hex;
extern crate serde_derive;
use config::Config;
use daemon::Daemon;
use networking::{Command, Message, Peer};
use serde_derive::{Deserializable, Serializable};
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};

// #[derive(Serializable, Deserializable, Debug)]
// pub struct MyTestStruct {
//     identifier: u32,
//     contents: [u8; 4],
// }

// fn test() {
//     // use shared::Deserializable;
//     let inner: [u8; 8] = [1, 2, 3, 4, 1, 2, 3, 4];
//     let test = MyTestStruct::deserialize(&mut std::io::Cursor::new(inner));
//     println!("{:?}", test);
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // test();
    let daemon = Daemon::new();
    // println!("{:?}", daemon);

    let command = Command::Version;
    let config = Config::mainnet();
    let addr = SocketAddr::from(([192, 168, 1, 9], 25000));
    let addr2 = SocketAddr::from(([192, 168, 1, 3], 2400));
    let mut raspi = Peer::at_address(1, addr, &config).await.unwrap();
    let mut peer2 = Peer::at_address(1, addr2, &config).await.unwrap();

    raspi.send(command).await;
    // execute_command(command, daemon);
    Ok(())
}
// fn execute_command(command: Command, daemon: Daemon) {
//     match command {
//         Command::Version => {
//             let mut msg = Message::new();
//             let addr = SocketAddr::from(([192, 168, 1, 8], 8333));

//             // println!("{:?} {:?}", msg.dump_header(), msg.dump_body());
//             // println!("{:?}", msg.dump_contents());
//             // println!("{:?}", msg.get_contents().get_bytes());

//             println!("{:?}", addr.ip());

//             if let Ok(mut stream) = TcpStream::connect(addr) {
//                 println!("Connecting...");
//                 let self_addr = stream.local_addr().unwrap();
//                 println!(
//                     "Connected to the server! Outbound port: {}",
//                     self_addr.port()
//                 );
//                 msg.create_version_body(&self_addr, &addr, &daemon.config);
//                 msg.create_header_for_body(Command::Version, &daemon.config);
//                 stream.set_read_timeout(Some(Duration::new(10, 0))).unwrap();
//                 let retval = stream.write(msg.get_header().get_bytes()).unwrap();
//                 println!("Write returned {}.", retval);
//                 // println!("{}", hex::encode(msg.get_body().get_bytes()));
//                 let retval2 = stream.write(msg.get_body().get_bytes()).unwrap();
//                 println!("Write returned {}.", retval2);
//                 let mut response = [0; 32];
//                 while match stream.read(&mut response) {
//                     Ok(size) => {
//                         // echo everything!
//                         println!("{}", hex::encode(&response[..size]));
//                         true
//                     }
//                     Err(e) => {
//                         println!(
//                             "An error occurred, terminating connection with {}; {}",
//                             stream.peer_addr().unwrap(),
//                             e
//                         );
//                         stream.shutdown(Shutdown::Both).unwrap();
//                         false
//                     }
//                 } {
//                     println!("Looping...");
//                 }
//             } else {
//                 println!("Couldn't connect to server...");
//             }
//         }
//         _ => {
//             println!("didn't match");
//         }
//     }
// }
