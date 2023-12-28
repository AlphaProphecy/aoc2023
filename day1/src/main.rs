use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    let file_path = match args.get(1) {
        Some(path) => path,
        None => panic!("Usage: day1 <input file>"),
    };

    let path = Path::new(file_path);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut reader = BufReader::new(file);

    let mut sum = 0;

    loop {
        let mut line = String::new();
        let len = match reader.read_line(&mut line) {
            Ok(len) => len,
            Err(e) => panic!("Error: {}", e),
        };

        if len == 0 {
            break;
        }

        sum += parse_value(&line);
    } 

    println!("Sum: {}", sum);
}

fn parse_value(value: &str) -> i32 {
    let mut first: Option<i32> = None;
    let mut second: Option<i32> = None;

    for char in value.chars() {
        let raw = char as u8;
        if raw < 48 || raw > 57 {
            continue;
        }

        let digit = raw - 48;
        first = match first {
            Some(num) => Some(num),
            None => Some(digit as i32),
        };

        second = Some(digit as i32);
    }

    return first.unwrap() * 10 + second.unwrap();
}