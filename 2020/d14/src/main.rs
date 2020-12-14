#[macro_use]
extern crate lazy_static;

use regex::Regex;
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

struct BitMaskSystem {
    mask_or: u64,
    mask_and: u64,
    memory: BTreeMap<usize, u64>,
}

impl BitMaskSystem {
    fn new(mask_or: u64, mask_and: u64, memory: BTreeMap<usize, u64>) -> Self {
        Self {
            mask_or,
            mask_and,
            memory,
        }
    }

    fn write_memory(&mut self, address: usize, value: u64) {
        self.memory
            .insert(address, (value | self.mask_or) & self.mask_and);
    }

    fn write_mask(&mut self, mask: &str) {
        let mut mask_and = 0;
        let mut mask_or = 0;
        for (i, c) in mask.chars().enumerate() {
            if i >= 36 {
                panic!("mask longer than 36 bit");
            }
            mask_and <<= 1;
            mask_or <<= 1;
            match c {
                '1' => {
                    mask_and |= 0x0000_0000_0000_0001;
                    mask_or |= 0x0000_0000_0000_0001;
                }
                '0' => {
                    mask_and &= 0x0000_000f_ffff_fffe;
                    mask_or &= 0x0000_000f_ffff_fffe;
                }
                'X' => {
                    mask_and |= 0x0000_0000_0000_0001;
                    mask_or &= 0x0000_000f_ffff_fffe;
                }
                _ => panic!("invalid bitmask"),
            }
        }
        self.mask_and = mask_and;
        self.mask_or = mask_or;
    }

    fn parse_instruction(&mut self, instruction: &str) {
        lazy_static! {
            static ref RE_MASK: Regex = Regex::new(r"mask = ([01X]*)").unwrap();
            static ref RE_VALUE: Regex = Regex::new(r"mem\[(\d*)\] = (\d*)").unwrap();
        }

        if RE_MASK.is_match(instruction) {
            let cap = RE_MASK.captures_iter(instruction).next().unwrap();
            self.write_mask(&cap[1]);
        } else if RE_VALUE.is_match(instruction) {
            let cap = RE_VALUE.captures_iter(instruction).next().unwrap();
            let address = cap[1].parse().unwrap();
            let value = cap[2].parse().unwrap();
            self.write_memory(address, value);
        } else {
            panic!("unknown instruction")
        }
    }

    fn sum_of_memory(&self) -> u64 {
        self.memory.iter().map(|(_, v)| v).sum()
    }
}

impl Default for BitMaskSystem {
    fn default() -> Self {
        let mask_or = 0x0000_0000_0000_0000;
        let mask_and = 0x0000_000f_ffff_ffff;
        Self::new(mask_or, mask_and, BTreeMap::new())
    }
}

struct BitMaskSystemV2 {
    mask_template: String,
    memory: BTreeMap<usize, u64>,
}

impl BitMaskSystemV2 {
    fn new(mask_template: String, memory: BTreeMap<usize, u64>) -> Self {
        Self {
            mask_template,
            memory,
        }
    }

    fn write_memory(&mut self, address: usize, value: u64) {
        assert_eq!(self.mask_template.chars().count(), 36);
        let n_x = self.mask_template.chars().filter(|c| *c == 'X').count() as u32;

        for x in 0..2u64.pow(n_x) {
            let mut address = address.rotate_right(36) as u64;
            let mut x = x.rotate_right(n_x) as u64;
            let mut real_address: u64 = 0;
            for c in self.mask_template.chars() {
                real_address <<= 1;
                address = address.rotate_left(1);
                match c {
                    '0' => {
                        real_address = (real_address & 0x0000_000f_ffff_fffe)
                            | (address & 0x0000_0000_0000_0001);
                    }
                    '1' => {
                        real_address |= 0x0000_0000_0000_0001;
                    }
                    'X' => {
                        x = x.rotate_left(1);
                        real_address =
                            (real_address & 0x0000_000f_ffff_fffe) | (x & 0x0000_0000_0000_0001)
                    }
                    _ => panic!("invalid bitmask"),
                }
            }
            self.memory.insert(real_address as usize, value);
        }
    }

    fn write_mask(&mut self, mask: &str) {
        self.mask_template = mask.to_string();
    }

    fn parse_instruction(&mut self, instruction: &str) {
        lazy_static! {
            static ref RE_MASK: Regex = Regex::new(r"mask = ([01X]*)").unwrap();
            static ref RE_VALUE: Regex = Regex::new(r"mem\[(\d*)\] = (\d*)").unwrap();
        }

        if RE_MASK.is_match(instruction) {
            let cap = RE_MASK.captures_iter(instruction).next().unwrap();
            self.write_mask(&cap[1]);
        } else if RE_VALUE.is_match(instruction) {
            let cap = RE_VALUE.captures_iter(instruction).next().unwrap();
            let address = cap[1].parse().unwrap();
            let value = cap[2].parse().unwrap();
            self.write_memory(address, value);
        } else {
            panic!("unknown instruction")
        }
    }

    fn sum_of_memory(&self) -> u64 {
        self.memory.iter().map(|(_, v)| v).sum()
    }
}

impl Default for BitMaskSystemV2 {
    fn default() -> Self {
        Self::new("".to_string(), BTreeMap::new())
    }
}

fn compute_solution_part_one(input: &str) -> u64 {
    let mut bit_mask_system = BitMaskSystem::default();
    for line in input.lines() {
        bit_mask_system.parse_instruction(line);
    }
    bit_mask_system.sum_of_memory()
}

fn compute_solution_part_two(input: &str) -> u64 {
    let mut bit_mask_system = BitMaskSystemV2::default();
    for line in input.lines() {
        bit_mask_system.parse_instruction(line);
    }
    bit_mask_system.sum_of_memory()
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d14 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
