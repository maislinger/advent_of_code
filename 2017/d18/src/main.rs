#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::{BTreeMap, VecDeque};

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
    Snd(InstructionValue),
    Set((String, InstructionValue)),
    Add((String, InstructionValue)),
    Mul((String, InstructionValue)),
    Mod((String, InstructionValue)),
    Rcv(String),
    Jgz((InstructionValue, InstructionValue)),
}

struct Machine {
    registers: BTreeMap<String, i64>,
    instructions: Vec<Instruction>,
    next_inst: Option<usize>,
    signal_queue: VecDeque<i64>,
    default_p: i64,
    wait: bool,
    output: Option<i64>,
    sent_values: usize,
}

impl Machine {
    fn new(instructions: Vec<Instruction>, default_p: i64) -> Machine {
        Machine {
            registers: BTreeMap::new(),
            instructions: instructions,
            next_inst: Some(0),
            signal_queue: VecDeque::new(),
            default_p: default_p,
            wait: false,
            output: None,
            sent_values: 0,
        }
    }

    fn set_reg(&mut self, name: String, val: i64) {
        self.registers.insert(name, val);
    }

    fn get_reg(&self, name: &str) -> i64 {
        match self.registers.get(name) {
            Some(x) => *x,
            None => if name == "p" { self.default_p } else { 0 },
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
            None
        };
    }

    fn snd(&mut self, val: &InstructionValue) {
        self.output = Some(self.parse_instruction_value(val));
    }

    fn set(&mut self, reg: String, val: &InstructionValue) {
        let newval = self.parse_instruction_value(val);
        self.set_reg(reg, newval);
    }

    fn add(&mut self, reg: String, val: &InstructionValue) {
        let increment = self.parse_instruction_value(val);
        let newval = self.get_reg(&reg) + increment;
        self.set_reg(reg, newval);
    }

    fn mul(&mut self, reg: String, val: &InstructionValue) {
        let multiplier = self.parse_instruction_value(val);
        let newval = self.get_reg(&reg) * multiplier;
        self.set_reg(reg, newval);
    }

    fn modulo(&mut self, reg: String, val: &InstructionValue) {
        let divisor = self.parse_instruction_value(val);
        let newval = self.get_reg(&reg) % divisor;
        self.set_reg(reg, newval);
    }

    fn rcv(&mut self, reg: String) {
        if self.signal_queue.is_empty() {
            self.wait = true;
        } else {
            let newval = self.signal_queue.pop_front().unwrap();
            self.set_reg(reg, newval);
            self.wait = false;
        }
    }

    fn jgz(&mut self, x: &InstructionValue, y: &InstructionValue) {
        let vx = self.parse_instruction_value(x);
        if vx > 0 {
            let vy = self.parse_instruction_value(y);
            let neg = vy < 0;
            let vy = vy.abs() as usize;
            let next_inst = self.next_inst.unwrap();
            self.next_inst = if neg && vy < next_inst {
                Some(next_inst - vy)
            } else if !neg && next_inst + vy < self.instructions.len() {
                Some(next_inst + vy)
            } else {
                None
            };
        } else {
            self.increase_inst();
        }
    }

    fn rvc_from(&mut self, other: &mut Machine) {
        if let Some(output) = other.output {
            self.signal_queue.push_back(output);
            other.output = None;
            other.sent_values += 1;
        }
    }

