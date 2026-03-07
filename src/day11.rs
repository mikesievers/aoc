use std::collections::HashMap;
use std::fs::read_to_string;

use anyhow::Result;

use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::{IResult, multi::separated_list0, sequence::separated_pair};

struct Device {
    id: String,
    conns: Vec<String>,
}

pub struct Rack {
    devices: HashMap<String, Device>,
}

impl Rack {
    pub fn from_file(path: &str) -> Result<Self> {
        let data = read_to_string(path)?;
        let (_, device_vec) = parse_rack(&data).expect("Could not parse file");
        let mut devices = HashMap::new();
        for device in device_vec {
            devices.insert(device.id.clone(), device);
        }
        Ok(Rack { devices })
    }
}

// Parsers
fn parse_device(input: &str) -> IResult<&str, Device> {
    let (rest, (id, conns)) =
        separated_pair(alpha1, tag(": "), separated_list0(char(' '), alpha1)).parse(input)?;

    Ok((
        rest,
        Device {
            id: id.to_owned(),
            conns: conns
                .into_iter()
                .map(|c: &str| c.to_owned().into())
                .collect(),
        },
    ))
}

fn parse_rack(input: &str) -> IResult<&str, Vec<Device>> {
    let (rest, devices) = separated_list1(line_ending, parse_device).parse(input)?;
    Ok((rest, devices))
}

impl Rack {
    pub fn nr_paths_to_out(&self, node: &str) -> u64 {
        // Count the number of different paths from given string to "out"
        // If the exit node is found, count as 1
        if node == "out" {
            return 1;
        }
        // recursively find the paths from the nodes connected to the current one
        self.devices[node]
            .conns
            .iter()
            .map(|node| self.nr_paths_to_out(&node))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_rack() -> Result<()> {
        let rack = Rack::from_file("input/day11_sample.txt")?;

        assert_eq!(rack.devices.len(), 10);
        assert_eq!(rack.devices["ddd"].id, "ddd");
        assert_eq!(rack.devices["bbb"].conns, vec!["ddd", "eee"]);

        assert_eq!(rack.nr_paths_to_out("out"), 1);
        assert_eq!(rack.nr_paths_to_out("you"), 5);

        Ok(())
    }

    #[test]
    fn test_day11_1() -> Result<()> {
        let rack = Rack::from_file("input/day11_input.txt")?;

        assert_eq!(rack.nr_paths_to_out("you"), 670);

        Ok(())
    }
}
