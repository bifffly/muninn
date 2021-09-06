use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::Path;

enum ReqType {
    PUSH,
    PULL,
    ERR,
}

fn main() {
    let listener = TcpListener::bind("172.31.86.4:1866").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let (rpath, rtype) = handle_request(&stream);
        match rtype {
            ReqType::PULL => {
                let path = rpath_to_filepath(rpath);
                println!("Path: {}", path);
                pull(stream, path);
            },
            ReqType::PUSH => {
                push(stream);
            },
            _ => {
                send_error_c(stream, rtype);
            },
        }
    }
}

fn handle_request(mut stream: &TcpStream) -> (String, ReqType) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let mut v = vec![];
    for byte in buffer {
        match byte {
            0 => break,
            _ => v.push(byte),
        }
    }
    let req = String::from_utf8_lossy(&v[..]).to_string();
    let rsplit = req.split("\t").collect::<Vec<&str>>();
    let (rpath, rtype);
    if rsplit.len() != 3 && rsplit[0].ne("rydja1") {
        rpath = "".to_string();
        rtype = ReqType::ERR;
    }
    else {
        rpath = rsplit[2].to_string();
        rtype = match rsplit[1] {
            "push" => ReqType::PUSH,
            "pull" => ReqType::PULL,
            _ => ReqType::ERR,
        };
    }
    return (rpath, rtype);
}

fn rpath_to_filepath(mut req: String) -> String {
    let home = "example".to_string();
    if !req.starts_with("/") {
        req = "/".to_string() + &req;
    }
    return home + &req;
}

fn pull(mut stream: TcpStream, path: String) {
    let response;
    if Path::new(&path).is_file() {
        let content = fs::read_to_string(path).unwrap();
        response = format!("rydja1\tA\r\n{}", content);
    }
    else {
        response = "rydja1\tB\r\n".to_string();
    }
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn push(stream: TcpStream) {
    send_error_c(stream, ReqType::PUSH);
}

fn send_error_c(mut stream: TcpStream, rtype: ReqType) {
    let response;
    match rtype {
        ReqType::PUSH => {
            response = "rydja1\tC\r\npush unsupported";
        }
        _ => {
            response = "rydja1\tC\r\n";
        }
    }
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
