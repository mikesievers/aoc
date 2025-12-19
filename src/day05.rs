//! Advent of Code 2025, Day 05
//! ```
//! use aoc::day05::Database;
//! let database = Database::from_file("input/day05_input.txt");
//! assert_eq!(database.nr_fresh(), 761);
//! assert_eq!(database.nr_total(), 345755049374932);
//! ```

use nom::{
    IResult, Parser,
    character::complete::{char, line_ending, u64},
    multi::{many1, separated_list1},
    sequence::separated_pair,
};
use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
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

    fn merge_first_range(&self, ranges: Vec<Range>) -> Option<Vec<Range>> {
        let nr_ranges_before = ranges.len();
        let mut merged_ranges = HashSet::new();
        let first = ranges[0].clone();
        // let mut joined = false;
        for other in &ranges[1..] {
            if first.start <= other.end && first.end >= other.start {
                println!("Merging {:?} and {:?}", first, other);
                merged_ranges.insert(Range {
                    start: [first.start, other.start].iter().min().unwrap().clone(),
                    end: [first.end, other.end].iter().max().unwrap().clone(),
                });
                println!("Merged ranges: {:?}", merged_ranges);
                // joined = true;
            } else {
                merged_ranges.insert(other.clone());
            }
            merged_ranges.insert(first);
        }
        match nr_ranges_before - merged_ranges.len() {
            0 => None,
            _ => Some(merged_ranges.into_iter().collect()),
        }
    }

    pub fn nr_total(&self) -> u64 {
        // Determine the total number of fresh items by counting
        // the items within the ranges.
        // For this, the ranges need to be merged when necessary
        let mut merged_ranges: HashSet<Range> = HashSet::from_iter(self.ranges.iter().cloned());

        'mainloop: loop {
            for this in merged_ranges.clone() {
                for other in merged_ranges.clone() {
                    if this == other {
                        continue;
                    }
                    if this.start <= other.end && this.end >= other.start {
                        // println!("Merging {:?} and {:?}", this, other);
                        merged_ranges.remove(&this);
                        merged_ranges.remove(&other);
                        merged_ranges.insert(Range {
                            start: [this.start, other.start].iter().min().unwrap().clone(),
                            end: [this.end, other.end].iter().max().unwrap().clone(),
                        });

                        // println!(" Now: {:?}", merged_ranges);
                        // Whenever something has been merged, restart the loop
                        continue 'mainloop;
                    }
                }
            }
            break;
        }

        merged_ranges.iter().fold(0, |mut acc, range| {
            println!("{} - {}", range.start, range.end);
            acc += range.end - range.start + 1;
            acc
        })
    }

    // pub fn nr_total_NOT_WORKING(&self) -> u64 {
    //     // Determine the total number of fresh items by counting
    //     // the items within the ranges.
    //     // For this, the ranges need to be merged when necessary
    //     let mut merged_ranges = self.ranges.clone();
    //
    //     'mainloop: loop {
    //         // Iterate through the ranges until one is merged, then start again
    //         for idx in 0..merged_ranges.len() - 2 {
    //             if let Some(new_merged_rest) = self.merge_first_range(merged_ranges[idx..].to_vec())
    //             {
    //                 match idx {
    //                     0 => merged_ranges = new_merged_rest,
    //                     n => {
    //                         merged_ranges = merged_ranges[0..n - 1].to_vec();
    //                         merged_ranges.append(&mut new_merged_rest.clone());
    //                     }
    //                 }
    //                 // Restart the merging process, because something has been merged
    //                 continue 'mainloop;
    //             }
    //         }
    //         break;
    //     }
    //
    //     merged_ranges.iter().fold(0, |mut acc, range| {
    //         println!("{} - {}", range.start, range.end);
    //         acc += range.end - range.start + 1;
    //         acc
    //     })
    // }
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

        assert_eq!(database.ranges[3].end, 18);
        assert_eq!(database.inventory[0], 1);
        assert_eq!(database.inventory[5], 32);

        assert_eq!(database.nr_fresh(), 3);

        assert_eq!(database.nr_total(), 14);
    }
}
