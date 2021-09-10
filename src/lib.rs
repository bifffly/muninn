use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum ReqType {
    PUSH,
    PULL,
    ERR,
}

pub fn read_request(mut stream: &TcpStream) -> String {
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

pub fn get_rtype(reqstr: &String) -> ReqType {
    let rsplit = reqstr.split("\t").collect::<Vec<&str>>();
    if rsplit.len() != 3 {
        return ReqType::ERR;
    }
    let rtype = match rsplit[0] {
        "odin" => match rsplit[1] {
            "push" => ReqType::PUSH,
            "pull" => ReqType::PULL,
            _ => ReqType::ERR,
        },
        _ => ReqType::ERR,
    };
    return rtype;
}

pub fn get_filepath(reqstr: &String, homedir: &String) -> String {
    let rsplit = reqstr.split("\t").collect::<Vec<&str>>();
    let mut path = rsplit[2].to_string();
    if !path.starts_with("/") {
        path = "/".to_owned() + &path;
    }
    return homedir.to_owned() + &path;
}

pub fn pull(path: &String) -> String {
    let response;
    if Path::new(path).is_file() {
        let content = fs::read_to_string(path).unwrap();
        response = format!("odin\tA\r\n{}", content);
    }
    else {
        response = "odin\tB\r\nFile not found".to_string();
    }
    return response;

    // stream.write(response.as_bytes()).unwrap();
    // stream.flush().unwrap();
}

pub fn push() -> String {
    return error_c(ReqType::PUSH);
}

pub fn error_c(rtype: ReqType) -> String {
    let response;
    match rtype {
        ReqType::PUSH => {
            response = "odin\tC\r\nRequest method \'push\' unsupported";
        }
        _ => {
            response = "odin\tC\r\n";
        }
    }
    return response.to_string(); 

    // stream.write(response.as_bytes()).unwrap();
    // stream.flush().unwrap();
}

pub fn error_d() -> String {
    let response = "odin\tD\r\nMalformed request";
    return response.to_string();

    // stream.write(response.as_bytes()).unwrap();
    // stream.flush().unwrap();
}
