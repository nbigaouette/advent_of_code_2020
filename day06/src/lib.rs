//! # Day 06: Custom Customs
//!
//! [GitHub source](https://github.com/nbigaouette/advent_of_code_2019/tree/master/day06)
//!
//! [Benchmarking report](../../../day06/target/criterion/report/index.html)
//!
//! ## Part One
//!
//! As your flight approaches the regional airport where you'll switch to a much larger plane,
//! [customs declaration forms](https://en.wikipedia.org/wiki/Customs_declaration) are distributed
//! to the passengers.
//!
//! The form asks a series of 26 yes-or-no questions marked `a` through `z`. All you need to do is
//! identify the questions for which _anyone in your group_ answers "yes". Since your group is just
//! you, this doesn't take very long.
//!
//! However, the person sitting next to you seems to be experiencing a language barrier and asks if
//! you can help. For each of the people in their group, you write down the questions for which they
//! answer "yes", one per line. For example:
//!
//! ```text
//! abcx
//! abcy
//! abcz
//! ```
//!
//! In this group, there are _`6`_ questions to which anyone answered "yes": `a`, `b`, `c`, `x`, `y`,
//! and `z`. (Duplicate answers to the same question don't count extra; each question counts at most once.)
//!
//! Another group asks for your help, then another, and eventually you've collected answers from
//! every group on the plane (your puzzle input). Each group's answers are separated by a blank line,
//! and within each group, each person's answers are on a single line. For example:
//!
//! ```text
//! abc
//!
//! a
//! b
//! c
//!
//! ab
//! ac
//!
//! a
//! a
//! a
//! a
//!
//! b
//! ```
//!
//! This list represents answers from five groups:
//!
//! * The first group contains one person who answered "yes" to _`3`_ questions: `a`, `b`, and `c`.
//! * The second group contains three people; combined, they answered "yes" to _`3`_ questions: `a`, `b`, and `c`.
//! * The third group contains two people; combined, they answered "yes" to _`3`_ questions: `a`, `b`, and `c`.
//! * The fourth group contains four people; combined, they answered "yes" to only _`1`_ question, `a`.
//! * The last group contains one person who answered "yes" to only _`1`_ question, `b`.
//!
//! In this example, the sum of these counts is `3 + 3 + 3 + 1 + 1` = _`11`_.
//!
//! For each group, count the number of questions to which anyone answered "yes". _What is the sum of those counts?_
//!
//! ## Part Two
//!
//!
//!

use std::fmt::Debug;

pub use anyhow::{Context, Result};
use shrinkwraprs::Shrinkwrap;

pub mod initial;
pub use crate::initial::Day06Initial;

#[derive(Debug, Shrinkwrap, PartialEq)]
pub struct Day06Entry(usize);

type Day06SolutionPart1 = i64;
type Day06SolutionPart2 = i64;

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

pub fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = Day06Entry> + 'a {
    input
        .lines()
        .map(str::trim)
        .map(|line| Day06Entry(line.trim().parse().expect("Invalid entry")))
}

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<'a, SolutionPart1 = Day06SolutionPart1, SolutionPart2 = Day06SolutionPart2>
                + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        unimplemented!()
        // vec![
        //     Box::new(Day06Initial::new(PUZZLE_INPUT)),
        // ]
    }
}

#[cfg(test)]
mod tests {
    use std::env;

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

        let parsed: Vec<Day06Entry> = parse_input(PUZZLE_INPUT).collect();
        assert_eq!(parsed.len(), 0);
        assert_eq!(
            &parsed[0..5],
            &[
                //
                Day06Entry(0),
            ]
        );
    }
}
