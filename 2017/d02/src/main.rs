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

fn convert_string(input: &str) -> Vec<Vec<u64>> {
    input
        .split('\n')
        .map(|s| {
            s.split_whitespace()
                .map(|s| {
                    let tmp = s.trim();
                    tmp.parse::<u64>().unwrap()
                })
                .collect()
        })
        .collect()
}

fn compute_checksum_one(numbers: &[Vec<u64>]) -> u64 {
    numbers
        .iter()
        .map(|v| v.iter().max().unwrap() - v.iter().min().unwrap())
        .sum()
}

fn compute_checksum_two_row(numbers: &[u64]) -> u64 {
    numbers
        .iter()
        .enumerate()
        .map(|(i, n)| {
            numbers
                .iter()
                .skip(i + 1)
                .map(|n2| if n % n2 == 0 {
                    n / n2
                } else if n2 % n == 0 {
                    n2 / n
                } else {
                    0
                })
                .sum::<u64>()
        })
        .sum()
}

fn compute_checksum_two(numbers: &[Vec<u64>]) -> u64 {
    numbers.iter().map(|v| compute_checksum_two_row(v)).sum()
}

fn compute_solution_part_one(input: &str) -> u64 {
    let numbers = convert_string(input);
    compute_checksum_one(&numbers)
}

fn compute_solution_part_two(input: &str) -> u64 {
    let numbers = convert_string(input);
    compute_checksum_two(&numbers)
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use compute_solution_part_one;
        use compute_solution_part_two;

        let s = "5 1 9 5\n7 5 3\n2 4 6 8";
        assert_eq!(compute_solution_part_one(s), 18);

        let s = "5 9 2 8\n9 4 7 3\n3 8 6 5";
        assert_eq!(compute_solution_part_two(s), 9);
    }
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
