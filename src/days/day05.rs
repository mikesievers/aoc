// The data to parse for day 05 will be used
// to explore the nom crate for data parsing.
// See https://docs.rs/nom/8.0.0/nom/index.html

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{char, line_ending, one_of},
    combinator::{map, recognize},
    error::Error,
    multi::{many0, many1, separated_list1},
    sequence::{preceded, terminated},
};

use std::fs::{self, read_to_string};
#[derive(Debug, PartialEq, Eq)]
pub struct Almanac {
    pub seeds: Vec<i32>,
    pub seed_to_soil: Vec<Vec<i32>>,
}

impl Almanac {
    fn from_file(fname: &str) -> Self {
        let input = fs::read_to_string(fname).expect("Could not read input file.");

        // Parser for a single integer

        let seeds = vec![];
        let seed_to_soil = vec![];

        Self {
            seeds,
            seed_to_soil,
        }
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

#[test]
fn test_parse_input() {
    let almanac = Almanac::from_file("resources/day05_sample.txt");

    assert_eq!(almanac.seeds, [79, 14, 55, 13]);
}

// nom experimentation area

fn decimal(input: &str) -> IResult<&str, String> {
    map(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |s: &str| s.replace('_', ""),
    )
    .parse(input)
}

fn decimal_nr(input: &str) -> IResult<&str, i32> {
    map(recognize(many1(one_of("0123456789"))), |s: &str| {
        s.parse().expect("Could not parse into integer")
    })
    .parse(input)
}

fn decimal_line(input: &str) -> IResult<&str, Vec<i32>> {
    terminated(separated_list1(tag(" "), decimal_nr), line_ending).parse(input)
}

fn seed_section(input: &str) -> IResult<&str, Vec<i32>> {
    preceded(tag("seeds: "), decimal_line).parse(input)
}

#[test]
fn test_parsing() {
    let mystr = "10000";
    //let res = mystr.parse::<i32>().unwrap();

    let res = decimal_nr.parse(mystr).unwrap();

    assert_eq!(res.1, 10000);

    let mystr = "1 2 3\r\n";
    let res = decimal_line.parse(mystr).unwrap();

    assert_eq!(res.1, vec![1, 2, 3]);
    assert_eq!(res.0, "");

    let input = read_to_string("resources/day05_sample.txt").unwrap();
    let seeds_res = seed_section.parse(input.as_str()).unwrap();

    assert_eq!(seeds_res.1, vec![79, 14, 55, 13])
}
