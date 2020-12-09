//! # Day 07: Handy Haversacks
//!
//! [GitHub source](https://github.com/nbigaouette/advent_of_code_2019/tree/master/day07)
//!
//! [Benchmarking report](../../../day07/target/criterion/report/index.html)
//!
//! ## Part One
//!
//! You land at the regional airport in time for your next flight. In fact, it looks like you'll even have time to
//! grab some food: all flights are currently delayed due to _issues in luggage processing_.
//!
//! Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags and their
//! contents; bags must be color-coded and must contain specific quantities of other color-coded bags. Apparently,
//! nobody responsible for these regulations considered how long they would take to enforce!
//!
//! For example, consider the following rules:
//!
//! ```text
//! light red bags contain 1 bright white bag, 2 muted yellow bags.
//! dark orange bags contain 3 bright white bags, 4 muted yellow bags.
//! bright white bags contain 1 shiny gold bag.
//! muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
//! shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
//! dark olive bags contain 3 faded blue bags, 4 dotted black bags.
//! vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
//! faded blue bags contain no other bags.
//! dotted black bags contain no other bags.
//! ```
//!
//! These rules specify the required contents for 9 bag types. In this example, every `faded blue` bag is empty,
//! every `vibrant plum` bag contains 11 bags (5 `faded blue` and 6 `dotted black`), and so on.
//!
//! You have a `_shiny gold_` bag. If you wanted to carry it in at least one other bag, how many different bag
//! colors would be valid for the outermost bag? (In other words: how many colors can, eventually, contain at
//! least one `shiny gold` bag?)
//!
//! In the above rules, the following options would be available to you:
//!
//! * A `bright white` bag, which can hold your `shiny gold` bag directly.
//! * A `muted yellow` bag, which can hold your `shiny gold` bag directly, plus some other bags.
//! * A `dark orange` bag, which can hold `bright white` and `muted yellow` bags, either of which could then hold
//!   your `shiny gold` bag.
//! * A `light red` bag, which can hold `bright white` and `muted yellow` bags, either of which could then hold your `shiny gold` bag.
//!
//! So, in this example, the number of bag colors that can eventually contain at least one `shiny gold` bag is `_4_`.
//!
//! _How many bag colors can eventually contain at least one `shiny gold` bag?_ (The list of rules is quite long; make
//! sure you get all of it.)
//!
//! ## Part Two
//!
//! It's getting pretty expensive to fly these days - not because of ticket prices, but because of the ridiculous number
//! of bags you need to buy!
//!
//! Consider again your `shiny gold` bag and the rules from the above example:
//!
//! * `faded blue` bags contain `0` other bags.
//! * `dotted black` bags contain `0` other bags.
//! * `vibrant plum` bags contain `11` other bags: 5 `faded blue` bags and 6 `dotted black` bags.
//! * `dark olive` bags contain `7` other bags: 3 `faded blue` bags and 4 `dotted black` bags.
//!
//! So, a single `shiny gold` bag must contain 1 `dark olive` bag (and the 7 bags within it) plus 2 `vibrant plum` bags
//! (and the 11 bags within _each_ of those): `1 + 1*7 + 2 + 2*11` = `_32_` bags!
//!
//! Of course, the actual rules have a small chance of going several levels deeper than this example; be sure to count
//! all of the bags, even if the nesting becomes topologically impractical!
//!
//! Here's another example:
//!
//! ```text
//! shiny gold bags contain 2 dark red bags.
//! dark red bags contain 2 dark orange bags.
//! dark orange bags contain 2 dark yellow bags.
//! dark yellow bags contain 2 dark green bags.
//! dark green bags contain 2 dark blue bags.
//! dark blue bags contain 2 dark violet bags.
//! dark violet bags contain no other bags.
//! ```
//!
//! In this example, a single `shiny gold` bag must contain `_126_` other bags.
//!
//! _How many individual bags are required inside your single `shiny gold` bag?_
//!

use std::{collections::HashMap, fmt::Debug};

pub use anyhow::{Context, Result};
use shrinkwraprs::Shrinkwrap;

pub mod initial;
pub use crate::initial::Day07Initial;

#[derive(Debug, Shrinkwrap, PartialEq)]
pub struct Day07Entry(usize);

type Day07SolutionPart1 = usize;
type Day07SolutionPart2 = usize;

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

pub fn parse_input<'a>(input: &'a str) -> HashMap<&'a str, Vec<(usize, &'a str)>> {
    input
        .trim()
        .lines()
        .map(str::trim)
        .map(|line| {
            let mut entry_iter = line.split(" bags contain ");
            let bag_containing = entry_iter.next().expect("a containing bag");
            let contained_bags_sentence = entry_iter.next().expect("contained bags");

            let contained_bags: Vec<(usize, &'a str)> = match contained_bags_sentence {
                "no other bags." => Vec::new(),
                contained_bags_sentence => contained_bags_sentence
                    .split(", ")
                    .map(|entry| {
                        let mut entry_iter = entry.split(' ');
                        let count_str = entry_iter.next().expect("a count string");

                        let bag_color = entry
                            .trim_start_matches(count_str)
                            .trim_end_matches('.')
                            .trim_end_matches("bags")
                            .trim_end_matches("bag")
                            .trim();

                        let count: usize = count_str.parse().expect("a count number");

                        (count, bag_color)
                    })
                    .collect(),
            };

            (bag_containing, contained_bags)
        })
        .collect()
}

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<'a, SolutionPart1 = Day07SolutionPart1, SolutionPart2 = Day07SolutionPart2>
                + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        vec![Box::new(Day07Initial::new(PUZZLE_INPUT))]
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

        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let expected = [("light red", vec![(1, "bright white"), (2, "muted yellow")])]
            .iter()
            .cloned()
            .collect::<HashMap<&str, Vec<(usize, &str)>>>();

        let rules = parse_input(input);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules, expected);
    }
    #[test]
    fn parse() {
        init_logger();

        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.";
        let expected = [
            ("light red", vec![(1, "bright white"), (2, "muted yellow")]),
            (
                "dark orange",
                vec![(3, "bright white"), (4, "muted yellow")],
            ),
            ("bright white", vec![(1, "shiny gold")]),
            ("muted yellow", vec![(2, "shiny gold"), (9, "faded blue")]),
            ("shiny gold", vec![(1, "dark olive"), (2, "vibrant plum")]),
            ("dark olive", vec![(3, "faded blue"), (4, "dotted black")]),
            ("vibrant plum", vec![(5, "faded blue"), (6, "dotted black")]),
            ("faded blue", vec![]),
            ("dotted black", vec![]),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<&str, Vec<(usize, &str)>>>();

        let rules = parse_input(input);
        assert_eq!(rules.len(), 9);
        assert_eq!(rules, expected);
    }
}
