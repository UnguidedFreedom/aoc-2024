use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let mut counts = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .counts();

    for _ in 0..25 {
        let mut new_counts: HashMap<u64, usize> = HashMap::with_capacity(2 * counts.len());

        for (val, c) in counts {
            if val == 0 {
                *new_counts.entry(1).or_default() += c;
                continue;
            }

            let i = val.ilog10() + 1;

            if i % 2 == 0 {
                let div = 10u64.pow(i / 2);
                let (first, second) = (val / div, val % div);

                *new_counts.entry(first).or_default() += c;
                *new_counts.entry(second).or_default() += c;
                continue;
            }

            *new_counts.entry(val * 2024).or_default() += c;
        }

        counts = new_counts;
    }

    let response: u32 = counts.into_values().sum::<usize>() as u32;
    Some(response)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut counts = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .counts();

    for _ in 0..75 {
        let mut new_counts: HashMap<u64, usize> = HashMap::with_capacity(2 * counts.len());

        for (val, c) in counts {
            if val == 0 {
                *new_counts.entry(1).or_default() += c;
                continue;
            }

            let i = val.ilog10() + 1;

            if i % 2 == 0 {
                let div = 10u64.pow(i / 2);
                let (first, second) = (val / div, val % div);

                *new_counts.entry(first).or_default() += c;
                *new_counts.entry(second).or_default() += c;
                continue;
            }

            *new_counts.entry(val * 2024).or_default() += c;
        }

        counts = new_counts;
    }

    let response: u64 = counts.into_values().sum::<usize>() as u64;
    Some(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
