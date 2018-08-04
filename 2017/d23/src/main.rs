#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::BTreeMap;

#[derive(Clone, Debug)]
enum InstructionValue {
    Number(i64),
    Name(String),
}

impl InstructionValue {
    fn parse(s: &str) -> InstructionValue {
        let v = s.parse::<i64>();
        match v {
            Ok(x) => InstructionValue::Number(x),
            _ => InstructionValue::Name(s.to_owned()),
        }
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    Set((String, InstructionValue)),
    Sub((String, InstructionValue)),
    Mul((String, InstructionValue)),
    Jnz((InstructionValue, InstructionValue)),
}

struct Machine {
    registers: BTreeMap<String, i64>,
    instructions: Vec<Instruction>,
    next_inst: Option<usize>,
    finished: bool,
    muls_called: usize,
    default_a: i64,
}

impl Machine {
    fn new(instructions: Vec<Instruction>, default_a: i64) -> Machine {
        Machine {
            registers: BTreeMap::new(),
            instructions,
            next_inst: Some(0),
            finished: false,
            muls_called: 0,
            default_a,
        }
    }

    fn set_reg(&mut self, name: String, val: i64) {
        self.registers.insert(name, val);
    }

    fn get_reg(&self, name: &str) -> i64 {
        match self.registers.get(name) {
            Some(x) => *x,
            None => if name == "a" { self.default_a } else { 0 },
        }
    }

    fn parse_instruction_value(&self, val: &InstructionValue) -> i64 {
        match *val {
            InstructionValue::Number(x) => x,
            InstructionValue::Name(ref x) => self.get_reg(x),
        }
    }

    fn increase_inst(&mut self) {
        let mut next_inst = self.next_inst.unwrap();
        next_inst += 1;
        self.next_inst = if next_inst < self.instructions.len() {
            Some(next_inst)
        } else {
            self.finished = true;
            None
        };
    }

    fn set(&mut self, reg: String, val: &InstructionValue) {
        let newval = self.parse_instruction_value(val);
        self.set_reg(reg, newval);
    }

    fn sub(&mut self, reg: String, val: &InstructionValue) {
        let decrement = self.parse_instruction_value(val);
        let newval = self.get_reg(&reg) - decrement;
        self.set_reg(reg, newval);
    }

    fn mul(&mut self, reg: String, val: &InstructionValue) {
        let multiplier = self.parse_instruction_value(val);
        let newval = self.get_reg(&reg) * multiplier;
        self.set_reg(reg, newval);
        self.muls_called += 1;
    }

    fn jnz(&mut self, x: &InstructionValue, y: &InstructionValue) {
        let vx = self.parse_instruction_value(x);
        if vx != 0 {
            let vy = self.parse_instruction_value(y);
            let neg = vy < 0;
            let vy = vy.abs() as usize;
            let next_inst = self.next_inst.unwrap();
            self.next_inst = if neg && vy < next_inst {
                Some(next_inst - vy)
            } else if !neg && next_inst + vy < self.instructions.len() {
                Some(next_inst + vy)
            } else {
                self.finished = true;
                None
            };
        } else {
            self.increase_inst();
        }
    }

    fn run_instruction(&mut self) {
        let next_inst = self.next_inst.unwrap();
        let increase = match self.instructions[next_inst] {
            Instruction::Jnz((_, _)) => false,
            _ => true,
        };

        match self.instructions[next_inst].clone() {
            Instruction::Set((x, y)) => self.set(x, &y),
            Instruction::Sub((x, y)) => self.sub(x, &y),
            Instruction::Mul((x, y)) => self.mul(x, &y),
            Instruction::Jnz((x, y)) => self.jnz(&x, &y),
        }

        if increase {
            self.increase_inst();
        }
    }
}

fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect(
        "something went wrong reading the file",
    );
    contents.trim().to_owned()
}

fn convert_input(input: &str) -> Vec<Instruction> {
    use regex::Regex;

    lazy_static! {
        static ref RE_SET: Regex = Regex::new("set ([a-z]+) (-?[0-9]+|[a-z]+)").unwrap();
        static ref RE_SUB: Regex = Regex::new("sub ([a-z]+) (-?[0-9]+|[a-z]+)").unwrap();
        static ref RE_MUL: Regex = Regex::new("mul ([a-z]+) (-?[0-9]+|[a-z]+)").unwrap();
        static ref RE_JNZ: Regex = Regex::new("jnz (-?[0-9]+|[a-z]+) (-?[0-9]+|[a-z]+)").unwrap();
    }

    let mut result = Vec::new();

    for s in input.split('\n') {
        let s = s.trim();
        if let Some(cap) = RE_SET.captures(s) {
            let x = cap[1].to_owned();
            let y = InstructionValue::parse(&cap[2]);
            result.push(Instruction::Set((x, y)));
        } else if let Some(cap) = RE_SUB.captures(s) {
            let x = cap[1].to_owned();
            let y = InstructionValue::parse(&cap[2]);
            result.push(Instruction::Sub((x, y)));
        } else if let Some(cap) = RE_MUL.captures(s) {
            let x = cap[1].to_owned();
            let y = InstructionValue::parse(&cap[2]);
            result.push(Instruction::Mul((x, y)));
        } else if let Some(cap) = RE_JNZ.captures(s) {
            let x = InstructionValue::parse(&cap[1]);
            let y = InstructionValue::parse(&cap[2]);
            result.push(Instruction::Jnz((x, y)));
        } else {
            unreachable!();
        }
    }

    result
}

fn is_prime(number: u64) -> bool {
    let mut d = 2;
    while d * d <= number {
        if number % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn compute_solution_part_one(input: &str) -> usize {
    let instructions = convert_input(input);
    let mut machine = Machine::new(instructions, 0);
    while !machine.finished {
        machine.run_instruction();
    }
    machine.muls_called
}

fn compute_solution_part_two() -> u64 {
    // This is the solution of my input only.
    // A general solution would take too long.

    let mut h = 0;
    for n in (109_900..=126_900).step_by(17) {
        if !is_prime(n) {
            h += 1;
        }
    }
    h
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d23 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two());
    }
}
