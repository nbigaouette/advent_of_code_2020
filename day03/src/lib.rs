//! # Day 03: Toboggan Trajectory
//!
//! [GitHub source](https://github.com/nbigaouette/advent_of_code_2019/tree/master/day03)
//!
//! [Benchmarking report](../../../day03/target/criterion/report/index.html)
//!
//! ## Part One
//!
//! With the toboggan login problems resolved, you set off toward the airport. While travel by toboggan might be easy,
//! it's certainly not safe: there's very minimal steering and the area is covered in trees. You'll need to see which
//! angles will take you near the fewest trees.
//!
//! Due to the local geology, trees in this area only grow on exact integer coordinates in a grid. You make a map (your
//! puzzle input) of the open squares (`.`) and trees (`#`) you can see. For example:
//!
//! ```text
//! ..##.......
//! #...#...#..
//! .#....#..#.
//! ..#.#...#.#
//! .#...##..#.
//! ..#.##.....
//! .#.#.#....#
//! .#........#
//! #.##...#...
//! #...##....#
//! .#..#...#.#
//! ```
//!
//! These aren't the only trees, though; due to something you read about once involving arboreal genetics and biome
//! stability, the same pattern repeats to the right many times:
//!
//! ```text
//! ..##.........##.........##.........##.........##.........##.......  --->
//! #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//! .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//! ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//! .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//! ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
//! .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//! .#........#.#........#.#........#.#........#.#........#.#........#
//! #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
//! #...##....##...##....##...##....##...##....##...##....##...##....#
//! .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//! ```
//!
//! You start on the open square (`.`) in the top-left corner and need to reach the bottom (below the bottom-most
//! row on your map).
//!
//! The toboggan can only follow a few specific slopes (you opted for a cheaper model that prefers rational numbers);
//! start by _counting all the trees_ you would encounter for the slope _right 3, down 1_:
//!
//! From your starting position at the top-left, check the position that is right 3 and down 1. Then, check the
//! position that is right 3 and down 1 from there, and so on until you go past the bottom of the map.
//!
//! The locations you'd check in the above example are marked here with `O` where there was an open square and
//! `X` where there was a tree:
//!
//! ```text
//! ..##.........##.........##.........##.........##.........##.......  --->
//! #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//! .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//! ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//! .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//! ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
//! .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//! .#........#.#........X.#........#.#........#.#........#.#........#
//! #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
//! #...##....##...##....##...#X....##...##....##...##....##...##....#
//! .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//! ```
//!
//! In this example, traversing the map using this slope would cause you to encounter `7` trees.
//!
//! Starting at the top-left corner of your map and following a slope of right 3 and down 1, _how many trees
//! would you encounter?_
//!
//! ## Part Two
//!
//!
//!

use std::fmt::Debug;

pub use anyhow::{Context, Result};

pub mod initial;
pub use crate::initial::Day03Initial;

#[derive(PartialEq, Debug)]
pub enum ForestSquare {
    Open,
    Tree,
}

#[derive(PartialEq, Debug)]
pub struct ForestLine<'a>(&'a str);

impl<'a> ForestLine<'a> {
    fn iter(&'a self) -> impl Iterator<Item = ForestSquare> + 'a {
        self.0.bytes().cycle().map(|b| match b {
            b'.' => ForestSquare::Open,
            b'#' => ForestSquare::Tree,
            _ => unreachable!(),
        })
    }
}

type Day03SolutionPart1 = u64;
type Day03SolutionPart2 = u64;

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

pub fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = ForestLine> + 'a {
    input.trim().lines().map(str::trim).map(ForestLine)
}

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<'a, SolutionPart1 = Day03SolutionPart1, SolutionPart2 = Day03SolutionPart2>
                + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        unimplemented!()
        // vec![
        //     Box::new(Day03Initial::new(PUZZLE_INPUT)),
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
    fn parse_square() {
        init_logger();

        assert_eq!(ForestLine(".").iter().next().unwrap(), ForestSquare::Open);
        assert_eq!(ForestLine("#").iter().next().unwrap(), ForestSquare::Tree);
    }

    #[test]
    fn parse() {
        init_logger();

        let input = "
            ..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#
        ";

        let parsed: Vec<_> = parse_input(input).collect();

        assert_eq!(
            parsed,
            vec![
                ForestLine("..##......."),
                ForestLine("#...#...#.."),
                ForestLine(".#....#..#."),
                ForestLine("..#.#...#.#"),
                ForestLine(".#...##..#."),
                ForestLine("..#.##....."),
                ForestLine(".#.#.#....#"),
                ForestLine(".#........#"),
                ForestLine("#.##...#..."),
                ForestLine("#...##....#"),
                ForestLine(".#..#...#.#"),
            ]
        );

        assert_eq!(
            parsed[0].iter().take(11).collect::<Vec<_>>(),
            vec![
                ForestSquare::Open,
                ForestSquare::Open,
                ForestSquare::Tree,
                ForestSquare::Tree,
                ForestSquare::Open,
                ForestSquare::Open,
                ForestSquare::Open,
                ForestSquare::Open,
                ForestSquare::Open,
                ForestSquare::Open,
                ForestSquare::Open,
            ]
        );

        assert_eq!(
            parsed[1].iter().take(11).collect::<Vec<_>>(),
            vec![
                ForestSquare::Tree,
                ForestSquare::Open,
                ForestSquare::Open,
                ForestSquare::Open,
                ForestSquare::Tree,
                ForestSquare::Open,
                ForestSquare::Open,
                ForestSquare::Open,
                ForestSquare::Tree,
                ForestSquare::Open,
                ForestSquare::Open,
            ]
        );
    }
}
