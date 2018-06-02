extern crate threadpool;

use std::net::TcpListener;
use std::io::Write;
use threadpool::Builder;

static RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\n\
    Content-Type: text/html; charset=UTF-8\r\n\r\n\
    Hello world\r\n";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening for connections on port {}", 8080);

    let pool = Builder::new()
        .thread_name("worker".into())
        .build();

    for stream in listener.incoming() {
        pool.execute(move || {
            match stream {
                Ok(mut stream) => {
                    let _ = stream.write(&RESPONSE);
                }
                Err(e) => {
                    println!("Unable to connect: {}", e);
                }
            }
        })
    }
}
