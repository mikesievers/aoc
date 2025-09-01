// Process files like the sample:
// Time:      7  15   30
// Distance:  9  40  200

use std::fs::read_to_string;

struct RaceLog {
    time: Vec<i64>,
    distance: Vec<i64>,
}

impl RaceLog {
    pub fn from_file(fname: &str) -> Self {
        let data = read_to_string(fname).unwrap();

        let time = vec![];
        let distance = vec![];

        Self { time, distance }
    }
}

#[test]
fn test_from_file() {
    let race_log = RaceLog::from_file("resources/day06_sample.txt");

    assert_eq!(race_log.time, [7, 15, 30]);
    assert_eq!(race_log.distance, [9, 40, 200]);
}
