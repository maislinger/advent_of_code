fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

fn parse_input(input: &str) -> (u32, u32) {
    let mut parts = input.split('-');
    let lower = parts.next().unwrap().parse().unwrap();
    let upper = parts.next().unwrap().parse().unwrap();
    (lower, upper)
}

fn password_to_vec(password: u32) -> Vec<u32> {
    password
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn is_increasing(password: &Vec<u32>) -> bool {
    for d in password.windows(2) {
        if d[0] > d[1] {
            return false;
        }
    }
    true
}

fn has_double(password: &Vec<u32>) -> bool {
    for d in password.windows(2) {
        if d[0] == d[1] {
            return true;
        }
    }
    false
}

fn has_isolated_double(password: &Vec<u32>) -> bool {
    let mut count = 0;
    for d in password.windows(2) {
        if d[0] == d[1] {
            count += 1;
        } else if count == 1 {
            return true;
        } else {
            count = 0;
        }
    }
    count == 1
}

fn compute_solution_part_one(input: &str) -> usize {
    let (lower, upper) = parse_input(input);
    (lower..=upper)
        .map(password_to_vec)
        .filter(is_increasing)
        .filter(has_double)
        .count()
}

fn compute_solution_part_two(input: &str) -> usize {
    let (lower, upper) = parse_input(input);
    (lower..=upper)
        .map(password_to_vec)
        .filter(is_increasing)
        .filter(has_isolated_double)
        .count()
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d04 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
