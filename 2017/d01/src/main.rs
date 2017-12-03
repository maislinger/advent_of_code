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

fn convert_string(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn compute_sum(numbers: &[u8], distance: usize) -> u64 {
    numbers
        .iter()
        .cycle()
        .skip(distance)
        .zip(numbers.iter())
        .filter(|&(&a, &b)| a == b)
        .map(|(a, _)| u64::from(*a))
        .sum()
}

fn compute_solution_part_one(input: &str) -> u64 {
    let numbers = convert_string(input);
    compute_sum(&numbers, 1)
}

fn compute_solution_part_two(input: &str) -> u64 {
    let numbers = convert_string(input);
    compute_sum(&numbers, numbers.len() / 2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use compute_solution_part_one;
        use compute_solution_part_two;

        assert_eq!(compute_solution_part_one("1122"), 3);
        assert_eq!(compute_solution_part_one("1111"), 4);
        assert_eq!(compute_solution_part_one("1234"), 0);
        assert_eq!(compute_solution_part_one("91212129"), 9);

        assert_eq!(compute_solution_part_two("1212"), 6);
        assert_eq!(compute_solution_part_two("1221"), 0);
        assert_eq!(compute_solution_part_two("123425"), 4);
        assert_eq!(compute_solution_part_two("123123"), 12);
        assert_eq!(compute_solution_part_two("12131415"), 4);
    }
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d01 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
