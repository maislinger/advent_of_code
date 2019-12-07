use itertools::Itertools;

fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

struct IntCodeMachine {
    data: Vec<i64>,
    input_signal: i64,
    phase_setting: Option<i64>,
    instruction_pointer: usize,
    output_signal: Option<i64>,
    halted: bool,
}

impl IntCodeMachine {
    fn new(input: &str, input_signal: i64, phase_setting: i64) -> Self {
        let data = input.split(',').map(|s| s.parse().unwrap()).collect();
        Self {
            data,
            input_signal,
            phase_setting: Some(phase_setting),
            instruction_pointer: 0,
            output_signal: None,
            halted: false,
        }
    }

    fn from_existing(&self, input_signal: i64, phase_setting: i64) -> Self {
        Self {
            data: self.data.clone(),
            input_signal,
            phase_setting: Some(phase_setting),
            instruction_pointer: 0,
            output_signal: None,
            halted: false,
        }
    }

    fn mode(&self, offset: usize) -> i64 {
        let mut instruction = self.data[self.instruction_pointer];
        instruction /= 100;
        for _ in 1..offset {
            instruction /= 10;
        }
        instruction % 10
    }

    fn step(&mut self) {
        let instruction = self.data[self.instruction_pointer];
        match instruction % 100 {
            1 => self.add(),
            2 => self.mul(),
            3 => self.input(),
            4 => self.output(),
            5 => self.jmp_true(),
            6 => self.jmp_false(),
            7 => self.less_than(),
            8 => self.equals(),
            99 => self.halted = true,
            _ => panic!("invalid opcode"),
        }
    }

    fn read_value(&mut self, offset: usize) -> i64 {
        let ip = self.instruction_pointer;
        if self.mode(offset) == 0 {
            let addr = self.data[ip + offset];
            assert!(addr >= 0);
            let addr = addr as usize;
            self.data[addr]
        } else {
            self.data[ip + offset]
        }
    }

    fn add(&mut self) {
        let ip = self.instruction_pointer;
        let v1 = self.read_value(1);
        let v2 = self.read_value(2);
        assert!(self.data[ip + 3] >= 0);
        let addr = self.data[ip + 3] as usize;
        self.data[addr] = v1 + v2;
        self.instruction_pointer += 4;
    }

    fn mul(&mut self) {
        let ip = self.instruction_pointer;
        let v1 = self.read_value(1);
        let v2 = self.read_value(2);
        assert!(self.data[ip + 3] >= 0);
        let addr = self.data[ip + 3] as usize;
        self.data[addr] = v1 * v2;
        self.instruction_pointer += 4;
    }

    fn input(&mut self) {
        let ip = self.instruction_pointer;
        assert_eq!(self.mode(1), 0);
        assert_eq!(self.mode(2), 0);
        let addr = self.data[ip + 1] as usize;
        if let Some(x) = self.phase_setting {
            self.data[addr] = x;
            self.phase_setting = None;
        } else {
            self.data[addr] = self.input_signal;
        }
        self.instruction_pointer += 2;
    }

    fn output(&mut self) {
        let v = self.read_value(1);
        self.output_signal = Some(v);
        self.instruction_pointer += 2;
    }

    fn jmp_true(&mut self) {
        let v1 = self.read_value(1);
        let v2 = self.read_value(2);
        if v1 != 0 {
            assert!(v2 >= 0);
            let addr = v2 as usize;
            self.instruction_pointer = addr;
        } else {
            self.instruction_pointer += 3;
        }
    }

    fn jmp_false(&mut self) {
        let v1 = self.read_value(1);
        let v2 = self.read_value(2);
        if v1 == 0 {
            assert!(v2 >= 0);
            let addr = v2 as usize;
            self.instruction_pointer = addr;
        } else {
            self.instruction_pointer += 3;
        }
    }

    fn less_than(&mut self) {
        let ip = self.instruction_pointer;
        let v1 = self.read_value(1);
        let v2 = self.read_value(2);
        assert!(self.data[ip + 3] >= 0);
        let addr = self.data[ip + 3] as usize;
        self.data[addr] = if v1 < v2 { 1 } else { 0 };
        self.instruction_pointer += 4;
    }

    fn equals(&mut self) {
        let ip = self.instruction_pointer;
        let v1 = self.read_value(1);
        let v2 = self.read_value(2);
        assert!(self.data[ip + 3] >= 0);
        let addr = self.data[ip + 3] as usize;
        self.data[addr] = if v1 == v2 { 1 } else { 0 };
        self.instruction_pointer += 4;
    }

    fn run_until_halted(&mut self) {
        while !self.halted {
            self.step();
        }
    }

    fn run_until_output_or_halt(&mut self) {
        while !self.halted && self.output_signal.is_none() {
            self.step();
        }
    }
}

fn compute_thrust(machine: &IntCodeMachine, phase_settings: Vec<i64>) -> i64 {
    let mut io = 0;
    for phase_setting in phase_settings {
        let mut local_machine = machine.from_existing(io, phase_setting);
        local_machine.run_until_halted();
        io = local_machine.output_signal.unwrap();
    }
    io
}

fn compute_loop_thrust(machine: &IntCodeMachine, phase_settings: Vec<i64>) -> i64{
    let mut machines: Vec<_> = phase_settings.iter().map(|p| machine.from_existing(0, *p)).collect();
    let mut machine_index = 0;
    let mut io = 0;
    let mut result = None;
    loop {
        let m = &mut machines[machine_index];
        m.input_signal = io;
        m.run_until_output_or_halt();
        if m.halted {
            break;
        }
        io = m.output_signal.unwrap();
        if machine_index == phase_settings.len() - 1 {
            result = Some(io);
        }
        m.output_signal = None;
        machine_index += 1;
        if machine_index >= phase_settings.len() {
            machine_index = 0;
        }
    }
    result.unwrap()
}

fn compute_solution_part_one(input: &str) -> i64 {
    let machine = IntCodeMachine::new(input, 0, 0);
    (0..=4).permutations(5).map(|c| compute_thrust(&machine, c)).max().unwrap()
}

fn compute_solution_part_two(input: &str) -> i64 {
    let machine = IntCodeMachine::new(input, 0, 0);
    (5..=9).permutations(5).map(|c| compute_loop_thrust(&machine, c)).max().unwrap()
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d07 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
