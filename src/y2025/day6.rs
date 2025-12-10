use crate::utils;

use std::cmp::Reverse;

pub fn part1() {
    let input: Vec<String> = utils::read_lines("inputs/2025/day6.txt");
    let result: u64 = solve_part1(&input);
    println!("2025 :: Day 6 :: Part 1: {}", result);
}
pub fn part2() {
    let input: Vec<String> = utils::read_lines("inputs/2025/day6-test.txt");
    let result: u64 = solve_part2(&input);
    println!("2025 :: Day 6 :: Part 2: {}", result);
}

fn solve_part1(input: &Vec<String>) -> u64 {
    let nums_vec: Vec<Vec<u64>> = input
        .iter()
        .take(input.len() - 1)
        .map(|line| {
            line.split_whitespace()
                .map(|num_str| num_str.parse::<u64>().expect("Failed to parse number"))
                .collect()
        })
        .collect();

    let operations: Vec<&str> = input
        .last()
        .expect("Input is empty")
        .split_whitespace()
        .collect();

    let mut total: u64 = 0;
    for (op_idx, &operation) in operations.iter().enumerate() {
        if operation == "+" {
            let subtotal: u64 = nums_vec.iter().map(|row| row[op_idx]).sum();
            total += subtotal;
        } else {
            let subtotal: u64 = nums_vec.iter().map(|row| row[op_idx]).product();
            total += subtotal;
        }
    }

    total
}

fn solve_part2(input: &Vec<String>) -> u64 {
    let nums_vec: Vec<Vec<&str>> = input
        .iter()
        .take(input.len() - 1)
        .map(|line| line.split_whitespace().collect())
        .collect();

    let operations: Vec<&str> = input
        .last()
        .expect("Input is empty")
        .split_whitespace()
        .collect();

    println!("Nums Vec: {:?}", nums_vec);
    println!("Ops Vec:  {:?}", operations);

    let mut total: u64 = 0;
    for (op_idx, &operation) in operations.iter().enumerate() {
        if operation == "+" {
            // Create a vector of numbers to work on sorted by size largest->smallest
            let mut nums_to_work_on: Vec<&str> = nums_vec.iter().map(|row| row[op_idx]).collect();
            nums_to_work_on.sort_by_key(|n_str| Reverse(n_str.len()));

            total += 1;
        } else {
            let mut nums_to_work_on: Vec<&str> = nums_vec.iter().map(|row| row[op_idx]).collect();
            nums_to_work_on.sort_by_key(|n_str| Reverse(n_str.len()));

            total += 1;
        }
    }
    total
}
