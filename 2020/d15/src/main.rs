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

fn parse_input(input: &str) -> BTreeMap<usize, usize> {
    let mut result = BTreeMap::new();
    for (i, number_str) in input.split(',').enumerate() {
        result.insert(number_str.parse().unwrap(), i + 1);
    }
    result
}

fn play_game_until(max_turn: usize, input: &str) -> usize {
    let mut numbers = parse_input(input);
    let (last_number, max_input_turn) = numbers.iter().max_by_key(|(_, v)| *v).unwrap();
    let mut last_number = *last_number;

    for turn in (max_input_turn + 1)..=max_turn {
        let next_number = numbers.get(&last_number).map(|l| turn - 1 - l).unwrap_or(0);
        numbers.insert(last_number, turn - 1);
        last_number = next_number;
    }
    last_number
}

fn compute_solution_part_one(input: &str) -> usize {
    play_game_until(2020, input)
}

fn compute_solution_part_two(input: &str) -> usize {
    play_game_until(30000000, input)
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d15 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
