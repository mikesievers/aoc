use std::fs::read_to_string;

use nom::Parser;
use nom::character::complete::{i64, line_ending};
use nom::{IResult, character::complete::space1, multi::separated_list1};

// Structs
struct Report {
    histories: Vec<Vec<i64>>,
}

impl Report {
    pub fn from_file(fname: &str) -> Self {
        let data = read_to_string(fname).unwrap();
        let (_, histories) = number_line_file.parse(data.as_str()).unwrap();
        Report { histories }
    }
}

#[test]
fn test_report(){
    let report = Report::from_file("resources/day09_sample.txt");
    
    assert_eq!(report.histories.len(), 3);
    assert_eq!(report.histories[1], vec![1_i64, 3_i64, 6_i64, 10_i64, 15_i64, 21_i64]);
}

// Parsers
fn number_line(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, i64).parse(input)
}

fn number_line_file(input: &str) -> IResult<&str, Vec<Vec<i64>>>{
    separated_list1(line_ending, number_line).parse(input)
}

#[test]
fn test_number_line() {
    let nline = "1 2 3 4";
    assert_eq!(
        number_line.parse(nline).unwrap(),
        ("", vec![1_i64, 2_i64, 3_i64, 4_i64])
    );
}

#[test]
fn test_number_line_file(){
    let data = read_to_string("resources/day09_sample.txt");



}