// Process files like the sample:
// Time:      7  15   30
// Distance:  9  40  200

use nom::bytes::complete::tag;
use nom::character::complete::{i64, line_ending, space0, space1};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded};
use nom::{IResult, Parser};
use std::fs::read_to_string;

struct RaceLog {
    time: Vec<i64>,
    distance: Vec<i64>,
}

impl RaceLog {
    pub fn from_file(fname: &str) -> Self {
        let data = read_to_string(fname).unwrap();

        let (_, (time, distance)) = (time_line, distance_line).parse(data.as_str()).unwrap();

        Self { time, distance }
    }
}

// <space>*<i64><space>+<i64>...<space>*<line_ending>
fn number_line(input: &str) -> IResult<&str, Vec<i64>> {
    delimited(space1, separated_list1(space1, i64), space0).parse(input)
}

fn time_line(input: &str) -> IResult<&str, Vec<i64>> {
    delimited(tag("Time:"), number_line, line_ending).parse(input)
}

fn distance_line(input: &str) -> IResult<&str, Vec<i64>> {
    delimited(tag("Distance:"), number_line, line_ending).parse(input)
}

#[test]
fn test_parser() {
    let numbers = "  3  2  1 \n";
    let (_, res) = number_line.parse(numbers).unwrap();
    assert_eq!(res, [3, 2, 1]);

    let timeline = "Time:   3    5 3 \n";
    let (_, res) = time_line.parse(timeline).unwrap();
    assert_eq!(res, [3, 5, 3]);
}

#[test]
fn test_from_file() {
    let race_log = RaceLog::from_file("resources/day06_sample.txt");

    assert_eq!(race_log.time, [7, 15, 30]);
    assert_eq!(race_log.distance, [9, 40, 200]);
}
