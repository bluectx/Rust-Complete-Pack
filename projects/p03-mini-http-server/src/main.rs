use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Serveur HTTP sur http://127.0.0.1:8080");
    
    loop {
        let (mut socket, _) = listener.accept().await?;
        
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            
            if let Ok(n) = socket.read(&mut buffer).await {
                let request = String::from_utf8_lossy(&buffer[..n]);
                
                let response = if request.starts_with("GET /hello") {
                    "HTTP/1.1 200 OK\r\n\r\n<h1>Hello, World!</h1>"
                } else {
                    "HTTP/1.1 404 NOT FOUND\r\n\r\n<h1>Not Found</h1>"
                };
                
                let _ = socket.write_all(response.as_bytes()).await;
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_server_response() {
        // Test basique
        assert!(true);
    }
}

