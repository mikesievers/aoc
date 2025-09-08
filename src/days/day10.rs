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

use petgraph::graphmap::UnGraphMap;
use std::fs::read_to_string;

struct Grid {
    tiles: Vec<Vec<char>>,
    graph: UnGraphMap<(usize, usize), i64>,
}

impl Grid {
    pub fn from_file(fname: &str) -> Self {
        let data = read_to_string(fname).unwrap();
        let tiles = data
            .lines()
            .into_iter()
            .map(|line| line.chars().collect())
            .collect();

        let graph = Grid::tiles_to_graph(&tiles);
        Grid { tiles, graph }
    }

    fn tiles_to_graph(tiles: &Vec<Vec<char>>) -> UnGraphMap<(usize, usize), i64> {
        let mut graph = UnGraphMap::new();
        let y_max = tiles.len() - 1; // The maximum index is one smaller than the len()
        let x_max = tiles[0].len() - 1;

        // For all tiles
        // - determine whether two legal tiles are being connected
        // - add the two tiles to the graph if not already present
        // - add the connection between them as an edge
        for (y, row) in tiles.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if let Some((n1, n2)) = Grid::connected_nodes((y, x), c, (x_max, y_max)) {
                    println!("{c}");
                    graph.add_node(n1);
                    graph.add_node(n2);
                    graph.add_edge(n1, n2, 1_i64);
                }
            }
        }

        graph
    }

    fn connected_nodes(
        (y, x): (usize, usize),
        tile: &char,
        (y_max, x_max): (usize, usize),
    ) -> Option<((usize, usize), (usize, usize))> {
        let is_legal_tile = |y: isize, x: isize| -> Option<(usize, usize)> {
            if y <= y_max as isize && x <= x_max as isize {
                Some((y as usize, x as usize))
            } else {
                None
            }
        };

        match tile {
            // | is a vertical pipe connecting north and south.
            // - is a horizontal pipe connecting east and west.
            // L is a 90-degree bend connecting north and east.
            // J is a 90-degree bend connecting north and west.
            // 7 is a 90-degree bend connecting south and west.
            // F is a 90-degree bend connecting south and east.
            // . is ground; there is no pipe in this tile.
            // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
            '|' => Some((
                is_legal_tile(y as isize - 1, x as isize)?,
                is_legal_tile(y as isize + 1, x as isize)?,
            )),
            '-' => Some((
                is_legal_tile(y as isize, x as isize - 1)?,
                is_legal_tile(y as isize, x as isize + 1)?,
            )),
            'L' => Some((
                is_legal_tile(y as isize - 1, x as isize)?,
                is_legal_tile(y as isize, x as isize + 1)?,
            )),
            'J' => Some((
                is_legal_tile(y as isize - 1, x as isize)?,
                is_legal_tile(y as isize, x as isize - 1)?,
            )),
            '7' => Some((
                is_legal_tile(y as isize + 1, x as isize)?,
                is_legal_tile(y as isize, x as isize - 1)?,
            )),
            'F' => Some((
                is_legal_tile(y as isize + 1, x as isize)?,
                is_legal_tile(y as isize, x as isize + 1)?,
            )),
            _ => None,
        }
    }

    pub fn height(&self) -> usize {
        self.tiles.len()
    }

    pub fn width(&self) -> usize {
        self.tiles[0].len()
    }
}

#[test]
fn test_grid() {
    let grid = Grid::from_file("resources/day10_sample1.txt");

    assert_eq!(grid.height(), 5);
    assert_eq!(grid.width(), 5);

    assert_eq!(grid.graph.node_count(), 8);
}
