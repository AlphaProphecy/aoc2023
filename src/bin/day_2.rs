use aoc_2023::day_2::*;
use aoc_2023::file_io::basic_file_reader;

fn main() {
    let sum = basic_file_reader("Usage: day_2 <input file>")
        .map(|x| parse_line(&x).store.to_power())
        .sum::<u32>();

    println!("Sum: {}", sum);
}
