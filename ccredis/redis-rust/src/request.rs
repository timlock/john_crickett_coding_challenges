use crate::resp::Resp;

#[derive(Debug)]
pub enum Request {
    Ping,
    Echo(String),
    Get(String),
    Set(String),
}
impl TryFrom<Resp> for Request {
    type Error = &'static str;

    fn try_from(value: Resp) -> Result<Self, Self::Error> {
        match value {
            Resp::Array(arr) => match &arr[0] {
                Resp::BulkString(s) => match s.as_str() {
                    "PING" => Ok(Request::Ping),
                    _ => Err("Unkown command"),
                },
                _ => Err("First argument should be of type bulkstring"),
            },
            // Resp::Array(arr) => {
            //     match &arr[0]{
            //         Resp::
            //     }
            // },
            _ => Err("Should be of type  array"),
        }
    }
}
