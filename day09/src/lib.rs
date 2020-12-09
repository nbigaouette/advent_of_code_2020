//! # Day 09: Encoding Error
//!
//! [GitHub source](https://github.com/nbigaouette/advent_of_code_2019/tree/master/day09)
//!
//! [Benchmarking report](../../../day09/target/criterion/report/index.html)
//!
//! ## Part One
//!
//! With your neighbor happily enjoying their video game, you turn your attention to an open data
//! port on the little screen in the seat in front of you.
//!
//! Though the port is non-standard, you manage to connect it to your computer through the clever
//! use of several paperclips. Upon connection, the port outputs a series of numbers (your puzzle
//! input).
//!
//! The data appears to be encrypted with the eXchange-Masking Addition System (XMAS) which,
//! conveniently for you, is an old cypher with an important weakness.
//!
//! XMAS starts by transmitting a _preamble_ of 25 numbers. After that, each number you receive
//! should be the sum of any two of the 25 immediately previous numbers. The two numbers will
//! have different values, and there might be more than one such pair.
//!
//! For example, suppose your preamble consists of the numbers `1` through `25` in a random order.
//! To be valid, the next number must be the sum of two of those numbers:
//!
//! * `26` would be a _valid_ next number, as it could be `1` plus `25` (or many other pairs,
//! like `2` and `24`).
//! * `49` would be a _valid_ next number, as it is the sum of `24` and `25`.
//! * `100` would _not_ be valid; no two of the previous 25 numbers sum to `100`.
//! * `50` would also _not_ be valid; although `25` appears in the previous 25 numbers, the two
//! numbers in the pair must be different.
//!
//! Suppose the 26th number is `45`, and the first number (no longer an option, as it is more than
//! 25 numbers ago) was `20`. Now, for the next number to be valid, there needs to be some pair of
//! numbers among `1`\-`19`, `21`\-`25`, or `45` that add up to it:
//!
//! * `26` would still be a _valid_ next number, as `1` and `25` are still within the previous 25
//! numbers.
//! * `65` would _not_ be valid, as no two of the available numbers sum to it.
//! * `64` and `66` would both be _valid_, as they are the result of `19+45` and `21+45`
//! respectively.
//!
//! Here is a larger example which only considers the previous _5_ numbers (and has a preamble
//! of length 5):
//!
//! ```text
//! 35
//! 20
//! 15
//! 25
//! 47
//! 40
//! 62
//! 55
//! 65
//! 95
//! 102
//! 117
//! 150
//! 182
//! 127
//! 219
//! 299
//! 277
//! 309
//! 576
//! ```
//!
//! In this example, after the 5-number preamble, almost every number is the sum of two of the
//! previous 5 numbers; the only number that does not follow this rule is _`127`_.
//!
//! The first step of attacking the weakness in the XMAS data is to find the first number in the
//! list (after the preamble) which is _not_ the sum of two of the 25 numbers before it.
//! _What is the first number that does not have this property?_
//!
//! ## Part Two
//!
//!
//!

use std::fmt::Debug;

pub use anyhow::{Context, Result};
use shrinkwraprs::Shrinkwrap;

pub mod initial;
pub use crate::initial::Day09Initial;

#[derive(Debug, Shrinkwrap, PartialEq)]
pub struct Day09Entry(usize);

type Day09SolutionPart1 = i64;
type Day09SolutionPart2 = i64;

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

pub fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = Day09Entry> + 'a {
    input
        .lines()
        .map(str::trim)
        .map(|line| Day09Entry(line.trim().parse().expect("Invalid entry")))
}

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<'a, SolutionPart1 = Day09SolutionPart1, SolutionPart2 = Day09SolutionPart2>
                + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        unimplemented!()
        // vec![
        //     Box::new(Day09Initial::new(PUZZLE_INPUT)),
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

        let parsed: Vec<Day09Entry> = parse_input(PUZZLE_INPUT).collect();
        assert_eq!(parsed.len(), 0);
        assert_eq!(
            &parsed[0..5],
            &[
                //
                Day09Entry(0),
            ]
        );
    }
}
