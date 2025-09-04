use std::{collections::HashMap, fs::read_to_string};

use nom::{
    IResult, Parser,
    bytes::tag,
    character::complete::{alpha1, alphanumeric1, line_ending, newline, one_of},
    combinator::{map, recognize},
    multi::{count, many1, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
};

#[derive(Debug)]
pub struct Map<'a> {
    data: String, // Own the file contents
    directions: Vec<Direction>,
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

#[derive(Debug, PartialEq)]
struct Node<'a> {
    node_name: &'a str,
    connections: (&'a str, &'a str),
}

#[derive(PartialEq, Debug)]
enum Direction {
    L,
    R,
}

impl<'a> Map<'a> {
    pub fn from_file(input: &str) -> Self {
        let data = read_to_string(input).unwrap();
        let data_ref: &'a str = unsafe { std::mem::transmute::<&str, &'a str>(&data) };
        let (_, (directions, nodes_raw)) = (direction_line, node_list).parse(data_ref).unwrap();

        let mut nodes = HashMap::new();
        for node in nodes_raw {
            nodes.insert(node.node_name, (node.connections.0, node.connections.1));
        }
        Map {
            data,
            directions,
            nodes,
        }
    }

    pub fn path_length(&self, start: &str) -> i64 {
        let mut current_node = start;
        let mut dir_idx = 0_usize;
        let mut path_length = 0;

        loop {
            path_length += 1;
            current_node = match self.directions[dir_idx] {
                Direction::L => self.nodes.get(current_node).unwrap().0,
                Direction::R => self.nodes.get(current_node).unwrap().1,
            };
            //if current_node == end {
            if current_node.ends_with("Z") {
                break;
            }
            dir_idx = (dir_idx + 1) % self.directions.len();
        }

        path_length
    }

    // Part 2, start on all nodes ending on "A"
    // NOTE: This would have worked eventually, but is way too slow by brute force.
    pub fn multi_path_length(&self) -> i64 {
        let mut current_nodes: Vec<&str> = self
            .nodes
            .keys()
            .into_iter()
            .filter(|n| n.ends_with('A'))
            .map(|a| *a)
            .collect();

        let mut dir_idx = 0_usize;
        let mut path_length = 0;

        loop {
            path_length += 1;

            for node_idx in 0..current_nodes.len() {
                current_nodes[node_idx] = match self.directions[dir_idx] {
                    Direction::L => self.nodes.get(current_nodes[node_idx]).unwrap().0,
                    Direction::R => self.nodes.get(current_nodes[node_idx]).unwrap().1,
                };
            }

            let z_nodes = current_nodes.iter().filter(|&&n| n.ends_with('Z')).count();
            if z_nodes > 3 {
                println!("z_nodes: {z_nodes} path_length: {path_length}");
            }
            if z_nodes == current_nodes.len() {
                break;
            }
            dir_idx = (dir_idx + 1) % self.directions.len();
        }

        path_length
    }
}
fn parse_direction(input: &str) -> IResult<&str, Direction> {
    map(one_of("LR"), |c| match c {
        'L' => Direction::L,
        'R' => Direction::R,
        _ => unreachable!(),
    })
    .parse(input)
}

// Parsers

fn direction_line(input: &str) -> IResult<&str, Vec<Direction>> {
    terminated(many1(parse_direction), line_ending).parse(input)
}

// "AAA"
fn node_name(input: &str) -> IResult<&str, &str> {
    alphanumeric1.parse(input)
    //alpha1.parse(input)
}

// " = (AAA, BBB)"
fn connections(input: &str) -> IResult<&str, (&str, &str)> {
    (
        delimited(tag(" = ("), node_name, tag(", ")),
        terminated(node_name, tag(")")),
    )
        .parse(input)
}

#[test]
fn test_node_name() {
    let (_, res) = node_name.parse("AAA").unwrap();
    assert_eq!(res, "AAA");
}

fn node(input: &str) -> IResult<&str, Node> {
    map((node_name, connections), |(node_name, connections)| Node {
        node_name,
        connections,
    })
    .parse(input)
}

fn node_list(input: &str) -> IResult<&str, Vec<Node>> {
    preceded(line_ending, separated_list1(line_ending, node)).parse(input)
}

#[test]
fn test_parse_connection() {
    assert_eq!(
        connections.parse(" = (AAA, BBB)").unwrap(),
        ("", ("AAA", "BBB"))
    );
}

#[test]
fn test_read_directions() {
    assert_eq!(
        direction_line.parse("LRRL\n").unwrap().1,
        vec![Direction::L, Direction::R, Direction::R, Direction::L]
    );
}

#[test]
fn test_read_node() {
    assert_eq!(
        node.parse("AAA = (BBB, CCC)").unwrap(),
        (
            "",
            Node {
                node_name: "AAA",
                connections: ("BBB", "CCC")
            }
        )
    )
}

#[test]
fn test_read_map() {
    let map = Map::from_file("resources/day08_sample.txt");

    println!("{:?}", map);
}

#[test]
fn test_part1() {
    let map = Map::from_file("resources/day08_sample.txt");
    assert_eq!(map.path_length("AAA"), 2);

    let map = Map::from_file("resources/day08_sample2.txt");
    assert_eq!(map.path_length("AAA"), 6);
}

#[test]
fn test_part2() {
    //let map = Map::from_file("resources/day08p2sample.txt");
    let map = Map::from_file("resources/day08_input.txt");
    println!("Map: {:?}", map);
    //assert_eq!(map.multi_path_length(), 6);

    // The 6 starts are:
    // XVA
    // GGA
    // DXA
    // LTA
    // BJA
    // AAA
    assert_eq!(map.path_length("XVA"), 16271);
    assert_eq!(map.path_length("GGA"), 24253);
    assert_eq!(map.path_length("DXA"), 13201);
    assert_eq!(map.path_length("LTA"), 14429);
    assert_eq!(map.path_length("BJA"), 18113);
    assert_eq!(map.path_length("AAA"), 22411);

    // prime factors:
    // 16271
    //  |	\
    // 307	 	53

    // 24253
    //  |	\
    // 307	 	79

    // 13201
    //  |	\
    // 307	 	43

    // 14429
    //  |	\
    // 307	 	47

    // 18113
    //  |	\
    // 307	 	59

    // 22411
    //  |	\
    // 307	 	73

    // The path_lenghts from the endings:
    assert_eq!(map.path_length("FCZ"), 18113);
    assert_eq!(map.path_length("RPZ"), 14429);
    assert_eq!(map.path_length("GHZ"), 13201);
    assert_eq!(map.path_length("GSZ"), 24253);
    assert_eq!(map.path_length("ZZZ"), 22411);
    assert_eq!(map.path_length("QXZ"), 16271);

    // The cycles are therefore the same, starting from *A or from *Z,
    // And all cycles meet after taking the product of their prime factors (counting 307 only once)
    assert_eq!(
        307_i64 * 53_i64 * 79_i64 * 43_i64 * 47_i64 * 59_i64 * 73_i64,
        11188774513823_i64
    );
}
