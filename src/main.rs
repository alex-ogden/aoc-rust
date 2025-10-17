mod utils;
mod y2019;
mod y2022;

fn main() {
    // Run all solutions
    // 2019
    y2019::day1::part1();
    y2019::day1::part2();
    y2019::day2::part1();
    y2019::day2::part2();
    y2019::day3::part1();

    // 2022
    y2022::day1::part1();
    y2022::day1::part2();
    y2022::day2::part1();
    y2022::day2::part2();
    y2022::day3::part1();
    y2022::day3::part2();
}
