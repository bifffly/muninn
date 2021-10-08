use serde::Deserialize;
use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;

#[derive(Deserialize)]
pub struct Config {
    pub network: NetworkConfig,
    pub server: ServerConfig
}

#[derive(Deserialize)]
pub struct NetworkConfig {
    pub ipaddr: String,
    pub port: String
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub homedir: String
}

pub fn parse_config(confpath: &str) -> Config {
    let confcontent = fs::read_to_string(confpath).unwrap();
    return toml::from_str(&confcontent).unwrap();
}

#[derive(Debug, PartialEq)]
pub enum ReqType {
    PREFLIGHT,
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
            "preflight" => ReqType::PREFLIGHT,
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

pub fn preflight(path: &String) -> String {
    let res;
    if Path::new(path).is_file() {
        let len = fs::read(path).unwrap().len();
        res = format!("odin\tA\t{}\r\n", len);
    }
    else {
        res = "odin\tB\r\n".to_string();
    }
    return res;
}

pub fn pull(path: &String) -> String {
    let res;
    if Path::new(path).is_file() {
        let content = fs::read_to_string(path).unwrap();
        res = format!("odin\tA\r\n{}", content);
    }
    else {
        res = "odin\tB\r\n".to_string();
    }
    return res;

    // stream.write(response.as_bytes()).unwrap();
    // stream.flush().unwrap();
}

pub fn error_c() -> String {
    let res = "odin\tC\r\n";
    return res.to_string();

    // stream.write(response.as_bytes()).unwrap();
    // stream.flush().unwrap();
}
