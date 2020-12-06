use crate::*;

#[derive(Debug)]
pub struct Day06Initial<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day06Initial<'a> {
    type SolutionPart1 = Day06SolutionPart1;
    type SolutionPart2 = Day06SolutionPart2;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &'a str) -> Day06Initial<'_> {
        Day06Initial { input }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        parse_input_part1(self.input).map(|entry| entry.len()).sum()
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        parse_input_part2(self.input).map(|entry| entry.len()).sum()
    }
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

                let expected = 6506;
                let to_check = Day06Initial::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day06Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 11;
                let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
                let to_check = Day06Initial::new(input).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day06Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }

    mod part2 {
        mod solution {
            use super::super::super::*;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 3243;
                let to_check = Day06Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day06Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 6;
                let input = "abc

                                a
                                b
                                c

                                ab
                                ac

                                a
                                a
                                a
                                a

                                b";
                let to_check = Day06Initial::new(input).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day06Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }
}
