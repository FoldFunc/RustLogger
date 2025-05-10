use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process::{Command, exit};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080";
    let mut stream = TcpStream::connect(addr).await?;
    println!("Connected to the server");

    let mut file = File::create("init.sh")?;
    let mut buffer = vec![0; 1024];

    loop {
        let n = stream.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        file.write_all(&buffer[..n])?;
    }

    println!("Received the file");
    run_script();
    Ok(())
}

fn run_script() {
    let status = Command::new("bash")
        .arg("init.sh")
        .status()
        .expect("Failed to execute the command");
    if status.success() {
        println!("Command run");
    } else {
        println!("Script failed with status: {}", status);
        exit(1);
    }



}
