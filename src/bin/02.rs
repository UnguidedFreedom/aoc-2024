#![feature(iter_map_windows)]

advent_of_code::solution!(2);

fn is_report_safe<'a, I>(report: &'a I) -> bool
where
    I: Iterator<Item = i32> + Clone + 'a,
{
    let diffs = report.clone().map_windows(|&[a, b]| b - a);

    diffs.clone().all(|x| (1..=3).contains(&x.abs()))
        && diffs
            .map_windows(|[a, b]| a.signum() == b.signum())
            .all(|x| x)
}

fn parse_report(report: &str) -> impl Iterator<Item = i32> + Clone + '_ {
    report.split_whitespace().map(|s| s.parse::<i32>().unwrap())
}

pub fn part_one(input: &str) -> Option<u32> {
    let answer = input
        .lines()
        .map(parse_report)
        .filter(|report| is_report_safe(report))
        .count();
    Some(answer as u32)
}

fn report_can_be_safe<'a, I>(report: &'a I) -> bool
where
    I: Iterator<Item = i32> + Clone + 'a,
{
    if is_report_safe(report) {
        return true;
    }

    let report = &report.clone().collect::<Vec<i32>>();

    (0..report.len()).any(|i| {
        is_report_safe(
            &report
                .iter()
                .enumerate()
                .filter_map(|(j, &val)| (i != j).then_some(val)),
        )
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    let answer = input
        .lines()
        .map(parse_report)
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
