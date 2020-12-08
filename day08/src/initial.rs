use crate::*;

#[derive(Debug)]
pub struct Day08Initial<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day08Initial<'a> {
    type SolutionPart1 = Day08SolutionPart1;
    type SolutionPart2 = Day08SolutionPart2;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &'a str) -> Day08Initial<'_> {
        Day08Initial { input }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        let instructions: Vec<Day08Entry> = parse_input(self.input).collect();
        let mut instructions_execution_count = vec![0; instructions.len()];

        let mut instruction_idx: isize = 0;
        let mut accumulator = 0;

        while instructions_execution_count[instruction_idx as usize] == 0 {
            instructions_execution_count[instruction_idx as usize] += 1;
            match instructions[instruction_idx as usize].0 {
                Instruction::Accumulator(argument) => {
                    accumulator += argument;
                    instruction_idx += 1;
                }
                Instruction::Jump(argument) => {
                    instruction_idx += argument;
                }
                Instruction::NoOp(_argument) => {
                    instruction_idx += 1;
                }
            }
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

                let expected = 1262;
                let to_check = Day08Initial::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day08Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 5;
                let input = "nop +0
                                acc +1
                                jmp +4
                                acc +3
                                jmp -3
                                acc -99
                                acc +1
                                jmp -4
                                acc +6";
                let to_check = Day08Initial::new(input).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day08Initial;
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
                let to_check = Day08Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day08Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                unimplemented!();

                let expected = 0;
                let input = "";
                let to_check = Day08Initial::new(input).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day08Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }
}
