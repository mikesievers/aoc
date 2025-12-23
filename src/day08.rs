//! Advent of Code day 08
//! ```
//! use aoc::day08::Graph;
//! let graph = Graph::from_file("input/day08_input.txt", 1000);
//! assert_eq!(graph.largest_circuits(), 79056);
//! ```
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

use glam::IVec3;
use nom::Parser;
use nom::character::complete::line_ending;
use nom::{
    IResult,
    character::complete::{char, i32},
    combinator::map,
    multi::separated_list1,
    sequence::terminated,
};

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<IVec3>,
    pub circuits: Vec<HashSet<IVec3>>,
    max: usize,
}

impl Graph {
    pub fn from_file(fname: &str, max: usize) -> Self {
        let data = read_to_string(fname).unwrap();
        let (_, nodes) = parse_nodes(data.as_str()).unwrap();

        let circuits = Graph::compute_circuits(&nodes, max);

        Graph {
            nodes,
            circuits,
            max,
        }
    }

    fn compute_circuits(nodes: &Vec<IVec3>, max: usize) -> Vec<HashSet<IVec3>> {
        // In the beginning, each node is a single cell circuit
        let mut circuits: Vec<HashSet<IVec3>> =
            nodes.iter().map(|&node| HashSet::from([node])).collect();

        // Generate a Vec of node pairs, sort by distances
        let mut node_dist = nodes
            .iter()
            .tuple_combinations()
            //.map(|(&n1, &n2)| (n1, n2, (n1).distance_squared(n2)))
            .map(|(&n1, &n2)| {
                (
                    n1,
                    n2,
                    (((n1.x as i64 - n2.x as i64).pow(2)
                        + (n1.y as i64 - n2.y as i64).pow(2)
                        + (n1.z as i64 - n2.z as i64).pow(2)) as f64)
                        .sqrt() as u64,
                )
            })
            .collect::<Vec<_>>();
        node_dist.sort_by_key(|(_, _, d)| *d);

        // Keep only the first max shortest onnections
        node_dist.truncate(max);
        // println!("node distances: {:?}", node_dist);

        // Merge circuits as necessary
        for pair in node_dist {
            // println!("pair: {}, {}", pair.0, pair.1);
            // determine circuits they are part of
            let existing_circs = circuits.iter().fold(
                Vec::new(),
                |mut circuit_acc: Vec<HashSet<IVec3>>, circuit: &HashSet<IVec3>| {
                    if circuit.contains(&pair.0) || circuit.contains(&pair.1) {
                        circuit_acc.push(circuit.clone());
                        circuit_acc
                    } else {
                        circuit_acc
                    }
                },
            );
            // println!("Existing circs: {:?}", existing_circs);
            match existing_circs.len() {
                // 2. If both are not part of a circuit, form a new circuit composed of the twoo
                // 0 => {
                //     let new_circuit = HashSet::from([pair.0, pair.1]);
                //     circuits.push(new_circuit);
                // }
                // If both are part of the same circuit, do nothing
                1 => {
                    // for mut circuit in &mut circuits {
                    //     if existing_circs.contains(&circuit) {
                    //         circuit.insert(pair.0);
                    //         circuit.insert(pair.1);
                    //     }
                    // }
                }
                // 4. If both are part of different circuits, merge the circuits
                2 => {
                    // Remove existing circs
                    circuits.retain(|x| x != &existing_circs[0]);
                    circuits.retain(|x| x != &existing_circs[1]);
                    // Add merged circuits and add them
                    let mut merged_circuits = existing_circs[0].clone();
                    merged_circuits.extend(existing_circs[1].clone());
                    circuits.push(merged_circuits);
                }
                _ => {
                    panic!("{}", format!("Illegal length {}", existing_circs.len()));
                }
            }
            // println!("--- circuits: {:?}", circuits.len());
            // if circuits.len() > 3 {
            //     panic!();
            // }
        }
        circuits
    }

    #[tracing::instrument]
    pub fn largest_circuits(&self) -> usize {
        let mut sorted_circuits = self.circuits.clone();
        sorted_circuits.sort_by_key(|circuit| circuit.len());
        sorted_circuits.reverse();
        sorted_circuits.truncate(3);
        // println!("{:?}", sorted_circuits);
        // println!(
        //     "{:?}",
        //     sorted_circuits.iter().map(|x| x.len()).collect::<Vec<_>>()
        // );
        sorted_circuits
            .iter()
            .map(|circuit| circuit.len())
            .product()
    }
}

fn parse_node(input: &str) -> IResult<&str, IVec3> {
    let (rest, node) = map(
        (terminated(i32, char(',')), terminated(i32, char(',')), i32),
        |(x, y, z)| IVec3 { x, y, z },
    )
    .parse(input)?;

    Ok((rest, node))
}

fn parse_nodes(input: &str) -> IResult<&str, Vec<IVec3>> {
    let (rest, nodes) = separated_list1(line_ending, parse_node).parse(input)?;
    Ok((rest, nodes))
}

#[cfg(test)]
mod tests {
    use glam::IVec3;

    use crate::day08::Graph;

    #[test]
    fn test_graph() {
        let graph = Graph::from_file("input/day08_sample.txt", 10);

        assert_eq!(
            graph.nodes.first(),
            Some(&IVec3 {
                x: 162,
                y: 817,
                z: 812
            })
        );
        assert_eq!(
            graph.nodes.last(),
            Some(&IVec3 {
                x: 425,
                y: 690,
                z: 689
            })
        );

        assert_eq!(graph.largest_circuits(), 40);
    }
}
