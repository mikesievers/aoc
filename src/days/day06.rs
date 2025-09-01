// Process files like the sample (ms, mm):
// Time:      7  15   30
// Distance:  9  40  200

// - initial speed is 0
// - charging for 1ms increases speed by 1mm/s
// - time = charge_time + race_time
// - race_time > 0
// - (time - charge_time) * speed > distance (travelled distance must be greater than the record)
//
// Thoughts:
// - speed is charge_time in mm/ms  => (time - charge_time)*charge_time > distance
//                                  => time * charge_time - charge_time**2 > distance  | *-1
//                                  => charge_time**2 - time*charge_time < distance    | (a-b)**2 = a**2 - 2ab + b**2
//                                  => (charge_time - time/2)**2  < distance - time**2/4
//                                  => time/2 - sqrt(distance-time**2/4) < charge_time < time/2 + sqrt(distance-time**2/4)

use nom::bytes::complete::tag;
use nom::character::complete::{i64, line_ending, space0, space1};
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::{IResult, Parser};
use std::fs::read_to_string;
use std::num;
use std::ops::Range;

pub struct RaceLog {
    time: Vec<i64>,
    distance: Vec<i64>,
}

impl RaceLog {
    pub fn from_file(fname: &str) -> Self {
        let data = read_to_string(fname).unwrap();

        let (_, (time, distance)) = (time_line, distance_line).parse(data.as_str()).unwrap();

        Self { time, distance }
    }

    pub fn winning_range_length(&self, time: i64, distance: i64) -> i64 {
        let delta = (-(distance as f64) + ((time * time) as f64) / 4.0).sqrt();
        let mut from = (time as f64) / 2.0 - delta;
        let mut to = (time as f64) / 2.0 + delta;

        // account for exactly hitting integer values
        if from.ceil() == from.floor() {
            from += 1.0;
        }
        if to.ceil() == to.floor() {
            to -= 1.0;
        }

        let mut ifrom = from.ceil() as i64;
        let mut ito = to.floor() as i64;

        ito - ifrom + 1
    }

    pub fn winning_ranges_product(&self) -> i64 {
        let mut range_product = 1;
        for idx in 0..self.time.len() {
            let range_len = self.winning_range_length(self.time[idx], self.distance[idx]);
            range_product *= range_len;
        }
        range_product
    }
}

// <space>*<i64><space>+<i64>...<space>*<line_ending>
fn number_line(input: &str) -> IResult<&str, Vec<i64>> {
    delimited(space1, separated_list1(space1, i64), space0).parse(input)
}

fn time_line(input: &str) -> IResult<&str, Vec<i64>> {
    delimited(tag("Time:"), number_line, line_ending).parse(input)
}

fn distance_line(input: &str) -> IResult<&str, Vec<i64>> {
    delimited(tag("Distance:"), number_line, line_ending).parse(input)
}

#[test]
fn test_range_lengths() {
    let race_log = RaceLog::from_file("resources/day06_sample.txt");

    assert_eq!(race_log.winning_ranges_product(), 288);
}

#[test]
fn test_parser() {
    let numbers = "  3  2  1 \n";
    let (_, res) = number_line.parse(numbers).unwrap();
    assert_eq!(res, [3, 2, 1]);

    let timeline = "Time:   3    5 3 \n";
    let (_, res) = time_line.parse(timeline).unwrap();
    assert_eq!(res, [3, 5, 3]);
}

#[test]
fn test_from_file() {
    let race_log = RaceLog::from_file("resources/day06_sample.txt");

    assert_eq!(race_log.time, [7, 15, 30]);
    assert_eq!(race_log.distance, [9, 40, 200]);
}
