#![feature(iter_map_windows)]

use ::phf::{phf_map, Map};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter;

advent_of_code::solution!(21);

static NUM_POS: Map<char, (isize, isize)> = phf_map! {
    '7' => (0, 0),
    '8' => (0, 1),
    '9' => (0, 2),
    '4' => (1, 0),
    '5' => (1, 1),
    '6' => (1, 2),
    '1' => (2, 0),
    '2' => (2, 1),
    '3' => (2, 2),
    '0' => (3, 1),
    'A' => (3, 2),
};

static DIR_POS: Map<char, (isize, isize)> = phf_map! {
    '^' => (0, 1),
    'A' => (0, 2),
    '<' => (1, 0),
    'v' => (1, 1),
    '>' => (1, 2),
};

fn apply_vert(result: &mut String, di: isize) {
    match di.cmp(&0) {
        Ordering::Greater => result.extend(iter::repeat_n("v", di as usize)),
        Ordering::Less => result.extend(iter::repeat_n("^", di.unsigned_abs())),
        _ => (),
    }
}

fn apply_horiz(result: &mut String, dj: isize) {
    match dj.cmp(&0) {
        Ordering::Greater => result.extend(iter::repeat_n(">", dj as usize)),
        Ordering::Less => result.extend(iter::repeat_n("<", dj.unsigned_abs())),
        _ => (),
    }
}

fn sequences(
    source: char,
    dest: char,
    map: &Map<char, (isize, isize)>,
    forbidden: (isize, isize),
) -> Vec<String> {
    let &(i1, j1) = map.get(&source).unwrap();
    let &(i2, j2) = map.get(&dest).unwrap();

    let (di, dj) = (i2 - i1, j2 - j1);

    let mut result = Vec::new();

    if (i1 + di, j1) != forbidden {
        let mut seq = String::new();
        apply_vert(&mut seq, di);
        apply_horiz(&mut seq, dj);
        seq.push('A');
        result.push(seq);
    }

    if (i1, j1 + dj) != forbidden {
        let mut seq = String::new();
        apply_horiz(&mut seq, dj);
        apply_vert(&mut seq, di);
        seq.push('A');

        if !result.contains(&seq) {
            result.push(seq);
        }
    }

    result
}

fn presses(input: String, depth: usize, memo: &mut HashMap<(String, usize), u64>) -> u64 {
    if depth == 0 {
        return input.len() as u64;
    }

    let key = (input.clone(), depth);

    if let Some(&result) = memo.get(&key) {
        return result;
    }

    let seq = "A".to_owned() + &input;
    let result = seq
        .chars()
        .map_windows(|&[a, b]| {
            sequences(a, b, &DIR_POS, (0, 0))
                .into_iter()
                .map(|s| presses(s, depth - 1, memo))
                .min()
                .unwrap()
        })
        .sum();

    memo.insert(key, result);
    result
}

fn solve_for_line(line: &str, depth: usize, memo: &mut HashMap<(String, usize), u64>) -> u64 {
    let seq = "A".to_owned() + line;

    let min_len = seq
        .chars()
        .map_windows(|&[a, b]| {
            sequences(a, b, &NUM_POS, (3, 0))
                .into_iter()
                .map(|s| presses(s, depth, memo))
                .min()
                .unwrap()
        })
        .sum::<u64>();

    let code_num = line[0..3].parse::<u64>().unwrap();

    code_num * min_len
}

fn solve(input: &str, depth: usize) -> Option<u64> {
    let mut memo = HashMap::new();

    let result = input
        .lines()
        .map(|line| solve_for_line(line, depth, &mut memo))
        .sum();

    Some(result)
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 25)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154115708116294));
    }
}
