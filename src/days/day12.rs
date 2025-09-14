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

// This is a recursive counting of matches:
// - Consider the whole string, does position 0 fit the first group?
//   - If yes, count all the possibilities in which the following groups match the rest of the string.
//   - If not all groups match, don't count as a match
//   - => need to track the number of matching groups and the number of groups matching overall

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
            .map(
                |record| match count_matches(record.chars.as_str(), &record.groups, 0) {
                    Some((n, _)) => n,
                    None => 0,
                },
            )
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

fn count_matches(line: &str, groups: &Vec<usize>, groups_matched: usize) -> Option<(usize, usize)> {
    // "groups_matched tracks how many groups have been matched".
    // Results should only be reported as >0 if all groups have been matched.

    // When all groups have been consumed, count as a match
    // And allow recursion to wrap up
    if groups.len() == 0 {
        return Some((0, groups_matched + 1));
    }

    // If the remaining string is smaller than the group size, it can't be a match
    let grp_size = groups[0];
    if line.len() < grp_size {
        return Some((0, groups_matched));
    }

    let mut count = 0;
    let mut all_groups_matched = 0;
    let mut in_group = false; // Track whether we are in a group or not

    for skips in 0..=(line.len() - grp_size) {
        let sub_line = line.chars().skip(skips).collect::<String>();
        if in_group && sub_line.chars().nth(0).unwrap() == '#' {
            continue;
        } else {
            in_group = false;
        }
        match is_group_matching(&sub_line, grp_size) {
            None if count == 0 => return Some((0, groups_matched)), // the group has never matched, but needed to
            Some(true) => {
                // it's a match. Determine the rest of the string and count matches against the remaining possibilities.
                let remaining_groups = groups
                    .iter()
                    .skip(1)
                    .map(|g| g.clone())
                    .collect::<Vec<usize>>();
                let remaining_string = get_rest(&sub_line, grp_size);
                match remaining_string {
                    None => {
                        // When nothing is left, count as overall match and return. Otherwise, no match.
                        if remaining_groups.len() > 0 && count == 0 {
                            return None;
                        } else {
                            match in_group {
                                false => {
                                    return Some((count + 1, groups_matched + 1));
                                }
                                true => {
                                    return Some((count, groups_matched + 1));
                                }
                            }
                        }
                    }
                    Some(remains) => {
                        let mut matches = 0;
                        (matches, all_groups_matched) =
                            count_matches(&remains, &remaining_groups, groups_matched)?;
                        count += matches;
                    }
                }
            }
            Some(false) => {}
            _ => {}
        }
        // finally, track whether we are in a group by remembering if the current spring was broken
        if sub_line.chars().nth(0) == Some('#') {
            in_group = true
        }
    }

    match all_groups_matched {
        n => {
            if n == groups.len() {
                return Some((count, all_groups_matched));
            } else {
                return Some((0, all_groups_matched));
            }
        }
    }
}

#[test]
fn test_count_matches() {
    let str1 = "?.###";
    assert_eq!(count_matches(str1, &Vec::from([3]), 0), Some((1, 1)));

    let str1 = "???.###";
    assert_eq!(count_matches(str1, &Vec::from([1]), 0), None); // The three # are not matched
    assert_eq!(count_matches(str1, &Vec::from([1, 3]), 0), Some((3, 3)));

    let str2 = "";
    assert_eq!(count_matches(str2, &Vec::from([1]), 0), Some((0, 0)));

    let str3 = "???.##";
    assert_eq!(count_matches(str3, &Vec::from([1, 3]), 0), Some((0, 1)));

    let str3 = "?###????????";
    assert_eq!(count_matches(str3, &Vec::from([3, 2, 1]), 0), Some((0, 10)));
}

// Check whether the string at the current position matches the current group
fn is_group_matching(input: &String, grp_size: usize) -> Option<bool> {
    // A string that is too short can't match
    if input.len() < grp_size {
        return Some(false);
    };

    // this is what a matching default group would look like
    let group_string = "#".repeat(grp_size);

    // If the first character is marked as defect, it must be accounted for by a match
    let must_match = input.chars().nth(0).unwrap() == '#';

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
        match must_match {
            true => return None, // A match was mandatory, this means the string can't match
            false => return Some(false), // The string just does not match here, but that's OK
        }
    };

    // if the following character is also a '#', it's not a match
    // (the group would have needed to be longer)
    // Also, since all defects must be matched, this can't be counted
    if input.len() > grp_size && input.chars().nth(grp_size) == Some('#') {
        // If the current character is not a defect spring '#', it's not a match but that's OK
        if input.chars().nth(0) != Some('#') {
            return Some(false);
        } else {
            return None;
        }
    } else {
        // This was the end of the string or the following char is . or ?
        return Some(true);
    }
}

#[test]
fn test_is_group_matching() {
    assert_eq!(is_group_matching(&"".to_string(), 2), Some(false)); // empty string does not match

    assert_eq!(is_group_matching(&".#.".to_string(), 2), Some(false)); // Does not match, but could next time
    assert_eq!(is_group_matching(&"#.".to_string(), 2), None); // ... not it must match!

    assert_eq!(is_group_matching(&"?#.".to_string(), 2), Some(true)); // wildcard allows matching
    assert_eq!(is_group_matching(&"##.".to_string(), 2), Some(true)); // Trivial case
    assert_eq!(is_group_matching(&"##.".to_string(), 1), None); // must match! (match group too small)
    assert_eq!(is_group_matching(&"###".to_string(), 2), None); // must match! (match group too small)
    assert_eq!(is_group_matching(&"##".to_string(), 2), Some(true)); // end of string

    assert_eq!(is_group_matching(&".###".to_string(), 3), Some(false)); // does not have to match (but would next iteration)
    assert_eq!(is_group_matching(&"?###".to_string(), 3), Some(false)); // does not have to match (but would next iteration)
}

fn get_rest(input: &String, grp_size: usize) -> Option<String> {
    let rest = input.chars().skip(grp_size + 1).collect::<String>();
    if rest.len() > 0 { Some(rest) } else { None }
}

#[test]
fn test_get_rest() {
    assert_eq!(get_rest(&".#.12".to_string(), 2), Some("12".to_string()));
    assert_eq!(get_rest(&".#?#12".to_string(), 3), Some("12".to_string()));
    assert_eq!(get_rest(&".#?".to_string(), 3), None);
    assert_eq!(get_rest(&".#?".to_string(), 2), None);
    assert_eq!(get_rest(&"".to_string(), 2), None);
}
