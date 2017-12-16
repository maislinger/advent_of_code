struct Generator {
    state: u64,
    multiplier: u64,
    divider: u64,
}

impl Generator {
    fn new(state: u64, multiplier: u64, divider: u64) -> Generator {
        Generator {
            state: state,
            multiplier: multiplier,
            divider: divider,
        }
    }

    fn new_a(state: u64) -> Generator {
        Generator::new(state, 16_807, 2_147_483_647)
    }

    fn new_b(state: u64) -> Generator {
        Generator::new(state, 48_271, 2_147_483_647)
    }

    fn step(&mut self) -> u64 {
        self.state = (self.state * self.multiplier) % self.divider;
        self.state
    }

    fn step_until_divisible_by(&mut self, nr: u64) -> u64 {
        loop {
            let result = self.step();
            if result % nr == 0 {
                break result;
            }
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

fn convert_input(input: &str) -> [u64; 2] {
    let vec: Vec<_> = input
        .split('\n')
        .map(|s| s.split("with ").nth(1).unwrap().parse().unwrap())
        .collect();
    assert_eq!(vec.len(), 2);
    [vec[0], vec[1]]
}

fn compute_solution_part_one(input: &str) -> usize {
    let seeds = convert_input(input);
    let mut gen_a = Generator::new_a(seeds[0]);
    let mut gen_b = Generator::new_b(seeds[1]);

    (0..40_000_000)
        .map(|_| {
            let a = gen_a.step() & 0b1111_1111_1111_1111;
            let b = gen_b.step() & 0b1111_1111_1111_1111;
            a == b
        })
        .filter(|b| *b)
        .count()
}

fn compute_solution_part_two(input: &str) -> usize {
    let seeds = convert_input(input);
    let mut gen_a = Generator::new_a(seeds[0]);
    let mut gen_b = Generator::new_b(seeds[1]);

    (0..5_000_000)
        .map(|_| {
            let a = gen_a.step_until_divisible_by(4) & 0b1111_1111_1111_1111;
            let b = gen_b.step_until_divisible_by(8) & 0b1111_1111_1111_1111;
            a == b
        })
        .filter(|b| *b)
        .count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use compute_solution_part_one;
        use compute_solution_part_two;

        let input = "Generator A starts with 65
                     Generator B starts with 8921";
        let solution = compute_solution_part_one(input);
        assert_eq!(solution, 588);

        let solution = compute_solution_part_two(input);
        assert_eq!(solution, 309);
    }
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d15 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
