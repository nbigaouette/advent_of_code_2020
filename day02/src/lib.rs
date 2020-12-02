//! # Day 02: Password Philosophy
//!
//! [GitHub source](https://github.com/nbigaouette/advent_of_code_2019/tree/master/day02)
//!
//! [Benchmarking report](../../../day02/target/criterion/report/index.html)
//!
//! ## Part One
//!
//! Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via
//! [toboggan](https://en.wikipedia.org/wiki/Toboggan).
//!
//! The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with our computers;
//! we can't log in!" You ask if you can take a look.
//!
//! Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the
//! Official Toboggan Corporate Policy that was in effect when they were chosen.
//!
//! To try to debug the problem, they have created a list (your puzzle input) of _passwords_ (according to the corrupted
//! database) and _the corporate policy when that password was set_.
//!
//! For example, suppose you have the following list:
//!
//! ```text    
//! 1-3 a: abcde
//! 1-3 b: cdefg
//! 2-9 c: ccccccccc
//! ```
//!
//! Each line gives the password policy and then the password. The password policy indicates the lowest and highest
//! number of times a given letter must appear for the password to be valid. For example, `1-3 a` means that the
//! password must contain `a` at least `1` time and at most `3` times.
//!
//! In the above example, `_2_` passwords are valid. The middle password, `cdefg`, is not; it contains no instances
//! of `b`, but needs at least `1`. The first and third passwords are valid: they contain one `a` or nine `c`, both
//! within the limits of their respective policies.
//!
//! _How many passwords are valid_ according to their policies?
//!
//! ## Part Two
//!
//!
//!

use std::fmt::Debug;

pub use anyhow::{Context, Result};
use shrinkwraprs::Shrinkwrap;

pub mod initial;
pub use crate::initial::Day02Initial;

#[derive(Debug, Shrinkwrap, PartialEq)]
pub struct Day02Entry(usize);

type Day02SolutionPart1 = i64;
type Day02SolutionPart2 = i64;

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

pub fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = Day02Entry> + 'a {
    input
        .lines()
        .map(|line| Day02Entry(line.trim().parse().expect("Invalid entry")))
}

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<'a, SolutionPart1 = Day02SolutionPart1, SolutionPart2 = Day02SolutionPart2>
                + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        unimplemented!()
        // vec![
        //     Box::new(Day02Initial::new(PUZZLE_INPUT)),
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

        let parsed: Vec<Day02Entry> = parse_input(PUZZLE_INPUT).collect();
        assert_eq!(parsed.len(), 0);
        assert_eq!(
            &parsed[0..5],
            &[
                //
                Day02Entry(0),
            ]
        );
    }
}
