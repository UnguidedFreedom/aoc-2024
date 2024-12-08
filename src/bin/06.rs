use std::collections::HashSet;

use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(6);

const DIRS: &[(isize, isize); 4] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

enum Spot {
    Obstacle(usize),
    Clear,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = input.lines().map(|s| s.as_bytes().to_vec()).collect_vec();

    let height = grid.len();
    let width = grid[0].len();

    let irange = 0..height as isize;
    let jrange = 0..width as isize;

    let mut result: u32 = 1;

    let (mut i, mut j): (isize, isize) = (0, 0);
    'outer: for (starti, row) in grid.iter().enumerate() {
        for (startj, char) in row.iter().enumerate() {
            if *char == b'^' {
                (i, j) = (starti as isize, startj as isize);
                break 'outer;
            }
        }
    }

    let mut dirs = DIRS.iter().cycle();
    let (mut di, mut dj) = dirs.next().unwrap();

    loop {
        let (i2, j2) = (i + di, j + dj);
        if !(irange.contains(&i2) && jrange.contains(&j2)) {
            return Some(result);
        }
        let (i2u, j2u) = (i2 as usize, j2 as usize);
        match grid[i2u][j2u] {
            b'#' => {
                (di, dj) = *dirs.next().unwrap();
            }
            b'.' => {
                (i, j) = (i2, j2);
                grid[i2u][j2u] = b'X';
                result += 1;
            }
            _ => {
                (i, j) = (i2, j2);
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut n: usize = 0;
    let (mut si, mut sj): (isize, isize) = (-1, -1);

    let grid = input
        .lines()
        .enumerate()
        .map(|(i, s)| {
            s.as_bytes()
                .iter()
                .enumerate()
                .map(|(j, &c)| {
                    if c == b'#' {
                        n += 1;
                        Spot::Obstacle(n - 1)
                    } else {
                        if c == b'^' {
                            (si, sj) = (i as isize, j as isize);
                        }
                        Spot::Clear
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    let height = grid.len();
    let width = grid[0].len();

    let irange = 0..height as isize;
    let jrange = 0..width as isize;

    let mut dirs = DIRS.iter().cycle();
    let (mut di, mut dj) = dirs.next().unwrap();

    let (mut i, mut j) = (si, sj);

    let mut path: HashSet<(isize, isize)> = HashSet::new();

    loop {
        let (i2, j2) = (i + di, j + dj);
        if !irange.contains(&i2) || !jrange.contains(&j2) {
            break;
        }
        match grid[i2 as usize][j2 as usize] {
            Spot::Obstacle(_) => {
                (di, dj) = *dirs.next().unwrap();
            }
            Spot::Clear => {
                path.insert((i2, j2));
                (i, j) = (i2, j2);
            }
        }
    }

    let result = path
        .par_iter()
        .filter(|&(i_obs, j_obs)| {
            let mut dirs_enum = DIRS
                .iter()
                .enumerate()
                .map(|(dir, &pos)| ((1 << dir) as u8, pos))
                .cycle();

            let (mut i, mut j) = (si, sj);
            let (mut dir, (mut di, mut dj)) = dirs_enum.next().unwrap();

            let mut visited = vec![0_u8; n + 1];

            loop {
                let (i2, j2) = (i + di, j + dj);
                if !irange.contains(&i2) || !jrange.contains(&j2) {
                    return false;
                }

                let idx: usize;
                if (i2, j2) == (*i_obs, *j_obs) {
                    idx = n;
                } else if let Spot::Obstacle(i) = grid[i2 as usize][j2 as usize] {
                    idx = i;
                } else {
                    (i, j) = (i2, j2);
                    continue;
                }

                if visited[idx] & dir != 0 {
                    return true;
                }
                visited[idx] |= dir;

                (dir, (di, dj)) = dirs_enum.next().unwrap();
            }
        })
        .count() as u32;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
