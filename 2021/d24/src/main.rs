// The code in this file works under the assumption that x and y get set to zero after every inp,
// which is mapped to w.

use std::collections::{BTreeSet, VecDeque};

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d24 <input filename>");
    } else {
        let input = read_file(&args[1]);
        run(&input);
    }
}

fn run(input: &str) {
    let mut inputs: Vec<Vec<Instruction>> = input
        .split("inp")
        .filter(|s| !s.trim().is_empty())
        .map(|s| {
            let mut s = s.to_owned();
            s = "inp".to_owned() + &s;
            s
        })
        .map(|s| {
            s.lines()
                .filter(|l| !l.is_empty())
                .map(|l| Instruction::from_str(l))
                .collect()
        })
        .collect();

    assert_eq!(inputs.len(), 14);
    inputs.reverse();

    let zmax = 100000;

    let mut z_targets = vec![BTreeSet::from([0])];

    for instructions in &inputs {
        let mut new_target = BTreeSet::new();
        for z in 0..zmax {
            for k in 1..10 {
                let inputs = VecDeque::from([k]);
                let mut alu = ALU::new(inputs, 0, 0, 0, z);

                for instruction in instructions {
                    alu.perform(*instruction);
                }

                let targets = z_targets.last().unwrap();
                for zt in targets.iter() {
                    if alu.z == *zt {
                        new_target.insert(z);
                    }
                }
            }
        }

        z_targets.push(new_target);
    }

    z_targets.reverse();
    inputs.reverse();

    fn print_solution(
        inputs: &[Vec<Instruction>],
        z_targets: &[BTreeSet<i64>],
        do_max: bool,
        prefix: &str,
    ) {
        let mut z = 0;
        let mut result_vec = Vec::new();

        for (instructions, targets) in inputs.iter().zip(z_targets.iter().skip(1)) {
            let mut next_z = None;

            let (k0, k1, delta_k) = if do_max {
                (10i64, 0i64, -1i64)
            } else {
                (0, 10, 1)
            };

            let mut k = k0;

            loop {
                k += delta_k;
                if k == k1 {
                    break;
                }

                let inputs = VecDeque::from([k]);
                let mut alu = ALU::new(inputs, 0, 0, 0, z);

                for instruction in instructions {
                    alu.perform(*instruction);
                }

                if targets.contains(&alu.z) {
                    result_vec.push(k);
                    next_z = Some(alu.z);
                    break;
                }
            }

            z = next_z.unwrap();
        }

        let mut result = "".to_owned();
        for k in &result_vec {
            result += &k.to_string();
        }

        println!("{}{}", prefix, result);
    }

    print_solution(&inputs, &z_targets, true, "Solution 1 = ");
    print_solution(&inputs, &z_targets, false, "Solution 2 = ");
}

struct ALU {
    inputs: VecDeque<i64>,
    w: i64,
    x: i64,
    y: i64,
    z: i64,
    verbose: bool,
}

impl ALU {
    fn new(inputs: VecDeque<i64>, w: i64, x: i64, y: i64, z: i64) -> Self {
        Self {
            inputs,
            w,
            x,
            y,
            z,
            verbose: false,
        }
    }

    fn get(&self, v: VariableOrValue) -> i64 {
        match v {
            VariableOrValue::Value(n) => n,
            VariableOrValue::Variable(q) => match q {
                Variable::W => self.w,
                Variable::X => self.x,
                Variable::Y => self.y,
                Variable::Z => self.z,
            },
        }
    }

    fn get_mut(&mut self, v: Variable) -> &mut i64 {
        match v {
            Variable::W => &mut self.w,
            Variable::X => &mut self.x,
            Variable::Y => &mut self.y,
            Variable::Z => &mut self.z,
        }
    }

    fn maybe_print_info(&self, s: &str) {
        if !self.verbose {
            return;
        }

        print!("{} ", s);
        println!("w: {}\tx: {}\ty: {}\tz: {}", self.w, self.x, self.y, self.z);
    }

    fn inp(&mut self, v: Variable) {
        *self.get_mut(v) = self.inputs.pop_front().unwrap();
        self.maybe_print_info("inp");
    }

    fn add(&mut self, a: Variable, b: VariableOrValue) {
        *self.get_mut(a) += self.get(b);
        self.maybe_print_info("add");
    }

    fn mul(&mut self, a: Variable, b: VariableOrValue) {
        *self.get_mut(a) *= self.get(b);
        self.maybe_print_info("mul");
    }

    fn div(&mut self, a: Variable, b: VariableOrValue) {
        *self.get_mut(a) /= self.get(b);
        self.maybe_print_info("div");
    }

    fn modulo(&mut self, a: Variable, b: VariableOrValue) {
        let q = self.get(b);
        let p = self.get_mut(a);
        assert!(*p >= 0);
        assert!(q > 0);

        *p %= q;
        self.maybe_print_info("mod");
    }

    fn eql(&mut self, a: Variable, b: VariableOrValue) {
        let c = *self.get_mut(a) == self.get(b);
        if c {
            *self.get_mut(a) = 1;
        } else {
            *self.get_mut(a) = 0;
        }
        self.maybe_print_info("eql");
    }

    fn perform(&mut self, i: Instruction) {
        match i {
            Instruction::Inp(a) => self.inp(a),
            Instruction::Add(a, b) => self.add(a, b),
            Instruction::Mul(a, b) => self.mul(a, b),
            Instruction::Div(a, b) => self.div(a, b),
            Instruction::Mod(a, b) => self.modulo(a, b),
            Instruction::Eql(a, b) => self.eql(a, b),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Variable {
    W,
    X,
    Y,
    Z,
}

impl Variable {
    fn from_str(s: &str) -> Self {
        match s.trim() {
            "w" | "W" => Self::W,
            "x" | "X" => Self::X,
            "y" | "Y" => Self::Y,
            "z" | "Z" => Self::Z,
            _ => panic!("unknown variable"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum VariableOrValue {
    Value(i64),
    Variable(Variable),
}

impl VariableOrValue {
    fn from_str(s: &str) -> Self {
        let v: Option<i64> = s.trim().parse().ok();

        match v {
            Some(n) => Self::Value(n),
            None => Self::Variable(Variable::from_str(s)),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Instruction {
    Inp(Variable),
    Add(Variable, VariableOrValue),
    Mul(Variable, VariableOrValue),
    Div(Variable, VariableOrValue),
    Mod(Variable, VariableOrValue),
    Eql(Variable, VariableOrValue),
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let mut iter = s.trim().split_whitespace();
        let a = iter.next().unwrap().trim();
        let b = iter.next().unwrap().trim();
        let c = iter.next();

        match a {
            "inp" => Self::Inp(Variable::from_str(b)),
            "add" => Self::Add(Variable::from_str(b), VariableOrValue::from_str(c.unwrap())),
            "mul" => Self::Mul(Variable::from_str(b), VariableOrValue::from_str(c.unwrap())),
            "div" => Self::Div(Variable::from_str(b), VariableOrValue::from_str(c.unwrap())),
            "mod" => Self::Mod(Variable::from_str(b), VariableOrValue::from_str(c.unwrap())),
            "eql" => Self::Eql(Variable::from_str(b), VariableOrValue::from_str(c.unwrap())),
            _ => panic!("unknown instruction"),
        }
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
