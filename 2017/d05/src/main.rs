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

fn convert_input(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

fn compute_solution_part_one(input: &str) -> u64 {
    let mut instructions = convert_input(input);
    let mut state = 0;
    let mut steps = 0;

    loop {
        let jump = instructions[state];
        let neg = jump < 0;
        let jump = jump.abs() as usize;
        instructions[state] += 1;
        steps += 1;
        if (neg && jump > state) || (!neg && jump + state >= instructions.len()) {
            break;
        }
        if neg {
            state -= jump;
        } else {
            state += jump;
        }
    }
    steps
}

fn compute_solution_part_two(input: &str) -> u64 {
    let mut instructions = convert_input(input);
    let mut state = 0;
    let mut steps = 0;

    loop {
        let jump = instructions[state];
        let neg = jump < 0;
        let jump = jump.abs() as usize;
        if instructions[state] < 3 {
            instructions[state] += 1;
        } else {
            instructions[state] -= 1;
        }
        steps += 1;
        if (neg && jump > state) || (!neg && jump + state >= instructions.len()) {
            break;
        }
        if neg {
            state -= jump;
        } else {
            state += jump;
        }
    }
    steps
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use compute_solution_part_one;
        use compute_solution_part_two;

        assert_eq!(compute_solution_part_one("0\n3\n0\n1\n-3"), 5);
        assert_eq!(compute_solution_part_two("0\n3\n0\n1\n-3"), 10);
    }
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
