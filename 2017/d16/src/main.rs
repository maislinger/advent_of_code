#[macro_use]
extern crate lazy_static;
extern crate regex;

#[derive(Debug)]
enum Instruction {
    Spin(usize),
    Exchange((usize, usize)),
    Partner((char, char)),
    Permute(Vec<usize>), // permute indices
    Table(Vec<u8>), // permute values
}

struct Programs {
    vals: Vec<u8>,
}

impl Programs {
    fn new(n: u8) -> Programs {
        let vals = (0..n).collect();
        Programs { vals: vals }
    }

    fn spin(&mut self, n: usize) {
        let newvec = {
            let len = self.vals.len();
            let (left, right) = self.vals.split_at(len - n);
            right.iter().chain(left.iter()).cloned().collect()
        };
        self.vals = newvec;
    }

    fn exchange(&mut self, a: usize, b: usize) {
        self.vals.swap(a, b);
    }

    fn char_to_u8(a: char) -> u8 {
        a as u8 - b'a'
    }

    fn u8_to_char(a: u8) -> char {
        (a + b'a') as char
    }

    fn partner(&mut self, a: char, b: char) {
        let a = Programs::char_to_u8(a);
        let b = Programs::char_to_u8(b);
        let a_ind = self.vals.iter().position(|v| *v == a).unwrap();
        let b_ind = self.vals.iter().position(|v| *v == b).unwrap();
        self.exchange(a_ind, b_ind);
    }

    fn permute(&mut self, inds: &[usize]) {
        self.vals = inds.iter().map(|i| self.vals[*i]).collect();
    }

    fn table(&mut self, vals: &[u8]) {
        self.vals
            .iter_mut()
            .map(|element| *element = vals[*element as usize])
            .count();
    }

    fn run_instruction(&mut self, instruction: &Instruction) {
        match *instruction {
            Instruction::Spin(n) => self.spin(n),
            Instruction::Exchange((a, b)) => self.exchange(a, b),
            Instruction::Partner((a, b)) => self.partner(a, b),
            Instruction::Permute(ref inds) => self.permute(inds),
            Instruction::Table(ref vals) => self.table(vals),
        }
    }

    fn to_string(&self) -> String {
        self.vals.iter().map(|i| Programs::u8_to_char(*i)).collect()
    }
}

fn compress_instructions(len: u8, instructions: &[Instruction]) -> Vec<Instruction> {
    let mut programs = Programs::new(len);
    let mut programs_partner = Programs::new(len);
    let mut result = Vec::new();

    for instruction in instructions {
        match *instruction {
            Instruction::Partner(_) => {
                programs_partner.run_instruction(instruction);
            }
            _ => {
                programs.run_instruction(instruction);
            }
        }
    }

    let vals = programs.vals.iter().map(|i| *i as usize).collect();
    result.push(Instruction::Permute(vals));
    result.push(Instruction::Table(programs_partner.vals));

    result
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
        static ref RE_SP: Regex = Regex::new(r"s([0-9]+)").unwrap();
        static ref RE_EX: Regex = Regex::new(r"x([0-9]+)/([0-9]+)").unwrap();
        static ref RE_PA: Regex = Regex::new(r"p([a-z])/([a-z])").unwrap();
    }

    let mut result = Vec::new();

    for s in input.split(',') {
        if let Some(cap) = RE_SP.captures(s) {
            let tmp = cap[1].parse().unwrap();
            result.push(Instruction::Spin(tmp));
        } else if let Some(cap) = RE_EX.captures(s) {
            let tmp = cap[1].parse().unwrap();
            let tmp2 = cap[2].parse().unwrap();
            result.push(Instruction::Exchange((tmp, tmp2)));
        } else if let Some(cap) = RE_PA.captures(s) {
            let tmp = cap[1].chars().nth(0).unwrap();
            let tmp2 = cap[2].chars().nth(0).unwrap();
            result.push(Instruction::Partner((tmp, tmp2)));
        } else {
            unreachable!();
        }
    }

    result
}

fn compute_solution_part_one(input: &str, len: u8) -> String {
    let instructions = convert_input(input);
    let mut programs = Programs::new(len);

    for instruction in &instructions {
        programs.run_instruction(instruction);
    }

    programs.to_string()
}

fn compute_solution_part_two(input: &str, len: u8) -> String {
    let instructions = convert_input(input);
    let instructions = compress_instructions(len, &instructions);
    let mut programs = Programs::new(len);

    let mut i = 0;

    loop {
        for instruction in &instructions {
            programs.run_instruction(instruction);
        }
        i += 1;
        if programs.vals == (0..len).collect::<Vec<u8>>() {
            break;
        }
    }

    let runs = 1_000_000_000 % i;

    for _ in 0..runs {
        for instruction in &instructions {
            programs.run_instruction(instruction);
        }
    }

    programs.to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use compute_solution_part_one;
        use compute_solution_part_two;

        let input = "s1,x3/4,pe/b";
        let solution = compute_solution_part_one(input, 5);
        assert_eq!(solution, "baedc");

        let solution = compute_solution_part_two(input, 5);
        assert_eq!(solution, "abcde");

    }
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d16 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input, 16));
        println!("solution 2 = {}", compute_solution_part_two(&input, 16));
    }
}
