use itertools::Itertools;
use std::ops::Range;

advent_of_code::solution!(12);

const DIRS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = input
        .lines()
        .map(|line| line.as_bytes().iter().map(|&x| (x, false)).collect_vec())
        .collect_vec();

    let mut response: u64 = 0;

    let height = grid.len();
    let width = grid[0].len();

    let irange = 0..height as isize;
    let jrange = 0..width as isize;

    for i in 0..height {
        for j in 0..width {
            if grid[i][j].1 {
                continue;
            }

            grid[i][j].1 = true;

            let mut candidates = vec![(i as isize, j as isize)];

            let mut perimeter = 0_u64;
            let mut area = 0_u64;

            let val = grid[i][j].0;

            while let Some((i, j)) = candidates.pop() {
                area += 1;
                for (di, dj) in DIRS {
                    let (i2, j2) = (i + di, j + dj);
                    if !(irange.contains(&i2) && jrange.contains(&j2)) {
                        perimeter += 1;
                        continue;
                    }

                    let contender = grid[i2 as usize][j2 as usize];
                    if contender.0 != val {
                        perimeter += 1;
                        continue;
                    }

                    if !contender.1 {
                        grid[i2 as usize][j2 as usize].1 = true;
                        candidates.push((i2, j2));
                    }
                }
            }

            response += area * perimeter;
        }
    }

    Some(response)
}

fn first_edge(
    grid: &[Vec<(u8, bool)>],
    (i, j): (isize, isize),
    val: u8,
    irange: &Range<isize>,
    jrange: &Range<isize>,
    idir: usize,
) -> bool {
    let (di, dj) = DIRS[(idir + 1) % 4];
    let (i2, j2) = (i + di, j + dj);
    if !(irange.contains(&i2) && jrange.contains(&j2)) || grid[i2 as usize][j2 as usize].0 != val {
        return true;
    }

    let (di, dj) = DIRS[idir];
    let (i2, j2) = (i2 + di, j2 + dj);

    if !(irange.contains(&i2) && jrange.contains(&j2)) || grid[i2 as usize][j2 as usize].0 != val {
        return false;
    }

    true
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = input
        .lines()
        .map(|line| line.as_bytes().iter().map(|&x| (x, false)).collect_vec())
        .collect_vec();

    let mut response: u64 = 0;

    let height = grid.len();
    let width = grid[0].len();

    let irange = 0..height as isize;
    let jrange = 0..width as isize;

    for i in 0..height {
        for j in 0..width {
            if grid[i][j].1 {
                continue;
            }

            grid[i][j].1 = true;

            let mut candidates = vec![(i as isize, j as isize)];

            let mut perimeter = 0_u64;
            let mut area = 0_u64;

            let val = grid[i][j].0;

            while let Some((i, j)) = candidates.pop() {
                area += 1;
                for (idir, (di, dj)) in DIRS.iter().enumerate() {
                    let (i2, j2) = (i + di, j + dj);
                    if !(irange.contains(&i2) && jrange.contains(&j2)) {
                        if first_edge(&grid, (i, j), val, &irange, &jrange, idir) {
                            perimeter += 1;
                        }
                        continue;
                    }

                    let contender = grid[i2 as usize][j2 as usize];
                    if contender.0 != val {
                        if first_edge(&grid, (i, j), val, &irange, &jrange, idir) {
                            perimeter += 1;
                        }
                        continue;
                    }

                    if !contender.1 {
                        grid[i2 as usize][j2 as usize].1 = true;
                        candidates.push((i2, j2));
                    }
                }
            }

            response += area * perimeter;
        }
    }

    Some(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
