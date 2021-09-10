use std::io::prelude::*;
use std::net::TcpListener;

use muninn::*;

fn main() {
    let conf: Config = parse_config("config/odin.toml");
    let homedir = conf.server.homedir;
    let ipaddr = conf.network.ipaddr;
    let port = conf.network.port;
    let connection = ipaddr + ":" + &port;

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

