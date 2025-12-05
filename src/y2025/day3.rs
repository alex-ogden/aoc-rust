use crate::utils;

pub fn part1() {
    let input: Vec<String> = utils::read_lines("inputs/2025/day3.txt");
    let result: u32 = solve_part1(&input);
    println!("2025 :: Day 3 :: Part 1: {}", result);
}

pub fn part2() {}

fn solve_part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|bank| {
            // Create a vector of integers out of the number
            let numbers: Vec<u8> = bank
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as u8)
                .collect();

            // Find the largest number in the vector
            // (and the first instance of it if duplicated)
            let (l_index, l_value) = numbers
                .iter()
                .take(numbers.len() - 1) // Exclude last element
                .enumerate()
                .rev()
                .max_by_key(|&(_, &val)| val)
                .unwrap();

            // Check if the largest number was the one before the end
            // If so, the second number has to be the following number
            let h_value: u8 = if l_index == numbers.len() - 2 {
                numbers[numbers.len() - 1]
            } else {
                // If not, scan numbers from l_index onwards to find the largest
                *numbers[l_index + 1..]
                    .to_vec()
                    .iter()
                    .rev()
                    .max_by_key(|&val| val)
                    .unwrap()
            };

            let l_val_str: String = l_value.to_string();
            let h_val_str: String = h_value.to_string();

            let result_str = format!("{}{}", l_val_str, h_val_str);
            let result: u32 = result_str.parse().unwrap();

            println!(
                "\nBank: {}\nL VAL: {}\nL IND: {}\nH VAL: {}\nResult: {}\n",
                bank, l_val_str, l_index, h_val_str, result
            );

            result
        })
        .sum()
}
