use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::cmp::{Ordering, Reverse};
use std::collections::HashSet;

advent_of_code::solution!(16);

type Position = (isize, isize);
const DIRS: [Position; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn parse_input(input: &str) -> (Vec<Vec<u8>>, Position, Position) {
    let mut s: Position = (0, 0);
    let mut e: Position = (0, 0);

    let grid = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(j, &char)| {
                    if char == b'S' {
                        s = (i as isize, j as isize);
                    } else if char == b'E' {
                        e = (i as isize, j as isize);
                    }
                    char
                })
                .collect_vec()
        })
        .collect_vec();

    (grid, s, e)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (grid, start, end) = parse_input(input);

    let height = grid.len();
    let width = grid[0].len();
    let size = height * width;

    let width = width as isize;

    let mut visited = vec![0_u8; size];

    let mut pq: PriorityQueue<(Position, usize), Reverse<u64>> = PriorityQueue::new();
    pq.push((start, 0_usize), Reverse(0));

    while let Some(((pos, dir), Reverse(dist))) = pq.pop() {
        if pos == end {
            return Some(dist);
        }

        let (i, j) = pos;

        let m = 1 << dir;
        let idx = (i * width + j) as usize;
        if visited[idx] & m != 0 {
            continue;
        }
        visited[idx] |= m;

        let (di, dj) = DIRS[dir];
        let (i2, j2) = (i + di, j + dj);
        if grid[i2 as usize][j2 as usize] != b'#' {
            pq.push_increase(((i2, j2), dir), Reverse(dist + 1));
        }

        for d in [(dir + 1) % 4, (dir + 3) % 4] {
            let (di, dj) = DIRS[d];
            let (i2, j2) = (i + di, j + dj);
            if grid[i2 as usize][j2 as usize] != b'#' {
                pq.push_increase(((i2, j2), d), Reverse(dist + 1001));
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let (grid, start, end) = parse_input(input);

    let height = grid.len();
    let width = grid[0].len();
    let size = height * width;

    let width = width as isize;

    let mut visited = vec![(u64::MAX, Vec::new()); size * 4];

    let mut pq = PriorityQueue::new();
    pq.push((start, 0_usize, None), Reverse(0));

    let mut end_reach = (u64::MAX, HashSet::new());

    while let Some(((pos, dir, from), Reverse(dist))) = pq.pop() {
        if pos == end {
            match dist.cmp(&end_reach.0) {
                Ordering::Greater => break,
                Ordering::Equal => {
                    end_reach.1.insert(dir);
                }
                Ordering::Less => {
                    end_reach.0 = dist;
                    end_reach.1 = HashSet::from([dir]);
                }
            }
        }

        let (i, j) = pos;

        let idx = ((i * width + j) as usize) << 2 | dir;
        let entry = &mut visited[idx];

        match dist.cmp(&entry.0) {
            Ordering::Greater => continue,
            Ordering::Equal => {
                if let Some(source) = from {
                    entry.1.push(source);
                }
            }
            Ordering::Less => {
                entry.0 = dist;
                if let Some(source) = from {
                    entry.1 = vec![source];
                } else {
                    entry.1 = Vec::new();
                }
            }
        }

        let (di, dj) = DIRS[dir];
        let (i2, j2) = (i + di, j + dj);
        if grid[i2 as usize][j2 as usize] != b'#' {
            pq.push_increase(((i2, j2), dir, Some((pos, dir))), Reverse(dist + 1));
        }

        for d in [(dir + 1) % 4, (dir + 3) % 4] {
            let (di, dj) = DIRS[d];
            let (i2, j2) = (i + di, j + dj);
            if grid[i2 as usize][j2 as usize] != b'#' {
                pq.push_increase(((i2, j2), d, Some((pos, dir))), Reverse(dist + 1001));
            }
        }
    }

    let mut spaces = HashSet::new();
    let mut queue = end_reach.1.iter().map(|&dir| (end, dir)).collect_vec();

    while let Some((pos, dir)) = queue.pop() {
        spaces.insert(pos);
        let (i, j) = pos;
        let idx = ((i * width + j) as usize) << 2 | dir;
        let sources = &visited[idx].1;
        for &source in sources {
            queue.push(source);
        }
    }

    Some(spaces.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
