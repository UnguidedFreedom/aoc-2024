use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut deps = HashMap::<u32, HashSet<u32>>::new();
    let mut in_first = true;

    let mut response: u32 = 0;
    for line in input.lines() {
        if line.is_empty() {
            in_first = false;
            continue;
        }

        if in_first {
            let (a, b): (u32, u32) = line
                .split('|')
                .map(|s| s.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap();
            deps.entry(a).or_insert(HashSet::new()).insert(b);
            continue;
        }

        let values = line
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let mut visited = HashSet::<u32>::new();
        let mut valid = true;
        for &val in values.iter() {
            visited.insert(val);
            if let Some(required) = deps.get(&val) {
                if !required.is_disjoint(&visited) {
                    valid = false;
                    break;
                }
            }
        }

        if valid {
            response += values[values.len() / 2];
        }
    }

    Some(response)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut deps = HashMap::<u32, HashSet<u32>>::new();
    let mut in_first = true;

    let mut response: u32 = 0;
    for line in input.lines() {
        if line.is_empty() {
            in_first = false;
            continue;
        }

        if in_first {
            let (a, b): (u32, u32) = line
                .split('|')
                .map(|s| s.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap();
            deps.entry(b).or_insert(HashSet::new()).insert(a);
            continue;
        }

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

            if prereq.len() == 0 {
                contenders.push(val);
            }

            for &pre in prereq.iter() {
                update_deps_rev
                    .entry(pre)
                    .or_insert(HashSet::new())
                    .insert(val);
            }

            update_deps.insert(val, prereq);
        }

        if valid {
            continue;
        }

        let mut ordered: Vec<u32> = Vec::new();

        while contenders.len() > 0 {
            let val = contenders.pop().unwrap();
            ordered.push(val);

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

        response += ordered[ordered.len() / 2];
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
