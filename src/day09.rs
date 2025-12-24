//! Advent of Code 2025 day 09
//! ```
//! use aoc::day09::Floor;
//! let floor = Floor::from_file("input/day09_input.txt");
//! assert_eq!(floor.largest_rect(), 4777824480);
//! assert_eq!(floor.largest_rg_rect(), 1542119040); // too low
//! ```

use std::fs::read_to_string;

use geo::Area;
use geo::BooleanOps;
use geo::LineString;
use geo::MultiPolygon;
use geo::Polygon;
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
        // Use the geo crate to define a polygon for the loop
        let lstring = self
            .red_tiles
            .iter()
            .map(|p| geo::Coord {
                x: p.x as f64,
                y: p.y as f64,
            })
            .collect_vec();
        let rg_area = Polygon::new(LineString::from(lstring), vec![]);

        // For all possible areas, filter those that lie completely inside the loop
        // Then find the maximum area
        self.red_tiles
            .iter()
            .tuple_combinations()
            .filter(|(p1, p2)| {
                // determine coordindates of candidate rect
                let x_min = p1.x.min(p2.x);
                let x_max = p1.x.max(p2.x);
                let y_min = p1.y.min(p2.y);
                let y_max = p1.y.max(p2.y);
                let candidate = Polygon::new(
                    LineString::from(vec![
                        (x_min as f64, y_min as f64),
                        (x_min as f64, y_max as f64),
                        (x_max as f64, y_max as f64),
                        (x_max as f64, y_min as f64),
                    ]),
                    vec![],
                );
                let intersection: MultiPolygon<f64> = candidate.intersection(&rg_area);
                (candidate.unsigned_area() - intersection.unsigned_area()).abs() < 0.01
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
