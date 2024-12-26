advent_of_code::solution!(25);

use rayon::prelude::*;

pub fn part_one(input: &str) -> Option<u64> {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    input.split("\n\n").for_each(|grid| {
        let is_lock = grid.starts_with("#");
        let mut data = [if is_lock { 0 } else { 5 }; 5];

        grid.lines().skip(1).take(5).for_each(|row| {
            row.bytes().enumerate().for_each(|(i, v)| {
                if v == b'#' {
                    if is_lock {
                        data[i] += 1;
                    } else {
                        data[i] -= 1;
                    }
                }
            })
        });

        if is_lock {
            locks.push(data);
        } else {
            keys.push(data);
        }
    });

    let result = locks
        .par_iter()
        .map(|lock| {
            keys.par_iter()
                .filter(|&key| key.iter().zip(lock).all(|(k, l)| l <= k))
                .count() as u64
        })
        .sum();

    Some(result)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
