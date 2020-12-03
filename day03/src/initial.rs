use crate::*;

#[derive(Debug)]
pub struct Day03Initial<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day03Initial<'a> {
    type SolutionPart1 = Day03SolutionPart1;
    type SolutionPart2 = Day03SolutionPart2;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &'a str) -> Day03Initial<'_> {
        Day03Initial { input }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        count_trees_on_slope(&Slope { down: 1, right: 3 }, self.input)
    }

struct Slope {
    down: usize,
    right: usize,
}

    // fn solution_part2(&self) -> Self::SolutionPart2 {
    // }
fn count_trees_on_slope(slope: &Slope, input: &str) -> usize {
    parse_input(input)
        .step_by(slope.down)
        .skip(1) // We don't start on first line
        .enumerate()
        .map(|(down_count, line)| {
            line.iter()
                .skip(down_count * slope.right + 1)
                .nth(slope.right - 1)
                .expect("Cycle iterator should always provide a value")
        })
        .filter(|square| *square == ForestSquare::Tree)
        .count()
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

                let expected = 230;
                let to_check = Day03Initial::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day03Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 7;
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
                let to_check = Day03Initial::new(input).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day03Initial;
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
                let to_check = Day03Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day03Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                unimplemented!();

                let expected = 0;
                let input = "";
                let to_check = Day03Initial::new(input).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day03Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }
}
