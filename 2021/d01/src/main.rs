fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

fn compute_solution_part_one(input: &str) -> usize {
    let depths: Vec<usize> = input.split('\n').map(|l| l.parse().unwrap()).collect();
    depths.iter().skip(1).zip(depths.iter()).filter(|(d1, d2)| d1 > d2).count()
}

fn compute_solution_part_two(input: &str) -> usize {
    let depths: Vec<usize> = input.split('\n').map(|l| l.parse().unwrap()).collect();
    depths.iter().skip(3).zip(depths.iter()).filter(|(d1, d2)| d1 > d2).count()
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
