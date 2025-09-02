// Camel Cards
// In Camel Cards, you get a list of hands, and your goal is to order them based
// on the strength of each hand. A hand consists of five cards labeled one of A,
// K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2. The relative strength of each card
// follows this order, where A is the highest and 2 is the lowest.

// Every hand is exactly one type. From strongest to weakest, they are:

//  -  Five of a kind, where all five cards have the same label: AAAAA
//  -  Four of a kind, where four cards have the same label and one card has a different label: AA8AA
//  -  Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
//  -  Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
//  -  Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
//  -  One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
//  -  High card, where all cards' labels are distinct: 23456

// Hands are primarily ordered based on type; for example, every full house is stronger than any three of a kind.

// If two hands have the same type, a second ordering rule takes effect. Start by
// comparing the first card in each hand. If these cards are different, the hand
// with the stronger first card is considered stronger. If the first card in each
// hand have the same label, however, then move on to considering the second card
// in each hand. If they differ, the hand with the higher second card wins;
// otherwise, continue with the third card in each hand, then the fourth, then the
// fifth.

// So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger because
// its first card is stronger. Similarly, 77888 and 77788 are both a full house,
// but 77888 is stronger because its third card is stronger (and both hands have
// the same first and second card).

use std::{collections::HashMap, fs::read_to_string};

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{i64, line_ending, one_of, space0},
    combinator::map,
    multi::{count, separated_list1},
    sequence::delimited,
};

// Models
#[derive(Debug, PartialOrd, PartialEq, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Hash, Debug, PartialOrd, PartialEq, Eq)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: i64,
    //hand_type: HandType,
}

#[derive(Debug)]
struct Game {
    hands: Vec<Hand>,
}

impl Game {
    pub fn from_file(input: &str) -> Self {
        let input = read_to_string(input).unwrap();
        let (_, hands) = parse_game(input.as_str()).unwrap();
        Game { hands }
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
    map((parse_cards, parse_bid), |(cards, bid)| Hand { cards, bid }).parse(input)
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

impl Game {
    fn score_hand(cards: Vec<Card>) -> HandType {
        let mut frequencies: HashMap<Card, usize> = HashMap::new();
        for card in cards {
            *frequencies.entry(card).or_insert(0) += 1;
        }

        let counts: Vec<i32> = frequencies
            .iter()
            .map(|(_card, count)| *count as i32)
            .collect();

        if counts.contains(&5) {
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
}

#[test]
fn test_score() {
    let cards = vec![Card::Four, Card::Four, Card::Four, Card::Four, Card::Four];
    assert_eq!(Game::score_hand(cards), HandType::FiveOfAKind);

    let cards = vec![Card::T, Card::T, Card::T, Card::T, Card::Two];
    assert_eq!(Game::score_hand(cards), HandType::FourOfAKind);

    let cards = vec![Card::T, Card::T, Card::T, Card::Two, Card::Two];
    assert_eq!(Game::score_hand(cards), HandType::FullHouse);

    let cards = vec![Card::T, Card::T, Card::T, Card::Three, Card::Two];
    assert_eq!(Game::score_hand(cards), HandType::ThreeOfAKind);

    let cards = vec![Card::K, Card::T, Card::T, Card::K, Card::Two];
    assert_eq!(Game::score_hand(cards), HandType::TwoPair);

    let cards = vec![Card::Q, Card::T, Card::J, Card::Q, Card::Two];
    assert_eq!(Game::score_hand(cards), HandType::OnePair);

    let cards = vec![Card::Q, Card::T, Card::J, Card::Four, Card::Two];
    assert_eq!(Game::score_hand(cards), HandType::HighCard);
}
