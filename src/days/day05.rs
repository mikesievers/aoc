// The data to parse for day 05 will be used

// to explore the nom crate for data parsing.

// See https://docs.rs/nom/8.0.0/nom/index.html

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{char, i32, line_ending, newline, one_of},
    combinator::{map, recognize},
    error::Error,
    multi::{many0, many1, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
};

use std::fs::{self, read_to_string};
#[derive(Debug, PartialEq, Eq)]
pub struct Almanac {
    pub seeds: Vec<i32>,
    // Maps are
    // destination range start, source range start, range length
    pub seed_to_soil: Vec<Vec<i32>>,
    pub soil_to_fertilizer: Vec<Vec<i32>>,
    pub fertilizer_to_water: Vec<Vec<i32>>,
    pub water_to_light: Vec<Vec<i32>>,
    pub light_to_temperature: Vec<Vec<i32>>,
    pub temperature_to_humidity: Vec<Vec<i32>>,
    pub humidity_to_location: Vec<Vec<i32>>,
}

impl Almanac {
    fn from_file(fname: &str) -> Self {
        let input = fs::read_to_string(fname).expect("Could not read input file.");

        let (
            input,
            (
                seeds,
                seed_to_soil,
                soil_to_fertilizer,
                fertilizer_to_water,
                water_to_light,
                light_to_temperature,
                temperature_to_humidity,
                humidity_to_location,
            ),
        ) = (
            seed_section,
            parse_named_section("seed-to-soil"),
            parse_named_section("soil-to-fertilizer"),
            parse_named_section("fertilizer-to-water"),
            parse_named_section("water-to-light"),
            parse_named_section("light-to-temperature"),
            parse_named_section("temperature-to-humidity"),
            parse_named_section("humidity-to-location"),
        )
            .parse(input.as_str())
            .unwrap();

        Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }

    pub fn perform_map(&self, map: &Vec<Vec<i32>>, input: i32) -> i32 {
        for line in map {
            let (dst_base, src_base, len) = (line[0], line[1], line[2]);
            if (src_base..src_base+len).contains(&input) {
                return dst_base + ( input - src_base);
            }

        }
        // If no mapping was possible, return the input
        input
    }

    pub fn map_vector(&self, map: &Vec<Vec<i32>>, input: &Vec<i32>) -> Vec<i32> {
        input.iter()
        .map( |val|
            self.perform_map(&map, *val)
        ).collect()


    }
}

#[test]
fn test_perform_map() {
    let almanac = Almanac::from_file("resources/day05_sample.txt");
    assert_eq!(almanac.perform_map(&almanac.seed_to_soil, 79), 81);
    assert_eq!(almanac.map_vector(&almanac.seed_to_soil, &almanac.seeds), [81,14,57,13]);
}

// Parser
// Parse a file like:
//
// seeds: 79 14 55 13
// seed-to-soil map:
// 50 98 2
// 52 50 48
// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15

#[test]

fn test_parse_input() {
    let almanac = Almanac::from_file("resources/day05_sample.txt");

    assert_eq!(almanac.seeds, [79, 14, 55, 13]);
    assert_eq!(almanac.seed_to_soil, [[50, 98, 2], [52, 50, 48]]);
    assert_eq!(
        almanac.soil_to_fertilizer,
        [[0, 15, 37], [37, 52, 2], [39, 0, 15]]
    );
    assert_eq!(almanac.humidity_to_location, [[60, 56, 37], [56, 93, 4]]);
}

// nom experimentation area

fn decimal_line(input: &str) -> IResult<&str, Vec<i32>> {
    terminated(separated_list1(tag(" "), i32), line_ending).parse(input)
}

fn seed_section(input: &str) -> IResult<&str, Vec<i32>> {
    preceded(tag("seeds: "), decimal_line).parse(input)
}

// Parse one section like
// seed-to-soil map:
// 50 98 2
// 52 50 48

fn parse_named_section(section_name: &str) -> impl Fn(&str) -> IResult<&str, Vec<Vec<i32>>> {
    let header = format!("\r\n{} map:", section_name);

    move |input: &str| {
        preceded((tag(header.as_str()), line_ending), many1(decimal_line)).parse(input)
    }
}

#[test]

fn test_parsing() {
    let mystr = "10000";
    let res: i32 = i32::<&str, Error<&str>>(mystr).unwrap().1;
    assert_eq!(res, 10000);

    let mystr = "1 2 3\r\n";
    let res = decimal_line.parse(mystr).unwrap();
    assert_eq!(res.1, vec![1, 2, 3]);
    assert_eq!(res.0, "");

    let input = read_to_string("resources/day05_sample.txt").unwrap();
    let seeds_res = seed_section.parse(input.as_str()).unwrap();
    assert_eq!(seeds_res.1, vec![79, 14, 55, 13]);

    let seed_to_soil_res = parse_named_section("seed-to-soil")(seeds_res.0).unwrap();
    assert_eq!(seed_to_soil_res.1, vec![vec![50, 98, 2], vec![52, 50, 48]]);
}
