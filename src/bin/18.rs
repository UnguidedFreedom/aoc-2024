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

    while let Some(((i, j), Reverse(dist))) = candidates.pop() {
        if (i, j) == end {
            return Some(dist);
        }

        visited[(i * dimension + j) as usize] = true;

        for (di, dj) in DIRS {
            let (i2, j2) = (i + di, j + dj);
            let idx2 = (i2 * dimension + j2) as usize;
            if !(range.contains(&i2) && range.contains(&j2)) || obstacles[idx2] || visited[idx2] {
                continue;
            }
            candidates.push_increase((i2, j2), Reverse(dist + 1));
        }
    }

    None
}

fn set_obstacle(input: &str, obstacles: &mut [bool], dimension: i32) {
    let (i, j) = input
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap();

    let idx = (i * dimension + j) as usize;
    obstacles[idx] = true;
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
        set_obstacle(line, &mut obstacles, dimension);
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

    let lines = input.lines().collect_vec();

    let mut a = to_take - 1; // highest known safe
    let mut b = lines.len() - 1; // lowest known unsafe

    let mut min_obstacles = vec![false; (dimension * dimension) as usize];

    for &line in lines.iter().take(to_take) {
        set_obstacle(line, &mut min_obstacles, dimension);
    }

    while a + 1 < b {
        let mid = (a + b) / 2;
        let mut obstacles = min_obstacles.clone();
        for &line in lines.iter().take(mid + 1).skip(a) {
            set_obstacle(line, &mut obstacles, dimension);
        }

        if shortest_path(&obstacles, dimension).is_some() {
            a = mid;
            min_obstacles = obstacles;
        } else {
            b = mid;
        }
    }

    Some(lines[b].to_string())
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
