//! # AOC 2023 Day 1
//!
//! <https://adventofcode.com/2023/day/1>
//!
//! Determine calibration values from a text input by forming
//! double digit numbers from the first and last digit on each line.
//!
//! Sample input:
//!
//! 1abc2
//! pqr3stu8vwx
//! a1b2c3d4e5f
//! treb7uchet
//!
//! Final Result Part 1:
//! ```
//! use aoc::days::day01;
//! assert_eq!(day01::sum_calibration_values("resources/day01_input.txt"), 56465);
//! ```
//!
//! The calibration values are: 12, 38, 15, 77. Adding them results in 142.
//!

use std::{fs::File, io::BufRead, io::BufReader};

pub fn read_input(fname: &str) -> Vec<String> {
    let file = File::open(fname).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map(|line| String::from(line.unwrap()))
        .collect();

    lines
}

pub fn extract_calibration_value(input: &str) -> Option<u32> {
    let mut first_digit_iter = input.chars().filter(|c| c.is_ascii_digit());

    let first_digit_char = first_digit_iter.next()?;
    let last_digit_char = first_digit_iter.last().unwrap_or(first_digit_char);

    let first_digit = first_digit_char.to_digit(10)?;
    let last_digit = last_digit_char.to_digit(10)?;

    Some(first_digit * 10 + last_digit)
}

pub fn sum_calibration_values(fname: &str) -> u32 {
    let lines = read_input(fname);
    lines
        .iter()
        .map(|line| extract_calibration_value(line).unwrap())
        .sum()
}
