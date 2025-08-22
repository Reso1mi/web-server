use std::fs;
use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let addr = "127.0.0.1:6789";
    // str实现了ToSocketAddrs
    let Ok(listener) = TcpListener::bind("127.0.0.1:6789") else {
        panic!("bind {addr} error")
    };

    println!("webserver is running on {}", addr);
    for stream_ret in listener.incoming() {
        match stream_ret {
            Ok(stream) => match handle_conn(stream) {
                Err(e) => eprintln!("handle tcp stream error: {e}"),
                _ => {}
            },
            Err(e) => eprintln!("accept tcp stream error: {e}"),
        }
    }
}

fn handle_conn(mut stream: TcpStream) -> Result<(), Error> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let get = b"GET / HTTP/1.1\r\n";
    let (http_status_line, resource_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "./static/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "./static/404.html")
    };

    let contents = fs::read_to_string(resource_name)?;
    let response = format!("{http_status_line}{contents}");
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}
