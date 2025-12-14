//! Advent of Code 2025 day 03

pub fn max_joltage(input: &str) -> u32 {
    98
}

#[cfg(test)]
mod tests {
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
}
