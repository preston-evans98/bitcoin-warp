extern crate hex;
extern crate serde_derive;
use tracing_subscriber::{filter::LevelFilter, fmt};
use warpd::run_shell;
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

// async fn sleeper() -> Result<(), std::io::Error> {
//     println!("1 Started");
//     sleep(Duration::from_secs(3)).await;
//     // tokio::time::sleep(tokio::time::Duration::from_secs(3));
//     println!("1 Done");
//     Ok(())
// }

// async fn sleeper2() -> Result<(), std::io::Error> {
//     println!("2 Started");
//     sleep(Duration::from_secs(3)).await;
//     // tokio::time::sleep(tokio::time::Duration::from_secs(3));
//     println!("2 Done");
//     Ok(())
// }

// async fn mini_main() {
//     // println!("Main started!");
//     // let handle = tokio::spawn(sleeper());
//     // let handle2 = tokio::spawn(async {
//     //     sleep(Duration::from_secs(1)).await;
//     //     sleeper2().await
//     // });
//     // sleep(Duration::from_secs(5)).await;
//     // println!("Main done!");
//     // tokio::join!(handle, handle2);
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
    //     .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
    //     .init();
    let subscriber = fmt().with_max_level(LevelFilter::TRACE).finish();
    let _ = tracing::subscriber::set_global_default(subscriber)
        .map_err(|_err| eprintln!("Unable to set global default subscriber"));
    run_shell().await?;
    // // test();

    // // println!("{:?}", warpd);

    // let command = Command::Version;
    // let config = Config::mainnet();
    // // let addr = SocketAddr::from(([192, 168, 1, 2], 8333));
    // let addr = SocketAddr::from(([192, 168, 1, 8], 8333));
    // let mut raspi = Peer::at_address(1, addr, &config).await.unwrap();

    // // let mut peer2 = Peer::at_address(1, addr2, &config).await.unwrap();
    // let result = raspi.perform_handshake(None).await;
    // println!("{:?}", result);
    // raspi.send(command).await;
    // execute_command(command, warpd);
    Ok(())
}

// fn execute_command(command: Command, warpd: Warpd) {
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
//                 msg.create_version_body(&self_addr, &addr, &warpd.config);
//                 msg.create_header_for_body(Command::Version, &warpd.config);
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
