use std::io::prelude::*;
use aoc_2023::file_io::open_file_from_env;
use aoc_2023::day_1::parse_value;

fn main() {
    let mut reader = open_file_from_env("Usage: day1 <input file>");

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