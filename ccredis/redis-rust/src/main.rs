use redis_rust::request::Request;
use redis_rust::resp::Resp;
use redis_rust::server::Server;

fn main() {
    let address = String::from("127.0.0.1:6379");
    let server = Server::new(address);
    server.serve(|command| {
        println!("Received {command:?}");
        match command {
            Request::Ping => Resp::SimpleString("PONG".to_string()),
            _ => {
                println!("should not be reached");
                Resp::SimpleError("Unkown command".to_string())
            }
        }
    })
}
