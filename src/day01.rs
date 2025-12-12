use std::fs::read_to_string;

use nom::IResult;
use nom::Parser;
use nom::character::complete::line_ending;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::terminated;

#[derive(PartialEq, Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(PartialEq, Debug)]
struct Rotation {
    direction: Direction,
    distance: i32,
}

struct Rotations {
    rotations: Vec<Rotation>,
}

impl Rotations {
    pub fn from_file(fname: &str) -> Self {
        let lines = read_to_string(fname).unwrap();

        let (_, rotations) = parse_rotation_file(lines.as_str()).unwrap();

        Rotations { rotations }
    }
}

struct Dial {
    position: i32,
    nr_zeroes: u32,
}

impl Dial {
    pub fn new() -> Self {
        Dial {
            position: 50,
            nr_zeroes: 0,
        }
    }

    pub fn rotate(&mut self, rotation: &Rotation) {
        match rotation.direction {
            Direction::Left => self.position = (100 + self.position - rotation.distance) % 100,
            Direction::Right => self.position = (self.position + rotation.distance) % 100,
        }

        if self.position == 0 {
            self.nr_zeroes += 1
        }
    }

    pub fn get_password(&self) -> u32 {
        self.nr_zeroes
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
