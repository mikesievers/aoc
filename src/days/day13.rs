use std::fs::read_to_string;

use nom::{character::complete::{newline, one_of}, multi::many1, sequence::terminated, IResult, Parser};

struct Map {
    data: Vec<Vec<char>>,
}

struct Atlas {
    maps: Vec<Map>,
}

impl Atlas {
    pub fn from_file(fname: &str) -> Self {
        let input =
            read_to_string(fname).unwrap_or_else(|_| panic!("Could not read input file {fname}"));

        let maps = Vec::new();

        Atlas { maps }
    }
}

#[test]
fn test_atlas() {
    let atlas = Atlas::from_file("resources/day13_sample.txt");

    assert_eq!(atlas.maps.len(), 2);
}

fn map_line(input: &str) -> IResult<&str, Vec<char>> {
    terminated(many1(one_of(".#")), newline).parse(input)
}

#[test]
fn test_map_line() {
    assert_eq!(map_line.parse(".#.\n").unwrap(), ("", vec!['.', '#', '.']));
}