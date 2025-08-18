use std::fs::*;
use std::io::{BufRead, BufReader};

pub fn read_input(fname: &str) -> Vec<String> {
    let file = File::open(fname).unwrap();
    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .map(|line| String::from(line.unwrap().trim()))
        .collect();

    lines
}
