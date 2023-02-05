use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("localhost:8080").await?;
    let (mut socket, _addr) = listener.accept().await?;

    let (read, mut write) = socket.split();
    let mut reader = BufReader::new(read);
    let mut line = String::new();

    loop {
        let bytes_read = reader.read_line(&mut line).await?;
        if bytes_read == 0 {
            break;
        }

        write.write_all(&line.as_bytes()).await?;
        line.clear();
    }

    Ok(())
}
