use std::fs::read_to_string;

use nom::Parser;
use nom::character::complete::{i64, line_ending};
use nom::{IResult, character::complete::space1, multi::separated_list1};

// Structs
pub struct Report {
    histories: Vec<Vec<i64>>,
}

impl Report {
    pub fn from_file(fname: &str) -> Self {
        let data = read_to_string(fname).unwrap();
        let (_, histories) = number_line_file.parse(data.as_str()).unwrap();
        Report { histories }
    }

    // The prediction is the last value of the line plus the prediction of the line below it
    // ... until the last line is only zeroes
    pub fn predict_value(&self, line: &Vec<i64>) -> i64 {
        if line.iter().filter(|n| **n != 0_i64).count() == 0 {
            return 0;
        };

        // Calculate the vector of differences
        let delta_line = line
            .iter()
            .zip(line.iter().skip(1))
            .map(|(n_i, n_i_plus_1)| *n_i_plus_1 - *n_i)
            .collect();

        let last_value = line[line.len() - 1];
        last_value + self.predict_value(&delta_line)
    }

    pub fn predictions_sum(&self) -> i64 {
        self.histories
            .iter()
            .map(|history| self.predict_value(history))
            .sum()
    }

    // Part 2
    // Predict the value to the left
    pub fn postdict_value(&self, line: &Vec<i64>) -> i64 {
        if line.iter().filter(|n| **n != 0_i64).count() == 0 {
            return 0;
        };

        // Calculate the vector of differences
        let delta_line = line
            .iter()
            .zip(line.iter().skip(1))
            .map(|(n_i, n_i_plus_1)| *n_i_plus_1 - *n_i)
            .collect();

        let first_value = line[0];
        first_value - self.postdict_value(&delta_line)
    }

    pub fn postdictions_sum(&self) -> i64 {
        self.histories
            .iter()
            .map(|history| self.postdict_value(history))
            .sum()
    }
}

#[test]
fn test_report() {
    let report = Report::from_file("resources/day09_sample.txt");

    assert_eq!(report.histories.len(), 3);
    assert_eq!(
        report.histories[1],
        vec![1_i64, 3_i64, 6_i64, 10_i64, 15_i64, 21_i64]
    );

    assert_eq!(report.predict_value(&report.histories[0]), 18);

    // Predictions of the rightmost number
    assert_eq!(report.predictions_sum(), 114);

    // Predictions of the leftmost number
    assert_eq!(report.postdictions_sum(), 2);
}

// Parsers
fn number_line(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, i64).parse(input)
}

fn number_line_file(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(line_ending, number_line).parse(input)
}

#[test]
fn test_number_line() {
    let nline = "1 2 3 4";
    assert_eq!(
        number_line.parse(nline).unwrap(),
        ("", vec![1_i64, 2_i64, 3_i64, 4_i64])
    );
}
