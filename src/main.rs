use std::net::TcpListener;
use rydja_server::*;

fn main() {
    let listener = TcpListener::bind("172.31.86.4:1866").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let reqstr = read_request(&stream);
        let rtype = get_rtype(&reqstr);
        match rtype {
            ReqType::PULL => {
                let path = get_filepath(&reqstr);
                println!("Path: {}", path);
                pull(stream, path);
            },
            ReqType::PUSH => {
                push(stream);
            },
            _ => {
                send_error_d(stream);
            },
        }
    }
}

/*
fn read_request(mut stream: &TcpStream) -> String {
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

fn get_rtype(reqstr: &String) -> ReqType {
    let rsplit = reqstr.split("\t").collect::<Vec<&str>>();
    let rtype = match rsplit[0] {
        "rydja1" => match rsplit[1] {
            "push" => ReqType::PUSH,
            "pull" => ReqType::PULL,
            _ => ReqType::ERR,
        },
        _ => ReqType::ERR,
    };
    return rtype;
}

fn get_filepath(reqstr: &String) -> String {
    let rsplit = reqstr.split("\t").collect::<Vec<&str>>();
    let home = "example".to_string();
    if !rsplit[2].starts_with("/") {
        rsplit[2].to_string().push_str("/");
    }
    return home + &rsplit[2];
}

fn pull(mut stream: TcpStream, path: String) {
    let response;
    if Path::new(&path).is_file() {
        let content = fs::read_to_string(path).unwrap();
        response = format!("rydja1\tA\r\n{}", content);
    }
    else {
        response = "rydja1\tB\r\nFile not found".to_string();
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
            response = "rydja1\tC\r\nRequest method \'push\' unsupported";
        }
        _ => {
            response = "rydja1\tC\r\n";
        }
    }
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn send_error_d(mut stream: TcpStream) {
    let response = "rydja1\tD\r\nInvalid request";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
*/
