fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

fn pattern(base_pattern: &[i64], r: usize) -> impl Iterator<Item = i64> + '_ {
    base_pattern
        .iter()
        .flat_map(move |&x| std::iter::repeat(x).take(r))
        .cycle()
        .skip(1)
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i64)
        .collect()
}

fn step(state: &[i64], base_pattern: &[i64]) -> Vec<i64> {
    let mut result = Vec::with_capacity(state.len());
    for r in 1..=state.len() {
        let mut v: i64 = state
            .iter()
            .zip(pattern(base_pattern, r))
            .map(|(a, b)| a * b)
            .sum();
        v = v.abs() % 10;
        result.push(v);
    }
    result
}

fn compute_solution_part_one(input: &str) -> String {
    let base_pattern = vec![0, 1, 0, -1];
    let mut state = parse_input(input);

    for _ in 0..100 {
        state = step(&state, &base_pattern);
    }

    state
        .iter()
        .map(|i| i.to_string().chars().nth(0).unwrap())
        .take(8)
        .collect()
}

fn lazy_step(state: &mut [i64]) {
    let lower = state.len() / 2;
    let upper = state.len() - 1;
    let mut sum = 0;

    for index in (lower..=upper).rev() {
        sum += state[index];
        state[index] = sum.abs() % 10;
    }
}

fn compute_solution_part_two(input: &str) -> String {
    let mut state = parse_input(input);
    state = state
        .iter()
        .cycle()
        .take(state.len() * 10000)
        .cloned()
        .collect();
    let index: usize = state
        .iter()
        .take(7)
        .rev()
        .enumerate()
        .map(|(i, x)| (*x as usize) * 10usize.pow(i as u32))
        .sum();

    assert!(index > state.len() / 2, "not implemented");

    for _ in 0..100 {
        lazy_step(&mut state);
    }

    state
        .iter()
        .skip(index)
        .map(|i| i.to_string().chars().nth(0).unwrap())
        .take(8)
        .collect()
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d16 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
