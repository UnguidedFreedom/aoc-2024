#![feature(iter_map_windows)]
extern crate core;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(23);

fn is_t(s: &str) -> bool {
    s.starts_with('t')
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut ts = HashSet::new();
    let mut conns: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.split("-").collect_tuple().unwrap();
        if is_t(a) {
            ts.insert(a);
        }
        if is_t(b) {
            ts.insert(b);
        }

        conns.entry(a).or_default().insert(b);
        conns.entry(b).or_default().insert(a);
    }

    let response = ts
        .iter()
        .flat_map(|&t| {
            conns[t].iter().combinations(2).filter(|v| {
                let [&a, &b] = v.as_slice() else {
                    return false;
                };
                if (is_t(a) && a < t) || (is_t(b) && b < t) {
                    return false;
                }

                conns[a].contains(b)
            })
        })
        .count() as u64;

    Some(response)
}

fn evaluate<'a>(
    so_far: &Vec<&'a str>,
    candidates: &HashSet<&'a str>,
    conns: &HashMap<&'a str, HashSet<&'a str>>,
) -> Vec<&'a str> {
    let mut longest = so_far.clone();
    let last = *so_far.last().unwrap();
    for &candidate in candidates.iter().filter(|&&x| x > last) {
        let intersection = candidates
            .intersection(&conns[candidate])
            .copied()
            .collect::<HashSet<&str>>();

        let mut sf2 = so_far.clone();
        sf2.push(candidate);

        let result = evaluate(&sf2, &intersection, conns);
        if result.len() > longest.len() {
            longest = result;
        }
    }
    longest
}

pub fn part_two(input: &str) -> Option<String> {
    let data = input
        .lines()
        .map(|line| line.split("-").collect_tuple::<(&str, &str)>().unwrap())
        .collect_vec();

    let mut conns: HashMap<&str, HashSet<&str>> = HashMap::new();

    for &(a, b) in data.iter() {
        conns.entry(a).or_default().insert(b);
        conns.entry(b).or_default().insert(a);
    }

    let longest = conns
        .iter()
        .map(|(&key, val)| evaluate(&vec![key], val, &conns))
        .max_by(|s1, s2| s1.len().cmp(&s2.len()))
        .unwrap();

    let result = longest.join(",");

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
