use crate::utils;

pub fn part1() {
    let input: Vec<String> = utils::read_lines("inputs/2025/day3.txt");
    let result: u32 = solve_part1(&input);
    println!("2025 :: Day 3 :: Part 1: {}", result);
}

fn solve_part1(input: &[String]) -> u32 {
    input.iter().map(|bank| {
        let bank_as_str: String = bank.to_string();

        let mut big_first: u8 = 0;
        let mut big_second: u8 = 0;

        for ch in bank_as_str.chars() {
            if ch.parse::<u8>().unwrap() > big_first {
                big_first = ch.parse::<u8>().unwrap();
            }
        }
    });
}
