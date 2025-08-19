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
    let lines = day03::read_input("resources/day03_sample.txt");

    // NEXT:
    // * parse the lines
    // * construct a grid
    // * find numbers and record their cells
    // * find symbols and record their cells

    assert!(false);
}
