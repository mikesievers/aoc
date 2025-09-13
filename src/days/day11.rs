use std::{collections::HashSet, fs::read_to_string};

struct Universe {
    space: Vec<Vec<char>>,
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
        let (empty_rows, empty_columns) = Universe::empty_rows_columns(&space);

        Universe {
            space,
            empty_rows,
            empty_columns,
        }
    }

    pub fn empty_rows_columns(space: &Vec<Vec<char>>) -> (HashSet<usize>, HashSet<usize>) {
        // First, assume space is empty
        
        let mut empty_rows = HashSet::from_iter((0..space.len()).into_iter());
        let mut empty_columns = HashSet::from_iter(0..space.len());
        for (y, row) in space.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    '#' => {
                        empty_rows.retain(|&row_idx| row_idx != y);
                        empty_columns.retain(|&col_idx| col_idx != x);
                    }
                    _ => {}
                }
            }
        }
        (empty_rows, empty_columns)
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
}
