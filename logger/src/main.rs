use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;
use std::fs::File;
use std::io::Read;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = send_file(&mut socket, "init.sh").await {
                eprintln!("Error sending init.sh: {:?}", e);
                return;
            }
            if let Err(e) = send_file(&mut socket, "init.bat").await {
                eprintln!("Error sending init.bat: {:?}", e);
                return;
            }
            // socket is dropped here, closing the connection
        });
    }
}

async fn send_file(socket: &mut TcpStream, filename: &str) -> Result<(), Box<dyn Error>> {
    // Read the file into memory
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // 1) Send an 8-byte big-endian length prefix
    let len = buffer.len() as u64;
    socket.write_all(&len.to_be_bytes()).await?;

    // 2) Send the actual file bytes
    socket.write_all(&buffer).await?;
    println!("Sent `{}` ({} bytes)", filename, len);

    Ok(())
}

