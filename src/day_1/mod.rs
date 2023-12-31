use std::collections::VecDeque;

pub fn parse_value(value: &str) -> i32 {
    let mut reader = Reader::new(VALUES.iter().map(|s| s.to_string()).collect());
    for char in value.chars() {
        reader.next(char);
    }

    reader.first.unwrap() * 10 + reader.second.unwrap()
}

const VALUES: &[&str; 18] = &[
    "1",
    "one",
    "2",
    "two",
    "3",
    "three",
    "4",
    "four",
    "5",
    "five",
    "6",
    "six",
    "7",
    "seven",
    "8",
    "eight",
    "9",
    "nine",
];

fn to_value(value: &str) -> i32 {
    match value {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!("Invalid value: {}", value),
    }
}

struct Reader {
    start_options: Vec<String>,
    options: Vec<String>,
    first: Option<i32>,
    second: Option<i32>,
    chars: VecDeque<char>,
}

impl Reader {
    fn new(options: Vec<String>) -> Reader {
        Reader {
            start_options: options.clone(),
            options,
            first: None,
            second: None,
            chars: VecDeque::new(),
        }
    }

    fn set_first_second(&mut self, value: i32) {
        self.first = match self.first {
            Some(num) => Some(num),
            None => Some(value),
        };

        self.second = Some(value);
    }

    fn next(&mut self, char: char) {
        self.chars.push_back(char);

        let mut start_size = self.options.len();
        while self.chars.len() > 0 {
            self.options = self.filter_options();

            if self.options.len() == 0 {
                self.chars.pop_front();
                self.options = self.start_options.clone();
            }

            if self.options.len() == start_size && self.options.len() != self.start_options.len() {
                break;
            }
    
            start_size = self.options.len();
        }
    }

    fn filter_options(&mut self) -> Vec<String> {
        let mut remaining = Vec::new();
        let size = self.chars.len();

        for option in &self.options {
            let slice: Vec<char> = option.chars().take(size).collect();
            if slice == Vec::from(self.chars.clone()) {
                if option.chars().count() == size {
                    let value = to_value(&option);
                    self.set_first_second(value);
                    self.options = self.start_options.clone();
                    return Vec::new();
                }
                remaining.push(option.to_string());
            }
        }

        remaining
    }
}
