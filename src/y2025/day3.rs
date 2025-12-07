use crate::utils;

pub fn part1() {
    let input: Vec<String> = utils::read_lines("inputs/2025/day3.txt");
    let result: u32 = solve_part1(&input);
    println!("2025 :: Day 3 :: Part 1: {}", result);
}

pub fn part2() {
    let input: Vec<String> = utils::read_lines("inputs/2025/day3.txt");
    let result: u64 = solve_part2(&input);
    println!("2025 :: Day 3 :: Part 2: {}", result);
}

fn solve_part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|bank| {
            let digits: Vec<u8> = bank.bytes().map(|b| b - b'0').collect();

            // Exclude last number as it cannot be the first number
            let (first_idx, &first) = digits[..digits.len() - 1]
                .iter()
                .enumerate()
                .rev()
                .max_by_key(|(_, d)| *d)
                .unwrap();

            let second = *digits[first_idx + 1..].iter().max().unwrap();

            (first * 10 + second) as u32
        })
        .sum()
}

fn solve_part2(input: &[String]) -> u64 {
    // Same as part 1 but we need the 12 largest
    input
        .iter()
        .map(|bank| {
            let digits: Vec<u8> = bank.bytes().map(|b| b - b'0').collect();
            let numbers_to_find = 12;
            let mut numbers_remaining = numbers_to_find;
            let mut next_start = 0;
            let mut final_number = Vec::with_capacity(numbers_to_find);

            for _ in 0..numbers_to_find {
                let (idx, &num) = digits[next_start..=digits.len() - numbers_remaining]
                    .iter()
                    .enumerate()
                    .rev()
                    .max_by_key(|(_, d)| *d)
                    .unwrap();

                next_start += idx + 1;
                numbers_remaining -= 1;
                final_number.push(num);
            }

            concat_digits(&final_number)
        })
        .sum()
}

// Helper function to go from Vec<u8> to u64
fn concat_digits(vec: &[u8]) -> u64 {
    // For each digit, multiply current by 10 then add next digit
    // e.g [1,2,3]
    // 0 * 10 + 1 = 1
    // 1 * 10 + 2 = 12
    // 12 * 10 + 3 = 123
    vec.iter().fold(0u64, |acc, &digit| acc * 10 + digit as u64)
}
