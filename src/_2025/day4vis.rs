use crate::utils;

pub fn part1() {
    let input = utils::read_grid(2025, 4, false);
    let result = solve_part1(&input);
    println!("2025 :: Day 4 :: Part 1: {}", result);
}

pub fn part2() {
    let mut input = utils::read_grid(2025, 4, false);
    let result = solve_part2(&mut input);
    println!("2025 :: Day 4 :: Part 2: {}", result);
}

fn solve_part1(input: &[Vec<String>]) -> u32 {
    let (width, height) = (input[0].len(), input.len());

    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(move |(x, cell)| {
                    cell.as_str() == "@"
                        && utils::get_num_neighbours(input, *x, y, height, width, "@") < 4
                })
                .map(|_| 1)
        })
        .sum()
}

fn solve_part2(input: &mut [Vec<String>]) -> u32 {
    (0..)
        .map(|_| remove_rolls(input))
        .take_while(|&removed| removed > 0)
        .sum()
}

fn remove_rolls(input: &mut [Vec<String>]) -> u32 {
    let (width, height) = (input[0].len(), input.len());

    let mut to_remove = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if input[y][x] == "@" && utils::get_num_neighbours(input, x, y, height, width, "@") < 4
            {
                to_remove.push((x, y));
            }
        }
    }

    let count = to_remove.len() as u32;

    for (x, y) in to_remove {
        input[y][x] = ".".to_string();
    }

    print_grid(input);

    count
}

fn print_grid(grid: &[Vec<String>]) {
    print!("\x1b[2J\x1b[H"); // Clear screen
    let mut rolls_remaining = 0;

    // Process 2 rows at a time to help fit on screen
    for y in (0..grid.len()).step_by(2) {
        for x in 0..grid[0].len() {
            let top = grid[y][x] == "@";
            let bottom = if y + 1 < grid.len() {
                grid[y + 1][x] == "@"
            } else {
                false
            };

            let (ch, adder) = match (top, bottom) {
                (true, true) => ("█", 2),  // Both filled
                (true, false) => ("▀", 1), // Top filled
                (false, true) => ("▄", 1), // Bottom filled
                (false, false) => (".", 0),  // Both empty
            };
            rolls_remaining += adder;
            print!("{}", ch);
        }
        println!();
    }
    println!("Rolls remaining: {}", rolls_remaining);
    std::thread::sleep(std::time::Duration::from_millis(50));
}
