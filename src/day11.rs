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
    path_cache: HashMap<String, u64>,
}

impl Rack {
    pub fn from_file(path: &str) -> Result<Self> {
        let data = read_to_string(path)?;
        let (_, device_vec) = parse_rack(&data).expect("Could not parse file");
        let mut devices = HashMap::new();
        for device in device_vec {
            devices.insert(device.id.clone(), device);
        }
        Ok(Rack {
            devices,
            path_cache: HashMap::new(),
        })
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

impl Rack {
    pub fn nr_problematic_paths_to_out(
        &mut self,
        node: &str,
        dac_seen: bool,
        fft_seen: bool,
    ) -> u64 {
        // Count the number of different paths from given string to "out"

        // Check against the cache first
        if let Some(&known_paths) = self
            .path_cache
            .get(&(format!("{}{}{}", node.to_string(), dac_seen, fft_seen)))
        {
            return known_paths;
        }

        // If the exit node is found, count as 1
        // IF both dac and fft have been seen along the way
        if node == "out" {
            if dac_seen && fft_seen {
                return 1;
            } else {
                return 0;
            }
        }
        // If this node is dac or fft, remember that they have been seen
        let mut out_dac_seen = dac_seen;
        let mut out_fft_seen = fft_seen;

        if node == "dac" {
            out_dac_seen = true
        }
        if node == "fft" {
            out_fft_seen = true
        }

        // recursively find the paths from the nodes connected to the current one
        self.devices[node]
            .conns
            .clone()
            .iter()
            .map(|node| {
                let val = self.nr_problematic_paths_to_out(
                    node.clone().as_str(),
                    out_dac_seen,
                    out_fft_seen,
                );
                self.path_cache.insert(
                    format!("{}{}{}", node.clone(), out_dac_seen, out_fft_seen),
                    val,
                );
                val
            })
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

    #[test]
    fn test_rack_pt2() -> Result<()> {
        let mut rack = Rack::from_file("input/day11_sample_2.txt")?;

        assert_eq!(rack.nr_problematic_paths_to_out("svr", false, false), 2);

        Ok(())
    }
    #[test]
    fn test_day11_2() -> Result<()> {
        let mut rack = Rack::from_file("input/day11_input.txt")?;

        assert_eq!(rack.nr_problematic_paths_to_out("svr", false, false), 332052564714990);

        Ok(())
    }
}
