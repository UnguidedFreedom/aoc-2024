use text_io::scan;

advent_of_code::solution!(14);

const P1_ITERS: i64 = 100;

pub fn part_one(input: &str) -> Option<u64> {
    let mut quadrants = [0_u64; 4];

    let (dimx, dimy) = (101, 103);
    #[cfg(test)]
    let (dimx, dimy) = (11, 7);

    let (midx, midy) = (dimx / 2, dimy / 2);

    for line in input.lines() {
        let (px, py, vx, vy): (i64, i64, i64, i64);
        scan!(line.bytes() => "p={},{} v={},{}", px, py, vx, vy);

        let (x, y) = (
            (px + P1_ITERS * vx).rem_euclid(dimx),
            (py + P1_ITERS * vy).rem_euclid(dimy),
        );

        if x == midx || y == midy {
            continue;
        }

        let quad: usize = (if x < midx { 0 } else { 1 }) | (if y < midy { 0 } else { 1 }) << 1;
        quadrants[quad] += 1;
    }

    let response = quadrants.iter().product();
    Some(response)
}

pub fn part_two(input: &str) -> Option<u64> {
    #[cfg(not(test))]
    let (dimx, dimy): (i64, i64) = (101, 103);
    #[cfg(test)]
    let (dimx, dimy): (i64, i64) = (11, 7);

    let mut guards = Vec::new();

    for line in input.lines() {
        let (px, py, vx, vy): (i64, i64, i64, i64);
        scan!(line.bytes() => "p={},{} v={},{}", px, py, vx, vy);

        guards.push(((px, py), (vx, vy)));
    }

    for iter in 1..100_000 {
        let mut grid = vec![vec![b' '; dimx as usize]; dimy as usize];
        let mut uniques = true;

        for (p, v) in guards.iter_mut() {
            *p = ((p.0 + v.0).rem_euclid(dimx), (p.1 + v.1).rem_euclid(dimy));
            if grid[p.1 as usize][p.0 as usize] != b' ' {
                uniques = false;
            } else {
                grid[p.1 as usize][p.0 as usize] = b'X';
            }
        }

        if uniques {
            let mut preview = "".to_string();
            let mut found = false;
            for row in grid {
                let r = String::from_utf8(row).unwrap();
                if r.contains("XXXXX") {
                    found = true;
                }
                preview += (r + "\n").as_str();
            }
            if found {
                println!("{preview}");
                return Some(iter);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
