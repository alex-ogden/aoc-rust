use crate::utils;

pub fn part1() {
    let input: String = utils::read_input("inputs/2025/day2.txt");
    let result: u64 = solve_part1(&input);
    println!("2025 :: Day02 :: Part01: {}", result);
}

pub fn part2() {
    let input: String = utils::read_input("inputs/2025/day2.txt");
    let result: u64 = solve_part2(&input);
    println!("2025 :: Day02 :: Part02: {}", result);
}

fn solve_part1(input: &str) -> u64 {
    let mut invalid_id_total: u64 = 0;

    input.split(",").for_each(|id_range| {
        let first_id: u64 = id_range.split_once("-").unwrap().0.trim().parse().unwrap();
        let last_id: u64 = id_range.split_once("-").unwrap().1.trim().parse().unwrap();

        for id in first_id..=last_id {
            // Check if id is made of digits repeated twice
            // Split id in half, check if two halves are the same
            let id_as_string: String = id.to_string();
            let first_half = id_as_string.split_at(id_as_string.chars().count() / 2).0;
            let second_half = id_as_string.split_at(id_as_string.chars().count() / 2).1;

            if first_half == second_half {
                // Invalid ID
                invalid_id_total += id;
            }
        }
    });

    invalid_id_total
}

fn solve_part2(input: &str) -> u64 {
    let mut invalid_id_total: u64 = 0;

    input.split(",").for_each(|id_range| {
        let first_id: u64 = id_range.split_once("-").unwrap().0.trim().parse().unwrap();
        let last_id: u64 = id_range.split_once("-").unwrap().1.trim().parse().unwrap();

        for id in first_id..=last_id {
            let id_as_string: String = id.to_string();
            let len = id_as_string.len();

            for pattern_len in 1..=(len / 2) {
                // Only check if the pattern divides evenly into the total length
                if len.is_multiple_of(pattern_len) {
                    let pattern = &id_as_string[..pattern_len];
                    if pattern.repeat(len / pattern_len) == id_as_string {
                        invalid_id_total += id;
                        break;
                    }
                }
            }
        }
    });
    invalid_id_total
}
