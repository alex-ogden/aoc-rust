use crate::utils;

pub fn part1() {
    let input: Vec<String> = utils::read_lines(2025, 1, false);
    let result: u32 = solve_part1(&input);
    println!("2025 :: Day 1 :: Part 1: {}", result);
}
pub fn part2() {
    let input: Vec<String> = utils::read_lines(2025, 1, false);
    let result: u32 = solve_part2(&input);
    println!("2025 :: Day 1 :: Part 2: {}", result);
}

fn solve_part1(input: &[String]) -> u32 {
    let mut position: i32 = 50;
    let mut zero_count: u32 = 0;

    for line in input.iter() {
        if line.is_empty() {
            continue;
        }

        let direction = line.chars().next().unwrap();
        let distance: i32 = line[1..].parse().unwrap();

        position = match direction {
            'L' => position.wrapping_sub(distance).rem_euclid(100),
            'R' => position.wrapping_add(distance).rem_euclid(100),
            _ => panic!("Unexpected direction: {}", direction),
        };

        if position == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

fn solve_part2(input: &[String]) -> u32 {
    let mut position: i32 = 50;
    let mut zero_count: u32 = 0;

    for line in input.iter() {
        if line.is_empty() {
            continue;
        }

        let direction = line.chars().next().unwrap();
        let distance: i32 = line[1..].parse().unwrap();

        match direction {
            'L' => {
                for _ in 0..distance {
                    position -= 1;
                    if position < 0 {
                        position = 99;
                    }
                    if position == 0 {
                        zero_count += 1;
                    }
                }
            }
            'R' => {
                for _ in 0..distance {
                    position += 1;
                    if position > 99 {
                        position = 0;
                    }
                    if position == 0 {
                        zero_count += 1;
                    }
                }
            }
            _ => panic!("Unexpected direction: {}", direction),
        }
    }

    zero_count
}
