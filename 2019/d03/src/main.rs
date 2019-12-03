extern crate regex;
use regex::Regex;

use std::collections::{HashMap, HashSet};

fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

#[derive(Debug)]
enum Direction {
    Right(u64),
    Up(u64),
    Left(u64),
    Down(u64),
}

fn parse_line(line: &str) -> Vec<Direction> {
    let re = Regex::new(r"(R|U|L|D)(\d*)").unwrap();
    re.captures_iter(line)
        .map(|c| {
            let v = c[2].parse().unwrap();
            match &c[1] {
                "R" => Direction::Right(v),
                "U" => Direction::Up(v),
                "L" => Direction::Left(v),
                "D" => Direction::Down(v),
                _ => panic!("unknown direction"),
            }
        })
        .collect()
}

fn parse_input(input: &str) -> Vec<Vec<Direction>> {
    input.lines().map(|l| parse_line(l)).collect()
}

fn directions_to_points(directions: &[Direction]) -> HashMap<(i64, i64), u64> {
    let mut x = 0;
    let mut y = 0;
    let mut steps = 0;
    let mut points = HashMap::new();
    for direction in directions {
        let (delta_x, delta_y, v) = match direction {
            Direction::Right(v) => (1, 0, v),
            Direction::Left(v) => (-1, 0, v),
            Direction::Up(v) => (0, 1, v),
            Direction::Down(v) => (0, -1, v),
        };
        for _ in 0..*v {
            x += delta_x;
            y += delta_y;
            steps += 1;
            points.entry((x, y)).or_insert(steps);
        }
    }
    points
}

fn compute_solution_part_one(input: &str) -> u64 {
    let directions = parse_input(input);
    let lines: Vec<_> = directions.iter().map(|d| directions_to_points(d)).collect();
    let points: Vec<HashSet<_>> = lines.iter().map(|p| p.keys().collect()).collect();
    points[0]
        .intersection(&points[1])
        .map(|(x, y)| (x.abs() + y.abs()) as u64)
        .min()
        .unwrap()
}

fn compute_solution_part_two(input: &str) -> u64 {
    let directions = parse_input(input);
    let lines: Vec<_> = directions.iter().map(|d| directions_to_points(d)).collect();
    let points: Vec<HashSet<_>> = lines.iter().map(|p| p.keys().collect()).collect();
    points[0]
        .intersection(&points[1])
        .map(|c| lines[0][c] + lines[1][c])
        .min()
        .unwrap()
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d03 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
