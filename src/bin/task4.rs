use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect(("google.com", 80)).await?;

    stream.write_all(b"GET / HTTP/1.1\r\nHost: google.com\r\n\r\n").await?;

    let mut buffer = [0u8; 1024];

    let len = stream.read(&mut buffer).await?;
    let buffer = buffer[..len].to_vec();
    let output = String::from_utf8(buffer).unwrap();

    println!("{output}");

    Ok(())
}
