use cccompression::{decode, encode};
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut e = false;
    let mut d = false;
    let mut src = None;
    let mut dst = None;
    for arg in env::args().skip(1) {
        if !e && !d {
            if arg == "-e" {
                e = true;
            } else if arg == "-d" {
                d = true;
            } else {
                println!("Unkown argument {arg}");
                return;
            }
        } else if e || d {
            if src.is_none() {
                src = Some(arg);
            } else if dst.is_none() {
                dst = Some(arg);
            } else {
                println!("Too many arguments");
                return;
            }
        } else {
            println!("Mode is not specified");
            return;
        }
    }
    if src.is_none() {
        println!("Path for source is not provided");
        return;
    }
    if dst.is_none() {
        println!("Path for destination is not provided");
        return;
    }
    let mut src_file = File::open(src.unwrap())
        .map_err(|err| {
            println!("Can not open source file error: {err}");
            return;
        })
        .unwrap();
    let mut dst_file = File::create(dst.unwrap())
        .map_err(|err| println!("Can not create destination file error: {err}"))
        .unwrap();
    if e {
        let mut buffer = String::new();
        src_file
            .read_to_string(&mut buffer)
            .map_err(|err| println!("Could not read source file error: {err}"))
            .unwrap();
        let encoded = encode(&buffer);
        dst_file
            .write_all(&encoded)
            .map_err(|err| {
                println!("Could not write to source file error: {err}");
                return;
            })
            .unwrap();
    } else if d {
        let mut buffer = Vec::new();
        src_file
            .read_to_end(&mut buffer)
            .map_err(|err| println!("Could not read source file error: {err}"))
            .unwrap();
        let decoded = match decode(buffer) {
            Ok(decoded) => decoded,
            Err(err) => {
                println!("Could not decode text error: {err}");
                return;
            }
        };
        dst_file
            .write_all(decoded.as_bytes())
            .map_err(|err| {
                println!("Could not write to source file error: {err}");
                return;
            })
            .unwrap();
    }
}
