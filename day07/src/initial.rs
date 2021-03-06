use std::collections::HashSet;

use crate::*;

#[derive(Debug)]
pub struct Day07Initial<'a> {
    input: &'a str,
}

impl<'a> AoC<'a> for Day07Initial<'a> {
    type SolutionPart1 = Day07SolutionPart1;
    type SolutionPart2 = Day07SolutionPart2;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &'a str) -> Day07Initial<'_> {
        Day07Initial { input }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        let rules = parse_input(self.input);

        // Invert the rules
        let mut inverted_rules: HashMap<&str, Vec<&str>> = HashMap::new();
        for (color_host, colors_contained) in &rules {
            for (_bags_count, color_contained) in colors_contained {
                inverted_rules
                    .entry(color_contained)
                    .or_insert(Vec::new())
                    .push(color_host);
            }
        }

        let mut stack: Vec<&str> = inverted_rules["shiny gold"].clone();
        let mut possibility: HashSet<&str> = HashSet::new();

        while !stack.is_empty() {
            let color_containing = stack.pop().expect("non-empty stack");

            possibility.insert(color_containing);
            if inverted_rules.contains_key(color_containing) {
                stack.extend(inverted_rules[color_containing].iter());
                possibility.extend(inverted_rules[color_containing].iter());
            }
        }

        possibility.len()
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        let rules: HashMap<&str, Vec<(usize, &str)>> = parse_input(self.input);

        let count = recursive_count(&rules, &(1, "shiny gold"));

        // We needed to count the initial bag for the recursion to work, remove it here.
        count - 1
    }
}

fn recursive_count(rules: &HashMap<&str, Vec<(usize, &str)>>, bag: &(usize, &str)) -> usize {
    let children = &rules[bag.1];
    if children.is_empty() {
        bag.0
    } else {
        let sum = children
            .iter()
            .map(|child| recursive_count(rules, child))
            .sum::<usize>();
        bag.0 + bag.0 * sum
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

                let expected = 259;
                let to_check = Day07Initial::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day07Initial;
            use crate::{parse_input, tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn input_length() {
                init_logger();
                let expected = 594;
                let rules = parse_input(PUZZLE_INPUT);
                assert_eq!(rules.len(), expected);
            }

            #[test]
            fn ex01() {
                init_logger();

                let expected = 4;
                let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
                dark orange bags contain 3 bright white bags, 4 muted yellow bags.
                bright white bags contain 1 shiny gold bag.
                muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
                shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
                dark olive bags contain 3 faded blue bags, 4 dotted black bags.
                vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
                faded blue bags contain no other bags.
                dotted black bags contain no other bags.";
                let to_check = Day07Initial::new(input).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day07Initial;
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

                let expected = 45018;
                let to_check = Day07Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day07Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 32;
                let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
                                dark orange bags contain 3 bright white bags, 4 muted yellow bags.
                                bright white bags contain 1 shiny gold bag.
                                muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
                                shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
                                dark olive bags contain 3 faded blue bags, 4 dotted black bags.
                                vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
                                faded blue bags contain no other bags.
                                dotted black bags contain no other bags.";
                let to_check = Day07Initial::new(input).solution_part2();

                assert_eq!(to_check, expected);
            }

            #[test]
            fn ex02() {
                init_logger();

                let expected = 126;
                let input = "shiny gold bags contain 2 dark red bags.
                                dark red bags contain 2 dark orange bags.
                                dark orange bags contain 2 dark yellow bags.
                                dark yellow bags contain 2 dark green bags.
                                dark green bags contain 2 dark blue bags.
                                dark blue bags contain 2 dark violet bags.
                                dark violet bags contain no other bags.";
                let to_check = Day07Initial::new(input).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day07Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }
}
