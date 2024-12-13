use text_io::scan;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u64> {
    let response = input.split("\n\n").filter_map(|chunk| {
        let (xa, ya, xb, yb, xt, yt): (i64, i64, i64, i64, i64, i64);
        scan!(chunk.bytes() => "Button A: X+{}, Y+{}\nButton B: X+{}, Y+{}\nPrize: X={}, Y={}", xa, ya, xb, yb, xt, yt);

        let div = xb * ya - xa * yb;
        if div == 0 {
            return None;
        }

        let rem = ya * xt - xa * yt;
        if rem % div != 0 {
            return None;
        }

        let b = rem / div;
        if b < 0 {
            return None;
        }

        let rem = xt - xb * b;
        if rem % xa != 0 {
            return None;
        }
        let a = rem / xa;
        if a < 0 {
            return None;
        }

        Some((3 * a + b) as u64)
    }).sum::<u64>();

    Some(response)
}

const OFFSET: i64 = 10000000000000;

pub fn part_two(input: &str) -> Option<u64> {
    let response = input.split("\n\n").filter_map(|chunk| {
            let (xa, ya, xb, yb, xt, yt): (i64, i64, i64, i64, i64, i64);
            scan!(chunk.bytes() => "Button A: X+{}, Y+{}\nButton B: X+{}, Y+{}\nPrize: X={}, Y={}", xa, ya, xb, yb, xt, yt);
            let (xt, yt) = (xt + OFFSET, yt + OFFSET);

            let div = xb * ya - xa * yb;
            if div == 0 {
                return None;
            }

            let rem = ya * xt - xa * yt;
            if rem % div != 0 {
                return None;
            }

            let b = rem / div;
            if b < 0 {
                return None;
            }

            let rem = xt - xb * b;
            if rem % xa != 0 {
                return None;
            }
            let a = rem / xa;
            if a < 0 {
                return None;
            }

            Some((3 * a + b) as u64)
        }).sum::<u64>();

    Some(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
