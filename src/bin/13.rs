use text_io::scan;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u64> {
    let mut response: u64 = 0;

    for chunk in input.split("\n\n") {
        let (xa, ya, xb, yb, xt, yt): (i64, i64, i64, i64, i64, i64);
        scan!(chunk.bytes() => "Button A: X+{}, Y+{}\nButton B: X+{}, Y+{}\nPrize: X={}, Y={}", xa, ya, xb, yb, xt, yt);

        let div = xb * ya - xa * yb;
        if div == 0 {
            continue;
        }

        let rem = ya * xt - xa * yt;
        if rem % div != 0 {
            continue;
        }

        let b = rem / div;
        if b < 0 {
            continue;
        }

        let rem = xt - xb * b;
        if rem % xa != 0 {
            continue;
        }
        let a = rem / xa;
        if a < 0 {
            continue;
        }

        response += (3 * a + b) as u64;
    }

    Some(response)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut response: u64 = 0;

    for chunk in input.split("\n\n") {
        let (xa, ya, xb, yb, xt, yt): (i64, i64, i64, i64, i64, i64);
        scan!(chunk.bytes() => "Button A: X+{}, Y+{}\nButton B: X+{}, Y+{}\nPrize: X={}, Y={}", xa, ya, xb, yb, xt, yt);

        let (xt, yt) = (xt + 10000000000000, yt + 10000000000000);

        let div = xb * ya - xa * yb;
        if div == 0 {
            continue;
        }

        let rem = ya * xt - xa * yt;
        if rem % div != 0 {
            continue;
        }

        let b = rem / div;
        if b < 0 {
            continue;
        }

        let rem = xt - xb * b;
        if rem % xa != 0 {
            continue;
        }
        let a = rem / xa;
        if a < 0 {
            continue;
        }

        response += (3 * a + b) as u64;
    }

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
