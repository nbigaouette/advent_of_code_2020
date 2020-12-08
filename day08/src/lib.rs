//! # Day 08: Handheld Halting
//!
//! [GitHub source](https://github.com/nbigaouette/advent_of_code_2019/tree/master/day08)
//!
//! [Benchmarking report](../../../day08/target/criterion/report/index.html)
//!
//! ## Part One
//!
//! Your flight to the major airline hub reaches cruising altitude without incident. While you
//! consider checking the in-flight menu for one of those drinks that come with a little
//! umbrella, you are interrupted by the kid sitting next to you.
//!
//! Their [handheld game console](https://en.wikipedia.org/wiki/Handheld_game_console) won't
//! turn on! They ask if you can take a look.
//!
//! You narrow the problem down to a strange _infinite loop_ in the boot code (your puzzle
//! input) of the device. You should be able to fix it, but first you need to be able to run
//! the code in isolation.
//!
//! The boot code is represented as a text file with one _instruction_ per line of text. Each
//! instruction consists of an _operation_ (`acc`, `jmp`, or `nop`) and an _argument_ (a signed
//! number like `+4` or `-20`).
//!
//! * `acc` increases or decreases a single global value called the _accumulator_ by the value
//!   given in the argument. For example, `acc +7` would increase the accumulator by 7. The
//!   accumulator starts at `0`. After an `acc` instruction, the instruction immediately below
//!   it is executed next.
//! * `jmp` _jumps_ to a new instruction relative to itself. The next instruction to execute
//!   is found using the argument as an _offset_ from the `jmp` instruction; for example,
//!   `jmp +2` would skip the next instruction, `jmp +1` would continue to the instruction
//!   immediately below it, and `jmp -20` would cause the instruction 20 lines above to be
//!   executed next.
//! * `nop` stands for _No OPeration_ - it does nothing. The instruction immediately below
//!   it is executed next.
//!
//! For example, consider the following program:
//!
//! ```text
//! nop +0
//! acc +1
//! jmp +4
//! acc +3
//! jmp -3
//! acc -99
//! acc +1
//! jmp -4
//! acc +6
//! ```
//!
//!
//! These instructions are visited in this order:
//!
//! ```text
//! nop +0  | 1
//! acc +1  | 2, 8(!)
//! jmp +4  | 3
//! acc +3  | 6
//! jmp -3  | 7
//! acc -99 |
//! acc +1  | 4
//! jmp -4  | 5
//! acc +6  |
//! ```
//!
//! First, the `nop +0` does nothing. Then, the accumulator is increased from 0 to 1
//! (`acc +1`) and `jmp +4` sets the next instruction to the other `acc +1` near the bottom.
//! After it increases the accumulator from 1 to 2, `jmp -4` executes, setting the next
//! instruction to the only `acc +3`. It sets the accumulator to 5, and `jmp -3` causes
//! the program to continue back at the first `acc +1`.
//!
//! This is an _infinite loop_: with this sequence of jumps, the program will run forever.
//! The moment the program tries to run any instruction a second time, you know it will
//! never terminate.
//!
//! Immediately _before_ the program would run an instruction a second time, the value in
//! the accumulator is _`5`_.
//!
//! Run your copy of the boot code. Immediately before any instruction is executed a second
//! time, _what value is in the accumulator?_
//!
//! ## Part Two
//!
//!
//!

use std::fmt::Debug;

pub use anyhow::{Context, Result};
use parse_display::{Display, FromStr};
use shrinkwraprs::Shrinkwrap;

pub mod initial;
pub use crate::initial::Day08Initial;

#[derive(Debug, Clone, Shrinkwrap, PartialEq)]
pub struct Day08Entry(Instruction);

type Day08SolutionPart1 = isize;
type Day08SolutionPart2 = isize;

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

#[derive(Display, Clone, FromStr, PartialEq, Debug)]
pub enum Instruction {
    #[display("acc {0}")]
    Accumulator(isize),
    #[display("jmp {0}")]
    Jump(isize),
    #[display("nop {0}")]
    NoOp(isize),
}

pub fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = Day08Entry> + 'a {
    input
        .trim()
        .lines()
        .map(str::trim)
        .map(|line| Day08Entry(line.trim().parse().expect("Invalid entry")))
}

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<'a, SolutionPart1 = Day08SolutionPart1, SolutionPart2 = Day08SolutionPart2>
                + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        unimplemented!()
        // vec![
        //     Box::new(Day08Initial::new(PUZZLE_INPUT)),
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

        let input = "nop +0
                        acc +1
                        jmp +4
                        acc +3
                        jmp -3
                        acc -99
                        acc +1
                        jmp -4
                        acc +6";
        let expected = &[
            Day08Entry(Instruction::NoOp(0)),
            Day08Entry(Instruction::Accumulator(1)),
            Day08Entry(Instruction::Jump(4)),
            Day08Entry(Instruction::Accumulator(3)),
            Day08Entry(Instruction::Jump(-3)),
            Day08Entry(Instruction::Accumulator(-99)),
            Day08Entry(Instruction::Accumulator(1)),
            Day08Entry(Instruction::Jump(-4)),
            Day08Entry(Instruction::Accumulator(6)),
        ];

        let parsed: Vec<Day08Entry> = parse_input(input).collect();
        assert_eq!(parsed.len(), 9);
        assert_eq!(parsed, expected);
    }
}
