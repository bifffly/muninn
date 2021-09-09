use std::io::prelude::*;
use std::net::TcpListener;
use rydja_server::*;

fn main() {
    let homedir = "example".to_string();
    let connection = "172.31.86.4:1866";

    let listener = TcpListener::bind(connection).unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let reqstr = read_request(&stream);
        let rtype = get_rtype(&reqstr);
        let response: String = match rtype {
            ReqType::PULL => {
                let path = get_filepath(&reqstr, &homedir);
                pull(&path)
            },
            ReqType::PUSH => push(),
            _ => error_d(),
        };
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

