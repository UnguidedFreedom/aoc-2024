use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines_iter = input.lines();

    let deps = lines_iter
        .by_ref()
        .take_while(|s| !s.is_empty())
        .map(|l| {
            l.split('|')
                .map(|s| s.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .into_grouping_map()
        .collect::<HashSet<_>>();

    let response: u32 = lines_iter
        .filter_map(|line| {
            let mut vals = line.split(',').map(|s| s.parse::<u32>().unwrap());

            let mut visited = HashSet::<u32>::new();

            for val in vals.clone() {
                if let Some(required) = deps.get(&val) {
                    if !required.is_disjoint(&visited) {
                        return None;
                    }
                }
                visited.insert(val);
            }

            vals.nth(visited.len() / 2)
        })
        .sum();

    Some(response)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut response: u32 = 0;

    let mut lines_iter = input.lines();

    let deps = lines_iter
        .by_ref()
        .take_while(|s| !s.is_empty())
        .map(|l| {
            l.split('|')
                .map(|s| s.parse::<u32>().unwrap())
                .rev()
                .collect_tuple()
                .unwrap()
        })
        .into_grouping_map()
        .collect::<HashSet<_>>();

    for line in lines_iter {
        let values = line
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let values_set: HashSet<u32> = HashSet::from_iter(values.iter().copied());
        let mut visited = HashSet::<u32>::new();
        let mut update_deps: HashMap<u32, HashSet<u32>> = HashMap::new();
        let mut update_deps_rev: HashMap<u32, HashSet<u32>> = HashMap::new();
        let mut valid = true;
        let mut contenders: Vec<u32> = Vec::new();

        for &val in values.iter() {
            visited.insert(val);

            let prereq: HashSet<_> = deps
                .get(&val)
                .unwrap_or(&HashSet::<u32>::new())
                .intersection(&values_set)
                .copied()
                .collect();

            if !prereq.is_subset(&visited) {
                valid = false;
            }

            if prereq.is_empty() {
                contenders.push(val);
            }

            for &pre in prereq.iter() {
                update_deps_rev.entry(pre).or_default().insert(val);
            }

            update_deps.insert(val, prereq);
        }

        if valid {
            continue;
        }

        let mut i = 0;
        let target = values.len() / 2;

        while let Some(val) = contenders.pop() {
            if i == target {
                response += val;
                break;
            }
            i += 1;

            if let Some(unblocks) = update_deps_rev.get(&val) {
                for &unblock in unblocks.iter() {
                    let d = update_deps.get_mut(&unblock).unwrap();
                    d.remove(&val);
                    if d.is_empty() {
                        contenders.push(unblock);
                    }
                }
            }
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
