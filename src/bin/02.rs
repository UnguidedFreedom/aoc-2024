#![feature(iter_map_windows)]
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

advent_of_code::solution!(2);

fn is_report_safe<I>(report: I) -> bool
where
    I: Iterator<Item = i32> + Clone,
{
    !report
        .map_windows(|&[a, b]| b - a)
        .fold_while(None, |state, x| {
            if !(1..=3).contains(&x.abs()) {
                return Done(state);
            }
            match state {
                Some(signum) if signum == x.signum() => Continue(state),
                Some(_) => Done(state),
                None => Continue(Some(x.signum())),
            }
        })
        .is_done()
}

fn parse_report(report: &str) -> impl Iterator<Item = i32> + Clone + '_ {
    report.split_whitespace().map(|s| s.parse::<i32>().unwrap())
}

pub fn part_one(input: &str) -> Option<u32> {
    let answer = input
        .lines()
        .filter(|&report| is_report_safe(parse_report(report)))
        .count();
    Some(answer as u32)
}

fn report_can_be_safe<I>(report: I) -> bool
where
    I: Iterator<Item = i32> + Clone,
{
    if is_report_safe(report.clone()) {
        return true;
    }

    let rep = report.collect::<Vec<i32>>();
    (0..rep.len()).any(|i| {
        is_report_safe(
            rep.iter()
                .enumerate()
                .filter_map(|(j, &val)| (i != j).then_some(val)),
        )
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    let answer = input
        .lines()
        .filter(|&report| report_can_be_safe(parse_report(report)))
        .count();
    Some(answer as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
