#[derive(Debug)]
pub struct Mapping {
    pub in_start: i64,
    pub out_start: i64,
    pub range: i64,    
}

impl Mapping {
    fn map(&self, input: i64) -> i64 {
        let offset = input - self.in_start;
        self.out_start + offset
    }

    fn in_range(&self, input: i64) -> bool {
        input >= self.in_start && input <= self.in_start + self.range
    }    
}

#[derive(Debug)]
pub struct Ranges {
    pub ranges: Vec<Mapping>,
}

impl Ranges {
    pub fn map(&self, input: i64) -> i64 {
        for mapping in self.ranges.iter() {
            if mapping.in_range(input) {
                return mapping.map(input);
            }
        }

        input
    }
}

#[derive(Debug, Clone)]
pub struct IntRange {
    start: i64,
    range: i64,
    count: i64,
}

impl IntRange {
    pub fn new(start: i64, range: i64) -> IntRange {
        IntRange {
            start,
            range,
            count: 0,
        }
    } 
}

impl Iterator for IntRange {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        if self.count >= self.range {
            return None;
        }
        let out = self.start + self.count;
        self.count += 1;
        Some(out)
    }
}

pub fn parse_mapping(input: &str) -> Mapping {
    let mut numbers = input.split(" ").map(|s| s.parse::<i64>().unwrap());
    let out_start = numbers.next().unwrap();
    let in_start = numbers.next().unwrap();
    let range = numbers.next().unwrap();

    Mapping {
        in_start,
        out_start,
        range,
    }
}

pub fn parse_ranges(input: Vec<String>) -> Ranges {
    let mut ranges = Vec::new();

    for line in input {
        ranges.push(parse_mapping(&line));
    }

    Ranges {
        ranges,
    }
}

pub fn map_through(ranges: &Vec<Ranges>, input: i64) -> i64 {
    ranges.iter().fold(input, |acc, r| r.map(acc))
}