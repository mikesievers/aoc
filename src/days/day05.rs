// The data to parse for day 05 will be used
// to explore the nom crate for data parsing.

// use nom::{
//     bytes::complete::tag, character::complete::{digit1, multispace0, newline, space1, u32}, combinator::map_res, multi::{many1, separated_list1}, sequence::{preceded, terminated}, IResult
// };
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space0, space1},
    combinator::map_res,
    multi::separated_list1,
};
use std::fs;
#[derive(Debug, PartialEq, Eq)]
pub struct Almanac {
    pub seeds: Vec<u32>,
    // pub seed_to_soil: Vec<Vec<u32>>,
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
    let (input, seeds_str) = separated_list1(space0, digit1).parse(input)?;
    let (input, _) = line_ending(input)?;
    let seeds = seeds_str.iter().map(|seed| seed.parse().unwrap()).collect();
    Ok((input, seeds))
}

#[test]
fn test_parse_input() {
    let almanac = Almanac::from_file("resources/day05_sample.txt");

    assert_eq!(almanac.seeds, [79, 14, 55, 13]);
}
