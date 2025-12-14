use crate::utils;

pub fn part1() {
    let input: Vec<String> = utils::read_lines("inputs/2025/day10-test.txt");
    let result: u64 = solve_part1(&input);
    println!("2025 :: Day 10 :: Part 1: {}", result);
}

pub fn part2() {}

fn solve_part1(input: &Vec<String>) -> u64 {
    input
        .iter()
        .map(|line| {
            let rc = line
                .find("[")
                .and_then(|s| line.find("]").map(|e| &line[s..=e]))
                .unwrap()
                .to_string();

            // Find available buttons
            let mut all_ab = Vec::new();
            let mut temp = line.as_str();
            while let Some(s) = temp.find("(") {
                if let Some(e) = temp[s..].find(")") {
                    all_ab.push(&temp[s..=s + e]);
                    temp = &temp[s + e + 1..];
                } else {
                    break;
                }
            }

            get_fastest_solution(&rc, &all_ab.join(" "))
        })
        .sum()
}

fn solve_part2() {}

// Helper functions
fn get_fastest_solution(required_config: &String, buttons_available: &String) -> u64 {
    let buttons: Vec<Vec<u64>> = buttons_available
        .split_whitespace()
        .map(|button| {
            button
                .replace("(", "")
                .replace(")", "")
                .split(",")
                .map(|num| num.parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    let mut config_state = required_config.replace("#", ".");

    for button in buttons {
        // change config state to match what would happen if you pushed the button(s)
        config_state = generate_config(&config_state, &button);
        if config_state == *required_config {
            println!("IT MATCHES!");
            return 31;
        }
    }

    30
}

fn generate_config(init_config: &String, button_press: &Vec<u64>) -> String {
    // Get the number of indicators on the required config to create a matching
    // all-off config to simulate
    let mut inner_config = init_config.replace("[", "").replace("]", "");

    for indicator_idx in button_press {
        if inner_config.chars().nth(*indicator_idx as usize).unwrap() == '.' {
            inner_config.replace_range(*indicator_idx as usize..(*indicator_idx as usize + 1), "#");
        } else {
            inner_config.replace_range(*indicator_idx as usize..(*indicator_idx as usize + 1), ".");
        }
    }

    let config: String = format!("{}{}{}", "[", inner_config, "]");

    println!("Button:          {:?}", button_press);
    println!("Initial config: {:?}", init_config);
    println!("Returning config: {:?}", config);

    config
}
