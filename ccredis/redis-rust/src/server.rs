use std::{
    collections::HashMap,
    io::{self, BufReader, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use crate::{command::Command, resp::Resp};

pub struct Server {
    address: String,
}
impl Server {
    pub fn new(address: String) -> Self {
        Server { address }
    }
    pub fn start(
        &self,
    ) -> Result<(Receiver<(Command, SocketAddr)>, Sender<(SocketAddr, Resp)>), io::Error> {
        let listener = TcpListener::bind(&self.address)?;
        listener.set_nonblocking(true)?;
        let (sender, receiver) = mpsc::channel();
        let (c_sender, c_receiver) = mpsc::channel();
        let _parser_thread = thread::spawn(move || {
            let mut connections = HashMap::new();
            loop {
                let result = match listener.accept() {
                    Ok((stream, address)) => Some((stream, address)),
                    Err(err) => match err.kind() {
                        io::ErrorKind::WouldBlock => None,
                        _ => {
                            println!("{err}");
                            break;
                        }
                    },
                };
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
                                    sender.send((command, address.clone())).unwrap();
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
                while let Ok((address, response)) = c_receiver.try_recv() {
                    let serialized = Vec::from(response);
                    let stream = connections.get_mut(&address).unwrap();
                    if let Err(err) = stream.get_mut().write_all(&serialized) {
                        println!("{err}");
                    }
                }
            }
            println!("Thread is closed");
        });

        Ok((receiver, c_sender))
    }
}

//TODO check if data is incomplete and return incomplete data
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
