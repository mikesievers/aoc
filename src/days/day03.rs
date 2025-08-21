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
                        // a '.' is a background character - check if the previous char was the end of a number
                        '.' => {
                            Grid::_check_and_track_number(
                                &mut number_candidate,
                                &mut numbers,
                                &mut cells,
                            );
                        }
                        // Anything else is a symbol worth recording
                        _ => {
                            symbols.push(Symbol {
                                id: symbols.len(),
                                character: c,
                                cell: (y, x),
                            });
                        }
                    }
                }
            }
        }
        // check if last line has ended with a digit
        Grid::_check_and_track_number(&mut number_candidate, &mut numbers, &mut cells);

        Grid {
            numbers: numbers,
            symbols: symbols,
        }
    }

    fn _check_and_track_number(
        number_candidate: &mut Option<u32>,
        numbers: &mut Vec<Number>,
        cells: &mut Vec<(usize, usize)>,
    ) {
        match number_candidate {
            Some(nr) => {
                numbers.push(Number {
                    id: numbers.len(),
                    value: *nr,
                    cells: cells.clone(),
                });
                *number_candidate = None;
                *cells = vec![];
            }
            None => {}
        }
    }
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
