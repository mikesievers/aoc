// Camel Cards
// In Camel Cards, you get a list of hands, and your goal is to order them based on the strength of each hand. A hand consists of five cards labeled one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2. The relative strength of each card follows this order, where A is the highest and 2 is the lowest.

// Every hand is exactly one type. From strongest to weakest, they are:

//     Five of a kind, where all five cards have the same label: AAAAA
//     Four of a kind, where four cards have the same label and one card has a different label: AA8AA
//     Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
//     Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
//     Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
//     One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
//     High card, where all cards' labels are distinct: 23456

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

use nom::{IResult, Parser, character::complete::one_of, combinator::map, multi::count};

// Models

#[derive(Debug, PartialOrd, PartialEq, Eq)]
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

struct Hand {
    cards: Vec<Card>,
    bid: i64,
}

struct Game {
    hands: Vec<Card>,
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
fn parse_hand(input: &str) -> IResult<&str, Vec<Card>> {
    count(parse_card, 5).parse(input)
}

#[test]
fn test_parsing() {
    let hand_str = "QQT92";
    let (_, hand) = parse_hand(hand_str).unwrap();
    assert_eq!(hand, vec![Card::Q, Card::Q, Card::T, Card::Nine, Card::Two]);

}
