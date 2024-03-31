use std::{
    error::Error,
    io::{BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::{request::Request, resp::Resp};

pub struct Server {
    address: String,
}
impl Server {
    pub fn new(address: String) -> Self {
        Server { address }
    }
    pub fn serve<F>(&self, callback: F)
    where
        F: Fn(Request) -> Resp,
    {
        let listener = TcpListener::bind(&self.address).unwrap();
        for stream in listener.incoming() {
            let mut stream = match stream {
                Ok(s) => s,
                Err(e) => {
                    println!("{e}");
                    continue;
                }
            };
            println!("Connection established!");
            let resp = match self.read_resp(&mut stream) {
                Ok(resp) => resp,
                Err(err) => {
                    println!("{err}");
                    continue;
                }
            };
            println!("Received: {resp}");
            let request = match Request::try_from(resp) {
                Ok(r) => r,
                Err(err) => {
                    println!("{err}");
                    continue;
                }
            };
            let response = callback(request);
            println!("Respond with: {response}");
            let serialized = Vec::from(response);
            stream
                .write_all(&serialized)
                .map_err(|err| println!("{err}"));
        }
    }
    fn read_resp(&self, stream: &mut TcpStream) -> Result<Resp, &'static str> {
        let mut buf_reader = BufReader::new(stream);
        let mut buffer = Vec::new();
        loop {
            let mut buf = [0; 1024];
            let size = match buf_reader.read(&mut buf) {
                Ok(size) => size,
                Err(_) => return Err("Encountered error while reading"),
            };
            println!("Read {size} bytes");
            buffer.extend_from_slice(&buf[..size]);
            if size < 1024 {
                break;
            }
        }
        match Resp::try_from(buffer.as_slice()) {
            Ok(resp) => Ok(resp),
            Err(_) => Err("Could not parse resp"),
        }
    }
}
