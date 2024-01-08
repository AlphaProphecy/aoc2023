use aoc_2023::day_3::*;
use aoc_2023::file_io::basic_file_reader;

fn main() {
    let mut symbol_map: Vec<Vec<Point>> = Vec::new();
    let mut number_map: Vec<Vec<Number>> = Vec::new();

    for line in basic_file_reader("Usage: day_3 <input file>") {
        let (numbers, symbols) = parse_line(&line);

        number_map.push(numbers);
        symbol_map.push(symbols);
    }

    let sum = calculate_sum(number_map, symbol_map);

    println!("Sum: {}", sum);
}
