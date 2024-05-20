
use crate::{
    command::Command, dictonary::{self, Dictonary}, resp::Resp
};

pub struct Worker {
    dictionary: Dictonary<String>,
}

impl Worker {
    pub fn new(dictonary: Dictonary<String>) -> Self {
        Self {
            dictionary: dictonary,
        }
    }
    pub fn handle_command(&mut self, command: Command) -> Resp {
        match command {
            Command::Ping => Resp::SimpleString("PONG".to_string()),
            Command::Echo(s) => Resp::BulkString(s),
            Command::Get(key) => match self.dictionary.get(&key) {
                Some(value) => Resp::BulkString(value.to_string()),
                None => Resp::Null,
            },
            Command::Set { key, value } => {
                self.dictionary.insert(key, value);
                Resp::ok()
            }
            Command::ConfigGet => Resp::Integer(0),
        }
    }
}
