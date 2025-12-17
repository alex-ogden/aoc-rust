use crate::utils;

pub fn part1() {
    let input: String = utils::read_input(2015, 2, false);
    let result: u64 = solve_part1(&input);
    println!("2015 :: Day 2 :: Part 1: {}", result);
}
pub fn part2() {
    let input: String = utils::read_input(2015, 2, false);
    let result: u64 = solve_part2(&input);
    println!("2015 :: Day 2 :: Part 2: {}", result);
}

fn solve_part1(input: &String) -> u64 {
    input
        .lines()
        .map(|line| {
            let measurements: Vec<u64> = line
                .split("x")
                .map(|num_str| num_str.parse::<u64>().unwrap())
                .collect();
            let l: u64 = measurements[0];
            let w: u64 = measurements[1];
            let h: u64 = measurements[2];

            (2 * l * w) + (2 * w * h) + (2 * h * l) + (l * w).min((w * h).min(h * l))
        })
        .sum()
}

fn solve_part2(input: &String) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut measurements: Vec<u64> = line
                .split("x")
                .map(|num_str| num_str.parse::<u64>().unwrap())
                .collect();
            let l: u64 = measurements[0];
            let w: u64 = measurements[1];
            let h: u64 = measurements[2];

            measurements.sort_unstable();

            let (first_smallest, second_smallest): (u64, u64) = (measurements[0], measurements[1]);

            (first_smallest * 2) + (second_smallest * 2) + (l * w * h)
        })
        .sum()
}
