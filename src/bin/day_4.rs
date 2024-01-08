use aoc_2023::day_4::*;
use aoc_2023::file_io::basic_file_reader;

fn main() {
    let cards = basic_file_reader("Usage: day_4 <input file>")
        .map(|x| parse_line(x))
        .collect::<Vec<Card>>();

    let mut result: Vec<u32> = vec![1; cards.len()];
    for (index, card) in cards.iter().enumerate() {
        let score = card.calculate_score() as usize;

        let previous = result[index];
        result[index + 1..index + score + 1]
            .iter_mut()
            .for_each(|x| *x += previous);
    }

    let sum = result.iter().sum::<u32>();
    println!("Sum: {}", sum);
}
