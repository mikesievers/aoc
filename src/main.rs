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

}
