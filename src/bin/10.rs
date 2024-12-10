use itertools::Itertools;
use rayon::prelude::*;
use std::ops::Range;

advent_of_code::solution!(10);

const DIRS: &[(isize, isize); 4] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];

fn trailheads_score(
    grid: &Vec<Vec<u8>>,
    (irange, jrange): (&Range<isize>, &Range<isize>),
    (i, j): (isize, isize),
    val: u8,
) -> Vec<(isize, isize)> {
    if !(irange.contains(&i) && jrange.contains(&j)) || grid[i as usize][j as usize] != val {
        return vec![];
    }

    if val == 9 {
        return vec![(i, j)];
    }

    DIRS.iter()
        .flat_map(|(di, dj)| trailheads_score(grid, (irange, jrange), (i + di, j + dj), val + 1))
        .collect::<Vec<(isize, isize)>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut starts: Vec<(isize, isize)> = Vec::new();

    let grid = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(j, char)| {
                    let val = char - b'0';
                    if val == 0 {
                        starts.push((i as isize, j as isize));
                    }
                    val
                })
                .collect_vec()
        })
        .collect_vec();

    let irange = 0..grid.len() as isize;
    let jrange = 0..grid[0].len() as isize;

    let response: u32 = starts
        .par_iter()
        .map(|&c| {
            trailheads_score(&grid, (&irange, &jrange), c, 0)
                .iter()
                .unique()
                .count() as u32
        })
        .sum::<u32>();

    Some(response)
}

fn trailheads_rating(
    grid: &Vec<Vec<u8>>,
    (irange, jrange): (&Range<isize>, &Range<isize>),
    (i, j): (isize, isize),
    val: u8,
) -> u32 {
    if !(irange.contains(&i) && jrange.contains(&j)) || grid[i as usize][j as usize] != val {
        return 0;
    }

    if val == 9 {
        return 1;
    }

    DIRS.iter()
        .map(|&(di, dj)| trailheads_rating(grid, (irange, jrange), (i + di, j + dj), val + 1))
        .sum::<u32>()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut starts: Vec<(isize, isize)> = Vec::new();

    let grid = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(j, char)| {
                    let val = char - b'0';
                    if val == 0 {
                        starts.push((i as isize, j as isize));
                    }
                    val
                })
                .collect_vec()
        })
        .collect_vec();

    let irange = 0..grid.len() as isize;
    let jrange = 0..grid[0].len() as isize;

    let response: u32 = starts
        .par_iter()
        .map(|&c| trailheads_rating(&grid, (&irange, &jrange), c, 0))
        .sum::<u32>();

    Some(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
