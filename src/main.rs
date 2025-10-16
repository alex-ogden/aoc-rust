mod utils;
mod y2019;
mod y2022;

fn main() {
    // Run all solutions
    // 2019
    println!("Running: 2019 Day1 Part1");
    y2019::day1::part1();
    println!("Running: 2019 Day1 Part2");
    y2019::day1::part2();
    println!("Running: 2019 Day2 Part1");
    y2019::day2::part1();
    println!("Running: 2019 Day2 Part2");
    y2019::day2::part2();
    println!("Running: 2019 Day3 Part1");
    y2019::day3::part1();

    // 2022
    println!("Running: 2022 Day1 Part1");
    y2022::day1::part1();
    println!("Running: 2022 Day1 Part2");
    y2022::day1::part2();
}
