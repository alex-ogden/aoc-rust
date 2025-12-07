use std::env;

mod utils;
mod y2019;
mod y2022;
mod y2025;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        run_all();
        return;
    }

    let year = &args[1];
    let day = args.get(2).map(|s| s.as_str());
    let part = args.get(3).map(|s| s.as_str());
    let variant = args.get(4).map(|s| s.as_str()); // mt, test, etc.

    match (year.as_str(), day, part, variant) {
        // 2025 with variants
        ("2025", Some("2"), Some("1"), Some("mt")) => y2025::day2mt::part1(),
        ("2025", Some("2"), Some("2"), Some("mt")) => y2025::day2mt::part2(),
        ("2025", Some("2"), None, Some("mt")) => {
            y2025::day2mt::part1();
            y2025::day2mt::part2();
        }

        // Standard 2025 specific parts
        ("2025", Some("1"), Some("1"), None) => y2025::day1::part1(),
        ("2025", Some("1"), Some("2"), None) => y2025::day1::part2(),
        ("2025", Some("2"), Some("1"), None) => y2025::day2::part1(),
        ("2025", Some("2"), Some("2"), None) => y2025::day2::part2(),
        ("2025", Some("3"), Some("1"), None) => y2025::day3::part1(),
        ("2025", Some("3"), Some("2"), None) => y2025::day3::part2(),
        ("2025", Some("4"), Some("1"), None) => y2025::day4::part1(),
        ("2025", Some("4"), Some("2"), None) => y2025::day4::part2(),

        // Whole day
        ("2025", Some("1"), None, None) => {
            y2025::day1::part1();
            y2025::day1::part2();
        }
        ("2025", Some("2"), None, None) => {
            y2025::day2::part1();
            y2025::day2::part2();
            y2025::day2mt::part1();
            y2025::day2mt::part2();
        }
        ("2025", Some("3"), None, None) => {
            y2025::day3::part1();
            y2025::day3::part2();
        }
        ("2025", Some("4"), None, None) => {
            y2025::day4::part1();
            y2025::day4::part2();
        }

        // Whole year
        ("2019", None, None, None) => run_2019(),
        ("2022", None, None, None) => run_2022(),
        ("2025", None, None, None) => run_2025(),

        // 2019 specific parts
        ("2019", Some("1"), Some("1"), None) => y2019::day1::part1(),
        ("2019", Some("1"), Some("2"), None) => y2019::day1::part2(),
        ("2019", Some("2"), Some("1"), None) => y2019::day2::part1(),
        ("2019", Some("2"), Some("2"), None) => y2019::day2::part2(),
        ("2019", Some("3"), Some("1"), None) => y2019::day3::part1(),

        ("2019", Some("1"), None, None) => {
            y2019::day1::part1();
            y2019::day1::part2();
        }
        ("2019", Some("2"), None, None) => {
            y2019::day2::part1();
            y2019::day2::part2();
        }
        ("2019", Some("3"), None, None) => {
            y2019::day3::part1();
        }

        // 2022 specific parts
        ("2022", Some("1"), Some("1"), None) => y2022::day1::part1(),
        ("2022", Some("1"), Some("2"), None) => y2022::day1::part2(),
        ("2022", Some("2"), Some("1"), None) => y2022::day2::part1(),
        ("2022", Some("2"), Some("2"), None) => y2022::day2::part2(),
        ("2022", Some("3"), Some("1"), None) => y2022::day3::part1(),
        ("2022", Some("3"), Some("2"), None) => y2022::day3::part2(),

        ("2022", Some("1"), None, None) => {
            y2022::day1::part1();
            y2022::day1::part2();
        }
        ("2022", Some("2"), None, None) => {
            y2022::day2::part1();
            y2022::day2::part2();
        }
        ("2022", Some("3"), None, None) => {
            y2022::day3::part1();
            y2022::day3::part2();
        }

        _ => println!(
            "Usage: cargo run [year] [day] [part] [variant]\nExamples:\n  cargo run 2025\n  cargo run 2025 2\n  cargo run 2025 2 1\n  cargo run 2025 2 1 mt"
        ),
    }
}

fn run_all() {
    run_2019();
    run_2022();
    run_2025();
}

fn run_2019() {
    y2019::day1::part1();
    y2019::day1::part2();
    y2019::day2::part1();
    y2019::day2::part2();
    y2019::day3::part1();
}

fn run_2022() {
    y2022::day1::part1();
    y2022::day1::part2();
    y2022::day2::part1();
    y2022::day2::part2();
    y2022::day3::part1();
    y2022::day3::part2();
}

fn run_2025() {
    y2025::day1::part1();
    y2025::day1::part2();
    y2025::day2::part1();
    y2025::day2::part2();
    y2025::day2mt::part1();
    y2025::day2mt::part2();
    y2025::day3::part1();
    y2025::day3::part2();
    y2025::day4::part1();
    y2025::day4::part2();
}
