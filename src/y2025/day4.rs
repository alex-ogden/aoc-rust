use crate::utils;

pub fn part1() {
    let input: Vec<Vec<String>> = utils::read_grid("inputs/2025/day4.txt");
    let result: u32 = solve_part1(&input);
    println!("2025 :: Day 4 :: Part 1: {}", result);
}
pub fn part2() {}

fn solve_part1(input: &Vec<Vec<String>>) -> u32 {
    let height = input.len();
    let width = input[0].len();

    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(move |(x, cell)| {
                    **cell == "@" && utils::get_num_neighbours(input, *x, y, height, width, "@") < 4
                })
                .map(|_| 1)
        })
        .sum()
}

// fn solve_part2(input: &Vec<Vec<String>>) -> u32 {
//     let height = input.len();
//     let width = input[0].len();
//
//     input.iter().enumerate().flat_map(|(y, row)| {
//         row.iter()
//             .enumerate()
//             .filter(move |(x, cell)| {
//                 **cell == "@" && utils::get_num_neighbours(input, *x, y, height, width, "@") < 4
//             })
//             .map(|_| 1)
//     })
// }
