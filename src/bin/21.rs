#![feature(iter_map_windows)]

use ::phf::{phf_map, Map};
use itertools::Itertools;
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

fn digit_sequence(source: char, dest: char) -> Vec<String> {
    let &(i1, j1) = NUM_POS.get(&source).unwrap();
    let &(i2, j2) = NUM_POS.get(&dest).unwrap();

    let (di, dj) = (i2 - i1, j2 - j1);

    let mut result = Vec::new();

    if (i1 + di, j1) != (3, 0) {
        let mut seq = String::new();
        apply_vert(&mut seq, di);
        apply_horiz(&mut seq, dj);
        seq.push('A');
        result.push(seq);
    }

    if (i1, j1 + dj) != (3, 0) {
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

fn arrow_sequence(source: char, dest: char) -> String {
    let &(i1, j1) = DIR_POS.get(&source).unwrap();
    let &(i2, j2) = DIR_POS.get(&dest).unwrap();

    let (di, dj) = (i2 - i1, j2 - j1);

    let mut seq = String::new();

    if (i1 + di, j1) != (0, 0) {
        apply_vert(&mut seq, di);
        apply_horiz(&mut seq, dj);
    } else {
        apply_horiz(&mut seq, dj);
        apply_vert(&mut seq, di);
    }

    seq.push('A');
    seq
}

fn next_arrow(input: String, memo: &mut HashMap<String, Vec<String>>) -> Vec<String> {
    if let Some(res) = memo.get(&input) {
        return res.clone();
    }

    let mut seq = "A".to_string();
    seq.push_str(input.clone().as_str());

    let output = seq
        .chars()
        .map_windows(|&[a, b]| arrow_sequence(a, b))
        .collect_vec();

    memo.insert(input, output.clone());
    output
}

fn next_arrows(input: Vec<String>, memo: &mut HashMap<String, Vec<String>>) -> Vec<String> {
    input
        .into_iter()
        .flat_map(|s| next_arrow(s, memo))
        .collect_vec()
}

fn into_possibles(input: &Vec<Vec<String>>) -> Vec<String> {
    let combos = input.iter().map(|x| x.len()).product::<usize>();

    let mut result = Vec::new();

    for k in 0..combos {
        let mut k2 = k;
        let mut val = String::new();
        for v in input {
            let l = v.len();
            let i = k2 % l;
            k2 /= l;
            val.push_str(v[i].as_str());
        }

        result.push(val);
    }

    result
}

fn vec_len(input: &[String]) -> usize {
    input.iter().map(|s| s.len()).sum::<usize>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines().collect_vec();

    let result = lines
        .iter()
        .map(|&line| {
            let mut seq = "A".to_string();
            seq.push_str(line);

            let output = seq
                .chars()
                .map_windows(|&[a, b]| digit_sequence(a, b))
                .collect_vec();

            // go one level to be within the realm of automation

            let mut sources = into_possibles(&output)
                .into_iter()
                .map(|poss| {
                    let mut seq = "A".to_string();
                    seq.push_str(poss.as_str());

                    seq.chars()
                        .map_windows(|&[a, b]| arrow_sequence(a, b))
                        .collect_vec()
                })
                .collect_vec();

            let mut memo = HashMap::new();

            sources = sources
                .into_iter()
                .map(|source| next_arrows(source, &mut memo))
                .collect_vec();

            let shortest = sources
                .into_iter()
                .min_by(|v1, v2| vec_len(v1).cmp(&vec_len(v2)))
                .unwrap();

            let min_len = vec_len(&shortest);
            let code_num = line[0..3].parse::<u64>().unwrap();

            code_num * min_len as u64
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
