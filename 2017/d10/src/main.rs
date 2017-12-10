struct Hasher {
    position: usize,
    skip: usize,
    vals: Vec<usize>,
}

impl Hasher {
    fn new(length: usize) -> Hasher {
        let vals = (0..length).collect();
        Hasher {
            position: 0,
            skip: 0,
            vals: vals,
        }
    }

    fn step(&mut self, length: usize) {
        let n = self.vals.len();
        let mut i = self.position;
        let mut j = (self.position + length - 1) % n;

        for _ in 0..length / 2 {
            self.vals.swap(i, j);
            i = if i == n - 1 { 0 } else { i + 1 };
            j = if j == 0 { n - 1 } else { j - 1 };
        }

        self.position += length + self.skip;
        self.skip += 1;
        self.position %= n;
    }

    fn parse_str_as_numbers(&mut self, input: &str) {
        input
            .split(',')
            .map(|sr| {
                let length = sr.trim().parse().unwrap();
                self.step(length);
            })
            .count();
    }

    fn parse_str_as_ascii(&mut self, input: &str) {
        input
            .chars()
            .map(|c| {
                let c = c as u8;
                let length = c as usize;
                self.step(length);
            })
            .count();
    }

    fn to_hex(&self) -> String {
        let mut result = "".to_owned();
        for i in 0..16 {
            let xor = self.vals.iter().skip(i * 16).take(16).fold(
                0usize,
                |acc, &x| acc ^ x,
            );
            result += &format!("{:02x}", xor);
        }
        result
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

fn compute_solution_part_one(input: &str, length: usize) -> usize {
    let mut hasher = Hasher::new(length);
    hasher.parse_str_as_numbers(input);
    hasher.vals[0] * hasher.vals[1]
}

fn compute_solution_part_two(input: &str) -> String {
    let mut hasher = Hasher::new(256);
    for _ in 0..64 {
        hasher.parse_str_as_ascii(input);
        hasher.parse_str_as_numbers("17,31,73,47,23");
    }
    hasher.to_hex()
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use compute_solution_part_one;
        use compute_solution_part_two;

        let solution = compute_solution_part_one("3, 4, 1, 5", 5);
        assert_eq!(solution, 12);

        let examples = vec!["", "AoC 2017", "1,2,3", "1,2,4"];
        let solutions = vec![
            "a2582a3a0e66e6e86e3812dcb672a272",
            "33efeb34ea91902bb2f59c9920caa6cd",
            "3efbe78a8d82f29979031a4aa0b16a9d",
            "63960835bcdc130f0b66d7ff4f6a5a8e",
        ];
        examples
            .iter()
            .zip(solutions.iter())
            .map(|(e, s)| assert_eq!(compute_solution_part_two(e), *s))
            .count();
    }
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d10 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input, 256));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
