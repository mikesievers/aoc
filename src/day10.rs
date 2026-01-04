//! Advent of Code 2025 day 10

use crate::lights_out::{Button, LightState as LightStateOL, LightsOut};
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

    fn optimize_machine(machine: &Machine) -> usize {
        // Use lights out module to find solution
        //
        // let initial_state = vec![LightState { v: 0, dim: 2 }, LightState { v: 1, dim: 2 }]; // Light 0 is off, 1 is on
        // let target_state = vec![LightState { v: 0, dim: 2 }, LightState { v: 0, dim: 2 }]; // All lights should be out
        // // Button 0 switches light 0,
        // // button 1 switches lights 0 and 1
        // let buttons: Vec<Button> = vec![vec![1, 0], vec![1, 1]];
        // let lo = LightsOut::new(buttons, initial_state, target_state);
        //
        // // Both buttons have to be pressed once:
        // assert_eq!(lo.solution(), Some(vec![1, 1]));
        let initial_state = machine
            .diagram
            .iter()
            .map(|light| match light {
                LightState::On => LightStateOL { v: 1, dim: 2 },
                LightState::Off => LightStateOL { v: 0, dim: 2 },
            })
            .collect_vec();
        println!("Initial state: {:?}", &initial_state);

        let target_state = initial_state
            .iter()
            .map(|_x| LightStateOL { v: 0, dim: 2 })
            .collect_vec();
        println!("Target state: {:?}", &target_state);

        // Configure which lights are switched by which button
        let mut buttons = Vec::with_capacity(machine.wiring.len());

        for wiring in &machine.wiring {
            let button = (0..target_state.len())
                .map(|light_idx| match wiring.contains(&light_idx) {
                    true => 1,
                    false => 0,
                })
                .collect_vec();
            buttons.push(button);
        }

        println!("Buttons: {:?}", buttons);

        // Create the problem and get the solution
        let lights_out = LightsOut::new(buttons, initial_state, target_state);

        let solution = lights_out
            .solution()
            .expect("LightsOut could not be solved!");

        solution.iter().map(|&x| x as usize).sum()
    }

    pub fn minimal_switch_length(&self) -> usize {
        // Treat this as an LP problem with booolean decision variables (switch on or off)
        self.machines
            .iter()
            .map(|machine| Self::optimize_machine(machine))
            .sum()
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
        assert_eq!(manual.minimal_switch_length(), 7);
    }
}
