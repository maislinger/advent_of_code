fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

struct Position {
    x: i64,
    depth: i64,
    aim: i64,
}

impl Position {
    fn apply_command(&mut self, c: &Command) {
        match c {
            Command::Forward(n) => self.x += n,
            Command::Up(n) => self.depth -= n,
            Command::Down(n) => self.depth += n,
        }
    }

    fn apply_command_with_aim(&mut self, c: &Command) {
        match c {
            Command::Forward(n) => {
                self.x += n;
                self.depth += n * self.aim;
            }
            Command::Up(n) => self.aim -= n,
            Command::Down(n) => self.aim += n,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            x: 0,
            depth: 0,
            aim: 0,
        }
    }
}

enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

fn parse_input(input: &str) -> Vec<Command> {
    let mut result = Vec::new();
    for line in input.split('\n') {
        let mut iter = line.split(' ');
        let cs = iter.next().unwrap();
        let amount: i64 = iter.next().unwrap().parse().unwrap();
        let c = match cs {
            "forward" => Command::Forward(amount),
            "up" => Command::Up(amount),
            "down" => Command::Down(amount),
            _ => panic!("unknown command"),
        };
        result.push(c);
    }
    result
}

fn compute_solution_part_one(input: &str) -> i64 {
    let commands = parse_input(input);
    let mut p = Position::default();
    for c in commands.iter() {
        p.apply_command(c);
    }
    p.x * p.depth
}

fn compute_solution_part_two(input: &str) -> i64 {
    let commands = parse_input(input);
    let mut p = Position::default();
    for c in commands.iter() {
        p.apply_command_with_aim(c);
    }
    p.x * p.depth
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d02 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
