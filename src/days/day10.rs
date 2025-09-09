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
use petgraph::visit::Dfs;
use std::fs::read_to_string;

pub struct Grid {
    tiles: Vec<Vec<char>>,
    graph: UnGraphMap<(usize, usize), i64>,
    start: Option<(usize, usize)>,
}

impl Grid {
    pub fn from_file(fname: &str) -> Self {
        let data = read_to_string(fname).unwrap();
        let tiles = data
            .lines()
            .into_iter()
            .map(|line| line.chars().collect())
            .collect();

        let graph = UnGraphMap::new();
        let mut grid = Grid {
            tiles,
            graph,
            start: None,
        };

        grid.tiles_to_graph();
        grid.find_start();

        grid
    }

    fn find_start(&mut self) -> () {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == 'S' {
                    self.start = Some((y, x))
                }
            }
        }
    }

    fn tiles_to_graph(&mut self) -> () {
        // For all tiles
        // - determine whether two legal tiles are being connected
        // - add the two tiles to the graph if not already present
        // - add the connection between them as an edge
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, _c) in row.iter().enumerate() {
                let this_node = (y, x);
                let south_node = (this_node.0 + 1, this_node.1);
                let east_node = (this_node.0, this_node.1 + 1);

                if self.is_connected_south(this_node, south_node) {
                    self.graph.add_node(this_node);
                    self.graph.add_node(south_node);
                    self.graph.add_edge(this_node, south_node, 1_i64);
                }

                if self.is_connected_east(this_node, east_node) {
                    self.graph.add_node(this_node);
                    self.graph.add_node(east_node);
                    self.graph.add_edge(this_node, east_node, 1_i64);
                }
            }
        }
    }

    fn is_connected_south(&self, node: (usize, usize), south_node: (usize, usize)) -> bool {
        let tile = self.tiles.get(node.0).unwrap().get(node.1).unwrap();

        if south_node.0 >= self.height() || south_node.1 >= self.width() {
            return false;
        }

        let south_tile = self
            .tiles
            .get(south_node.0)
            .unwrap()
            .get(south_node.1)
            .unwrap();

        (*tile == '|' && *south_tile == 'L')
            || (*tile == '|' && *south_tile == 'J')
            || (*tile == '|' && *south_tile == '|')
            || (*tile == '|' && *south_tile == 'S')
            || (*tile == 'S' && *south_tile == 'L')
            || (*tile == 'S' && *south_tile == 'J')
            || (*tile == 'S' && *south_tile == '|')
            || (*tile == '7' && *south_tile == 'L')
            || (*tile == '7' && *south_tile == 'J')
            || (*tile == '7' && *south_tile == '|')
            || (*tile == '7' && *south_tile == 'S')
            || (*tile == 'F' && *south_tile == 'L')
            || (*tile == 'F' && *south_tile == 'J')
            || (*tile == 'F' && *south_tile == '|')
            || (*tile == 'F' && *south_tile == 'S')
    }

    fn is_connected_east(&self, node: (usize, usize), east_node: (usize, usize)) -> bool {
        let tile = self.tiles.get(node.0).unwrap().get(node.1).unwrap();

        if east_node.0 >= self.height() || east_node.1 >= self.width() {
            return false;
        }

        let east_tile = self
            .tiles
            .get(east_node.0)
            .unwrap()
            .get(east_node.1)
            .unwrap();

        (*tile == '-' && *east_tile == '7')
            || (*tile == '-' && *east_tile == 'J')
            || (*tile == '-' && *east_tile == '-')
            || (*tile == '-' && *east_tile == 'S')
            || (*tile == 'S' && *east_tile == '7')
            || (*tile == 'S' && *east_tile == 'J')
            || (*tile == 'S' && *east_tile == '-')
            || (*tile == 'F' && *east_tile == '7')
            || (*tile == 'F' && *east_tile == 'J')
            || (*tile == 'F' && *east_tile == '-')
            || (*tile == 'F' && *east_tile == 'S')
            || (*tile == 'L' && *east_tile == '7')
            || (*tile == 'L' && *east_tile == 'J')
            || (*tile == 'L' && *east_tile == '-')
            || (*tile == 'L' && *east_tile == 'S')
    }

    pub fn height(&self) -> usize {
        self.tiles.len()
    }

    pub fn width(&self) -> usize {
        self.tiles[0].len()
    }

    pub fn cycle_length(&self) -> Option<usize> {
        let mut length = 0;

        let mut dfs = Dfs::new(&self.graph, self.start.unwrap());
        while let Some(nx) = dfs.next(&self.graph) {
            if length > 0 && nx == self.start.unwrap() {
                println!("node: {:?}", nx);
                return Some(length);
            }
            length += 1;
        }
        // it's OK to return the total length, because the DFS stops one short of the original node
        Some(length)
    }

    // Part 2:
    // It is necessary to find which which nodes are completely enclosed by the
    // loop and which are outside of it.
    //
    // Approach:
    // Use ray casting:
    // - cast a ray
    // - count the number of times the loop is traversed
    // - an uneven count means the cell is in the loop
    // - to avoid running along edges, don't check on the same level
    //   - but count the number of nodes to the right with a connection
    //     to the node below them
    pub fn count_inside_nodes(&self) -> i32 {
        let mut inside_nodes = 0;

        for (y, row) in self.tiles.iter().enumerate() {
            for (x, _c) in row.iter().enumerate() {
                if self.is_inside((y, x)) {
                    inside_nodes += 1;
                }
            }
        }

        inside_nodes
    }

    fn is_inside(&self, node: (usize, usize)) -> bool {
        // Any part of the graph can't be outside
        if self.graph.contains_node(node) {
            return false;
        }
        // any node at top or bottom is outside
        if node.0 == 0 || node.0 == self.height() {
            return false;
        }

        // All other nodes need to cast a ray to the right and count intersections
        let mut nr_intersections = 0;

        for x in node.1 + 1..self.width() - 1 {
            let ref_node = (node.0, x);
            let south_node = (node.0 - 1, x);
            if self.graph.contains_edge(ref_node, south_node)
                || self.graph.contains_edge(south_node, ref_node)
            {
                nr_intersections += 1;
            }
        }

        nr_intersections % 2 != 0
    }
}

#[test]
fn test_part2() {
    let grid = Grid::from_file("resources/day10p2_sample0.txt");
    assert_eq!(grid.count_inside_nodes(), 4);

    let grid = Grid::from_file("resources/day10p2_sample2.txt");
    assert_eq!(grid.count_inside_nodes(), 8);

    let grid = Grid::from_file("resources/day10p2_sample1.txt");
    assert_eq!(grid.count_inside_nodes(), 10);
}

#[test]
fn test_grid() {
    let grid = Grid::from_file("resources/day10_sample1.txt");

    assert_eq!(grid.height(), 5);
    assert_eq!(grid.width(), 5);

    assert_eq!(grid.start, Some((1, 1)));

    // All nodes connected to a neighbor node somehow
    assert_eq!(grid.graph.node_count(), 17);

    // length of the loop from S
    assert_eq!(grid.cycle_length(), Some(8));
}
