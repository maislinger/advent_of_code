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

fn compute_solution_part_one(input: &str) -> usize {
    let input_number = input.parse::<usize>().unwrap();

    let mut state = vec![0];
    let mut pos = 0;

    for i in 1..2018 {
        pos = (pos + input_number) % state.len();
        state.insert(pos + 1, i);
        pos += 1;
    }

    state[(pos + 1) % state.len()]
}

fn compute_solution_part_two(input: &str) -> usize {
    let input_number = input.parse::<usize>().unwrap();

    let mut pos = 0;
    let mut len = 1;
    let mut at_pos_one = 0;

    for i in 1..50_000_000 {
        pos = (pos + input_number) % len;
        len += 1;
        if pos == 0 {
            at_pos_one = i;
        }
        pos += 1;
    }

    at_pos_one
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use compute_solution_part_one;
        use compute_solution_part_two;

        let input = "3";
        let solution = compute_solution_part_one(input);
        assert_eq!(solution, 638);

        let solution = compute_solution_part_two(input);
        assert_eq!(solution, 1222153);
    }
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d17 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
