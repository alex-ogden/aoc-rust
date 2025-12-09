use range_set_blaze::RangeSetBlaze;
use std::str::FromStr;

use crate::utils;

pub fn part1() {
    let input: String = utils::read_input("inputs/2025/day5.txt");
    let result: u64 = solve_part1(&input);
    println!("2025 :: Day 5 :: Part 1: {}", result);
}

pub fn part2() {
    let input: String = utils::read_input("inputs/2025/day5.txt");
    let result: u128 = solve_part2(&input);
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
fn solve_part2(input: &String) -> u128 {
    let val_split = input.split("\n\n");

    // Create a vector of vectors, one containing the ids, one with the ranges
    let mut split_vectors: Vec<Vec<&str>> = val_split
        .map(|block| block.lines().collect::<Vec<&str>>())
        .collect();

    // Assign those two vectors
    let range_strings: Vec<&str> = split_vectors.remove(0);
    // Use u128 directly, we will unwrap results immediately
    let mut ranges = Vec::<(u128, u128)>::new();

    for range_str in range_strings {
        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() != 2 {
            // Panic as requested if format is strictly wrong
            panic!("Couldn't extract two parts from {}", range_str);
        }

        // Use unwrap() to extract the number or panic if parsing fails
        let start = u128::from_str(parts[0]).unwrap();
        let end = u128::from_str(parts[1]).unwrap();
        ranges.push((start, end));
    }

    // `RangeSetBlaze` handles merging, sorting, and deduplication
    let range_set: RangeSetBlaze<u128> =
        ranges.into_iter().map(|(start, end)| start..=end).collect();

    println!("Stored {} unique, merged ranges", range_set.len());

    // Return the total count of numbers using .count() method provided by the crate
    range_set.total_count()
}
