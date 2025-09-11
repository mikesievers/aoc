use std::fs::read_to_string;

struct Universe {
    space: Vec<Vec<char>>,
}

impl Universe {
    pub fn from_file(fname: &str) -> Self {
        let data = read_to_string(fname).unwrap();
        let space = data
            .lines()
            .into_iter()
            .map(|line| line.chars().collect())
            .collect();

        Universe { space }
    }
}

#[test]
pub fn test_universe() {
    let universe = Universe::from_file("resources/day11_sample.txt");
    assert_eq!(universe.space.len(), 10);
    assert_eq!(*universe.space.get(2).unwrap().get(0).unwrap(), '#');
    assert_eq!(*universe.space.get(1).unwrap().get(1).unwrap(), '.');
}
