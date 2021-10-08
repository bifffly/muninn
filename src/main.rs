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
        println!("{}", reqstr);
        let rtype = get_rtype(&reqstr);
        let response: String = match rtype {
            ReqType::PREFLIGHT => {
                let path = get_filepath(&reqstr, &homedir);
                preflight(&path)
            },
            ReqType::PULL => {
                let path = get_filepath(&reqstr, &homedir);
                pull(&path)
            },
            _ => error_c(),
        };
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

