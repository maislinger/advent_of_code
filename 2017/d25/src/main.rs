extern crate regex;

use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

struct Instruction {
    write_value: bool,
    dir: Direction,
    next_state: u8,
}

impl Instruction {
    fn new(write_value: bool, dir: Direction, next_state: u8) -> Self {
        Instruction {
            write_value,
            dir,
            next_state,
        }
    }
}

struct TuringMachine {
    state: u8,
    position: usize,
    tape: VecDeque<bool>,
    instructions: BTreeMap<(u8, bool), Instruction>,
}

impl TuringMachine {
    fn new(state: u8, instructions: BTreeMap<(u8, bool), Instruction>) -> Self {
        let mut tape = VecDeque::new();
        tape.push_back(false);
        let position = 0;
        TuringMachine {
            state,
            position,
            tape,
            instructions,
        }
    }

    fn step(&mut self) {
        let instructions = std::mem::replace(&mut self.instructions, BTreeMap::new());
        {
            let instruction = &instructions[&(self.state, self.tape[self.position])];
            self.parse_instruction(instruction);
            self.trim();
        }
        self.instructions = instructions;
    }

    fn parse_instruction(&mut self, instruction: &Instruction) {
        self.tape[self.position] = instruction.write_value;
        self.state = instruction.next_state;
        match instruction.dir {
            Direction::Left => {
                if self.position == 0 {
                    self.tape.push_front(false);
                } else {
                    self.position -= 1;
                }
            }
            Direction::Right => {
                if self.position == self.tape.len() - 1 {
                    self.tape.push_back(false);
                }
                self.position += 1;
            }
        }
    }

    fn trim(&mut self) {
        while self.tape.len() > 1 {
            if !self.tape[0] && self.position != 0 {
                self.tape.pop_front();
                self.position -= 1;
            } else if !self.tape[self.tape.len() - 1] && self.position != self.tape.len() - 1 {
                self.tape.pop_back();
            } else {
                break;
            }
        }
        if self.tape.capacity() > self.tape.len() * 2 {
            self.tape.shrink_to_fit();
        }
    }

    fn count_ones(&self) -> usize {
        self.tape.iter().filter(|&&s| s).count()
    }
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

fn convert_input(input: &str) -> (TuringMachine, usize) {
    use regex::Regex;

    let re: Regex = Regex::new(r"Begin in state ([A-Za-z0-9]+)\.").unwrap();
    let begin_state = re
        .captures_iter(input)
        .map(|cap| cap[1].to_owned())
        .nth(0)
        .unwrap();

    let re: Regex = Regex::new(r"Perform a diagnostic checksum after (\d+) steps\.").unwrap();
    let checksum_after: usize = re
        .captures_iter(input)
        .map(|cap| cap[1].to_owned().parse().unwrap())
        .nth(0)
        .unwrap();

    let re: Regex = Regex::new(r"In state\s([A-Z]+):").unwrap();
    let mut states = BTreeMap::new();
    let mut id: u8 = 0;
    for cap in re.captures_iter(input) {
        let statename = cap[1].to_owned();
        states.entry(statename).or_insert_with(|| {
            id += 1;
            id - 1
        });
    }

    let current_states: Vec<_> = re.captures_iter(input).map(|cap| states[&cap[1]]).collect();

    let re: Regex = Regex::new(r"If the current value is (0|1):").unwrap();
    let current_values: Vec<_> = re
        .captures_iter(input)
        .map(|cap| match &cap[1] {
            "0" => false,
            "1" => true,
            _ => unreachable!(),
        }).collect();

    let re: Regex = Regex::new(r"- Write the value (0|1).").unwrap();
    let next_values: Vec<_> = re
        .captures_iter(input)
        .map(|cap| match &cap[1] {
            "0" => false,
            "1" => true,
            _ => unreachable!(),
        }).collect();

    let re: Regex = Regex::new(r"- Move one slot to the ((?:right)|(?:left)).").unwrap();
    let directions: Vec<_> = re
        .captures_iter(input)
        .map(|cap| match &cap[1] {
            "left" => Direction::Left,
            "right" => Direction::Right,
            _ => unreachable!(),
        }).collect();

    let re: Regex = Regex::new(r"- Continue with state ([A-Z]+).").unwrap();
    let next_states: Vec<_> = re.captures_iter(input).map(|cap| states[&cap[1]]).collect();

    let mut instructions = BTreeMap::new();

    for i in 0..next_states.len() {
        let current_state = current_states[i / 2];
        let current_value = current_values[i];
        let next_value = next_values[i];
        let dir = directions[i];
        let next_state = next_states[i];
        let instruction = Instruction::new(next_value, dir, next_state);
        instructions.insert((current_state, current_value), instruction);
    }

    let begin_state = states[&begin_state];
    (
        TuringMachine::new(begin_state, instructions),
        checksum_after,
    )
}

fn compute_solution_part_one(input: &str) -> usize {
    let (mut machine, check_after) = convert_input(input);
    for _ in 0..check_after {
        machine.step();
    }
    machine.count_ones()
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use compute_solution_part_one;
        let input = "Begin in state A.
                     Perform a diagnostic checksum after 6 steps.

                     In state A:
                     If the current value is 0:
                         - Write the value 1.
                         - Move one slot to the right.
                         - Continue with state B.
                     If the current value is 1:
                         - Write the value 0.
                         - Move one slot to the left.
                         - Continue with state B.

                     In state B:
                     If the current value is 0:
                         - Write the value 1.
                         - Move one slot to the left.
                         - Continue with state A.
                     If the current value is 1:
                         - Write the value 1.
                         - Move one slot to the right.
                         - Continue with state A.";
        let solution = compute_solution_part_one(input);
        assert_eq!(solution, 3);
    }
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d22 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
    }
}
