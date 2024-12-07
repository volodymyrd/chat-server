use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};

const HELP_MSG: &str = include_str!("help.txt");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = TcpListener::bind("127.0.0.1:42069").await?;
    // create broadcast channel
    let (tx, _) = broadcast::channel::<String>(32);

    loop {
        let (tcp, addr) = server.accept().await?;

        // clone it for every connected client
        tokio::spawn(handle_user(tcp, tx.clone(), addr));
    }
}

async fn handle_user(
    mut tcp: TcpStream,
    tx: Sender<String>,
    addr: SocketAddr,
) -> anyhow::Result<()> {
    let (reader, writer) = tcp.split();
    let mut stream = FramedRead::new(reader, LinesCodec::new());
    let mut sink = FramedWrite::new(writer, LinesCodec::new());
    // get a receiver from the sender
    let mut rx = tx.subscribe();

    // send list of server commands to
    // the user as soon as they connect
    sink.send(HELP_MSG).await?;

    loop {
        tokio::select! {
            msg = stream.next() => {
                let mut msg = match msg {
                    Some(msg) => msg?,
                    None => break,
                };
                if msg.starts_with("/help") {
                    sink.send(HELP_MSG).await?;
                } else if msg.starts_with("/quit") {
                    break;
                } else {
                    msg.push_str(" ❤️");
                    let _ = tx.send(addr.to_string() + &msg);
                }
            },
            peer_msg = rx.recv() => {
                sink.send(peer_msg?).await?;
            },
        }
    }
    Ok(())
}