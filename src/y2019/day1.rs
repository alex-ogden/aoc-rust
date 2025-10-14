use crate::utils;

pub fn part1() {
    let input_lines: Vec<String> = utils::read_lines("inputs/2019/day1.txt");
    let result = solve_part1(&input_lines);
    println!("2019 :: Day1 :: Part1: {}", result);
}

pub fn part2() {
    let input_lines: Vec<String> = utils::read_lines("inputs/2019/day1.txt");
    let result = solve_part2(&input_lines);
    println!("2019 :: Day1 :: Part2: {}", result);
}

fn solve_part1(lines: &[String]) -> i32 {
    // For each line:
    //      convert to integer
    //      divide by 3
    //      round down to nearest integer
    //      subtract 2
    //      add result to total
    lines
        .iter()
        .filter_map(|line| line.parse::<i32>().ok()) // convert string to int32, skip failures
        .map(|mass| mass / 3 - 2) // integer division auto-rounds-down
        .sum() // Sum it all up 
}

fn solve_part2(lines: &[String]) -> i32 {
    lines
        .iter()
        .filter_map(|line| line.parse::<i32>().ok())
        .map(|mass| calculate_fuel(mass))
        .sum()

    /*
            Upon doing some more "research", I found this solution which is even fancier...

    fn solve_part2(lines: &[String]) -> i32 {
        lines.iter()
            .filter_map(|line| line.parse::<i32>().ok())
            .map(|mass| {
                std::iter::successors(Some(mass), |&m| {
                    let fuel = m / 3 - 2;
                    (fuel > 0).then_some(fuel)
                })
                .skip(1)  // Skip the initial mass
                .sum::<i32>()
            })
            .sum()
    }

            Very fancy but I've not a clue what's going on.
         */
}

fn calculate_fuel(mass: i32) -> i32 {
    let fuel = mass / 3 - 2; // Work out the fuel needed for a given mass 
    if fuel <= 0 {
        // If the fuel needed is <= 0, return 0
        0
    } else {
        // Otherwise, recursively call the function to add the fuel
        // required
        fuel + calculate_fuel(fuel)
    }
}
