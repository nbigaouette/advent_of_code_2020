//! # Day 04: Passport Processing
//!
//! [GitHub source](https://github.com/nbigaouette/advent_of_code_2019/tree/master/day04)
//!
//! [Benchmarking report](../../../day04/target/criterion/report/index.html)
//!
//! ## Part One
//!
//! You arrive at the airport only to realize that you grabbed your North Pole Credentials instead of your passport.
//! While these documents are extremely similar, North Pole Credentials aren't issued by a country and therefore
//! aren't actually valid documentation for travel in most of the world.
//!
//! It seems like you're not the only one having problems, though; a very long line has formed for the automatic
//! passport scanners, and the delay could upset your travel itinerary.
//!
//! Due to some questionable network security, you realize you might be able to solve both of these problems at
//! the same time.
//!
//! The automatic passport scanners are slow because they're having trouble _detecting which passports have all
//! required fields_. The expected fields are as follows:
//!
//! * `byr` (Birth Year)
//! * `iyr` (Issue Year)
//! * `eyr` (Expiration Year)
//! * `hgt` (Height)
//! * `hcl` (Hair Color)
//! * `ecl` (Eye Color)
//! * `pid` (Passport ID)
//! * `cid` (Country ID)
//!
//! Passport data is validated in batch files (your puzzle input). Each passport is represented as a sequence
//! of `key:value` pairs separated by spaces or newlines. Passports are separated by blank lines.
//!
//! Here is an example batch file containing four passports:
//!
//! ```text
//! ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
//! byr:1937 iyr:2017 cid:147 hgt:183cm
//!
//! iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
//! hcl:#cfa07d byr:1929
//!
//! hcl:#ae17e1 iyr:2013
//! eyr:2024
//! ecl:brn pid:760753108 byr:1931
//! hgt:179cm
//!
//! hcl:#cfa07d eyr:2025 pid:166559648
//! iyr:2011 ecl:brn hgt:59in
//! ```
//!
//! The first passport is _valid_ - all eight fields are present. The second passport is _invalid_ - it is missing
//! `hgt` (the Height field).
//!
//! The third passport is interesting; the _only missing field_ is `cid`, so it looks like data from North Pole
//! Credentials, not a passport at all! Surely, nobody would mind if you made the system temporarily ignore
//! missing `cid` fields. Treat this "passport" as _valid_.
//!
//! The fourth passport is missing two fields, `cid` and `byr`. Missing `cid` is fine, but missing any other
//! field is not, so this passport is _invalid_.
//!
//! According to the above rules, your improved system would report `_2_` valid passports.
//!
//! Count the number of _valid_ passports - those that have all required fields. Treat `cid` as optional. _In
//! your batch file, how many passports are valid?_
//!
//! ## Part Two
//!
//!
//!

use std::fmt::Debug;

pub use anyhow::{Context, Result};
use shrinkwraprs::Shrinkwrap;

pub mod initial;
pub use crate::initial::Day04Initial;

#[derive(Debug, Shrinkwrap, PartialEq)]
pub struct Day04Entry(usize);

type Day04SolutionPart1 = i64;
type Day04SolutionPart2 = i64;

pub trait AoC<'a>: Debug {
    type SolutionPart1;
    type SolutionPart2;

    fn description(&self) -> &'static str {
        "None"
    }

    fn new(input: &'a str) -> Self
    where
        Self: Sized;

    fn solution_part1(&self) -> Self::SolutionPart1 {
        unimplemented!()
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        unimplemented!()
    }
}

pub fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = Day04Entry> + 'a {
    input
        .lines()
        .map(str::trim)
        .map(|line| Day04Entry(line.trim().parse().expect("Invalid entry")))
}

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<'a, SolutionPart1 = Day04SolutionPart1, SolutionPart2 = Day04SolutionPart2>
                + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        unimplemented!()
        // vec![
        //     Box::new(Day04Initial::new(PUZZLE_INPUT)),
        // ]
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use env_logger;
    use pretty_assertions::assert_eq;

    use super::*;

    pub fn init_logger() {
        env::var("RUST_LOG")
            .or_else(|_| -> Result<String, ()> {
                let rust_log = "debug".to_string();
                println!("Environment variable 'RUST_LOG' not set.");
                println!("Setting to: {}", rust_log);
                env::set_var("RUST_LOG", &rust_log);
                Ok(rust_log)
            })
            .unwrap();
        let _ = env_logger::try_init();
    }

    #[test]
    fn parse() {
        init_logger();

        unimplemented!();

        let parsed: Vec<Day04Entry> = parse_input(PUZZLE_INPUT).collect();
        assert_eq!(parsed.len(), 0);
        assert_eq!(
            &parsed[0..5],
            &[
                //
                Day04Entry(0),
            ]
        );
    }
}
