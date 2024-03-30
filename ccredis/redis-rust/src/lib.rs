use std::{io::BufReader, net::TcpListener};

mod serialize;

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
        let buf_reader = BufReader::new(&mut stream);
        println!("Connection established!");
    }
}
