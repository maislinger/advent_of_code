struct Scanner {
    depth: usize,
    range: usize,
}

impl Scanner {
    fn new(depth: usize, range: usize) -> Scanner {
        Scanner {
            depth: depth,
            range: range,
        }
    }

    fn from_str(input: &str) -> Scanner {
        let vals: Vec<_> = input
            .split(": ")
            .map(|s| s.trim().parse().unwrap())
            .collect();
        assert_eq!(vals.len(), 2);
        Scanner::new(vals[0], vals[1])
    }

    fn catch(&self, delay: usize) -> bool {
        let state = (delay + self.depth) % (2 * (self.range - 1));
        state == 0
    }

    fn severity(&self, delay: usize) -> usize {
        if self.catch(delay) {
            self.depth * self.range
        } else {
            0
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

fn convert_input(input: &str) -> Vec<Scanner> {
    input
        .split('\n')
        .map(|s| Scanner::from_str(s.trim()))
        .collect()
}

fn compute_solution_part_one(input: &str) -> usize {
    let scanners = convert_input(input);
    scanners.iter().map(|s| s.severity(0)).sum()
}

fn compute_solution_part_two(input: &str) -> usize {
    let scanners = convert_input(input);
    for delay in 0.. {
        let mut passed = true;
        for scanner in &scanners {
            if scanner.catch(delay) {
                passed = false;
                break;
            }
        }
        if passed {
            return delay;
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use compute_solution_part_one;
        use compute_solution_part_two;

        let input = "0: 3
                     1: 2
                     4: 4
                     6: 4";

        let solution = compute_solution_part_one(input);
        assert_eq!(solution, 24);

        let solution = compute_solution_part_two(input);
        assert_eq!(solution, 10);
    }
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
