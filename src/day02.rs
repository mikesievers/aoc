//! Advent of Code day02
use std::fs::read_to_string;

use nom::{
    IResult, Parser,
    character::complete::{char, line_ending, u64},
    combinator::opt,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
};

#[derive(PartialEq, Debug)]
struct IDRange {
    start: u64,
    end: u64,
}

pub struct IDRanges {
    // The ranges have the following features:
    // 1. start and end can have different lengths
    // 2. end length is at most one more than the start
    ranges: Vec<IDRange>,
}

impl IDRanges {
    pub fn from_file(fname: &str) -> Self {
        let data = read_to_string(fname).unwrap();
        let (_, ranges) = parse_range_file(data.as_str()).unwrap();

        // Ensure that the end length is at most 1 greater than the start
        for range in ranges.iter() {
            assert!(range.start.to_string().len() <= range.end.to_string().len() + 1);
        }

        IDRanges { ranges }
    }
}

// Parsers
fn parse_idrange(input: &str) -> IResult<&str, IDRange> {
    let (rest, (start, end)) = separated_pair(u64, char('-'), u64).parse(input)?;
    Ok((rest, IDRange { start, end }))
}

fn parse_range_file(input: &str) -> IResult<&str, Vec<IDRange>> {
    let (rest, ranges) =
        terminated(separated_list1(char(','), parse_idrange), opt(line_ending)).parse(input)?;
    Ok((rest, ranges))
}

#[cfg(test)]
mod tests {
    use super::IDRange;
    use super::IDRanges;
    use super::parse_idrange;

    #[test]
    fn test_parse_idrange() {
        let input = "1123-2222";
        let expected = IDRange {
            start: 1123,
            end: 2222,
        };

        assert_eq!(parse_idrange(input).unwrap().1, expected);
    }

    #[test]
    fn test_id_ranges() {
        let idranges = IDRanges::from_file("input/day02_sample.txt");
        // let idranges = IDRanges::from_file("input/day02_input.txt");

        assert_eq!(
            idranges.ranges.first().unwrap(),
            &IDRange { start: 11, end: 22 }
        );

        assert_eq!(
            idranges.ranges.last().unwrap(),
            &IDRange {
                start: 2121212118,
                end: 2121212124
            }
        );

        todo!("Ensure the whole range or part of it is splittable in n (2) and
        check the possible number starting with 1-9 whether they can make up
        numbers in the series if repeated");
    }
}
