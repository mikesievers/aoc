//! Advent of Code 2025, Day 05
//! ```
//! use aoc::day05::Database;
//! let database = Database::from_file("input/day05_input.txt");
//! assert_eq!(database.nr_fresh(), 761);
//! ```

use nom::{
    IResult, Parser,
    character::complete::{char, line_ending, u64},
    multi::{many1, separated_list1},
    sequence::separated_pair,
};
use std::fs::read_to_string;

pub struct Range {
    start: u64,
    end: u64,
}

pub struct Database {
    ranges: Vec<Range>,
    inventory: Vec<u64>,
}

impl Database {
    pub fn from_file(fname: &str) -> Self {
        let data = read_to_string(fname).unwrap();

        let (_, database) = parse_database(data.as_str()).unwrap();

        database
    }

    pub fn nr_fresh(&self) -> usize {
        // Count the items that are fresh, i.e. inventory items contained in the ranges
        self.inventory
            .iter()
            .map(|&x| {
                let mut is_in_a_range = 0;
                for range in self.ranges.iter() {
                    if x >= range.start && x <= range.end {
                        is_in_a_range = 1;
                        break;
                    }
                }
                is_in_a_range
            })
            .sum()
    }
}

// Parsers
fn parse_database(input: &str) -> IResult<&str, Database> {
    let (rest, (ranges_raw, inventory)) = separated_pair(
        separated_list1(line_ending, separated_pair(u64, char('-'), u64)),
        many1(line_ending),
        separated_list1(line_ending, u64),
    )
    .parse(input)?;
    let ranges = ranges_raw
        .into_iter()
        .map(|x| Range {
            start: x.0,
            end: x.1,
        })
        .collect();
    Ok((rest, Database { ranges, inventory }))
}

#[cfg(test)]
mod tests {
    use super::Database;

    #[test]
    fn test_x() {
        let database = Database::from_file("input/day05_sample.txt");

        assert_eq!(database.ranges[0].start, 3);
        assert_eq!(database.ranges[3].end, 18);
        assert_eq!(database.inventory[0], 1);
        assert_eq!(database.inventory[5], 32);

        assert_eq!(database.nr_fresh(), 3);
    }
}
