use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::{thread, time};

mod server;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        handle_connection(&mut stream);
    }
}

fn handle_connection(stream: &mut TcpStream) {
    let get = b"GET / HTTP/1.1\r\n";
    // let post = b"POST /send HTTP/1.1\r\n";


    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    if !buffer.starts_with(get) {
        stream.write_all("HTTP/1.1 404 NOT FOUND\r\n\r\n".as_bytes()).unwrap();
        stream.flush().unwrap();
        return
    }

    let first = fs::read_to_string("first.html").unwrap();

    stream.write_all("HTTP/1.1 200 OK\r\nContent-type: text/html; charset=utf-8\r\n\r\n".as_bytes()).unwrap();
    stream.write_all(first.as_bytes()).unwrap();
    stream.flush().unwrap();

    for i in 0..120 {
        thread::sleep(time::Duration::from_secs(1));
        stream.write_all(format!("<p>and a {}</p>", i.to_string()).as_bytes()).unwrap();
        stream.write_all(" ".as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    // thread::sleep(time::Duration::from_secs(5));
    stream.write_all(format!("<p>Gay {}</p>", "GAY".to_string()).as_bytes()).unwrap();

    stream.flush().unwrap();

}