//! # Advent of Code 2023
//!
//! Consult the individual days for code in the `days` module.
//!
mod days;

use days::*;

fn main() {
    println!("The following outputs the results of all days.");

    // Day 01
    let day01_p1 = day01::sum_calibration_values("resources/day01_input.txt");
    println!("Day 01 part 01: {}", day01_p1);

    let day01_p2 = day01::sum_subbed_calibration_values("resources/day01_input.txt");
    println!("Day 01 part 02: {}", day01_p2); // 55902 is wrong

    // Day 02
    let day02_p1 = day02::sum_possible_games("resources/day02_input.txt");
    println!("Day 02 part 01: {}", day02_p1);

    let day02_p2 = day02::sum_power("resources/day02_input.txt");
    println!("Day 02 part 01: {}", day02_p2);

    // Day 03
    let day03_p1 = day03::sum_of_parts("resources/day03_input.txt");
    println!("Day 03 part 01: {}", day03_p1);
    // 9633538 is too high!
    let day03_p2 = day03::sum_gear_products("resources/day03_input.txt");
    println!("Day 03 part 02: {}", day03_p2);

    // Day 04
    let day04_p1_stack = day04::Stack::from("resources/day04_input.txt");
    println!("Day 04 part 01: {}", day04_p1_stack.sum_scores());

    // Part2: 9425061
    //println!("Day 04 part 02: {}", day04_p1_stack.sum_all_cards());
    println!("Day 04 part 02: 9425061 (Not calculated on the fly because of slow recursion)");

    // Day 05
    // Part1: 313045984
    let almanac = day05::Almanac::from_file("resources/day05_input.txt");
    println!("Day 05 part 01: {}", almanac.minimum_location());

    // Part2:
    // println!(
    //     "Day 05 part 02: {}",
    //     almanac.minimum_location_in_ranges().unwrap()
    // );
    println!(
        "Day 05 part 02: 20283860  (Not calculated dynamically because long running brute force was performed)"
    );

    // Day 06
    // Part1: 219849
    let race_log = day06::RaceLog::from_file("resources/day06_input.txt");
    println!("Day 06 part 01: {}", race_log.winning_ranges_product());
    // Part2: 29432455
    let race_log = day06::RaceLog::from_bad_kerning_file("resources/day06_input.txt");
    println!("Day 06 part 01: {}", race_log.winning_ranges_product());

    // Day 07
    // Part1: 252052080
    let game = day07::Game::from_file("resources/day07_input.txt");
    println!("Day 07 part 01: {}", game.winnings());

    // Part 2: 252898370
    let game2 = day07p2::Game::from_file("resources/day07_input.txt");
    println!("Day 07 part 02: {}", game2.winnings());

    // Day 08
    // Part1: 22411
    let map = day08::Map::from_file("resources/day08_input.txt");
    println!("Day 08 part 01: {}", map.path_length());
}
