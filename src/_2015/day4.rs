use crate::utils;

// Allowing myself to use the md5 crate as I'm not writing an MD5 function
// for Advent of Code :)
use md5;
use std::fmt::Write;

pub fn part1() {
    let input: String = utils::read_input(2015, 4, false);
    let result: u64 = solve(&input, 5);
    println!("2015 :: Day 4 :: Part 1: {}", result);
}

pub fn part2() {
    let input: String = utils::read_input(2015, 4, false);
    let result: u64 = solve(&input, 6);
    println!("2015 :: Day 4 :: Part 2: {}", result);
}

// Realised after solving part 1 that part 2 is the same problem, different number of zeros
fn solve(input: &str, zeroes: usize) -> u64 {
    let input = input.trim();
    let mut num: u64 = 0;
    // Create buffer string with room for up to a 20 digit number
    let mut buffer = String::with_capacity(input.len() + 20);

    loop {
        // Clear the buffer and add current number and input
        buffer.clear();
        buffer.push_str(input);
        write!(&mut buffer, "{num}").unwrap();

        // Create the hash (16 raw bytes)
        let digest = md5::compute(&buffer);

        //
        let ok: bool = match zeroes {
            // Match the first 2 bytes then half of the third (a hex 0 is 4 bits, 2 per byte)
            5 => digest[0] == 0 && digest[1] == 0 && (digest[2] & 0xF0) == 0,
            // Match the first 3 bytes for 6 total hex 0's
            6 => digest[0] == 0 && digest[1] == 0 && digest[2] == 0,
            _ => unreachable!(),
        };

        // If we found 5/6 leading zeros, return
        if ok {
            return num;
        }
        // Otherwise increment
        num += 1;
    }
}
