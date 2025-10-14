use crate::utils;

pub fn part1() {
    let input_lines: Vec<String> = utils::read_lines("inputs/2019/day1.txt");
    let result = solve_part1(&input_lines);
    println!("2019 :: Day1 :: Part1: {}", result);
}

pub fn part2() {
    let input_lines: Vec<String> = utils::read_lines("inputs/2019/day1.txt");
    let result = solve_part2(&input_lines);
    println!("2019 :: Day1 :: Part2: {}", result);
}

fn solve_part1(lines: &[String]) -> i32 {
    lines
        .iter()
        .filter_map(|line| line.parse::<i32>().ok()) // convert string to int32, skip failures
        .map(|mass| mass / 3 - 2) // integer division auto-rounds-down
        .sum() // Sum it all up 
}

fn solve_part2(lines: &[String]) -> i32 {
    lines
        .iter()
        .filter_map(|line| line.parse::<i32>().ok())
        .map(|mass| calculate_fuel(mass))
        .sum()
}

// Recursive function for part2
fn calculate_fuel(mass: i32) -> i32 {
    let fuel = mass / 3 - 2; // Work out the fuel needed for a given mass 
    if fuel <= 0 {
        // If the fuel needed is <= 0, return 0
        0
    } else {
        // Otherwise, recursively call the function to add the fuel
        // required
        fuel + calculate_fuel(fuel)
    }
}
