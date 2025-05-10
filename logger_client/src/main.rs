use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process::{Command, exit};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080";
    let mut stream = TcpStream::connect(addr).await?;
    println!("Connected to the server");

    // Receive both files with a simple length‐prefix protocol:
    receive_file(&mut stream, "init.sh").await?;
    println!("✅ Received init.sh");

    receive_file(&mut stream, "init.bat").await?;
    println!("✅ Received init.bat");

    // Now you can run your script
    //run_script();
    Ok(())
}

async fn receive_file(
    stream: &mut TcpStream,
    filename: &str
) -> Result<(), Box<dyn Error>> {
    // 1) Read exactly 8 bytes for the big‐endian length prefix:
    let mut len_buf = [0u8; 8];
    stream.read_exact(&mut len_buf).await?;
    let mut remaining = u64::from_be_bytes(len_buf);

    // 2) Open the output file:
    let mut file = File::create(filename)?;

    // 3) Read until we've got `remaining` bytes:
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

fn run_script() {
    let status = Command::new("bash")
        .arg("init.sh")
        .status()
        .expect("Failed to execute the command");
    if status.success() {
        println!("Script ran successfully");
    } else {
        eprintln!("Script failed with status: {}", status);
        exit(1);
    }
}

