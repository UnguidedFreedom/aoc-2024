use itertools::Itertools;
use std::collections::HashMap;
use text_io::scan;

advent_of_code::solution!(24);

fn solve(
    mut states: HashMap<String, u64>,
    gates: &HashMap<String, (String, String, String)>,
    deps: &HashMap<String, Vec<String>>,
) -> u64 {
    let mut frontier = states.keys().cloned().collect_vec();

    let mut rev_deps: HashMap<String, u8> = HashMap::new();

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
                let entry = rev_deps.entry(dest.clone()).or_default();
                *entry += 1;

                if *entry == 2 {
                    frontier.push(dest.clone());
                }
            }
        }
    }

    states
        .into_iter()
        .filter_map(|(key, val)| {
            if let Some(i) = key.strip_prefix("z") {
                let i = i.parse::<u32>().unwrap();
                Some(val << i)
            } else {
                None
            }
        })
        .fold(0, |acc, x| acc | x)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let mut states = HashMap::new();

    for line in (&mut lines).take_while(|l| !l.is_empty()) {
        let (name, val) = line.split(": ").collect_tuple().unwrap();
        let val = val.parse::<u64>().unwrap();
        let name = name.to_string();
        states.insert(name, val);
    }

    let mut gates = HashMap::new();
    let mut deps: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let (a, op, b, dest): (String, String, String, String);
        scan!(line.bytes() => "{} {} {} -> {}", a, op, b, dest);

        deps.entry(a.clone()).or_default().push(dest.clone());
        deps.entry(b.clone()).or_default().push(dest.clone());

        gates.insert(dest, (a, op, b));
    }

    let result = solve(states, &gates, &deps);

    Some(result)
}

fn check(
    i: usize,
    x: u64,
    y: u64,
    states_ref: &HashMap<String, u64>,
    gates: &HashMap<String, (String, String, String)>,
    deps: &HashMap<String, Vec<String>>,
) -> bool {
    let mut states = states_ref.clone();
    let x_name = format!("x{i:02}");
    let y_name = format!("y{i:02}");

    *states.get_mut(&x_name).unwrap() = x;
    *states.get_mut(&y_name).unwrap() = y;

    let res = solve(states, gates, deps);
    let expected = (x + y) << i;

    res == expected
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let mut states = HashMap::new();

    for line in (&mut lines).take_while(|l| !l.is_empty()) {
        let (name, _) = line.split(": ").collect_tuple().unwrap();
        let name = name.to_string();
        states.insert(name, 0_u64);
    }

    let mut gates = HashMap::new();
    let mut deps: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let (a, op, b, dest): (String, String, String, String);
        scan!(line.bytes() => "{} {} {} -> {}", a, op, b, dest);

        deps.entry(a.clone()).or_default().push(dest.clone());
        deps.entry(b.clone()).or_default().push(dest.clone());

        gates.insert(dest, (a, op, b));
    }

    let n = states.len() / 2;

    for i in 0..n {
        if !check(i, 0, 1, &states, &gates, &deps) {
            println!("{i} didn't yield expected");
        }
    }

    // From here was finished manually

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
