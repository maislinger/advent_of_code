use std::collections::BTreeMap;

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
    let mut result: Vec<i64> = input.lines().map(|l| l.parse().unwrap()).collect();
    result.sort_unstable();
    result
}

fn compute_solution_part_one(input: &str) -> i64 {
    let joltage_ratings = parse_input(input);
    let mut n_one_difference = 0;
    let mut n_three_difference = 0;
    match joltage_ratings[0] {
        1 => n_one_difference += 1,
        3 => n_three_difference += 1,
        _ => panic!("invalid lowest joltage"),
    }
    for difference in joltage_ratings.windows(2).map(|w| w[1] - w[0]) {
        match difference {
            1 => n_one_difference += 1,
            3 => n_three_difference += 1,
            _ => panic!("unexpected difference"),
        };
    }
    n_one_difference * (n_three_difference + 1)
}

fn compute_solution_part_two(input: &str) -> usize {
    let joltage_ratings = parse_input(input);
    let mut n_ways: BTreeMap<i64, usize> = BTreeMap::new();
    n_ways.insert(0, 1);
    for j in joltage_ratings.iter() {
        let mut count = 0;
        for diff in [3, 2, 1].iter() {
            if n_ways.contains_key(&(j - diff)) {
                count += n_ways.get(&(j - diff)).unwrap();
            }
        }
        n_ways.insert(*j, count);
    }
    *n_ways.get(joltage_ratings.last().unwrap()).unwrap()
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d10 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
