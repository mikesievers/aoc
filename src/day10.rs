//! Advent of Code 2025 day 10

use std::fs::read_to_string;

use itertools::Itertools;

use nom::{
    IResult, Parser,
    character::complete::{self, char, line_ending, one_of, space1},
    multi::{many1, separated_list1},
    sequence::{delimited, terminated},
};

#[derive(Clone, Debug)]
pub enum LightState {
    On,
    Off,
}

#[derive(Debug)]
pub struct Machine {
    diagram: Vec<LightState>,
    wiring: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

pub struct Manual {
    machines: Vec<Machine>,
}

impl Manual {
    pub fn from_file(fname: &str) -> Self {
        let data = read_to_string(fname).expect("File could not be read");

        let (_, machines) = parse_machines
            .parse(data.as_str())
            .expect("Could not parse machines");

        Manual { machines }
    }
}

// Parsers
fn parse_diagram(input: &str) -> IResult<&str, Vec<LightState>> {
    let (rest, state_raw) = delimited(char('['), many1(one_of(".#")), char(']')).parse(input)?;

    let state = state_raw
        .iter()
        .map(|c| match c {
            '.' => LightState::Off,
            '#' => LightState::On,
            _ => panic!("Unexpected light state"),
        })
        .collect_vec();

    Ok((rest, state))
}

fn parse_wiring(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list1(
        char(' '),
        delimited(
            char('('),
            separated_list1(char(','), complete::usize),
            char(')'),
        ),
    )
    .parse(input)
}

fn parse_joltage(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        char('{'),
        separated_list1(char(','), complete::usize),
        char('}'),
    )
    .parse(input)
}

fn parse_machines(input: &str) -> IResult<&str, Vec<Machine>> {
    let (rest, machines_raw) = separated_list1(
        line_ending,
        (
            terminated(parse_diagram, space1),
            terminated(parse_wiring, space1),
            parse_joltage,
        ),
    )
    .parse(input)?;
    let machines = machines_raw
        .iter()
        .map(|(diagram, wiring, joltage)| Machine {
            diagram: diagram.clone(),
            wiring: wiring.clone(),
            joltage: joltage.clone(),
        })
        .collect_vec();

    Ok((rest, machines))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manual() {
        let manual = Manual::from_file("input/day10_sample.txt");
        assert_eq!(true, false);
    }
}
