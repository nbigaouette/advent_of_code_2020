use std::collections::HashSet;

use crate::*;

#[derive(Debug)]
pub struct Day01Initial<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day01Initial<'a> {
    type SolutionPart1 = Day01SolutionPart1;
    type SolutionPart2 = Day01SolutionPart2;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &'a str) -> Day01Initial<'_> {
        Day01Initial { input }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        const SUM_TARGET: u64 = 2020;

        // Double the memory
        let entries: Vec<_> = parse_input(self.input).collect();
        let entries_set: HashSet<_> = entries.iter().collect();

        let left = entries
            .iter()
            .map(|e| Day01Entry(SUM_TARGET - e.0))
            .find(|diff| entries_set.contains(diff))
            .expect("At least one find");

        let right = Day01Entry(SUM_TARGET - left.0);

        left.0 * right.0
    }

    // fn solution_part2(&self) -> Self::SolutionPart2 {
    // }
}

#[cfg(test)]
mod tests {
    mod part1 {
        mod solution {
            use super::super::super::*;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 545379;
                let to_check = Day01Initial::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day01Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 514579;
                let input = "1721
                    979
                    366
                    299
                    675
                    1456";
                let to_check = Day01Initial::new(input).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day01Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }

    /*
    mod part2 {
        mod solution {
            use super::super::super::*;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                unimplemented!();

                let expected = 0;
                let to_check = Day01Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day01Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                unimplemented!();

                let expected = 0;
                let input = "";
                let to_check = Day01Initial::new(input).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day01Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }
    */
}
