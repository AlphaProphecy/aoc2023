use aoc_2023::day_6::*;
use aoc_2023::file_io::basic_file_reader;

fn main() {
    let mut file = basic_file_reader("Usage: day_6 <input file>");

    let times = parse_line(file.next().unwrap());

    let distances = parse_line(file.next().unwrap());

    println!("Wins: {:?}", calculate_wins(vec![times], vec![distances]));
}
