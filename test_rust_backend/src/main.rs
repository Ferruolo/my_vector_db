use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Bind the server to localhost:3000
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("Server listening on http://127.0.0.1:3000");

    loop {
        // Accept new connectionsq
        let (mut socket, addr) = listener.accept().await?;

        // Spawn a new task for each connection
        tokio::spawn(async move {
            let mut buffer = vec![0; 1024];

            // Read the request
            match socket.read(&mut buffer).await {
                Ok(n) => {
                    let request = String::from_utf8_lossy(&buffer[..n]);
                    println!("Received request from {addr}:\n{request}\n");

                    // Send a simple response
                    let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!";
                    if let Err(e) = socket.write_all(response.as_bytes()).await {
                        eprintln!("Failed to send response: {e}");
                    }
                }
                Err(e) => eprintln!("Failed to read from socket: {e}"),
            }
        });
    }
}