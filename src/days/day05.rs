// The data to parse for day 05 will be used

// to explore the nom crate for data parsing.

// See https://docs.rs/nom/8.0.0/nom/index.html

use nom::{
    bytes::complete::tag, character::complete::{char, i64, line_ending, newline, one_of, space1}, combinator::{map, recognize}, error::Error, multi::{many0, many1, separated_list1}, sequence::{delimited, preceded, terminated, tuple}, IResult, Parser
};

use std::fs::{self, read_to_string};
#[derive(Debug, PartialEq, Eq)]
pub struct Almanac {
    pub seeds: Vec<i64>,
    // Maps are
    // destination range start, source range start, range length
    pub seed_to_soil: Vec<Vec<i64>>,
    pub soil_to_fertilizer: Vec<Vec<i64>>,
    pub fertilizer_to_water: Vec<Vec<i64>>,
    pub water_to_light: Vec<Vec<i64>>,
    pub light_to_temperature: Vec<Vec<i64>>,
    pub temperature_to_humidity: Vec<Vec<i64>>,
    pub humidity_to_location: Vec<Vec<i64>>,
}

impl Almanac {
    pub fn from_file(fname: &str) -> Self {
        let input = fs::read_to_string(fname).expect("Could not read input file.");

        let (
            _,
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

    pub fn perform_map(&self, map: &Vec<Vec<i64>>, input: i64) -> i64 {
        for line in map {
            let (dst_base, src_base, len) = (line[0], line[1], line[2]);
            if (src_base..src_base + len).contains(&input) {
                return dst_base + (input - src_base);
            }
        }
        // If no mapping was possible, return the input
        input
    }

    pub fn map_vector(&self, map: &Vec<Vec<i64>>, input: &Vec<i64>) -> Vec<i64> {
        input
            .iter()
            .map(|val| self.perform_map(&map, *val))
            .collect()
    }

    pub fn seed_to_location(&self) -> Vec<i64> {
        let soils = self.map_vector(&self.seed_to_soil, &self.seeds);
        let fertilizers = self.map_vector(&self.soil_to_fertilizer, &soils);
        let waters = self.map_vector(&self.fertilizer_to_water, &fertilizers);
        let lights = self.map_vector(&self.water_to_light, &waters);
        let temperatures = self.map_vector(&self.light_to_temperature, &lights);
        let humidities = self.map_vector(&self.temperature_to_humidity, &temperatures);
        self.map_vector(&self.humidity_to_location, &humidities)
    }

    pub fn minimum_location(&self) -> i64 {
        *self.seed_to_location().iter().min().unwrap()
    }
}

#[test]
fn test_perform_map() {
    let almanac = Almanac::from_file("resources/day05_sample.txt");
    assert_eq!(almanac.perform_map(&almanac.seed_to_soil, 79), 81);
    assert_eq!(
        almanac.map_vector(&almanac.seed_to_soil, &almanac.seeds),
        [81, 14, 57, 13]
    );
    assert_eq!(almanac.seed_to_location(), [82, 43, 86, 35]);
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

fn decimal_line(input: &str) -> IResult<&str, Vec<i64>> {
    terminated(separated_list1(space1, i64), tag("\n")).parse(input)
}

fn seed_section(input: &str) -> IResult<&str, Vec<i64>> {
    preceded(tag("seeds: "), decimal_line).parse(input)
}

// Parse one section like
// seed-to-soil map:
// 50 98 2
// 52 50 48

fn parse_named_section(section_name: &str) -> impl Fn(&str) -> IResult<&str, Vec<Vec<i64>>> {
    let header = format!("\n{} map:", section_name);

    move |input: &str| {
        preceded((tag(header.as_str()), tag("\n")), many1(decimal_line)).parse(input)
    }
}

#[test]

fn test_parsing() {
    let mystr = "10000";
    let res: i64 = i64::<&str, Error<&str>>(mystr).unwrap().1;
    assert_eq!(res, 10000);

    let mystr = "1 2 3\n";
    let res = decimal_line.parse(mystr).unwrap();
    assert_eq!(res.1, vec![1, 2, 3]);
    assert_eq!(res.0, "");

    let input = read_to_string("resources/day05_sample.txt").unwrap();
    let seeds_res = seed_section.parse(input.as_str()).unwrap();
    assert_eq!(seeds_res.1, vec![79, 14, 55, 13]);

    let seed_to_soil_res = parse_named_section("seed-to-soil")(seeds_res.0).unwrap();
    assert_eq!(seed_to_soil_res.1, vec![vec![50, 98, 2], vec![52, 50, 48]]);
}
