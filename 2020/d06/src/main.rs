use std::collections::BTreeSet;

fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

fn compute_solution_part_one(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let set: BTreeSet<_> = group
                .split_whitespace()
                .flat_map(|line| line.chars())
                .collect();
            set.len()
        })
        .sum()
}

fn compute_solution_part_two(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let initial_set: BTreeSet<_> = group
                .split_whitespace()
                .map(|line| line.chars().collect())
                .next()
                .unwrap();
            let set = group
                .split_whitespace()
                .map(|line| line.chars().collect())
                .skip(1)
                .fold(initial_set, |acc, s| {
                    acc.intersection(&s).cloned().collect()
                });
            set.len()
        })
        .sum()
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
