use crate::utils;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn parse_directions(s: &str) -> (char, i32) {
    let direction = s.chars().next().unwrap();
    let distance = s[1..].parse::<i32>().unwrap();
    (direction, distance)
}

fn get_wire_points(path: &str) -> HashSet<Point> {
    let mut points = HashSet::new();
    let mut current = Point::new(0, 0);

    for instruction in path.split(",") {
        let (direction, distance) = parse_directions(instruction.trim());

        let (dy, dx) = match direction {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, -1),
            'D' => (0, 1),
            _ => panic!("Unknown direction: {}", direction),
        };

        for _ in 0..distance {
            current.x += dx;
            current.y += dy;
            points.insert(current);
        }
    }

    points
}

fn find_closest_intersection(wire1: &str, wire2: &str) -> Option<i32> {
    let points1 = get_wire_points(wire1);
    let points2 = get_wire_points(wire2);

    // Get all intersections
    let intersections: Vec<&Point> = points1.intersection(&points2).collect();

    // Find smallest manhattan distance
    intersections.iter().map(|p| p.manhattan_distance()).min()
}

pub fn part1() {
    let input: Vec<String> = utils::read_lines("inputs/2019/day3.txt");
    let result: i32 = solve_part1(&input);
    println!("2019 :: Day3 :: Part1: {}", result);
}

fn solve_part1(wire_paths: &Vec<String>) -> i32 {
    let wire1 = &wire_paths[0];
    let wire2 = &wire_paths[1];

    let distance = find_closest_intersection(&wire1, &wire2).unwrap();
    distance
}
