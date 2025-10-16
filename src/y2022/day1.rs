use crate::utils;

pub fn part1() {
    let input: Vec<String> = utils::read_lines("inputs/2022/day1.txt");
    let result = solve_part1(&input);
    println!("2022 :: Day 1 :: Part 1: {}", result);
}
pub fn part2() {}

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
    /*
        Less idiomatic way of doing it

    let mut elf_calories: Vec<u32> = vec![0];
    let mut current_elf: u32 = 0;

    for line in input {
        if line == "" {
            current_elf += 1;
            elf_calories.push(0);
            continue;
        }
        elf_calories[current_elf as usize] += line.parse::<u32>().unwrap();
    }

    elf_calories.into_iter().max().unwrap()
    */
}

fn solve_part2() {}
