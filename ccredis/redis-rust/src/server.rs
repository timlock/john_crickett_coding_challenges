use std::{
    collections::HashMap,
    io::{self, BufReader, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

use crate::{command::Command, resp::Resp};

pub struct Server {
    address: String,
}
impl Server {
    pub fn new(address: String) -> Self {
        Server { address }
    }

    pub fn handle<F>(&self, mut callback: F) -> Result<(), io::Error>
    where
        F: FnMut(Command) -> Resp,
    {
        let listener = TcpListener::bind(&self.address)?;
        listener.set_nonblocking(true)?;
        let mut connections = HashMap::new();
        loop {
            let result = try_accept(&listener);
            if let Some((stream, address)) = result {
                println!("New Connection {address} total {}", connections.len());
                stream.set_nonblocking(true).unwrap();
                connections.insert(address, BufReader::new(stream));
            }

            let mut disconnected = Vec::new();
            for (address, stream) in connections.iter_mut() {
                match try_read(stream) {
                    Ok(bytes) => match parse(&bytes) {
                        Ok(commands) => {
                            for command in commands {
                                let response = callback(command);
                                let serialized = Vec::from(response);
                                if let Err(err) = stream.get_mut().write_all(&serialized) {
                                    println!("{err}");
                                }
                            }
                        }
                        Err(err) => {
                            disconnected.push(address.clone());
                            println!("{err}");
                        }
                    },
                    Err(err) => {
                        disconnected.push(address.clone());
                        println!("{err}");
                    }
                }
            }
            for address in disconnected {
                connections.remove(&address);
            }
        }
        Ok(())
    }
}
fn try_accept(listener: &TcpListener) -> Option<(TcpStream, SocketAddr)> {
    match listener.accept() {
        Ok((stream, address)) => Some((stream, address)),
        Err(err) => match err.kind() {
            io::ErrorKind::WouldBlock => None,
            _ => {
                println!("{err}");
                None
            }
        },
    }
}

//TODO check if data is incomplete buffer incomplete data and discard corrupted data
fn parse(data: &[u8]) -> Result<Vec<Command>, Resp> {
    let resps = Resp::parse(data)?;
    let mut commands = Vec::new();
    for resp in resps {
        let command = Command::try_from(resp)?;
        commands.push(command);
    }
    Ok(commands)
}
fn try_read(buf_reader: &mut BufReader<TcpStream>) -> Result<Vec<u8>, io::Error> {
    let mut buffer = Vec::new();
    loop {
        let mut buf = [0; 1024];
        let size = match buf_reader.read(&mut buf) {
            Ok(size) => {
                if size == 0 {
                    return Err(io::Error::new(
                        io::ErrorKind::ConnectionAborted,
                        "Connection closed",
                    ));
                }
                size
            }
            Err(err) => match err.kind() {
                io::ErrorKind::WouldBlock => return Ok(buffer),
                _ => return Err(err),
            },
        };
        buffer.extend_from_slice(&buf[..size]);
        if size < 1024 {
            break;
        }
    }
    Ok(buffer)
}
