use itertools::Itertools;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u64> {
    let (first, second) = input.split("\n\n").collect_tuple().unwrap();

    let (mut i, mut j) = (0, 0);

    let mut grid = first
        .lines()
        .enumerate()
        .map(|(curr_i, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(curr_j, &char)| {
                    if char == b'@' {
                        (i, j) = (curr_i as isize, curr_j as isize);
                    }
                    char
                })
                .collect_vec()
        })
        .collect_vec();

    for mov in second.bytes() {
        let (di, dj): (isize, isize) = match mov {
            b'^' => (-1, 0),
            b'v' => (1, 0),
            b'<' => (0, -1),
            b'>' => (0, 1),
            _ => continue,
        };

        let (i2, j2) = (i + di, j + dj);
        let (mut i3, mut j3) = (i2, j2);

        while grid[i3 as usize][j3 as usize] == b'O' {
            (i3, j3) = (i3 + di, j3 + dj);
        }

        if grid[i3 as usize][j3 as usize] == b'#' {
            continue;
        }

        grid[i3 as usize][j3 as usize] = b'O';
        grid[i2 as usize][j2 as usize] = b'@';
        grid[i as usize][j as usize] = b'.';
        (i, j) = (i2, j2);
    }

    let mut result: u64 = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, &char) in row.iter().enumerate() {
            if char == b'O' {
                result += (i * 100 + j) as u64;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (first, second) = input.split("\n\n").collect_tuple().unwrap();

    let (mut i, mut j) = (0, 0);

    let mut grid = first
        .lines()
        .enumerate()
        .map(|(curr_i, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .flat_map(|(curr_j, &char)| match char {
                    b'@' => {
                        (i, j) = (curr_i as isize, 2 * curr_j as isize);
                        [b'@', b'.']
                    }
                    b'#' => [b'#', b'#'],
                    b'O' => [b'[', b']'],
                    _ => [b'.', b'.'],
                })
                .collect_vec()
        })
        .collect_vec();

    'outer: for mov in second.bytes() {
        if mov == b'<' || mov == b'>' {
            let row = &mut grid[i as usize];
            let (dj, comp): (isize, u8) = if mov == b'<' { (-1, b']') } else { (1, b'[') };
            let mut j2 = j + dj;
            while row[j2 as usize] == comp {
                j2 += 2 * dj;
            }
            if row[j2 as usize] == b'#' {
                continue;
            }
            row.remove(j2 as usize);
            row.insert(j as usize, b'.');
            j += dj;
        } else if mov == b'^' || mov == b'v' {
            let di: isize = if mov == b'^' { -1 } else { 1 };
            let mut i2 = i + di;
            let mut contenders = vec![match grid[i2 as usize][j as usize] {
                b'.' => {
                    grid[i2 as usize][j as usize] = b'@';
                    grid[i as usize][j as usize] = b'.';
                    i = i2;
                    continue;
                }
                b'[' => vec![j],
                b']' => vec![j - 1],
                _ => continue,
            }];

            loop {
                i2 += di;
                let mut latest = Vec::new();
                for &j_cont in contenders[contenders.len() - 1].iter() {
                    let above_l = grid[i2 as usize][j_cont as usize];
                    let above_r = grid[i2 as usize][j_cont as usize + 1];
                    if above_l == b'#' || above_r == b'#' {
                        continue 'outer;
                    }

                    if above_l == b'[' {
                        latest.push(j_cont);
                    } else if above_l == b']' {
                        latest.push(j_cont - 1);
                    }
                    if above_r == b'[' {
                        latest.push(j_cont + 1);
                    }
                }
                if latest.is_empty() {
                    break;
                }
                contenders.push(latest);
            }

            // if we reach this point, it's doable so actually move everything
            for (i_off, conts) in contenders.iter().enumerate().rev() {
                let i_row = (i + (i_off as isize + 1) * di) as usize;
                let target_row = (i + (i_off as isize + 2) * di) as usize;
                for &j_val in conts {
                    let j_val = j_val as usize;
                    grid[target_row][j_val] = b'[';
                    grid[target_row][j_val + 1] = b']';
                    grid[i_row][j_val] = b'.';
                    grid[i_row][j_val + 1] = b'.';
                }
            }
            grid[i as usize][j as usize] = b'.';
            grid[(i + di) as usize][j as usize] = b'@';
            i += di;
        }
    }

    let mut result: u64 = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, &char) in row.iter().enumerate() {
            if char == b'[' {
                result += (i * 100 + j) as u64;
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
