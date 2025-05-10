use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;
use std::fs::File;
use std::io::Read;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on port: {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            if let Err(e) = send_file(&mut socket).await {
                eprintln!("Error sending file through socket: {:?}", e);
            }
        });
    }
}

async fn send_file(socket: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let file_path = "init.sh";

    // ✅ FIX 1: Handle Result properly with ?
    let mut file = File::open(file_path)?;

    let mut buffer = Vec::new();

    // ✅ FIX 2: Synchronous read is OK here, but don't forget the ?
    file.read_to_end(&mut buffer)?;

    // ✅ FIX 3: Use async write to socket
    socket.write_all(&buffer).await?;
    println!("File sent through socket");

    Ok(())
}

