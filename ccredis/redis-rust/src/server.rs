use std::{
    io::{self, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::{command::Command, resp::Resp};

pub struct Server {
    address: String,
}
impl Server {
    pub fn new(address: String) -> Self {
        Server { address }
    }
    pub fn serve<F>(&self, callback: F) -> Result<(), io::Error>
    where
        F: Fn(Command) -> Resp,
    {
        let listener = TcpListener::bind(&self.address)?;
        for stream in listener.incoming() {
            let mut stream = match stream {
                Ok(s) => s,
                Err(e) => {
                    println!("{e}");
                    continue;
                }
            };
            match parse_command(&mut stream) {
                Ok(command) => {
                    println!("Received {command:?}");
                    let response = callback(command);
                    println!("Send {:?}", response.to_string());
                    let serialized = Vec::from(response);
                    println!("Serialized {serialized:?}");
                    if let Err(err) = stream.write_all(&serialized) {
                        println!("{err}");
                    }
                }
                Err(err) => println!("{err}"),
            };
        }
        Ok(())
    }
}

fn parse_command(stream: &mut TcpStream) -> Result<Command, String> {
    let received_bytes = read_all(stream).map_err(|_| "Failed to read byte from tcp stream")?;
    let resp = Resp::try_from(received_bytes.as_slice()).map_err(|_| "Could not parse resp")?;
    Command::try_from(resp).map_err(|err| err.to_string())
}
fn read_all(stream: &mut TcpStream) -> Result<Vec<u8>, io::Error> {
    let mut buf_reader = BufReader::new(stream);
    let mut buffer = Vec::new();
    loop {
        let mut buf = [0; 1024];
        let size = buf_reader.read(&mut buf)?;
        buffer.extend_from_slice(&buf[..size]);
        if size < 1024 {
            break;
        }
    }
    Ok(buffer)
}
