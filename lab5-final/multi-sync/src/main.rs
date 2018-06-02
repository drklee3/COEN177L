use std::net::TcpListener;
use std::io::Write;
use std::thread;

static RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\n\
    Content-Type: text/html; charset=UTF-8\r\n\r\n\
    Hello world\r\n";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening for connections on port {}", 8080);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    let _ = stream.write(&RESPONSE);
                });
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
