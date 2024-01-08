use std::cmp::min;

#[derive(Clone, Debug)]
pub struct Point {
    x: i32,
}

#[derive(Clone, Debug)]
pub struct Number {
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
        return self.start.x - 1 <= point.x && self.end.x + 1 >= point.x;
    }
}

pub fn parse_line(line: &str) -> (Vec<Number>, Vec<Point>) {
    let mut symbols: Vec<Point> = Vec::new();
    let mut numbers: Vec<Number> = Vec::new();

    let mut current_number: Number = Number {
        value: 0,
        start: Point { x: 0 },
        end: Point { x: 0 },
    };

    for (index, char) in line.chars().enumerate() {
        let x = index as i32;
        if char == '\n' {
            if !current_number.is_new() {
                numbers.push(current_number.clone());
                current_number.reset();
            }
            break;
        }

        if char == '.' {
            if !current_number.is_new() {
                numbers.push(current_number.clone());
                current_number.reset();
            }
            continue;
        }

        if char == '*' {
            symbols.push(Point { x });
            if !current_number.is_new() {
                numbers.push(current_number.clone());
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
        }
    }

    (numbers, symbols)
}

pub fn calculate_sum(number_map: Vec<Vec<Number>>, symbol_map: Vec<Vec<Point>>) -> i32 {
    let mut sum = 0;
    for (index, line) in symbol_map.iter().enumerate() {
        for symbol in line {
            let min_index = match index {
                0 => 0,
                _ => index - 1,
            } as usize;

            let max_index = min(index + 1, number_map.len() - 1);

            let mut adjacent_numbers: Vec<Number> = Vec::new();

            for number_lines in number_map[min_index..=max_index].iter() {
                for number in number_lines {
                    if number.next_to_point(symbol) {
                        adjacent_numbers.push(number.clone());
                    }
                }
            }

            if adjacent_numbers.len() == 2 {
                sum += adjacent_numbers[0].value * adjacent_numbers[1].value;
            }
        }
    }
    sum
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
