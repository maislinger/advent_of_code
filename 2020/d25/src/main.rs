fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

fn parse_input(input: &str) -> (usize, usize) {
    let card_public_key = input.lines().next().unwrap().parse().unwrap();
    let door_public_key = input.lines().nth(1).unwrap().parse().unwrap();
    (card_public_key, door_public_key)
}

fn transform(value: usize, subject_number: usize) -> usize {
    (value * subject_number) % 20_201_227
}

fn find_loop_size(public_key: usize) -> usize {
    let mut loop_nr = 0;
    let mut value = 1;
    let subject_number = 7;
    loop {
        if value == public_key {
            return loop_nr;
        }
        value = transform(value, subject_number);
        loop_nr += 1;
    }
}

fn encrypt(subject_number: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value = transform(value, subject_number);
    }
    value
}

fn compute_solution_part_one(input: &str) -> usize {
    let (card_public_key, door_public_key) = parse_input(input);
    let card_loop_size = find_loop_size(card_public_key);
    let door_loop_size = find_loop_size(door_public_key);
    let encryption_key = encrypt(door_public_key, card_loop_size);
    assert_eq!(encryption_key, encrypt(card_public_key, door_loop_size));
    encryption_key
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d25 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
    }
}
