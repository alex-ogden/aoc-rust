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

            let contains_forbidden: bool = forbidden_substrings.iter().any(|&substring| line.contains(substring));

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
    //  - has one letter that repeats with a letter between them (i.e aya or uuu)
    //  - has repeating, but not overapping pairs that appear at least twice (i.e yxyx or aabytaa)
    input
        .iter()
        .map(|line| {
            // check for repeating letter with letter between
            let mut has_repeat: bool = false;
            let mut has_repeat_no: bool = false;

            println!("Line: {}", line);

            for (idx,letter) in line.chars().enumerate() {
                if idx < line.len() - 2 {
                    let section = &line[idx..idx+3];
                    if section.chars().nth(0) == section.chars().nth(2) {
                        has_repeat = true;
                        println!("\tHas repeat: {} ({})", if has_repeat { "true" } else { "false" }, section);
                        break;
                    }
                }
            }

            for (idx,letter) in line.chars().enumerate() {
                if idx < line.len() - 3 {
                    let section = &line[idx..idx+2].to_string();
                    let check_section = &line[idx+3..].to_string();
                    if check_section.matches(section).count() > 0 {
                        has_repeat_no = true;
                        println!("\tHas repeat no: {} ({})", if has_repeat_no { "true" } else { "false" }, section);
                        break;
                    }
                }
            }

            if has_repeat && has_repeat_no {
                println!("NICE!");
                1 
            } else {
                println!("NOT NICE!");
                0
            }
        })
        .sum()
}
