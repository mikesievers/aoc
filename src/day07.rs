//! Advent of Code 2025 Day 07

use glam::IVec2;
use std::{collections::HashSet, fs::read_to_string};

pub struct Manifold {
    beam_pos: IVec2,
    splitters: HashSet<IVec2>,
}

impl Manifold {
    pub fn from_file(fname: &str) -> Self {
        let data = read_to_string(fname).unwrap();

        let beam_pos = *data
            .lines()
            .nth(0)
            .unwrap()
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == 'S')
            .map(|(x, _)| IVec2 {
                x: x as i32,
                y: 1_i32,
            })
            .collect::<Vec<_>>()
            .first()
            .unwrap();

        let splitters = data
            .lines()
            .filter(|x| !x.is_empty())
            .enumerate()
            .flat_map(move |(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '^')
                    .map(move |(x, _)| IVec2 {
                        x: x as i32,
                        y: y as i32,
                    })
            })
            .collect();

        Manifold {
            beam_pos,
            splitters,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Manifold;
    use glam::IVec2;

    #[test]
    fn test_manifold() {
        let manifold = Manifold::from_file("input/day07_sample.txt");

        assert_eq!(manifold.beam_pos, IVec2 { x: 7, y: 1 });
        assert!(manifold.splitters.contains(&IVec2 { x: 7, y: 2 }));
        assert!(manifold.splitters.contains(&IVec2 { x: 1, y: 14 }));
        assert!(!manifold.splitters.contains(&IVec2 { x: 2, y: 14 }));
        assert!(manifold.splitters.contains(&IVec2 { x: 3, y: 14 }));
    }
}
