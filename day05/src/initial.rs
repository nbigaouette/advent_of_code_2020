use std::ops::RangeInclusive;

use crate::*;

#[derive(Debug)]
pub struct Day05Initial<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day05Initial<'a> {
    type SolutionPart1 = Day05SolutionPart1;
    type SolutionPart2 = Day05SolutionPart2;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &'a str) -> Day05Initial<'_> {
        Day05Initial { input }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        parse_input(self.input)
            .map(|direction| find_seat(&direction).seat_id())
            .max()
            .expect("Maximum seat id")
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        let mut seat_ids: Vec<_> = parse_input(self.input)
            .map(|direction| find_seat(&direction).seat_id())
            .collect();
        seat_ids.sort_unstable();

        let non_consecutive_pair = seat_ids
            .windows(2)
            .find(|pair| pair[0] + 1 != pair[1])
            .expect("Non-consecutive pair shouuld be present");

        non_consecutive_pair[0] + 1
    }
}

#[derive(Debug, PartialEq)]
pub struct Seat {
    row: u8,
    column: u8,
}

impl Seat {
    fn seat_id(&self) -> usize {
        (self.row as usize) * 8 + (self.column as usize)
    }
}

fn find_seat<'a>(direction: &Day05Entry<'a>) -> Seat {
    let direction = &direction.0;

    /*
            0             32  40 47    63                           127
            |--------------------------------------------------------|
        F   |---------------------------|
        B                  |------------|
        F                  |------|
        B                      |--|
    */

    let row = direction
        .row_iter
        .iter()
        .fold(
            RangeInclusive::new(0, 127),
            |range, direction| match direction {
                DirectionRow::Front => RangeInclusive::new(
                    // Keep the same start
                    *range.start(),
                    // Cut the range in half
                    range.start() + (range.end() - range.start()) / 2,
                ),
                DirectionRow::Back => RangeInclusive::new(
                    // Cut the range in half
                    range.start() + (range.end() - range.start()) / 2 + 1,
                    // Keep the same end
                    *range.end(),
                ),
            },
        );

    let column =
        direction.col_iter.iter().fold(
            RangeInclusive::new(0, 7),
            |range, direction| match direction {
                DirectionColumn::Left => RangeInclusive::new(
                    // Keep the same start
                    *range.start(),
                    // Cut the range in half
                    range.start() + (range.end() - range.start()) / 2,
                ),
                DirectionColumn::Right => RangeInclusive::new(
                    // Cut the range in half
                    range.start() + (range.end() - range.start()) / 2 + 1,
                    // Keep the same end
                    *range.end(),
                ),
            },
        );

    Seat {
        row: *row.start(),
        column: *column.start(),
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

                let expected = 922;
                let to_check = Day05Initial::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day05Initial;
            use crate::{
                initial::{find_seat, Seat},
                parse_input,
                tests::init_logger,
                AoC,
            };

            #[test]
            fn ex01_seat() {
                init_logger();

                let input = "FBFBBFFRLR";
                let expected = Seat { row: 44, column: 5 };
                let direction = parse_input(input).next().unwrap();
                let to_check = find_seat(&direction);

                assert_eq!(to_check, expected);
                assert_eq!(to_check.seat_id(), 357);
            }

            #[test]
            fn ex02_seat() {
                init_logger();

                let input = "BFFFBBFRRR";
                let expected = Seat { row: 70, column: 7 };
                let direction = parse_input(input).next().unwrap();
                let to_check = find_seat(&direction);

                assert_eq!(to_check, expected);
                assert_eq!(to_check.seat_id(), 567);
            }

            #[test]
            fn ex03_seat() {
                init_logger();

                let input = "FFFBBBFRRR";
                let expected = Seat { row: 14, column: 7 };
                let direction = parse_input(input).next().unwrap();
                let to_check = find_seat(&direction);

                assert_eq!(to_check, expected);
                assert_eq!(to_check.seat_id(), 119);
            }

            #[test]
            fn ex04_seat() {
                init_logger();

                let input = "BBFFBBFRLL";
                let expected = Seat {
                    row: 102,
                    column: 4,
                };
                let direction = parse_input(input).next().unwrap();
                let to_check = find_seat(&direction);

                assert_eq!(to_check, expected);
                assert_eq!(to_check.seat_id(), 820);
            }

            #[test]
            fn ex01() {
                init_logger();

                let input = "FBFBBFFRLR
                                BFFFBBFRRR
                                FFFBBBFRRR
                                BBFFBBFRLL";
                let expected = 820;
                let to_check = Day05Initial::new(input).solution_part1();
                assert_eq!(to_check, expected);
            }
        }
    }

    mod part2 {
        mod solution {
            use super::super::super::*;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 747;
                let to_check = Day05Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }
    }
}
