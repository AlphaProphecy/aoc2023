use aoc_2023::file_io::basic_file_reader;
use aoc_2023::day_5::*;

fn main() {
    let mut lines = basic_file_reader("Usage: day_5 <input file>");

    let seeds = lines.next().unwrap()
        .split(":").nth(1).unwrap()
        .trim().split(" ")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut ranges: Vec<Ranges> = Vec::new();

    let split = lines.skip(1).fold(Vec::new(), |mut acc: Vec<Vec<String>>, line| {
        if line.contains(":") {
            return acc;
        }
        match acc.last_mut() {
            Some(last) => {
                if !line.is_empty() {
                    last.push(line);
                } else {
                    acc.push(Vec::new());
                }
            },
            None => {
                acc.push(vec![line]);
            }
        };
        acc
    });

    for range in split {
        ranges.push(parse_ranges(range));
    }

    let mut seed_ranges = seeds.iter()
        .fold(Vec::new(), |mut acc: Vec<Vec<i64>>, s | {
            match acc.last_mut() {
                Some(last) => {
                    if last.len() < 2 {
                        last.push(*s);
                    } else {
                        acc.push(vec![*s]);
                    }
                },
                None => {
                    acc.push(vec![*s]);
                }
            };
            acc
        }).iter().map(|range| IntRange::new(range[0], range[1]))
        .collect::<Vec<IntRange>>();

    let v = seed_ranges.iter_mut()
        .map(
            |range|
            range.into_iter().map(|i: i64| map_through(&ranges, i)).min().unwrap()
        ).min().unwrap();

    println!("Min location: {:?}", v);
}