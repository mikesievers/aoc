//! Advent of Code 2025 day 03
//! ```
//! use aoc::day03::Banklist;
//!
//! let banklist = Banklist::from_file("input/day03_input.txt");
//! assert_eq!(banklist.sum_joltages(), 17412);
//! ```
use std::fs::read_to_string;

pub fn max_joltage(input: &str) -> u32 {
    let mut digits = Vec::new();
    for ch in input.chars() {
        if let Some(digit) = ch.to_digit(10) {
            digits.push(digit);
        }
    }

    let max_val = digits.iter().max().unwrap();
    let max_pos = digits.iter().position(|&x| x == *max_val).unwrap();

    let max_before_option = digits[0..max_pos].iter().max();

    // If the maximum value is at the last position, find the highest digit before it
    // And we no there has to be a digit before it, therefore we can unwrap()
    if max_pos == digits.len() - 1 {
        return max_before_option.unwrap() * 10 + max_val;
    }

    let max_after = digits[max_pos + 1..].iter().max().unwrap();

    max_val * 10 + max_after
}

pub struct Banklist {
    pub banks: Vec<String>,
}

impl Banklist {
    pub fn from_file(fname: &str) -> Self {
        let data = read_to_string(fname).unwrap();

        let banks = data.lines().map(|x| x.to_string()).collect();
        Banklist { banks }
    }

    pub fn sum_joltages(&self) -> u32 {
        self.banks.iter().fold(0, |mut acc, x| {
            acc += max_joltage(x.as_str());
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::day03::Banklist;

    use super::max_joltage;
    use rstest::rstest;

    #[rstest]
    #[case("987654321111111", 98)]
    #[case("811111111111119", 89)]
    #[case("234234234234278", 78)]
    #[case("818181911112111", 92)]
    fn test_max_joltage(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(max_joltage(input), expected);
    }

    #[test]
    fn test_banklist() {
        let banklist = Banklist::from_file("input/day03_sample.txt");
        assert_eq!(banklist.sum_joltages(), 357);
    }
}
