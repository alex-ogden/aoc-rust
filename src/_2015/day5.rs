use crate::utils;

pub fn part1() {
    let input: Vec<String> = utils::read_lines(2015, 5, false);
    let result = solve_part1(&input);
    println!("2015 :: Day 5 :: Part 1: {}", result);
}

pub fn part2() {
    let input: Vec<String> = utils::read_lines(2015, 5, false);
    let result = solve_part2(&input);
    println!("2015 :: Day 5 :: Part 2: {}", result);
}

fn solve_part1(input: &Vec<String>) -> u64 {
    // Nice strings:
    //  - has at least 3 vowels
    //  - has at least 1 letter that appears twice in a row
    //  - does not contain ab, cd, pq, or xy
    let vowels = "aeiou";
    let forbidden_substrings = ["ab", "cd", "pq", "xy"];
    input
        .iter()
        .map(|line| {
            let mut vowel_count: usize = 0;
            let mut has_repeat: bool = false;
            for letter in vowels.chars() {
                vowel_count += line.chars().filter(|&c| c == letter).count();
            }
            for (idx, letter) in line.chars().enumerate() {
                if idx != line.len() - 1 {
                    if letter == line.as_bytes()[idx + 1] as char {
                        has_repeat = true;
                    }
                }
            }

            let contains_forbidden: bool = forbidden_substrings
                .iter()
                .any(|&substring| line.contains(substring));

            if (vowel_count >= 3) && has_repeat && !contains_forbidden {
                return 1;
            } else {
                return 0;
            }
        })
        .sum()
}

fn solve_part2(input: &Vec<String>) -> u64 {
    // Nice strings:
    //  - has repeating, but not overapping pairs that appear at least twice (i.e yxyx or aabytaa)
    //  - has one letter that repeats with a letter between them (i.e aya or uuu)
    input
        .iter()
        .map(|line| {
            let mut repeat_with_no_overlap: bool = false;
            let mut repeat_with_letter_between: bool = false;

            // Check for repeat with no overlap
            for letter_index in 0..line.len() {
                if letter_index < line.len() - 3 {
                    // Take groups of two letters
                    let check_section = &line[letter_index..letter_index + 2];
                    let remaining_section: &str = &line[letter_index + 2..line.len()];

                    if remaining_section.contains(check_section) {
                        repeat_with_no_overlap = true;
                        break;
                    }
                }
            }

            // Check for repeat with letter between
            for letter_index in 0..line.len() {
                if letter_index < line.len() - 2 {
                    let check_section = &line[letter_index..letter_index + 3];
                    if check_section.chars().nth(0) == check_section.chars().nth(2) {
                        repeat_with_letter_between = true;
                        break;
                    }
                }
            }

            if repeat_with_no_overlap && repeat_with_letter_between {
                1
            } else {
                0
            }
        })
        .sum()
}
