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
    let digits: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let n_columns = 25;
    let n_rows = 6;

    let mut counts = 0;
    let mut product = 0;

    for (i, layer) in digits.chunks(n_columns * n_rows).enumerate() {
        let counter = |n| layer.iter().filter(|c| **c == n).count();
        let c = counter(0);
        let p = counter(1) * counter(2);

        if c < counts || i == 0 {
            counts = c;
            product = p;
        }
    }

    product
}

fn mix(upper: u32, lower: u32) -> u32 {
    match (upper, lower) {
        (0, _) => 0,
        (1, _) => 1,
        (2, d) => d,
        _ => panic!("unknown colors"),
    }
}

fn print_solution_part_two(input: &str) {
    let digits: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let n_columns = 25;
    let n_rows = 6;
    let mut image = vec![2; n_columns * n_rows];

    for layer in digits.chunks(n_columns * n_rows) {
        for (i, l) in image.iter_mut().zip(layer.iter()) {
            *i = mix(*i, *l);
        }
    }

    let mut column = 0;
    for i in image {
        match i {
            0 => print!("■"),
            1 => print!("□"),
            _ => panic!("not every pixel has a color"),
        }
        column += 1;
        if column == n_columns {
            println!();
            column = 0;
        }
    }
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d08 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2");
        print_solution_part_two(&input);
    }
}
