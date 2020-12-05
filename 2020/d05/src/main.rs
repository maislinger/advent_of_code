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

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .split_whitespace()
        .map(|line| {
            let mut row: usize = 0;
            let mut col: usize = 0;
            for c in line.chars() {
                match c {
                    'F' | 'B' => row <<= 1,
                    'R' | 'L' => col <<= 1,
                    _ => unreachable!(),
                };
                match c {
                    'B' => row += 1,
                    'R' => col += 1,
                    _ => (),
                };
            }
            (row, col)
        })
        .collect()
}

fn boarding_pass_to_id(pass: &(usize, usize)) -> usize {
    pass.0 * 8 + pass.1
}

fn compute_solution_part_one(input: &str) -> usize {
    let boarding_passes = parse_input(input);
    boarding_passes
        .iter()
        .map(boarding_pass_to_id)
        .max()
        .unwrap()
}

fn compute_solution_part_two(input: &str) -> usize {
    let boarding_passes = parse_input(input);
    let boarding_ids: BTreeSet<usize> = boarding_passes
        .iter()
        .filter(|(r, _)| *r != 0 && *r != 127)
        .map(boarding_pass_to_id)
        .collect();
    for id in &boarding_ids {
        if !boarding_ids.contains(&(id - 1)) && boarding_ids.contains(&(id - 2)) {
            return id - 1;
        }
    }
    panic!("no boarding pass found")
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d05 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
