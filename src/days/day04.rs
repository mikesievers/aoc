use std::fs::*;
use std::io::{BufRead, BufReader};

pub struct Stack {
    cards: Vec<Card>,
}

struct Card {
    number: i32,
    winners: Vec<i32>,
    numbers: Vec<i32>,
}

impl Stack {
    pub fn from(fname: &str) -> Self {
        let lines = Stack::read_input(fname);
        let cards = lines
            .iter()
            .map(|line| Stack::parse_line(line.to_string()))
            .collect();
        Stack { cards }
    }

    fn read_input(fname: &str) -> Vec<String> {
        let file = File::open(fname).unwrap();
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(|line| String::from(line.unwrap().trim()))
            .collect()
    }

    fn parse_line(line: String) -> Card {
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        let cardnr_and_date = line.splitn(2, ':').collect::<Vec<&str>>();

        let number = cardnr_and_date[0].split_whitespace().collect::<Vec<&str>>()[1].parse().unwrap();

        //  41 48 83 86 17 | 83 86  6 31 17  9 48 53
        let winners_and_numbers = cardnr_and_date[1].splitn(2, '|').collect::<Vec<&str>>();

        let winners = winners_and_numbers[0]
            .split_ascii_whitespace()
            .map(|nr| nr.parse().unwrap())
            .collect();
        let numbers = winners_and_numbers[1]
            .split_ascii_whitespace()
            .map(|nr| nr.parse().unwrap())
            .collect();
        Card {
            number,
            winners,
            numbers,
        }
    }

    fn score(card: &Card) -> i32 {
        let nr_matches = card
            .winners
            .iter()
            .map(|winner| match card.numbers.contains(winner) {
                true => 1,
                false => 0,
            })
            .sum();
        match nr_matches {
            0 => 0,
            _ => 2_i32.pow(nr_matches - 1),
        }
    }

    pub fn sum_scores(&self) -> i32 {
        self.cards
            .iter()
            .map(|c| Stack::score(c))
            .collect::<Vec<i32>>()
            .iter()
            .sum()
    }
}

#[test]
fn test_stack() {
    let stack = Stack::from("resources/day04_sample.txt");

    assert_eq!(stack.cards[0].number, 1);
    assert_eq!(stack.cards[0].winners[2], 83);
    assert_eq!(stack.cards[0].numbers[2], 6);
    assert_eq!(Stack::score(&stack.cards[0]), 8);
    assert_eq!(stack.sum_scores(), 13);
}
