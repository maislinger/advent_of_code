use int_code_machine::IntCodeMachine;
use std::cmp::Ordering;

fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

fn compute_solution_part_one(input: &str) -> u64 {
    let mut machine = IntCodeMachine::from_string(input);
    let mut blocks = 0;
    while !machine.halted {
        machine.run_until_output_or_halt();
        machine.run_until_output_or_halt();
        machine.run_until_output_or_halt();
        let tile_id = machine.last_output_signal().unwrap();
        if tile_id == 2 {
            blocks += 1;
        }
    }
    blocks
}

fn compute_solution_part_two(input: &str) -> i64 {
    let mut machine = IntCodeMachine::from_string(input);
    machine.data[0] = 2;
    machine.default_input = Some(0);
    let mut ball_x = None;
    let mut paddle_x = None;
    let mut score = None;
    while !machine.halted {
        machine.run_until_output_or_halt();
        let x = machine.last_output_signal().unwrap();
        machine.run_until_output_or_halt();
        let y = machine.last_output_signal().unwrap();
        machine.run_until_output_or_halt();
        let tile_id = machine.last_output_signal().unwrap();

        match (x, y) {
            (-1, 0) => score = Some(tile_id),
            (x, _) => match tile_id {
                3 => paddle_x = Some(x),
                4 => ball_x = Some(x),
                _ => (),
            },
        }

        if let (Some(px), Some(bx)) = (paddle_x, ball_x) {
            let diff = bx - px;
            let input = match diff.cmp(&0) {
                Ordering::Less => -1,
                Ordering::Greater => 1,
                Ordering::Equal => 0,
            };
            machine.default_input = Some(input);
        }
    }
    score.unwrap()
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d13 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
