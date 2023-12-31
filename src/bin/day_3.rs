use std::io::prelude::*;
use aoc_2023::file_io::open_file_from_env;
use aoc_2023::day_3::*;

fn main () {
    let mut reader = open_file_from_env("Usage: day3 <input file>");

    let mut symbol_map: Vec<Vec<Point>> = Vec::new();
    let mut number_map: Vec<Vec<Number>> = Vec::new();

    loop {
        let mut line = String::new();
        let len = match reader.read_line(&mut line) {
            Ok(len) => len,
            Err(e) => panic!("Error: {}", e),
        };

        if len == 0 {
            break;
        }

        let (numbers, symbols) = parse_line(&line);

        number_map.push(numbers);
        symbol_map.push(symbols);
    }

    let sum = calculate_sum(number_map, symbol_map);

    println!("Sum: {}", sum);
}