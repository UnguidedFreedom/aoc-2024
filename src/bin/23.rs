#![feature(iter_map_windows)]
extern crate core;

use itertools::Itertools;
use rayon::prelude::*;
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
    so_far: &[&'a str],
    candidates: &HashSet<&'a str>,
    conns: &HashMap<&'a str, HashSet<&'a str>>,
) -> Vec<&'a str> {
    let mut longest = so_far.to_owned();
    for &candidate in candidates {
        let mut sf2 = so_far.to_owned();
        sf2.push(candidate);

        if let Some(vals) = conns.get(&candidate) {
            let intersection = candidates
                .intersection(vals)
                .copied()
                .collect::<HashSet<&str>>();

            sf2 = evaluate(&sf2, &intersection, conns);
        }

        if sf2.len() > longest.len() {
            longest = sf2;
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
        if a < b {
            conns.entry(a).or_default().insert(b);
        } else {
            conns.entry(b).or_default().insert(a);
        }
    }

    let longest = conns
        .par_iter()
        .map(|(&key, val)| evaluate(&[key], val, &conns))
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
