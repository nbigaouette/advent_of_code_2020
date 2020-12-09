use std::collections::HashSet;

use crate::*;

#[derive(Debug)]
pub struct Day09Initial<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day09Initial<'a> {
    type SolutionPart1 = Day09SolutionPart1;
    type SolutionPart2 = Day09SolutionPart2;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &'a str) -> Day09Initial<'_> {
        Day09Initial { input }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        let parsed: Vec<Day09Entry> = parse_input(self.input).collect();
        solution_part1(&parsed, 25)
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        let parsed: Vec<Day09Entry> = parse_input(self.input).collect();

        // Solution to part 1
        let target: isize = 217430975;

        solution_part2(&parsed, target)
    }
}

#[derive(Debug)]
struct DiffMap<'a> {
    map: HashSet<isize>,
    window: &'a [isize],
}

impl<'a> DiffMap<'a> {
    fn new(window: &'a [isize]) -> DiffMap<'a> {
        let map: HashSet<isize> = HashSet::new();

        DiffMap { map, window }
    }

    fn update(&mut self, new_window: &'a [isize]) {
        assert_eq!(self.window.len(), new_window.len());

        self.window = new_window;

        // Clear the hashset (keeps allocated memory)
        self.map.clear();

        for (i, w0) in new_window.iter().enumerate() {
            self.map.extend(new_window.iter().skip(i).map(|w1| w0 + w1));
        }
    }

    fn contains(&self, value: isize) -> bool {
        self.map.contains(&value)
    }
}

fn solution_part1(entries: &[Day09Entry], window: usize) -> isize {
    let mut map = DiffMap::new(&entries[0..window]);
    let (_found_window, value) = entries
        .windows(window)
        .zip(entries.iter().skip(window))
        .find(|(window_slice, next_value)| {
            map.update(window_slice);
            !map.contains(**next_value)
        })
        .expect("A matching value");

    *value
}

fn solution_part2(entries: &[Day09Entry], target: isize) -> isize {
    for start_idx in 0..entries.len() {
        for end_idx in (start_idx + 2)..entries.len() {
            let slice = &entries[start_idx..end_idx];
            let slice_sum: isize = slice.iter().sum();
            if slice_sum > target {
                // If the sum is too large, we start a new window lower range
                break;
            }
            if slice_sum == target {
                return slice.iter().max().expect("a max") + slice.iter().min().expect("a min");
            }
        }
    }
    unreachable!()
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

                let expected = 217430975;
                let to_check = Day09Initial::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use crate::{initial::solution_part1, parse_input, tests::init_logger};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 127;
                let input = "35
                                20
                                15
                                25
                                47
                                40
                                62
                                55
                                65
                                95
                                102
                                117
                                150
                                182
                                127
                                219
                                299
                                277
                                309
                                576";
                let parsed: Vec<_> = parse_input(input).collect();
                let to_check = solution_part1(&parsed, 5);

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day09Initial;
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

                let expected = 28509180;
                let to_check = Day09Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use crate::{initial::solution_part2, parse_input, tests::init_logger};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 62;
                let input = "35
                                20
                                15
                                25
                                47
                                40
                                62
                                55
                                65
                                95
                                102
                                117
                                150
                                182
                                127
                                219
                                299
                                277
                                309
                                576";
                let parsed: Vec<_> = parse_input(input).collect();
                let to_check = solution_part2(&parsed, 127);

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day09Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }
}
