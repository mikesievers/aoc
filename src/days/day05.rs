// The data to parse for day 05 will be used
// to explore the nom crate for data parsing.

// use nom::{
//     bytes::complete::tag, character::complete::{digit1, multispace0, newline, space1, u32}, combinator::map_res, multi::{many1, separated_list1}, sequence::{preceded, terminated}, IResult
// };
use nom::{
    bytes::complete::tag, character::complete::{digit1, line_ending, space0, space1}, combinator::map_res, multi::separated_list1, sequence::terminated, IResult, Parser
};
use std::fs;
#[derive(Debug, PartialEq, Eq)]
pub struct Almanac {
    pub seeds: Vec<u32>,
    //    pub seed_to_soil: Vec<Vec<u32>>,
}

impl Almanac {
    fn from_file(fname: &str) -> Self {
        let input = fs::read_to_string(fname).expect("Could not read input file.");

        let (input, seeds) = parse_seeds(input.as_str()).unwrap();

        Self { seeds }
    }
}

// Parser
// Parse a file like:
//
// seeds: 79 14 55 13

// seed-to-soil map:
// 50 98 2
// 52 50 48

// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15

fn parse_seeds(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) =
        separated_list1(space0, map_res(digit1, |s: &str| s.parse::<u32>())).parse(input)?;

    let (input, _) = line_ending(input)?;
    Ok((input, seeds))
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    // `separated_list1` parses one or more items separated by a delimiter.
    // Here, the items are `parse_u32` and the separator is `space1`.
    separated_list1(space1, parse_u32)(input)
}

/// Parses a single u32 number.
fn parse_u32(input: &str) -> IResult<&str, u32> {
    // `map_res` takes the result of a parser and tries to convert it.
    // `digit1` parses one or more digits, then we try to parse it as `u32`.
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

/// Parses a single u32 number.
// fn parse_u32(input: &str) -> impl Parser<&str> {
//     // `map_res` takes the result of a parser and tries to convert it.
//     // `digit1` parses one or more digits, then we try to parse it as `u32`.
//     map_res(digit1, |s: &str| s.parse::<u32>())
// }

#[test]
fn test_parse_input() {
    let almanac = Almanac::from_file("resources/day05_sample.txt");

    assert_eq!(almanac.seeds, [79, 14, 55, 13]);
}
