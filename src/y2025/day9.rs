use crate::utils;

pub fn part1() {
    let input: Vec<String> = utils::read_lines("inputs/2025/day9.txt");
    let result: u64 = solve_part1(&input);
    println!("2025 :: Day 9 :: Part 1: {}", result);
}
pub fn part2() {}
fn solve_part2() {}

fn solve_part1(input: &Vec<String>) -> u64 {
    // Parse coordinates as tuples of u64s
    let coords: Vec<(u64, u64)> = input
        .iter()
        .map(|line| {
            let mut parts = line.split(",");
            let x = parts.next().unwrap().parse::<u64>().unwrap();
            let y = parts.next().unwrap().parse::<u64>().unwrap();
            (x, y)
        })
        .collect();

    let mut max_area = 0u64;
    for i in 0..coords.len() {
        // Always ensure j is checking one ahead of where i is
        for j in (i + 1)..coords.len() {
            let (x1, y1) = coords[i];
            let (x2, y2) = coords[j];
            let area = get_area(x1, y1, x2, y2);

            // Rather than storing areas in an array and sorting it (or taking the largest element)
            // Set max_area to whatever is bigger out of the current max_area and the area we've
            // just worked out
            max_area = max_area.max(area);
        }
    }

    max_area
}

// Helper functions
fn get_area(x1: u64, y1: u64, x2: u64, y2: u64) -> u64 {
    let height = y1.max(y2) - y1.min(y2) + 1;
    let width = x1.max(x2) - x1.min(x2) + 1;
    height * width
}
