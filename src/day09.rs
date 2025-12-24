//! Advent of Code 2025 day 09
//! ```
//! use aoc::day09::Floor;
//! let floor = Floor::from_file("input/day09_input.txt");
//! assert_eq!(floor.largest_rect(), 4777824480);
//! ```

use std::fs::read_to_string;

use glam::IVec2;
use itertools::Itertools;
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

pub struct Floor {
    red_tiles: Vec<IVec2>,
}

impl Floor {
    pub fn from_file(fname: &str) -> Self {
        let data = read_to_string(fname).unwrap();
        let (_, red_tiles) = parse_tiles(data.as_str()).unwrap();
        Floor { red_tiles }
    }

    pub fn largest_rect(&self) -> u64 {
        self.red_tiles
            .iter()
            .tuple_combinations()
            .map(|(p1, p2)| area_plus1(p1, p2))
            .max()
            .unwrap()
    }

    pub fn largest_rg_rect(&self) -> u64 {
        // It is a LOOP, i.e. no intersections assumed
        // Given two red tiles
        // find min, max x and min, max y
        // Start at top left
        // If any red tile is between the extrama (not equal to)
        //  then the square is not a candidate
        self.red_tiles
            .iter()
            .tuple_combinations()
            .filter(|(p1, p2)| {
                // ensure that no red tile is in the area between these tiles
                let x_min = p1.x.min(p2.x);
                let x_max = p1.x.max(p2.x);
                let y_min = p1.y.min(p2.y);
                let y_max = p1.y.max(p2.y);
                self.red_tiles.iter().all(|other| {
                    // Filter all nodes inside the squere
                    !((other.x > x_min)
                        && (other.y > y_min)
                        && (other.x < x_max)
                        && (other.y < y_max))
                    // And then filter all nodes that are on
                    // connecting lines, but on the wrong side
                    // && (other.x == x_max && other.x>)
                })
            })
            .inspect(|p| println!("{:?}", p))
            .map(|(p1, p2)| area_plus1(p1, p2))
            .inspect(|a| println!("  area {:?}", a))
            .max()
            .unwrap()
    }
}

fn parse_tiles(input: &str) -> IResult<&str, Vec<IVec2>> {
    let (rest, tuples) = separated_list1(
        line_ending,
        separated_pair(complete::i32, tag(","), complete::i32),
    )
    .parse(input)?;

    let tiles = tuples
        .into_iter()
        .map(|(x, y)| IVec2 { x, y })
        .collect::<Vec<_>>();

    Ok((rest, tiles))
}

fn area_plus1(p1: &IVec2, p2: &IVec2) -> u64 {
    (((p1.x - p2.x).abs() + 1) as u64) * ((p1.y - p2.y).abs() + 1) as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_floor() {
        let floor = Floor::from_file("input/day09_sample.txt");

        assert_eq!(floor.red_tiles[0], IVec2 { x: 7, y: 1 });
        assert_eq!(
            floor.red_tiles.iter().last().unwrap(),
            &IVec2 { x: 7, y: 3 }
        );

        assert_eq!(floor.largest_rect(), 50);
        assert_eq!(floor.largest_rg_rect(), 24);
    }

    #[rstest]
    #[case (IVec2 {x:2, y:5}, IVec2 {x:11, y:1}, 50)]
    #[case (IVec2 {x:7, y:3}, IVec2 {x:2, y:3}, 6)]
    #[case (IVec2 {x:7, y:1}, IVec2 {x:11, y:7}, 35)]
    fn test_area_plus1(#[case] p1: IVec2, #[case] p2: IVec2, #[case] expected: u64) {
        assert_eq!(area_plus1(&p1, &p2), expected);
    }
}
