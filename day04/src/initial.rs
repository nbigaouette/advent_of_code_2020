use crate::*;

#[derive(Debug)]
pub struct Day04Initial<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day04Initial<'a> {
    type SolutionPart1 = Day04SolutionPart1;
    type SolutionPart2 = Day04SolutionPart2;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &'a str) -> Day04Initial<'_> {
        Day04Initial { input }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        parse_input(self.input)
            .filter_map(|passport_builder| passport_builder.build_part1())
            .count()
    }

    // fn solution_part2(&self) -> Self::SolutionPart2 {
#[derive(Debug, PartialEq)]
pub struct ValidPassportCommon<'a> {
    /// Height
    hgt: Height<'a>,
    /// Issue Year
    iyr: IssueYear<'a>,
    /// Expiration Year
    eyr: ExpirationYear<'a>,
    /// Hair Color
    hcl: HairColor<'a>,
    /// Eye Color
    ecl: EyeColor<'a>,
    /// Passport ID
    pid: PassportId<'a>,
    /// Birth Year
    byr: BirthYear<'a>,
}

#[derive(Debug, PartialEq)]
pub struct ValidPassportWithCid<'a> {
    /// Country ID
    cid: CountryId<'a>,

    common: ValidPassportCommon<'a>,
}

#[derive(Debug, PartialEq)]
pub struct ValidPassportWithoutCid<'a> {
    common: ValidPassportCommon<'a>,
}

#[derive(Debug, PartialEq)]
pub enum ValidPassport<'a> {
    ValidPassportWithCid(ValidPassportWithCid<'a>),
    ValidPassportWithoutCid(ValidPassportWithoutCid<'a>),
}

impl<'a> PassportBuilder<'a> {
    pub fn build_part1(self) -> Option<ValidPassport<'a>> {
        match (
            self.byr, self.iyr, self.eyr, self.hgt, self.hcl, self.ecl, self.pid,
        ) {
            (Some(byr), Some(iyr), Some(eyr), Some(hgt), Some(hcl), Some(ecl), Some(pid)) => {
                let common = ValidPassportCommon {
                    hgt,
                    iyr,
                    eyr,
                    hcl,
                    ecl,
                    pid,
                    byr,
                };

                match self.cid {
                    Some(cid) => Some(ValidPassport::ValidPassportWithCid(ValidPassportWithCid {
                        cid,
                        common,
                    })),
                    None => Some(ValidPassport::ValidPassportWithoutCid(
                        ValidPassportWithoutCid { common },
                    )),
                }
            }
            _ => None,
        }
    }

    pub fn is_valid_part1(&self) -> bool {
        self.clone().build_part1().is_some()
    }
}
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

                let expected = 206;
                let to_check = Day04Initial::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day04Initial;
            use crate::{
                parse_input,
                tests::{init_logger, EXAMPLE_INPUT},
                AoC,
            };

            #[test]
            fn ex01() {
                init_logger();

                let expected = 2;
                let input = EXAMPLE_INPUT;
                let to_check = Day04Initial::new(input).solution_part1();

                assert_eq!(to_check, expected);
            }

            #[test]
            fn ex01_passport_1_is_valid() {
                init_logger();

                let passports: Vec<_> = parse_input(EXAMPLE_INPUT).collect();

                assert!(passports[0].is_valid_part1());
            }

            #[test]
            fn ex01_passport_2_is_invalid() {
                init_logger();

                let passports: Vec<_> = parse_input(EXAMPLE_INPUT).collect();

                assert!(!passports[1].is_valid_part1());
            }

            #[test]
            fn ex01_passport_3_is_valid() {
                init_logger();

                let passports: Vec<_> = parse_input(EXAMPLE_INPUT).collect();

                assert!(passports[2].is_valid_part1());
            }

            #[test]
            fn ex01_passport_4_is_invalid() {
                init_logger();

                let passports: Vec<_> = parse_input(EXAMPLE_INPUT).collect();

                assert!(!passports[3].is_valid_part1());
            }
        }

        /*
        mod extra {
            use super::super::super::Day04Initial;
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
                let to_check = Day04Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day04Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                unimplemented!();

                let expected = 0;
                let input = "";
                let to_check = Day04Initial::new(input).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day04Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }
}
