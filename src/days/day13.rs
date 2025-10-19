use std::{collections::HashSet, fs::read_to_string};

use nom::{
    IResult, Parser,
    character::complete::{line_ending, one_of},
    combinator::{map, opt},
    multi::{many1, separated_list1},
    sequence::terminated,
};

//Main Structures

#[derive(PartialEq, Debug)]
struct Map {
    data: Vec<Vec<char>>,
}

#[derive(PartialEq, Debug)]
pub struct Atlas {
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

    assert_eq!(find_horizontal_symmetry(&atlas.maps[0]), Some(5));
    assert_eq!(find_vertical_symmetry(&atlas.maps[1]), Some(4));

    assert_eq!(atlas.summarize(), 405);
}
// Main methods
impl Atlas {
    pub fn summarize(&self) -> usize {
        let mut summary = 0;
        let _: Vec<_> = self
            .maps
            .iter()
            .map(|map| {
                if let Some(n) = find_horizontal_symmetry(map) {
                    summary += n;
                }
                if let Some(n) = find_vertical_symmetry(map) {
                    summary += 100 * n;
                }
            })
            .collect();

        summary
    }
}

// Processing

// Part 2

impl Atlas {
    pub fn summarize_2(&self) -> usize {
        let mut summary = 0;

        // Markers to track matches axis, init to impossible values
        let mut horiz_axis = 1000000000;
        let mut vert_axis = 1000000000;

        let _: Vec<_> = self
            .maps
            .iter()
            .map(|map| {
                if let Some(n) = find_horizontal_symmetry(map) {
                    horiz_axis = n;
                }
                if let Some(n) = find_vertical_symmetry(map) {
                    vert_axis = n;
                }
                // Find symmetries with distance 1, but not original axis
                if let Some(n) = find_horizontal_symmetry_2(map, horiz_axis) {
                    summary += n;
                }
                if let Some(n) = find_vertical_symmetry_2(map, vert_axis) {
                    summary += 100 * n;
                }
            })
            .collect();

        summary
    }
}

fn find_vertical_symmetry_2(map: &Map, not_axis: usize) -> Option<usize> {
    // Flip the map
    let rows = map.data.len();
    let cols = map.data[0].len();

    let mut transposed = vec![vec!['.'; rows]; cols];

    for y in 0..rows {
        for x in 0..cols {
            transposed[x][y] = map.data[y][x]
        }
    }

    find_horizontal_symmetry_2(&Map { data: transposed }, not_axis)
}

fn find_horizontal_symmetry_2(map: &Map, not_axis: usize) -> Option<usize> {
    // Determine all possible axes for each line
    let axes = map
        .data
        .iter()
        .map(|line| {
            let mut axis_cand = HashSet::new();
            for n in 1..(line.len()) {
                if n == not_axis {
                    continue;
                } // Can't reuse the old axis
                if is_symmetric_after_by(line, n, 1) {
                    axis_cand.insert(n);
                }
            }
            axis_cand
        })
        .collect::<Vec<HashSet<usize>>>();

    // Find if one axis is common to all lines
    let common_axis = axes.iter().skip(1).fold(axes[0].clone(), |acc, e| {
        acc.intersection(e).map(|&x| x).collect()
    });

    if common_axis.is_empty() {
        return None;
    }
    if common_axis.len() == 1 {
        return common_axis.iter().next().copied();
    }
    panic!("More than one folding axis found.")
}

#[test]
fn test_atlas_2() {
    let atlas = Atlas::from_file("resources/day13_sample.txt");

    assert_eq!(find_vertical_symmetry_2(&atlas.maps[0], 1000000), Some(3));
    assert_eq!(find_vertical_symmetry_2(&atlas.maps[1], 3), Some(1));

    assert_eq!(atlas.summarize_2(), 400);
}

fn is_symmetric_after_by(line: &Vec<char>, n: usize, distance: usize) -> bool {
    let delta: usize = line[0..n]
        .iter()
        .rev()
        .zip(&line[n..])
        .map(|(&a, &b)| match a == b {
            true => 0,
            false => 1,
        })
        .sum();

    delta == distance
}

#[test]
fn test_is_symmetric_after_by() {
    assert_eq!(
        is_symmetric_after_by(&"#..##...".chars().collect(), 4, 1),
        true
    );
    assert_eq!(
        is_symmetric_after_by(&"##..##".chars().collect(), 3, 0),
        true
    );
    assert_eq!(
        is_symmetric_after_by(&"##..##".chars().collect(), 3, 1),
        false
    );
    assert_eq!(
        is_symmetric_after_by(&"#.##..##.".chars().collect(), 5, 1),
        false
    );
    assert_eq!(
        is_symmetric_after_by(&"#.##..#..".chars().collect(), 5, 1),
        true
    );
}

// Part 1
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

fn find_vertical_symmetry(map: &Map) -> Option<usize> {
    // Flip the map
    let rows = map.data.len();
    let cols = map.data[0].len();

    let mut transposed = vec![vec!['.'; rows]; cols];

    for y in 0..rows {
        for x in 0..cols {
            transposed[x][y] = map.data[y][x]
        }
    }

    find_horizontal_symmetry(&Map { data: transposed })
}

fn find_horizontal_symmetry(map: &Map) -> Option<usize> {
    // Determine all possible axes for each line
    let axes = map
        .data
        .iter()
        .map(|line| {
            let mut axis_cand = HashSet::new();
            for n in 1..(line.len()) {
                if is_symmetric_after(line, n) {
                    axis_cand.insert(n);
                }
            }
            axis_cand
        })
        .collect::<Vec<HashSet<usize>>>();

    // Find if one axis is common to all lines
    let common_axis = axes.iter().skip(1).fold(axes[0].clone(), |acc, e| {
        acc.intersection(e).map(|&x| x).collect()
    });

    if common_axis.is_empty() {
        return None;
    }
    if common_axis.len() == 1 {
        return common_axis.iter().next().copied();
    }
    panic!("More than one folding axis found.")
}

#[test]
fn test_find_horizontal_symmetry() {
    let map = map_parser("#..\r\n###").unwrap().1;
    assert_eq!(is_symmetric_after(&map.data[0], 2), true);
    assert_eq!(is_symmetric_after(&map.data[1], 2), true);
    assert_eq!(find_horizontal_symmetry(&map), Some(2));

    let map = map_parser("...\r\n..#").unwrap().1;
    assert_eq!(find_horizontal_symmetry(&map), Some(1));
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
