fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

fn parse_input(input: &str) -> [usize; 9] {
    let mut result = [0; 9];
    for n in input.split(',') {
        let index: usize = n.parse().unwrap();
        result[index] += 1;
    }
    result
}

fn step(state: &[usize; 9]) -> [usize; 9] {
    let mut new_state = [0; 9];
    new_state[..(state.len() - 1)].clone_from_slice(&state[1..]);
    new_state[8] += state[0];
    new_state[6] += state[0];
    new_state
}

fn compute_solution_part_one(input: &str) -> usize {
    let mut state = parse_input(input);
    for _ in 0..80 {
        state = step(&state);
    }
    state.iter().sum()
}

fn compute_solution_part_two(input: &str) -> usize {
    let mut state = parse_input(input);
    for _ in 0..256 {
        state = step(&state);
    }
    state.iter().sum()
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d06 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
