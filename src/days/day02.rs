use std::fs::*;
use std::io::{BufRead, BufReader};

pub const INPUT: &str = "resources/day02_input.txt";
pub const SAMPLE: &str = "resources/day02_sample.txt";

pub struct Set {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}
pub struct Game {
    pub nr: usize,
    pub sets: Vec<Set>,
}

impl Set {
    pub fn is_set_possible(&self, red: usize, green: usize, blue: usize) -> bool {
        (self.red <= red) && (self.green <= green) && (self.blue <= blue)
    }
}

impl Game {
    pub fn is_game_possible(&self, red: usize, green: usize, blue: usize) -> bool {
        let is_possible = self
            .sets
            .iter()
            .map(|set| {
                let set_possible = set.is_set_possible(red, green, blue);
                set_possible
            })
            .all(|x| {
                x
            });
        is_possible
    }
    pub fn from_line(line: &str) -> Self {
        // Split game lines such as
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        // into game number and reg/green/blue number of cubes

        // game/ sets
        let v: Vec<&str> = line.trim().splitn(2, ':').collect();

        // game number is v[0] "Game 1"
        let game_nr_str: Vec<&str> = v[0].split_whitespace().collect();
        let game_nr: usize = game_nr_str[1].parse().unwrap();

        // v[1] remains " 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        // this is to be split into the sets and each set into the number of cubes
        let mut sets = Vec::<Set>::new();
        let _: Vec<_> = v[1]
            .trim()
            .split(';')
            .map(|set| {
                let mut nr_red = 0_usize;
                let mut nr_green = 0_usize;
                let mut nr_blue = 0_usize;

                //let y: Vec<_> = set.trim().split(',').map(|cube| {
                let _: Vec<_> = set
                    .trim()
                    .split(',')
                    .map(|cube| {
                        let cube_type: Vec<&str> = cube.trim().split_whitespace().collect();
                        match cube_type[1] {
                            "red" => {
                                nr_red = cube_type[0].parse().unwrap();
                            }
                            "blue" => {
                                nr_blue = cube_type[0].parse().unwrap();
                            }
                            "green" => {
                                nr_green = cube_type[0].parse().unwrap();
                            }
                            _ => {
                                panic!("Unkown color found: {}", cube_type[1]);
                            }
                        }
                    })
                    .collect();

                sets.push(Set {
                    red: nr_red,
                    green: nr_green,
                    blue: nr_blue,
                });
            })
            .collect();
        Game {
            nr: game_nr,
            sets: sets,
        }
    }
}

pub fn read_input(fname: &str) -> Vec<String> {
    let file = File::open(fname).unwrap();
    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .map(|line| String::from(line.unwrap().trim()))
        .collect();

    lines
}
