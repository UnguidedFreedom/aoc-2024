use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;

advent_of_code::solution!(6);

const DIRS: &[(isize, isize); 4] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = input
        .lines()
        .map(|s| s.as_bytes().iter().copied().collect_vec())
        .collect_vec();

    let height = grid.len();
    let width = grid[0].len();

    let irange = 0..height as isize;
    let jrange = 0..width as isize;

    let mut result: u32 = 0;

    let (mut i, mut j): (isize, isize) = (0, 0);
    'outer: for (starti, row) in grid.iter().enumerate() {
        for (startj, char) in row.iter().enumerate() {
            if *char == b'^' {
                (i, j) = (starti as isize, startj as isize);
                // *char = b'X';
                result += 1;
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
    let mut grid = input
        .lines()
        .map(|s| {
            s.as_bytes()
                .iter()
                .map(|&c| if c == b'.' { u8::MAX } else { c })
                .collect_vec()
        })
        .collect_vec();

    let height = grid.len();
    let width = grid[0].len();

    let irange = 0..height as isize;
    let jrange = 0..width as isize;

    let (mut si, mut sj): (isize, isize) = (-1, -1);
    'outer: for (starti, row) in grid.iter().enumerate() {
        for (startj, &char) in row.iter().enumerate() {
            if char == b'^' {
                (si, sj) = (starti as isize, startj as isize);
                break 'outer;
            }
        }
    }

    let mut dirs = DIRS.iter().cycle();
    let (mut di, mut dj) = dirs.next().unwrap();

    let (mut i, mut j) = (si, sj);

    loop {
        let (i2, j2) = (i + di, j + dj);
        if !(irange.contains(&i2) && jrange.contains(&j2)) {
            break;
        }
        let (i2u, j2u) = (i2 as usize, j2 as usize);
        match grid[i2u][j2u] {
            b'#' => {
                (di, dj) = *dirs.next().unwrap();
            }
            u8::MAX => {
                (i, j) = (i2, j2);
                grid[i2u][j2u] = 0;
            }
            _ => {
                (i, j) = (i2, j2);
            }
        }
    }

    let result = grid
        .par_iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.par_iter()
                .enumerate()
                .filter_map(move |(j, &char)| (char == 0).then_some((i, j)))
        })
        .filter(|&(i_obs, j_obs)| {
            let mut visited: HashMap<(isize, isize), u8> = HashMap::new();

            let mut dirs_enum = DIRS
                .iter()
                .enumerate()
                .map(|(dir, &pos)| ((1 << dir) as u8, pos))
                .cycle();

            let (mut i, mut j) = (si, sj);
            let (mut dir, (mut di, mut dj)) = dirs_enum.next().unwrap();

            loop {
                let (i2, j2) = (i + di, j + dj);
                if !(irange.contains(&i2) && jrange.contains(&j2)) {
                    return false;
                }
                let (i2u, j2u) = (i2 as usize, j2 as usize);
                if grid[i2u][j2u] == b'#' || (i2u, j2u) == (i_obs, j_obs) {
                    let entry = visited.entry((i, j)).or_default();
                    if *entry & dir != 0 {
                        return true;
                    }
                    *entry |= dir;

                    (dir, (di, dj)) = dirs_enum.next().unwrap();

                    continue;
                }
                (i, j) = (i2, j2);
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
