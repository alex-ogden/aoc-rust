use crate::utils;

pub fn part1() {
    let input: Vec<String> = utils::read_lines("inputs/2025/day10-test.txt");
    let result: u64 = solve_part1(&input);
    println!("2025 :: Day 10 :: Part 1: {}", result);
}

pub fn part2() {}

fn solve_part1(input: &Vec<String>) -> u64 {
    // For each line, work out the fastest (fewest presses) solution
    // Then sum at the end
    input
        .iter()
        .map(|line| {
            // The following code would be much smaller/neater if I could use the regex crate
            // Get required indicator configuration
            let rc = line
                .find("[")
                .and_then(|s| line.find("]").map(|e| &line[s..=e]))
                .unwrap()
                .to_string();

            // Find available buttons that can be pressed
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

            // Given available buttons, and the required configuration, workout the least number of
            // presses to get the required configuration of indicators
            get_fastest_solution(&rc, &all_ab.join(" "))
        })
        .sum() // Find the sum of all fastest solutions and return 
}

fn solve_part2() {}

// Helper functions
fn get_fastest_solution(required_config: &String, buttons_available: &String) -> u64 {
    // Split buttons into a vector where each button is it's own vector containing the lights that
    // are affected by that button
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

    // Start with a clean slate (all off)
    let mut config_state = required_config.replace("#", ".");

    // Mutate the state as we try different button combos
    for button in buttons {
        // Mutate state to match what would happen if you pushed the button(s)
        config_state = generate_config(&config_state, &button);

        // Is that the correct state?
        if config_state == *required_config {
            println!("IT MATCHES!");
            return 31;
        }
    }

    // Return random number for now to keep linter/compiler happy
    30
}

fn generate_config(init_config: &String, button_press: &Vec<u64>) -> String {
    // Takes a indicator configuration (all off if first time, current state if not) and apply
    // transformation based on the buttons pressed and the indicator lights they affect
    let mut inner_config = init_config.replace("[", "").replace("]", "");

    for indicator_idx in button_press {
        if inner_config.chars().nth(*indicator_idx as usize).unwrap() == '.' {
            inner_config.replace_range(*indicator_idx as usize..(*indicator_idx as usize + 1), "#");
        } else {
            inner_config.replace_range(*indicator_idx as usize..(*indicator_idx as usize + 1), ".");
        }
    }

    // Add the square brackets back to form the original string, but now mutated. Makes comparing
    // to required config easier later on
    let config: String = format!("{}{}{}", "[", inner_config, "]");

    // DEBUG: Output to ensure state is properly mutated
    println!("Button:          {:?}", button_press);
    println!("Initial config: {:?}", init_config);
    println!("Returning config: {:?}", config);

    config
}
