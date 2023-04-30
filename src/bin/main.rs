use std::io::{self, ErrorKind};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use std::time::{Duration, Instant};

use log::{trace, debug, info, warn, error};

const DEFAULT_PORT: u16 = 8080;

async fn process_socket(mut socket: TcpStream) {
    let mut buffer = [0u8; 1024];


    let peer_addr =socket.peer_addr().unwrap();
    info!("New connection: {peer_addr:?}");

    loop {
        match socket.read(&mut buffer).await {
            Ok(r) if r > 0 => {
                info!("Received {} bytes: {:?}", r, &buffer[..r]);
            }
            _ => {
                info!("Connection terminated.");
                break;
            }
        }
    }
}

const LOG_VAR: &str = "RUST_LOG";
const DEFAULT_LOG_LEVEL: &str = "info";

fn setup_logging_env() {
    if std::env::var(LOG_VAR).is_err() {
        std::env::set_var(LOG_VAR, DEFAULT_LOG_LEVEL);
    }

}

#[tokio::main]
async fn main() -> io::Result<()> {
    setup_logging_env();
    pretty_env_logger::init_custom_env(LOG_VAR);


    let port = DEFAULT_PORT;
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).await?;
    info!("Listening on 127.0.0.1:{port}");

    loop {
        let (socket, _) = listener.accept().await?;
        process_socket(socket).await;
    }
}
