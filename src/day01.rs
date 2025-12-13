//! Advent of Code day01
//! ```
//! use aoc::day01::{Rotations, Dial};
//!
//! let rotations = Rotations::from_file("input/day01_input.txt");
//! let mut dial = Dial::new();
//! dial.perform_rotations(&rotations);
//!
//! // Part 1
//! assert_eq!(dial.get_password(), 1078);
//! // Part 2
//! assert_eq!(dial.get_password_with_method(), 6412);
//! ```
//!
use std::fs::read_to_string;

use nom::IResult;
use nom::Parser;
use nom::character::complete::line_ending;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::terminated;

#[derive(PartialEq, Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(PartialEq, Debug)]
pub struct Rotation {
    direction: Direction,
    distance: i32,
}

pub struct Rotations {
    rotations: Vec<Rotation>,
}

impl Rotations {
    pub fn from_file(fname: &str) -> Self {
        let lines = read_to_string(fname).unwrap();

        let (_, rotations) = parse_rotation_file(lines.as_str()).unwrap();

        Rotations { rotations }
    }
}

pub struct Dial {
    position: i32,
    nr_zeroes: u32,
    nr_zero_passes: i32,
}

impl Default for Dial {
    fn default() -> Self {
        Self::new()
    }
}

impl Dial {
    pub fn new() -> Self {
        Dial {
            position: 50,
            nr_zeroes: 0,
            nr_zero_passes: 0,
        }
    }

    pub fn rotate(&mut self, rotation: &Rotation) {
        // Adjust the position
        match rotation.direction {
            Direction::Left => {
                // Make sure starting from 0 is not counted as a double zero
                if self.position == 0 && rotation.distance != 0 {
                    self.nr_zero_passes -= 1;
                }
                self.position -= rotation.distance
            }
            Direction::Right => self.position += rotation.distance,
        }

        // Count how many times 0 has been passed
        while self.position > 100 {
            self.position -= 100;
            self.nr_zero_passes += 1;
        }
        // If it's exactly 100, decrease to 0 but do not count - it will be
        // counted as zero below
        if self.position == 100 {
            self.position = 0;
        }

        while self.position < 0 {
            self.position += 100;
            // Don't count as pass if it stops on negative 0
            self.nr_zero_passes += 1;
        }

        // Count as zero if the dial stops exactly on 0
        if self.position == 0 {
            self.nr_zeroes += 1
        }
    }

    // Password for part 1
    pub fn get_password(&self) -> u32 {
        self.nr_zeroes
    }

    // Password for part 2
    pub fn get_password_with_method(&self) -> u32 {
        self.nr_zeroes + self.nr_zero_passes as u32
    }

    pub fn perform_rotations(&mut self, rotations: &Rotations) {
        for rotation in rotations.rotations.iter() {
            self.rotate(rotation);
        }
    }
}

// Parsers
fn parse_rotation(input: &str) -> IResult<&str, Rotation> {
    // Parse e.g. "L68\n"
    let (input, direction) = nom::branch::alt((
        nom::character::complete::char('L').map(|_| Direction::Left),
        nom::character::complete::char('R').map(|_| Direction::Right),
    ))
    .parse(input)?;

    let (input, distance) =
        terminated(nom::character::complete::i32, opt(line_ending)).parse(input)?;

    Ok((
        input,
        Rotation {
            direction,
            distance,
        },
    ))
}

fn parse_rotation_file(input: &str) -> IResult<&str, Vec<Rotation>> {
    many1(parse_rotation).parse(input)
}

#[cfg(test)]
mod tests {
    use super::Direction;
    use super::Rotation;

    use super::Dial;
    use super::Rotations;

    use super::parse_rotation;

    #[test]
    fn test_dial_rotations() {
        let rotations = Rotations::from_file("input/day01_sample.txt");

        assert_eq!(
            rotations.rotations[0],
            Rotation {
                direction: Direction::Left,
                distance: 68
            }
        );

        assert_eq!(
            rotations.rotations.last().unwrap(),
            &Rotation {
                direction: Direction::Left,
                distance: 82
            }
        );

        let mut dial = Dial::new();
        dial.perform_rotations(&rotations);
        assert_eq!(dial.get_password(), 3);
        assert_eq!(dial.nr_zero_passes, 3);
        assert_eq!(dial.get_password_with_method(), 6);
    }

    #[test]
    fn test_dial() {
        let mut dial = Dial::new();

        assert_eq!(dial.position, 50);

        dial.rotate(&Rotation {
            direction: Direction::Left,
            distance: 50,
        });

        assert_eq!(&dial.position, &0);
        assert_eq!(&dial.nr_zeroes, &1);
        assert_eq!(&dial.get_password(), &1);

        dial.rotate(&Rotation {
            direction: Direction::Left,
            distance: 3,
        });

        assert_eq!(&dial.position, &97);

        dial.rotate(&Rotation {
            direction: Direction::Right,
            distance: 5,
        });

        assert_eq!(&dial.position, &2);
    }

    #[test]
    fn test_parse_rotation() {
        let rotation_str = "L13\nR12";

        let (rest, rotation) = parse_rotation(rotation_str).unwrap();

        assert_eq!(
            rotation,
            Rotation {
                direction: Direction::Left,
                distance: 13
            }
        );

        assert_eq!(rest, "R12");
    }
}
