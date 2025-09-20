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
            .map(|record| count_matches_brute_force(record.chars.as_str(), &record.groups))
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

fn count_matches_brute_force(line: &str, groups: &Vec<usize>) -> usize {
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

#[test]
fn test_count_matches_brute_force() {
    let str1 = "???.###";
    assert_eq!(count_matches_brute_force(str1, &Vec::from([1, 3])), 3);
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

fn count_matches(input: &str, groups: &Vec<usize>) -> Option<usize> {
    // If all groups have been used up, this is a valid combination
    if groups.len() == 0 {
        return Some(1);
    };

    // The first group is the current group
    let grp_size = groups[0];

    let mut matches = 0;
    let mut last_spring_was_broken = false;

    // If there are not enough characters left for the current group, it can't
    // be a match
    for cindex in 0..=(input.len() - grp_size) {
        let cinput = &input[cindex..];
        if cinput.len() < grp_size {
            return None;
        }

        // If the last spring seen was broken, we are in the middle of a group
        if last_spring_was_broken {
            continue;
        } else {
            match cinput.chars().nth(0).unwrap() {
                '#' => {
                    last_spring_was_broken = true;
                }
                _ => {
                    last_spring_was_broken = false;
                }
            }
        }

        // Now we know we are not inside a group and can try to match
        if is_group_matching(&cinput.to_string(), grp_size) {
            // Determine remaining string, call with remaining groups
            match cinput.len() as i64 - (cindex as i64 + grp_size as i64) {
                ..=0 => match groups.len() {
                    1 => {
                        matches += 1;
                    }
                    _ => {
                        return None;
                    }
                },
                1.. => {
                    let rest_str = &cinput[2..];
                    let rest_grps = groups.iter().skip(1).map(|&g| g).collect::<Vec<usize>>();
                    if let Some(restmatches) = count_matches(rest_str, &rest_grps) {
                        matches += restmatches;
                    } else {
                        return None;
                    }
                }
            }
        } else {
        }
    }

    Some(matches)
}

#[test]
fn test_count_matches() {
    assert_eq!(count_matches("#", &Vec::from([1])), Some(1));
    assert_eq!(count_matches("?", &Vec::from([1])), Some(1));
    assert_eq!(count_matches(".", &Vec::from([1])), Some(0));
    assert_eq!(count_matches("....", &Vec::from([1])), Some(0));
    assert_eq!(count_matches(".?..", &Vec::from([1])), Some(1));
    assert_eq!(count_matches(".?.?.", &Vec::from([1])), Some(2));
    assert_eq!(count_matches(".?.?", &Vec::from([1])), Some(2));
    assert_eq!(count_matches(".#.....", &Vec::from([1])), Some(1));
    assert_eq!(count_matches(".....#.", &Vec::from([1])), Some(1));
    assert_eq!(count_matches("......#", &Vec::from([1])), Some(1));
    assert_eq!(count_matches("??.?..#", &Vec::from([1])), Some(4));

    assert_eq!(count_matches("....###", &Vec::from([1, 3])), Some(0));
    assert_eq!(count_matches("??.###", &Vec::from([1, 3])), Some(2));
    assert_eq!(count_matches(".??.###", &Vec::from([1, 3])), Some(2));
    assert_eq!(count_matches("???.###", &Vec::from([1, 3])), Some(3));
}

fn is_group_matching(input: &String, grp_size: usize) -> bool {
    // A string that is too short can't match
    if input.len() < grp_size {
        return false;
    };

    // this is what a matching default group would look like
    let group_string = "#".repeat(grp_size);

    // prepare a substring of the first grp_size characters where it is assumed
    // that '?' means defect spring (otherwise it would not match)
    let assume_defect = input
        .chars()
        .take(grp_size)
        .map(|c| if c == '?' { '#' } else { c })
        .collect::<String>();

    // If the string with assumed defect does not match the group string, it's
    // not a match
    if assume_defect != group_string {
        return false;
    }

    // if the following character is also a '#', it's not a match
    // (the group would have needed to be longer)
    if input.len() > grp_size && input.chars().nth(grp_size) == Some('#') {
        return false;
    } else {
        // This was the end of the string or the following char is . or ?
        return true;
    }
}

#[test]
fn test_is_group_matching() {
    assert_eq!(is_group_matching(&".#.".to_string(), 2), false);
    assert_eq!(is_group_matching(&"?#.".to_string(), 2), true);
    assert_eq!(is_group_matching(&"##.".to_string(), 2), true);
    assert_eq!(is_group_matching(&"##?".to_string(), 2), true);
    assert_eq!(is_group_matching(&"###".to_string(), 2), false);
    assert_eq!(is_group_matching(&"##".to_string(), 2), true);
}
