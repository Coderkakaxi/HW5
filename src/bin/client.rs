use mini_redis::Frame;
use tokio::net::TcpStream;
use tokio::stream::StreamExt;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6379";
    let mut socket = TcpStream::connect(&addr).await.unwrap();

    let (mut rx, _) = broadcast::channel(1);

    tokio::spawn(async move {
        while let Some(frame) = rx.recv().await.unwrap() {
            println!("Received: {:?}", frame);
        }
    });

    loop {
        let mut input = String::new();
        tokio::io::stdin().read_line(&mut input).await.unwrap();

        let command: Vec<&str> = input.trim().split_whitespace().collect();

        let frame = match command[0] {
            "GET" if command.len() == 2 => {
                vec![
                    Frame::BulkString(command[0].to_string().into_bytes()),
                    Frame::BulkString(command[1].to_string().into_bytes()),
                ]
            }
            "SET" if command.len() == 3 => {
                vec![
                    Frame::BulkString(command[0].to_string().into_bytes()),
                    Frame::BulkString(command[1].to_string().into_bytes()),
                    Frame::BulkString(command[2].to_string().into_bytes()),
                ]
            }
            _ => vec![Frame::Error("unimplemented".to_string())],
        };
        for frame in frame {
            mini_redis::Frame::write(&frame, &mut socket).await.unwrap();
        }

        if let Ok(response) = mini_redis::Frame::read(&mut socket).await {
            tx.send(response.clone()).unwrap();
        } else {
            break;
        }
    }
}