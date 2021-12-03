fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

struct Bitset {
    data: Vec<bool>,
    n_bits: usize,
    n_lines: usize,
}

impl Bitset {
    fn from_input(input: &str) -> Self {
        let mut data = Vec::new();
        let n_bits = input.lines().next().unwrap().chars().count();
        let n_lines = input.lines().count();

        for line in input.lines() {
            assert_eq!(
                n_bits,
                line.chars().count(),
                "Not every input line has the same number of bits."
            );
            for c in line.chars() {
                match c {
                    '0' => data.push(false),
                    '1' => data.push(true),
                    _ => panic!("input not a binary number"),
                }
            }
        }

        Self {
            data,
            n_bits,
            n_lines,
        }
    }

    fn get(&self, line_number: usize, bit_number: usize) -> bool {
        assert!(line_number < self.n_lines);
        assert!(bit_number < self.n_bits);
        self.data[line_number * self.n_bits + bit_number]
    }
}

fn compute_solution_part_one(input: &str) -> u64 {
    let bitset = Bitset::from_input(input);

    let mut counts_one = vec![0usize; bitset.n_bits];
    let mut counts_zero = vec![0usize; bitset.n_bits];

    for i in 0..bitset.n_lines {
        for j in 0..bitset.n_bits {
            if bitset.get(i, j) {
                counts_one[j] += 1;
            } else {
                counts_zero[j] += 1;
            }
        }
    }

    let mut gamma_rate: u64 = 0;
    let mut epsilon_rate: u64 = 0;
    for (c_one, c_zero) in counts_one.iter().zip(counts_zero.iter()) {
        gamma_rate <<= 1;
        epsilon_rate <<= 1;
        if c_one > c_zero {
            gamma_rate += 1;
        } else {
            epsilon_rate += 1;
        }
    }

    gamma_rate * epsilon_rate
}

fn reduce_bitset(bitset: &Bitset, keep_function: fn(usize, usize) -> bool) -> u64 {
    let mut do_lines: Vec<usize> = (0..bitset.n_lines).collect();

    let mut j = 0;

    while do_lines.len() > 1 {
        let mut counts_one = 0;
        let mut counts_zero = 0;

        for i in do_lines.iter() {
            if bitset.get(*i, j) {
                counts_one += 1;
            } else {
                counts_zero += 1;
            }
        }

        let keep_value = keep_function(counts_zero, counts_one);
        do_lines = do_lines
            .iter()
            .filter(|i| bitset.get(**i, j) == keep_value)
            .cloned()
            .collect();
        j = (j + 1) % bitset.n_bits;
    }

    assert!(do_lines.len() == 1);
    let i = do_lines[0];
    let mut result = 0;
    for j in 0..bitset.n_bits {
        result <<= 1;
        if bitset.get(i, j) {
            result += 1;
        }
    }
    result
}

fn compute_solution_part_two(input: &str) -> u64 {
    let bitset = Bitset::from_input(input);
    let oxygen = reduce_bitset(&bitset, |count_zero, count_one| count_one >= count_zero);
    let co2 = reduce_bitset(&bitset, |count_zero, count_one| count_zero > count_one);

    oxygen * co2
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d03 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
