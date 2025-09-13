use itertools::Itertools;
use nom::multi::count;
use std::fs::read_to_string;

// Springs:
// . : operational
// # : broken
// ? : unknown state
// numbers: 1,2,3 : contiguous groups of damaged springs
// => how many possible combinations for each observation

// Approach:
// - find all possible match positions for the first group
//   - For each matching position, find the matching positions of the remaining subgroups
//   - For the last subgroup, return a match if possible, otherwise the number of matching positions

#[derive(Debug)]
pub struct Ledger {
    data: String,
    records: Vec<Record>,
}

#[derive(Debug)]
struct Record {
    chars: String,
    groups: Vec<usize>,
}

impl Ledger {
    pub fn from_file(fname: &str) -> Self {
        let data = read_to_string(fname).expect("Could not read input file.");

        let mut records = Vec::new();

        for line in data.lines() {
            let mut split = line.split_whitespace();
            let chars = split.next().unwrap().to_string();
            let groups = split
                .next()
                .unwrap()
                .split(',')
                .map(|c| c.parse().unwrap())
                .collect::<Vec<usize>>();

            records.push(Record { chars, groups });
        }
        Ledger { data, records }
    }

    pub fn sum_match_counts(&self) -> usize {
        self.records
            .iter()
            .map(|record| count_matches(record.chars.as_str(), &record.groups))
            .sum()
    }
}

#[test]
fn test_records() {
    let ledger = Ledger::from_file("resources/day12_sample.txt");

    println!("records:\n{}", ledger.data);
    println!("Ledger:\n{:?}", ledger);
    assert_eq!(ledger.sum_match_counts(), 21);
}

fn count_matches(line: &str, groups: &Vec<usize>) -> usize {
    let mut count = 0;
    let mut chars: Vec<char> = line.chars().collect();
    let jokers: Vec<usize> = chars
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == '?')
        .map(|(idx, _)| idx)
        .collect();

    // loop over all possible combinations

    let states = ['.', '#'];
    let combinations = std::iter::repeat(states.iter().cloned())
        .take(jokers.len())
        .multi_cartesian_product();

    for combination in combinations {
        for (idx, state) in combination.iter().enumerate() {
            chars[jokers[idx]] = *state;
        }
        let groups_found = get_groups(&chars);
        if groups_found == *groups {
            count += 1;
        }
    }
    count
}

fn get_groups(chars: &Vec<char>) -> Vec<usize> {
    let mut groups = Vec::new();

    let mut current_group = 0;

    for char in chars {
        match char {
            '#' => current_group += 1,
            '.' if current_group > 0 => {
                groups.push(current_group);
                current_group = 0;
            }
            _ => {}
        }
    }
    if current_group > 0 {
        groups.push(current_group);
    }

    groups
}

#[test]
fn test_get_groups() {
    let grps = get_groups(&"##..#.###...#".chars().collect());
    assert_eq!(grps, Vec::<usize>::from([2, 1, 3, 1]));
}

#[test]
fn test_count_matches() {
    let str1 = "???.###";
    assert_eq!(count_matches(str1, &Vec::from([1, 3])), 3);
}
