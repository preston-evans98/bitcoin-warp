pub mod shell {
    use crate::Warpd;
    use networking::Message;
    use networking::Peer;
    use std::io::BufRead;
    use std::io::Write;
    use tokio::sync::mpsc::UnboundedSender;
    struct StdReader {
        instream: std::io::BufReader<std::io::Stdin>,
        sender: UnboundedSender<String>,
        quit: tokio::sync::oneshot::Receiver<()>,
    }
    impl StdReader {
        fn run(&mut self) {
            self.sender.send(String::from("help")).expect("Uh-oh");
            loop {
                let mut raw_input = String::new();
                match self.quit.try_recv() {
                    Ok(_) => break,
                    Err(_) => {}
                }
                self.instream.read_line(&mut raw_input).unwrap();

                self.sender
                    .send(raw_input.clone())
                    .expect("Couldn't send. Odd");
                match raw_input.trim_end() {
                    "q" | "quit" => break,
                    _ => {}
                }
            }
        }
    }

    fn write_prompt(message: &str) {
        print!("{} ", message);
        std::io::stdout()
            .flush()
            .expect("Yikes. Stdout isn't working");
    }

    async fn peer_mode(
        peer: &mut Peer,
        rx: &mut tokio::sync::mpsc::UnboundedReceiver<String>,
    ) -> Result<(), networking::PeerError> {
        loop {
            write_prompt("warp shell - peer mode>");
            tokio::select! {
                val = peer.receive(None) => {
                    if let Err(e) = val {
                        println!("{:?}", e);
                        println!("  Peer is experiencing issues. Disconnecting...");
                        return Err(e)
                    }
                    println!("");
                },
                val = rx.recv() => {
                    let contents = val.expect("Nothing received");
                    let cmd = contents.trim_end();
                    match cmd {
                        "b" => return Ok(()),
                        "q" | "quit" => std::process::exit(0),
                        "l" => {
                            println!("Waiting to receive...");
                                let _ = peer.receive(None).await;},
                        "h" | "help" => {
                            println!("\npeer shell commands: ");
                            println!("   q: quit");
                            println!("   h: help");
                            println!("   b: back to top levlel");
                            println!("   version: send version msg");
                            println!("   verack: send verack");
                            println!("");
                        }
                        "Version" | "version" =>  {
                            write_prompt("  Sending... ");
                            peer.send(peer.create_version_msg(None)).await.unwrap();
                            write_prompt("Done\n\n");
                        }
                        "Verack" | "verack" =>  {
                            write_prompt("  Sending... ");
                            peer.send(Message::Verack{}).await.unwrap();
                            write_prompt("  Done\n\n");
                        }
                        _ => {
                            println!("Command not recognized: {}\n", cmd);
                            continue;
                        },
                    }
                }
            }
        }
    }

    pub async fn run_shell() -> Result<(), Box<dyn std::error::Error>> {
        // let mut raw_input = String::new();
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let (quitter, quit_recv) = tokio::sync::oneshot::channel();
        let mut std_reader = StdReader {
            instream: std::io::BufReader::new(std::io::stdin()),
            sender: tx,
            quit: quit_recv,
        };
        let h1 = tokio::task::spawn_blocking(|| async move {
            std_reader.run();
        });
        let h2 = tokio::spawn(async move { main_loop(rx, quitter).await });
        let (_, _) = tokio::join!(h2, h1.await.expect("Couldn't run stdReader"));
        Ok(())
    }

    pub async fn main_loop(
        mut rx: tokio::sync::mpsc::UnboundedReceiver<String>,
        quitter: tokio::sync::oneshot::Sender<()>,
    ) {
        let mut warpd = Warpd::new();
        print!("          <Welcome to ");
        loop {
            write_prompt("warp shell>");
            let input = rx.recv().await.expect("Nothing received");
            match input.trim_end() {
                "exit" | "q" => {
                    let _ = quitter.send(());
                    break;
                }
                "help" | "h" => {
                    println!("\nwarp shell commands: ");
                    println!("   q: quit");
                    println!("   h: help");
                    println!("   a: add a peer (outbound)");
                    println!("   l: listen for connection (add peer - inbound)");
                    println!("   p: switch to peer mode");
                    println!("");
                }
                "add" | "a" | "connect" | "c" => {
                    write_prompt("  enter an ip address and port (i.e. 127.0.0.1:8333):");
                    let input = rx.recv().await.expect("Nothing received");
                    if let Ok(addr) = input.trim_end().parse() {
                        println!("  connecting...");
                        if let Err(e) = warpd.add_peer(addr).await {
                            println!("could not connect to peer: {}", e.to_string());
                            continue;
                        } else {
                            println!("peer added at {}\n", addr);
                            let peer: &mut Peer = warpd
                                .conn_man
                                .peers
                                .last_mut()
                                .expect("Should have just added a peeer");
                            if let Err(_) = peer_mode(peer, &mut rx).await {
                                warpd.conn_man.peers.pop();
                            }
                        }
                    } else {
                        println!("could not interpret {} as ip address and port", input);
                    }
                }
                "listen" | "l" => {
                    write_prompt("  enter a port number:");
                    let addr = rx.recv().await.expect("Nothing received");
                    println!("  listening...");
                    if let Err(e) = warpd.accept_peer(addr.trim_end()).await {
                        println!("could not accept connection: {}", e.to_string());
                        continue;
                    } else {
                        println!("peer added at {}", addr);
                    }
                    let peer: &mut Peer = warpd
                        .conn_man
                        .peers
                        .last_mut()
                        .expect("Should have just added a peeer");
                    if let Err(_) = peer_mode(peer, &mut rx).await {
                        warpd.conn_man.peers.pop();
                    }
                }
                "peer" | "p" => match warpd.conn_man.peers.len() {
                    0 => {
                        write_prompt("  No peers found. Press 'a' to add a peer.\n\n");
                        continue;
                    }
                    1 => {
                        write_prompt("  1 peer found. autoselecting...\n\n");
                        let peer = &mut warpd.conn_man.peers[0];
                        if let Err(_) = peer_mode(peer, &mut rx).await {
                            warpd.conn_man.peers.pop();
                        }
                    }
                    _ => loop {
                        write_prompt(&format!(
                            "  select a peer by id (0...{}) or 'b' to go back:",
                            warpd.conn_man.peers.len() - 1
                        ));
                        let input = rx.recv().await.expect("Nothing received");
                        if let Ok(id) = input.trim_end().parse::<usize>() {
                            let peer = &mut warpd.conn_man.peers[id];
                            if let Err(_) = peer_mode(peer, &mut rx).await {
                                warpd.conn_man.peers.remove(id);
                            }
                            break;
                        }
                        if input == "b" {
                            break;
                        }
                        continue;
                    },
                },
                _ => continue,
            }
        }
    }
}
