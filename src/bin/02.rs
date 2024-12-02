#![feature(iter_map_windows)]

advent_of_code::solution!(2);

fn is_report_safe<'a, I>(report: &'a I) -> bool
where
    I: Iterator<Item = i32> + 'a + Clone,
{
    let result = report
        .clone()
        .map_windows(|[a, b]| *b - *a)
        .map_windows(|[a, b]| {
            (1..=3_i32).contains(&a.abs())
                && (1..=3_i32).contains(&b.abs())
                && a.signum() == b.signum()
        })
        .all(|x| x);
    result
}

fn parse_report<'a>(report: &'a str) -> impl Iterator<Item = i32> + Clone + 'a {
    report.split_whitespace().map(|s| s.parse::<i32>().unwrap())
}

pub fn part_one(input: &str) -> Option<u32> {
    let answer = input
        .lines()
        .map(|line| parse_report(line))
        .filter(|report| is_report_safe(report))
        .count();
    Some(answer as u32)
}

fn report_can_be_safe<'a, I>(report: &I) -> bool
where
    I: Iterator<Item = i32> + Clone + 'a,
{
    if is_report_safe(report) {
        return true;
    }

    report
        .clone()
        .enumerate()
        .map(|(i, _)| {
            report
                .clone()
                .enumerate()
                .filter_map(move |(j, val)| {
                    if i != j {
                        return Some(val);
                    }
                    None
                })
                .into_iter()
        })
        .any(|part| is_report_safe(&part))
}

pub fn part_two(input: &str) -> Option<u32> {
    let answer = input
        .lines()
        .map(|line| parse_report(line))
        .filter(|report| report_can_be_safe(report))
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
