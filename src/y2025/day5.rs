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
    let sections: Vec<&str> = input.split("\n\n").collect();
    let ranges_section = sections[0];

    // Parse all ranges
    let mut ranges: Vec<(u128, u128)> = ranges_section
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split("-").collect();
            let start = parts[0].parse::<u128>().unwrap();
            let end = parts[1].parse::<u128>().unwrap();
            (start, end)
        })
        .collect();

    // Sort ranges by start position
    ranges.sort_by_key(|r| r.0);

    // Merge overlapping ranges
    let merged = merge_ranges(ranges);

    // Count total IDs in merged ranges
    merged.iter().map(|(start, end)| end - start + 1).sum()
}

fn merge_ranges(ranges: Vec<(u128, u128)>) -> Vec<(u128, u128)> {
    if ranges.is_empty() {
        return vec![];
    }

    // Create new vector including the first range
    let mut merged = vec![ranges[0]];

    // Loop through the other ranges from the second onwards
    for (start, end) in ranges.into_iter().skip(1) {
        let last_idx = merged.len() - 1;
        let (last_start, last_end) = merged[last_idx];

        // Check if current range overlaps or is adjacent to the last merged range
        if start <= last_end + 1 {
            // Merge by extending the end
            merged[last_idx] = (last_start, last_end.max(end));
        } else {
            // If no overlap, add as new range
            merged.push((start, end));
        }
    }

    merged
}
