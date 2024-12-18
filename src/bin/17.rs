use itertools::Itertools;
use text_io::scan;

advent_of_code::solution!(17);

fn parse_input(input: &str) -> (u64, u64, u64, Vec<u64>) {
    let (a, b, c, program): (u64, u64, u64, String);

    scan!(input.bytes() => "Register A: {}\nRegister B: {}\nRegister C: {}\n\nProgram: {}", a, b, c, program);

    (
        a,
        b,
        c,
        program
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .collect_vec(),
    )
}

fn combo(operand: u64, a: u64, b: u64, c: u64) -> u64 {
    match operand {
        0..=3 => operand,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("invalid combo operand"),
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut a, mut b, mut c, program) = parse_input(input);

    let mut ic = 0;
    let mut outputs = Vec::new();

    while ic < program.len() {
        let (opcode, operand) = (program[ic], program[ic + 1]);
        match opcode {
            0 => a /= 1 << combo(operand, a, b, c),
            1 => b ^= operand,
            2 => b = combo(operand, a, b, c) % 8,
            3 => {
                if a != 0 {
                    ic = operand as usize;
                    continue;
                }
            }
            4 => b ^= c,
            5 => outputs.push(combo(operand, a, b, c) % 8),
            6 => b = a / (1 << combo(operand, a, b, c)),
            7 => c = a / (1 << combo(operand, a, b, c)),
            _ => panic!("invalid opcode"),
        };
        ic += 2;
    }

    let res = outputs.iter().map(|val| val.to_string()).join(",");

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, _, _, program) = parse_input(input);

    let mut possible_as = vec![0_u64];
    for &expected in program.iter().rev() {
        let mut new_possibles = Vec::new();
        for &old_a in &possible_as {
            for a_candidate in 0..8 {
                let a = old_a << 3 | a_candidate;
                let b = a_candidate ^ 5;
                let out = (b ^ 6 ^ (a / (1 << b))) % 8;

                if out == expected {
                    new_possibles.push(a);
                }
            }
        }
        possible_as = new_possibles;
    }

    possible_as.first().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(117440));
    // }
}
