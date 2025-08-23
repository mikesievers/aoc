//! Day 03:
//! ```
//! use aoc::days::day03;
//! assert_eq!(day03::sum_of_parts("resources/day03_input.txt"), 553825)
//! ```
//!
//! Part 2 only considers the stars to be valid gears.
//! The number sought is the sum of the products of the numbers connected by a gears
//!
use std::fs::*;
use std::io::{BufRead, BufReader};

// A number represents a number found on the grid
// and it knows the cells it comes from (y(down), x(right))
pub struct Number {
    pub id: usize,
    pub value: u32,
    pub cells: Vec<(usize, usize)>,
}

// A symbol is any char of the grid that is not
// an numeric digit or period '.'
pub struct Symbol {
    pub id: usize,
    pub character: char,
    pub cell: (usize, usize),
}

pub struct Grid {
    pub numbers: Vec<Number>,
    pub symbols: Vec<Symbol>,
}

impl Grid {
    pub fn valid_parts(&self) -> Vec<u32> {
        // check if parts are valid and return a vector of them
        let mut valid_parts: Vec<u32> = vec![];

        let _: () = self
            .numbers
            .iter()
            .map(|number| {
                let mut is_valid = false;
                let _: () = number
                    .cells
                    .iter()
                    .map(|cell| {
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                let y_check = cell.0 as i32 + dy;
                                let x_check = cell.1 as i32 + dx;
                                if (x_check >= 0)
                                    && (y_check >= 0)
                                    && self.symbols.iter().any(|symbol| {
                                        symbol.cell == (y_check as usize, x_check as usize)
                                    })
                                {
                                    is_valid = true;
                                }
                            }
                        }
                    })
                    .collect();
                if is_valid {
                    valid_parts.push(number.value);
                }
            })
            .collect();

        valid_parts
    }

    pub fn from_lines(lines: Vec<Vec<char>>) -> Self {
        let mut numbers: Vec<Number> = vec![];
        let mut symbols: Vec<Symbol> = vec![];

        let y_max = lines.len(); // vertically down from 0
        let x_max = lines[0].len(); // to right from 0 

        // Number candidates are tracked in this structure
        let mut number_candidate: Option<u32> = None;
        // The cells making up the number are tracked in cells
        let mut cells: Vec<(usize, usize)> = vec![];

        for y in 0..y_max {
            // Check if the previous line has ended with a digit
            Grid::_check_and_track_number(&mut number_candidate, &mut numbers, &mut cells);

            for x in 0..x_max {
                let c = lines[y][x];
                // If it's a digit, add it to the number
                if let Some(digit) = c.to_digit(10) {
                    number_candidate = match number_candidate {
                        // we are moving left to right, so a previous candidate is moved to the 10's column
                        // so that the new digit can be the 1's column
                        Some(current_candidate) => Some(current_candidate * 10 + digit),
                        None => Some(digit),
                    };
                    // Remember the cell the digit was found in
                    cells.push((y, x));
                } else {
                    // If the character is not a digit:
                    match c {
                        // a '.' is a background character - nothing happens
                        '.' => {}
                        // Anything else is a symbol worth recording
                        _ => {
                            symbols.push(Symbol {
                                id: symbols.len(),
                                character: c,
                                cell: (y, x),
                            });
                        }
                    }
                    // in any case, before the special character, there might have been a number
                    Grid::_check_and_track_number(&mut number_candidate, &mut numbers, &mut cells);
                }
            }
        }
        // check if last line has ended with a digit
        Grid::_check_and_track_number(&mut number_candidate, &mut numbers, &mut cells);

        Grid { numbers, symbols }
    }

    fn _check_and_track_number(
        number_candidate: &mut Option<u32>,
        numbers: &mut Vec<Number>,
        cells: &mut Vec<(usize, usize)>,
    ) {
        if let Some(nr) = number_candidate {
            numbers.push(Number {
                id: numbers.len(),
                value: *nr,
                cells: cells.clone(),
            });
            *number_candidate = None;
            *cells = vec![];
        }
    }
}

pub fn read_input(fname: &str) -> Vec<Vec<char>> {
    let file = File::open(fname).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().trim().chars().collect())
        .collect()
}

pub fn sum_of_parts(fname: &str) -> u32 {
    let lines = read_input(fname);

    let grid = Grid::from_lines(lines);

    let valid_parts = grid.valid_parts();

    let sum: u32 = valid_parts.iter().sum();
    sum
}
