//! Advent of Code 2025 day 06

pub enum Op {
    Times,
    Plus,
}
pub struct Homework {
    table: Vec<Vec<i64>>,
    operations: Vec<Op>,
}

impl Homework {
    pub fn from_file(fname: &str) -> Self {
        let table = Vec::new();
        let operations = Vec::new();
        Homework { table, operations }
    }
}

#[cfg(test)]
mod tests {
    use super::Homework;

    #[test]
    fn test_homework() {
        let homework = Homework::from_file("input/day06_sample.txt");

        assert_eq!(1, 1);
    }
}
