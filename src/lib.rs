use mini_redis::{Frame, Result};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn filter(server_addr: &str, mut socket: TcpStream) -> Result<()> {
    let mut buf = vec![0; 512];
    let bytes_read = socket.read(&mut buf).await?;
    


    let server_socket = TcpStream::connect(server_addr).await?;
    server_socket.write_all(&buf[..bytes_read]).await?;
    server_socket.flush().await?;

    let mut response_buf = vec![0; 512];
    let response_bytes_read = server_socket.read(&mut response_buf).await?;
    socket.write_all(&response_buf[..response_bytes_read]).await?;
    socket.flush().await?;
    
    Ok(())
}