use itertools::Itertools;
use std::collections::HashMap;
use std::iter::zip;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut first, mut second): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip();

    first.sort();
    second.sort();

    let result = zip(first, second).map(|(a, b)| a.abs_diff(b)).sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left: HashMap<u32, u32> = HashMap::new();
    let mut right: HashMap<u32, u32> = HashMap::new();
    for line in input.lines() {
        let (a, b) = line
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();
        *left.entry(a).or_insert(0) += 1;
        *right.entry(b).or_insert(0) += 1;
    }

    let result = left
        .iter()
        .filter_map(|(val, count)| Some(right.get(val)? * val * count))
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
