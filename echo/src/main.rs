use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};

const HELP_MSG: &str = include_str!("help.txt");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = TcpListener::bind("127.0.0.1:42069").await?;
    loop {
        let (mut tcp, _) = server.accept().await?;
        let (reader, writer) = tcp.split();
        let mut stream = FramedRead::new(reader, LinesCodec::new());
        let mut sink = FramedWrite::new(writer, LinesCodec::new());

        // send list of server commands to
        // the user as soon as they connect
        sink.send(HELP_MSG).await?;

        while let Some(Ok(mut msg)) = stream.next().await {
            if msg.starts_with("/help") { // handle new /help command
                sink.send(HELP_MSG).await?;
            } else if msg.starts_with("/quit") { // handle new /quit command
                break;
            } else {
                msg.push_str(" ❤️");
                sink.send(msg).await?;
            }
        }
    }
}
