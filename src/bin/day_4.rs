use std::io::prelude::*;
use aoc_2023::file_io::open_file_from_env;
use aoc_2023::day_4::*;

fn main() {
    let mut reader = open_file_from_env("Usage: day_4 <input file>");

    let mut cards = Vec::new();    
    loop {
        let mut line = String::new();
        let len = match reader.read_line(&mut line) {
            Ok(len) => len,
            Err(e) => panic!("Error: {}", e),
        };

        if len == 0 {
            break;
        }

        cards.push(parse_line(line));
    }

    let mut result: Vec<u32> = vec![1; cards.len()];
    for (index, card) in cards.iter().enumerate() {
        let score = card.calculate_score() as usize;

        let previous = result[index];
        result[index + 1 .. index + score + 1].iter_mut().for_each(|x| *x += previous);
    }

    let sum = result.iter().sum::<u32>();
    println!("Sum: {}", sum);
}