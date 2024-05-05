use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

use redis_rust::command::Command;
use redis_rust::resp::Resp;
use redis_rust::server::Server;

fn main() -> Result<(), std::io::Error> {
    let address = String::from("127.0.0.1:6379");
    let server = Server::new(address, 50);
    let mut dictionary: HashMap<String, String> = HashMap::new();
    let mut dictionary = Arc::new(RwLock::new(dictionary));
    server.handle(move |command| match command {
        Command::Ping => Resp::SimpleString("PONG".to_string()),
        Command::Echo(s) => Resp::BulkString(s),
        Command::Get(key) => match dictionary.read().unwrap().get(&key) {
            Some(value) => Resp::BulkString(value.to_string()),
            None => Resp::Null,
        },
        Command::Set { key, value } => {
            dictionary.write().unwrap().insert(key, value);
            Resp::ok()
        }
        Command::ConfigGet => Resp::Integer(0),
    })
}
