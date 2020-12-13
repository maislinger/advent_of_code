fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

fn parse_input(input: &str) -> (i64, Vec<Option<i64>>) {
    let current_time = input.lines().next().unwrap().parse().unwrap();
    let bus_ids = input
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|id_str| {
            if id_str == "x" {
                None
            } else {
                Some(id_str.parse().unwrap())
            }
        })
        .collect();
    (current_time, bus_ids)
}

// https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn compute_solution_part_one(input: &str) -> i64 {
    let wait_time = |current_time: i64, bus_id: i64| bus_id - current_time % bus_id;
    let (current_time, bus_ids) = parse_input(input);
    let min_id = bus_ids
        .iter()
        .filter(|bus_id| bus_id.is_some())
        .min_by_key(|bus_id| wait_time(current_time, bus_id.unwrap()))
        .unwrap()
        .unwrap();
    wait_time(current_time, min_id) * min_id
}

fn compute_solution_part_two(input: &str) -> i64 {
    let (_, bus_ids) = parse_input(input);
    let wait_times: Vec<i64> = bus_ids
        .iter()
        .enumerate()
        .filter(|(_, bus_id)| bus_id.is_some())
        .map(|(i, _)| i as i64)
        .collect();
    let bus_ids: Vec<i64> = bus_ids
        .iter()
        .filter(|bus_id| bus_id.is_some())
        .map(|bus_id| bus_id.unwrap())
        .collect();
    let residues: Vec<i64> = wait_times
        .iter()
        .zip(bus_ids.iter())
        .map(|(w, b)| b - w % b)
        .collect();
    chinese_remainder(&residues, &bus_ids).unwrap()
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d13 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
