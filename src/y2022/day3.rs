use crate::utils;

pub fn part1() {
    let input: Vec<String> = utils::read_lines("inputs/2022/day3.txt");
    let result: u32 = solve_part1(&input);
    println!("2022 :: Day 3 :: Part 1: {}", result);
}

pub fn part2() {
    let input: Vec<String> = utils::read_lines("inputs/2022/day3.txt");
    let result: u32 = solve_part2(&input);
    println!("2022 :: Day 3 :: Part 2: {}", result);
}

fn solve_part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| {
            let (comp1, comp2) = line.split_at(line.len() / 2);
            let common_char = comp1
                .chars()
                .find(|&c| comp2.contains(c))
                .expect("No shared characters found");

            get_priority(common_char)
        })
        .sum()
}

fn solve_part2(input: &[String]) -> u32 {
    input
        .chunks(3)
        .map(|elf_group| {
            elf_group[0]
                .chars()
                .find(|&c| elf_group[1].contains(c) && elf_group[2].contains(c))
                .map(get_priority)
                .expect("No common badge found")
        })
        .sum()
}

// Helper functions
fn get_priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => unreachable!(),
    }
}
