//! # Day 05: Binary Boarding
//!
//! [GitHub source](https://github.com/nbigaouette/advent_of_code_2019/tree/master/day05)
//!
//! [Benchmarking report](../../../day05/target/criterion/report/index.html)
//!
//! ## Part One
//!
//! You board your plane only to discover a new problem: you dropped your boarding pass! You aren't sure
//! which seat is yours, and all of the flight attendants are busy with the flood of people that suddenly
//! made it through passport control.
//!
//! You write a quick program to use your phone's camera to scan all of the nearby boarding passes (your
//! puzzle input); perhaps you can find your seat through process of elimination.
//!
//! Instead of [zones or groups](https://www.youtube.com/watch?v=oAHbLRjF0vo), this airline uses _binary
//! space partitioning_ to seat people. A seat might be specified like `FBFBBFFRLR`, where `F` means
//! "front", `B` means "back", `L` means "left", and `R` means "right".
//!
//! The first 7 characters will either be `F` or `B`; these specify exactly one of the _128 rows_ on the
//! plane (numbered `0` through `127`). Each letter tells you which half of a region the given seat is in.
//! Start with the whole list of rows; the first letter indicates whether the seat is in the _front_ (`0`
//! through `63`) or the _back_ (`64` through `127`). The next letter indicates which half of that region
//! the seat is in, and so on until you're left with exactly one row.
//!
//! For example, consider just the first seven characters of `FBFBBFFRLR`:
//!
//! * Start by considering the whole range, rows `0` through `127`.
//! * `F` means to take the _lower half_, keeping rows `0` through `63`.
//! * `B` means to take the _upper half_, keeping rows `32` through `63`.
//! * `F` means to take the _lower half_, keeping rows `32` through `47`.
//! * `B` means to take the _upper half_, keeping rows `40` through `47`.
//! * `B` keeps rows `44` through `47`.
//! * `F` keeps rows `44` through `45`.
//! * The final `F` keeps the lower of the two, _row `44`_.
//!
//! The last three characters will be either `L` or `R`; these specify exactly one of the _8 columns_
//! of seats on the plane (numbered `0` through `7`). The same process as above proceeds again, this
//! time with only three steps. `L` means to keep the _lower half_, while `R` means to keep the _upper half_.
//!
//! For example, consider just the last 3 characters of `FBFBBFFRLR`:
//!
//! * Start by considering the whole range, columns `0` through `7`.
//! * `R` means to take the _upper half_, keeping columns `4` through `7`.
//! * `L` means to take the _lower half_, keeping columns `4` through `5`.
//! * The final `R` keeps the upper of the two, _column `5`_.
//!
//! So, decoding `FBFBBFFRLR` reveals that it is the seat at _row `44`, column `5`_.
//!
//! Every seat also has a unique _seat ID_: multiply the row by 8, then add the column. In this
//! example, the seat has ID `44 * 8 + 5 = _357_`.
//!
//! Here are some other boarding passes:
//!
//! * `BFFFBBFRRR`: row `70`, column `7`, seat ID `567`.
//! * `FFFBBBFRRR`: row `14`, column `7`, seat ID `119`.
//! * `BBFFBBFRLL`: row `102`, column `4`, seat ID `820`.
//!
//! As a sanity check, look through your list of boarding passes. _What is the highest seat ID on a
//! boarding pass?_
//!
//! ## Part Two
//!
//! _Ding!_ The "fasten seat belt" signs have turned on. Time to find your seat.
//!
//! It's a completely full flight, so your seat should be the only missing boarding pass in your list.
//! However, there's a catch: some of the seats at the very front and back of the plane don't exist on
//! this aircraft, so they'll be missing from your list as well.
//!
//! Your seat wasn't at the very front or back, though; the seats with IDs +1 and -1 from yours will
//! be in your list.
//!
//! _What is the ID of your seat?_
//!

use std::fmt::Debug;

pub use anyhow::{Context, Result};

pub mod initial;
pub use crate::initial::Day05Initial;

#[derive(Debug, PartialEq)]
pub struct Day05Entry<'a>(Direction<'a>);

type Day05SolutionPart1 = usize;
type Day05SolutionPart2 = usize;

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

#[derive(Debug, PartialEq)]
pub enum DirectionRow {
    Front,
    Back,
}

#[derive(Debug, PartialEq)]
pub enum DirectionColumn {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub struct DirectionRowIterator<'a> {
    directions: &'a [u8],
}

#[derive(Debug, PartialEq)]
pub struct DirectionColumnIterator<'a> {
    directions: &'a [u8],
}

impl<'a> DirectionRowIterator<'a> {
    fn iter(&'a self) -> impl Iterator<Item = DirectionRow> + 'a {
        self.directions.iter().map(|b| match b {
            b'F' => DirectionRow::Front,
            b'B' => DirectionRow::Back,
            _ => unreachable!(),
        })
    }
}

impl<'a> DirectionColumnIterator<'a> {
    fn iter(&'a self) -> impl Iterator<Item = DirectionColumn> + 'a {
        self.directions.iter().map(|b| match b {
            b'L' => DirectionColumn::Left,
            b'R' => DirectionColumn::Right,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Direction<'a> {
    line: &'a str,
    row_iter: DirectionRowIterator<'a>,
    col_iter: DirectionColumnIterator<'a>,
}

pub fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = Day05Entry> + 'a {
    input.trim().lines().map(str::trim).map(|line| {
        let (split_left, split_right) = line.as_bytes().split_at(7);
        let row_iter = DirectionRowIterator {
            directions: split_left,
        };
        let col_iter = DirectionColumnIterator {
            directions: split_right,
        };
        Day05Entry(Direction {
            line,
            row_iter,
            col_iter,
        })
    })
}

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<'a, SolutionPart1 = Day05SolutionPart1, SolutionPart2 = Day05SolutionPart2>
                + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        unimplemented!()
        // vec![
        //     Box::new(Day05Initial::new(PUZZLE_INPUT)),
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
    fn parse_single() {
        init_logger();

        let parsed: Vec<Day05Entry> = parse_input("FBFBBFFRLR").collect();
        assert_eq!(parsed.len(), 1);
        assert_eq!(
            parsed,
            &[Day05Entry(Direction {
                line: "FBFBBFFRLR",
                row_iter: DirectionRowIterator {
                    directions: b"FBFBBFF"
                },
                col_iter: DirectionColumnIterator { directions: b"RLR" },
            }),]
        );

        assert_eq!(
            parsed[0].0.row_iter.iter().collect::<Vec<DirectionRow>>(),
            vec![
                DirectionRow::Front,
                DirectionRow::Back,
                DirectionRow::Front,
                DirectionRow::Back,
                DirectionRow::Back,
                DirectionRow::Front,
                DirectionRow::Front
            ]
        );

        assert_eq!(
            parsed[0]
                .0
                .col_iter
                .iter()
                .collect::<Vec<DirectionColumn>>(),
            vec![
                DirectionColumn::Right,
                DirectionColumn::Left,
                DirectionColumn::Right,
            ]
        );
    }
}
