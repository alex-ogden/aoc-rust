use crate::utils;

pub fn part1() {
    let input: String = utils::read_input(2015, 1, false);
    let result: i64 = solve_part1(&input);
    println!("2015 :: Day 1 :: Part 1: {}", result);
}
pub fn part2() {
    let input: String = utils::read_input(2015, 1, false);
    let result: u64 = solve_part2(&input);
    println!("2015 :: Day 1 :: Part 2: {}", result);
}

fn solve_part1(input: &String) -> i64 {
    let mut floor: i64 = 0;

    for instr in input.trim().chars() {
        match instr {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => unreachable!(),
        }
    }

    floor
}

fn solve_part2(input: &String) -> u64 {
    let mut floor: i64 = 0;

    for (idx, instr) in input.trim().chars().enumerate() {
        match instr {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => unreachable!(),
        }
        if floor < 0 {
            return idx as u64 + 1;
        }
    }

    // We should never get here
    10
}
