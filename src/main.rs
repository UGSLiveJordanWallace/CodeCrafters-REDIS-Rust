use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let stream = listener.accept().await;

        match stream {
            Ok((mut stream, _)) => {
                tokio::spawn(async move {
                    let mut buf = [0; 1024];
                    loop {
                        let read_buf_count = stream.read(&mut buf).await.unwrap();
                        if read_buf_count == 0 {
                            break;
                        }
                        stream.write(b"+PONG\r\n").await.unwrap();
                    }
                });
            },
            Err (e) => {
                println!("error: {e}");
            }
        }
    }
}
