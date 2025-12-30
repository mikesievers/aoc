//! Advent of Code 2025 day 10

use std::fs::read_to_string;

use good_lp::{Expression, Solution, SolverModel, coin_cbc, constraint, variables};

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
        // let mut vars = variables!();
        let n_switches = machine.diagram.len();
        let n_buttons = machine.wiring.len();

        // variables! { button_presses: x[n_buttons] (binary) ;}
        //variables! { button_presses: -0.5<= x[n_buttons] (integer) <= 1.5 ;}
        variables! { button_presses: -0.5<= x[n_buttons] (binary) <= 1.5 ;}

        let mut objective: Expression = 0.into();
        for i in 0..n_buttons {
            objective += x[i];
        }

        // Add constraints for diagram
        let mut constraints = vec![];
        for i_switch in 0..n_switches {
            let mut wiring_constraint: Expression = 0.into();
            for button_idx in 0..n_buttons {
                for &idx in &machine.wiring[button_idx] {
                    if idx == i_switch {
                        wiring_constraint += x[button_idx];
                    }
                }
            }
            match machine.diagram[i_switch] {
                LightState::On => {
                    constraints.push(constraint! {  wiring_constraint.clone() >= 0.99 });
                }
                LightState::Off => {
                    constraints.push(constraint! { wiring_constraint.clone() <= 0.001 });
                }
            };

            // constraints.push(constraint! { wiring_constraint.clone() == target_value });
            // constraints.push(constraint! { wiring_constraint.clone() >= target_lower });
            // constraints.push(constraint! { wiring_constraint.clone() <= target_upper });
        }

        for c in &constraints {
            println!("{:?}", c);
        }

        let solution = button_presses
            .minimise(objective)
            //.using(default_solver)
            .using(coin_cbc)
            .with_all(constraints)
            .solve()
            .expect("Problem could not be solved");

        let sum = x.iter().map(|&x| solution.value(x)).sum::<f64>();
        sum as usize
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
