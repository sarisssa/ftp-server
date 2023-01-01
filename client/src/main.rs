use clap::{value_parser, Arg, ArgAction, Command, Parser};
use std::{
    fs::File,
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    path::PathBuf,
    str::FromStr,
};

use anyhow::{anyhow, Context, Result};

// FIRST INPUT: 127.0.0.1:8080
// Second Input: world.txt

#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
struct Args {
    addr: String,
    file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let addr = args.addr;
    let file = args.file;

    //Convert CLI input into SocketAddr and PathBuf respectively
    let socket_addr = SocketAddr::from_str(&addr).context(format!("Invalid socket address"))?;
    let file_path = PathBuf::from(&file);

    //Does it make sense to make a check to see if file_path exists or is Line 51 sufficient?
    if !file_path.exists() {
        return Err(anyhow!("Input file {:?} does not exist", file_path));
    }

    let mut sock = TcpStream::connect(socket_addr)
        .context(format!("Unable to connect to '{}", socket_addr))?;

    println!("Connected to server: {:?}", sock.peer_addr());

    // Get the filename from pathbuf
    let filename = file_path.file_name().unwrap().to_str().unwrap();

    // Ensure filename has a \0 terminator
    let mut filename = filename.to_string();

    filename.push('\0');
    sock.write(filename.as_bytes())
        .context(format!("Unable to write '{}' to socket", filename))?;

    let mut data = vec![0; 1024];

    // Read file into data
    let mut file =
        File::open(&file_path).context(format!("Unable to open '{}'", file_path.display()))?;
    let mut bytes_written = 0;
    let filesize = file.metadata()?.len();

    // While there is data to read from file in chucks of 1024 bytes
    while let Ok(n) = file
        .read(&mut data)
        .context(format!("Unable to read data from '{:?}'", file))
    {
        // If there is no data to read
        if n == 0 {
            // Break out of the loop
            break;
        }
        sock.write_all(&data[0..n])
            .context(format!("Unable to write to '{:?}'", sock))?;
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

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn test_01() {
//         let m = Command::new("myapp")
//             .arg(
//                 Arg::new("port")
//                     .value_parser(value_parser!(usize))
//                     .action(ArgAction::Set)
//                     .required(true),
//             )
//             .get_matches_from(vec!["myapp", "2020"]);

//         let port: usize = *m.get_one("port").expect("`port`is required");
//         println!("This is the port: {:?}", port);
//         assert_eq!(port, 2020);
//     }
// }
