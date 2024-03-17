#[derive(Debug, PartialEq, Eq)]
pub enum Resp {
    SimpleString(String),
    SimpleError(String),
    Integer(i64),
    BulkString(String),
    Array(Vec<Resp>),
    Null,
}
impl TryFrom<&[u8]> for Resp {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match parse_resp(value) {
            (Some(resp), r) => {
                if r.is_empty() {
                    Ok(resp)
                } else {
                    Err(())
                }
            }
            _ => Err(()),
        }
    }
}

fn parse_resp(value: &[u8]) -> (Option<Resp>, &[u8]) {
    if value.is_empty() {
        return (None, value);
    }
    let body = &value[1..];
    match value[0] {
        b'+' => parse_simple_string(body),
        b'-' => parse_simple_error(body),
        b':' => parse_integer(body),
        b'$' => parse_bulk_string(body),
        b'*' => parse_array(body),
        _ => (None, value),
    }
}
fn parse_simple_string(value: &[u8]) -> (Option<Resp>, &[u8]) {
    let pos = value.iter().position(|b| *b == b'\r');
    if pos.is_none() {
        return (None, value);
    }
    let pos = pos.unwrap();
    let (data, remaining) = value.split_at(pos);
    if remaining.len() < 2 || b"\r\n" != &remaining[..2] {
        return (None, value);
    }
    let text = String::from_utf8_lossy(data).to_string();
    (Some(Resp::SimpleString(text)), &remaining[2..])
}
fn parse_simple_error(value: &[u8]) -> (Option<Resp>, &[u8]) {
    match parse_simple_string(value) {
        (Some(Resp::SimpleString(s)), r) => (Some(Resp::SimpleError(s)), r),
        _ => (None, value),
    }
}

fn parse_integer(value: &[u8]) -> (Option<Resp>, &[u8]) {
    match parse_length(value) {
        (Some(i), r) => (Some(Resp::Integer(i)), r),
        _ => (None, value),
    }
}

fn parse_array(value: &[u8]) -> (Option<Resp>, &[u8]) {
    let (length, remaining) = parse_length(value);
    if length.is_none() {
        return (None, value);
    }
    if length.unwrap() == -1 {
        match parse_null(value) {
            (Some(null), r) => return (Some(null), r),
            _ => return (None, value),
        }
    }
    let length = length.unwrap() as usize;
    let mut array = Vec::with_capacity(length);
    let mut contents = remaining;
    loop {
        match parse_resp(contents) {
            (Some(resp), r) => {
                array.push(resp);
                contents = r;
            }
            (None, r) => {
                if r.is_empty() && array.len() == length {
                    return (Some(Resp::Array(array)), r);
                }
                return (None, r);
            }
        }
    }
}

fn parse_bulk_string(value: &[u8]) -> (Option<Resp>, &[u8]) {
    let (length, remaining) = parse_length(value);
    if length.is_none() {
        return (None, value);
    }
    if length.unwrap() == -1 {
        match parse_null(value) {
            (Some(null), r) => return (Some(null), r),
            _ => return (None, value),
        }
    }
    let length = length.unwrap() as usize;
    let (data, remaining) = remaining.split_at(length);
    let text = String::from_utf8_lossy(data).to_string();
    if text.len() != length || remaining.len() < 2 || b"\r\n" != &remaining[..2] {
        return (None, value);
    }
    (Some(Resp::BulkString(text)), &remaining[2..])
}

fn parse_length(value: &[u8]) -> (Option<i64>, &[u8]) {
    let pos = value.iter().position(|b| *b == b'\r');
    if pos.is_none() {
        return (None, value);
    }
    let pos = pos.unwrap();
    let (binary, remaining) = value.split_at(pos);
    let binary_str = match std::str::from_utf8(binary) {
        Ok(s) => s,
        Err(_) => return (None, value),
    };
    let length = match binary_str.parse() {
        Ok(l) => l,
        Err(_) => return (None, value),
    };
    if value.len() <= pos + 1 || value[pos + 1] != b'\n' {
        return (None, value);
    }
    (Some(length), &remaining[2..])
}

