use std::collections::VecDeque;

#[derive(Clone)]
pub struct IntCodeMachine {
    pub data: Vec<i64>,
    pub input_signals: VecDeque<i64>,
    pub output_signals: Vec<i64>,
    pub instruction_pointer: usize,
    pub relative_base_offset: i64,
    pub halted: bool,
    pub keep_last_input: bool,
}

impl IntCodeMachine {
    pub fn from_string(input: &str) -> Self {
        let data = input.split(',').map(|s| s.parse().unwrap()).collect();
        Self {
            data,
            input_signals: VecDeque::new(),
            output_signals: Vec::new(),
            instruction_pointer: 0,
            relative_base_offset: 0,
            halted: false,
            keep_last_input: true,
        }
    }

    pub fn step(&mut self) {
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
            9 => self.adjust_relative_base(),
            99 => self.halted = true,
            _ => panic!("invalid opcode"),
        }
    }

    pub fn add_input_signal(&mut self, input_signal: i64) {
        self.input_signals.push_back(input_signal);
    }

    pub fn last_output_signal(&self) -> Option<i64> {
        self.output_signals.last().cloned()
    }

    pub fn run_until_halt(&mut self) {
        while !self.halted {
            self.step();
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

    fn read_value(&mut self, offset: usize) -> i64 {
        let ip = self.instruction_pointer;
        let mode = self.mode(offset);
        if mode == 0 || mode == 2 {
            let addr = if mode == 0 {
                self.data[ip + offset]
            } else {
                self.data[ip + offset] + self.relative_base_offset
            };
            assert!(addr >= 0);
            let addr = addr as usize;
            if addr >= self.data.len() {
                self.data.resize(addr + 1, 0);
            }
            self.data[addr]
        } else if mode == 1 {
            self.data[ip + offset]
        } else {
            panic!("unknown mode");
        }
    }

    fn write_value(&mut self, new_value: i64, offset: usize) {
        let ip = self.instruction_pointer;
        let addr = if self.mode(offset) == 0 {
            self.data[ip + offset]
        } else if self.mode(offset) == 2 {
            self.data[ip + offset] + self.relative_base_offset
        } else {
            panic!("unknown mode");
        };
        assert!(addr >= 0);
        let addr = addr as usize;
        if addr >= self.data.len() {
            self.data.resize(addr + 1, 0);
        }
        self.data[addr] = new_value;
    }

    fn add(&mut self) {
        let v1 = self.read_value(1);
        let v2 = self.read_value(2);
        let v = v1 + v2;
        self.write_value(v, 3);
        self.instruction_pointer += 4;
    }

    fn mul(&mut self) {
        let v1 = self.read_value(1);
        let v2 = self.read_value(2);
        let v = v1 * v2;
        self.write_value(v, 3);
        self.instruction_pointer += 4;
    }

    fn input(&mut self) {
        if self.input_signals.len() == 0 {
            panic!("no available input signals")
        }
        let v = if self.input_signals.len() == 1 && self.keep_last_input {
            self.input_signals[0]
        } else {
            self.input_signals.pop_front().unwrap()
        };
        self.write_value(v, 1);
        self.instruction_pointer += 2;
    }

    fn output(&mut self) {
        let v = self.read_value(1);
        self.output_signals.push(v);
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
        let v1 = self.read_value(1);
        let v2 = self.read_value(2);
        let v = if v1 < v2 { 1 } else { 0 };
        self.write_value(v, 3);
        self.instruction_pointer += 4;
    }

    fn equals(&mut self) {
        let v1 = self.read_value(1);
        let v2 = self.read_value(2);
        let v = if v1 == v2 { 1 } else { 0 };
        self.write_value(v, 3);
        self.instruction_pointer += 4;
    }

    fn adjust_relative_base(&mut self) {
        let delta = self.read_value(1);
        self.relative_base_offset += delta;
        self.instruction_pointer += 2;
    }
}
