// Camel Cards
// Day 07, part 2
//
// Most is the same as in Part 1, except the joker is the lowest card
// but can be used for any combination.
//
// NEXT:
// Implement scoring
// - Write tests for the classifications of the sample data set
// - count jokers separate from the rest
// - for the different possibilities of the rest, consider adding the jokers to get the next best class

use std::{cmp::Ordering, collections::HashMap, fs::read_to_string};

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{i64, line_ending, one_of, space0},
    combinator::map,
    multi::{count, separated_list1},
    sequence::delimited,
};

// Models
#[derive(Copy, Clone, Debug, Ord, PartialOrd, PartialEq, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone, Copy, Hash, Debug, Ord, PartialOrd, PartialEq, Eq)]
enum Card {
    J, // In Part 2, the Joker is the lowest card
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
    bid: i64,
    hand_type: HandType,
}

#[derive(Debug)]
pub struct Game {
    hands: Vec<Hand>,
}

impl Game {
    pub fn from_file(input: &str) -> Self {
        let input = read_to_string(input).unwrap();
        let (_, mut hands) = parse_game(input.as_str()).unwrap();
        hands.sort();
        Game { hands }
    }

    fn score_hand(orig_cards: &Vec<Card>) -> HandType {
        let mut frequencies: HashMap<Card, usize> = HashMap::new();

        // Count the jokers separately, they can augment any card
        let mut jokers = 0;
        let mut cards = orig_cards.clone();

        for card in cards {
            match card {
                Card::J => {
                    jokers += 1;
                }
                _ => {
                    *frequencies.entry(card).or_insert(0) += 1;
                }
            }
        }

        let mut counts: Vec<i32> = frequencies
            .iter()
            .map(|(_card, count)| *count as i32)
            .collect();

        // No counts means only jokers
        if jokers == 5 {
            return HandType::FiveOfAKind;
        }

        // Add the number of jokers to the highest count
        let max_count = counts.iter().max();
        match max_count {
            Some(max_count) => {
                for idx in 0..5 {
                    if counts[idx] == *max_count {
                        counts[idx] += jokers;
                        break;
                    }
                }
            }
            _ => {}
        }

        if counts.contains(&(5)) {
            return HandType::FiveOfAKind;
        }
        if counts.contains(&4) {
            return HandType::FourOfAKind;
        }
        if counts.contains(&3) && counts.contains(&2) {
            return HandType::FullHouse;
        }
        if counts.contains(&3) {
            return HandType::ThreeOfAKind;
        }
        if counts.len() == 3 && counts.contains(&1) && counts.contains(&2) {
            return HandType::TwoPair;
        }
        if counts.contains(&2) {
            return HandType::OnePair;
        }
        HandType::HighCard
    }

    pub fn winnings(&self) -> i64 {
        let mut winnings = 0_i64;
        for (zero_based_rank, hand) in self.hands.iter().enumerate() {
            winnings += (zero_based_rank as i64 + 1_i64) * hand.bid;
        }
        winnings
    }
}

#[test]
fn test_card() {
    let two = Card::Two;
    let three = Card::Three;
    let three_too = Card::Three;
    assert!(three > two);
    assert!(three == three_too);
}

// Parsing

fn parse_card(input: &str) -> IResult<&str, Card> {
    map(one_of("23456789TJQKA"), |c| match c {
        '2' => Card::Two,
        '3' => Card::Three,
        '4' => Card::Four,
        '5' => Card::Five,
        '6' => Card::Six,
        '7' => Card::Seven,
        '8' => Card::Eight,
        '9' => Card::Nine,
        'T' => Card::T,
        'J' => Card::J,
        'Q' => Card::Q,
        'K' => Card::K,
        'A' => Card::A,
        _ => unreachable!(),
    })
    .parse(input)
}
fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    count(parse_card, 5).parse(input)
}

