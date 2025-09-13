use std::{collections::HashSet, fs::read_to_string};

struct Universe {
    space: Vec<Vec<char>>,
    galaxies: Vec<(usize, usize)>,
    empty_rows: HashSet<usize>,
    empty_columns: HashSet<usize>,
}

impl Universe {
    pub fn from_file(fname: &str) -> Self {
        let data = read_to_string(fname).unwrap();
        let space = data
            .lines()
            .into_iter()
            .map(|line| line.chars().collect())
            .collect();
        let (empty_rows, empty_columns, galaxies) = Universe::parse_universe(&space);

        Universe {
            space,
            galaxies,
            empty_rows,
            empty_columns,
        }
    }

    fn parse_universe(
        space: &Vec<Vec<char>>,
    ) -> (HashSet<usize>, HashSet<usize>, Vec<(usize, usize)>) {
        // First, assume space is empty

        let mut empty_rows = HashSet::from_iter((0..space.len()).into_iter());
        let mut empty_columns = HashSet::from_iter(0..space.len());
        let mut galaxies = Vec::new();
        for (y, row) in space.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    '#' => {
                        empty_rows.remove(&y);
                        empty_columns.remove(&x);
                        galaxies.push((y, x));
                    }
                    _ => {}
                }
            }
        }
        (empty_rows, empty_columns, galaxies)
    }

    fn expanded_galaxies(&self) -> Vec<(usize, usize)> {
        let mut expanded_galaxies = Vec::new();

        for galaxy in &self.galaxies {
            let space_before_y = self
                .empty_rows
                .iter()
                .filter(|&&row_idx| row_idx < galaxy.0)
                .count();
            let space_before_x = self
                .empty_columns
                .iter()
                .filter(|&&col_idx| col_idx < galaxy.1)
                .count();
            expanded_galaxies.push((galaxy.0 + space_before_y, galaxy.1 + space_before_x));
        }

        expanded_galaxies
    }
}

#[test]
pub fn test_universe() {
    let universe = Universe::from_file("resources/day11_sample.txt");
    assert_eq!(universe.space.len(), 10);
    assert_eq!(*universe.space.get(2).unwrap().get(0).unwrap(), '#');
    assert_eq!(*universe.space.get(1).unwrap().get(1).unwrap(), '.');
    assert_eq!(universe.empty_rows, HashSet::from_iter(vec![3, 7]));
    assert_eq!(universe.empty_columns, HashSet::from_iter(vec![2, 5, 8]));
    assert_eq!(
        universe.galaxies,
        vec![
            (0, 3),
            (1, 7),
            (2, 0),
            (4, 6),
            (5, 1),
            (6, 9),
            (8, 7),
            (9, 0),
            (9, 4)
        ]
    );
    assert_eq!(
        universe.expanded_galaxies(),
        vec![(0, 4), (1, 9), (2, 0), (5, 8), (6, 1), (7, 12), (10, 9), (11, 0), (11, 5)]
    );
}
