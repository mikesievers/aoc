// Pipes
//
// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
//
// Possible good to solve with https://github.com/petgraph/petgraph?tab=readme-ov-file
// Petgraph on docs.rs => https://docs.rs/petgraph/latest/petgraph/index.html

use std::fs::read_to_string;

struct Grid {
    tiles: Vec<Vec<char>>,
}

impl Grid {
    pub fn from_file(fname: &str) -> Self {
        let data = read_to_string(fname).unwrap();
        let tiles = data
            .lines()
            .into_iter()
            .map(|line| line.chars().collect())
            .collect();
        Grid { tiles }
    }
}

#[test]
fn test_read_graph() {
    let grid = Grid::from_file("resources/day10_sample1.txt");

    assert_eq!(grid.tiles.len(), 5);
    assert_eq!(grid.tiles[0].len(), 5);
}
