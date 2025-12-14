//! Advent of Code day02
//! ```
//! use  aoc::day02::{IDRanges, sum_invalid_ids};
//!
//! let idranges = IDRanges::from_file("input/day02_input.txt");
//! assert_eq!(sum_invalid_ids(&idranges), 19605500130);
//! ```
use std::fs::read_to_string;

use nom::{
    IResult, Parser,
    character::complete::{char, line_ending, u64},
    combinator::opt,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
};

#[derive(PartialEq, Debug)]
pub struct IDRange {
    start: u64,
    end: u64,
}

pub struct IDRanges {
    // The ranges have the following features:
    // 1. start and end can have different lengths
    // 2. end length is at most one more than the start
    pub ranges: Vec<IDRange>,
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

// id considerations
fn is_invalid_id(id: u64) -> bool {
    let id_string = id.to_string();
    let id_len = id_string.len();

    if (id_len % 2) != 0 {
        return false;
    }

    if id_string[0..id_len / 2] == id_string[id_len / 2..] {
        true
    } else {
        false
    }
}

pub fn sum_invalid_ids(idranges: &IDRanges) -> u64 {
    idranges
        .ranges
        .iter()
        .fold(0, |mut sum_invalid_ids, range| {
            for id in range.start..=range.end {
                if is_invalid_id(id) {
                    sum_invalid_ids += id
                }
            }
            sum_invalid_ids
        })
}

#[cfg(test)]
mod tests {
    use super::is_invalid_id;

    use super::IDRange;
    use super::IDRanges;
    use super::parse_idrange;
    use super::sum_invalid_ids;

    use rstest::rstest;

    #[test]
    fn test_parse_idrange() {
        let input = "1123-2222";
        let expected = IDRange {
            start: 1123,
            end: 2222,
        };

        assert_eq!(parse_idrange(input).unwrap().1, expected);
    }

    #[rstest]
    #[case(11, true)]
    #[case(22, true)]
    #[case(99, true)]
    #[case(101, false)]
    #[case(1188511885, true)]
    #[case(222222, true)]
    #[case(1698522, false)]
    #[case(446446, true)]
    #[case(38593859, true)]
    fn test_is_invalid_id(#[case] input: u64, #[case] expected: bool) {
        assert_eq!(is_invalid_id(input), expected);
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

        assert_eq!(sum_invalid_ids(&idranges), 1227775554);
    }
}
