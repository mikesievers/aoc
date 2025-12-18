//! AOC 2025 day 04
//! ```
//! use aoc::day04::Grid;
//!
//! let grid = Grid::from_file("input/day04_input.txt");
//! assert_eq!(grid.nr_accessible(), 1437);
//! ```

use nom::{Parser, character::complete::line_ending, multi::separated_list1};
use std::{fs::read_to_string, iter::Product};

use nom::{
    IResult,
    character::complete::{char, one_of},
    multi::many1,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Tile {
    Floor,
    Roll,
}
pub struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    pub fn from_file(fname: &str) -> Self {
        let data = read_to_string(fname).unwrap();

        let tiles = parse_grid(&data).unwrap().1;
        Grid { tiles }
    }

    fn tile_at(&self, y: i32, x: i32) -> Option<Tile> {
        if x < 0
            || y < 0
            || x > (self.tiles.len() as i32 - 1)
            || y > (self.tiles[0].len() as i32 - 1)
        {
            return None;
        }

        Some(self.tiles[y as usize][x as usize].clone())
    }

    fn nr_neighbors_at(&self, y: i32, x: i32) -> usize {
        let mut cnt = 0;
        for delta_y in [-1, 0, 1] {
            for delta_x in [-1, 0, 1] {
                if (delta_y == 0) && (delta_x == 0) {
                    continue; // Don't check the tile itself
                }
                if self.tile_at(y + delta_y, x + delta_x) == Some(Tile::Roll) {
                    cnt += 1;
                }
            }
        }

        cnt
    }

    pub fn nr_accessible(&self) -> usize {
        let mut cnt_accessible = 0;

        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[0].len() {
                if self.tile_at(y as i32, x as i32) == Some(Tile::Roll)
                    && self.nr_neighbors_at(y as i32, x as i32) < 4
                {
                    cnt_accessible += 1;
                }
            }
        }

        cnt_accessible
    }
}

// Parsers
fn parse_row(input: &str) -> IResult<&str, Vec<Tile>> {
    let (rest, row) = many1(one_of(".@")).parse(input)?;
    let tiles = row
        .iter()
        .map(|&c| match c {
            '.' => Tile::Floor,
            '@' => Tile::Roll,
            _ => panic!("Unknown character {c}"),
        })
        .collect();
    Ok((rest, tiles))
}

fn parse_grid(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    let (rest, tiles) = separated_list1(line_ending, parse_row).parse(input)?;

    Ok((rest, tiles))
}

#[cfg(test)]
mod tests {
    use crate::day04::{Grid, Tile, parse_row};

    #[test]
    fn test_grid() {
        let grid = Grid::from_file("input/day04_sample.txt");

        assert_eq!(grid.tile_at(-1, 0), None);
        assert_eq!(grid.tile_at(0, 0), Some(Tile::Floor));
        assert_eq!(grid.tile_at(9, 8), Some(Tile::Roll));

        assert_eq!(grid.nr_neighbors_at(1, 0), 3);
        assert_eq!(grid.nr_neighbors_at(1, 1), 6);

        assert_eq!(grid.nr_accessible(), 13);
    }

    #[test]
    fn test_parse_row() {
        let row = ".@.";
        assert_eq!(
            parse_row(row).unwrap().1,
            vec![Tile::Floor, Tile::Roll, Tile::Floor]
        );
    }
}
