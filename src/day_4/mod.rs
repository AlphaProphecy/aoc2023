use crate::vectors::intersection;

#[derive(Debug)]
pub struct Card {
    winners: Vec<u32>,
    deck: Vec<u32>,
}

impl Card {
    pub fn calculate_score(&self) -> u32 {
        let duplicates = intersection(&self.winners, &self.deck);

        duplicates.len() as u32
    }
}

pub fn parse_line(line: String) -> Card {
    let mut data = line.split(":").skip(1).next().unwrap().split("|");

    let winners = parse_numbers(data.next().unwrap());
    let deck = parse_numbers(data.next().unwrap());

    Card {
        winners,
        deck,
    }
}

fn parse_numbers(line: &str) -> Vec<u32> {
    line.trim().split(" ").filter_map(|x| x.parse().ok()).collect::<Vec<u32>>()
}