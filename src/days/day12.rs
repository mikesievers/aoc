use itertools::Itertools;
use nom::multi::count;
use onig::{Regex, RegexOptions, Syntax};
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
            .map(|record| count_matches(record.chars.as_str(), &record.groups, 0))
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

fn count_matches(line: &str, groups: &Vec<usize>, groups_matched: usize) -> usize {
    let text = "aaab";
    let re = Regex::with_options(
        r"(a+)(b+)",
        RegexOptions::REGEX_OPTION_NONE,
        Syntax::default(),
    )
    .unwrap();

    // Collect all matches
    let mut results = Vec::new();
    let mut region = onig::Region::new();

    // `search_with_options` gives us control over backtracking
    let mut pos = 0;
    while pos <= text.len() {
        match re.search_with_options(
            text,
            pos,
            text.len(),
            onig::SearchOptions::SEARCH_OPTION_NONE,
            Some(&mut region),
        ) {
            Some(_) => {
                let g1 = region.pos(1).map(|(s, e)| &text[s..e]);
                let g2 = region.pos(2).map(|(s, e)| &text[s..e]);
                results.push((g1, g2));

                // Advance one step to force exploration of alternative paths
                pos += 1;
            }
            None => break,
        }
    }

    println!("All possibilities:");
    for r in results {
        println!("{:?}", r);
    }
    42
}

#[test]
fn test_count_matches() {
    let str1 = "?.###";
    assert_eq!(count_matches(str1, &Vec::from([3]), 0), 1);

    let str1 = "???.###";
    assert_eq!(count_matches(str1, &Vec::from([1]), 0), 0); // The ### are not matched
    assert_eq!(count_matches(str1, &Vec::from([1, 3]), 0), 3);

    let str2 = "";
    assert_eq!(count_matches(str2, &Vec::from([1]), 0), 0);

    let str3 = "???.##";
    assert_eq!(count_matches(str3, &Vec::from([1, 3]), 0), 0);

    let str4a = ".??..??...?##.";
    assert_eq!(count_matches(str4a, &Vec::from([1, 1, 3]), 0), 4);

    let str4 = "????.######..#####.";
    assert_eq!(count_matches(str4, &Vec::from([1, 6, 5]), 0), 4);

    let str5a = "???????";
    assert_eq!(count_matches(str5a, &Vec::from([2, 1]), 0), 10);

    let str5b = "?###???";
    assert_eq!(count_matches(str5b, &Vec::from([3, 1]), 0), 2);

    let str5c = "?####????????";
    assert_eq!(count_matches(str5c, &Vec::from([4, 2, 1]), 0), 10);

    let str5e = "###????????";
    assert_eq!(count_matches(str5e, &Vec::from([3]), 0), 1);

    let str5d = "###????????";
    assert_eq!(count_matches(str5d, &Vec::from([3, 2, 1]), 0), 10);

    let str5 = "?###????????";
    assert_eq!(count_matches(str5, &Vec::from([3, 2, 1]), 0), 10);
}
