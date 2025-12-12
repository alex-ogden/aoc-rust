use crate::utils;

const MAX_ITERS: usize = 1000;

pub fn part1() {
    let input: Vec<String> = utils::read_lines("inputs/2025/day8.txt");
    let result: i64 = solve_part1(&input);
    println!("2025 :: Day 8 :: Part 1: {}", result);
}

pub fn part2() {
    let input: Vec<String> = utils::read_lines("inputs/2025/day8.txt");
    let result: i64 = solve_part2(&input);
    println!("2025 :: Day 8 :: Part 2: {}", result);
}

fn solve_part1(input: &Vec<String>) -> i64 {
    let mut sorted_slds: Vec<(i64, String)> = Vec::new();
    let mut circuits: Vec<Vec<String>> = Vec::new();
    let mut input_copy: Vec<String> = input.clone();

    // Create circuits for all boxes and get all slds
    // For each coordinate, remove it from the copy vector and compare against what remains
    // Ensuring no duplicate checks are made i.e 1->5 then 5->1
    for coord1 in input.iter().take(input.len() - 1) {
        input_copy.retain(|num_str| num_str != coord1);
        circuits.push(vec![coord1.to_string()]);
        for coord2 in input_copy.iter() {
            if coord1 == coord2 {
                continue;
            }
            let coord1_vec: Vec<i64> = get_vec(&coord1);
            let coord2_vec: Vec<i64> = get_vec(&coord2);
            let sld: i64 = get_sld(&coord1_vec, &coord2_vec);
            sorted_slds.push((sld, format!("{} {}", coord1, coord2)));
        }
    }

    // Sort SLDs
    sorted_slds.sort_unstable();

    for (idx, sld) in sorted_slds.iter().enumerate() {
        if idx == MAX_ITERS {
            break;
        }
        let coords: Vec<&str> = sld.1.split_whitespace().collect();
        let (coord1, coord2) = (coords[0], coords[1]);

        // If the box has a circuit where it's on it's own, remove it (as we're adding it to a new
        // circuit)
        circuits.retain(|circuit| {
            !(circuit.len() == 1 && (circuit[0] == coord1 || circuit[0] == coord2))
        });

        // Find the circuit that the other coord already belongs to
        let mut contains_both: bool = false;
        let mut coord1_circuit_idx: usize = 0;
        let mut coord2_circuit_idx: usize = 0;
        let mut found_coord1_circuit: bool = false;
        let mut found_coord2_circuit: bool = false;

        for (idx, circuit) in circuits.iter_mut().enumerate() {
            // Make sure a circuit doesn't already contain both coords
            if circuit.contains(&coord1.to_string()) && circuit.contains(&coord2.to_string()) {
                contains_both = true;
                break;
            }
            if circuit.contains(&coord1.to_string()) {
                found_coord1_circuit = true;
                coord1_circuit_idx = idx;
            } else if circuit.contains(&coord2.to_string()) {
                found_coord2_circuit = true;
                coord2_circuit_idx = idx;
            }
        }

        if contains_both {
            continue;
        }

        if !found_coord1_circuit && !found_coord2_circuit {
            // If we didn't find a circuit for either, they were on their own and have been removed
            // create a new circuit and add them both to it
            circuits.push(coords.iter().map(|coord| coord.to_string()).collect());
        } else if !found_coord1_circuit {
            // We only found coord2, meaning it was part of a multibox circuit already, add coord1
            // to it
            circuits[coord2_circuit_idx].push(coord1.to_string());
        } else if !found_coord2_circuit {
            // Same thing but the other way around
            circuits[coord1_circuit_idx].push(coord2.to_string());
        } else {
            // Otherwise, we found both, meaning their both in multi-box circuits, merge the two
            // and remove them, create one mega-circuit and add it back
            let mut coord2_circuit = circuits[coord2_circuit_idx].clone();
            circuits[coord1_circuit_idx].append(&mut coord2_circuit);
            circuits.remove(coord2_circuit_idx);
        }
    }

    // Sort in descending order
    circuits.sort_by_key(|circuit| std::cmp::Reverse(circuit.len()));

    // Take the product of the largest 3 circuits
    circuits
        .iter()
        .take(3)
        .map(|circuit| circuit.len() as i64)
        .product()
}

