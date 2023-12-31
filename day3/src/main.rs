use std::char;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::env;
use std::cmp::min;

fn open_file(file_path: &String) -> BufReader<File> {
    let path = Path::new(file_path);

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    BufReader::new(file)
}

#[derive(Clone, Debug)]
struct Point {
    x: i32,
}

#[derive(Clone, Debug)]
struct Number {
    value: i32,
    start: Point,
    end: Point,
}

impl Number {
    fn is_new(&self) -> bool {
        self.value == 0
    }

    fn reset(&mut self) {
        self.value = 0;
        self.start = Point { x: 0 };
        self.end = Point { x: 0 };
    }

    fn add_number(&mut self, number: i32) {
        self.value = self.value * 10 + number;
    }

    fn next_to_point(&self, point: &Point) -> bool {
        return self.start.x -1 <= point.x && self.end.x + 1 >= point.x; 
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = match args.get(1) {
        Some(path) => path,
        None => panic!("Usage: day2 <input file>"),
    };

    let mut reader = open_file(file_path);

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
        number_map.push(Vec::new());
        symbol_map.push(Vec::new());

    

        let mut current_number: Number = Number {
            value: 0,
            start: Point { x: 0 },
            end: Point { x: 0 },
        };

        for (index, char) in line.chars().enumerate() {
            let x = index as i32;
            if char == '\n' {
                if !current_number.is_new() {
                    number_map.last_mut().unwrap().push(current_number.clone());
                    current_number.reset();
                }
                break;
            }

            if char == '.' {
                if !current_number.is_new() {
                    number_map.last_mut().unwrap().push(current_number.clone());
                    current_number.reset();
                }
                continue;
            }
            let parsed_number = try_parse_number(char);

            if parsed_number.is_some() {
                if current_number.is_new() {
                    current_number.start = Point { x };
                }
                current_number.add_number(parsed_number.unwrap());
                current_number.end = Point { x };
            } else {
                symbol_map.last_mut().unwrap().push(Point { x });
                if !current_number.is_new() {
                    number_map.last_mut().unwrap().push(current_number.clone());
                    current_number.reset();
                }
            }

        }
    }

    let mut sum = 0;
    for (index, line) in number_map.iter().enumerate() {
        for number in line {
            let min_index = match index {
                0 => 0,
                _ => index - 1,
            } as usize;

            let max_index = min(index + 1, symbol_map.len() - 1);

            'symbol_loop: for symbol_lines in symbol_map[min_index ..= max_index].iter() {
                for symbol in symbol_lines {
                    if number.next_to_point(symbol) {
                        sum += number.value;
                        break 'symbol_loop;
                    }
                }
            }
        }
    }

    let max = number_map.iter().map(|line| line.iter().map(|number| number.value).max()).max().unwrap().unwrap();
    let max_sum = number_map.iter().map(|line| line.iter().map(|number| number.value).sum::<i32>()).sum::<i32>();
    let number_count = number_map.iter().map(|line| line.len()).sum::<usize>();

    println!("Number count: {}", number_count);
    println!("Max: {}", max);
    println!("Max sum: {}", max_sum);
    println!("Sum: {}", sum);
}

fn try_parse_number(char: char) -> Option<i32> {
    match char {
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        '0' => Some(0),
        _ => None,
    }
}