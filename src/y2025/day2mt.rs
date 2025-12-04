use crate::utils;
use rayon::prelude::*;

pub fn part1() {
    let input: String = utils::read_input("inputs/2025/day2.txt");
    let result: u64 = solve_part1(&input);
    println!("2025 :: Day 2 :: Part 1 (MT): {}", result);
}

pub fn part2() {
    let input: String = utils::read_input("inputs/2025/day2.txt");
    let result: u64 = solve_part2(&input);
    println!("2025 :: Day 2 :: Part 2 (MT): {}", result);
}

fn solve_part1(input: &str) -> u64 {
    input
        .par_split(',')
        .map(|id_range| {
            let (first, last) = id_range.split_once('-').unwrap();
            let first_id: u64 = first.trim().parse().unwrap();
            let last_id: u64 = last.trim().parse().unwrap();

            let mut local_total = 0;
            for id in first_id..=last_id {
                let id_as_string = id.to_string();
                let mid = id_as_string.len() / 2;
                let (first_half, second_half) = id_as_string.split_at(mid);

                if first_half == second_half {
                    local_total += id;
                }
            }
            local_total
        })
        .sum()
}

fn solve_part2(input: &str) -> u64 {
    input
        .par_split(',')
        .map(|id_range| {
            let (first, last) = id_range.split_once('-').unwrap();
            let first_id: u64 = first.trim().parse().unwrap();
            let last_id: u64 = last.trim().parse().unwrap();

            let mut local_total = 0;
            for id in first_id..=last_id {
                let id_as_string = id.to_string();
                let len = id_as_string.len();

                for pattern_len in 1..=(len / 2) {
                    if len % pattern_len == 0 {
                        let pattern = &id_as_string[..pattern_len];
                        if pattern.repeat(len / pattern_len) == id_as_string {
                            local_total += id;
                            break;
                        }
                    }
                }
            }
            local_total
        })
        .sum()
}
