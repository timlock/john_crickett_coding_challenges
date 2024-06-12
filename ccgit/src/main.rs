use std::{env, fs, io, path};

use ccgit::HeadFile;

fn main() {
    let mut args = env::args();
    if args.len() < 2 {
        return;
    }
    let command = args.skip(1).next().unwrap();
    match command.as_str() {
        "init" => match args.next() {
            Some(path) => init(&path).unwrap(),
            None => init("unnamd repository").unwrap(),
        },
        _ => println!("Unkown command {command}"),
    }
}

fn init(path: &str) -> io::Result<()> {
    let mut path = path::PathBuf::from(path);
    fs::create_dir(".git")?;
    HeadFile::new(path);
    println!("Initialized empty Git repository in {path}");
    Ok(())
}
