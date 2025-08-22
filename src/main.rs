use std::fs;
use std::io::Error;
use std::time::Duration;

use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::task;
use futures::AsyncReadExt;
use futures::AsyncWriteExt;
use futures::StreamExt;

#[async_std::main]
async fn main() {
    let addr = "127.0.0.1:6789";
    // str 实现了ToSocketAddrs
    let Ok(listener) = TcpListener::bind("127.0.0.1:6789").await else {
        panic!("bind {addr} error")
    };

    println!("webserver is running on {}", addr);

    listener
        .incoming()
        .for_each_concurrent(None, |tcp_stream| async {
            match tcp_stream {
                Ok(tcp_stream) => match handle_conn(tcp_stream).await {
                    Err(e) => eprintln!("handle tcp stream error: {e}"),
                    _ => {}
                },
                Err(e) => eprintln!("accept tcp stream error: {e}"),
            }
        })
        .await;
}

async fn handle_conn(mut stream: TcpStream) -> Result<(), Error> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await?;

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (http_status_line, resource_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "src/static/index.html")
    } else if buffer.starts_with(sleep) {
        // 让出控制权，非阻塞线程的睡眠
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "src/static/404.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "src/static/404.html")
    };

    let contents = fs::read_to_string(resource_name)?;
    let response = format!("{http_status_line}{contents}");
    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;
    Ok(())
}
