use std::fs::*;
use std::io::{BufRead, BufReader};

// A number represents a number found on the grid
// and it knows the cells it comes from (y(down), x(right))
struct Number {
    id: usize,
    value: i32,
    cells: Vec<(i32, i32)>,
}

// A symbol is any char of the grid that is not
// an numeric digit or period '.'
struct Symbol {
    id: usize,
    character: char,
    cell: (i32, i32),
}

struct Grid {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

pub fn read_input(fname: &str) -> Vec<Vec<char>> {
    let file = File::open(fname).unwrap();
    let reader = BufReader::new(file);

    let grid = reader
        .lines()
        .map(|line| line.unwrap().trim().chars().collect())
        .collect();
    grid
}
