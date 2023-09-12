use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::stream::StreamExt;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6379";
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: {}", addr);

    let db = Arc::new(Mutex::new(HashMap::new()));
    let (tx, _) = broadcast::channel::<Frame>();

    while let Ok((socket, _)) = listener.accept().await {
        let db = db.clone();
        let tx = tx.clone();

        tokio::spawn(async move {
            process(socket, db, tx).await;
        });
    }
}

async fn process(socket: TcpStream, db: Arc<Mutex<HashMap<String, String>>>, tx: broadcast::Sender<Frame>) {
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        match frame {
            Frame::Array(frames) if frames.len() >= 2 => {
                let response = match (&frames[0], &frames[1]) {
                    (&Frame::BulkString(ref command), &Frame::BulkString(ref key)) => {
                        let mut db = db.lock().unwrap();
                        match command.as_ref() {
                            b"GET" => {
                                if let Some(value) = db.get(key) {
                                    Frame::BulkString(value.clone())
                                } else {
                                    Frame::Null
                                }
                            }
                            b"SET" if frames.len() >= 3 => {
                                let value = frames[2].clone();
                                db.insert(key.clone(), value);
                                Frame::SimpleString("OK".to_string())
                            }
                            _ => Frame::Error("unimplemented".to_string())
                        }
                    }
                    _ => Frame::Error("unimplemented".to_string())
                };
                tx.send(response.clone()).unwrap();
                connection.write_frame(&response).await.unwrap();
            }
            _ => {}
        }
    }
}