use std::{
    env, fs, io::{self, Read},
};

fn main() {
    let (option, path) = match parse_arguments(env::args()) {
        Ok(a) => a,
        Err(err) => {
            println!("{err}");
            return;
        }
    };
    let text = match path {
        Some(path) => match fs::read_to_string(&path) {
            Ok(file) => file,
            Err(err) => {
                println!("{err}");
                return;
            }
        },
        None => {
            let mut buffer = Vec::new();
            if let Err(err) = io::stdin().read_to_end(&mut buffer) {
                println!("{err}");
                return;
            }
            String::from_utf8_lossy(&buffer).to_string()
        }
    };
    let result = match option {
        Some(option) => {
            let result = match option {
                Select::Bytes => text.len(),
                Select::Lines => text.lines().count(),
                Select::Words => text.split_ascii_whitespace().count(),
                Select::Characters => text.chars().count(),
            };
            result.to_string()
        }
        None => {
            let lines = text.lines().count().to_string();
            let words = text.split_ascii_whitespace().count().to_string();
            let bytes = text.len().to_string();
            [lines, words, bytes].join(" ")
        }
    };
    println!("{result} ");
}

fn parse_arguments(mut args: env::Args) -> Result<(Option<Select>, Option<String>), &'static str> {
    args.next();
    let mut option = None;
    let mut path = None;
    let first = match args.next() {
        Some(option) => option,
        None => {
            println!("No arguments provided");
            return Ok((None, None));
        }
    };
    if first.contains('-') {
        option = match Select::try_from(first.as_ref()) {
            Ok(option) => Some(option),
            Err(err) => {
                return Err(err);
            }
        };
    } else {
        path = Some(first);
    }
    if let Some(p) = args.next() {
        path = Some(p);
    }
    Ok((option, path))
}

#[derive(Debug)]
enum Select {
    Bytes,
    Lines,
    Words,
    Characters,
}
impl TryFrom<&str> for Select {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        for c in value.chars() {
            match c {
                'c' => return Ok(Select::Bytes),
                'l' => return Ok(Select::Lines),
                'w' => return Ok(Select::Words),
                'm' => return Ok(Select::Characters),
                _ => {}
            };
        }
        return Err("Value contains no known selector");
    }
}