fn parse_bid(input: &str) -> IResult<&str, i64> {
    delimited(tag(" "), i64, space0).parse(input)
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    map((parse_cards, parse_bid), |(cards, bid)| {
        let hand_type = Game::score_hand(&cards);
        Hand {
            cards,
            bid,
            hand_type,
        }
    })
    .parse(input)
}

fn parse_game(input: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(line_ending, parse_hand).parse(input)
}

#[test]
fn test_parsing() {
    let hand_str = "QQT92";
    let (_, hand) = parse_cards(hand_str).unwrap();
    assert_eq!(hand, vec![Card::Q, Card::Q, Card::T, Card::Nine, Card::Two]);

    let row = "TTT32 28 \n";
    let (_, (hand, bid)) = (parse_cards, parse_bid).parse(row).unwrap();
    assert_eq!(
        hand,
        vec![Card::T, Card::T, Card::T, Card::Three, Card::Two]
    );
    assert_eq!(bid, 28);

    let game = Game::from_file("resources/day07_sample.txt");
    println!("{:?}", game);
}

#[test]
fn test_score() {
    // 5
    let cards = vec![Card::Four, Card::Four, Card::Four, Card::Four, Card::Four];
    assert_eq!(Game::score_hand(&cards), HandType::FiveOfAKind);

    let cards = vec![Card::Four, Card::Four, Card::J, Card::Four, Card::Four];
    assert_eq!(Game::score_hand(&cards), HandType::FiveOfAKind);

    let cards = vec![Card::J, Card::Four, Card::J, Card::Four, Card::Four];
    assert_eq!(Game::score_hand(&cards), HandType::FiveOfAKind);

    let cards = vec![Card::J, Card::Four, Card::J, Card::J, Card::J];
    assert_eq!(Game::score_hand(&cards), HandType::FiveOfAKind);

    let cards = vec![Card::J, Card::J, Card::J, Card::J, Card::J];
    assert_eq!(Game::score_hand(&cards), HandType::FiveOfAKind);

    // 4
    let cards = vec![Card::T, Card::T, Card::T, Card::J, Card::Two];
    assert_eq!(Game::score_hand(&cards), HandType::FourOfAKind);

    let cards = vec![Card::T, Card::T, Card::T, Card::Two, Card::Two];
    assert_eq!(Game::score_hand(&cards), HandType::FullHouse);

    let cards = vec![Card::T, Card::T, Card::T, Card::Three, Card::Two];
    assert_eq!(Game::score_hand(&cards), HandType::ThreeOfAKind);

    let cards = vec![Card::K, Card::T, Card::T, Card::K, Card::Two];
    assert_eq!(Game::score_hand(&cards), HandType::TwoPair);

    let cards = vec![Card::Q, Card::T, Card::J, Card::Q, Card::Two];
    assert_eq!(Game::score_hand(&cards), HandType::ThreeOfAKind);

    let cards = vec![Card::Q, Card::T, Card::J, Card::Four, Card::Two];
    assert_eq!(Game::score_hand(&cards), HandType::OnePair);
}

// Hand: Ordering
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_type_cmp = self.hand_type.cmp(&other.hand_type);
        if hand_type_cmp != Ordering::Equal {
            return hand_type_cmp;
        }

        for (idx, card) in self.cards.iter().enumerate() {
            let card_cmp = card.cmp(&other.cards[idx]);
            if card_cmp != Ordering::Equal {
                return card_cmp;
            }
        }
        unreachable!();
    }
}

#[test]
fn test_ordering() {
    let game = Game::from_file("resources/day07_sample.txt");
    println!("{:?}", game);

    let hand1 = game.hands[0].clone();
    let hand1_too = game.hands[0].clone();
    assert_eq!(hand1, hand1_too);

    let hand2 = game.hands[1].clone();
    assert!(hand2 > hand1);
}

#[test]
fn test_winnings() {
    let game = Game::from_file("resources/day07_sample.txt");

    assert_eq!(game.winnings(), 5905);
}
