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

        accumulator
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        let mut instructions: Vec<Day08Entry> = parse_input(self.input).collect();

        let instructions_to_replace: Vec<(usize, Instruction)> = instructions
            .iter()
            .enumerate()
            .filter_map(|(idx, instruction)| match instruction.0 {
                Instruction::Jump(argument) => Some((idx, Instruction::NoOp(argument))),
                Instruction::NoOp(argument) => Some((idx, Instruction::Jump(argument))),
                _ => None,
            })
            .collect();

        let max_steps = instructions.len();

        for (idx, mut to_replace) in instructions_to_replace {
            log::info!("Replacing index {} with {:?}", idx, to_replace);

            std::mem::swap(&mut to_replace, &mut instructions[idx].0);

            match part2_run_instructions(&instructions, max_steps) {
                None => {
                    // Reset and continue
                    std::mem::swap(&mut to_replace, &mut instructions[idx].0);
                }
                Some(answer) => {
                    // We found a solution!
                    return answer;
                }
            }
        }

        unreachable!()
    }
}

fn part2_run_instructions(instructions: &[Day08Entry], max_instructions: usize) -> Option<isize> {
    let mut instruction_idx: isize = 0;
    let mut accumulator = 0;
    let mut instruction_count = 0;

    loop {
        if instruction_count == max_instructions {
            break None;
        } else {
            instruction_count += 1;

            if instruction_idx == instructions.len() as isize {
                break Some(accumulator);
            } else {
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
        }
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

                let expected = 1643;
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

                let expected = 8;
                let input = "nop +0
                                acc +1
                                jmp +4
                                acc +3
                                jmp -3
                                acc -99
                                acc +1
                                jmp -4
                                acc +6";
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
