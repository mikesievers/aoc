//! AOC 2025 day 04

use nom::{Parser, character::complete::line_ending, multi::separated_list1};
use std::fs::read_to_string;

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

impl Grid {
    fn tile_at(&self, x: i32, y: i32) -> Option<Tile> {
        if x < 0 || y < 0 || x > (self.tiles.len() as i32 - 1) || y > (self.tiles.len() as i32 - 1)
        {
            return None;
        }

        Some(self.tiles[x as usize][y as usize].clone())
    }
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
