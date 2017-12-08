#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::BTreeMap;

enum Comparison {
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
}

enum Direction {
    Increase,
    Decrease,
}

struct Instruction {
    mod_reg: String,
    dir: Direction,
    amount: i64,
    cond_reg: String,
    cond_op: Comparison,
    cond_value: i64,
}

impl Instruction {
    fn new(
        mod_reg: String,
        dir: Direction,
        amount: i64,
        cond_reg: String,
        cond_op: Comparison,
        cond_value: i64,
    ) -> Instruction {
        Instruction {
            mod_reg: mod_reg,
            dir: dir,
            amount: amount,
            cond_reg: cond_reg,
            cond_op: cond_op,
            cond_value: cond_value,
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

        static ref RE: Regex = {
            let head = r"([a-z]+) (inc|dec) (-?[0-9]+) if ";
            let tail = r"([a-z]+) (==|<|<=|>|>=|!=) (-?[0-9]+)";
            let re_string = head.to_owned() + tail;
            Regex::new(&re_string).unwrap()
        };
    }

    let mut result = Vec::new();

    for cap in RE.captures_iter(input) {
        let mod_reg = cap.get(1).unwrap().as_str().to_owned();
        let dir = {
            let tmp = cap.get(2).unwrap().as_str();
            if tmp == "inc" {
                Direction::Increase
            } else {
                Direction::Decrease
            }
        };
        let amount = cap.get(3).unwrap().as_str().parse().unwrap();
        let cond_reg = cap.get(4).unwrap().as_str().to_owned();
        let cond_op = {
            let tmp = cap.get(5).unwrap().as_str();
            match tmp {
                "<" => Comparison::Less,
                "<=" => Comparison::LessEqual,
                ">" => Comparison::Greater,
                ">=" => Comparison::GreaterEqual,
                "==" => Comparison::Equal,
                "!=" => Comparison::NotEqual,
                _ => unreachable!(),
            }
        };
        let cond_value = cap.get(6).unwrap().as_str().parse().unwrap();
        result.push(Instruction::new(
            mod_reg,
            dir,
            amount,
            cond_reg,
            cond_op,
            cond_value,
        ));
    }

    result
}

fn run_instruction(inst: &Instruction, regs: &mut BTreeMap<String, i64>) -> i64 {
    let mut mod_reg_val = match regs.get(&inst.mod_reg) {
        Some(i) => *i,
        None => 0,
    };
    let cond_reg_val = match regs.get(&inst.cond_reg) {
        Some(i) => *i,
        None => 0,
    };

    let do_operation = match inst.cond_op {
        Comparison::Less => cond_reg_val < inst.cond_value,
        Comparison::LessEqual => cond_reg_val <= inst.cond_value,
        Comparison::Greater => cond_reg_val > inst.cond_value,
        Comparison::GreaterEqual => cond_reg_val >= inst.cond_value,
        Comparison::Equal => cond_reg_val == inst.cond_value,
        Comparison::NotEqual => cond_reg_val != inst.cond_value,
    };

    if do_operation {
        match inst.dir {
            Direction::Increase => mod_reg_val += inst.amount,
            Direction::Decrease => mod_reg_val -= inst.amount,
        }

        regs.insert(inst.mod_reg.clone(), mod_reg_val);
    }
    mod_reg_val
}

fn compute_solution_part_one(input: &str) -> i64 {
    let mut regs = BTreeMap::new();
    let instructions = convert_input(input);
    for inst in &instructions {
        run_instruction(inst, &mut regs);
    }
    *regs.iter().map(|e| e.1).max().unwrap()
}

fn compute_solution_part_two(input: &str) -> i64 {
    let mut regs = BTreeMap::new();
    let instructions = convert_input(input);
    let mut max = 0;
    for inst in &instructions {
        let new_val = run_instruction(inst, &mut regs);
        if new_val > max {
            max = new_val;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use compute_solution_part_one;
        use compute_solution_part_two;

        let input = "b inc 5 if a > 1
                     a inc 1 if b < 5
                     c dec -10 if a >= 1
                     c inc -20 if c == 10"
            .to_owned();

        let solution = compute_solution_part_one(&input);
        assert_eq!(solution, 1);
        let solution = compute_solution_part_two(&input);
        assert_eq!(solution, 10);
    }
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
