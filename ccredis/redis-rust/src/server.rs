use std::{
    collections::HashMap,
    io::{self, BufReader, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

use crate::{command::Command, resp::Resp};

pub struct Server {
    listener: TcpListener,
    connections: HashMap<SocketAddr, BufReader<TcpStream>>,
}
impl Server {
    pub fn new(address: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(address)?;
        listener.set_nonblocking(true)?;
        Ok(Server {
            listener,
            connections: HashMap::new(),
        })
    }

    pub fn handle<F>(&mut self, mut callback: F)
    where
        F: FnMut(Command) -> Resp,
    {
        loop {
            let result = try_accept(&self.listener);
            if let Some((stream, address)) = result {
                stream.set_nonblocking(true).unwrap();
                self.connections.insert(address, BufReader::new(stream));
            }
            let mut disconnected = Vec::new();
            for (address, stream) in self.connections.iter_mut() {
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
                self.connections.remove(&address);
            }
        }
    }
}
fn try_accept(listener: &TcpListener) -> Option<(TcpStream, SocketAddr)> {
    listener
        .accept()
        .map_err(|err| {
            if err.kind() != io::ErrorKind::WouldBlock {
                println!("{err}")
            }
        })
        .ok()
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

fn try_read(buf_reader: &mut BufReader<TcpStream>) -> io::Result<Vec<u8>> {
    const CHUNK_SIZE: usize = 1028;
    let mut buffer = Vec::with_capacity(CHUNK_SIZE);
    let mut buf = [0; CHUNK_SIZE];
    loop {
        match buf_reader.read(&mut buf) {
            Ok(size) if size == 0 => {
                return Err(io::Error::new(
                    io::ErrorKind::ConnectionAborted,
                    "Connection closed",
                ))
            }
            Ok(size) => {
                buffer.extend_from_slice(&buf[..size]);
                if size < CHUNK_SIZE {
                    break;
                }
            }
            Err(err) if err.kind() == io::ErrorKind::WouldBlock => break,
            Err(err) => return Err(err),
        }
    }
    Ok(buffer)
}
