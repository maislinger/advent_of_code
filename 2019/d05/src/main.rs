use int_code_machine::IntCodeMachine;

fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

fn compute_solution_part_one(input: &str) -> i64 {
    let mut machine = IntCodeMachine::from_string(input);
    machine.default_input = Some(1);
    while !machine.halted {
        machine.step();
    }
    machine.last_output_signal().unwrap()
}

fn compute_solution_part_two(input: &str) -> i64 {
    let mut machine = IntCodeMachine::from_string(input);
    machine.default_input = Some(5);
    while !machine.halted {
        machine.step();
    }
    machine.last_output_signal().unwrap()
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d05 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
