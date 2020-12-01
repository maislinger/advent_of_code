#[macro_use]
extern crate itertools;

fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn compute_solution_part_one(input: &str) -> usize {
    let data = parse_input(input);
    for (i, j) in iproduct!(0..data.len(), 0..data.len()) {
        if j <= i {
            continue;
        }
        if data[i] + data[j] == 2020 {
            return data[i] * data[j];
        }
    }
    panic!("no combination adding to 2020 found in input");
}

fn compute_solution_part_two(input: &str) -> usize {
    let data = parse_input(input);
    for (i, j, k) in iproduct!(0..data.len(), 0..data.len(), 0..data.len()) {
        if j <= i || k <= j {
            continue;
        }
        if data[i] + data[j] + data[k] == 2020 {
            return data[i] * data[j] * data[k];
        }
    }
    panic!("no combination adding to 2020 found in input");
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d01 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
