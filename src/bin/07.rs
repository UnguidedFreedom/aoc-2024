use itertools::Itertools;

advent_of_code::solution!(7);

fn doable_one<I>(target: u64, first: u64, mut iter: I) -> bool
where
    I: Iterator<Item = u64> + Clone,
{
    match iter.next() {
        None => target == first,
        Some(val) => {
            if target % val == 0 && doable_one(target / val, first, iter.clone()) {
                true
            } else if val > target {
                false
            } else {
                doable_one(target - val, first, iter)
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .filter_map(|line| {
            let (target, data) = line.split(": ").collect_tuple().unwrap();

            let target = target.parse::<u64>().unwrap();
            let mut iter = data.split_whitespace().map(|s| s.parse::<u64>().unwrap());

            let first = iter.next().unwrap();

            doable_one(target, first, iter.rev()).then_some(target)
        })
        .sum::<u64>();

    Some(result)
}

fn doable_two<I>(target: u64, first: u64, mut iter: I) -> bool
where
    I: Iterator<Item = (u64, u64)> + Clone,
{
    match iter.next() {
        None => target == first,
        Some((div, val)) => {
            if (target % val == 0 && doable_two(target / val, first, iter.clone()))
                || (target % div == val && doable_two(target / div, first, iter.clone()))
            {
                true
            } else if val > target {
                false
            } else {
                doable_two(target - val, first, iter)
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .filter_map(|line| {
            let (target, data) = line.split(": ").collect_tuple().unwrap();

            let target = target.parse::<u64>().unwrap();
            let mut iter = data
                .split_whitespace()
                .map(|s| (10u64.pow(s.len() as u32), s.parse::<u64>().unwrap()));

            let (_, first) = iter.next().unwrap();

            doable_two(target, first, iter.rev()).then_some(target)
        })
        .sum::<u64>();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
