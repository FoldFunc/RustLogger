use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::fs::File;
use std::io::Write;
use std::error::Error;
use std::process::{Command, exit};
use std::path::Path;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080";
    let mut stream = TcpStream::connect(addr).await?;
    println!("Connected to the server");
    while let Ok(Some(filename)) = receive_filename(&mut stream).await {
        println!("Receiving: {}", filename);
        if let Err(e) = receive_file(&mut stream, &filename).await {
            eprintln!("Error receiving file {}: {}", filename, e);
            break;
        }
        println!("Received {}", filename);
    }
    open_init().await;
    Ok(())
}

async fn open_init() {
    let script_path_bash = Path::new("send/init.sh");
    let status = Command::new("bash")
        .arg(script_path_bash)
        .status()
        .expect("Failed to execute the script");
    if status.success() {
        println!("Succes");
    }else {
        println!("Error");
        exit(1);
    }
    let script_path_bat = Path::new("send/init.bat");
    let status = Command::new("cmd")
        .arg("/C")
        .arg(script_path_bat)
        .status()
        .expect("Failed to execute the script");
    if status.success() {
        println!("Succes");
    }else {
        println!("Error");
        exit(1);
    }
}

async fn receive_filename(stream: &mut TcpStream) -> Result<Option<String>, Box<dyn Error>> {
    let mut name_len_buf = [0u8; 8];
    match stream.read_exact(&mut name_len_buf).await {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(Box::new(e)),
    }
    let name_len = u64::from_be_bytes(name_len_buf);
    let mut name_buf = vec![0u8; name_len as usize];
    stream.read_exact(&mut name_buf).await?;
    let filename = String::from_utf8(name_buf)?;

    Ok(Some(filename))
}

async fn receive_file(stream: &mut TcpStream, filename: &str) -> Result<(), Box<dyn Error>> {
    // 1) Read 8-byte length prefix for file size
    let mut len_buf = [0u8; 8];
    stream.read_exact(&mut len_buf).await?;
    let mut remaining = u64::from_be_bytes(len_buf);

    // 2) Open file for writing
    let path = format!("send/{}", filename);
    let mut file = File::create(path)?;

    // 3) Read file content
    let mut buf = [0u8; 1024];
    while remaining > 0 {
        let to_read = std::cmp::min(remaining, buf.len() as u64) as usize;
        let n = stream.read(&mut buf[..to_read]).await?;
        if n == 0 {
            return Err("Unexpected EOF while reading file".into());
        }
        file.write_all(&buf[..n])?;
        remaining -= n as u64;
    }

    Ok(())
}

