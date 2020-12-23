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

#[derive(Debug)]
struct GameState {
    successors: BTreeMap<usize, usize>,
    current_cup: usize,
    min_cup: usize,
    max_cup: usize,
}

impl GameState {
    fn new(input: &str) -> Self {
        let cups: Vec<usize> = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        let min_cup = *cups.iter().min().unwrap();
        let max_cup = *cups.iter().max().unwrap();
        let current_cup = cups[0];
        let mut successors = BTreeMap::new();
        successors.insert(cups[cups.len() - 1], current_cup);

        for (&a, &b) in cups.iter().zip(cups.iter().skip(1)) {
            successors.insert(a, b);
        }

        Self {
            successors,
            current_cup,
            min_cup,
            max_cup,
        }
    }

    fn step(&mut self) {
        let first_moved = self.successors[&self.current_cup];
        let mut last_moved = first_moved;
        let mut moved_cups = [first_moved, 0, 0];
        for m in moved_cups.iter_mut().skip(1) {
            last_moved = self.successors[&last_moved];
            *m = last_moved;
        }
        let first_unmoved = self.successors[&last_moved];

        let mut destination_cup = if self.current_cup == self.min_cup {
            self.max_cup
        } else {
            self.current_cup - 1
        };

        while moved_cups.iter().any(|&c| c == destination_cup) {
            destination_cup = if destination_cup == self.min_cup {
                self.max_cup
            } else {
                destination_cup - 1
            };
        }

        let destination_cup_successor = self.successors[&destination_cup];

        self.successors.insert(self.current_cup, first_unmoved);
        self.successors.insert(destination_cup, first_moved);
        self.successors
            .insert(last_moved, destination_cup_successor);
        self.current_cup = self.successors[&self.current_cup];
    }

    fn cup_string(&self, base_cup: usize) -> String {
        let mut result = "".to_string();
        let mut successor = self.successors[&base_cup];
        while successor != base_cup {
            result.push_str(&successor.to_string());
            successor = self.successors[&successor];
        }
        result
    }

    fn increase_cups_to(&mut self, new_max: usize) {
        let mut c = self.current_cup;
        let mut s = self.successors[&c];
        while s != self.current_cup {
            c = s;
            s = self.successors[&c];
        }
        while self.max_cup < new_max {
            self.max_cup += 1;
            self.successors.insert(c, self.max_cup);
            c = self.max_cup;
        }
        self.successors.insert(self.max_cup, self.current_cup);
    }
}

fn compute_solution_part_one(input: &str) -> String {
    let mut game_state = GameState::new(input);
    for _ in 0..100 {
        game_state.step();
    }
    game_state.cup_string(1)
}

fn compute_solution_part_two(input: &str) -> usize {
    let mut game_state = GameState::new(input);
    game_state.increase_cups_to(1_000_000);
    for _ in 0..10_000_000 {
        game_state.step();
    }
    let a = game_state.successors[&1];
    let b = game_state.successors[&a];
    a * b
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d22 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
