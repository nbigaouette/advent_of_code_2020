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
        parse_input(self.input).filter(is_valid_part1).count()
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        parse_input(self.input).filter(is_valid_part2).count()
    }
}

fn is_valid_part1(entry: &Day02Entry) -> bool {
    let byte_count = entry
        .password
        .iter()
        .filter(|byte| **byte == entry.char)
        .count() as u8;
    (entry.char_count.start() <= &byte_count) && (&byte_count <= entry.char_count.end())
}

fn is_valid_part2(entry: &Day02Entry) -> bool {
    let i0 = (*entry.char_count.start() as usize) - 1;
    let i1 = (*entry.char_count.end() as usize) - 1;
    let first_char: bool = entry.password[i0] == entry.char;
    let second_char: bool = entry.password[i1] == entry.char;

    first_char ^ second_char // XOR
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

                let expected = 737;
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

                let expected = 1;
                let input = "1-3 a: abcde
                                1-3 b: cdefg
                                2-9 c: ccccccccc";
                let to_check = Day02Initial::new(input).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod extra {
            use crate::{initial::is_valid_part2, parse_input, tests::init_logger};

            #[test]
            fn ex01_line_1_is_valid() {
                init_logger();

                let expected = true;
                let input = "1-3 a: abcde";
                let parsed = parse_input(input).next().unwrap();
                let to_check = is_valid_part2(&parsed);
                assert_eq!(to_check, expected);
            }

            #[test]
            fn ex01_line_2_is_invalid() {
                init_logger();

                let expected = false;
                let input = "1-3 b: cdefg";
                let parsed = parse_input(input).next().unwrap();
                let to_check = is_valid_part2(&parsed);
                assert_eq!(to_check, expected);
            }

            #[test]
            fn ex01_line_3_is_invalid() {
                init_logger();

                let expected = false;
                let input = "2-9 c: ccccccccc";
                let parsed = parse_input(input).next().unwrap();
                let to_check = is_valid_part2(&parsed);
                assert_eq!(to_check, expected);
            }
        }
    }
}
