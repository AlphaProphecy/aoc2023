use std::cmp::max;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Game {
    pub store: Store,
    id: u32,
}

#[derive(Debug)]
pub struct Store {
    red: u32,
    green: u32,
    blue: u32,
}

impl Store {
    pub fn to_power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn str_parse(s: &str) -> u32 {
    match s.parse::<u32>() {
        Ok(num) => num,
        Err(e) => panic!("Error: {}", e),
    }
}

pub fn parse_line(line: &String) -> Game {
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

    Game { store, id }
}
