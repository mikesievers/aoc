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

        let number = cardnr_and_date[0].split_whitespace().collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();

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
        let nr_matches = Stack::count_matches(card);
        match nr_matches {
            0 => 0,
            _ => 2_i32.pow(nr_matches - 1),
        }
    }

    fn count_matches(card: &Card) -> u32 {
        card.winners
            .iter()
            .map(|winner| match card.numbers.contains(winner) {
                true => 1,
                false => 0,
            })
            .sum()
    }

    pub fn sum_scores(&self) -> i32 {
        self.cards
            .iter()
            .map(|c| Stack::score(c))
            .collect::<Vec<i32>>()
            .iter()
            .sum()
    }

    // Part 2
    pub fn sum_all_cards(&self) -> usize {
        // Accumulate all cards' indices
        // Start with the original ones
        let card_indices: Vec<usize> = (0..self.cards.len()).collect();

        // Determine which card copies have been won and add them to the indices

        let won_cards = self.get_won_copies(&card_indices);

        // count the number of original cards and the sum of all cards won
        card_indices.len() + won_cards.len()
    }

    fn get_won_copies(&self, card_indices: &Vec<usize>) -> Vec<usize> {
        // for all cards in the Vec
        //    get won copies for this card
        //    return the card
        //    add them to the Vec
        card_indices
            .iter()
            .map(|card_idx| {
                let nr_matches = Stack::count_matches(&self.cards[*card_idx]) as usize;
                match nr_matches {
                    0 => vec![],
                    n => {
                        // ensure there is room for at least one card when matches are found
                        if card_idx + n < self.cards.len() - 1 {
                            let upper_idx = (card_idx + n).min(self.cards.len() - 1);
                            let mut won_cards: Vec<usize> = ((card_idx + 1)..=upper_idx).collect();
                            // println!("Card {card_idx} won cards: {:?}", won_cards);
                            won_cards.extend(self.get_won_copies(&won_cards));
                            won_cards
                        } else {
                            vec![]
                        }
                    }
                }
            })
            .into_iter()
            .flatten()
            .collect::<Vec<usize>>()
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

// Part 2
// There's no such thing as "points". Instead, scratchcards only cause you to win more scratchcards equal to the number of winning numbers you have.

// Specifically, you win copies of the scratchcards below the winning card equal to the number of matches. So, if card 10 were to have 5 matching
// numbers, you would win one copy each of cards 11, 12, 13, 14, and 15.

// Copies of scratchcards are scored like normal scratchcards and have the same card number as the card they copied. So, if you win a copy of card 10
// and it has 5 matching numbers, it would then win a copy of the same cards that the original card 10 won: cards 11, 12, 13, 14, and 15. This process
// repeats until none of the copies cause you to win any more cards. (Cards will never make you copy a card past the end of the table.)

// This time, the above example goes differently:

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11

//     Card 1 has four matching numbers, so you win one copy each of the next four cards: cards 2, 3, 4, and 5.
//     Your original card 2 has two matching numbers, so you win one copy each of cards 3 and 4.
//     Your copy of card 2 also wins one copy each of cards 3 and 4.
//     Your four instances of card 3 (one original and three copies) have two matching numbers, so you win four copies each of cards 4 and 5.
//     Your eight instances of card 4 (one original and seven copies) have one matching number, so you win eight copies of card 5.
//     Your fourteen instances of card 5 (one original and thirteen copies) have no matching numbers and win no more cards.
//     Your one instance of card 6 (one original) has no matching numbers and wins no more cards.

// Once all of the originals and copies have been processed, you end up with 1 instance of card 1, 2 instances of card 2, 4 instances of card 3,
// 8 instances of card 4, 14 instances of card 5, and 1 instance of card 6. In total, this example pile of scratchcards causes you to ultimately have 30 scratchcards!

#[test]
fn test_sum_all_cards() {
    let stack = Stack::from("resources/day04_sample_p2.txt");

    assert_eq!(stack.sum_all_cards(), 30);
}
