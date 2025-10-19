use std::fs::read_to_string;

struct Platform {
    space: Vec<Vec<char>>,
}

impl Platform {
    pub fn from_file(fname: &str) -> Self {
        let data = read_to_string(fname).unwrap();

        let space = data.lines().map(|line| line.chars().collect()).collect();

        Platform { space }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform() {
        let platform = Platform::from_file("resources/day14_sample.txt");

        assert_eq!(platform.space.len(), 10);
        assert_eq!(platform.space[0].len(), 10);

        assert_eq!(platform.space[0][0], 'O');
        assert_eq!(platform.space[1][0], 'O');
        assert_eq!(platform.space[1][1], '.');
        assert_eq!(platform.space[1][4], '#');

    }
}
