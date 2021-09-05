use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1866").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let req = handle_request(&stream);
        let path = request_to_path(req);
        send_response(stream, path);
    }
}

fn handle_request(mut stream: &TcpStream) -> String {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let mut v = vec![];
    for byte in buffer {
        match byte {
            0 => break,
            _ => v.push(byte),
        }
    }
    return String::from_utf8_lossy(&v[..]).to_string();
}

fn request_to_path(req: String) -> String {
    let home = "example".to_string();
    return home + &req;
}

fn send_response(mut stream: TcpStream, path: String) {
    let response;
    let content = fs::read_to_string(path).unwrap();
    response = format!("rydja1\tA\r\n{}", content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
