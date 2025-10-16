use std::fs;

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read file")
}

pub fn read_lines(path: &str) -> Vec<String> {
    // Breaks string (returned by read_input) into lines, maps the lines to strings
    // and collects into an iterable.
    read_input(path).lines().map(|s| s.to_string()).collect()
}

pub fn read_grid(path: &str) -> Vec<Vec<String>> {
    // Takes a path, uses read_lines() to get a vector of strings, then
    // breaks each line into a vector of strings, appending those to the main
    // vector to return a 2D grid of strings
    read_lines(path)
        .iter()
        .map(|line| line.chars().map(|c| c.to_string()).collect())
        .collect()
}

pub fn read_grid_by_char(path: &str, delimiter: char) -> Vec<Vec<String>> {
    read_lines(path)
        .iter()
        .map(|line| line.split(delimiter).map(|s| s.to_string()).collect())
        .collect()
}
