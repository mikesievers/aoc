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
//! Final Result Part 2:
//! ```
//! use aoc::days::day01;
//! assert_eq!(day01::sum_subbed_calibration_values("resources/day01_input.txt"), 55902);
//! ```
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

// Part 2
// Replace spelled out digits in the text by actual digits
// Take care to replace the first instance from the left
// And the last instance from the right (or any remaining)
//
// This is probably too complicated, but it works.
//
pub fn words_to_digits(mut line: String) -> String {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    // create a reference value symbolizing that the digit has not been found
    let idx_not_found = line.len();

    // if a decimal digit exist in the string, find the position of the first digit
    let first_digit_idx = match line.char_indices().find(|(_, c)| c.is_ascii_digit()) {
        Some((idx, _)) => idx,
        None => idx_not_found,
    };

    // Check if a spelled out digit is contained at the left of the string
    let mut digit_idx_to_replace = idx_not_found;
    let mut written_digit_idx_found = idx_not_found;
    for (idx, digit) in digits.iter().enumerate() {
        let digit_found_left_idx = line.find(digit).unwrap_or(idx_not_found);
        if (digit_found_left_idx < written_digit_idx_found)
            && (digit_found_left_idx < first_digit_idx)
        {
            digit_idx_to_replace = idx;
            written_digit_idx_found = digit_found_left_idx;
        }
    }

    // replace the leftmost spelled out digit (if any) with the decimal digit
    if written_digit_idx_found < idx_not_found {
        line = line.replacen(
            digits[digit_idx_to_replace],
            (digit_idx_to_replace + 1).to_string().as_str(),
            1,
        );
    };

    // Repeat all this for the rightmost number
    // if a decimal digit exist in the string, find the position of the first digit
    let idx_not_found_right: i32 = -1;
    let last_digit_idx = match line.char_indices().rfind(|(_, c)| c.is_ascii_digit()) {
        Some((idx, _)) => idx as i32,
        None => idx_not_found_right,
    };

    // Check if a spelled out digit is contained at the right of the string
    let mut digit_idx_to_replace = idx_not_found_right;
    let mut written_digit_idx_found_right = idx_not_found_right;
    for (idx, digit) in digits.iter().enumerate() {
        if let Some(written_idx) = line.rfind(digit) {
            if (written_idx as i32 > written_digit_idx_found_right)
                && (written_idx as i32 > last_digit_idx)
            {
                written_digit_idx_found_right = written_idx as i32;
                digit_idx_to_replace = idx as i32;
            }
        }
    }

    // replace the rightmost spelled out digit (if any) with the decimal digit
    if written_digit_idx_found_right > idx_not_found_right {
        let target = digits[digit_idx_to_replace as usize];
        let replacement = (digit_idx_to_replace + 1).to_string();
        if let Some(pos) = line.rfind(target) {
            line = format!(
                "{}{}{}",
                &line[..pos],
                replacement,
                &line[pos + target.len()..]
            );
        }
    };

    line
}

pub fn sum_subbed_calibration_values(fname: &str) -> u32 {
    let lines = read_input(fname);
    lines
        .iter()
        .map(|line| words_to_digits(line.to_string()))
        .map(|line| extract_calibration_value(&line).unwrap())
        .sum()
}
