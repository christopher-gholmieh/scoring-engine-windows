// Written by: Christopher Gholmieh
// Crates:
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::fs;
use std::path::Path;

use tokio::sync::{Mutex, mpsc};
use std::sync::Arc;

// Super:
use super::engine::Update;

// Socket:
pub struct Socket {}

// Implementation:
impl Socket {
    // Constructor:
    pub fn new() -> Self {
        Self {}
    }

    // Methods:
    fn file_to_mime(path: &str) -> &str {
        if path.ends_with(".html") {
            "text/html"
        } else if path.ends_with(".css") {
            "text/css"
        } else if path.ends_with(".js") {
            "application/javascript"
        } else if path.ends_with(".png") {
            "image/png"
        } else if path.ends_with(".jpg") || path.ends_with(".jpeg") {
            "image/jpeg"
        } else {
            "application/octet-stream"
        }
    }

    fn serve_file(path: &str) -> Result<(String, Vec<u8>), std::io::Error> {
        // Variables (Assignment):
        // Path:
        let file_path: String = if path == "/" {
            "./website/index.html".to_string()
        } else {
            format!("./website{}", path)
        };

        // Logic:
        if Path::new(&file_path).exists() {
            // Variables (Assignment):
            // MIME:
            let mime: &str = Self::file_to_mime(&file_path);

            // Logic:
            match fs::read(&file_path) {
                Ok(bytes) => {
                    // Variables (Assignment):
                    // Header:
                    let header = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
                        bytes.len(),
                        mime
                    );

                    // Logic:
                    Ok((header, bytes))
                }
                Err(error) => Err(error),
            }
        } else {
            // Variables (Assignment):
            // Body:
            let body = b"<h1>404 Not Found</h1>".to_vec();

            // Header:
            let header = format!(
                "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n",
                body.len()
            );

            // Logic:
            Ok((header, body))
        }
    }

    pub async fn serve(&self, mut receiver: mpsc::Receiver<Update>) -> Result<(), Box<dyn std::error::Error>> {
        // Variables (Assignment):
        // Listener:
        let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").await?;

        // Update:
        let latest_update: Arc<Mutex<Option<Update>>> = Arc::new(Mutex::new(None::<Update>));

        // Clone:
        let update_clone: Arc<Mutex<Option<Update>>> = Arc::clone(&latest_update);

        tokio::spawn(async move {
            while let Some(update) = receiver.recv().await {
                // Variables (Assignment):
                // Guard:
                let mut guard = update_clone.lock().await;

                // Logic:
                *guard = Some(update);
            }
        });

        // Logic:
        loop {
            // Variables (Assignment):
            // Socket & Address:
            let (mut socket, _) = listener.accept().await?;

            // Clone:
            let latest_clone: Arc<Mutex<Option<Update>>> = Arc::clone(&latest_update);

            // Logic:
            tokio::spawn(async move {
                // Variables (Assignment):
                // Buffer:
                let mut buffer = [0u8; 1024];

                // Number:
                let number_bytes: usize = match socket.read(&mut buffer).await {
                    // Close:
                    Ok(number) if number == 0 => return,

                    // Bytes:
                    Ok(number) => number,

                    // Error:
                    Err(_) => return,
                };

                // Request:
                let request = String::from_utf8_lossy(&buffer[..number_bytes]);

                // Path:
                let path: &str = request.lines().next()
                    .and_then(|line| line.split_whitespace().nth(1))
                    .unwrap_or("/");

                // Logic:
                if path == "/api" {
                    // Variables (Assignment):
                    // Guard:
                    let guard = latest_clone.lock().await;

                    // Body:
                    let body = if let Some(update) = guard.as_ref() {
                        serde_json::to_vec(update).unwrap_or_else(|_| b"{}".to_vec())
                    } else {
                        b"{}".to_vec()
                    };

                    // Header:
                    let header: String = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: application/json\r\n\r\n",
                        body.len()
                    );

                    // Logic:
                    socket.write_all(header.as_bytes()).await;
                    socket.write_all(&body).await;
                } else {
                    match Socket::serve_file(path) {
                        // Success:
                        Ok((header, body)) => {
                            socket.write_all(header.as_bytes()).await;
                            socket.write_all(&body).await;
                        }

                        // Error:
                        Err(err) => eprintln!("File error: {}", err),
                    }
                }
            });
        }
    }
}