fn solve_part2(input: &Vec<String>) -> i64 {
    let mut sorted_slds: Vec<(i64, String)> = Vec::new();
    let mut circuits: Vec<Vec<String>> = Vec::new();
    let mut input_copy: Vec<String> = input.clone();

    for coord1 in input.iter().take(input.len() - 1) {
        input_copy.retain(|num_str| num_str != coord1);
        circuits.push(vec![coord1.to_string()]);
        for coord2 in input_copy.iter() {
            if coord1 == coord2 {
                continue;
            }
            let coord1_vec: Vec<i64> = get_vec(&coord1);
            let coord2_vec: Vec<i64> = get_vec(&coord2);
            let sld: i64 = get_sld(&coord1_vec, &coord2_vec);
            sorted_slds.push((sld, format!("{} {}", coord1, coord2)));
        }
    }

    sorted_slds.sort_unstable();

    let (mut prev_circuit_len, mut curr_circuit_len) = (circuits.len(), circuits.len());

    for sld in sorted_slds.iter() {
        let coords: Vec<&str> = sld.1.split_whitespace().collect();
        let (coord1, coord2) = (coords[0], coords[1]);

        prev_circuit_len = curr_circuit_len;

        // If the box has a circuit where it's on it's own, remove it (as we're adding it to a new
        // circuit)
        circuits.retain(|circuit| {
            !(circuit.len() == 1 && (circuit[0] == coord1 || circuit[0] == coord2))
        });

        // Find the circuit that the other coord already belongs to
        let mut contains_both: bool = false;
        let mut coord1_circuit_idx: usize = 0;
        let mut coord2_circuit_idx: usize = 0;
        let mut found_coord1_circuit: bool = false;
        let mut found_coord2_circuit: bool = false;

        for (idx, circuit) in circuits.iter_mut().enumerate() {
            // Make sure a circuit doesn't already contain both coords
            if circuit.contains(&coord1.to_string()) && circuit.contains(&coord2.to_string()) {
                contains_both = true;
                break;
            }
            if circuit.contains(&coord1.to_string()) {
                found_coord1_circuit = true;
                coord1_circuit_idx = idx;
            } else if circuit.contains(&coord2.to_string()) {
                found_coord2_circuit = true;
                coord2_circuit_idx = idx;
            }
        }

        if contains_both {
            continue;
        }

        if !found_coord1_circuit && !found_coord2_circuit {
            // If we didn't find a circuit for either, they were on their own and have been removed
            // create a new circuit and add them both to it
            circuits.push(coords.iter().map(|coord| coord.to_string()).collect());
        } else if !found_coord1_circuit {
            // We only found coord2, meaning it was part of a multibox circuit already, add coord1
            // to it
            circuits[coord2_circuit_idx].push(coord1.to_string());
        } else if !found_coord2_circuit {
            // Same thing but the other way around
            circuits[coord1_circuit_idx].push(coord2.to_string());
        } else {
            // Otherwise, we found both, meaning their both in multi-box circuits, merge the two
            // and remove them, create one mega-circuit and add it back
            let mut coord2_circuit = circuits[coord2_circuit_idx].clone();
            circuits[coord1_circuit_idx].append(&mut coord2_circuit);
            circuits.remove(coord2_circuit_idx);
        }

        // Check if we've gone from 2 circuits -> 1 circuit
        // Whatever connection caused that to happen, take the X coords of both boxes and return
        // the product
        curr_circuit_len = circuits.len();
        if curr_circuit_len == 1 && prev_circuit_len == 2 {
            let coord1_vec = get_vec(&coord1.to_string());
            let coord2_vec = get_vec(&coord2.to_string());
            return coord1_vec[0] * coord2_vec[0];
        }
    }

    // We should never get here, so if we get a result of 30, something's gone wrong :)
    30
}

// Helper functions
fn get_vec(coords: &String) -> Vec<i64> {
    coords
        .split(",")
        .map(|num_str| num_str.parse::<i64>().unwrap())
        .collect()
}
fn get_sld(coords1: &Vec<i64>, coords2: &Vec<i64>) -> i64 {
    ((coords1[0] - coords2[0]).pow(2)
        + (coords1[1] - coords2[1]).pow(2)
        + (coords1[2] - coords2[2]).pow(2))
    .isqrt()
}
