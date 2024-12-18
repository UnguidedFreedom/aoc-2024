use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

advent_of_code::solution!(18);

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn shortest_path(obstacles: &[bool], dimension: i32) -> Option<u64> {
    let end = (dimension - 1, dimension - 1);
    let range = 0..dimension;

    let mut candidates = PriorityQueue::new();
    candidates.push((0, 0), Reverse(0));

    let mut visited = vec![false; (dimension * dimension) as usize];

    while let Some((coords, Reverse(dist))) = candidates.pop() {
        if coords == end {
            return Some(dist);
        }

        let (i, j) = coords;
        let idx = (i * dimension + j) as usize;

        if visited[idx] {
            continue;
        }
        visited[idx] = true;

        for (di, dj) in DIRS {
            let new_coords = (i + di, j + dj);
            let (i2, j2) = new_coords;
            let idx2 = (i2 * dimension + j2) as usize;
            if !(range.contains(&i2) && range.contains(&j2)) || obstacles[idx2] || visited[idx2] {
                continue;
            }
            candidates.push_increase(new_coords, Reverse(dist + 1));
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u64> {
    #[cfg(not(test))]
    let dimension = 71;
    #[cfg(test)]
    let dimension = 7;

    #[cfg(not(test))]
    let to_take = 1024;
    #[cfg(test)]
    let to_take = 12;

    let mut obstacles = vec![false; (dimension * dimension) as usize];

    for line in input.lines().take(to_take) {
        let (i, j) = line
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();

        let idx = (i * dimension + j) as usize;
        obstacles[idx] = true;
    }

    shortest_path(&obstacles, dimension)
}

pub fn part_two(input: &str) -> Option<String> {
    #[cfg(not(test))]
    let dimension = 71;
    #[cfg(test)]
    let dimension = 7;

    #[cfg(not(test))]
    let to_take = 1024;
    #[cfg(test)]
    let to_take = 12;

    let mut obstacles = vec![false; (dimension * dimension) as usize];

    for (k, line) in input.lines().enumerate() {
        let (i, j) = line
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();

        let idx = (i * dimension + j) as usize;
        obstacles[idx] = true;

        if k > to_take && shortest_path(&obstacles, dimension).is_none() {
            return Some(line.to_string());
        }
    }

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
        assert_eq!(result, Some("6,1".to_string()));
    }
}
