use std::collections::HashMap;

advent_of_code::solution!(1);

#[macro_use]
extern crate text_io;

pub fn part_one(input: &str) -> Option<u32> {
    let mut first: Vec<u32> = Vec::new();
    let mut second: Vec<u32> = Vec::new();
    for line in input.lines() {
        let (a, b): (u32, u32);
        scan!(line.bytes() => "{} {}", a, b);
        first.push(a);
        second.push(b);
    }
    first.sort();
    second.sort();

    let result = first
        .iter()
        .zip(second.iter())
        .map(|(&a, &b)| a.abs_diff(b))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left: HashMap<u32, u32> = HashMap::new();
    let mut right: HashMap<u32, u32> = HashMap::new();
    for line in input.lines() {
        let (a, b): (u32, u32);
        scan!(line.bytes() => "{} {}", a, b);
        *left.entry(a).or_insert(0) += 1;
        *right.entry(b).or_insert(0) += 1;
    }

    let result = left
        .iter()
        .filter_map(|(val, count)| Some(val * count * right.get(val)?))
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
