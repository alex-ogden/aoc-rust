use crate::utils;

use std::collections::{HashMap, HashSet};

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
    let mut xpos_tracker: HashMap<u64, u64> = HashMap::new();
    for i in 0..width {
        xpos_tracker.insert(i as u64, 0u64);
        if input[0][i] == "S" {
            init_x_pos = i;
        }
    }

    *xpos_tracker.entry(init_x_pos as u64).or_default() += 1;

    for y in 0..height {
        println!("Working on line {}/{}", y + 1, height);
        for xpos in 0..width {
            println!("\t{}/{} ", xpos + 1, width);
            if input[y][xpos] == "^" {
                for _ in 0..*xpos_tracker.get(&(xpos as u64)).unwrap() as u64 {
                    *xpos_tracker.entry((xpos as u64) - 1 as u64).or_default() += 1;
                    *xpos_tracker.entry((xpos as u64) + 1 as u64).or_default() += 1;
                    xpos_tracker.insert(xpos as u64, 0);
                }
            }
        }
        println!();
    }

    let mut total = 0;
    for k in xpos_tracker.keys() {
        total += xpos_tracker.get(k).unwrap();
    }

    total
}
