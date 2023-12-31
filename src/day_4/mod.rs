use crate::vectors::intersection;

#[derive(Debug)]
pub struct Card {
    pub winners: Vec<u32>,
    pub deck: Vec<u32>,
}

impl Card {
    pub fn calculate_score(&self) -> u32 {
        let duplicates = intersection(&self.winners, &self.deck);

        let length = duplicates.len();
        if length == 0 {
            return 0;
        }

        let base: u32 = 2;
        base.pow((length as u32) - 1)
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