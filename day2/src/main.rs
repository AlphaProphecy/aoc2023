use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::env;
use std::cmp::max;

fn open_file(file_path: &String) -> BufReader<File> {
    let path = Path::new(file_path);

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    BufReader::new(file)
}

#[derive(Debug)]
struct Game {
    store: Store,
    id: u32,
}

#[derive(Debug)]
struct Store {
    red: u32,
    green: u32,
    blue: u32,
}

impl Store {
    fn to_power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = match args.get(1) {
        Some(path) => path,
        None => panic!("Usage: day2 <input file>"),
    };

    let mut reader = open_file(file_path);

    let mut sum = 0;

    loop {
        let mut line = String::new();
        let len = match reader.read_line(&mut line) {
            Ok(len) => len,
            Err(e) => panic!("Error: {}", e),
        };

        if len == 0 {
            break;
        }

        let game = parse_line(&line);

        sum += game.store.to_power();
    }

    println!("Sum: {}", sum);
}

fn str_parse(s: &str) -> u32 {
    match s.parse::<u32>() {
        Ok(num) => num,
        Err(e) => panic!("Error: {}", e),
    }
}

fn parse_line(line: &String) -> Game {
    let split: Vec<&str> = line.split(":").collect();

    let string_id = split.get(0).unwrap().split(" ").nth(1).unwrap();
    let id = str_parse(string_id);

    let mut store = Store {
        red: 0,
        green: 0,
        blue: 0,
    };

    let game = split.get(1).unwrap().trim();
    for set in game.split(";") {
        for reveal in set.split(",") {
            let cut = reveal.trim().split(" ").collect::<Vec<&str>>();
            let count = str_parse(cut.get(0).unwrap());
            let color = cut.get(1).unwrap().trim();

            match color {
                "red" => store.red = max(count, store.red),
                "green" => store.green = max(count, store.green),
                "blue" => store.blue = max(count, store.blue),
                _ => panic!("Unknown color: {}", color),
            }

        }
    }

    Game {
        store,
        id
    }
}