## FTP-Server

FTP-Server is a Rust based application that leverages the Clap crate to allow users to upload and download files from the
command line.

More functionality to come!

## Setup

This application requires the cargo package manager to be installed. Instructions on how to install cargo can be found at:

https://doc.rust-lang.org/cargo/getting-started/installation.html

```
git clone git@github.com:sarisssa/ftp-server.git
cd ftp-server
cargo build

```

First, start the Tokio server

```
cd server
cargo run
```

To upload and download the file once we cd back into the root directory

```
cd client
cargo run -- 127.0.0.1:3500 ../world.txt
```

Note, the TCP Listener on the server is currently hard-coded to listen exclusively to Port 3500 which means
that the socket address provided by the user must also end with 3500. I will look into this in the future
to see if the server can listen to a dynamically chosen port.

In addition, I have provided a world.txt file within the root directory of this application for easy demonstration.
Feel free to use other files as the application will work so long as a valid relative file path is provided as an input into the CLI.

## Dependencies

- Rust
- Clap
- Tokio
- Anyhow
