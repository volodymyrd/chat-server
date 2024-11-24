use futures::{StreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = TcpListener::bind("127.0.0.1:42069").await?;
    loop {
        let (mut tcp, _) = server.accept().await?;
        let (reader, writer) = tcp.split();
        let stream = FramedRead::new(reader, LinesCodec::new());
        let sink = FramedWrite::new(writer, LinesCodec::new());

        stream.map(|msg| {
            let mut msg = msg?;
            msg.push_str(" ❤️");
            Ok(msg)
        }).forward(sink).await?
    }
}
