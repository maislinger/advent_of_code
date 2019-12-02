fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

fn parse_input(input: &str) -> Vec<usize> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

fn compute_output(noun: usize, verb: usize, data: &mut [usize]) -> usize {
    data[1] = noun;
    data[2] = verb;
    let mut position = 0;
    while data[position] != 99 {
        let i1 = data[position + 1];
        let i2 = data[position + 2];
        let ir = data[position + 3];
        match data[position] {
            1 => {
                data[ir] = data[i1] + data[i2];
            }
            2 => {
                data[ir] = data[i1] * data[i2];
            }
            _ => {
                panic!("Unsupported opcode");
            }
        }
        position += 4;
    }
    data[0]
}

fn compute_solution_part_one(input: &str) -> usize {
    let mut data = parse_input(input);
    compute_output(12, 2, &mut data)
}

fn compute_solution_part_two(input: &str) -> usize {
    let data = parse_input(input);
    for noun in 0..100 {
        for verb in 0..100 {
            let mut d = data.clone();
            let output = compute_output(noun, verb, &mut d);
            if output == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("No solution found");
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d02 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
