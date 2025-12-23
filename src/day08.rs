//! Advent of Code day 08
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

pub struct Graph {
    pub nodes: Vec<IVec3>,
}

impl Graph {
    pub fn from_file(fname: &str) -> Self {
        let data = read_to_string(fname).unwrap();
        let (_, nodes) = parse_nodes(data.as_str()).unwrap();

        Graph { nodes }
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
        let graph = Graph::from_file("input/day08_sample.txt");

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
    }
}
