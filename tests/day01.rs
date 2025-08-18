use aoc::days::*;
use rstest::rstest;

#[test]
fn test_read_input() {
    let lines = day01::read_input("resources/day01_sample.txt");

    assert!(lines.len() > 1);
}

#[rstest(
    input,
    expected,
    case("1abc2", 12),
    case("pqr3stu8vwx", 38),
    case("a1b2c3d4e5f", 15),
    case("treb7uchet", 77)
)]
#[test]
fn test_extract_calibration_value(input: &str, expected: u32) {
    assert_eq!(day01::extract_calibration_value(input), Some(expected));
}

#[rstest(
    fname,
    expected,
    case("resources/day01_sample.txt", 142),
    case("resources/day01_input.txt", 56465)
)]
#[test]
fn test_sum_calibration_values(fname: &str, expected: u32) {
    assert_eq!(day01::sum_calibration_values(fname), expected);
}

/// Part 2: Some text needs to be preprocessed:
/// 'It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".'
#[rstest(
    line_in,
    expected,
    case("three4dckqnone2two1eightwoq", "34dckqnone2two1eigh2q"),
    case("threethreefour", "3three4"),
    case("threefourfour", "3four4"),
    case("asone34five", "as1345"),
    case("Idotwoefive7", "Ido2efive7"),
    case("eightwothree", "8wo3"),
    case("nine", "9"),
    case("two1nine", "219"),
    case("eightwothree", "8wo3"),
    case("abcone2threexyz", "abc123xyz"),
    case("xtwone3four", "x2ne34"),
    case("4nineeightseven2", "4nineeightseven2"),
    case("zoneight234", "z1ight234"),
    case("7pqrstsixteen", "7pqrst6teen"),
    case("fpnine1", "fp91")
)]
fn test_words_to_digits(line_in: &str, expected: &str) {
    assert_eq!(
        day01::words_to_digits(String::from(line_in)),
        String::from(expected)
    );
}

#[test]
fn test_sum_subbed_calibration_values() {
    let fname = "resources/day01_sample2.txt";
    assert_eq!(day01::sum_subbed_calibration_values(fname), 281);
}
