use http_server::http::{Body, Message, Request, Response};
use std::io::{self};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use log::{debug, info, warn};

const DEFAULT_PORT: u16 = 8080;

async fn handle_request(request: Request) -> Response {
    let body_contents = String::from("hello from Rust").as_bytes().to_vec();

    let message = Message {
        version: request.message.version,
        headers: Vec::new(),
        body: Some(Body {
            contents: body_contents,
        }),
    };

    Response {
        message,
        status_code: http_server::http::StatusCode::OK,
    }
}

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
                        let request = Request::try_from(s.as_str());

                        match request {
                            Ok(request) => {
                                info!("Request: {:?}", request);
                                let response = handle_request(request).await;
                                socket
                                    .write_all(response.serialize().as_slice())
                                    .await
                                    .expect("Failed to write response to socket");
                            }

                            Err(err) => {
                                warn!("Error handling request: {err:?}. Request: {s}");
                            }
                        }
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
    let bind_addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&bind_addr).await?;
    info!("Listening on {bind_addr}");

    loop {
        let (socket, _) = listener.accept().await?;
        process_socket(socket).await;
    }
}
