use std::fs::*;
use std::io::{BufRead, BufReader};

pub const INPUT: &str = "resources/day02_input.txt";
pub const SAMPLE: &str = "resources/day02_sample.txt";

pub fn read_input(fname: &str) -> Vec<String> {
    let file = File::open(fname).unwrap();
    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .map(|line| String::from(line.unwrap().trim()))
        .collect();

    lines
}
