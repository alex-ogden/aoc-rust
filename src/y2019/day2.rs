use crate::utils;

pub fn part1() {
    let input = utils::read_input("inputs/2019/day2.txt");
    let result = solve_both_parts(&input, 12, 2);
    println!("2019 :: Day2 :: Part1: {}", result);
}

pub fn part2() {
    let input = utils::read_input("inputs/2019/day2.txt");
    for i in 0..100 {
        for j in 0..100 {
            let result = solve_both_parts(&input, i, j);
            if result == "19690720" {
                println!("2019 :: Day2 :: Part2: {}", 100 * i + j);
            }
        }
    }
}

/*
   Turns out we can use the same function to work out part1 and 2, how we call
   it is different however
*/
fn solve_both_parts(input: &str, noun: u32, verb: u32) -> String {
    let mut program_counter = 0;

    // Split input string by commas into slice
    let mut input_vec: Vec<String> = input
        .trim()
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();

    // Replace values
    input_vec[1] = noun.to_string();
    input_vec[2] = verb.to_string();

    loop {
        let opcode = input_vec[program_counter].parse::<u32>().expect(&format!(
            "Failed to parse opcode at position: {}",
            program_counter
        ));

        if opcode == 99 {
            break;
        }

        let value_one_pos = input_vec[program_counter + 1]
            .parse::<usize>()
            .expect(&format!(
                "Failed to parse value_one_pos at position {}",
                program_counter + 1
            ));
        let value_two_pos = input_vec[program_counter + 2]
            .parse::<usize>()
            .expect(&format!(
                "Failed to parse value_two_pos at position {}",
                program_counter + 2
            ));
        let store_addr = input_vec[program_counter + 3]
            .parse::<usize>()
            .expect(&format!(
                "Failed to parse store_addr at position {}",
                program_counter + 3
            ));

        let value_one = input_vec[value_one_pos].parse::<u32>().expect(&format!(
            "Failed to parse value at position {}",
            value_one_pos
        ));
        let value_two = input_vec[value_two_pos].parse::<u32>().expect(&format!(
            "Failed to parse value at position {}",
            value_two_pos
        ));

        // Work out what to do based on opcode
        let result = match opcode {
            1 => (value_one + value_two).to_string(),
            2 => (value_one * value_two).to_string(),
            _ => panic!("Unknown opcode: {}", opcode),
        };

        input_vec[store_addr] = result;
        program_counter += 4;
    }
    // Return first index
    input_vec[0].clone()
}
