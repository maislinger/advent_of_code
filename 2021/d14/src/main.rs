use std::collections::HashMap;

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d14 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}

fn compute_solution_part_one(input: &str) -> usize {
    count_max_min(input, 10)
}

fn compute_solution_part_two(input: &str) -> usize {
    count_max_min(input, 40)
}

fn count_max_min(input: &str, turns: usize) -> usize {
    let init_state_vec: Vec<char> = input.lines().next().unwrap().chars().collect();

    let first = *init_state_vec.first().unwrap();
    let last = *init_state_vec.last().unwrap();

    let mut state: HashMap<(char, char), usize> = HashMap::new();
    let mut substitutions = HashMap::new();

    for line in input.lines().skip(2) {
        let mut line_iter = line.chars().filter(|c| *c != ' ' && *c != '-' && *c != '>');
        let a = line_iter.next().unwrap();
        let b = line_iter.next().unwrap();
        let r = line_iter.next().unwrap();
        substitutions.insert((a, b), r);
    }

    for w in init_state_vec.windows(2) {
        let n = state.entry((w[0], w[1])).or_insert(0);
        *n += 1;
    }

    for _ in 0..turns {
        let mut new_state: HashMap<(char, char), usize> = HashMap::new();

        for (k, v) in state.iter() {
            let a = k.0;
            let b = k.1;

            match substitutions.get(&(a, b)) {
                Some(&c) => {
                    let n = new_state.entry((a, c)).or_insert(0);
                    *n += v;
                    let n = new_state.entry((c, b)).or_insert(0);
                    *n += v;
                }
                None => {
                    let n = new_state.entry((a, b)).or_insert(0);
                    *n += v;
                }
            }
        }

        state = new_state;
    }

    let mut counter: HashMap<char, usize> = HashMap::new();
    counter.insert(first, 1);
    counter.insert(last, 1);
    for (&(a, b), v) in state.iter() {
        let n = counter.entry(a).or_insert(0);
        *n += v;
        let n = counter.entry(b).or_insert(0);
        *n += v;
    }

    let max = counter.iter().map(|(_, v)| v).max().unwrap();
    let min = counter.iter().map(|(_, v)| v).min().unwrap();

    (max - min) / 2
}

fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}
