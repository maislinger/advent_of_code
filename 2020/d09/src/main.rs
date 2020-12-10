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

fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn is_sum(pos: usize, vals: &[i64], offset: usize) -> bool {
    for (i, j) in iproduct!((pos - offset)..pos, (pos - offset)..pos) {
        if j >= i || vals[i] == vals[j] {
            continue;
        }
        if vals[i] + vals[j] == vals[pos] {
            return true;
        }
    }
    false
}

fn compute_solution_part_one(input: &str) -> i64 {
    let vals = parse_input(input);
    for i in 25..vals.len() {
        if !is_sum(i, &vals, 25) {
            return vals[i];
        }
    }
    panic!("no invalid number found in cipher");
}

fn compute_solution_part_two(input: &str) -> i64 {
    let vals = parse_input(input);
    let number = compute_solution_part_one(input);
    let iter_slice = |i, n_consecutives| vals.iter().skip(i).take(n_consecutives);

    for i in 0..vals.len() {
        let mut n_consecutives = 2;
        loop {
            let sum: i64 = iter_slice(i, n_consecutives).sum();
            if sum < number {
                n_consecutives += 1;
            } else if sum == number {
                return iter_slice(i, n_consecutives).min().unwrap()
                    + iter_slice(i, n_consecutives).max().unwrap();
            } else if sum > number {
                break;
            }
        }
    }
    panic!("no weakness found in cipher");
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d09 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
