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

        // We search "left*right" such that "left + right == 2020"

        // Store the whole input
        let entries: Vec<_> = parse_input(self.input).collect();

        let (left, right) = solution_part1(SUM_TARGET, &entries).expect("At least one pair");

        left.0 * right.0
    }

    // fn solution_part2(&self) -> Self::SolutionPart2 {}
}

fn solution_part1(sum_target: u64, entries: &[Day01Entry]) -> Option<(Day01Entry, Day01Entry)> {
    // Copy into a hashset to test presence
    let entries_set: HashSet<_> = entries.iter().collect();

    // Find the presence in the hashset of the difference between the target number
    // and the elements.
    entries
        .iter()
        .map(|e| Day01Entry(sum_target - e.0))
        .find(|diff| entries_set.contains(diff))
        .map(|left| {
            // Now that we found the left element, the right one is the difference with the target
            let right = Day01Entry(sum_target - left.0);

            assert_eq!(left.0 + right.0, sum_target);

            (left, right)
        })
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
