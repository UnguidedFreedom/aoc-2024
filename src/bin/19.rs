use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(19);

fn doable(line: &str, towels: &[&str]) -> bool {
    if line.is_empty() {
        return true;
    }

    for &towel in towels {
        if let Some(stripped) = line.strip_prefix(towel) {
            if doable(stripped, towels) {
                return true;
            }
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let towels = lines.next().unwrap().split(", ").collect_vec();

    lines.next();

    let response = lines.filter(|line| doable(line, &towels)).count() as u64;

    Some(response)
}

fn doable_count(line: &str, towels: &[&str], memo: &mut HashMap<String, u64>) -> u64 {
    if line.is_empty() {
        return 1;
    }

    if let Some(&count) = memo.get(line) {
        return count;
    }

    let mut ways = 0;

    for &towel in towels {
        if let Some(stripped) = line.strip_prefix(towel) {
            ways += doable_count(stripped, towels, memo);
        } else if towel > line {
            break;
        }
    }

    memo.insert(line.to_string(), ways);

    ways
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let mut towels = lines.next().unwrap().split(", ").collect_vec();

    towels.sort();

    lines.next();

    let mut memo = HashMap::new();

    let response = lines
        .map(|line| doable_count(line, &towels, &mut memo))
        .sum();

    Some(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
