use std::fs;

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path)
        .expect("Failed to read file")
}

pub fn read_lines(path: &str) -> Vec<String> {
    // Breaks string (returned by read_input) into lines, maps the lines to strings
    // and collects into an iterable.
    read_input(path)
        .lines()
        .map(|s| s.to_string())
        .collect()
}
