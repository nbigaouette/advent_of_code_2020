use crate::*;

#[derive(Debug)]
pub struct Day02Initial<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day02Initial<'a> {
    type SolutionPart1 = Day02SolutionPart1;
    type SolutionPart2 = Day02SolutionPart2;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &'a str) -> Day02Initial<'_> {
        Day02Initial { input }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        parse_input(self.input)
            .filter(|entry| {
                let byte_count = entry
                    .password
                    .iter()
                    .filter(|byte| **byte == entry.char)
                    .count() as u8;
                (entry.char_count.start() <= &byte_count) && (&byte_count <= entry.char_count.end())
            })
            .count()
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

                let expected = 645;
                let to_check = Day02Initial::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day02Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 2;
                let input = "1-3 a: abcde
                                1-3 b: cdefg
                                2-9 c: ccccccccc";
                let to_check = Day02Initial::new(input).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day02Initial;
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

                unimplemented!();

                let expected = 0;
                let to_check = Day02Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day02Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                unimplemented!();

                let expected = 0;
                let input = "";
                let to_check = Day02Initial::new(input).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day02Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }
}
