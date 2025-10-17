use crate::utils;

pub fn part1() {
    let input: Vec<String> = utils::read_lines("inputs/2022/day1.txt");
    let result = solve_part1(&input);
    println!("2022 :: Day 1 :: Part 1: {}", result);
}

pub fn part2() {
    let input: Vec<String> = utils::read_lines("inputs/2022/day1.txt");
    let result = solve_part2(&input);
    println!("2022 :: Day 1 :: Part 1: {}", result);
}

fn solve_part1(input: &[String]) -> u32 {
    input
        .split(|line| line.is_empty())
        .map(|group| {
            group
                .iter()
                .filter_map(|line| line.parse::<u32>().ok())
                .sum()
        })
        .max()
        .unwrap_or(0)
}

fn solve_part2(input: &[String]) -> u32 {
    let mut total_cals: Vec<u32> = input
        .split(|line| line.is_empty())
        .map(|group| {
            group
                .iter()
                .filter_map(|line| line.parse::<u32>().ok())
                .sum()
        })
        .collect();

    total_cals.sort_unstable();
    total_cals.reverse();

    total_cals.iter().take(3).sum()
}
