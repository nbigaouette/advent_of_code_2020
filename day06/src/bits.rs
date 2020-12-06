use crate::*;

#[derive(Debug)]
pub struct Day06Bits<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day06Bits<'a> {
    type SolutionPart1 = Day06SolutionPart1;
    type SolutionPart2 = Day06SolutionPart2;

    fn description(&self) -> &'static str {
        "Bits manipulation"
    }

    fn new(input: &'a str) -> Day06Bits<'_> {
        Day06Bits { input }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        let result: u32 = self
            .input
            .split("\n\n")
            .map(|group| {
                group
                    .bytes()
                    .filter(|&c| c != b'\n')
                    .fold(0_u32, |acc, choice| acc | 1 << (choice - b'a'))
                    .count_ones()
            })
            .sum();
        result as usize
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        self.input
            .trim()
            .split("\n\n")
            .map(|block| {
                block
                    .split('\n')
                    .map(str::trim)
                    .map(|line| {
                        line.bytes()
                            .fold(0_u32, |acc, choice| acc | 1 << (choice - b'a'))
                    })
                    .fold(std::u32::MAX, |everyone, one| everyone & one)
                    .count_ones()
            })
            .sum::<u32>() as usize
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
                let to_check = Day06Bits::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day06Bits;
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
                let to_check = Day06Bits::new(input).solution_part1();

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
                let to_check = Day06Bits::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day06Bits;
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
                let to_check = Day06Bits::new(input).solution_part2();

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
