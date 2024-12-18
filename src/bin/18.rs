use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashSet;

advent_of_code::solution!(18);

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn part_one(input: &str) -> Option<u64> {
    #[cfg(not(test))]
    let dimension = 71;
    #[cfg(test)]
    let dimension = 7;

    #[cfg(not(test))]
    let to_take = 1024;
    #[cfg(test)]
    let to_take = 12;

    let end = (dimension - 1, dimension - 1);
    let range = 0..dimension;

    let obstacles: HashSet<(i32, i32)> =
        HashSet::from_iter(input.lines().take(to_take).map(|line| {
            line.split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect_tuple::<(i32, i32)>()
                .unwrap()
        }));

    let mut candidates = PriorityQueue::new();
    candidates.push((0, 0), Reverse(0));

    let mut visited = HashSet::new();

    while let Some((coords, Reverse(dist))) = candidates.pop() {
        if coords == end {
            return Some(dist);
        }

        if visited.contains(&coords) {
            continue;
        }
        visited.insert(coords);

        let (i, j) = coords;

        for (di, dj) in DIRS {
            let new_coords = (i + di, j + dj);
            let (i2, j2) = new_coords;
            if !(range.contains(&i2) && range.contains(&j2))
                || obstacles.contains(&new_coords)
                || visited.contains(&new_coords)
            {
                continue;
            }
            candidates.push_increase(new_coords, Reverse(dist + 1));
        }
    }

    None
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
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
