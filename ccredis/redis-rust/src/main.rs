use std::collections::HashMap;
use std::sync::mpsc;

use redis_rust::command::Command;
use redis_rust::dictionary::{self, Dictionary};
use redis_rust::resp::Resp;
use redis_rust::server::{Server, ServerThread};
use redis_rust::worker::Worker;

fn main() -> Result<(), std::io::Error> {
    let address = "127.0.0.1:6379";
    let mut server = Server::new(address, Worker::new(Dictionary::new()))?;
    let (sender, receiver) = mpsc::channel();
    server.start(receiver);
    Ok(())
}
