//! # Advent of Code 2023
//!
//! Consult the individual days for code in the `days` module.
//!
mod days;

use days::*;

fn main() {
    println!("The following outputs the results of all days.");

    // Day 1
    let day01_p1 = day01::sum_calibration_values("resources/day01_input.txt");
    println!("Day 01 part 01: {}", day01_p1);

    let day01_p2 = day01::sum_subbed_calibration_values("resources/day01_input.txt");
    println!("Day 01 part 02: {}", day01_p2); // 55902 is wrong

    // Day 2
    let day02_p1 = day02::sum_possible_games("resources/day02_input.txt");
    println!("Day 02 part 01: {}", day02_p1);

    let day02_p2 = day02::sum_power("resources/day02_input.txt");
    println!("Day 02 part 01: {}", day02_p2);

    // Day 3
    let day03_p1 = day03::sum_of_parts("resources/day03_input.txt");
    println!("Day 03 part 01: {}", day03_p1);
    // 9633538 is too high!
    let day03_p2 = day03::sum_gear_products("resources/day03_input.txt");
    println!("Day 03 part 02: {}", day03_p2);
}
