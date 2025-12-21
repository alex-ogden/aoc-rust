use crate::utils;

// Trying using enums for this problem, don't think I use them often enough
#[derive(Copy, Clone)]
enum Action {
    On,
    Off,
    Toggle,
}

// Same here with structs, don't use them as much as I should
struct Instruction {
    action: Action,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

pub fn part1() {
    let input: Vec<String> = utils::read_lines(2015, 6, false);
    let result: u64 = solve_part1(&input);
    println!("2015 :: Day 6 :: Part 1: {}", result);
}
pub fn part2() {
    let input: Vec<String> = utils::read_lines(2015, 6, false);
    let result: u64 = solve_part2(&input);
    println!("2015 :: Day 6 :: Part 2: {}", result);
}

fn parse_input(input: &[String]) -> Vec<Instruction> {
    // Parse lines into a vector of Instructions using byte-level checking instead of string
    // checking
    input
        .iter()
        .map(|line| {
            let bytes = line.as_bytes();
            // Determine action and start index of coordinates
            let (action, start_idx) = if bytes.starts_with(b"turn on") {
                (Action::On, 8)
            } else if bytes.starts_with(b"turn off") {
                (Action::Off, 9)
            } else {
                (Action::Toggle, 7)
            };

            let rest = &line[start_idx..];
            let (p1, p2) = rest.split_once(" through ").unwrap();
            let (x1, y1) = p1.split_once(',').unwrap();
            let (x2, y2) = p2.split_once(',').unwrap();

            Instruction {
                action,
                x1: x1.parse().unwrap(),
                y1: y1.parse().unwrap(),
                x2: x2.parse().unwrap(),
                y2: y2.parse().unwrap(),
            }
        })
        .collect()
}

pub fn solve_part1(input: &[String]) -> u64 {
    let instructions = parse_input(input);
    let mut lights = Box::new([0u8; 1_000_000]);

    // Since part1 is about setting 1 or 0 or toggling the value, we can use .fill() to efficiently
    // set entire rows to be 0 or 1 (we still loop through for toggling)
    for inst in instructions {
        for y in inst.y1..=inst.y2 {
            let row = &mut lights[y * 1000 + inst.x1..=y * 1000 + inst.x2];
            match inst.action {
                Action::On => row.fill(1),
                Action::Off => row.fill(0),
                Action::Toggle => {
                    for val in row {
                        *val ^= 1;
                    }
                }
            }
        }
    }
    lights.iter().map(|&v| v as u64).sum()
}

pub fn solve_part2(input: &[String]) -> u64 {
    let instructions = parse_input(input);
    let mut lights = Box::new([0u16; 1_000_000]);

    // Because in part2 we are +='ing and -='ing by 1 (and += 2 for toggle), we can't use .fill() so it'll be
    // _slightly_ slower
    for inst in instructions {
        for y in inst.y1..=inst.y2 {
            let row: &mut [u16] = &mut lights[y * 1000 + inst.x1..=y * 1000 + inst.x2];
            match inst.action {
                Action::On => {
                    for v in row {
                        *v += 1;
                    }
                }
                Action::Off => {
                    for v in row {
                        *v = v.saturating_sub(1); // Using saturating_sub() to avoid checking to
                                                  // prevent integer underflow
                    }
                }
                Action::Toggle => {
                    for v in row {
                        *v += 2;
                    }
                }
            }
        }
    }
    lights.iter().map(|&v| v as u64).sum() // Sum up and return the total brightness
}
