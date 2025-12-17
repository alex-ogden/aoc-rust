use std::env;

mod _2015;
mod _2025;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        run_all();
        return;
    }

    let year = &args[1];
    let day = args.get(2).and_then(|s| s.parse::<usize>().ok());
    let part = args.get(3).and_then(|s| s.parse::<usize>().ok());

    match year.as_str() {
        "2015" => run_year(2015, &_2015::PART1, &_2015::PART2, day, part),
        "2025" => run_year(2025, &_2025::PART1, &_2025::PART2, day, part),
        _ => println!("Unknown year"),
    }
}

fn run_year(year: u32, part1: &[fn()], part2: &[fn()], day: Option<usize>, part: Option<usize>) {
    match (day, part) {
        (None, None) => {
            for i in 0..part1.len() {
                run_timed(year, i + 1, 1, part1[i]);
                run_timed(year, i + 1, 2, part2[i]);
            }
        }
        (Some(d), None) => {
            run_timed(year, d, 1, part1[d - 1]);
            run_timed(year, d, 2, part2[d - 1]);
        }
        (Some(d), Some(1)) => run_timed(year, d, 1, part1[d - 1]),
        (Some(d), Some(2)) => run_timed(year, d, 2, part2[d - 1]),
        _ => println!("Invalid arguments"),
    }
}

fn run_timed(year: u32, day: usize, part: usize, f: fn()) {
    let label = format!("{} Day {} Part {}", year, day, part);
    utils::time(&label, f);
}

fn run_all() {
    run_year(2015, &_2015::PART1, &_2015::PART2, None, None);
    run_year(2025, &_2025::PART1, &_2025::PART2, None, None);
}
