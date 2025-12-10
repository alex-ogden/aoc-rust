use crate::utils;

use std::cmp::Reverse;

pub fn part1() {
    let input: Vec<String> = utils::read_lines("inputs/2025/day6.txt");
    let result: u64 = solve_part1(&input);
    println!("2025 :: Day 6 :: Part 1: {}", result);
}
pub fn part2() {
    let input: Vec<String> = utils::read_lines("inputs/2025/day6.txt");
    let result: u64 = solve_part2(&input);
    println!("2025 :: Day 6 :: Part 2: {}", result);
}

fn solve_part1(input: &Vec<String>) -> u64 {
    // Create vector of vectors of numbers
    let nums_vec: Vec<Vec<u64>> = input
        .iter()
        .take(input.len() - 1)
        .map(|line| {
            line.split_whitespace()
                .map(|num_str| num_str.parse::<u64>().expect("Failed to parse number"))
                .collect()
        })
        .collect();

    // Create vector of operations
    let operations: Vec<&str> = input
        .last()
        .expect("Input is empty")
        .split_whitespace()
        .collect();

    let mut total: u64 = 0;
    // For each index on the 'x-axis', perform calculation on all numbers in the 'column' (if you
    // think of it like a 2D grid)
    for (op_idx, &operation) in operations.iter().enumerate() {
        if operation == "+" {
            // Sum up all numbers in the column
            let subtotal: u64 = nums_vec.iter().map(|row| row[op_idx]).sum();
            total += subtotal;
        } else {
            // Get the product of all numbers in the column
            let subtotal: u64 = nums_vec.iter().map(|row| row[op_idx]).product();
            total += subtotal;
        }
    }

    total
}

fn solve_part2(input: &[String]) -> u64 {
    let grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut total = 0u64;
    let mut subtotal = 0u64;
    let mut operation = '*';

    for col in 0..width {
        // Check if column is all spaces
        if grid.iter().all(|row| row[col] == ' ') {
            total += subtotal;
            subtotal = 0;
            continue;
        }

        // Extract operator from last row
        let last_char = grid[height - 1][col];
        if last_char == '+' || last_char == '*' {
            operation = last_char;
        }

        // Extract number from column (excluding last row which has operator)
        let num_str: String = grid[..height - 1].iter().map(|row| row[col]).collect();

        // Parse and apply operation
        if let Ok(num) = num_str.trim().parse::<u64>() {
            subtotal = match operation {
                '+' => subtotal + num,
                '*' => {
                    if subtotal == 0 {
                        num
                    } else {
                        subtotal * num
                    }
                }
                _ => subtotal,
            };
        }
    }

    total + subtotal
}
