use crate::utils;

use std::collections::HashSet;

pub fn part1() {
    let input: Vec<Vec<String>> = utils::read_grid("inputs/2025/day7.txt");
    let result: u64 = solve_part1(&input);
    println!("2025 :: Day 7 :: Part 1: {}", result);
}
pub fn part2() {
    let input: Vec<Vec<String>> = utils::read_grid("inputs/2025/day7.txt");
    let result: u64 = solve_part2(&input);
    println!("2025 :: Day 7 :: Part 2: {}", result);
}

fn solve_part1(input: &Vec<Vec<String>>) -> u64 {
    let height = input.len();
    let width = input[0].len();

    let mut init_x_pos = 0;
    for i in 0..width {
        if input[0][i] == "S" {
            init_x_pos = i;
        }
    }

    let mut xpos_tracker = HashSet::new();
    xpos_tracker.insert(init_x_pos);
    let mut num_splits = 0;

    for y in 0..height {
        let mut to_remove = Vec::new();
        let mut to_add = Vec::new();

        for &xpos in xpos_tracker.iter() {
            if input[y][xpos] == "^" {
                num_splits += 1;
                to_remove.push(xpos);
                to_add.push(xpos + 1);
                to_add.push(xpos - 1);
            }
        }

        for &idx in to_remove.iter() {
            xpos_tracker.remove(&idx);
        }

        for &idx in to_add.iter() {
            xpos_tracker.insert(idx);
        }
    }

    let mut splits = HashSet::new();

    for x_pos in xpos_tracker.iter() {
        splits.insert(x_pos);
    }

    num_splits
}
fn solve_part2(input: &Vec<Vec<String>>) -> u64 {
    let height = input.len();
    let width = input[0].len();

    let mut init_x_pos = 0;
    for i in 0..width {
        if input[0][i] == "S" {
            init_x_pos = i;
        }
    }

    let mut xpos_tracker = Vec::new();
    xpos_tracker.push(init_x_pos);
    let mut num_splits = 0;

    for y in 0..height {
        let mut to_remove = Vec::new();
        let mut to_add = Vec::new();

        for &xpos in xpos_tracker.iter() {
            if input[y][xpos] == "^" {
                num_splits += 1;
                to_remove.push(xpos);
                to_add.push(xpos + 1);
                to_add.push(xpos - 1);
            }
        }

        for &xpos in to_remove.iter() {
            xpos_tracker.remove(xpos);
        }

        for &xpos in to_add.iter() {
            xpos_tracker.push(xpos);
        }

        println!("{:?}", xpos_tracker);
    }

    let mut splits = HashSet::new();

    for x_pos in xpos_tracker.iter() {
        splits.insert(x_pos);
    }

    num_splits
}
