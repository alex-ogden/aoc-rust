use crate::utils;

pub fn part1() {
    let input: Vec<String> = utils::read_lines("inputs/2022/day1.txt");
    let result = solve_part1(&input);
    println!("2022 :: Day 1 :: Part 1: {}", result);
}
pub fn part2() {}

fn solve_part1(input: &[String]) -> u32 {
    // A vector contianing the total calories per elf
    let mut elf_calories: Vec<u32> = vec![0];
    let mut current_elf: u32 = 0;

    // Make sure we don't increment the elf twice if we hit 2 blank lines

    for line in input {
        // When we hit a blank line, increment the current elf number and add a new
        // item to the vector starting at 0 calories
        if line == "" {
            current_elf += 1;
            elf_calories.push(0);
            continue;
        }
        // Add each following line as a u32 to the total held at elf_calories[current_elf]
        elf_calories[current_elf as usize] += line.parse::<u32>().unwrap();
    }

    // Return the highest number in the vector
    elf_calories.into_iter().max().unwrap()
}

fn solve_part2() {}
