use std::fs::read_to_string;

use nom::{
    IResult, Parser,
    character::complete::{newline, one_of},
    combinator::map,
    multi::{SeparatedList1, many1, separated_list1},
    sequence::terminated,
};

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

        atlas_parser.parse(&input).unwrap().1
    }
}

#[test]
fn test_atlas() {
    let atlas = Atlas::from_file("resources/day13_sample.txt");

    assert_eq!(atlas.maps.len(), 2);
}

// Parsing
fn map_line_parser(input: &str) -> IResult<&str, Vec<char>> {
    terminated(many1(one_of(".#")), newline).parse(input)
}

#[test]
fn test_map_line() {
    assert_eq!(
        map_line_parser.parse(".#.\n").unwrap(),
        ("", vec!['.', '#', '.'])
    );
}

fn map_parser(input: &str) -> IResult<&str, Map> {
    map(terminated(many1(map_line_parser), newline), |data| Map { data }).parse(input)
}

fn atlas_parser(input: &str) -> IResult<&str, Atlas> {
    map(many1(map_parser), |maps| Atlas { maps }).parse(input)
}
