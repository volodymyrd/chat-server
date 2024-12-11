use futures::{SinkExt, StreamExt};
use shared::random_name;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};

const HELP_MSG: &str = include_str!("help.txt");

#[derive(Clone)]
struct Names(Arc<Mutex<HashSet<String>>>);

impl Names {
    fn new() -> Self {
        Self(Arc::new(Mutex::new(HashSet::new())))
    }

    /// Returns true if name was inserted.
    fn insert(&self, name: String) -> bool {
        self.0.lock().unwrap().insert(name)
    }

    /// Removes name.
    fn remove(&self, name: &str) -> bool {
        self.0.lock().unwrap().remove(name)
    }

    /// Returns unique name.
    fn get_unique(&self) -> String {
        let mut name = random_name();
        let mut guard = self.0.lock().unwrap();
        while !guard.insert(name.clone()) {
            name = random_name();
        }
        name
    }
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = TcpListener::bind("127.0.0.1:42069").await?;
    // create broadcast channel
    let (tx, _) = broadcast::channel::<String>(32);

    loop {
        let (tcp, _) = server.accept().await?;

        let names = Names::new();

        // clone it for every connected client
        tokio::spawn(handle_user(tcp, tx.clone(), names.clone()));
    }
}

async fn handle_user(mut tcp: TcpStream, tx: Sender<String>, names: Names) -> anyhow::Result<()> {
    let (reader, writer) = tcp.split();
    let mut stream = FramedRead::new(reader, LinesCodec::new());
    let mut sink = FramedWrite::new(writer, LinesCodec::new());
    // get a receiver from the sender
    let mut rx = tx.subscribe();
    // get a unique name for new user
    let mut name = names.get_unique();
    // send list of server commands to
    // the user as soon as they connect
    sink.send(HELP_MSG).await?;
    sink.send(format!("You are {name}")).await?;
    loop {
        tokio::select! {
            msg = stream.next() => {
                let msg = match msg {
                    Some(msg) => msg?,
                    None => break,
                };
                // handle new /name command
                if msg.starts_with("/name") {
                    let new_name = msg
                        .split_ascii_whitespace()
                        .nth(1)
                        .unwrap()
                        .to_owned();
                    // check if name is unique
                    let changed_name = names.insert(new_name.clone());
                    if changed_name {
                        // notify everyone that user changed their name
                        tx.send(format!("{name} is now {new_name}"))?;
                        // remove previous name
                        names.remove(&name);
                        // set new name
                        name = new_name;
                    } else {
                        // tell user that name is already taken
                        sink.send(format!("{new_name} is already taken")).await?;
                    }
                } else if msg.starts_with("/help") {
                    sink.send(HELP_MSG).await?;
                } else if msg.starts_with("/quit") {
                    break;
                } else {
                    tx.send(format!("{name}: {msg}"))?;
                }
            },
            peer_msg = rx.recv() => {
                sink.send(peer_msg?).await?;
            },
        }
    }
    Ok(())
}
