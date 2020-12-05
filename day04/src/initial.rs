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

    fn solution_part2(&self) -> Self::SolutionPart2 {
        parse_input(self.input)
            .filter_map(|passport_builder| passport_builder.build_part2())
            .count()
    }
}

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

    pub fn build_part2(self) -> Option<ValidPassport<'a>> {
        match (
            self.byr, self.iyr, self.eyr, self.hgt, self.hcl, self.ecl, self.pid,
        ) {
            (Some(byr), Some(iyr), Some(eyr), Some(hgt), Some(hcl), Some(ecl), Some(pid)) => {
                // Check each field for their own validity
                if hgt.is_valid()
                    && iyr.is_valid()
                    && eyr.is_valid()
                    && hcl.is_valid()
                    && ecl.is_valid()
                    && pid.is_valid()
                    && byr.is_valid()
                {
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
                        Some(cid) => {
                            Some(ValidPassport::ValidPassportWithCid(ValidPassportWithCid {
                                cid,
                                common,
                            }))
                        }
                        None => Some(ValidPassport::ValidPassportWithoutCid(
                            ValidPassportWithoutCid { common },
                        )),
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn is_valid_part2(&self) -> bool {
        self.clone().build_part2().is_some()
    }
}

mod part2 {
    use super::*;

    impl<'a> BirthYear<'a> {
        pub fn is_valid(&self) -> bool {
            // four digits; at least 1920 and at most 2002.
            let value = self.0;
            match value.chars().count() {
                4 => match value.parse::<i16>() {
                    Ok(parsed) => 1920 <= parsed && parsed <= 2002,
                    Err(_err) => {
                        // log::info!("Failed to parse {:?} as integer", value);
                        false
                    }
                },
                _ => false,
            }
        }
    }

    impl<'a> IssueYear<'a> {
        pub fn is_valid(&self) -> bool {
            // four digits; at least 2010 and at most 2020.
            let value = self.0;
            match value.chars().count() {
                4 => match value.parse::<i16>() {
                    Ok(parsed) => 2010 <= parsed && parsed <= 2020,
                    Err(_err) => {
                        // log::info!("Failed to parse {:?} as integer", value);
                        false
                    }
                },
                _ => false,
            }
        }
    }

    impl<'a> ExpirationYear<'a> {
        pub fn is_valid(&self) -> bool {
            // four digits; at least 2020 and at most 2030.
            let value = self.0;
            match value.chars().count() {
                4 => match value.parse::<i16>() {
                    Ok(parsed) => 2020 <= parsed && parsed <= 2030,
                    Err(_err) => {
                        // log::info!("Failed to parse {:?} as integer", value);
                        false
                    }
                },
                _ => false,
            }
        }
    }

    impl<'a> Height<'a> {
        pub fn is_valid(&self) -> bool {
            // a number followed by either cm or in:
            // If cm, the number must be at least 150 and at most 193.
            // If in, the number must be at least 59 and at most 76.
            let value = self.0;
            let height = value.trim_end_matches("cm").trim_end_matches("in");
            match height.parse::<i16>() {
                Ok(height) => {
                    if value.ends_with("cm") {
                        150 <= height && height <= 193
                    } else if value.ends_with("in") {
                        59 <= height && height <= 76
                    } else {
                        // log::info!("Failed to get unit for height {:?}", value);
                        false
                    }
                }
                Err(_err) => {
                    // log::info!("Failed to parse height {:?} as integer", height);
                    false
                }
            }
        }
    }

    impl<'a> HairColor<'a> {
        pub fn is_valid(&self) -> bool {
            // a # followed by exactly six characters 0-9 or a-f.
            let value = self.0;
            match value.chars().count() {
                7 => {
                    let mut color_iter = value.chars();
                    match color_iter.next().expect("a character") {
                        '#' => color_iter.all(|c| matches!(c, ('0'..='9') | ('a'..='f'))),
                        _ => false,
                    }
                }
                _ => false,
            }
        }
    }

    impl<'a> EyeColor<'a> {
        pub fn is_valid(&self) -> bool {
            // exactly one of: amb blu brn gry grn hzl oth.
            let value = self.0;

            matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
        }
    }

    impl<'a> PassportId<'a> {
        pub fn is_valid(&self) -> bool {
            // a nine-digit number, including leading zeroes.
            let value = self.0;

            match value.chars().count() {
                9 => value.chars().all(|c| matches!(c, '0'..='9')),
                _ => false,
            }
        }
    }

    // impl<'a> CountryId<'a> {
    //     pub fn is_valid(&self) -> bool {
    //         // ignored, missing or not.
    //         true
    //     }
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

                let expected = 123;
                let to_check = Day04Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use crate::{
                parse_input, tests::init_logger, BirthYear, EyeColor, HairColor, Height, PassportId,
            };

            #[test]
            fn birth_year() {
                init_logger();

                assert!(BirthYear("2002").is_valid());
                assert!(!BirthYear("2003").is_valid());
            }

            #[test]
            fn height() {
                init_logger();

                assert!(Height("60in").is_valid());
                assert!(Height("190cm").is_valid());
                assert!(!Height("190in").is_valid());
                assert!(!Height("190").is_valid());
            }

            #[test]
            fn hair_color() {
                init_logger();

                assert!(HairColor("#123abc").is_valid());
                assert!(!HairColor("#123abz").is_valid());
                assert!(!HairColor("123abc").is_valid());
            }

            #[test]
            fn eye_color() {
                init_logger();

                assert!(EyeColor("brn").is_valid());
                assert!(!EyeColor("wat").is_valid());
            }

            #[test]
            fn passport_id() {
                init_logger();

                assert!(PassportId("000000001").is_valid());
                assert!(!PassportId("0123456789").is_valid());
            }

            #[test]
            fn invalid_passports() {
                let passports = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

                assert!(parse_input(passports).all(|passport| !passport.is_valid_part2()));
            }

            #[test]
            fn valid_passports() {
                let passports = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

                assert!(parse_input(passports).all(|passport| passport.is_valid_part2()));
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
