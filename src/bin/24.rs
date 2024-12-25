use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use text_io::scan;

advent_of_code::solution!(24);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let mut states = HashMap::new();

    let mut frontier = Vec::new();

    for line in (&mut lines).take_while(|l| !l.is_empty()) {
        let (name, val) = line.split(": ").collect_tuple().unwrap();
        let val = val.parse::<u8>().unwrap();
        let name = name.to_string();
        frontier.push(name.clone());
        states.insert(name, val);
    }

    let mut gates = HashMap::new();
    let mut deps: HashMap<String, HashSet<String>> = HashMap::new();
    let mut rev_deps: HashMap<String, HashSet<String>> = HashMap::new();

    for line in lines {
        let (a, op, b, dest): (String, String, String, String);
        scan!(line.bytes() => "{} {} {} -> {}", a, op, b, dest);

        deps.entry(a.clone()).or_default().insert(dest.clone());
        deps.entry(b.clone()).or_default().insert(dest.clone());

        let rev_dep = rev_deps.entry(dest.clone()).or_default();
        rev_dep.insert(a.clone());
        rev_dep.insert(b.clone());

        gates.insert(dest, (a, op, b));
    }

    while let Some(wire) = frontier.pop() {
        if let Some((a, op, b)) = gates.get(&wire) {
            let a_state = states[a];
            let b_state = states[b];
            let val = match op.as_str() {
                "AND" => a_state & b_state,
                "OR" => a_state | b_state,
                "XOR" => a_state ^ b_state,
                _ => continue,
            };
            states.insert(wire.clone(), val);
        }

        if let Some(dests) = deps.get(&wire) {
            for dest in dests {
                let rev_dep = rev_deps.get_mut(dest).unwrap();
                rev_dep.remove(&wire);
                if rev_dep.is_empty() {
                    frontier.push(dest.clone());
                }
            }
        }
    }

    let result: u64 = states
        .into_iter()
        .filter(|(key, _)| key.starts_with("z"))
        .sorted_by(|(k1, _), (k2, _)| k1.cmp(k2))
        .enumerate()
        .map(|(i, (_, val))| (val as u64) << i)
        .fold(0, |acc, x| acc | x);

    Some(result)
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
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
