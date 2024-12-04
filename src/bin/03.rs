advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let result = re
        .captures_iter(input)
        .map(|caps| {
            caps.extract::<2>()
                .1
                .iter()
                .map(|s| s.parse::<u32>().unwrap())
                .product::<u32>()
        })
        .sum::<u32>();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(
        r"mul\((?<lhs>[0-9]{1,3}),(?<rhs>[0-9]{1,3})\)|(?<do>do\(\))|(?<dont>don't\(\))",
    )
    .unwrap();
    let result = re
        .captures_iter(input)
        .fold((0, true), |state, caps| {
            if caps.name("do").is_some() {
                return (state.0, true);
            }
            if caps.name("dont").is_some() {
                return (state.0, false);
            }

            if !state.1 {
                return state;
            }

            let lhs = caps["lhs"].parse::<u32>().unwrap();
            let rhs = caps["rhs"].parse::<u32>().unwrap();

            (state.0 + lhs * rhs, state.1)
        })
        .0;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
