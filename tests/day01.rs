use aoc::days::*;
use rstest::rstest;

#[test]
fn test_read_input() {
    let lines = day01::read_input("src/days/day01_sample.txt");

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
    case("src/days/day01_sample.txt", 142),
    case("src/days/day01_input.txt", 56465)
)]
#[test]
fn test_sum_calibration_values(fname: &str, expected: u32) {
    assert_eq!(day01::sum_calibration_values(fname), expected);
}
