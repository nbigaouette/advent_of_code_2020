//! # Day 01:
//!
//! [GitHub source](https://github.com/nbigaouette/advent_of_code_2019/tree/master/day01)
//!
//! [Benchmarking report](../../../day01/target/criterion/report/index.html)
//!
//! ## Part One
//!
//! After saving Christmas [five years in a row](/events), you've decided to take a vacation at
//! a nice resort on a tropical island. Surely, Christmas will go on without you.
//!
//! The tropical island has its own currency and is entirely cash-only. The gold coins used
//! there have a little picture of a starfish; the locals just call them _stars_. None of the
//! currency exchanges seem to have heard of them, but somehow, you'll need to find fifty of
//! these coins by the time you arrive so you can pay the deposit on your room.
//!
//! To save your vacation, you need to get all _fifty stars_ by December 25th.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each day in the
//! Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle
//! grants _one star_. Good luck!
//!
//! Before you leave, the Elves in accounting just need you to fix your _expense report_
//! (your puzzle input); apparently, something isn't quite adding up.
//!
//! Specifically, they need you to _find the two entries that sum to `2020`_ and then multiply
//! those two numbers together.
//!
//! For example, suppose your expense report contained the following:
//!
//! ```text
//! 1721
//! 979
//! 366
//! 299
//! 675
//! 1456
//! ```
//!
//! In this list, the two entries that sum to `2020` are `1721` and `299`. Multiplying them
//! together produces `1721 * 299 = 514579`, so the correct answer is `_514579_`.
//!
//! Of course, your expense report is much larger. _Find the two entries that sum to `2020`;
//! what do you get if you multiply them together?_
//!
//! ## Part Two
//!
//!
//!

use std::fmt::Debug;

pub use anyhow::{Context, Result};
use shrinkwraprs::Shrinkwrap;

pub mod initial;
pub use crate::initial::Day01Initial;

#[derive(Debug, Shrinkwrap, PartialEq, Ord, Eq, PartialOrd, Hash)]
pub struct Day01Entry(u64);

type Day01SolutionPart1 = u64;
type Day01SolutionPart2 = u64;

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

pub fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = Day01Entry> + 'a {
    input
        .lines()
        .map(|line| Day01Entry(line.trim().parse().expect("Invalid entry")))
}

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<'a, SolutionPart1 = Day01SolutionPart1, SolutionPart2 = Day01SolutionPart2>
                + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        unimplemented!()
        // vec![
        //     Box::new(Day01Initial::new(PUZZLE_INPUT)),
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

        let parsed: Vec<Day01Entry> = parse_input(
            "1721
            979
            366
            299
            675
            1456",
        )
        .collect();
        assert_eq!(parsed.len(), 6);
        assert_eq!(
            parsed,
            &[
                Day01Entry(1721),
                Day01Entry(979),
                Day01Entry(366),
                Day01Entry(299),
                Day01Entry(675),
                Day01Entry(1456),
            ]
        );
    }
}
