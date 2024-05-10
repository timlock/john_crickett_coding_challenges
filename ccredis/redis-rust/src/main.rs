use std::collections::HashMap;

use redis_rust::command::Command;
use redis_rust::resp::Resp;
use redis_rust::server::Server;

fn main() -> Result<(), std::io::Error> {
    let address = String::from("127.0.0.1:6379");
    let server = Server::new(address);
    let mut dictionary: HashMap<String, String> = HashMap::new();
    let (receiver, threads) = server.start()?;
    while let Ok((id, command, address)) = receiver.recv() {
        let response = match command {
            Command::Ping => Resp::SimpleString("PONG".to_string()),
            Command::Echo(s) => Resp::BulkString(s),
            Command::Get(key) => match dictionary.get(&key) {
                Some(value) => Resp::BulkString(value.to_string()),
                None => Resp::Null,
            },
            Command::Set { key, value } => {
                dictionary.insert(key, value);
                Resp::ok()
            }
            Command::ConfigGet => Resp::Integer(0),
        };
        threads.get(&id).unwrap().send((address, response)).unwrap();
    }
    Ok(())
}
