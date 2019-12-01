fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

fn naive_fuel(mass: u64) -> u64 {
    let fuel = mass / 3;
    if fuel > 2 {
        fuel - 2
    } else {
        0
    }
}

fn recursive_fuel(mass: u64) -> u64 {
    let mut fuel = naive_fuel(mass);
    let mut unaccounted_fuel = fuel;
    while unaccounted_fuel > 0 {
        unaccounted_fuel = naive_fuel(unaccounted_fuel);
        fuel += unaccounted_fuel;
    }
    fuel
}

fn compute_solution_part_one(input: &str) -> u64 {
    input
        .lines()
        .map(|s| s.parse::<u64>().unwrap())
        .map(naive_fuel)
        .sum()
}

fn compute_solution_part_two(input: &str) -> u64 {
    input
        .lines()
        .map(|s| s.parse::<u64>().unwrap())
        .map(recursive_fuel)
        .sum()
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
