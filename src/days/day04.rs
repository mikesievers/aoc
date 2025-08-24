use std::fs::*;
use std::io::{BufRead, BufReader};

const DAY04_SAMPLE_INPUT: &str ="resources/day04_sample.txt";
const DAY04_INPUT: &str ="resources/day04_input.txt";

struct Stack {
    lines: Vec<String>,

    cards: Vec<Card>
}

struct Card {
    winners: Vec<i32>,
    numbers: Vec<i32>,
}

impl Stack {
    fn from(fname: &str) -> Self {
        let lines = Stack::read_input(fname); 
        let cards = lines.iter().map(|line| Stack::parse_line(line.to_string())).collect();
        Stack { lines, cards}
    }

    fn read_input(fname: &str) -> Vec<String> {
        let file = File::open(fname).unwrap();
        let reader = BufReader::new(file);

        reader.lines().map(
                | line | {
                    String::from(line.unwrap().trim())
                    }).collect()
    }

    fn parse_line(line: String) -> Card {
        let winners_and_numbers = line.splitn(2,':').collect::<Vec<&str>>()[1]
        .splitn(2,'|').collect::<Vec<&str>>();
        let winners = winners_and_numbers[0].split_ascii_whitespace().map(|nr| nr.parse().unwrap()).collect();
        let numbers = winners_and_numbers[1].split_ascii_whitespace().map(|nr| nr.parse().unwrap()).collect();
        Card { winners, numbers }
    }

    fn score(card: &Card) -> i32 {
        let nr_matches = card.winners.iter()
        .map(|winner| match card.numbers.contains(winner) {
            true => 1,
            false => 0,
        }).sum();
        match nr_matches {
            0 => 0,
            _ => 2_i32.pow(nr_matches-1)
        }
    }

    fn sum_scores(&self) -> i32 {
        self.cards.iter().map(
            |c| Stack::score(c)
        ).collect::<Vec<i32>>().iter().sum()
    }

}

#[test]
fn test_stack() {
    let stack = Stack::from(DAY04_SAMPLE_INPUT);

    assert!(stack.lines.len() > 0);
    assert_eq!(stack.cards.len(), stack.lines.len());
    assert_eq!(stack.cards[0].winners[2], 83);
    assert_eq!(stack.cards[0].numbers[2], 6);
    assert_eq!(Stack::score(&stack.cards[0]), 8);
    assert_eq!(stack.sum_scores(), 13);

}