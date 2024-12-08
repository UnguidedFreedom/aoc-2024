use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let antennas = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter_map(move |(j, &char)| {
                    (char != b'.').then_some((char, (i as isize, j as isize)))
                })
        })
        .into_group_map();

    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let irange = 0..height as isize;
    let jrange = 0..width as isize;

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for (_, ants) in antennas {
        for coords in ants.iter().combinations(2) {
            let (&&a, &&b) = coords.iter().collect_tuple().unwrap();
            let before = (2 * a.0 - b.0, 2 * a.1 - b.1);
            if irange.contains(&before.0) && jrange.contains(&before.1) {
                antinodes.insert(before);
            }
            let after = (2 * b.0 - a.0, 2 * b.1 - a.1);
            if irange.contains(&after.0) && jrange.contains(&after.1) {
                antinodes.insert(after);
            }
        }
    }

    Some(antinodes.len() as u32)
}

fn gcd(a: isize, b: isize) -> isize {
    if a == 0 || b == 0 {
        return 1;
    }
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

pub fn part_two(input: &str) -> Option<u32> {
    let antennas = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter_map(move |(j, &char)| {
                    (char != b'.').then_some((char, (i as isize, j as isize)))
                })
        })
        .into_group_map();

    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let irange = 0..height as isize;
    let jrange = 0..width as isize;

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for (_, ants) in antennas {
        for coords in ants.iter().combinations(2) {
            let (&&a, &&b) = coords.iter().collect_tuple().unwrap();
            let diffs = (a.0 - b.0, a.1 - b.1);
            let gcd_diffs = gcd(diffs.0, diffs.1);
            let diffs = (diffs.0 / gcd_diffs, diffs.1 / gcd_diffs);

            let (mut i, mut j) = a;
            while irange.contains(&i) && jrange.contains(&j) {
                antinodes.insert((i, j));
                (i, j) = (i - diffs.0, j - diffs.1);
            }

            (i, j) = (a.0 + diffs.0, a.1 + diffs.1);
            while irange.contains(&i) && jrange.contains(&j) {
                antinodes.insert((i, j));
                (i, j) = (i + diffs.0, j + diffs.1);
            }
        }
    }

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
