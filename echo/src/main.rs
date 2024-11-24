use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = TcpListener::bind("127.0.0.1:42069").await?;
    loop {
        let (mut tcp, _) = server.accept().await?;
        let mut buffer = [0u8; 16];
        loop {
            let n = tcp.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            // convert byte slice to a String
            let mut line = String::from_utf8(buffer[..n].to_vec())?;
            // remove line terminating chars added by telnet
            line.pop(); // remove \n char
            line.pop(); // remove \r char
            // add our own line terminator :)
            line.push_str(" ❤️\n");
            let _ = tcp.write(line.as_bytes()).await?;
        }
    }
}
