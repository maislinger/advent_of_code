fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

struct HandheldProcessor {
    instructions: Vec<Instruction>,
    visited_instructions: Vec<bool>,
    instruction_pos: i64,
    accumulator: i64,
}

impl HandheldProcessor {
    fn from_str(input: &str) -> Self {
        let mut instructions = Vec::new();
        for line in input.lines() {
            let mut iter = line.split_whitespace();
            let inst = iter.next().unwrap();
            let v: i64 = iter.next().unwrap().parse().unwrap();
            assert!(iter.next().is_none());

            let instruction = match inst {
                "acc" => Instruction::Acc(v),
                "jmp" => Instruction::Jmp(v),
                "nop" => Instruction::Nop(v),
                _ => panic!("unknown instruction"),
            };
            instructions.push(instruction);
        }

        let n = instructions.len();

        Self {
            instructions,
            visited_instructions: vec![false; n],
            instruction_pos: 0,
            accumulator: 0,
        }
    }

    fn step(&mut self) {
        self.visited_instructions[self.instruction_pos as usize] = true;
        match self.instructions[self.instruction_pos as usize] {
            Instruction::Acc(v) => {
                self.accumulator += v;
                self.instruction_pos += 1;
            }
            Instruction::Jmp(v) => {
                self.instruction_pos += v;
            }
            Instruction::Nop(_) => {
                self.instruction_pos += 1;
            }
        }
    }

    fn run_until_loop_or_outside(&mut self) {
        loop {
            if self.instruction_pos < 0
                || self.instruction_pos >= (self.instructions.len() as i64)
                || self.visited_instructions[self.instruction_pos as usize]
            {
                break;
            }
            self.step()
        }
    }
}

fn compute_solution_part_one(input: &str) -> i64 {
    let mut processor = HandheldProcessor::from_str(input);
    processor.run_until_loop_or_outside();
    processor.accumulator
}

fn compute_solution_part_two(input: &str) -> i64 {
    let mut processor = HandheldProcessor::from_str(input);
    let n = processor.instructions.len();
    for i in 0..n {
        processor = HandheldProcessor::from_str(input);
        match processor.instructions[i] {
            Instruction::Acc(_) => continue,
            Instruction::Jmp(v) => {
                processor.instructions[i] = Instruction::Nop(v);
            }
            Instruction::Nop(v) => {
                processor.instructions[i] = Instruction::Jmp(v);
            }
        }
        processor.run_until_loop_or_outside();
        if processor.instruction_pos == (processor.instructions.len() as i64) {
            return processor.accumulator;
        }
    }
    panic!("no suitable modification found")
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d08 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