    fn run_instruction(&mut self) {
        let next_inst = self.next_inst.unwrap();
        let increase = match self.instructions[next_inst] {
            Instruction::Jgz((_, _)) => false,
            _ => true,
        };

        match self.instructions[next_inst].clone() {
            Instruction::Snd(x) => self.snd(&x),
            Instruction::Set((x, y)) => self.set(x, &y),
            Instruction::Add((x, y)) => self.add(x, &y),
            Instruction::Mul((x, y)) => self.mul(x, &y),
            Instruction::Mod((x, y)) => self.modulo(x, &y),
            Instruction::Rcv(x) => self.rcv(x),
            Instruction::Jgz((x, y)) => self.jgz(&x, &y),
        }

        if increase && !self.wait {
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
        static ref RE_SND: Regex = Regex::new("snd (-?[0-9]+|[a-z]+)").unwrap();
        static ref RE_SET: Regex = Regex::new("set ([a-z]+) (-?[0-9]+|[a-z]+)").unwrap();
        static ref RE_ADD: Regex = Regex::new("add ([a-z]+) (-?[0-9]+|[a-z]+)").unwrap();
        static ref RE_MUL: Regex = Regex::new("mul ([a-z]+) (-?[0-9]+|[a-z]+)").unwrap();
        static ref RE_MOD: Regex = Regex::new("mod ([a-z]+) (-?[0-9]+|[a-z]+)").unwrap();
        static ref RE_RCV: Regex = Regex::new("rcv ([a-z]+)").unwrap();
        static ref RE_JGZ: Regex = Regex::new("jgz (-?[0-9]+|[a-z]+) (-?[0-9]+|[a-z]+)").unwrap();
    }

    let mut result = Vec::new();

    for s in input.split('\n') {
        let s = s.trim();
        if let Some(cap) = RE_SND.captures(s) {
            let x = InstructionValue::parse(&cap[1]);
            result.push(Instruction::Snd(x));
        } else if let Some(cap) = RE_SET.captures(s) {
            let x = cap[1].to_owned();
            let y = InstructionValue::parse(&cap[2]);
            result.push(Instruction::Set((x, y)));
        } else if let Some(cap) = RE_ADD.captures(s) {
            let x = cap[1].to_owned();
            let y = InstructionValue::parse(&cap[2]);
            result.push(Instruction::Add((x, y)));
        } else if let Some(cap) = RE_MUL.captures(s) {
            let x = cap[1].to_owned();
            let y = InstructionValue::parse(&cap[2]);
            result.push(Instruction::Mul((x, y)));
        } else if let Some(cap) = RE_MOD.captures(s) {
            let x = cap[1].to_owned();
            let y = InstructionValue::parse(&cap[2]);
            result.push(Instruction::Mod((x, y)));
        } else if let Some(cap) = RE_RCV.captures(s) {
            let x = cap[1].to_owned();
            result.push(Instruction::Rcv(x));
        } else if let Some(cap) = RE_JGZ.captures(s) {
            let x = InstructionValue::parse(&cap[1]);
            let y = InstructionValue::parse(&cap[2]);
            result.push(Instruction::Jgz((x, y)));
        } else {
            unreachable!();
        }
    }

    result
}

fn compute_solution_part_one(input: &str) -> i64 {
    let instructions = convert_input(input);
    let mut machine = Machine::new(instructions, 0);
    machine.signal_queue.push_back(0);
    loop {
        if let Some(output) = machine.output {
            machine.signal_queue[0] = output;
        }
        let next_inst = machine.next_inst.unwrap();
        if let Instruction::Rcv(_) = machine.instructions[next_inst].clone() {
            break machine.signal_queue.pop_back().unwrap();
        }
        machine.run_instruction();
    }
}

fn compute_solution_part_two(input: &str) -> usize {
    let instructions = convert_input(input);
    let mut machine_0 = Machine::new(instructions.clone(), 0);
    let mut machine_1 = Machine::new(instructions.clone(), 1);

    loop {
        machine_0.rvc_from(&mut machine_1);
        machine_1.rvc_from(&mut machine_0);
        machine_0.run_instruction();
        machine_1.run_instruction();

        if machine_0.wait && machine_1.wait {
            break machine_1.sent_values;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use compute_solution_part_one;
        use compute_solution_part_two;

        let input = "set a 1
                     add a 2
                     mul a a
                     mod a 5
                     snd a
                     set a 0
                     rcv a
                     jgz a -1
                     set a 1
                     jgz a -2";

        let solution = compute_solution_part_one(input);
        assert_eq!(solution, 4);

        let solution = compute_solution_part_two(input);
        assert_eq!(solution, 1);
    }
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d18 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
