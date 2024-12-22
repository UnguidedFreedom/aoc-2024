#![feature(iter_map_windows)]

use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;

advent_of_code::solution!(22);

fn mix_prune(new: u64, old: u64) -> u64 {
    (new ^ old) % 16777216
}

fn next_val(val: u64) -> u64 {
    let mut val = mix_prune(val * 64, val);
    val = mix_prune(val / 32, val);
    mix_prune(val * 2048, val)
}

fn calc(initial: u64, n: usize) -> u64 {
    let mut val = initial;

    for _ in 0..n {
        val = next_val(val);
    }

    val
}

pub fn part_one(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .map(|s| s.parse::<u64>().unwrap())
        .map(|val| calc(val, 2000))
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let starts = input
        .lines()
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();

    let n = starts.len();

    let mut sequences = HashMap::new();

    // TODO: generate individual hashmaps and merge them at the end?

    for (i, &start) in starts.iter().enumerate() {
        let mut values = vec![start];
        let mut val = start;
        for _ in 0..2000 {
            val = next_val(val);
            values.push(val);
        }

        let values = values.into_iter().map(|x| (x % 10) as i64).collect_vec();

        let diffs = values.iter().map_windows(|&[&a, &b]| b - a);

        let seqs = diffs.map_windows(|&[a, b, c, d]| (a, b, c, d));

        for (j, seq) in seqs.enumerate() {
            let entry = sequences.entry(seq).or_insert(vec![None; n]);
            if entry[i].is_none() {
                entry[i] = Some(values[j + 4]);
            }
        }
    }

    let max = sequences
        .par_iter()
        .map(|(_, vals)| vals.iter().flatten().sum::<i64>())
        .max()
        .unwrap() as u64;

    Some(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
