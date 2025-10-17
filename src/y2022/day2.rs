use crate::utils;

pub fn part1() {
    let input: Vec<String> = utils::read_lines("inputs/2022/day2.txt");
    let result: u32 = solve_part1(&input);
    println!("2022 :: Day 2 :: Part 1: {}", result);
}

pub fn part2() {
    let input: Vec<String> = utils::read_lines("inputs/2022/day2.txt");
    let result: u32 = solve_part2(&input);
    println!("2022 :: Day 2 :: Part 2: {}", result);
}

fn solve_part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| {
            let (opp, player) = line.split_once(" ").expect("Invalid line format!");
            score_round(opp, player)
        })
        .sum()
}

fn solve_part2(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| {
            let (opp, req_outcome) = line.split_once(" ").expect("Invalid line format!");
            score_round2(opp, req_outcome)
        })
        .sum()
}

// Helper functions
fn score_round(opp: &str, player: &str) -> u32 {
    let hand_score = match player {
        "X" => 1, // Rock
        "Y" => 2, // Paper
        "Z" => 3, // Scissors
        _ => panic!("Invalid player hand: {}", player),
    };

    let game_score = match (opp, player) {
        ("A", "Z") | ("B", "X") | ("C", "Y") => 0, // Loss
        ("A", "X") | ("B", "Y") | ("C", "Z") => 3, // Draw
        ("A", "Y") | ("B", "Z") | ("C", "X") => 6, // Win
        _ => panic!("Invalid opponent hand: {}", opp),
    };

    hand_score + game_score
}

fn score_round2(opp: &str, req_outcome: &str) -> u32 {
    let outcome_score = match req_outcome {
        "X" => 0, // Loss
        "Y" => 3, // Draw
        "Z" => 6, // Win
        _ => panic!("Invalid outcome: {}", req_outcome),
    };

    let hand_selection = match (opp, req_outcome) {
        ("B", "X") | ("A", "Y") | ("C", "Z") => 1, // You have to choose rock
        ("C", "X") | ("B", "Y") | ("A", "Z") => 2, // You have to choose paper
        ("A", "X") | ("C", "Y") | ("B", "Z") => 3, // You have to choose scissors
        _ => panic!("Invalid requirement"),
    };

    outcome_score + hand_selection
}
