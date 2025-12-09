use std::collections::HashSet;
use std::ops::Index;

use crate::utils;

pub fn part1() {
    let input: String = utils::read_input("inputs/2025/day5.txt");
    let result: u64 = solve_part1(&input);
    println!("2025 :: Day 5 :: Part 1: {}", result);
}

pub fn part2() {
    let input: String = utils::read_input("inputs/2025/day5.txt");
    let result: u64 = solve_part2(&input);
    println!("2025 :: Day 5 :: Part 2: {}", result);
}

fn solve_part1(input: &String) -> u64 {
    // split incoming input by 2 newlines (to seperate ids and ranges)
    let val_split = input.split("\n\n");

    // Create a vector of vectors, one contianing the ids, one with the ranges
    let mut split_vectors: Vec<Vec<&str>> = val_split
        .map(|block| block.lines().collect::<Vec<&str>>())
        .collect();

    // Assign those two vectors
    let ranges: Vec<&str> = split_vectors.remove(0);
    let ids: Vec<&str> = split_vectors.remove(0);
    let mut total_fresh = 0;

    for id in ids.iter() {
        let num_id: u64 = id.parse().unwrap();

        for range in ranges.iter() {
            let b_range: u64 = range.split("-").next().unwrap().parse().unwrap();
            let t_range: u64 = range.split("-").nth(1).unwrap().parse().unwrap();

            if num_id >= b_range && num_id <= t_range {
                // ID found in a range, add one to fresh count
                total_fresh += 1;
                break;
            }
        }
    }

    total_fresh
}

fn solve_part2(input: &String) -> u64 {
    let val_split = input.split("\n\n");

    // Create a vector of vectors, one contianing the ids, one with the ranges
    let mut split_vectors: Vec<Vec<&str>> = val_split
        .map(|block| block.lines().collect::<Vec<&str>>())
        .collect();

    // Assign those two vectors
    let ranges: Vec<&str> = split_vectors.remove(0);
    let mut fresh_set = HashSet::new();

    for range in ranges.iter() {
        println!("Working on range: {}", range);
        let b_range: u64 = range.split("-").next().unwrap().parse().unwrap();
        let t_range: u64 = range.split("-").nth(1).unwrap().parse().unwrap();

        for id in b_range..=t_range {
            fresh_set.insert(id);
        }
    }

    fresh_set.len() as u64
}
