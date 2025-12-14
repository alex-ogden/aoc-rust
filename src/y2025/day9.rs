use crate::utils;

use rayon::prelude::*;
use std::collections::HashSet;

pub fn part1() {
    let input: Vec<String> = utils::read_lines("inputs/2025/day9.txt");
    let result: u64 = solve_part1(&input);
    println!("2025 :: Day 9 :: Part 1: {}", result);
}

pub fn part2() {
    let input: Vec<String> = utils::read_lines("inputs/2025/day9.txt");
    let result: u64 = solve_part2(&input);
    println!("2025 :: Day 9 :: Part 2: {}", result);
}

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

    let mut max_area: u64 = 0;
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
fn solve_part2(input: &Vec<String>) -> u64 {
    let mut valid_recs: Vec<u64> = Vec::new();
    let coords: Vec<(u64, u64)> = input
        .iter()
        .map(|line| {
            let mut parts = line.split(",");
            let x = parts.next().unwrap().parse::<u64>().unwrap();
            let y = parts.next().unwrap().parse::<u64>().unwrap();
            (x, y)
        })
        .collect();
    let mut coords_copy: Vec<(u64, u64)> = coords.clone();

    // Get a vector of perimeter coordinates
    let perimeter_coords: Vec<(u64, u64)> = get_per_coords(&coords);

    for (idx1, coord1) in coords.iter().enumerate() {
        if coords_copy.len() == 0 {
            break;
        }
        coords_copy.remove(0);
        for (idx2, coord2) in coords_copy.iter().enumerate() {
            let (x1, y1) = coord1;
            let (x2, y2) = coord2;

            println!(
                "{}/{}: {}/{}: Checking {},{} against {},{}",
                idx1,
                coords.len(),
                idx2,
                coords_copy.len(),
                x1,
                y1,
                x2,
                y2
            );

            if is_in_perimeter(*x1, *y1, *x2, *y2, &perimeter_coords) {
                valid_recs.push(get_area(*x1, *y1, *x2, *y2));
            }
        }
    }

    valid_recs.sort_unstable();
    println!("Valid Recs: {:?}", valid_recs);
    *valid_recs.last().unwrap()
}

// Helper functions
fn get_area(x1: u64, y1: u64, x2: u64, y2: u64) -> u64 {
    let height = y1.max(y2) - y1.min(y2) + 1;
    let width = x1.max(x2) - x1.min(x2) + 1;
    height * width
}

fn get_per_coords(coords: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut perimeter: HashSet<(u64, u64)> = HashSet::new();
    for i in 0..coords.len() {
        if i == coords.len() - 1 {
            let j = 0;
            let (x1, y1) = coords[i];
            let (x2, y2) = coords[j];

            if x1 == x2 {
                // Vertical line
                for y in y1.min(y2)..=y1.max(y2) {
                    perimeter.insert((x1, y));
                }
            } else {
                // Horizontal line
                for x in x1.min(x2)..=x1.max(x2) {
                    perimeter.insert((x, y1));
                }
            }
        } else {
            let j = i + 1;
            let (x1, y1) = coords[i];
            let (x2, y2) = coords[j];

            if x1 == x2 {
                // Vertical line
                for y in y1.min(y2)..=y1.max(y2) {
                    perimeter.insert((x1, y));
                }
            } else {
                // Horizontal line
                for x in x1.min(x2)..=x1.max(x2) {
                    perimeter.insert((x, y1));
                }
            }
        }
    }

    let perimeter: Vec<(u64, u64)> = perimeter.into_iter().collect();
    // draw_perimeter(&perimeter, (0, 0));
    perimeter
}

fn is_in_perimeter(x1: u64, y1: u64, x2: u64, y2: u64, perimeter_coords: &Vec<(u64, u64)>) -> bool {
    let mut in_bounds: bool = true;

    // We already have two corners of the rectangle, workout the other two corners
    let corners: Vec<(u64, u64)> = vec![(x1, y1), (x2, y1), (x1, y2), (x2, y2)];
    for corner in corners.iter() {
        let corner_x = corner.0;
        let corner_y = corner.1;

        let x_coords: Vec<u64> = perimeter_coords
            .iter()
            .filter(|(coord_x, coord_y)| *coord_y == corner_y)
            .map(|(x, _)| *x)
            .collect();

        let min_x = *x_coords.iter().min().unwrap();
        let max_x = *x_coords.iter().max().unwrap();

        // ONLY UNCOMMENT FOR TEST INPUT
        // draw_perimeter(&perimeter_coords, (corner_x, corner_y));
        // std::thread::sleep(std::time::Duration::from_millis(100));
        // print!("\x1B[2J\x1B[1;1H");

        // Check if we're in bounds
        if !(min_x <= corner_x && corner_x <= max_x) {
            in_bounds = false;
        }
    }
    in_bounds
}

fn draw_perimeter(p: &Vec<(u64, u64)>, plot_point: (u64, u64)) {
    let (mut x_vec, mut y_vec): (Vec<u64>, Vec<u64>) = p.iter().copied().unzip();

    x_vec.sort();
    y_vec.sort();

    let (smallest_x, smallest_y) = (x_vec.iter().take(1), y_vec.iter().take(1));
    let (largest_x, largest_y) = (x_vec.last().unwrap(), y_vec.last().unwrap());

    let height = largest_y + 3;
    let width = largest_x + 3;

    for y in 0..height {
        for x in 0..width {
            print!(
                "{}",
                if (x, y) == plot_point {
                    "@"
                } else if p.contains(&(x, y)) {
                    "#"
                } else {
                    "."
                }
            );
        }
        println!();
    }
}
