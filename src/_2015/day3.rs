use crate::utils;

use std::collections::HashSet;

pub fn part1() {
    let input: String = utils::read_input(2015, 3, false);
    let result: u64 = solve_part1(&input);
    println!("2015 :: Day 3 :: Part 1: {}", result);
}

pub fn part2() {
    let input: String = utils::read_input(2015, 3, false);
    let result: u64 = solve_part2(&input);
    println!("2015 :: Day 3 :: Part 2: {}", result);
}

fn solve_part1(input: &String) -> u64 {
    let mut santa_pos: (i64, i64) = (0, 0);
    let mut visited_houses: HashSet<i64> = HashSet::new();

    let _ = input.trim().chars().map(|dir| {
        match dir {
            '>' => santa_pos.0 += 1,
            '<' => santa_pos.0 -= 1,
            'v' => santa_pos.1 += 1,
            '^' => santa_pos.1 -= 1,
            _ => unreachable!(),
        }

        // We don't know the size of the grid (puzzle says infinite) so just use a massive number
        // as the 'grid length'
        visited_houses.insert(santa_pos.1 * 1_000_000 + santa_pos.0);
    });

    visited_houses.len() as u64
}

fn solve_part2(input: &String) -> u64 {
    let mut santa_pos: (i64, i64) = (0, 0);
    let mut robo_santa_pos: (i64, i64) = (0, 0);
    let mut visited_houses: HashSet<i64> = HashSet::new();

    for (idx, dir) in input.trim().chars().enumerate() {
        let pos = if idx % 2 == 0 {
            &mut santa_pos
        } else {
            &mut robo_santa_pos
        };

        match dir {
            '>' => pos.0 += 1,
            '<' => pos.0 -= 1,
            'v' => pos.1 += 1,
            '^' => pos.1 -= 1,
            _ => unreachable!(),
        }

        let house_visited: i64 = pos.1 * 1_000_000 + pos.0;
        visited_houses.insert(house_visited);
    }

    visited_houses.len() as u64
}
