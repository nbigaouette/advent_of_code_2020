//! # Day 04:
//!
//! [GitHub source](https://github.com/nbigaouette/advent_of_code_2019/tree/master/day04)
//!
//! [Benchmarking report](../../../day04/target/criterion/report/index.html)
//!
//! ## Part One
//!
//!
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
