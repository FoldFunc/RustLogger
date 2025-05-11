use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::fs;
use tokio::fs::File;
use std::error::Error;
use std::path::Path;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let subdir = "send/";

            match list_files_in_subdir(subdir).await {
                Ok(files) => {
                    println!("Files in directory: {:?}", files);
                    for file in files {
                        if let Err(e) = send_file(&mut socket, &file).await {
                            eprintln!("Error sending {}: {}", file, e);
                            return;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading files in subdir: {}", e);
                }
            }
        });
    }
}

async fn list_files_in_subdir<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let mut all_files = Vec::new();
    let mut entries = fs::read_dir(path).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.to_str() {
                all_files.push(name.to_string());
            }
        }
    }

    Ok(all_files)
}

async fn send_file(socket: &mut TcpStream, filepath: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(filepath);
    let file_name = match path.file_name().and_then(|n| n.to_str()) {
        Some(name) => name,
        None => return Err("Invalid filename".into()),
    };

    // 1. Send file name length and bytes
    let name_bytes = file_name.as_bytes();
    let name_len = name_bytes.len() as u64;
    socket.write_all(&name_len.to_be_bytes()).await?;
    socket.write_all(name_bytes).await?;

    // 2. Send file content length and content
    let mut file = File::open(filepath).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;
    let file_len = buffer.len() as u64;

    socket.write_all(&file_len.to_be_bytes()).await?;
    socket.write_all(&buffer).await?;

    println!("📤 Sent file: {} ({} bytes)", file_name, file_len);
    Ok(())
}

