use aoc::days::*;
use rstest::rstest;

#[test]
pub fn test_read_input() {
    let grid = day03::read_input("resources/day03_sample.txt");
    assert!(grid.len() > 1);

    assert_eq!(grid[0][1], '6');
    assert_eq!(grid[1][3], '*');
}

#[test]
pub fn test_parse_grid() {
    // Check if the lines can be parsed successfully in numbers and symbols
    let lines = day03::read_input("resources/day03_sample.txt");

    let grid = day03::Grid::from_lines(lines);

    assert_eq!(grid.numbers[1].value, 114);
    assert_eq!(grid.symbols[1].cell, (3, 6));
}

#[test]
pub fn test_is_valid_part() {
    let lines = day03::read_input("resources/day03_sample.txt");
    //let lines = day03::read_input("resources/day03_input.txt");

    let grid = day03::Grid::from_lines(lines);

    let valid_parts = grid.valid_parts();

    assert!(valid_parts.contains(&35));
    assert!(!valid_parts.contains(&114));
    assert!(!valid_parts.contains(&58));
}

#[test]
pub fn test_sum_of_parts() {
    assert_eq!(day03::sum_of_parts("resources/day03_sample.txt"), 4361);
}
