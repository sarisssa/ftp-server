mod error;
use clap::Parser;
use error::Error;
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
struct Args {
    addr: SocketAddr,
    file: PathBuf,
}
fn main() -> Result<(), Error> {
    let args = Args::parse();
    let addr = args.addr;
    let file = args.file;
    //Check if file exists
    if !file.exists() {
        return Err(Error::FileNotFound(file));
    }

    let mut sock = TcpStream::connect(addr)?;
    println!("Connected to server: {:?}", sock.peer_addr()?);

    // Get the filename from pathbuf
    let filename = file.file_name().unwrap().to_str().unwrap();

    // Ensure filename has a \0 terminator
    let mut filename = filename.to_string();

    filename.push('\0');
    sock.write(filename.as_bytes())?;

    let mut data = vec![0; 1024];

    // Read file into data
    let mut file = std::fs::File::open(file)?;
    let mut bytes_written = 0;
    let filesize = file.metadata()?.len();

    // While there is data to read from file in chucks of 1024 bytes
    while let Ok(n) = file.read(&mut data) {
        // If there is no data to read
        if n == 0 {
            // Break out of the loop
            break;
        }
        sock.write_all(&data[0..n])?;
        bytes_written += n;
        println!(
            "Sent {} bytes of {} ({:.0}%)",
            bytes_written,
            filesize,
            (bytes_written as f64 / filesize as f64) * 100.0
        );
    }
    Ok(())
}
