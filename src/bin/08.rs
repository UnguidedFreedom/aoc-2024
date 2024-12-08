use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let irange = 0..height as isize;
    let jrange = 0..width as isize;

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    input
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
        .into_group_map()
        .iter()
        .flat_map(|(_, ants)| {
            ants.iter()
                .tuple_combinations::<(&(isize, isize), &(isize, isize))>()
        })
        .for_each(|(&(i_a, j_a), &(i_b, j_b))| {
            let (i, j) = (2 * i_a - i_b, 2 * j_a - j_b);
            if irange.contains(&i) && jrange.contains(&j) {
                antinodes.insert((i, j));
            }

            let (i, j) = (2 * i_b - i_a, 2 * j_b - j_a);
            if irange.contains(&i) && jrange.contains(&j) {
                antinodes.insert((i, j));
            }
        });

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
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let irange = 0..height as isize;
    let jrange = 0..width as isize;

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    input
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
        .into_group_map()
        .iter()
        .flat_map(|(_, ants)| {
            ants.iter()
                .tuple_combinations::<(&(isize, isize), &(isize, isize))>()
        })
        .for_each(|(&(i_a, j_a), &(i_b, j_b))| {
            let (di, dj) = (i_a - i_b, j_a - j_b);
            let gcd_diffs = gcd(di, dj);
            let (di, dj) = (di / gcd_diffs, dj / gcd_diffs);

            let (mut i, mut j) = (i_a, i_b);
            while irange.contains(&i) && jrange.contains(&j) {
                antinodes.insert((i, j));
                (i, j) = (i - di, j - dj);
            }

            (i, j) = (i_a + di, j_a + dj);
            while irange.contains(&i) && jrange.contains(&j) {
                antinodes.insert((i, j));
                (i, j) = (i + di, j + dj);
            }
        });

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
