use itertools::{Either, Itertools};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut values = input
        .trim()
        .as_bytes()
        .iter()
        .map(|&c| (c - b'0') as u64)
        .collect_vec();

    let count: u64 = values
        .iter()
        .enumerate()
        .filter_map(|(i, val)| (i % 2 == 0).then_some(val))
        .sum();

    let mut result: u64 = 0;
    let (mut i, mut j): (usize, usize) = (0, values.len() - 1);
    for idx in 0..count {
        while values[i] == 0 {
            i += 1;
        }

        while values[j] == 0 {
            j -= 2;
        }

        if i % 2 == 0 {
            result += idx * (i / 2) as u64;
            values[i] -= 1;
        } else {
            result += idx * (j / 2) as u64;
            values[i] -= 1;
            values[j] -= 1;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut so_far: u64 = 0;
    let (blocks, mut spaces): (Vec<_>, Vec<_>) = input
        .trim()
        .as_bytes()
        .iter()
        .map(|&c| (c - b'0') as u64)
        .enumerate()
        .partition_map(|(i, val)| {
            let idx = so_far;
            so_far += val;
            if i % 2 == 0 {
                Either::Left((idx, val))
            } else {
                Either::Right((idx, val))
            }
        });

    let mut response: u64 = 0;

    for (val, block) in blocks.iter().enumerate().rev() {
        let mut pos = block.0;
        for (idx, space) in spaces.iter().enumerate() {
            if space.0 > pos {
                break;
            }
            if space.1 >= block.1 {
                pos = space.0;
                if space.1 == block.1 {
                    spaces.remove(idx);
                } else {
                    spaces[idx] = (space.0 + block.1, space.1 - block.1);
                }
                break;
            }
        }

        for i in 0..block.1 {
            response += val as u64 * (pos + i);
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
