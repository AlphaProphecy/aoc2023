use aoc_2023::file_io::basic_file_reader;
use aoc_2023::day_1::parse_value;

fn main() {
    let sum = basic_file_reader("Usage: day1 <input file>")
        .map(|x| parse_value(&x))
        .sum::<i32>();

    println!("Sum: {}", sum);
}