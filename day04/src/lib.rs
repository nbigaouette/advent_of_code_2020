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

pub mod initial;
pub use crate::initial::Day04Initial;

type Day04Entry<'a> = PassportBuilder<'a>;

type Day04SolutionPart1 = usize;
type Day04SolutionPart2 = usize;

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

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BirthYear<'a>(&'a str);

#[derive(Debug, Default, Clone, PartialEq)]
pub struct IssueYear<'a>(&'a str);

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ExpirationYear<'a>(&'a str);

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Height<'a>(&'a str);

#[derive(Debug, Default, Clone, PartialEq)]
pub struct HairColor<'a>(&'a str);

#[derive(Debug, Default, Clone, PartialEq)]
pub struct EyeColor<'a>(&'a str);

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PassportId<'a>(&'a str);

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CountryId<'a>(&'a str);

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PassportBuilder<'a> {
    /// Height
    hgt: Option<Height<'a>>,
    /// Issue Year
    iyr: Option<IssueYear<'a>>,
    /// Expiration Year
    eyr: Option<ExpirationYear<'a>>,
    /// Hair Color
    hcl: Option<HairColor<'a>>,
    /// Eye Color
    ecl: Option<EyeColor<'a>>,
    /// Passport ID
    pid: Option<PassportId<'a>>,
    /// Birth Year
    byr: Option<BirthYear<'a>>,
    /// Country ID
    cid: Option<CountryId<'a>>,
}

impl<'a> PassportBuilder<'a> {
    fn new() -> PassportBuilder<'a> {
        Default::default()
    }

    fn parse_line(mut self, line: &'a str) -> PassportBuilder<'a> {
        line.split(' ').for_each(|pair| {
            let mut pair_iter = pair.split(':');
            let key = pair_iter.next().expect("a key");
            let value = pair_iter.next().expect("a value");

            match key {
                "byr" => self.byr = Some(BirthYear(value)),
                "iyr" => self.iyr = Some(IssueYear(value)),
                "eyr" => self.eyr = Some(ExpirationYear(value)),
                "hgt" => self.hgt = Some(Height(value)),
                "hcl" => self.hcl = Some(HairColor(value)),
                "ecl" => self.ecl = Some(EyeColor(value)),
                "pid" => self.pid = Some(PassportId(value)),
                "cid" => self.cid = Some(CountryId(value)),
                _ => unreachable!(),
            }
        });

        self
    }
}

pub fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = PassportBuilder<'a>> + 'a {
    input.trim().split("\n\n").map(str::trim).map(|block| {
        let passport_builder = block.lines().fold(PassportBuilder::new(), |builder, line| {
            builder.parse_line(line)
        });

        passport_builder
    })
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

    pub const EXAMPLE_INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

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

        let passport_builders: Vec<Day04Entry> = parse_input(EXAMPLE_INPUT).collect();
        assert_eq!(
            passport_builders,
            &[
                PassportBuilder {
                    hgt: Some(Height("183cm")),
                    iyr: Some(IssueYear("2017")),
                    eyr: Some(ExpirationYear("2020")),
                    hcl: Some(HairColor("#fffffd")),
                    ecl: Some(EyeColor("gry")),
                    pid: Some(PassportId("860033327")),
                    byr: Some(BirthYear("1937")),
                    cid: Some(CountryId("147")),
                },
                PassportBuilder {
                    hgt: None,
                    iyr: Some(IssueYear("2013")),
                    eyr: Some(ExpirationYear("2023")),
                    hcl: Some(HairColor("#cfa07d")),
                    ecl: Some(EyeColor("amb")),
                    pid: Some(PassportId("028048884")),
                    byr: Some(BirthYear("1929")),
                    cid: Some(CountryId("350")),
                },
                PassportBuilder {
                    hgt: Some(Height("179cm")),
                    iyr: Some(IssueYear("2013")),
                    eyr: Some(ExpirationYear("2024")),
                    hcl: Some(HairColor("#ae17e1")),
                    ecl: Some(EyeColor("brn")),
                    pid: Some(PassportId("760753108")),
                    byr: Some(BirthYear("1931")),
                    cid: None,
                },
                PassportBuilder {
                    hgt: Some(Height("59in")),
                    iyr: Some(IssueYear("2011")),
                    eyr: Some(ExpirationYear("2025")),
                    hcl: Some(HairColor("#cfa07d")),
                    ecl: Some(EyeColor("brn")),
                    pid: Some(PassportId("166559648")),
                    byr: None,
                    cid: None,
                },
            ]
        );
    }
}
