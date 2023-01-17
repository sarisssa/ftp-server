pub mod error;

use error::Error;

use std::net::SocketAddr;

use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let allow_overwrites = true;
    let dest_dir = std::env::current_dir()?.join("downloads");

    //Ensure the download directory exists
    std::fs::create_dir_all(&dest_dir)?;

    let listener = TcpListener::bind(SocketAddr::new("0.0.0.0".parse().unwrap(), 3500))
        .await
        .unwrap();

    loop {
        let (mut socket, _) = listener.accept().await?;

        let mut dest_dir = dest_dir.clone();
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let mut file: Option<File> = None;

            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");
                if n == 0 {
                    // if 0 bytes are read, the client closed the connection
                    if file.is_some() {
                        file.unwrap().sync_all().await.expect("failed to sync file");
                        println!("File received");
                    }
                    return;
                }

                //Open file for writing with filename from before \0
                if file.is_none() {
                    if let Some(i) = buf.iter().position(|&b| b == 0) {
                        let filename = String::from_utf8_lossy(&buf[0..i]).to_string();
                        dest_dir.push(filename); // append the filename to the destination directory

                        // if the file exists, break the connection
                        if !allow_overwrites && dest_dir.exists() {
                            println!("File already exists, closing connection");
                            return;
                        }

                        file = Some(File::create(&dest_dir).await.unwrap());

                        // Write remaining data from the buffer to the file
                        file.as_mut()
                            .unwrap()
                            .write_all(&buf[(i + 1)..n])
                            .await
                            .unwrap();
                    }
                } else {
                    // write data to file
                    file.as_mut().unwrap().write_all(&buf[0..n]).await.unwrap();
                }
            }
        });
    }
}