fn parse_null(value: &[u8]) -> (Option<Resp>, &[u8]) {
    if value.len() < 4 {
        return (None, value);
    }
    let null = &value[..4];
    match null {
        b"-1\r\n" => (Some(Resp::Null), &value[4..]),
        _ => (None, &value[4..]),
    }
}

mod tests {
    use super::*;

    #[test]
    fn parse_null() -> Result<(), &'static str> {
        let input = "$-1\r\n";
        match parse_resp(input.as_bytes()) {
            (Some(Resp::Null), r) => {
                assert!(r.is_empty());
                Ok(())
            }
            _ => Err("Should be null"),
        }
    }
    #[test]
    fn parse_array() -> Result<(), &'static str> {
        let input = "*1\r\n$4\r\nping\r\n";
        match parse_resp(input.as_bytes()) {
            (Some(Resp::Array(arr)), r) => {
                assert!(r.is_empty());
                assert_eq!(arr.len(), 1);
                match &arr[0] {
                    Resp::BulkString(s) => assert_eq!(s, "ping"),
                    _ => return Err("Array should contain a simple string"),
                }
                Ok(())
            }
            _ => Err("Should be of type array"),
        }
    }

    #[test]
    fn parse_array2() -> Result<(), &'static str> {
        let input = "*2\r\n$4\r\necho\r\n$11\r\nhello world\r\n";
        match parse_resp(input.as_bytes()) {
            (Some(Resp::Array(arr)), r) => {
                assert!(r.is_empty());
                assert_eq!(arr.len(), 2);
                match &arr[0] {
                    Resp::BulkString(s) => assert_eq!(s, "echo"),
                    _ => return Err("Array should contain a simple string"),
                }
                match &arr[1] {
                    Resp::BulkString(s) => assert_eq!(s, "hello world"),
                    _ => return Err("Array should contain a simple string"),
                }
                Ok(())
            }
            _ => Err("Should be of type array"),
        }
    }
    #[test]
    fn parse_array3() -> Result<(), &'static str> {
        let input = "*2\r\n$3\r\nget\r\n$3\r\nkey\r\n";
        match parse_resp(input.as_bytes()) {
            (Some(Resp::Array(arr)), r) => {
                assert!(r.is_empty());
                assert_eq!(arr.len(), 2);
                match &arr[0] {
                    Resp::BulkString(s) => assert_eq!(s, "get"),
                    _ => return Err("Array should contain a simple string"),
                }
                match &arr[1] {
                    Resp::BulkString(s) => assert_eq!(s, "key"),
                    _ => return Err("Array should contain a simple string"),
                }
                Ok(())
            }
            _ => Err("Should be of type array"),
        }
    }
    #[test]
    fn parse_simple_string() -> Result<(), &'static str> {
        let input = "+OK\r\n";
        match parse_resp(input.as_bytes()) {
            (Some(Resp::SimpleString(s)), r) => {
                assert_eq!(s, "OK");
                Ok(())
            }
            _ => Err("Should be of type simple string"),
        }
    }
    #[test]
    fn parse_simple_error() -> Result<(), &'static str> {
        let input = "-ERROR message\r\n";
        match parse_resp(input.as_bytes()) {
            (Some(Resp::SimpleError(s)), r) => {
                assert_eq!(s, "ERROR message");
                Ok(())
            }
            _ => Err("Should be of type simple error"),
        }
    }

    #[test]
    fn parse_empty_bulk_string() -> Result<(), &'static str> {
        let input = "$0\r\n\r\n";
        match parse_resp(input.as_bytes()) {
            (Some(Resp::BulkString(s)), r) => {
                assert_eq!(s, "");
                Ok(())
            }
            _ => Err("Should be of type bulk string"),
        }
    }
    #[test]
    fn parse_simple_string2() -> Result<(), &'static str> {
        let input = "+hello world\r\n";
        match parse_resp(input.as_bytes()) {
            (Some(Resp::SimpleString(s)), r) => {
                assert_eq!(s, "hello world");
                Ok(())
            }
            _ => Err("Should be of type simple string"),
        }
    }
}
