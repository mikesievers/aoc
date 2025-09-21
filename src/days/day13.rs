use std::fs::read_to_string;

use nom::{
    IResult, Parser,
    character::complete::{line_ending, newline, one_of},
    combinator::{map, opt},
    multi::{SeparatedList1, many1, separated_list1},
    sequence::terminated,
};

//Main Structures

#[derive(PartialEq, Debug)]
struct Map {
    data: Vec<Vec<char>>,
}

#[derive(PartialEq, Debug)]
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

// Processing

fn is_symmetric_after(line: &Vec<char>, n: usize) -> bool {
    line[0..n]
        .iter()
        .rev()
        .zip(&line[n..])
        .all(|(&a, &b)| a == b)
}

#[test]
fn test_is_symmetric_after() {
    assert_eq!(is_symmetric_after(&"#.##..##.".chars().collect(), 3), false);
    assert_eq!(is_symmetric_after(&"#.##..##.".chars().collect(), 4), false);
    assert_eq!(is_symmetric_after(&"#.##..##.".chars().collect(), 5), true);
    assert_eq!(is_symmetric_after(&"#.##..##.".chars().collect(), 6), false);
}

// Parsing
fn map_line_parser(input: &str) -> IResult<&str, Vec<char>> {
    terminated(many1(one_of(".#")), opt(line_ending)).parse(input)
}

#[test]
fn test_map_line() {
    assert_eq!(
        map_line_parser.parse(".#.\r\n").unwrap(),
        ("", vec!['.', '#', '.'])
    );
}

fn map_parser(input: &str) -> IResult<&str, Map> {
    //map(terminated(many1(map_line_parser), line_ending), |data| {
    map(many1(map_line_parser), |data| Map { data }).parse(input)
}

#[test]
fn test_map_parser() {
    let map = map_parser(".#\r\n##\n\n..\r\n#.").unwrap().1;
    let expected = Map {
        data: vec![vec!['.', '#'], vec!['#', '#']],
    };
    assert_eq!(map, expected);
}

fn atlas_parser(input: &str) -> IResult<&str, Atlas> {
    map(separated_list1(line_ending, map_parser), |maps| Atlas {
        maps,
    })
    .parse(input)
}

#[test]
fn test_atlas_parser() {
    let atlas = atlas_parser.parse(".#\r\n##\n\n..\n#.").unwrap().1;
    let maps_expected = vec![
        Map {
            data: vec![vec!['.', '#'], vec!['#', '#']],
        },
        Map {
            data: vec![vec!['.', '.'], vec!['#', '.']],
        },
    ];
    assert_eq!(
        atlas,
        Atlas {
            maps: maps_expected
        }
    );
}
