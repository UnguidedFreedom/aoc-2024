use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(20);

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn part_one(input: &str) -> Option<u64> {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut grid = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '#' => None,
                    'S' => {
                        start = (i as isize, j as isize);
                        Some(0)
                    }
                    'E' => {
                        end = (i as isize, j as isize);
                        Some(u64::MAX)
                    }
                    '.' => Some(u64::MAX),
                    _ => panic!("Invalid character: {}", c),
                })
                .collect_vec()
        })
        .collect_vec();

    let height = grid.len();
    let width = grid[0].len();

    let irange = 0..height as isize;
    let jrange = 0..width as isize;

    let mut distance = 0;
    let (mut i, mut j) = start;

    while (i, j) != end {
        for (di, dj) in DIRS {
            let (i2, j2) = (i + di, j + dj);
            if irange.contains(&i2)
                && jrange.contains(&j2)
                && grid[i2 as usize][j2 as usize] == Some(u64::MAX)
            {
                distance += 1;
                grid[i2 as usize][j2 as usize] = Some(distance);
                (i, j) = (i2, j2);
                break;
            }
        }
    }

    (i, j) = start;

    let mut response = 0;

    while (i, j) != end {
        let curr_dist = grid[i as usize][j as usize].unwrap();

        let mut next = (i, j);
        for (di, dj) in DIRS {
            let (i2, j2) = (i + 2 * di, j + 2 * dj);
            if irange.contains(&i2) && jrange.contains(&j2) {
                if let Some(dist) = grid[i2 as usize][j2 as usize] {
                    if dist > curr_dist + 2 {
                        let savings = dist - curr_dist - 2;
                        if savings >= 100 {
                            response += 1;
                        }
                    }
                }
            }
            let (i3, j3) = (i + di, j + dj);
            if irange.contains(&i3)
                && jrange.contains(&j3)
                && grid[i3 as usize][j3 as usize] == Some(curr_dist + 1)
            {
                next = (i3, j3);
            }
        }

        (i, j) = next;
    }

    Some(response)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut grid = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '#' => None,
                    'S' => {
                        start = (i as isize, j as isize);
                        Some(0)
                    }
                    'E' => {
                        end = (i as isize, j as isize);
                        Some(u64::MAX)
                    }
                    '.' => Some(u64::MAX),
                    _ => panic!("Invalid character: {}", c),
                })
                .collect_vec()
        })
        .collect_vec();

    let height = grid.len();
    let width = grid[0].len();

    let irange = 0..height as isize;
    let jrange = 0..width as isize;

    let mut distance = 0;
    let (mut i, mut j) = start;

    let mut path = vec![start];

    while (i, j) != end {
        for (di, dj) in DIRS {
            let (i2, j2) = (i + di, j + dj);
            if irange.contains(&i2)
                && jrange.contains(&j2)
                && grid[i2 as usize][j2 as usize] == Some(u64::MAX)
            {
                distance += 1;
                grid[i2 as usize][j2 as usize] = Some(distance);
                (i, j) = (i2, j2);
                path.push((i, j));
                break;
            }
        }
    }

    let response = path
        .par_iter()
        .enumerate()
        .map(|(d1, &(i1, j1))| {
            path.par_iter()
                .enumerate()
                .skip(d1 + 100 + 2)
                .filter(|(d2, &(i2, j2))| {
                    let cheat_size = ((i1 - i2).abs() + (j1 - j2).abs()) as usize;
                    cheat_size <= 20 && d2 - d1 - cheat_size >= 100
                })
                .count() as u64
        })
        .sum();

    Some(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
    //
    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
