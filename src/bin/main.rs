use std::io::{self, ErrorKind};
use http_server::http::Request;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use std::time::{Duration, Instant};

use log::{debug, error, info, trace, warn};

const DEFAULT_PORT: u16 = 8080;

async fn process_socket(mut socket: TcpStream) {
    let mut buffer = [0u8; 1024];

    let peer_addr = socket.peer_addr().unwrap();
    info!("New connection: {peer_addr:?}");

    match socket.read(&mut buffer).await {
        Ok(n_bytes) => {
            if n_bytes > 0 {
                match String::from_utf8(buffer[..n_bytes].to_vec()) {
                    Ok(s) => {
                        debug!("Received data from {}: {}", peer_addr, s);
                        let request = Request::try_from(s);
                        info!("Request: {:?}", request);
                    }
                    Err(_err) => {
                        warn!(
                            "Received {n_bytes} bytes from {peer_addr}, but could not parse them to UTF8",
                        );
                    }
                }
            } else {
                warn!("Accepted a connection from {peer_addr} but received no data");
            }
        }

        Err(err) => {
            debug!("Socket read error: {err:?}");
        }
    }
    
    info!("Connection to {peer_addr} terminated");

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
