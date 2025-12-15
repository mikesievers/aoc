//! Advent of Code 2025 day 03
//! ```
//! use aoc::day03::Banklist;
//!
//! let banklist = Banklist::from_file("input/day03_input.txt");
//! assert_eq!(banklist.sum_joltages(), 17412);
//! assert_eq!(banklist.sum_unlimited_joltages(), 172681562473501);
//! ```
use std::fs::read_to_string;

// Part 1
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

// Part 2
pub fn max_unlimited_joltage(input: &str) -> u64 {
    // Exactly 12 digits are to be chosen
    // Iterate until only 12 are left:
    //   Find the digit whose removal yields the highest number and remove it
    let mut result = input.to_string();
    while result.len() > 12 {
        result = remove_less_significant_digit(result);
    }

    result.parse().unwrap()
}

fn remove_less_significant_digit(digits: String) -> String {
    let chars: Vec<char> = digits.chars().collect();
    for idx in 0..chars.len() - 1 {
        if chars[idx] < chars[idx + 1] {
            let mut result = chars.clone();
            result.remove(idx);
            return result.into_iter().collect();
        }
    }
    chars[0..chars.len() - 1].into_iter().collect()
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

    pub fn sum_unlimited_joltages(&self) -> u64 {
        self.banks.iter().fold(0, |mut acc, x| {
            acc += max_unlimited_joltage(x.as_str());
            println!("{acc}");
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::day03::Banklist;

    use super::max_joltage;
    use super::max_unlimited_joltage;
    use super::remove_less_significant_digit;
    use rstest::rstest;

    // Part 1
    #[rstest]
    #[case("987654321111111", 98)]
    #[case("811111111111119", 89)]
    #[case("234234234234278", 78)]
    #[case("818181911112111", 92)]
    fn test_max_joltage(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(max_joltage(input), expected);
    }

    // Part 2
    #[rstest]
    #[case("987654321111111", 987654321111)]
    #[case("811111111111119", 811111111119)]
    #[case("234234234234278", 434234234278)]
    #[case("818181911112111", 888911112111)]
    fn test_max_unlimited_joltage(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(max_unlimited_joltage(input), expected);
    }

    #[rstest]
    #[case("181", "81")]
    #[case("118", "18")]
    #[case("818181911112111", "88181911112111")]
    #[case("118", "18")]
    fn test_remove_less_significant_digit(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(
            remove_less_significant_digit(input.to_string()),
            expected.to_string()
        );
    }

    // Overall
    #[test]
    fn test_banklist() {
        let banklist = Banklist::from_file("input/day03_sample.txt");
        assert_eq!(banklist.sum_joltages(), 357);
        assert_eq!(banklist.sum_unlimited_joltages(), 3121910778619);
    }
}
