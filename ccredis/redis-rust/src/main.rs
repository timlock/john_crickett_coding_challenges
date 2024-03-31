use redis_rust::command::Command;
use redis_rust::resp::Resp;
use redis_rust::server::Server;

fn main() -> Result<(), std::io::Error> {
    let address = String::from("127.0.0.1:6379");
    let server = Server::new(address);
    server.serve(|command| {
        println!("Received {command:?}");
        match command {
            Command::Ping => Resp::SimpleString("PONG".to_string()),
            Command::Echo(s) => Resp::SimpleString(s),
            Command::Get(_) => todo!(),
            Command::Set(_) => todo!(),
        }
    })
}
