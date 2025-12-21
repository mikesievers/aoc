//! Advent of Code 2025 Day 07
//! ```
//! use aoc::day07::Manifold;
//! let manifold = Manifold::from_file("input/day07_input.txt");
//! assert_eq!(manifold.n_splits(), 1550);
//! ```

use glam::IVec2;
use std::{collections::HashSet, fs::read_to_string};

pub struct Manifold {
    height: usize,
    beam_pos: IVec2,
    splitters: HashSet<IVec2>,
}

impl Manifold {
    pub fn from_file(fname: &str) -> Self {
        let data = read_to_string(fname).unwrap();

        let height = data.lines().filter(|x| !x.is_empty()).count();

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
            height,
            beam_pos,
            splitters,
        }
    }

    pub fn n_splits(&self) -> usize {
        // propagate or split the beams into the next row until the last row
        // for row in self.
        let mut beams: HashSet<IVec2> = HashSet::new();
        beams.insert(self.beam_pos);

        let mut n_splits = 0;

        for _ in 0..self.height {
            let mut new_beams: HashSet<IVec2> = HashSet::new();
            for beam in &beams {
                // The candidate coordinates are straight down
                let x = beam.x;
                let y = beam.y + 1;

                match self.splitters.contains(&IVec2 { x, y }) {
                    false => {
                        new_beams.insert(IVec2 { x, y });
                    }
                    true => {
                        n_splits += 1;
                        new_beams.insert(IVec2 { x: x - 1, y });
                        new_beams.insert(IVec2 { x: x + 1, y });
                    }
                }
            }
            beams = new_beams;
        }

        n_splits
    }
}

#[cfg(test)]
mod tests {
    use super::Manifold;
    use glam::IVec2;

    #[test]
    fn test_manifold() {
        let manifold = Manifold::from_file("input/day07_sample.txt");

        assert_eq!(manifold.height, 16);

        assert_eq!(manifold.beam_pos, IVec2 { x: 7, y: 1 });
        assert!(manifold.splitters.contains(&IVec2 { x: 7, y: 2 }));
        assert!(manifold.splitters.contains(&IVec2 { x: 1, y: 14 }));
        assert!(!manifold.splitters.contains(&IVec2 { x: 2, y: 14 }));
        assert!(manifold.splitters.contains(&IVec2 { x: 3, y: 14 }));

        assert_eq!(manifold.n_splits(), 21);
    }
}
