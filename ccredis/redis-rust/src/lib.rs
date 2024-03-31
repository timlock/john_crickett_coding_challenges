use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

use resp::Resp;

mod resp;

pub fn serve() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(s) => s,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };

        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut buffer = Vec::new();
    loop {
        let mut buf = [0; 1024];
        let size = match buf_reader.read(&mut buf) {
            Ok(size) => size,
            Err(err) => {
                println!("{err}");
                panic!()
            }
        };
        println!("Read {size} bytes");
        buffer.extend_from_slice(&buf[..size]);
        if size < 1024 {
            break;
        }
    }
    let resp = match Resp::try_from(buffer.as_slice()) {
        Ok(resp) => resp,
        Err(err) => panic!("Could not parse resp"),
    };
    let response = handle_resp(resp);
    stream.write_all(response.into());
    // println!("Received command {resp}");
}
fn handle_resp(resp: Resp) -> Resp{

}
