advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    const MAS: &[u8] = "MAS".as_bytes();
    const DIRS: &[(i32, i32); 8] = &[
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let data = input.lines().map(|s| s.as_bytes()).collect::<Vec<&[u8]>>();

    let irange = 0..data.len() as i32;
    let jrange = 0..data[0].len() as i32;

    let result = data
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let i = i as i32;
            line.iter()
                .enumerate()
                .map(|(j, &char)| {
                    let j = j as i32;
                    if char != b'X' {
                        return 0;
                    }

                    DIRS.iter()
                        .filter(|&(idir, jdir)| {
                            MAS.iter().enumerate().all(|(k, &char)| {
                                let offset = k as i32 + 1;
                                let i2 = i + offset * idir;
                                let j2 = j + offset * jdir;

                                irange.contains(&i2)
                                    && jrange.contains(&j2)
                                    && data[i2 as usize][j2 as usize] == char
                            })
                        })
                        .count() as u32
                })
                .sum::<u32>()
        })
        .sum();

    Some(result)
}

fn is_mas(pair: (u8, u8)) -> bool {
    pair == (b'M', b'S') || pair == (b'S', b'M')
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = input.lines().map(|s| s.as_bytes()).collect::<Vec<&[u8]>>();

    let height = data.len();
    let width = data[0].len();

    let result = (1..height - 1)
        .map(|i| {
            (1..width - 1)
                .filter(|&j| {
                    data[i][j] == b'A'
                        && is_mas((data[i - 1][j - 1], data[i + 1][j + 1]))
                        && is_mas((data[i - 1][j + 1], data[i + 1][j - 1]))
                })
                .count() as u32
        })
        .sum::<u32>();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
