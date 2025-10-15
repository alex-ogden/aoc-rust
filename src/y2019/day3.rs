use crate::utils;

pub fn part1() {
    let input: Vec<String> = utils::read_lines("inputs/2019/day3.txt");
    let result: i32 = solve_part1(&input);
    println!("2019 :: Day3 :: Part1: {}", result);
}

//pub fn part2() {}

fn solve_part1(wire_paths: &Vec<String>) -> i32 {
    let wire1_path: String = wire_paths[0];
    let wire2_path: String = wire_paths[1];

    // Store every place on the grid a wire covers (to find clashes/crossovers)
    // Stored as string i.e "21" would be y=2 x=1 
    let mut wire1_coords: Vec<String> = Vec::new();
    let mut wire2_coords: Vec<String> = Vec::new();

    // Plot wire1 path
    for dir in wire1_path {
        direction = dir[0];
        distance = dir[1:];

        match direction {
        }
    }
}

//fn solve_part2() -> i32 {}
