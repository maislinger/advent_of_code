use int_code_machine::IntCodeMachine;
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

fn compute_thrust(machine: &IntCodeMachine, phase_settings: Vec<i64>) -> i64 {
    let mut io = 0;
    for phase_setting in phase_settings {
        let mut local_machine = machine.clone();
        local_machine.add_input_signal(phase_setting);
        local_machine.default_input = Some(io);
        local_machine.run_until_halt();
        io = local_machine.last_output_signal().unwrap();
    }
    io
}

fn compute_loop_thrust(machine: &IntCodeMachine, phase_settings: Vec<i64>) -> i64 {
    let mut machines: Vec<_> = phase_settings
        .iter()
        .map(|p| {
            let mut m = machine.clone();
            m.add_input_signal(*p);
            m
        })
        .collect();
    let mut machine_index = 0;
    let mut io = 0;
    let mut result = None;
    loop {
        let m = &mut machines[machine_index];
        m.default_input = Some(io);
        m.run_until_output_or_halt();
        if m.halted {
            break;
        }
        io = m.last_output_signal().unwrap();
        if machine_index == phase_settings.len() - 1 {
            result = Some(io);
        }
        machine_index += 1;
        if machine_index >= phase_settings.len() {
            machine_index = 0;
        }
    }
    result.unwrap()
}

fn compute_solution_part_one(input: &str) -> i64 {
    let machine = IntCodeMachine::from_string(input);
    (0..=4)
        .permutations(5)
        .map(|c| compute_thrust(&machine, c))
        .max()
        .unwrap()
}

fn compute_solution_part_two(input: &str) -> i64 {
    let machine = IntCodeMachine::from_string(input);
    (5..=9)
        .permutations(5)
        .map(|c| compute_loop_thrust(&machine, c))
        .max()
        .unwrap()
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
